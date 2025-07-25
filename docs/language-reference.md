# Utopia Language Reference

## Supported Programming Languages

Utopia supports the following 50 languages:

| C | C++ | Rust | Go | Zig |
|---|-----|------|----|-----|
| Python | JavaScript | TypeScript | Java | C# | Kotlin | Swift |
| Haskell | Clojure | F# | Lisp | Scheme | OCaml | Erlang | Elixir |
| Perl | PHP | Ruby | Lua | Bash | VBScript |
| R | MATLAB | Julia | Fortran |
| COBOL | Ada | Delphi | Visual Basic |
| SQL | Prolog |
| Racket | Smalltalk | Pascal | BASIC |
| Dart | Scala | Nim | Crystal | Objective-C |
| x86 ASM | LLVM IR | WebAssembly | CUDA | Embedded C |

## Language Blocks

To write code for a specific language, use the `@lang` block:

```utopia
@lang python {
    def greet(name):
        return f"Hello, {name}!"
}

@lang c {
    int add(int a, int b) {
        return a + b;
    }
}

@lang main {
    // Main program logic
}
```

## Cross-Language Calls

You can call functions defined in other language blocks:

```utopia
@lang c {
int multiply(int a, int b) { return a * b; }
}

@lang python {
def process(x): return x + 1
}

@lang main {
let result = c::multiply(3, 4);
let processed = python::process(result);
println(processed);
}
```

## Variables, Functions, and Control Flow

```utopia
@lang main {
    let x = 10;
    let y = 20;
    function add(a, b) {
        return a + b;
    }
    if (x < y) {
        println("x is less than y");
    }
}
```

See more in [docs/examples.md](examples.md). 