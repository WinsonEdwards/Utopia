# Utopia Testing and Debugging Guide

This guide provides comprehensive testing and debugging procedures for the Utopia multi-language compiler.

## Table of Contents

1. [Unit Testing](#unit-testing)
2. [Integration Testing](#integration-testing)
3. [Compiler Testing](#compiler-testing)
4. [Language Target Testing](#language-target-testing)
5. [Performance Testing](#performance-testing)
6. [Debugging Tools](#debugging-tools)
7. [Error Handling](#error-handling)
8. [Test Automation](#test-automation)

## Unit Testing

### Running Unit Tests

```bash
# Run all unit tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test module
cargo test parser

# Run tests with verbose output
cargo test --verbose

# Run tests in parallel
cargo test --jobs 4
```

### Test Structure

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_basic() {
        let input = "let x = 5;";
        let result = parse(input);
        assert!(result.is_ok());
    }

    #[test]
    fn test_transformer_python() {
        let ast = create_test_ast();
        let transformer = PythonTransformer::new();
        let result = transformer.transform(&ast);
        assert!(result.is_ok());
    }
}
```

## Integration Testing

### End-to-End Compilation Tests

```bash
# Test Python compilation
utopia compile examples/basic_example.uto --target python --output test.py
python3 test.py

# Test JavaScript compilation
utopia compile examples/basic_example.uto --target javascript --output test.js
node test.js

# Test C compilation
utopia compile examples/basic_example.uto --target c --output test.c
gcc test.c -o test
./test

# Test Rust compilation
utopia compile examples/basic_example.uto --target rust --output test.rs
rustc test.rs -o test
./test
```

### Cross-Language Integration Tests

```bash
# Test Python to JavaScript cross-calls
utopia compile examples/basic_example.uto --target javascript --output cross_test.js
node cross_test.js

# Test JavaScript to Python cross-calls
utopia compile examples/basic_example.uto --target python --output cross_test.py
python3 cross_test.py
```

## Compiler Testing

### Syntax Validation Tests

```bash
# Test valid syntax
utopia check examples/basic_example.uto

# Test invalid syntax (should fail)
echo "invalid syntax here" > invalid.uto
utopia check invalid.uto

# Test with verbose output
utopia check examples/basic_example.uto --verbose
```

### Compilation Pipeline Tests

```bash
# Test lexer
utopia analyze examples/basic_example.uto --lexer

# Test parser
utopia analyze examples/basic_example.uto --parser

# Test AST generation
utopia analyze examples/basic_example.uto --ast

# Test type checking
utopia analyze examples/basic_example.uto --types
```

## Language Target Testing

### Python Target Tests

```bash
# Test basic Python features
utopia compile test_python.uto --target python --output test.py
python3 -m py_compile test.py  # Check syntax
python3 test.py  # Run and verify output

# Test Python-specific features
echo '@lang python { function test() { return "Hello"; } }' > python_test.uto
utopia compile python_test.uto --target python --output python_test.py
```

### JavaScript Target Tests

```bash
# Test basic JavaScript features
utopia compile test_js.uto --target javascript --output test.js
node -c test.js  # Check syntax
node test.js  # Run and verify output

# Test JavaScript-specific features
echo '@lang javascript { function test() { return "Hello"; } }' > js_test.uto
utopia compile js_test.uto --target javascript --output js_test.js
```

### C Target Tests

```bash
# Test C compilation
utopia compile test_c.uto --target c --output test.c
gcc -Wall -Wextra test.c -o test
./test

# Test with different C standards
gcc -std=c99 test.c -o test_c99
gcc -std=c11 test.c -o test_c11
```

### Assembly Target Tests

```bash
# Test x86_64 assembly generation
utopia compile test_asm.uto --target assembly --output test.s
# Note: May require specific assembler for your platform
```

## Performance Testing

### Compilation Speed Tests

```bash
# Time compilation
time utopia compile examples/basic_example.uto --target python

# Profile compilation
cargo build --release
perf record --call-graph=dwarf utopia compile examples/basic_example.uto --target python
perf report

# Memory usage during compilation
valgrind --tool=massif utopia compile examples/basic_example.uto --target python
ms_print massif.out.*
```

### Runtime Performance Tests

```bash
# Benchmark generated code
utopia benchmark examples/basic_example.uto --target python
utopia benchmark examples/basic_example.uto --target javascript
utopia benchmark examples/basic_example.uto --target c
```

## Debugging Tools

### Compiler Debugging

```bash
# Enable debug logging
RUST_LOG=debug utopia compile examples/basic_example.uto --target python

# Enable trace logging
RUST_LOG=trace utopia compile examples/basic_example.uto --target python

# Debug specific components
RUST_LOG=utopia::parser=debug utopia compile examples/basic_example.uto --target python
RUST_LOG=utopia::transformer=debug utopia compile examples/basic_example.uto --target python
```

### Interactive Debugging

```bash
# Start REPL with debug info
utopia repl --debug

# Debug specific language block
utopia repl --lang python --debug
```

### Code Analysis

```bash
# Analyze code structure
utopia analyze examples/basic_example.uto --detailed

# Show AST
utopia analyze examples/basic_example.uto --ast --pretty

# Show symbol table
utopia analyze examples/basic_example.uto --symbols
```

## Error Handling

### Common Error Patterns

```bash
# Test syntax errors
echo "invalid syntax" > syntax_error.uto
utopia compile syntax_error.uto --target python

# Test type errors
echo "let x: string = 42;" > type_error.uto
utopia compile type_error.uto --target python

# Test semantic errors
echo "function test() { return undefined_variable; }" > semantic_error.uto
utopia compile semantic_error.uto --target python
```

### Error Recovery

```bash
# Test error recovery in REPL
utopia repl
> let x = 5;
> let y = ;  # Syntax error
> let z = 10;  # Should continue working
```

## Test Automation

### Automated Test Suite

```bash
# Run all tests
./scripts/run_tests.sh

# Run specific test categories
./scripts/run_tests.sh --unit
./scripts/run_tests.sh --integration
./scripts/run_tests.sh --performance

# Generate test report
./scripts/run_tests.sh --report
```

### Continuous Integration

```bash
# GitHub Actions workflow
.github/workflows/test.yml

# Local CI simulation
./scripts/ci_local.sh
```

## Test Files

### Sample Test Files

```bash
# Create test files for each language target
mkdir -p tests/targets
touch tests/targets/python_basic.uto
touch tests/targets/javascript_basic.uto
touch tests/targets/c_basic.uto
touch tests/targets/rust_basic.uto
```

### Expected Output Files

```bash
# Store expected outputs
mkdir -p tests/expected
touch tests/expected/python_basic.py
touch tests/expected/javascript_basic.js
touch tests/expected/c_basic.c
touch tests/expected/rust_basic.rs
```

## Debugging Checklist

### Before Reporting Issues

- [ ] Run `cargo test` to ensure all tests pass
- [ ] Check if issue reproduces with `--verbose` flag
- [ ] Verify issue occurs with latest commit
- [ ] Test with different language targets
- [ ] Check generated output files
- [ ] Review error messages and stack traces

### Performance Issues

- [ ] Profile with `perf` or `valgrind`
- [ ] Check memory usage patterns
- [ ] Compare with previous versions
- [ ] Test with different input sizes
- [ ] Verify optimization flags are enabled

### Language Target Issues

- [ ] Test with minimal example
- [ ] Verify target language syntax
- [ ] Check for language-specific features
- [ ] Test cross-language calls
- [ ] Validate generated code manually

## Quick Debug Commands

```bash
# Quick syntax check
utopia check file.uto

# Quick compilation test
utopia compile file.uto --target python --output test.py && python3 test.py

# Quick REPL test
echo "let x = 5; println(x);" | utopia repl

# Quick performance test
time utopia compile file.uto --target python

# Quick memory test
valgrind --leak-check=full utopia compile file.uto --target python
```

This testing and debugging guide provides a comprehensive framework for ensuring the reliability and performance of the Utopia compiler. 