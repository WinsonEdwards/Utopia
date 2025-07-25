# ⚙️ **Utopia CLI Reference**

Complete command-line interface documentation for the Utopia Multi-Language Compiler.

---

## 📋 **Table of Contents**

- [Installation](#-installation)
- [Basic Commands](#-basic-commands)
- [Compilation Options](#-compilation-options)
- [Advanced Features](#-advanced-features)
- [Configuration](#-configuration)
- [Troubleshooting](#-troubleshooting)
- [Examples](#-examples)

---

## 🚀 **Installation**

### **Prerequisites**
```bash
# Required
Rust 1.70+     # Install from https://rustup.rs/
Git            # For cloning the repository

# Optional (for testing compiled output)
Python 3.8+    # For running Python output
Node.js 16+    # For running JavaScript/TypeScript output
LLVM          # For LLVM IR compilation
```

### **Install Utopia**
```bash
# Clone the repository
git clone https://github.com/WinsonEdwards/Utopia.git
cd Utopia

# Build and install globally
cd utopia-rs
cargo install --path .

# Verify installation
utopia --version
```

### **Update Utopia**
```bash
# Pull latest changes
git pull origin main

# Rebuild and reinstall
cd utopia-rs
cargo install --path . --force
```

---

## 🔧 **Basic Commands**

### **`utopia --help`**
Display help information and available commands.

```bash
utopia --help
utopia -h
```

### **`utopia --version`**
Show the current version of Utopia.

```bash
utopia --version
utopia -V
```

### **`utopia compile`**
Compile Utopia source code to target languages.

**Basic Syntax:**
```bash
utopia compile <input_file> --target <language> [options]
```

**Examples:**
```bash
# Compile to Python
utopia compile hello.uto --target python

# Compile with custom output file
utopia compile hello.uto --target javascript --output hello.js

# Compile with optimization
utopia compile app.uto --target typescript --optimize
```

### **`utopia run`**
Execute Utopia source code directly (interpreter mode).

**Basic Syntax:**
```bash
utopia run <input_file> [options]
```

**Examples:**
```bash
# Run a Utopia file
utopia run hello.uto

# Run with verbose output
utopia run hello.uto --verbose

# Run with debug information
utopia run hello.uto --debug
```

### **`utopia repl`**
Start the interactive Read-Eval-Print Loop.

**Basic Syntax:**
```bash
utopia repl [language] [options]
```

**Examples:**
```bash
# Start default REPL
utopia repl

# Start Python-focused REPL
utopia repl python

# Start with verbose output
utopia repl --verbose
```

---

## 🎯 **Compilation Options**

### **Target Languages**

| Language | Flag | Output Extension | Status |
|----------|------|------------------|--------|
| **Python** | `--target python` | `.py` | ✅ Stable |
| **JavaScript** | `--target javascript` | `.js` | ✅ Stable |
| **TypeScript** | `--target typescript` | `.ts` | ✅ Stable |
| **x86_64 Assembly** | `--target x86_64` | `.s` | ✅ Stable |
| **LLVM IR** | `--target llvm` | `.ll` | ✅ Stable |
| **WebAssembly** | `--target wasm` | `.wat` | ✅ Stable |
| **CUDA** | `--target cuda` | `.cu` | ✅ Stable |

**Future Languages (In Development):**
```bash
# Coming soon
utopia compile app.uto --target c          # C language
utopia compile app.uto --target cpp        # C++
utopia compile app.uto --target rust       # Rust
utopia compile app.uto --target go         # Go
utopia compile app.uto --target java       # Java
```

### **Output Options**

#### **`--output <file>` / `-o <file>`**
Specify the output file name.

```bash
utopia compile hello.uto --target python --output my_app.py
utopia compile hello.uto -t javascript -o app.js
```

#### **`--format <format>`**
Specify output format (when applicable).

```bash
# For assembly output
utopia compile app.uto --target x86_64 --format intel  # Intel syntax
utopia compile app.uto --target x86_64 --format att    # AT&T syntax

# For WebAssembly
utopia compile app.uto --target wasm --format wat      # Text format
utopia compile app.uto --target wasm --format wasm     # Binary format
```

### **Optimization Options**

#### **`--optimize` / `-O`**
Enable optimization during compilation.

```bash
utopia compile app.uto --target python --optimize
utopia compile app.uto -t javascript -O
```

#### **`--optimization-level <level>`**
Set specific optimization level.

```bash
utopia compile app.uto --target llvm --optimization-level 0  # No optimization
utopia compile app.uto --target llvm --optimization-level 1  # Basic optimization
utopia compile app.uto --target llvm --optimization-level 2  # Standard optimization
utopia compile app.uto --target llvm --optimization-level 3  # Aggressive optimization
```

---

## 🔍 **Advanced Features**

### **Debug Options**

#### **`--debug`**
Enable debug output and information.

```bash
utopia compile app.uto --target python --debug
utopia run app.uto --debug
```

#### **`--verbose` / `-v`**
Enable verbose output for detailed information.

```bash
utopia compile app.uto --target javascript --verbose
utopia run app.uto -v
```

#### **`--debug-parser`**
Enable parser-specific debugging.

```bash
utopia compile app.uto --target python --debug-parser
```

#### **`--debug-transformer`**
Enable transformer-specific debugging.

```bash
utopia compile app.uto --target javascript --debug-transformer
```

### **Cross-Language Features**

#### **`--allow-cross-calls`**
Enable cross-language function calls (default: enabled).

```bash
utopia compile app.uto --target python --allow-cross-calls
```

#### **`--strict-mode`**
Enable strict type checking and validation.

```bash
utopia compile app.uto --target typescript --strict-mode
```

### **Performance Options**

#### **`--profile`**
Enable performance profiling during compilation.

```bash
utopia compile app.uto --target python --profile
```

#### **`--benchmark`**
Run compilation benchmarks.

```bash
utopia compile app.uto --target javascript --benchmark
```

---

## ⚙️ **Configuration**

### **Configuration File**
Create a `utopia.toml` file in your project root:

```toml
[compiler]
default_target = "python"
optimization_level = 2
enable_debug = false

[output]
directory = "./build"
preserve_comments = true

[cross_language]
allow_calls = true
runtime_checks = true

[targets.python]
version = "3.8"
format_code = true

[targets.javascript]
es_version = "ES2020"
generate_source_maps = true

[targets.typescript]
strict_mode = true
generate_declarations = true
```

### **Environment Variables**

```bash
# Set default target language
export UTOPIA_DEFAULT_TARGET=python

# Set output directory
export UTOPIA_OUTPUT_DIR=./build

# Enable debug mode globally
export UTOPIA_DEBUG=1

# Set optimization level
export UTOPIA_OPTIMIZATION=2
```

### **Project Structure**
```
my-utopia-project/
├── utopia.toml          # Configuration file
├── src/
│   ├── main.uto         # Main source file
│   ├── utils.uto        # Utility functions
│   └── types.uto        # Type definitions
├── build/               # Compiled output
└── tests/
    └── test_main.uto    # Test files
```

---

## 🛠️ **Troubleshooting**

### **Common Issues**

#### **"Command not found: utopia"**
```bash
# Solution: Ensure Rust and Cargo are installed and PATH is set
echo $PATH | grep -q "$HOME/.cargo/bin" || echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc

# Reinstall if needed
cd utopia-rs
cargo install --path . --force
```

#### **"Compilation failed: Parse error"**
```bash
# Debug with verbose output
utopia compile problematic.uto --target python --debug-parser --verbose

# Check syntax with smaller test file
echo 'println("Hello, World!")' > test.uto
utopia compile test.uto --target python
```

#### **"Target language not supported"**
```bash
# Check available targets
utopia compile --help | grep -A 20 "TARGET LANGUAGES"

# Use supported target
utopia compile app.uto --target python  # Instead of unsupported target
```

#### **"Cross-language call failed"**
```bash
# Enable debug output for cross-language calls
utopia compile app.uto --target python --debug --verbose

# Check language block syntax
# Correct: @lang python
# Incorrect: @language python
```

### **Performance Issues**

#### **Slow Compilation**
```bash
# Use optimization flags
utopia compile large_app.uto --target python --optimize

# Check for recursive includes or large files
utopia compile large_app.uto --target python --profile
```

#### **Large Output Files**
```bash
# Use minification (when available)
utopia compile app.uto --target javascript --optimize --minify

# Split large applications
utopia compile main.uto --target python --output main.py
utopia compile utils.uto --target python --output utils.py
```

### **Getting Help**

```bash
# Command-specific help
utopia compile --help
utopia run --help
utopia repl --help

# Debug information
utopia --version
utopia compile test.uto --target python --debug --verbose
```

---

## 📝 **Examples**

### **Basic Compilation Workflow**

```bash
# 1. Create a simple Utopia file
cat > hello.uto << 'EOF'
@lang main

println("Hello, World!")
let name = "Developer"
println("Welcome,", name)
EOF

# 2. Compile to different targets
utopia compile hello.uto --target python --output hello.py
utopia compile hello.uto --target javascript --output hello.js
utopia compile hello.uto --target typescript --output hello.ts

# 3. Run the compiled outputs
python hello.py
node hello.js
npx ts-node hello.ts
```

### **Multi-Language Project**

```bash
# Create a multi-language application
cat > app.uto << 'EOF'
@lang python
def calculate_fibonacci(n):
    if n <= 1:
        return n
    return calculate_fibonacci(n-1) + calculate_fibonacci(n-2)

@lang javascript
function formatOutput(value) {
    return `Fibonacci result: ${value}`;
}

@lang main
let n = 10
let result = py::calculate_fibonacci(n)
let formatted = js::formatOutput(result)
println(formatted)
EOF

# Compile and run
utopia compile app.uto --target python --optimize
python app.py
```

### **Development Workflow**

```bash
# 1. Start with REPL for quick testing
utopia repl
> println("Testing...")
> let x = 5
> println("x =", x)
> exit

# 2. Develop in file with auto-compilation
utopia compile myapp.uto --target python --debug --verbose

# 3. Test the output
python myapp.py

# 4. Optimize for production
utopia compile myapp.uto --target python --optimize --output myapp_prod.py
```

### **Batch Compilation**

```bash
# Compile multiple files
for file in src/*.uto; do
    utopia compile "$file" --target python --output "build/$(basename "$file" .uto).py"
done

# Compile to multiple targets
targets=("python" "javascript" "typescript")
for target in "${targets[@]}"; do
    utopia compile app.uto --target "$target" --output "build/app.$target"
done
```

### **CI/CD Integration**

```bash
#!/bin/bash
# build.sh - Continuous Integration script

set -e

echo "Building Utopia project..."

# Clean build directory
rm -rf build/
mkdir -p build/

# Compile to all targets
utopia compile src/main.uto --target python --optimize --output build/main.py
utopia compile src/main.uto --target javascript --optimize --output build/main.js
utopia compile src/main.uto --target typescript --optimize --output build/main.ts

# Test compiled outputs
echo "Testing Python output..."
python build/main.py

echo "Testing JavaScript output..."
node build/main.js

echo "Testing TypeScript output..."
npx ts-node build/main.ts

echo "Build successful!"
```

---

## 🎯 **Performance Tips**

### **Compilation Speed**
- Use `--optimize` for production builds
- Enable `--profile` to identify bottlenecks
- Split large files into smaller modules
- Use configuration files to avoid repetitive flags

### **Output Quality**
- Use `--strict-mode` for TypeScript for better type safety
- Enable `--debug` during development for better error messages
- Use appropriate optimization levels for your target
- Test with `--verbose` to understand the compilation process

### **Development Efficiency**
- Use the REPL for quick experimentation
- Set up environment variables for common settings
- Create project configuration files
- Use batch scripts for multi-target compilation

---

<div align="center">

**Happy Compiling! 🚀**

*Master the Utopia CLI for efficient multi-language development*

[**🏠 Back to Docs**](README.md) • [**📖 Language Guide**](utopia-language-guide.md) • [**💻 Examples**](examples.md)

</div> 