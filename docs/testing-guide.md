# 🧪 **Utopia Testing Guide**

Complete testing and quality assurance guide for the Utopia Multi-Language Compiler. From basic validation to advanced testing strategies.

---

## 📋 **Testing Overview**

### **🎯 Why Test Utopia Code?**
- **Multi-Language Consistency** - Ensure code works across all target languages
- **Cross-Language Integration** - Verify seamless communication between language blocks
- **Compilation Accuracy** - Validate generated code matches expected behavior
- **Performance Verification** - Confirm optimal compilation and execution speed
- **Error Handling** - Test graceful failure and error reporting

### **🏗️ Testing Architecture**
```
Utopia Testing Framework
├── 🔧 Compiler Tests
│   ├── Unit Tests (Rust code)
│   ├── Integration Tests  
│   └── Performance Tests
├── 🌐 Language Target Tests
│   ├── Python Output Tests
│   ├── JavaScript Output Tests
│   ├── TypeScript Output Tests
│   └── Assembly Output Tests
├── 🔗 Cross-Language Tests
│   ├── Function Call Tests
│   ├── Data Transfer Tests
│   └── Error Propagation Tests
└── 📊 End-to-End Tests
    ├── Real Application Tests
    ├── Performance Benchmarks
    └── User Workflow Tests
```

---

## 🚀 **Quick Start Testing**

### **1. Basic Validation**
Test that your Utopia installation works correctly:

```bash
# Test basic compilation
echo 'println("Hello, Test!")' > test.uto
utopia run test.uto

# Expected output: Hello, Test!
```

### **2. Multi-Language Test**
Verify cross-language functionality:

```bash
# Create multi-language test
cat << 'EOF' > multi-test.uto
@lang python
def add(a, b):
    return a + b

@lang main
let result = py::add(5, 3)
println("5 + 3 =", result)
EOF

utopia run multi-test.uto
# Expected output: 5 + 3 = 8
```

### **3. Compilation Test**
Test target language compilation:

```bash
# Compile to different targets
utopia compile multi-test.uto --target python --output test.py
utopia compile multi-test.uto --target javascript --output test.js

# Verify generated files
ls -la test.py test.js
```

✅ **If all tests pass, your Utopia installation is working correctly!**

---

## 🔧 **Compiler Testing**

### **Unit Tests**
Test individual compiler components:

```bash
# Run Rust unit tests
cd utopia-rs
cargo test

# Run specific test modules
cargo test lexer
cargo test parser
cargo test transformers

# Run tests with output
cargo test -- --nocapture
```

### **Integration Tests**
Test complete compilation pipeline:

```bash
# Test basic compilation
cargo test test_basic_compilation

# Test multi-language compilation
cargo test test_cross_language_calls

# Test error handling
cargo test test_error_handling
```

### **Performance Tests**
Measure compilation performance:

```bash
# Run benchmark tests
cargo test --release test_performance

# Measure compilation time
time utopia compile large_file.uto --target python
```

---

## 🌐 **Language Target Testing**

### **Python Output Testing**

#### **Basic Python Test**
```utopia
// test-python-basic.uto
let x = 10
let y = 20
let sum = x + y
println("Sum:", sum)

let name = "Python"
println("Hello,", name)

if (sum > 25) {
    println("Sum is greater than 25")
} else {
    println("Sum is 25 or less")
}
```

**Test Commands:**
```bash
# Compile to Python
utopia compile test-python-basic.uto --target python --output test_basic.py

# Run generated Python
python3 test_basic.py

# Expected output:
# Sum: 30
# Hello, Python
# Sum is greater than 25
```

#### **Python Advanced Test**
```utopia
// test-python-advanced.uto
@lang python
import math

def calculate_circle_area(radius):
    return math.pi * radius ** 2

def fibonacci(n):
    if n <= 1:
        return n
    return fibonacci(n-1) + fibonacci(n-2)

@lang main
let radius = 5
let area = py::calculate_circle_area(radius)
println("Circle area:", area)

let fib_number = py::fibonacci(8)
println("8th Fibonacci number:", fib_number)

// Test arrays and loops
let numbers = [1, 2, 3, 4, 5]
for (let i = 0; i < numbers.length; i++) {
    println("Number:", numbers[i])
}
```

**Validation:**
```bash
utopia compile test-python-advanced.uto --target python --output test_advanced.py
python3 test_advanced.py

# Verify mathematical accuracy
python3 -c "import math; print(f'Expected area: {math.pi * 25}')"
```

### **JavaScript Output Testing**

#### **Basic JavaScript Test**
```utopia
// test-javascript-basic.uto
let message = "Hello, JavaScript!"
println(message)

let numbers = [10, 20, 30]
let total = 0

for (let i = 0; i < numbers.length; i++) {
    total += numbers[i]
}

println("Total:", total)

function greet(name) {
    return "Hello, " + name + "!"
}

let greeting = greet("World")
println(greeting)
```

**Test Commands:**
```bash
# Compile to JavaScript
utopia compile test-javascript-basic.uto --target javascript --output test_basic.js

# Run with Node.js
node test_basic.js

# Expected output:
# Hello, JavaScript!
# Total: 60
# Hello, World!
```

#### **JavaScript Cross-Language Test**
```utopia
// test-js-cross-lang.uto
@lang python
def process_data(data):
    return [x * 2 for x in data if x > 0]

@lang javascript
function formatResults(data) {
    return data.map((item, index) => `Item ${index + 1}: ${item}`);
}

@lang main
let input = [1, -2, 3, -4, 5]
let processed = py::process_data(input)
let formatted = js::formatResults(processed)

println("Input:", input)
println("Processed:", processed)
for (let i = 0; i < formatted.length; i++) {
    println(formatted[i])
}
```

**Validation:**
```bash
utopia compile test-js-cross-lang.uto --target javascript --output test_cross.js
node test_cross.js

# Verify cross-language data flow
```

### **TypeScript Output Testing**

#### **TypeScript Test with Types**
```utopia
// test-typescript.uto
@lang typescript
interface User {
    id: number;
    name: string;
    email: string;
}

function createUser(id: number, name: string, email: string): User {
    return { id, name, email };
}

function getUserInfo(user: User): string {
    return `${user.name} (${user.email})`;
}

@lang main
let user = ts::createUser(1, "Alice", "alice@example.com")
let info = ts::getUserInfo(user)
println("User info:", info)
```

**Test Commands:**
```bash
# Compile to TypeScript
utopia compile test-typescript.uto --target typescript --output test.ts

# Compile TypeScript to JavaScript
npx tsc test.ts

# Run compiled JavaScript
node test.js
```

---

## 🔗 **Cross-Language Integration Testing**

### **Data Type Conversion Test**
```utopia
// test-data-conversion.uto
@lang python
def test_data_types():
    return {
        'string': 'Hello',
        'number': 42,
        'boolean': True,
        'list': [1, 2, 3],
        'dict': {'key': 'value'}
    }

@lang javascript
function validateTypes(data) {
    const results = {
        string: typeof data.string === 'string',
        number: typeof data.number === 'number',
        boolean: typeof data.boolean === 'boolean',
        array: Array.isArray(data.list),
        object: typeof data.dict === 'object'
    };
    
    return results;
}

@lang main
let testData = py::test_data_types()
let validation = js::validateTypes(testData)

println("Data from Python:", testData)
println("Type validation:", validation)

// Verify each type
println("String valid:", validation.string)
println("Number valid:", validation.number)
println("Boolean valid:", validation.boolean)
println("Array valid:", validation.array)
println("Object valid:", validation.object)
```

### **Error Handling Test**
```utopia
// test-error-handling.uto
@lang python
def divide_numbers(a, b):
    if b == 0:
        raise ValueError("Cannot divide by zero")
    return a / b

def safe_divide(a, b):
    try:
        return divide_numbers(a, b)
    except ValueError as e:
        return {'error': str(e)}

@lang main
// Test successful division
let result1 = py::safe_divide(10, 2)
println("10 / 2 =", result1)

// Test division by zero
let result2 = py::safe_divide(10, 0)
println("10 / 0 =", result2)

// Test with different numbers
let numbers = [[15, 3], [20, 4], [7, 0], [100, 10]]
for (let i = 0; i < numbers.length; i++) {
    let pair = numbers[i]
    let result = py::safe_divide(pair[0], pair[1])
    println(pair[0], "/", pair[1], "=", result)
}
```

### **Performance Test**
```utopia
// test-performance.uto
@lang python
import time

def python_heavy_computation(n):
    start = time.time()
    result = sum(i * i for i in range(n))
    end = time.time()
    return {
        'result': result,
        'time': end - start,
        'language': 'Python'
    }

@lang javascript
function jsHeavyComputation(n) {
    const start = Date.now();
    let result = 0;
    for (let i = 0; i < n; i++) {
        result += i * i;
    }
    const end = Date.now();
    
    return {
        result: result,
        time: (end - start) / 1000,
        language: 'JavaScript'
    };
}

@lang main
let n = 1000000
println("Performance test with n =", n)

let pythonResult = py::python_heavy_computation(n)
let jsResult = js::jsHeavyComputation(n)

println("\nPython computation:")
println("  Result:", pythonResult.result)
println("  Time:", pythonResult.time, "seconds")

println("\nJavaScript computation:")
println("  Result:", jsResult.result)
println("  Time:", jsResult.time, "seconds")

// Verify results match
if (pythonResult.result == jsResult.result) {
    println("\n✅ Results match!")
} else {
    println("\n❌ Results don't match!")
}

// Compare performance
if (pythonResult.time < jsResult.time) {
    println("🏆 Python was faster")
} else {
    println("🏆 JavaScript was faster")
}
```

---

## 📊 **Automated Testing Scripts**

### **Complete Test Suite**
Create `run_all_tests.sh`:

```bash
#!/bin/bash

# Utopia Complete Test Suite
set -e

echo "🧪 Starting Utopia Test Suite"
echo "=============================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Test counter
TESTS_RUN=0
TESTS_PASSED=0
TESTS_FAILED=0

# Function to run a test
run_test() {
    local test_name="$1"
    local test_command="$2"
    local expected_output="$3"
    
    echo -n "Running $test_name... "
    TESTS_RUN=$((TESTS_RUN + 1))
    
    if output=$(eval "$test_command" 2>&1); then
        if [[ -z "$expected_output" ]] || echo "$output" | grep -q "$expected_output"; then
            echo -e "${GREEN}PASS${NC}"
            TESTS_PASSED=$((TESTS_PASSED + 1))
            return 0
        else
            echo -e "${RED}FAIL${NC} (unexpected output)"
            echo "Expected: $expected_output"
            echo "Got: $output"
            TESTS_FAILED=$((TESTS_FAILED + 1))
            return 1
        fi
    else
        echo -e "${RED}FAIL${NC} (command failed)"
        echo "$output"
        TESTS_FAILED=$((TESTS_FAILED + 1))
        return 1
    fi
}

# 1. Compiler Tests
echo -e "\n${YELLOW}1. Compiler Tests${NC}"
echo "=================="

cd utopia-rs
run_test "Rust unit tests" "cargo test --quiet" "test result: ok"
run_test "Lexer tests" "cargo test lexer --quiet" "test result: ok"
run_test "Parser tests" "cargo test parser --quiet" "test result: ok"
run_test "Transformer tests" "cargo test transformers --quiet" "test result: ok"
cd ..

# 2. Basic Compilation Tests
echo -e "\n${YELLOW}2. Basic Compilation Tests${NC}"
echo "=========================="

# Create test files
echo 'println("Hello, Test!")' > basic_test.uto
run_test "Basic compilation" "utopia run basic_test.uto" "Hello, Test!"

echo 'let x = 5; let y = 3; println("Sum:", x + y)' > math_test.uto
run_test "Math operations" "utopia run math_test.uto" "Sum: 8"

# 3. Multi-Language Tests
echo -e "\n${YELLOW}3. Multi-Language Tests${NC}"
echo "======================"

cat << 'EOF' > multi_test.uto
@lang python
def add(a, b):
    return a + b

@lang main
let result = py::add(10, 5)
println("Result:", result)
EOF

run_test "Python integration" "utopia run multi_test.uto" "Result: 15"

cat << 'EOF' > js_test.uto
@lang javascript
function multiply(a, b) {
    return a * b;
}

@lang main
let result = js::multiply(6, 7)
println("Product:", result)
EOF

run_test "JavaScript integration" "utopia run js_test.uto" "Product: 42"

# 4. Target Compilation Tests
echo -e "\n${YELLOW}4. Target Compilation Tests${NC}"
echo "==========================="

run_test "Python compilation" "utopia compile basic_test.uto --target python --output test.py && python3 test.py" "Hello, Test!"
run_test "JavaScript compilation" "utopia compile basic_test.uto --target javascript --output test.js && node test.js" "Hello, Test!"

# 5. Error Handling Tests
echo -e "\n${YELLOW}5. Error Handling Tests${NC}"
echo "======================"

echo 'let x = 5 +' > syntax_error.uto
run_test "Syntax error detection" "utopia run syntax_error.uto 2>&1 | grep -i error" "error"

echo 'let x = py::nonexistent_function()' > missing_function.uto
run_test "Missing function error" "utopia run missing_function.uto 2>&1 | grep -i error" "error"

# 6. Performance Tests
echo -e "\n${YELLOW}6. Performance Tests${NC}"
echo "==================="

cat << 'EOF' > perf_test.uto
for (let i = 0; i < 1000; i++) {
    let x = i * i
}
println("Performance test completed")
EOF

run_test "Performance test" "timeout 5 utopia run perf_test.uto" "Performance test completed"

# Cleanup
rm -f *.uto *.py *.js test_*

# Results Summary
echo -e "\n${YELLOW}Test Results Summary${NC}"
echo "===================="
echo "Tests run: $TESTS_RUN"
echo -e "Tests passed: ${GREEN}$TESTS_PASSED${NC}"
echo -e "Tests failed: ${RED}$TESTS_FAILED${NC}"

if [[ $TESTS_FAILED -eq 0 ]]; then
    echo -e "\n🎉 ${GREEN}All tests passed!${NC}"
    exit 0
else
    echo -e "\n❌ ${RED}Some tests failed.${NC}"
    exit 1
fi
```

### **Quick Development Test**
Create `quick_test.sh`:

```bash
#!/bin/bash

# Quick Utopia Development Test
echo "⚡ Quick Utopia Test"
echo "==================="

# Test basic functionality
echo "Testing basic compilation..."
echo 'println("Quick test passed!")' | utopia run -

if [[ $? -eq 0 ]]; then
    echo "✅ Basic test: PASS"
else
    echo "❌ Basic test: FAIL"
    exit 1
fi

# Test multi-language
echo "Testing multi-language..."
cat << 'EOF' | utopia run -
@lang python
def test():
    return "Multi-language works!"

@lang main
let result = py::test()
println(result)
EOF

if [[ $? -eq 0 ]]; then
    echo "✅ Multi-language test: PASS"
else
    echo "❌ Multi-language test: FAIL"
    exit 1
fi

echo "🎉 Quick tests completed successfully!"
```

### **Continuous Integration Test**
Create `.github/workflows/test.yml`:

```yaml
name: Utopia Tests

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main ]

jobs:
  test:
    runs-on: ubuntu-latest
    
    strategy:
      matrix:
        rust: [stable, beta]
        node: [16, 18, 20]
        python: [3.8, 3.9, 3.10, 3.11]
    
    steps:
    - uses: actions/checkout@v3
    
    - name: Setup Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: ${{ matrix.rust }}
        override: true
    
    - name: Setup Node.js
      uses: actions/setup-node@v3
      with:
        node-version: ${{ matrix.node }}
    
    - name: Setup Python
      uses: actions/setup-python@v4
      with:
        python-version: ${{ matrix.python }}
    
    - name: Build Utopia
      run: |
        cd utopia-rs
        cargo build --release
        cargo install --path .
    
    - name: Run Rust Tests
      run: |
        cd utopia-rs
        cargo test
    
    - name: Run Integration Tests
      run: |
        chmod +x run_all_tests.sh
        ./run_all_tests.sh
    
    - name: Run Quick Tests
      run: |
        chmod +x quick_test.sh
        ./quick_test.sh
```

---

## 🐛 **Debugging and Troubleshooting**

### **Common Issues and Solutions**

#### **Compilation Fails**
```bash
# Check Utopia installation
utopia --version

# Verify Rust installation
cd utopia-rs
cargo check

# Check syntax
utopia compile your_file.uto --debug
```

#### **Cross-Language Calls Don't Work**
```bash
# Test Python availability
python3 --version

# Test Node.js availability
node --version

# Debug specific language block
utopia run your_file.uto --debug-python
utopia run your_file.uto --debug-javascript
```

#### **Generated Code Issues**
```bash
# Inspect generated code
utopia compile your_file.uto --target python --output debug.py
cat debug.py

# Test generated code directly
python3 debug.py
node debug.js
```

### **Debug Mode Testing**
```bash
# Enable verbose output
export UTOPIA_DEBUG=1
utopia run your_file.uto

# Test with maximum verbosity
utopia run your_file.uto --verbose --debug

# Log compilation steps
utopia compile your_file.uto --target python --log-level debug
```

### **Performance Debugging**
```bash
# Measure compilation time
time utopia compile large_file.uto --target python

# Profile memory usage
/usr/bin/time -v utopia run memory_intensive.uto

# Compare target performance
time python3 generated.py
time node generated.js
```

---

## 📈 **Testing Best Practices**

### **🎯 Testing Strategy**

#### **1. Test Pyramid**
```
     🔺 End-to-End Tests (Few)
    ████ Integration Tests (Some)  
   ████████ Unit Tests (Many)
```

#### **2. Test Categories**
- **Unit Tests**: Test individual functions and components
- **Integration Tests**: Test language interactions
- **System Tests**: Test complete workflows
- **Performance Tests**: Measure speed and memory usage
- **Regression Tests**: Prevent old bugs from returning

#### **3. Test-Driven Development**
```bash
# 1. Write failing test
echo 'function add(a, b) { return a + b; }' > test.uto
utopia run test.uto  # Should work

# 2. Write minimal code to pass
# 3. Refactor and repeat
```

### **🔍 Code Coverage**
```bash
# Generate coverage report
cd utopia-rs
cargo tarpaulin --out Html

# View coverage
open tarpaulin-report.html
```

### **⚡ Performance Testing**
```bash
# Benchmark compilation speed
cd utopia-rs
cargo bench

# Memory usage testing
valgrind --tool=memcheck utopia run large_program.uto

# Stress testing
for i in {1..100}; do utopia run stress_test.uto; done
```

### **🤖 Automated Testing**
```bash
# Set up pre-commit hooks
git config core.hooksPath .githooks

# Create .githooks/pre-commit
cat << 'EOF' > .githooks/pre-commit
#!/bin/bash
./quick_test.sh
EOF

chmod +x .githooks/pre-commit
```

---

## 🚀 **Advanced Testing Scenarios**

### **Real-World Application Testing**
```utopia
// test-web-app.uto
@lang python
from datetime import datetime
import json

def create_user_session(user_id, username):
    return {
        'user_id': user_id,
        'username': username,
        'created_at': datetime.now().isoformat(),
        'expires_at': (datetime.now().timestamp() + 3600),
        'permissions': ['read', 'write']
    }

def validate_session(session):
    current_time = datetime.now().timestamp()
    return session['expires_at'] > current_time

@lang javascript
function renderUserDashboard(session) {
    if (!session) {
        return '<div>Please log in</div>';
    }
    
    return `
        <div class="dashboard">
            <h1>Welcome, ${session.username}!</h1>
            <p>Session expires: ${new Date(session.expires_at * 1000).toLocaleString()}</p>
            <div class="permissions">
                Permissions: ${session.permissions.join(', ')}
            </div>
        </div>
    `;
}

function calculateSessionTimeLeft(session) {
    const now = Date.now() / 1000;
    const timeLeft = session.expires_at - now;
    
    if (timeLeft <= 0) {
        return 'Session expired';
    }
    
    const minutes = Math.floor(timeLeft / 60);
    const seconds = Math.floor(timeLeft % 60);
    
    return `${minutes}m ${seconds}s remaining`;
}

@lang main
// Test user session creation
let session = py::create_user_session(123, "testuser")
println("Session created:", session)

// Test session validation
let isValid = py::validate_session(session)
println("Session valid:", isValid)

// Test UI rendering
let dashboard = js::renderUserDashboard(session)
println("Dashboard HTML length:", dashboard.length)

// Test time calculation
let timeLeft = js::calculateSessionTimeLeft(session)
println("Time remaining:", timeLeft)

// Test session expiry (simulate expired session)
session.expires_at = 0
let expiredValid = py::validate_session(session)
let expiredTime = js::calculateSessionTimeLeft(session)

println("Expired session valid:", expiredValid)
println("Expired session time:", expiredTime)
```

### **Data Processing Pipeline Testing**
```utopia
// test-data-pipeline.uto
@lang python
import json
import re
from datetime import datetime

def extract_data(raw_text):
    """Extract structured data from raw text"""
    email_pattern = r'\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b'
    phone_pattern = r'\b\d{3}-\d{3}-\d{4}\b'
    
    emails = re.findall(email_pattern, raw_text)
    phones = re.findall(phone_pattern, raw_text)
    
    return {
        'emails': emails,
        'phones': phones,
        'extracted_at': datetime.now().isoformat()
    }

def clean_data(data):
    """Clean and validate extracted data"""
    cleaned = {
        'emails': [email.lower() for email in data['emails']],
        'phones': [phone.replace('-', '') for phone in data['phones']],
        'processed_at': datetime.now().isoformat()
    }
    
    return cleaned

@lang javascript
function transformData(data) {
    // Transform to final format
    const contacts = [];
    
    // Create contact objects
    const maxLength = Math.max(data.emails.length, data.phones.length);
    
    for (let i = 0; i < maxLength; i++) {
        const contact = {
            id: i + 1,
            email: data.emails[i] || null,
            phone: data.phones[i] || null,
            created: new Date().toISOString()
        };
        
        contacts.push(contact);
    }
    
    return {
        contacts: contacts,
        summary: {
            total_contacts: contacts.length,
            emails_found: data.emails.length,
            phones_found: data.phones.length
        }
    };
}

function generateReport(transformed_data) {
    const report = {
        title: 'Data Processing Report',
        generated_at: new Date().toISOString(),
        summary: transformed_data.summary,
        sample_contacts: transformed_data.contacts.slice(0, 3),
        stats: {
            complete_contacts: transformed_data.contacts.filter(c => c.email && c.phone).length,
            email_only: transformed_data.contacts.filter(c => c.email && !c.phone).length,
            phone_only: transformed_data.contacts.filter(c => !c.email && c.phone).length
        }
    };
    
    return report;
}

@lang main
// Test data
let rawText = `
Contact us at info@company.com or call 555-123-4567.
You can also reach our support team at support@company.com 
or call our helpline at 555-987-6543.
For sales inquiries, email sales@company.com or call 555-555-0123.
`

println("🔄 Testing Data Processing Pipeline")
println("===================================")

// Step 1: Extract data
println("Step 1: Extracting data...")
let extracted = py::extract_data(rawText)
println("Extracted emails:", extracted.emails.length)
println("Extracted phones:", extracted.phones.length)

// Step 2: Clean data
println("\nStep 2: Cleaning data...")
let cleaned = py::clean_data(extracted)
println("Cleaned emails:", cleaned.emails)
println("Cleaned phones:", cleaned.phones)

// Step 3: Transform data
println("\nStep 3: Transforming data...")
let transformed = js::transformData(cleaned)
println("Total contacts:", transformed.summary.total_contacts)
println("Emails found:", transformed.summary.emails_found)
println("Phones found:", transformed.summary.phones_found)

// Step 4: Generate report
println("\nStep 4: Generating report...")
let report = js::generateReport(transformed)

println("\n📊 FINAL REPORT")
println("===============")
println("Title:", report.title)
println("Generated:", report.generated_at)
println("Total contacts:", report.summary.total_contacts)
println("Complete contacts:", report.stats.complete_contacts)
println("Email only:", report.stats.email_only)
println("Phone only:", report.stats.phone_only)

println("\nSample contacts:")
for (let i = 0; i < report.sample_contacts.length; i++) {
    let contact = report.sample_contacts[i]
    println((i + 1) + ".", "Email:", contact.email, "Phone:", contact.phone)
}
```

---

## 🎯 **Testing Checklist**

### **Before Release**
- [ ] All unit tests pass
- [ ] Integration tests pass
- [ ] Cross-language tests pass
- [ ] Performance benchmarks met
- [ ] Error handling verified
- [ ] Documentation tests current
- [ ] Examples work correctly
- [ ] CLI commands tested
- [ ] Target languages compile
- [ ] Generated code runs correctly

### **Regression Testing**
- [ ] Previous bug fixes still work
- [ ] Backward compatibility maintained
- [ ] Performance hasn't degraded
- [ ] Memory usage acceptable
- [ ] New features don't break existing ones

### **User Acceptance Testing**
- [ ] Real-world examples work
- [ ] Documentation is accurate
- [ ] Installation process smooth
- [ ] Error messages helpful
- [ ] Performance acceptable
- [ ] Learning curve reasonable

---

<div align="center">

**🧪 Test with Confidence!**

*Ensure your Utopia code works perfectly across all languages*

[**🏠 Back to Docs**](README.md) • [**📝 Language Guide**](utopia-language-guide.md) • [**💡 Examples**](examples.md)

</div> 