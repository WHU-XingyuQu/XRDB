use std::iter::Peekable;

use ast::Column;
use lexer::{Keyword, Lexer, Token};

use crate::error::{Error, Result};
use crate::sql::parser::lexer::Keyword::Default;
use super::types::DataType;

pub mod ast;
mod lexer;

pub struct Parser<'a>{
    lexer: Peekable<Lexer<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Parser{
            lexer: Lexer::new(input).peekable(),
        }
    }

    // get AST
    pub fn parse(&mut self) -> Result<ast::Statement> {
        let stmt = self.parse_statement()?; // consume all the tokens
        self.next_expect(Token::Semicolon)?; // expect: end with ;
        if let Some(token) = self.peek()? {
            return Err(Error::Parse(format!("[Parser] Unexpected token {}", token)));
        }
        Ok(stmt)
    }

    // define type of SQL
    fn parse_statement(&mut self) -> Result<ast::Statement> {
        match self.peek()?{
            Some(Token::Keyword(Keyword::Create)) => self.parse_ddl(),
            Some(Token::Keyword(Keyword::Select)) => self.parse_select(),
            Some(Token::Keyword(Keyword::Insert)) => self.parse_insert(),
            Some(t) => Err(Error::Parse(format!("[Parser] Unexpected token {}", t))),
            None => Err(Error::Parse(format!("[Parser] Unexpected end of input"))),
        }
    }

    fn parse_ddl(&mut self) -> Result<ast::Statement> {
        match self.next()?{
            Token::Keyword(Keyword::Create) =>
                match self.next()?{
                    Token::Keyword(Keyword::Table) => self.parse_ddl_create_table(),
                    token=> Err(Error::Parse(format!("[Parser] Unexpected token {}", token))),
                }
            token=> Err(Error::Parse(format!("[Parser] Unexpected token {}", token))),
        }
    }

    fn parse_select(&mut self) -> Result<ast::Statement> {
        self.next_expect(Token::Keyword(Keyword::Select))?;
        self.next_expect(Token::Asterisk)?;
        self.next_expect(Token::Keyword(Keyword::From))?;
        let table_name = self.next_ident()?;
        Ok(ast::Statement::Select {table_name})
    }

    fn parse_insert(&mut self) -> Result<ast::Statement> {
        self.next_expect(Token::Keyword(Keyword::Insert))?;
        self.next_expect(Token::Keyword(Keyword::Into))?;
        let table_name = self.next_ident()?;

        // columns
        let columns =if self.next_if_token(Token::OpenParen).is_some(){
            let mut cols = Vec::new();
            loop {
                cols.push(self.next_ident()?.to_string());
                match self.next()? {
                    Token::CloseParen => {
                        break;
                    }
                    Token::Comma => {},
                    token => return Err(Error::Parse(format!("[Parser] Unexpected token {}", token))),
                }
            }
            Some(cols)
        }else{
            None
        };

        // values
        self.next_expect(Token::Keyword(Keyword::Values))?;
        // insert into tbl(a, b, c) values (1, 2, 3),(4, 5, 6);
        let mut values=Vec::new();
        loop{
            self.next_expect(Token::OpenParen)?;
            let mut sub_values=Vec::new();
            loop{
                sub_values.push(self.parse_expression()?);
                match self.next()? {
                    Token::CloseParen => {
                        break;
                    }
                    Token::Comma => {},
                    token => return Err(Error::Parse(format!("[Parser] Unexpected token {}", token))),
                }
            }
            values.push(sub_values);
            if self.next_if_token(Token::Comma).is_none(){
                break;
            }
        }

        Ok(ast::Statement::Insert {
            table_name,
            columns,
            values,
        })
    }

    fn parse_ddl_create_table(&mut self) -> Result<ast::Statement> {
        // table name following "Create Table"
        let table_name = self.next_ident()?;
        // ( following "("
        self.next_expect(Token::OpenParen)?;

        let mut columns = Vec::new();

        loop {
            columns.push(self.next_ddl_column()?);
            if self.next_if_token(Token::Comma).is_none() {
                break;
            }
        }

        self.next_expect(Token::CloseParen)?;
        Ok(ast::Statement::CreateTable {
            name: table_name,
            columns,
        })
    }

    fn next_ddl_column(&mut self) -> Result<ast::Column> {
        let mut column = Column{
            name: self.next_ident()?,
            data_type: match self.next()?{
                Token::Keyword(Keyword::Int) | Token::Keyword(Keyword::Integer) => {
                    DataType::Integer
                }
                Token::Keyword(Keyword::Bool) | Token::Keyword(Keyword::Boolean) => {
                    DataType::Boolean
                }
                Token::Keyword(Keyword::Float) | Token::Keyword(Keyword::Double) => DataType::Float,
                Token::Keyword(Keyword::String)
                | Token::Keyword(Keyword::Text)
                | Token::Keyword(Keyword::Varchar) => DataType::String,
                token => return Err(Error::Parse(format!("[Parser] Unexpected token {}", token))),
            },
            nullable: None,
            default: None,
        };

        // column's nullable and default
        while let Some(Token::Keyword(keyword)) = self.next_if_keyword(){
            match keyword {
                Keyword::Null => column.nullable = Some(true),
                Keyword::Not => {
                    self.next_expect(Token::Keyword(Keyword::Null))?;
                    column.nullable = Some(false);
                }
                Keyword::Default => column.default = Some(self.parse_expression()?),
                k => return Err(Error::Parse(format!("[Parser] Unexpected keyword {}", k))),
            }
        }

        Ok(column)
    }

    fn parse_expression(&mut self) -> Result<ast::Expression> {
        let expression = match self.next()?{
            Token::Number(n)=>{
                if n.chars().all(|c| c.is_ascii_digit()) {
                    ast::Consts::Integer(n.parse()?).into()
                }else{
                    ast::Consts::Float(n.parse()?).into()
                }
            }
            Token::String(s)=> ast::Consts::String(s).into(),
            Token::Keyword(Keyword::True)=>ast::Consts::Boolean(true).into(),
            Token::Keyword(Keyword::False)=>ast::Consts::Boolean(false).into(),
            Token::Keyword(Keyword::Null)=>ast::Consts::Null.into(),
            token => return Err(Error::Parse(format!("[Parser] Unexpected expression token {}", token))),
        };
        Ok(expression)
    }


    fn peek(&mut self) -> Result<Option<Token>> {
        self.lexer.peek().cloned().transpose()
        // use "transpose" to transfer Option<Result<T, E>> to Result<Option<T, E>>
    }

    fn next(&mut self) -> Result<Token>{
        self.lexer.next()
            .unwrap_or_else(|| Err(Error::Parse(format!("[Parser] Unexpected end of input"))))
    }

    fn next_expect(&mut self, expected: Token) -> Result<()> {
        let token = self.next()?;
        if token == expected {
            Ok(())
        } else {
            Err(Error::Parse(format!(
                "[Parser] Expected token {}, got {}",
                expected, token
            )))
        }
    }

    fn next_ident(&mut self) -> Result<String> {
        match self.next()?{
            Token::Ident(ident) => Ok(ident),
            t => Err(Error::Parse(format!("[Parser] Expected ident, got token {}", t))),
        }
    }

    fn next_if<F: Fn(&Token) -> bool>(&mut self, predicate:F) -> Option<Token>{
        self.peek().unwrap_or(None).filter(|token| predicate(token))?;
        self.next().ok()
    }

    fn next_if_keyword(&mut self) -> Option<Token> {
        self.next_if(|t| matches!(t, Token::Keyword(_)))
    }

    fn next_if_token(&mut self, token: Token) -> Option<Token> {
        self.next_if(|t| t == &token)
    }
}

#[cfg(test)]
mod tests {
    use crate::{error::Result, sql::parser::ast};

    use super::Parser;

    #[test]
    fn test_parser_create_table() -> Result<()> {
        let sql1 = "
            create table tbl1 (
                a int default 100,
                b float not null,
                c varchar null,
                d bool default true
            );
        ";
        let stmt1 = Parser::new(sql1).parse()?;

        let sql2 = "
        create            table tbl1 (
            a int default     100,
            b float not null     ,
            c varchar      null,
            d       bool default        true
        );
        ";
        let stmt2 = Parser::new(sql2).parse()?;
        assert_eq!(stmt1, stmt2);

        let sql3 = "
            create            table tbl1 (
            a int default     100,
            b float not null     ,
            c varchar      null,
            d       bool default        true
        )
        ";

        let stmt3 = Parser::new(sql3).parse();
        assert!(stmt3.is_err());
        Ok(())
    }

    #[test]
    fn test_parser_insert() -> Result<()> {
        let sql1 = "insert into tbl1 values (1, 2, 3, 'a', true);";
        let stmt1 = Parser::new(sql1).parse()?;
        assert_eq!(
            stmt1,
            ast::Statement::Insert {
                table_name: "tbl1".to_string(),
                columns: None,
                values: vec![vec![
                    ast::Consts::Integer(1).into(),
                    ast::Consts::Integer(2).into(),
                    ast::Consts::Integer(3).into(),
                    ast::Consts::String("a".to_string()).into(),
                    ast::Consts::Boolean(true).into(),
                ]],
            }
        );

        let sql2 = "insert into tbl2 (c1, c2, c3) values (3, 'a', true),(4, 'b', false);";
        let stmt2 = Parser::new(sql2).parse()?;
        assert_eq!(
            stmt2,
            ast::Statement::Insert {
                table_name: "tbl2".to_string(),
                columns: Some(vec!["c1".to_string(), "c2".to_string(), "c3".to_string()]),
                values: vec![
                    vec![
                        ast::Consts::Integer(3).into(),
                        ast::Consts::String("a".to_string()).into(),
                        ast::Consts::Boolean(true).into(),
                    ],
                    vec![
                        ast::Consts::Integer(4).into(),
                        ast::Consts::String("b".to_string()).into(),
                        ast::Consts::Boolean(false).into(),
                    ],
                ],
            }
        );

        Ok(())
    }

    #[test]
    fn test_parser_select() -> Result<()> {
        let sql = "select * from tbl1;";
        let stmt = Parser::new(sql).parse()?;
        assert_eq!(
            stmt,
            ast::Statement::Select {
                table_name: "tbl1".to_string()
            }
        );
        Ok(())
    }
}