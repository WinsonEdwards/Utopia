# Utopia Examples

This directory contains example Utopia programs demonstrating various features.

## Basic Examples

- `simple_test.uto` - Basic syntax and operations
- `basic_example.uto` - Variables, functions, and control flow

## Advanced Examples

- `unified_syntax_simple.uto` - Simple cross-language syntax
- `utopia_unified_syntax_demo.uto` - Comprehensive syntax demonstration
- `multi_lang_showcase.uto` - Multi-language integration examples

## Real-world Examples

- `web_api.uto` - Web API development example
- `data_processing.uto` - Data processing and analysis

## Running Examples

```bash
# Run directly
utopia run simple_test.uto

# Compile to specific language
utopia compile basic_example.uto --target python
utopia compile multi_lang_showcase.uto --target javascript
```

## Creating Your Own Examples

1. Create a `.uto` file with your code
2. Use `@lang` blocks for language-specific code
3. Use `@lang main` for the main program logic
4. Test with `utopia run` or `utopia compile` 