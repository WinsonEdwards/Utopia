# Contributing to Utopia

Thank you for your interest in contributing to Utopia. This document provides guidelines for contributing to the project.

## Development Setup

### Prerequisites

- Rust (latest stable version)
- Cargo package manager
- Git

### Getting Started

1. Fork the repository
2. Clone your fork locally:
   ```bash
   git clone https://github.com/your-username/utopia.git
   cd utopia
   ```

3. Build the project:
   ```bash
   cd utopia-rs
   cargo build
   ```

4. Run tests to ensure everything works:
   ```bash
   cargo test
   ```

## Code Style

- Follow standard Rust conventions
- Use meaningful variable and function names
- Add comments for complex logic
- Keep functions focused and reasonably sized
- Run `cargo fmt` before committing

## Making Changes

### Adding Language Support

To add support for a new programming language:

1. Create a new transformer in `utopia-rs/src/transformers.rs`
2. Implement the `Transformer` trait for your language
3. Register the new transformer in `TransformerManager::new()`
4. Add test cases in the `tests/` directory
5. Update documentation to include the new language

### Bug Fixes

1. Create an issue describing the bug
2. Write a failing test that reproduces the issue
3. Fix the bug
4. Ensure all tests pass
5. Submit a pull request

### New Features

1. Discuss the feature in an issue first
2. Write tests for the new functionality
3. Implement the feature
4. Update documentation as needed
5. Submit a pull request

## Testing

Run the test suite before submitting changes:

```bash
cargo test
```

For testing specific language outputs:

```bash
cargo run -- compile tests/basic_syntax_test.uto --target python
```

## Pull Request Guidelines

1. Use clear, descriptive commit messages
2. Include tests for new functionality
3. Update documentation when adding features
4. Ensure all existing tests pass
5. Keep pull requests focused on a single change

## Documentation

- Update relevant documentation when adding features
- Use clear, concise language
- Include code examples where helpful
- Test documentation examples to ensure they work

## Questions or Issues

If you have questions about contributing, please:

1. Check existing issues and discussions
2. Create a new issue if your question hasn't been addressed
3. Be specific about your environment and the issue you're facing

## Code of Conduct

This project follows standard open source community guidelines. Be respectful and constructive in all interactions.

## License

By contributing to Utopia, you agree that your contributions will be licensed under the MIT License. 