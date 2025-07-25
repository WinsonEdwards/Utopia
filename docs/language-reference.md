# Utopia Language Reference

## Overview

Utopia is a multi-language programming language that can compile to various target languages. It provides a unified syntax while allowing language-specific code blocks.

## File Extension

Utopia source files use the `.uto` extension.

## Basic Syntax

### Language Blocks

Code is organized into language-specific blocks:

```utopia
@lang python {
    def greet(name):
        return f"Hello, {name}!"
}

@lang javascript {
    function process(data) {
        return data.map(x => x * 2);
    }
}

@lang main {
    // Main program logic
}
```

### Variables

```utopia
let x = 10;
let message = "Hello";
let isActive = true;
```

### Functions

```utopia
function add(a, b) {
    return a + b;
}

function greet(name) {
    println("Hello,", name);
}
```

### Control Flow

```utopia
if (x > 5) {
    println("x is greater than 5");
} else {
    println("x is 5 or less");
}

let i = 0;
while (i < 10) {
    println("Count:", i);
    i = i + 1;
}
```

### Arrays and Objects

```utopia
let numbers = [1, 2, 3, 4, 5];
let person = {
    name: "Alice",
    age: 30
};
```

### Cross-Language Calls

```utopia
@lang python {
    def python_function(x):
        return x * 2
}

@lang main {
    let result = python::python_function(5);
    println("Result:", result);
}
```

## Built-in Functions

- `println(...)` - Print with newline
- `print(...)` - Print without newline
- `len(array)` - Get array length
- `type_of(value)` - Get value type

## Comments

```utopia
// Single line comment
/* Multi-line
   comment */
```

## Type System

Utopia uses dynamic typing with type inference:

- Numbers: `42`, `3.14`
- Strings: `"hello"`
- Booleans: `true`, `false`
- Arrays: `[1, 2, 3]`
- Objects: `{key: "value"}`
- Null: `null` 