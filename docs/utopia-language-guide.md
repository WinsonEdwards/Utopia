# Utopia Language Guide

## What is Utopia?

Utopia is a **unified multi-language programming language** that allows you to write code in one syntax and compile it to multiple programming languages. Think of it as a "universal translator" for programming!

## 🌟 Utopia Language Basics

### File Extension
Utopia source files use the `.uto` extension:
```
myprogram.uto
```

### Language Block Syntax
Use `@lang` to define code blocks for different languages:

```utopia
@lang python {
    # This is Python code
    def hello():
        return "Hello from Python!"
}

@lang javascript {
    // This is JavaScript code
    function greet() {
        return "Hello from JavaScript!";
    }
}

@lang main {
    // This is the main program code
    println("Hello, Utopia!");
}
```

### Variable Declaration
```utopia
@lang main {
    let name = "Alice";
    let age = 25;
    let isStudent = true;
}
```

### Function Definition
```utopia
@lang main {
    function add(a, b) {
        return a + b;
    }
    
    let result = add(5, 3);
    println("Result:", result);
}
```

### Control Flow
```utopia
@lang main {
    let score = 85;
    
    if (score >= 90) {
        println("Excellent!");
    } else if (score >= 80) {
        println("Good!");
    } else {
        println("Needs improvement!");
    }
    
    let i = 0;
    while (i < 5) {
        println("Count:", i);
        i = i + 1;
    }
}
```

## 🔗 Cross-Language Calls

This is Utopia's most powerful feature!

### Basic Cross-Language Calls
```utopia
@lang python {
def process_data(data):
    return [x * 2 for x in data]
}

@lang c {
int fast_calc(int a, int b) {
    return a * b;
}
}

@lang main {
    let numbers = [1, 2, 3, 4, 5];
    
    // Call Python function
    let processed = python::process_data(numbers);
    
    // Call C function
    let result = c::fast_calc(10, 5);
    
    println("Processed data:", processed);
    println("Calculation result:", result);
}
```

### Complex Example: Data Processing Pipeline
```utopia
@lang python {
def ml_predict(data):
    # Simulate machine learning prediction
    return [x * 0.8 + 2 for x in data]
}

@lang c {
int optimize(int value) {
    // Simulate performance optimization
    return value * 2 + 10;
}
}

@lang javascript {
function format(data) {
    // Format output
    return data.map(x => `Value: ${x}`).join(', ');
}
}

@lang main {
    let raw_data = [1, 2, 3, 4, 5];
    
    // 1. Use Python for machine learning prediction
    let predicted = python::ml_predict(raw_data);
    
    // 2. Use C for performance optimization
    let optimized = [];
    for (let i = 0; i < predicted.length; i++) {
        optimized.push(c::optimize(predicted[i]));
    }
    
    // 3. Use JavaScript for formatting output
    let formatted = javascript::format(optimized);
    
    println("Final result:", formatted);
}
```

## 🎯 Real-World Applications

### Web Development
```utopia
@lang python {
def api_process(data):
    return {"status": "success", "data": data}
}

@lang javascript {
function frontend_render(data) {
    return `<div>${data.status}: ${data.data}</div>`;
}
}

@lang main {
    let user_data = {"name": "Alice", "age": 25};
    let api_result = python::api_process(user_data);
    let html_output = javascript::frontend_render(api_result);
    println("HTML:", html_output);
}
```

### Scientific Computing
```utopia
@lang python {
def numpy_calc(data):
    import numpy as np
    return np.mean(data)
}

@lang c {
double fast_math(double a, double b) {
    return a * b + sqrt(a + b);
}
}

@lang main {
    let dataset = [1.5, 2.3, 3.7, 4.1, 5.9];
    let average = python::numpy_calc(dataset);
    let calculation = c::fast_math(average, 2.0);
    println("Average:", average);
    println("Calculation result:", calculation);
}
```

## 🚀 How to Use Utopia

### Installation
```bash
# Clone the repository
git clone https://github.com/your-username/utopia.git
cd utopia/utopia-rs

# Build the compiler
cargo build --release

# Install globally
cargo install --path .
```

### Running Code
```bash
# Run directly
utopia run myprogram.uto

# Compile to Python
utopia compile myprogram.uto --target python --output out.py

# Compile to JavaScript
utopia compile myprogram.uto --target javascript --output out.js

# Start interactive environment
utopia repl
```

### Interactive Programming
```bash
$ utopia repl
Utopia REPL v1.0.0
> let x = 10
> let y = 20
> function add(a, b) { return a + b; }
> println(add(x, y))
30
> exit
```

## 📚 Learning Path

1. **Start Simple**: Begin with basic variables and functions
2. **Practice Cross-Language Calls**: This is Utopia's core feature
3. **Try Real Projects**: Use Utopia to solve actual problems
4. **Read Documentation**: Check the `docs/` directory for detailed guides
5. **Join the Community**: Ask questions and contribute on GitHub

## 🎨 Project Icon

The Utopia project icon represents:
- **Unity**: A single symbol (like a star) at the center
- **Diversity**: Elements representing different programming languages around it
- **Innovation**: Multiple colors showing multi-language support

Suggested icon: 🌟 Utopia

## 🔧 Advanced Features

### Type Safety
```utopia
@lang main {
    let name: string = "Alice";
    let age: number = 25;
    let scores: array = [85, 92, 78];
}
```

### Error Handling
```utopia
@lang main {
    try {
        let result = divide(10, 0);
        println("Result:", result);
    } catch (error) {
        println("Error:", error);
    }
}
```

### Modules and Imports
```utopia
@lang main {
    import math from "math.uto";
    let result = math::sqrt(16);
    println("Square root:", result);
}
```

## 🌟 Why Choose Utopia?

1. **Learn Multiple Languages**: Understand different programming paradigms
2. **Optimize Performance**: Use the best language for each task
3. **Rapid Prototyping**: Quick development with multiple language support
4. **Educational Value**: Great for learning compiler design and Rust
5. **Future-Proof**: Adapt to new languages and technologies

---

Utopia makes programming more flexible and fun! You can write in one syntax and enjoy the benefits of multiple languages. What would you like to learn more about? 