# Utopia - Multi-Language Compiler

Utopia is a Rust-based compiler that lets you write code in a unified syntax and compile it to multiple programming languages, or run it directly using the Utopia runtime. The project aims to make cross-language development simple and accessible.

## Features
- **Multi-language compilation**: Convert Utopia code to Python, JavaScript, C, Rust, and more
- **Direct execution**: Run Utopia code natively (no need to compile to another language)
- **Interactive REPL**: Test and experiment with code interactively
- **Cross-language calls**: Call functions between different language blocks
- **Type safety**: Compile-time type checking

## Installation

```bash
# Clone the repository
git clone https://github.com/WinsonEdwards/Utopia.git
cd Utopia/utopia-rs

# Build the compiler
cargo build --release

# (Optional) Install globally
cargo install --path .
```

## Usage

### Run a Utopia program directly
```bash
utopia run path/to/file.uto
```

### Compile to a target language
```bash
utopia compile path/to/file.uto --target python --output out.py
utopia compile path/to/file.uto --target javascript --output out.js
```

### Start the REPL
```bash
utopia repl
```

## CLI Commands

- `compile` — Compile Utopia code to a target language
- `run` — Run Utopia code directly
- `repl` — Start the interactive shell
- `analyze` — Analyze code for errors and warnings
- `check` — Type-check code
- `format` — Format Utopia code
- `info` — Show project/language info
- `benchmark` — Run performance benchmarks
- `new` — Create a new Utopia project
- `clean` — Remove build artifacts

See `docs/cli-reference.md` for full details and options.

## Documentation
- [Language Reference](docs/language-reference.md)
- [CLI Reference](docs/cli-reference.md)
- [Examples](docs/examples.md)

## Contributing

Contributions are welcome! Please see `CONTRIBUTING.md` for guidelines.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
