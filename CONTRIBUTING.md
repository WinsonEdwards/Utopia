# Contributing to Utopia

Thank you for your interest in contributing to Utopia! This document provides guidelines for contributing to the project.

## Development Setup

1. **Clone the repository**
   ```bash
   git clone https://github.com/your-username/utopia.git
   cd utopia
   ```

2. **Build the compiler**
   ```bash
   cd utopia-rs
   cargo build
   ```

3. **Run tests**
   ```bash
   cargo test
   ```

## Code Style

- Follow Rust conventions for Rust code
- Use meaningful variable and function names
- Add comments for complex logic
- Keep functions small and focused

## Adding New Features

### Adding a New Language Target

1. Create a new transformer in `utopia-rs/src/transformers.rs`
2. Implement the `Transformer` trait
3. Register the transformer in `TransformerManager::new()`
4. Add tests in `tests/`
5. Update documentation

### Adding New CLI Commands

1. Add the command to `utopia-rs/src/cli.rs`
2. Implement the command handler
3. Add help text
4. Add tests
5. Update documentation

## Testing

- Write unit tests for new features
- Test with multiple language targets
- Ensure cross-language calls work correctly
- Run the full test suite before submitting

## Submitting Changes

1. Create a feature branch
2. Make your changes
3. Add tests
4. Update documentation
5. Submit a pull request

## Documentation

When adding new features, please update:
- `README.md` for user-facing changes
- `docs/` for detailed documentation
- `examples/` for code examples

## Questions?

Feel free to open an issue for questions or discussions about the project. 