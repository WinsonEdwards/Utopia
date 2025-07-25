# 📝 **Utopia Syntax Reference**

Complete syntax documentation for the Utopia Multi-Language Compiler with practical examples and detailed explanations.

---

## 📋 **Table of Contents**

- [Basic Syntax](#-basic-syntax)
- [Data Types](#-data-types)
- [Variables and Constants](#-variables-and-constants)
- [Operators](#-operators)
- [Control Flow](#-control-flow)
- [Functions](#-functions)
- [Data Structures](#-data-structures)
- [Language Blocks](#-language-blocks)
- [Cross-Language Calls](#-cross-language-calls)
- [Comments](#-comments)
- [Advanced Features](#-advanced-features)

---

## 🔤 **Basic Syntax**

### **Program Structure**
Every Utopia program consists of language blocks and/or global statements:

```utopia
// Global statements (executed in main context)
let greeting = "Hello, World!"
println(greeting)

// Language-specific blocks
@lang python
def calculate_fibonacci(n):
    if n <= 1:
        return n
    return calculate_fibonacci(n-1) + calculate_fibonacci(n-2)

@lang javascript
function formatNumber(num) {
    return num.toLocaleString();
}

// Main execution block
@lang main
let result = py::calculate_fibonacci(10)
let formatted = js::formatNumber(result)
println("Fibonacci 10:", formatted)
```

### **Case Sensitivity**
Utopia is **case-sensitive**:
```utopia
let myVariable = 10     // Different from
let MyVariable = 20     // this variable
let MYVARIABLE = 30     // and this one
```

### **Statement Termination**
Statements are terminated with semicolons (`;`):
```utopia
let x = 5;              // Required
println("Hello");       // Required
function test() { }     // Optional for blocks
```

---

## 🏗️ **Data Types**

### **Primitive Types**

#### **Number**
All numbers are 64-bit floating point (IEEE 754):
```utopia
let integer = 42
let decimal = 3.14159
let negative = -17
let zero = 0
let scientific = 1e6        // 1,000,000 (planned)
let hex = 0xFF             // 255 (planned)
let binary = 0b1010        // 10 (planned)
```

#### **String**
UTF-8 encoded text with single or double quotes:
```utopia
let singleQuote = 'Hello, World!'
let doubleQuote = "Hello, Universe!"
let empty = ""
let multiline = "Line 1\nLine 2"
let escaped = "Quote: \"Hello\""
let unicode = "🚀 Utopia"
```

**String Escape Sequences:**
```utopia
"\n"    // Newline
"\t"    // Tab
"\r"    // Carriage return
"\\"    // Backslash
"\""    // Double quote
"\'"    // Single quote
"\0"    // Null character (planned)
```

#### **Boolean**
True or false values:
```utopia
let isTrue = true
let isFalse = false
let result = (5 > 3)        // true
let comparison = (10 == 5)  // false
```

#### **Null**
Represents absence of value:
```utopia
let empty = null
let uninitialized = null
function maybeReturn(condition) {
    if (condition) {
        return "value"
    }
    return null  // Explicit null return
}
```

---

## 📦 **Variables and Constants**

### **Variable Declaration**
Use `let` for mutable variables:
```utopia
let count = 0
let name = "Alice"
let isActive = true

// Variables can be reassigned
count = 10
name = "Bob"
isActive = false
```

### **Constant Declaration**
Use `const` for immutable values:
```utopia
const PI = 3.14159
const MAX_USERS = 100
const APP_NAME = "Utopia Compiler"

// Constants cannot be reassigned
// PI = 3.14  // Error: Cannot reassign constant
```

### **Type Annotations** (Optional)
Explicitly specify types for clarity:
```utopia
let age: number = 25
let username: string = "developer"
let isLoggedIn: boolean = false
let data: null = null

// Arrays
let numbers: number[] = [1, 2, 3, 4, 5]
let names: string[] = ["Alice", "Bob", "Charlie"]

// Function types
let calculator: (number, number) -> number = add
```

### **Type Inference**
Utopia automatically infers types:
```utopia
let count = 42          // Inferred as number
let message = "Hello"   // Inferred as string
let flag = true         // Inferred as boolean
let items = [1, 2, 3]   // Inferred as number[]
```

---

## ⚙️ **Operators**

### **Arithmetic Operators**
```utopia
let a = 10
let b = 3

let addition = a + b        // 13
let subtraction = a - b     // 7
let multiplication = a * b  // 30
let division = a / b        // 3.333...
let modulo = a % b          // 1

// Unary operators
let positive = +a           // 10
let negative = -a           // -10
```

### **Assignment Operators**
```utopia
let x = 10

x += 5      // x = x + 5  (15)
x -= 3      // x = x - 3  (12)
x *= 2      // x = x * 2  (24)
x /= 4      // x = x / 4  (6)
x %= 5      // x = x % 5  (1)
```

### **Increment/Decrement Operators**
```utopia
let counter = 0

counter++   // Post-increment: counter = 1
counter--   // Post-decrement: counter = 0

// In expressions
let arr = [1, 2, 3]
for (let i = 0; i < arr.length; i++) {
    println(arr[i])
}
```

### **Comparison Operators**
```utopia
let a = 5
let b = 10

let equal = (a == b)        // false
let notEqual = (a != b)     // true
let less = (a < b)          // true
let greater = (a > b)       // false
let lessEqual = (a <= b)    // true
let greaterEqual = (a >= b) // false
```

### **Logical Operators**
```utopia
let x = true
let y = false

let and = x && y            // false
let or = x || y             // true
let not = !x                // false

// Short-circuit evaluation
let result = x && someFunction()  // someFunction() only called if x is true
let value = y || getDefault()    // getDefault() only called if y is false
```

### **Operator Precedence** (Highest to Lowest)
```utopia
1. () [] .                  // Grouping, array access, member access
2. ! + - (unary)           // Logical NOT, unary plus/minus
3. * / %                   // Multiplication, division, modulo
4. + -                     // Addition, subtraction
5. < <= > >=               // Relational operators
6. == !=                   // Equality operators
7. &&                      // Logical AND
8. ||                      // Logical OR
9. = += -= *= /= %=        // Assignment operators
```

---

## 🔄 **Control Flow**

### **If Statements**
```utopia
let age = 18

if (age >= 18) {
    println("You are an adult")
}

if (age >= 21) {
    println("You can drink alcohol")
} else {
    println("You cannot drink alcohol")
}

// Multi-branch if-else
let score = 85
if (score >= 90) {
    println("Grade: A")
} else if (score >= 80) {
    println("Grade: B")
} else if (score >= 70) {
    println("Grade: C")
} else {
    println("Grade: F")
}
```

### **While Loops**
```utopia
let count = 0
while (count < 5) {
    println("Count:", count)
    count++
}

// Infinite loop with break
let running = true
while (running) {
    let input = getInput()
    if (input == "quit") {
        running = false
    }
    processInput(input)
}
```

### **For Loops**
```utopia
// Traditional for loop
for (let i = 0; i < 10; i++) {
    println("Iteration:", i)
}

// For loop with array
let numbers = [1, 2, 3, 4, 5]
for (let i = 0; i < numbers.length; i++) {
    println("Number:", numbers[i])
}

// For loop with step
for (let i = 0; i < 100; i += 10) {
    println("Step:", i)
}

// Nested for loops
for (let row = 0; row < 3; row++) {
    for (let col = 0; col < 3; col++) {
        println("Position:", row, col)
    }
}
```

---

## 🔧 **Functions**

### **Function Declaration**
```utopia
// Basic function
function greet() {
    println("Hello, World!")
}

// Function with parameters
function add(a, b) {
    return a + b
}

// Function with type annotations
function multiply(x: number, y: number): number {
    return x * y
}

// Function with default parameters (planned)
function greetUser(name: string = "Guest"): string {
    return "Hello, " + name + "!"
}
```

### **Function Calls**
```utopia
greet()                     // Call function with no parameters
let sum = add(5, 3)         // Call function with parameters
let product = multiply(4, 7) // Call function with type annotations

// Store function in variable
let calculator = add
let result = calculator(10, 20)
```

### **Return Statements**
```utopia
function absoluteValue(x) {
    if (x < 0) {
        return -x
    }
    return x
}

// Multiple return points
function findMax(a, b, c) {
    if (a >= b && a >= c) return a
    if (b >= c) return b
    return c
}

// No return value (void function)
function logMessage(message) {
    println("[LOG]", message)
    // Implicit return null
}
```

### **Function Scope**
```utopia
let globalVar = "global"

function outerFunction() {
    let outerVar = "outer"
    
    function innerFunction() {
        let innerVar = "inner"
        println(globalVar)  // Can access global
        println(outerVar)   // Can access outer
        println(innerVar)   // Can access inner
    }
    
    innerFunction()
    // innerVar is not accessible here
}

outerFunction()
// outerVar and innerVar are not accessible here
```

---

## 📊 **Data Structures**

### **Arrays**
```utopia
// Array creation
let numbers = [1, 2, 3, 4, 5]
let names = ["Alice", "Bob", "Charlie"]
let mixed = [1, "hello", true, null]
let empty = []

// Array access
let first = numbers[0]      // 1
let last = numbers[4]       // 5
numbers[1] = 10             // Modify element

// Array methods (planned)
numbers.push(6)             // Add element
let length = numbers.length // Get length
```

### **Objects**
```utopia
// Object creation
let person = {
    name: "Alice",
    age: 30,
    isEmployed: true
}

let empty = {}

// Object access
let name = person.name      // "Alice"
let age = person["age"]     // 30

// Object modification
person.age = 31
person["isEmployed"] = false
person.city = "New York"    // Add new property

// Nested objects
let company = {
    name: "Tech Corp",
    address: {
        street: "123 Main St",
        city: "San Francisco",
        zip: "94105"
    },
    employees: [
        { name: "Alice", role: "Developer" },
        { name: "Bob", role: "Designer" }
    ]
}
```

---

## 🌐 **Language Blocks**

### **Language Block Syntax**
```utopia
@lang <language_name>
<language_specific_code>
```

### **Supported Languages**
```utopia
// Production ready
@lang python
@lang javascript
@lang typescript
@lang main

// Assembly targets
@lang x86_64
@lang llvm
@lang wasm
@lang cuda

// Planned languages
@lang c
@lang cpp
@lang rust
@lang go
@lang java
```

### **Language Block Examples**

#### **Python Block**
```utopia
@lang python
import math

def calculate_circle_area(radius):
    return math.pi * radius ** 2

def process_data(data):
    return [x * 2 for x in data if x > 0]

class Calculator:
    def __init__(self):
        self.history = []
    
    def add(self, a, b):
        result = a + b
        self.history.append(f"{a} + {b} = {result}")
        return result
```

#### **JavaScript Block**
```utopia
@lang javascript
function formatCurrency(amount) {
    return new Intl.NumberFormat('en-US', {
        style: 'currency',
        currency: 'USD'
    }).format(amount);
}

async function fetchUserData(userId) {
    const response = await fetch(`/api/users/${userId}`);
    return response.json();
}

class EventEmitter {
    constructor() {
        this.events = {};
    }
    
    on(event, callback) {
        if (!this.events[event]) {
            this.events[event] = [];
        }
        this.events[event].push(callback);
    }
    
    emit(event, data) {
        if (this.events[event]) {
            this.events[event].forEach(callback => callback(data));
        }
    }
}
```

#### **TypeScript Block**
```utopia
@lang typescript
interface User {
    id: number;
    name: string;
    email: string;
    roles: string[];
}

interface ApiResponse<T> {
    data: T;
    status: number;
    message: string;
}

class UserService {
    private users: User[] = [];
    
    async getUser(id: number): Promise<ApiResponse<User>> {
        const user = this.users.find(u => u.id === id);
        if (user) {
            return {
                data: user,
                status: 200,
                message: "User found"
            };
        }
        throw new Error("User not found");
    }
    
    addUser(user: User): void {
        this.users.push(user);
    }
}
```

---

## 🔗 **Cross-Language Calls**

### **Call Syntax**
```utopia
<language_prefix>::<function_name>(<arguments>)
```

### **Language Prefixes**
```utopia
py::    // Python functions
js::    // JavaScript functions
ts::    // TypeScript functions
c::     // C functions (planned)
rs::    // Rust functions (planned)
go::    // Go functions (planned)
```

### **Cross-Language Examples**

#### **Python ↔ JavaScript Integration**
```utopia
@lang python
import json
import requests

def fetch_weather_data(city):
    response = requests.get(f"http://api.weather.com/{city}")
    return json.loads(response.text)

def analyze_temperature(data):
    temperatures = [d['temp'] for d in data]
    return {
        'average': sum(temperatures) / len(temperatures),
        'min': min(temperatures),
        'max': max(temperatures)
    }

@lang javascript
function formatWeatherReport(analysis) {
    return `
        Weather Analysis:
        Average: ${analysis.average.toFixed(1)}°F
        Range: ${analysis.min}°F - ${analysis.max}°F
    `;
}

function displayNotification(message) {
    console.log(`[NOTIFICATION] ${message}`);
    // Could integrate with browser notification API
}

@lang main
let city = "San Francisco"
let weatherData = py::fetch_weather_data(city)
let analysis = py::analyze_temperature(weatherData)
let report = js::formatWeatherReport(analysis)
js::displayNotification(report)
```

#### **Multi-Language Data Processing**
```utopia
@lang python
import numpy as np
import pandas as pd

def load_dataset(filename):
    return pd.read_csv(filename)

def statistical_analysis(data):
    return {
        'mean': data.mean().to_dict(),
        'std': data.std().to_dict(),
        'correlation': data.corr().to_dict()
    }

@lang javascript
function createVisualization(stats) {
    // Integration with Chart.js or D3.js
    return {
        type: 'chart',
        data: stats,
        config: {
            responsive: true,
            scales: {
                y: { beginAtZero: true }
            }
        }
    };
}

function generateDashboard(charts) {
    return {
        title: "Data Analysis Dashboard",
        charts: charts,
        timestamp: new Date().toISOString()
    };
}

@lang main
let dataset = py::load_dataset("sales_data.csv")
let stats = py::statistical_analysis(dataset)
let chart = js::createVisualization(stats)
let dashboard = js::generateDashboard([chart])
println("Dashboard created:", dashboard.title)
```

---

## 💬 **Comments**

### **Single-Line Comments**
```utopia
// This is a single-line comment
let x = 5;  // Comment at end of line

// Multiple single-line comments
// can be used for longer explanations
// that span multiple lines
```

### **Multi-Line Comments**
```utopia
/*
 * This is a multi-line comment
 * that can span multiple lines
 * and is useful for longer explanations
 */

let y = 10;

/*
Multi-line comments can also
be written without asterisks
for each line
*/
```

### **Documentation Comments** (Planned)
```utopia
/**
 * Calculates the factorial of a number
 * @param n The number to calculate factorial for
 * @returns The factorial result
 * @throws Error if n is negative
 */
function factorial(n: number): number {
    if (n < 0) throw new Error("Negative numbers not allowed");
    if (n <= 1) return 1;
    return n * factorial(n - 1);
}
```

---

## 🚀 **Advanced Features**

### **Error Handling** (Planned)
```utopia
try {
    let result = py::risky_operation()
    println("Success:", result)
} catch (error) {
    println("Error occurred:", error.message)
} finally {
    println("Cleanup code runs here")
}

// Throwing errors
function validateAge(age) {
    if (age < 0) {
        throw new Error("Age cannot be negative")
    }
    if (age > 150) {
        throw new Error("Age seems unrealistic")
    }
    return true
}
```

### **Modules and Imports** (Planned)
```utopia
// math_utils.uto
export function add(a, b) {
    return a + b
}

export function multiply(a, b) {
    return a * b
}

export const PI = 3.14159

// main.uto
import { add, multiply, PI } from "./math_utils.uto"

let sum = add(5, 3)
let product = multiply(4, 7)
let circleArea = PI * 5 * 5
```

### **Classes and Objects** (Planned)
```utopia
class Person {
    constructor(name, age) {
        this.name = name
        this.age = age
    }
    
    greet() {
        return `Hello, I'm ${this.name} and I'm ${this.age} years old`
    }
    
    haveBirthday() {
        this.age++
        println(`Happy birthday ${this.name}! Now ${this.age} years old.`)
    }
}

let person = new Person("Alice", 25)
println(person.greet())
person.haveBirthday()
```

### **Generics** (Planned)
```utopia
function identity<T>(arg: T): T {
    return arg
}

let stringResult = identity<string>("hello")
let numberResult = identity<number>(42)

class Container<T> {
    private value: T
    
    constructor(value: T) {
        this.value = value
    }
    
    getValue(): T {
        return this.value
    }
    
    setValue(value: T): void {
        this.value = value
    }
}

let stringContainer = new Container<string>("hello")
let numberContainer = new Container<number>(42)
```

### **Async/Await** (Planned)
```utopia
async function fetchData(url) {
    let response = await fetch(url)
    let data = await response.json()
    return data
}

async function processMultipleUrls(urls) {
    let promises = urls.map(url => fetchData(url))
    let results = await Promise.all(promises)
    return results
}

// Usage
let data = await fetchData("https://api.example.com/data")
println("Data received:", data)
```

---

## 🎯 **Best Practices**

### **Naming Conventions**
```utopia
// Variables and functions: camelCase
let userName = "alice"
let userAge = 25
function calculateTotal() { }

// Constants: UPPER_SNAKE_CASE
const MAX_RETRIES = 3
const API_BASE_URL = "https://api.example.com"

// Classes: PascalCase (planned)
class UserManager { }
class DatabaseConnection { }
```

### **Code Organization**
```utopia
// 1. Constants first
const CONFIG = {
    maxUsers: 100,
    timeout: 5000
}

// 2. Helper functions
function validateInput(input) {
    return input && input.length > 0
}

// 3. Main logic
function processUser(userData) {
    if (!validateInput(userData.name)) {
        throw new Error("Invalid name")
    }
    // Process user...
}

// 4. Language blocks
@lang python
def heavy_computation(data):
    # CPU-intensive operations in Python
    pass

// 5. Main execution
@lang main
let result = py::heavy_computation(largeDataset)
println("Processing complete")
```

### **Error Handling Patterns**
```utopia
// Check for null values
function safeAccess(obj, property) {
    if (obj == null) {
        return null
    }
    return obj[property]
}

// Validate function parameters
function divide(a, b) {
    if (b == 0) {
        throw new Error("Division by zero")
    }
    return a / b
}

// Use meaningful error messages
function validateEmail(email) {
    if (!email.includes("@")) {
        throw new Error(`Invalid email format: ${email}. Must contain @ symbol.`)
    }
}
```

---

<div align="center">

**Master Utopia Syntax! 🎯**

*Write once, compile everywhere with Utopia's unified syntax*

[**🏠 Back to Docs**](README.md) • [**📖 Language Guide**](utopia-language-guide.md) • [**⚙️ CLI Reference**](cli-reference.md)

</div> 