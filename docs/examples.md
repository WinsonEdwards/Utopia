# Utopia Examples

This document contains practical examples demonstrating Utopia's features.

## Basic Examples

### Hello World

```utopia
@lang main {
    println("Hello, World!");
}
```

### Variables and Functions

```utopia
@lang main {
    let name = "Alice";
    let age = 30;
    
    function greet(person, years) {
        return "Hello, " + person + "! You are " + years + " years old.";
    }
    
    let message = greet(name, age);
    println(message);
}
```

### Control Flow

```utopia
@lang main {
    let score = 85;
    
    if (score >= 90) {
        println("Grade: A");
    } else if (score >= 80) {
        println("Grade: B");
    } else if (score >= 70) {
        println("Grade: C");
    } else {
        println("Grade: F");
    }
    
    let i = 0;
    while (i < 5) {
        println("Count:", i);
        i = i + 1;
    }
}
```

### Arrays and Objects

```utopia
@lang main {
    let numbers = [1, 2, 3, 4, 5];
    let person = {
        name: "Bob",
        age: 25,
        city: "New York"
    };
    
    println("Numbers:", numbers);
    println("Person name:", person.name);
    println("Person age:", person.age);
    
    let sum = 0;
    let i = 0;
    while (i < len(numbers)) {
        sum = sum + numbers[i];
        i = i + 1;
    }
    println("Sum:", sum);
}
```

## Cross-Language Examples

### Python and JavaScript Integration

```utopia
@lang python {
    def calculate_average(numbers):
        if len(numbers) == 0:
            return 0
        return sum(numbers) / len(numbers)
    
    def filter_even(numbers):
        return [x for x in numbers if x % 2 == 0]
}

@lang javascript {
    function process_data(data) {
        return data.map(x => x * 2).filter(x => x > 10);
    }
    
    function format_output(result) {
        return "Processed: " + result.join(", ");
    }
}

@lang main {
    let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    let average = python::calculate_average(data);
    let even_numbers = python::filter_even(data);
    let processed = javascript::process_data(data);
    let formatted = javascript::format_output(processed);
    
    println("Original data:", data);
    println("Average:", average);
    println("Even numbers:", even_numbers);
    println("Processed result:", formatted);
}
```

### Multi-Language Data Processing

```utopia
@lang python {
    def analyze_text(text):
        words = text.split()
        return {
            "word_count": len(words),
            "char_count": len(text),
            "avg_word_length": sum(len(word) for word in words) / len(words) if words else 0
        }
}

@lang javascript {
    function validate_email(email) {
        const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
        return emailRegex.test(email);
    }
    
    function format_user_info(user) {
        return `${user.name} (${user.email}) - ${user.age} years old`;
    }
}

@lang main {
    let text = "Hello world from Utopia compiler";
    let user = {
        name: "John Doe",
        email: "john@example.com",
        age: 28
    };
    
    let analysis = python::analyze_text(text);
    let is_valid_email = javascript::validate_email(user.email);
    let user_info = javascript::format_user_info(user);
    
    println("Text analysis:", analysis);
    println("Valid email:", is_valid_email);
    println("User info:", user_info);
}
```

## Advanced Examples

### Recursive Functions

```utopia
@lang main {
    function factorial(n) {
        if (n <= 1) {
            return 1;
        }
        return n * factorial(n - 1);
    }
    
    function fibonacci(n) {
        if (n <= 1) {
            return n;
        }
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
    
    println("Factorial of 5:", factorial(5));
    println("Fibonacci(10):", fibonacci(10));
}
```

### Error Handling

```utopia
@lang main {
    function safe_divide(a, b) {
        if (b == 0) {
            println("Error: Division by zero");
            return null;
        }
        return a / b;
    }
    
    let result1 = safe_divide(10, 2);
    let result2 = safe_divide(10, 0);
    
    println("10 / 2 =", result1);
    println("10 / 0 =", result2);
}
```

### Working with Files

```utopia
@lang python {
    def read_file_content(filename):
        try:
            with open(filename, 'r') as file:
                return file.read()
        except FileNotFoundError:
            return "File not found: " + filename
        except Exception as e:
            return "Error reading file: " + str(e)
}

@lang main {
    let filename = "test.txt";
    let content = python::read_file_content(filename);
    println("File content:", content);
}
```

## Running Examples

To run these examples:

```bash
# Save example to file (e.g., example.uto)
# Then run directly
utopia run example.uto

# Or compile to specific language
utopia compile example.uto --target python
utopia compile example.uto --target javascript
``` 