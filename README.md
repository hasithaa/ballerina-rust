# Ballerina Compiler

A compiler implementation for the Ballerina programming language in Rust.

## Building

```bash
# Build all crates
cargo build

# Run tests
cargo test

# Run CLI
./target/debug/bal-cli build ballerina-src/m1/exp1.bal
```

## Development

This project is organized as a Rust workspace with multiple crates:

- bal-ast: Defines the Abstract Syntax Tree structures and validation
- bal-cli: Command-line interface for the compiler
- bal-codegen: Handles code generation from AST with source mapping
- bal-lsp: Language Server Protocol implementation
- bal-parser: Parses the source code into an AST
- bal-syntax: Defines the syntax and lexing rules
- bal-wasm: WebAssembly bindings for the compiler

## Ballerina Language Features:

See [LanguageSupport.md](LanguageSupport.md)

## Features

Work in progress

## License

MIT

## Contributing

[Contribution guidelines to be added]

## Author

Hasitha Aravinda - Copyright 2025
