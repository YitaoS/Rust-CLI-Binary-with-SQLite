![CI Status](https://github.com/YitaoS/Rust-CLI-Binary-with-SQLite/actions/workflows/cicd.yml/badge.svg)
# Rust CLI Binary: Quick Start Guide

This guide provides an overview and demonstration of the **Rust CLI Binary with SQLite** project, designed to perform CRUD operations on a SQLite database directly from the command line.

VIDEO WEB: https://drive.google.com/file/d/1NJ-TXXnww_Tyc_7kTPccG4Xag_RrY81j/view?usp=sharing

## Prerequisites

- **Rust**: Ensure Rust is installed by running `rustc --version`.
- **SQLite**: Required for database management. Install from [SQLite Downloads](https://www.sqlite.org/download.html).

## Installation

1. **Clone the Repository**:
   ```bash
   git clone https://github.com/YitaoS/py_vs_rs.git
   cd py_vs_rs
   ```

2. **Build the Binary**:
   ```bash
   cargo build --release
   ```

   This will create an optimized binary in the `target/release` directory.

## Usage

### 1. Running the Binary

Navigate to the `target/release` folder and run the binary:

```bash
./crud_op --help
```

Replace `crud_op` with the actual name of your compiled binary, as specified in `Cargo.toml`. The `--help` flag displays the available commands and options.

### 2. Commands

The CLI offers the following CRUD operations:

- **Create a New Record**:
  ```bash
  ./crud_op create --name "Alice" --age 30
  ```

- **Read All Records**:
  ```bash
  ./crud_op read
  ```

- **Update an Existing Record**:
  ```bash
  ./crud_op update --name "Alice" --age 35
  ```

- **Delete a Record**:
  ```bash
  ./crud_op delete --name "Alice"
  ```

### 3. Examples

Below is an example workflow using the CLI to create, read, update, and delete records in the database.

1. **Create a User**:
   ```bash
   ./crud_op create --name "Bob" --age 25
   ```

2. **List All Users**:
   ```bash
   ./crud_op read
   ```
   Output:
   ```
   ID: 1, Name: Bob, Age: 25
   ```

3. **Update a User's Age**:
   ```bash
   ./crud_op update --name "Bob" --age 26
   ```

4. **Delete a User**:
   ```bash
   ./crud_op delete --name "Bob"
   ```

## Code Structure

- **`src/main.rs`**: Contains the primary functions for database operations and the CLI interface.
- **`Cargo.toml`**: Manages dependencies, such as `rusqlite` for SQLite interaction and `anyhow` for error handling.

## Development

### Run Tests

To run tests, use:

```bash
cargo test
```

### Format Code

Ensure code consistency with:

```bash
cargo fmt
```

## Logging

- **Database Logs**: Every CRUD operation and query executed during runtime is logged in `log.txt`, providing a trace of the program’s database interactions.

## CI/CD Pipeline

The project is set up with GitHub Actions. The workflow file `.github/workflows/cicd.yml` runs the following steps on every push:
1. Set up the Rust environment
2. Install dependencies
3. Format and lint the code
4. Run tests
5. Log test outputs and database as artifacts

## Use of Copilot
1. Rapid Prototyping and Scaffolding
For starting new features or structuring boilerplate code, Copilot can instantly suggest a basic scaffold, saving time on routine setup and enabling me to dive into feature-specific code faster.
In projects where I’m working with known libraries or frameworks, Copilot provides auto-suggestions for function signatures, common idioms, and syntax, reducing repetitive typing.
2. Code Suggestions and Autocompletion
Copilot offers relevant code completions based on the context of my code. For instance, if I’m implementing CRUD operations, Copilot can suggest SQL syntax or Rust code patterns for database connections, minimizing the chances of syntax errors.
For functions involving repetitive patterns (like iterating over data or handling specific error types), Copilot's contextual suggestions allow me to complete these blocks of code quickly without losing focus on the broader functionality.
3. Debugging and Error Resolution
When I encounter errors or am unsure of the best way to implement a specific feature, I can prompt Copilot or another LLM for potential solutions, especially when it comes to language-specific quirks (e.g., Rust’s ownership model or lifetimes).

