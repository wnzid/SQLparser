<div align="center">

# SQLparser

**A dependency-free SQL tokenizer, Pratt expression parser, and interactive AST explorer.**

`Rust` · `Lexer` · `Pratt parser` · `No external crates`

</div>

SQLparser reads a focused subset of SQL, converts the input into tokens, builds an abstract syntax tree, and prints the parsed structure. It is intentionally organized without Cargo so the complete learning implementation remains small and visible.

## Supported surface

- Numbers, quoted strings, identifiers, punctuation, and SQL keywords
- Arithmetic and comparison expressions
- Boolean expression precedence through Pratt parsing
- `SELECT … FROM … WHERE … ORDER BY …`
- `CREATE TABLE` statements and column constraints
- Multi-line interactive input terminated by `;`

This is a learning parser, not a validating implementation of the complete SQL standard.

## Build

```bash
rustc main.rs
```

Run the result:

```bash
./main        # macOS/Linux
main.exe      # Windows
```

Example input:

```sql
SELECT id, name
FROM users
WHERE id > 10
ORDER BY name ASC;
```

The CLI prints a Rust representation of the resulting `Statement` tree. Use the terminal's end-of-input shortcut on an empty line to exit (`Ctrl+Z`, then Enter on Windows; `Ctrl+D` on macOS/Linux).

## Source map

| File | Responsibility |
| --- | --- |
| `token.rs` | Token and keyword definitions |
| `tokenizer.rs` | Character stream to tokens |
| `statement.rs` | AST types and display logic |
| `parser.rs` | Statement parsing and Pratt expressions |
| `main.rs` | Interactive command-line loop |

## License

Licensed under the [MIT License](LICENSE).
