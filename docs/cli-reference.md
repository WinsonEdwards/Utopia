# Utopia CLI Reference

## Overview

The Utopia command-line interface provides tools for compiling, running, and managing Utopia code.

## Installation

```bash
# Build from source
cd utopia-rs
cargo build --release

# Install globally
cargo install --path .
```

## Commands

### compile

Compile Utopia code to a target language.

```bash
utopia compile <file> --target <language> [options]
```

**Arguments:**
- `file` - Input Utopia file (.uto)

**Options:**
- `--target <language>` - Target language (python, javascript, c, rust, etc.)
- `--output <file>` - Output file path
- `--verbose` - Enable verbose output

**Examples:**
```bash
utopia compile app.uto --target python
utopia compile app.uto --target javascript --output app.js
utopia compile app.uto --target python --verbose
```

### run

Execute Utopia code directly without compilation.

```bash
utopia run <file> [options]
```

**Arguments:**
- `file` - Utopia file to execute (.uto)

**Options:**
- `--verbose` - Enable verbose output

**Examples:**
```bash
utopia run app.uto
utopia run app.uto --verbose
```

### repl

Start an interactive REPL (Read-Eval-Print Loop).

```bash
utopia repl [options]
```

**Options:**
- `--verbose` - Enable verbose output

**Examples:**
```bash
utopia repl
utopia repl --verbose
```

### analyze

Analyze code structure and dependencies.

```bash
utopia analyze <file> [options]
```

**Arguments:**
- `file` - Utopia file to analyze (.uto)

**Examples:**
```bash
utopia analyze app.uto
```

### check

Type check Utopia code.

```bash
utopia check <file> [options]
```

**Arguments:**
- `file` - Utopia file to check (.uto)

**Examples:**
```bash
utopia check app.uto
```

### format

Format Utopia code.

```bash
utopia format <file> [options]
```

**Arguments:**
- `file` - Utopia file to format (.uto)

**Examples:**
```bash
utopia format app.uto
```

### info

Show compiler information.

```bash
utopia info
```

### benchmark

Run performance benchmarks.

```bash
utopia benchmark <file> [options]
```

**Arguments:**
- `file` - Utopia file to benchmark (.uto)

**Examples:**
```bash
utopia benchmark app.uto
```

### new

Create a new Utopia project.

```bash
utopia new <project-name> [options]
```

**Arguments:**
- `project-name` - Name of the new project

**Examples:**
```bash
utopia new my-project
```

### clean

Clean build artifacts.

```bash
utopia clean
```

### help

Show help information.

```bash
utopia --help
utopia <command> --help
```

## Global Options

- `--verbose` - Enable verbose output
- `--quiet` - Suppress output
- `--version` - Show version information

## Environment Variables

- `UTOPIA_TARGET` - Default target language
- `UTOPIA_OUTPUT_DIR` - Default output directory 