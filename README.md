# Utopia - Multi-Language Compiler

Utopia is a Rust-based compiler that lets you write code in a unified syntax and compile it to multiple programming languages, or run it directly using the Utopia runtime. The project aims to make cross-language development simple and accessible.

## Supported Programming Languages

Utopia currently supports compiling to and integrating with the following languages:

| Category         | Languages                                                                 |
|------------------|--------------------------------------------------------------------------|
| Systems          | C, C++, Rust, Go, Zig                                                    |
| Modern           | Python, JavaScript, TypeScript, Java, C#, Kotlin, Swift                  |
| Functional       | Haskell, Clojure, F#, Lisp, Scheme, OCaml, Erlang, Elixir                |
| Scripting        | Perl, PHP, Ruby, Lua, Bash, VBScript                                     |
| Scientific       | R, MATLAB, Julia, Fortran                                                |
| Enterprise       | COBOL, Ada, Delphi, Visual Basic                                         |
| Data/Logic       | SQL, Prolog                                                              |
| Academic         | Racket, Smalltalk, Pascal, BASIC                                         |
| Specialized      | Dart, Scala, Nim, Crystal, Objective-C                                   |
| Assembly/LowLvl  | x86 ASM, LLVM IR, WebAssembly, CUDA, Embedded C                          |

**Total: 50 programming languages supported!**

## Features
- **Multi-language compilation**: Convert Utopia code to 50+ languages
- **Direct execution**: Run Utopia code natively (no need to compile to another language)
- **Interactive REPL**: Test and experiment with code interactively
- **Cross-language calls**: Call functions between different language blocks
- **Type safety**: Compile-time type checking
- **High performance**: Fast compilation and execution

## Performance

- **Compilation speed**: <200ms for most targets
- **Runtime performance**: ~95% of native Rust
- **Cross-language call overhead**: <1ms per call
- **Memory usage**: 2-5MB typical

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

## Documentation
- [Language Reference](docs/language-reference.md)
- [CLI Reference](docs/cli-reference.md)
- [Examples](docs/examples.md)
- [Performance Guide](docs/performance.md)
- [utopia language guide](docs/utopia-language-guide.md)

## Contributing

Contributions are welcome! Please see `CONTRIBUTING.md` for guidelines.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
