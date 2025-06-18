# SQLparser

SQLparser is a small command line application written in pure Rust. It tokenizes
and parses a subset of SQL statements and prints out the resulting abstract
syntax tree (AST). It is intended as a demonstration of how to build a simple
lexer and a Pratt parser without relying on external crates.

## Features

- Built-in lexer for numbers, strings, identifiers and keywords
- Pratt style expression parser (arithmetic, comparison and logical operators)
- AST representation for `SELECT` and `CREATE TABLE` statements, including column constraints
- Interactive CLI for multi-line input

## Building

The repository is not organised as a Cargo project. You can compile it directly
using `rustc`:

```bash
rustc main.rs
```

This produces an executable named `main` in the project directory.

## Usage

Run the compiled binary from your terminal:

```bash
./main
```

Enter SQL statements terminated with a semicolon (`;`). Statements can span
multiple lines. Use `Ctrl+Z` on an empty line to exit.

The CLI prints the parsed `Statement` structure or an error if the statement
cannot be parsed.

### Example

```
> SELECT id, name FROM users WHERE id > 10 ORDER BY name ASC;
Select {
    columns: [
        Identifier("id"),
        Identifier("name"),
    ],
    from: "users",
    where: Some(BinaryOperation {
        left_operand: Box::new(Identifier("id")),
        operator: GreaterThan,
        right_operand: Box::new(Number(10)),
    }),
    orderby: [UnaryOperation {
        operand: Box::new(Identifier("name")),
        operator: Asc,
    }],
}
```

## Source Layout

- `token.rs` – definitions of tokens and SQL keywords
- `tokenizer.rs` – converts raw input into a stream of tokens
- `statement.rs` – AST structures and display implementations
- `parser.rs` – main Pratt parser that produces the AST
- `main.rs` – interactive command line interface

## Contributing

Contributions in the form of bug reports, feature requests or pull requests are
welcome. This project is intended primarily as a learning resource, so the code
is deliberately kept simple and dependency free.

