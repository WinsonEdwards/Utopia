# Utopia CLI Reference

Command-line interface documentation for the Utopia compiler.

## Installation

### Prerequisites

- Rust (latest stable version)
- Git
- Cargo package manager

### Build and Install

```bash
git clone https://github.com/WinsonEdwards/Utopia.git
cd Utopia/utopia-rs
cargo install --path .
```

### Verify Installation

```bash
utopia --version
```

## Basic Commands

### compile

Compile Utopia source code to a target language.

```bash
utopia compile <input_file> --target <language> [options]
```

**Examples:**
```bash
utopia compile hello.uto --target python
utopia compile app.uto --target javascript --output app.js
utopia compile main.uto --target rust --verbose
```

**Options:**
- `--target <language>` - Target language (required)
- `--output <file>` - Output file path
- `--verbose` - Enable verbose output
- `--optimize` - Enable optimizations

### run

Compile and execute Utopia code directly.

```bash
utopia run <input_file> [options]
```

**Examples:**
```bash
utopia run hello.uto
utopia run app.uto --target python
```

### repl

Start an interactive Read-Eval-Print Loop.

```bash
utopia repl [options]
```

**Options:**
- `--language <lang>` - Set default target language
- `--completion` - Enable tab completion

### help

Display help information.

```bash
utopia help [command]
```

## Supported Target Languages

### Systems Languages
- `c` - C
- `cpp` - C++
- `rust` - Rust
- `go` - Go
- `assembly` - Assembly

### Web Technologies
- `javascript` - JavaScript
- `typescript` - TypeScript
- `webassembly` - WebAssembly

### Enterprise Languages
- `java` - Java
- `csharp` - C#
- `kotlin` - Kotlin
- `scala` - Scala

### Scripting Languages
- `python` - Python
- `ruby` - Ruby
- `php` - PHP
- `perl` - Perl

### Functional Languages
- `haskell` - Haskell
- `ocaml` - OCaml
- `lisp` - Lisp
- `clojure` - Clojure

### Scientific Languages
- `r` - R
- `matlab` - MATLAB
- `julia` - Julia
- `fortran` - Fortran

### Other Languages
- `cobol` - COBOL
- `ada` - Ada
- `prolog` - Prolog
- `sql` - SQL

## Configuration

### Config File

Create a `utopia.toml` file in your project directory:

```toml
[compiler]
default_target = "python"
optimize = true
verbose = false

[output]
directory = "./output"
preserve_comments = true
```

### Environment Variables

- `UTOPIA_TARGET` - Default target language
- `UTOPIA_OUTPUT_DIR` - Default output directory
- `UTOPIA_VERBOSE` - Enable verbose output (true/false)

## Examples

### Basic Compilation

```bash
# Compile to Python
utopia compile hello.uto --target python

# Compile with custom output name
utopia compile app.uto --target javascript --output myapp.js

# Compile with optimizations
utopia compile main.uto --target rust --optimize
```

### Advanced Usage

```bash
# Verbose compilation with benchmarking
utopia compile complex.uto --target cpp --verbose --benchmark

# Multiple targets (requires script)
for lang in python javascript rust; do
    utopia compile app.uto --target $lang --output "app.$lang"
done
```

### REPL Examples

```bash
# Start REPL with Python as default target
utopia repl --language python

# Interactive session
utopia repl
> let x = 42
> println("The answer is:", x)
> exit
```

## Troubleshooting

### Common Issues

**Command not found**
- Ensure `cargo install --path .` completed successfully
- Check that `~/.cargo/bin` is in your PATH

**Compilation errors**
- Verify source file syntax with `utopia compile --check-syntax`
- Use `--verbose` flag for detailed error information

**Target language not supported**
- Check supported languages with `utopia help compile`
- Verify spelling of target language name

### Getting Help

- Run `utopia help` for general help
- Run `utopia help <command>` for command-specific help
- Check GitHub issues for known problems
- Create new issue for bugs or feature requests 