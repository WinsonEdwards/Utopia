# Utopia Language Syntax Guide

Utopia is a modern, unified programming language designed for multi-language compilation. This guide covers the complete syntax and language features of Utopia.

## Table of Contents

1. [Basic Syntax](#basic-syntax)
2. [Variables and Data Types](#variables-and-data-types)
3. [Operators](#operators)
4. [Control Flow](#control-flow)
5. [Functions](#functions)
6. [Data Structures](#data-structures)
7. [Error Handling](#error-handling)
8. [Modules and Imports](#modules-and-imports)
9. [Comments](#comments)
10. [Examples](#examples)

## Basic Syntax

### Program Structure

Every Utopia program starts with optional imports and contains statements:

```utopia
// Import statements (optional)
import "module_name"

// Main program code
println("Hello, Utopia!")
```

### Statements

Statements in Utopia end with semicolons or newlines:

```utopia
x = 5;           // With semicolon
y = 10           // Without semicolon (newline)
println(x + y)   // Function call
```

## Variables and Data Types

### Variable Declaration

Variables are declared using `let` or direct assignment:

```utopia
// Explicit declaration
let name: string = "Utopia"
let version: number = 1.0
let isAwesome: boolean = true

// Type inference
name = "Utopia"      // string
version = 1.0        // number
isAwesome = true     // boolean
```

### Data Types

Utopia supports the following basic types:

#### Numbers
```utopia
let integer: number = 42
let float: number = 3.14
let negative: number = -10
```

#### Strings
```utopia
let message: string = "Hello, World!"
let multiLine: string = """
    This is a
    multi-line string
"""
```

#### Booleans
```utopia
let isTrue: boolean = true
let isFalse: boolean = false
```

#### Arrays
```utopia
let numbers: number[] = [1, 2, 3, 4, 5]
let mixed: any[] = [1, "hello", true]
```

#### Objects
```utopia
let person: object = {
    name: "John",
    age: 30,
    city: "New York"
}
```

## Operators

### Arithmetic Operators
```utopia
let a: number = 10
let b: number = 3

let sum = a + b        // 13
let difference = a - b // 7
let product = a * b    // 30
let quotient = a / b   // 3.333...
let remainder = a % b  // 1
let power = a ** b     // 1000
```

### Comparison Operators
```utopia
let x: number = 5
let y: number = 10

let equal = x == y        // false
let notEqual = x != y     // true
let lessThan = x < y      // true
let greaterThan = x > y   // false
let lessEqual = x <= y    // true
let greaterEqual = x >= y // false
```

### Logical Operators
```utopia
let a: boolean = true
let b: boolean = false

let and = a && b    // false
let or = a || b     // true
let not = !a        // false
```

### Assignment Operators
```utopia
let x: number = 5

x += 3   // x = 8
x -= 2   // x = 6
x *= 4   // x = 24
x /= 3   // x = 8
x %= 3   // x = 2
```

## Control Flow

### If Statements
```utopia
let age: number = 18

if age >= 18 {
    println("You are an adult")
} else if age >= 13 {
    println("You are a teenager")
} else {
    println("You are a child")
}
```

### Switch Statements
```utopia
let day: string = "Monday"

switch day {
    case "Monday":
        println("Start of the week")
    case "Friday":
        println("TGIF!")
    default:
        println("Regular day")
}
```

### Loops

#### For Loop
```utopia
// Traditional for loop
for let i = 0; i < 5; i++ {
    println("Count:", i)
}

// For-in loop (arrays)
let fruits: string[] = ["apple", "banana", "orange"]
for fruit in fruits {
    println("I like", fruit)
}

// For-in loop (objects)
let person: object = {name: "John", age: 30}
for key, value in person {
    println(key, ":", value)
}
```

#### While Loop
```utopia
let count: number = 0
while count < 5 {
    println("Count:", count)
    count++
}
```

#### Do-While Loop
```utopia
let i: number = 0
do {
    println("Value:", i)
    i++
} while i < 3
```

## Functions

### Function Declaration
```utopia
// Basic function
function greet(name: string): string {
    return "Hello, " + name + "!"
}

// Function with multiple parameters
function add(a: number, b: number): number {
    return a + b
}

// Function with default parameters
function greetWithTitle(name: string, title: string = "Mr."): string {
    return "Hello, " + title + " " + name + "!"
}

// Function with rest parameters
function sum(...numbers: number[]): number {
    let total: number = 0
    for num in numbers {
        total += num
    }
    return total
}
```

### Function Calls
```utopia
let message = greet("Utopia")
let result = add(5, 3)
let greeting = greetWithTitle("Smith")
let total = sum(1, 2, 3, 4, 5)
```

### Anonymous Functions
```utopia
let multiply = function(a: number, b: number): number {
    return a * b
}

let result = multiply(4, 5) // 20
```

### Arrow Functions
```utopia
let square = (x: number): number => x * x
let add = (a: number, b: number): number => a + b

let result1 = square(5)  // 25
let result2 = add(3, 4)  // 7
```

## Data Structures

### Arrays
```utopia
// Array declaration
let numbers: number[] = [1, 2, 3, 4, 5]
let names: string[] = ["Alice", "Bob", "Charlie"]

// Array methods
let length = numbers.length
let first = numbers[0]
let last = numbers[numbers.length - 1]

// Array operations
numbers.push(6)           // Add to end
numbers.pop()             // Remove from end
numbers.unshift(0)        // Add to beginning
numbers.shift()           // Remove from beginning
```

### Objects
```utopia
// Object declaration
let person: object = {
    name: "John Doe",
    age: 30,
    city: "New York",
    hobbies: ["reading", "coding"]
}

// Accessing properties
let name = person.name
let age = person["age"]

// Adding properties
person.email = "john@example.com"
person["phone"] = "123-456-7890"
```

## Error Handling

### Try-Catch Blocks
```utopia
try {
    let result = divide(10, 0)
    println("Result:", result)
} catch error {
    println("Error occurred:", error.message)
} finally {
    println("Cleanup code here")
}
```

### Custom Errors
```utopia
function validateAge(age: number): void {
    if age < 0 {
        throw new Error("Age cannot be negative")
    }
    if age > 150 {
        throw new Error("Age seems unrealistic")
    }
}
```

## Modules and Imports

### Importing Modules
```utopia
// Import entire module
import "math"

// Import specific functions
import { sqrt, pow } from "math"

// Import with alias
import { sqrt as squareRoot } from "math"
```

### Creating Modules
```utopia
// math.uto
export function add(a: number, b: number): number {
    return a + b
}

export function multiply(a: number, b: number): number {
    return a * b
}

export const PI: number = 3.14159
```

## Comments

### Single-line Comments
```utopia
// This is a single-line comment
let x = 5 // Inline comment
```

### Multi-line Comments
```utopia
/*
This is a multi-line comment
It can span multiple lines
Useful for documentation
*/
```

### Documentation Comments
```utopia
/**
 * Calculates the factorial of a number
 * @param n The number to calculate factorial for
 * @returns The factorial of n
 */
function factorial(n: number): number {
    if n <= 1 {
        return 1
    }
    return n * factorial(n - 1)
}
```

## Examples

### Complete Program Example
```utopia
// Simple calculator program
function calculate(operation: string, a: number, b: number): number {
    switch operation {
        case "add":
            return a + b
        case "subtract":
            return a - b
        case "multiply":
            return a * b
        case "divide":
            if b == 0 {
                throw new Error("Division by zero")
            }
            return a / b
        default:
            throw new Error("Unknown operation")
    }
}

// Main program
let num1: number = 10
let num2: number = 5

println("Addition:", calculate("add", num1, num2))
println("Subtraction:", calculate("subtract", num1, num2))
println("Multiplication:", calculate("multiply", num1, num2))
println("Division:", calculate("divide", num1, num2))
```

### Class-like Structure (using objects)
```utopia
// Person object with methods
let person: object = {
    name: "John Doe",
    age: 30,
    
    greet: function(): string {
        return "Hello, my name is " + this.name
    },
    
    haveBirthday: function(): void {
        this.age++
        println("Happy birthday! You are now", this.age, "years old")
    }
}

println(person.greet())
person.haveBirthday()
```

This syntax guide covers all the essential features of the Utopia programming language. The language is designed to be intuitive and familiar to developers while providing powerful features for multi-language compilation. 