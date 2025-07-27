# Utopia Language Reference

Technical reference for the Utopia programming language syntax and features.

## Language Overview

Utopia is a multi-language compiler that uses a unified syntax to generate code in multiple target languages. The language combines familiar C-like syntax with features that enable cross-language integration.

### Design Goals

- Provide a unified syntax for multi-language programming
- Enable seamless function calls between different programming languages
- Generate idiomatic code for each target language
- Maintain type safety across language boundaries

## Basic Syntax

### Comments

```utopia
// Single-line comment

/*
   Multi-line comment
*/
```

### Variables

```utopia
let name = "John"
let age = 30
let isActive = true
let numbers = [1, 2, 3, 4, 5]
```

### Functions

```utopia
function greet(name) {
    return "Hello, " + name
}

function add(a, b) {
    return a + b
}
```

### Control Flow

#### If Statements

```utopia
if age >= 18 {
    println("Adult")
} else {
    println("Minor")
}
```

#### Loops

```utopia
// For loop
for i in range(0, 10) {
    println(i)
}

// While loop
while x < 100 {
    x = x * 2
}
```

## Data Types

### Primitive Types

- `number` - Numeric values (integers and floats)
- `string` - Text strings
- `boolean` - true/false values
- `null` - Null/undefined value

### Composite Types

- `array` - Ordered collections of values
- `object` - Key-value mappings

### Examples

```utopia
let age = 25                    // number
let name = "Alice"              // string
let isStudent = true            // boolean
let nothing = null              // null
let scores = [95, 87, 92]       // array
let person = {                  // object
    name: "Bob",
    age: 30
}
```

## Language Blocks

Utopia supports language-specific code blocks for cross-language integration:

```utopia
@lang python {
def analyze_data(data):
    return sum(data) / len(data)
}

@lang javascript {
function formatCurrency(amount) {
    return `$${amount.toFixed(2)}`;
}
}

// Cross-language function calls
let data = [10, 20, 30, 40, 50]
let average = python::analyze_data(data)
let formatted = javascript::formatCurrency(average)
println(formatted)
```

## Operators

### Arithmetic Operators

- `+` - Addition
- `-` - Subtraction
- `*` - Multiplication
- `/` - Division
- `%` - Modulo

### Comparison Operators

- `==` - Equal to
- `!=` - Not equal to
- `<` - Less than
- `>` - Greater than
- `<=` - Less than or equal to
- `>=` - Greater than or equal to

### Logical Operators

- `&&` - Logical AND
- `||` - Logical OR
- `!` - Logical NOT

### Assignment Operators

- `=` - Assignment
- `+=` - Add and assign
- `-=` - Subtract and assign
- `*=` - Multiply and assign
- `/=` - Divide and assign

## Built-in Functions

### Input/Output

```utopia
println("Hello, World!")       // Print with newline
print("Hello")                 // Print without newline
```

### Type Conversion

```utopia
toString(42)                   // Convert to string
toNumber("42")                 // Convert to number
toBoolean(1)                   // Convert to boolean
```

### Array Functions

```utopia
len(array)                     // Get array length
push(array, item)              // Add item to array
pop(array)                     // Remove last item
```

## Error Handling

Utopia uses a simple error handling mechanism:

```utopia
function divide(a, b) {
    if b == 0 {
        throw "Division by zero"
    }
    return a / b
}

try {
    let result = divide(10, 0)
    println(result)
} catch error {
    println("Error:", error)
}
```

## Target Language Compatibility

### Supported Languages

Utopia can compile to the following target languages:

**Systems:** C, C++, Rust, Go, Assembly  
**Web:** JavaScript, TypeScript, WebAssembly  
**Enterprise:** Java, C#, Kotlin, Scala  
**Scripting:** Python, Ruby, PHP, Perl  
**Functional:** Haskell, OCaml, Lisp, Clojure  
**Scientific:** R, MATLAB, Julia, Fortran  
**Other:** COBOL, Ada, Prolog, SQL

### Language-Specific Features

When compiling to specific languages, Utopia generates idiomatic code:

- **Python**: Uses list comprehensions, proper indentation
- **JavaScript**: Uses modern ES6+ features
- **Rust**: Implements proper ownership and borrowing
- **C**: Manages memory allocation explicitly

## Memory Management

Memory management varies by target language:

- **Garbage Collected** (Python, JavaScript, Java): Automatic memory management
- **Manual Management** (C, C++): Explicit allocation and deallocation
- **Ownership-based** (Rust): Compile-time memory safety
- **Reference Counting** (Swift): Automatic reference counting

## Type System

Utopia uses optional typing with type inference:

```utopia
// Explicit typing
let name: string = "Alice"
let age: number = 30

// Type inference
let city = "New York"  // Inferred as string
let count = 42         // Inferred as number

// Function with types
function calculate(x: number, y: number): number {
    return x + y
}
```

## Best Practices

### Code Organization

- Use meaningful variable and function names
- Keep functions focused and reasonably sized
- Group related functionality together
- Use consistent naming conventions

### Cross-Language Integration

- Use language blocks for language-specific optimizations
- Keep cross-language interfaces simple
- Handle type conversions explicitly
- Test cross-language functionality thoroughly

### Performance

- Use appropriate target languages for different components
- Minimize cross-language calls in performance-critical code
- Consider target language characteristics when writing code
- Profile and optimize for your specific use case 