# Utopia CLI Reference

## Overview

The Utopia CLI lets you compile, run, and manage code in 50 programming languages from a single tool.

## Main Commands

| Command   | Description                                  |
|-----------|----------------------------------------------|
| compile   | Compile Utopia code to a target language     |
| run       | Run Utopia code directly                     |
| repl      | Start the interactive shell                  |
| analyze   | Analyze code for errors and warnings         |
| check     | Type-check code                              |
| format    | Format Utopia code                          |
| info      | Show project/language info                   |
| benchmark | Run performance benchmarks                   |
| new       | Create a new Utopia project                  |
| clean     | Remove build artifacts                       |

## Compiling to a Target Language

```bash
utopia compile file.uto --target python --output out.py
utopia compile file.uto --target c --output out.c
utopia compile file.uto --target rust --output out.rs
utopia compile file.uto --target javascript --output out.js
```

## Supported Language Targets

- C, C++, Rust, Go, Zig
- Python, JavaScript, TypeScript, Java, C#, Kotlin, Swift
- Haskell, Clojure, F#, Lisp, Scheme, OCaml, Erlang, Elixir
- Perl, PHP, Ruby, Lua, Bash, VBScript
- R, MATLAB, Julia, Fortran
- COBOL, Ada, Delphi, Visual Basic
- SQL, Prolog
- Racket, Smalltalk, Pascal, BASIC
- Dart, Scala, Nim, Crystal, Objective-C
- x86 ASM, LLVM IR, WebAssembly, CUDA, Embedded C

## Examples

```bash
utopia compile myprog.uto --target python --output myprog.py
utopia compile myprog.uto --target c --output myprog.c
utopia run myprog.uto
utopia repl
```

See [docs/language-reference.md](language-reference.md) for language details. 