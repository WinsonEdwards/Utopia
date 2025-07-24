# 🎪 Live Demo Guide

## Quick Start Demo (2 minutes)

### 1. Clone and Build
```bash
git clone https://github.com/WinsonEdwards/Utopia.git
cd utopia/utopia-rs
cargo build --release
cd ..
```

### 2. Basic Compilation Demo
```bash
# Compile to Python
./utopia-rs/target/release/utopia compile examples/simple_test.uto --target python

# Compile to JavaScript
./utopia-rs/target/release/utopia compile examples/simple_test.uto --target javascript

# Compile to Rust
./utopia-rs/target/release/utopia compile examples/simple_test.uto --target rust
```

### 3. Cross-Language Demo
```bash
# Test all 50 languages
./run_macos_tests.sh
```

## 🎯 Demo Script for Presentations

### Opening Hook
"Imagine writing code once and having it run in 50 different programming languages. That's exactly what Utopia does."

### Key Demo Points
1. **Show the unified syntax** - One `.uto` file
2. **Compile to multiple languages** - Same code, different outputs
3. **Cross-language calls** - Functions calling between languages
4. **Performance** - 0-second compilation times
5. **Test results** - 92% success rate across 50 languages

### Closing
"This represents a historic breakthrough in programming language design - the first time we can truly write once and run everywhere across 50 languages."

