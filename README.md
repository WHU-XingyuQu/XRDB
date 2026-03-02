# XRDB
<img width="1229" height="852" alt="image" src="https://github.com/user-attachments/assets/9008c8fd-229f-4980-b5c7-364f1ada9b2e" />

XRDB is a minimalist relational database engine written in Rust. This is a toy project designed specifically for **beginners learning Rust and underlying database principles**.

This project does not pursue extreme high concurrency or massive data processing performance. Instead, it focuses on **clear project structure, readability, and the practical application of Rust features**. It is perfect for those who want to understand "how a SQL statement is parsed and executed in a database" and "how to build system software from scratch using Rust".

## 🌟 Features

* **Minimalist SQL Support**: Supports the most basic relational database operations: `CREATE TABLE`, `INSERT`, and `SELECT *` (full table scan).
* **Flexible Storage Engines**: Through Rust's `trait` abstraction, it implements a memory-based storage (`MemoryEngine`) and an append-only log-based disk KV storage (`DiskEngine`), with support for simple log compaction.
* **MVCC Transaction Support**: Implements basic Multi-Version Concurrency Control (MVCC), supporting transaction begin, commit, rollback, and snapshot isolation level.
* **Parser Written from Scratch**: Hand-written lexical analyzer (Lexer) and syntax analyzer (Parser) without relying on third-party SQL parsing libraries, clearly demonstrating the entire process of building an Abstract Syntax Tree (AST) from a SQL string.

## 📂 Project Structure

The core module division strictly follows classic database architecture with clear responsibilities:

* `src/sql/parser/`: Lexical and syntax analysis, parsing SQL text into AST Tokens.
* `src/sql/plan/`: Query Planner, converting AST into an executable logical execution plan.
* `src/sql/executor/`: Executor, scheduling underlying read/write mutations based on the logical plan.
* `src/storage/`: Underlying storage engine abstraction and implementation.
    * `memory.rs`: Pure memory engine based on `BTreeMap`.
    * `disk.rs`: Log-structured disk engine with basic crash recovery mechanisms.
    * `mvcc.rs`: Core of transaction concurrency control mechanisms.

## 🛠️ Rust Feature Highlights

This project fully utilizes modern Rust language features during development, serving as a great practical case:

* **Trait Abstraction Design**: Defines unified `Engine` and `Executor` interfaces, decoupling execution logic from underlying storage.
* **Algebraic Data Types (Enum)**: Extensively uses Enums to define AST nodes (`Statement`, `Expression`) and Tokens. Combined with `match` pattern matching, the parsing logic is both safe and elegant.
* **Custom Error Handling**: Uniformly defines an `Error` type and implements the `From` trait for seamless conversion of various errors like I/O, parsing, and concurrency (`Result<T>`).
* **Serialization & Deserialization**: Leverages `serde` and `bincode` for efficient encoding and decoding of data rows (Row) and complex MVCC Keys.

## 💻 Supported SQL Syntax

Currently, XRDB supports parsing and executing the following core SQL syntax:

**1. Create Table**
```sql
CREATE TABLE users (
    id INT PRIMARY KEY,
    name VARCHAR,
    is_active BOOL DEFAULT true
);

```

*(Supported data types: INT/INTEGER, BOOL/BOOLEAN, FLOAT/DOUBLE, STRING/TEXT/VARCHAR)*

**2. Insert Data**

```sql
INSERT INTO users (id, name, is_active) VALUES (1, 'db_user', true);

```

**3. Scan Data**

```sql
SELECT * FROM users;

```

## 🚀 Getting Started

This project has minimal dependencies, ensuring high reproducibility. You only need to install the standard Rust toolchain to start exploring:

**1. Clone the repository**

```bash
git clone https://github.com/WHU-XingyuQu/XRDB.git

```

**2. Run tests**
The project includes rich unit tests covering Parser, Planner, Executor, and MVCC concurrent transactions. You can quickly understand the input and output of each module by running tests:

```bash
cargo test

```
