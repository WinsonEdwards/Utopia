# Utopia

A multi-language compiler that supports syntax transformation across 50+ programming languages.

## Overview

Utopia is an experimental compiler that parses a unified syntax and generates equivalent code in multiple target languages. The project demonstrates cross-language syntax transformation using a shared Abstract Syntax Tree (AST).

## Features

- Support for 50+ programming languages including Python, JavaScript, C++, Rust, Go, Java
- Unified syntax that transforms to idiomatic code in each target language
- Command-line interface for compilation and execution
- Cross-language function syntax demonstration

## Installation

### Prerequisites

- Rust (latest stable version)
- Cargo package manager

### Build from Source

```bash
git clone https://github.com/WinsonEdwards/Utopia.git
cd Utopia/utopia-rs
cargo build --release
cargo install --path .
```

## Usage

### Basic Compilation

```bash
# Compile to Python
utopia compile hello_world.uto --target python

# Compile to JavaScript
utopia compile hello_world.uto --target javascript

# Compile to C++
utopia compile hello_world.uto --target cpp
```

### Example Syntax

```utopia
let message = "Hello, World"
let numbers = [1, 2, 3, 4, 5]

function greet(name) {
    return "Hello, " + name
}

for i in numbers {
    println(greet("User" + i))
}
```

This generates appropriate syntax for each target language while maintaining semantic equivalence.

## Supported Languages

The compiler currently supports syntax transformation for:

**Systems Languages:** C, C++, Rust, Go, Assembly  
**Web Technologies:** JavaScript, TypeScript, WebAssembly  
**Enterprise:** Java, C#, Kotlin, Scala  
**Scripting:** Python, Ruby, PHP, Perl  
**Functional:** Haskell, OCaml, Lisp, Clojure  
**Scientific:** R, MATLAB, Julia, Fortran  
**Other:** COBOL, Ada, Prolog, SQL, and more

## Documentation

- [CLI Reference](docs/cli-reference.md) - Command-line usage
- [Language Reference](docs/language-reference.md) - Syntax documentation
- [Contributing](CONTRIBUTING.md) - Development guidelines

## Project Status

This is an experimental project exploring multi-language compilation concepts. The compiler handles basic syntax transformation but does not implement full semantic analysis or runtime integration that production compilers require.

## Contributing

Contributions are welcome. Please read the contributing guidelines before submitting pull requests.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
