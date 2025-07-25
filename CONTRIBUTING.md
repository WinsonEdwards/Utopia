# 🤝 **Contributing to Utopia**

Thank you for your interest in contributing to **Utopia**! We're building the future of multi-language programming, and we'd love your help. This guide will help you get started with contributing to the project.

## 🌟 **Why Contribute?**

- 🚀 **Shape the Future** - Help build the next generation of programming tools
- 🎓 **Learn & Grow** - Gain experience with Rust, compilers, and language design
- 🌍 **Global Impact** - Your contributions will help developers worldwide
- 🏆 **Recognition** - Get credited for your contributions and build your portfolio
- 🤝 **Community** - Join a passionate community of developers and researchers

---

## 🛠️ **Ways to Contribute**

### **🐛 Bug Reports & Issues**
Found a bug? We want to hear about it!
- Search [existing issues](https://github.com/WinsonEdwards/Utopia/issues) first
- Use our [bug report template](https://github.com/WinsonEdwards/Utopia/issues/new?template=bug_report.md)
- Include steps to reproduce, expected vs actual behavior
- Add your system info (OS, Rust version, Utopia version)

### **💡 Feature Requests**
Have an idea for improving Utopia?
- Check our [roadmap](README.md#roadmap) and existing discussions
- Use our [feature request template](https://github.com/WinsonEdwards/Utopia/issues/new?template=feature_request.md)
- Explain the use case and benefits
- Consider starting a [discussion](https://github.com/WinsonEdwards/Utopia/discussions) first

### **📝 Documentation**
Help make Utopia more accessible:
- Fix typos, improve clarity
- Add examples and tutorials
- Translate documentation
- Create video tutorials or blog posts

### **🧪 Testing**
Improve Utopia's reliability:
- Add test cases for edge cases
- Test on different platforms
- Performance testing and benchmarking
- Integration testing with real projects

### **🔧 Code Contributions**
The main event! Areas where we need help:

#### **High Priority**
- 🌐 **New Language Targets** (Go, Rust, C++, etc.)
- 🛠️ **Language Server Protocol (LSP)** for IDE support
- ⚡ **Performance Optimizations** in the compiler
- 🧪 **Test Coverage** expansion

#### **Medium Priority**
- 📦 **Package Manager** design and implementation
- 🔄 **Hot Reload** functionality
- 🎨 **Error Message** improvements
- 📊 **Debugging Tools** enhancement

#### **Good First Issues**
- 📚 Documentation improvements
- 🐛 Small bug fixes
- ✨ CLI enhancements
- 🎨 Code formatting and cleanup

---

## 🚀 **Getting Started**

### **1. Set Up Your Development Environment**

**Prerequisites:**
```bash
# Required
- Rust 1.70+ (https://rustup.rs/)
- Git
- A good text editor (VS Code, Vim, etc.)

# Optional but recommended  
- LLVM (for LLVM IR target)
- Node.js (for JavaScript testing)
- Python 3.8+ (for Python testing)
```

**Clone and Build:**
```bash
# Fork the repo on GitHub first, then:
git clone https://github.com/YOUR_USERNAME/Utopia.git
cd Utopia

# Set up the upstream remote
git remote add upstream https://github.com/WinsonEdwards/Utopia.git

# Build the project
cd utopia-rs
cargo build

# Run tests to ensure everything works
cargo test
```

### **2. Understand the Codebase**

**Project Structure:**
```
utopia/
├── utopia-rs/              # Main Rust compiler
│   ├── src/
│   │   ├── main.rs         # CLI entry point
│   │   ├── lexer.rs        # Tokenization
│   │   ├── parser.rs       # Parsing to AST
│   │   ├── ast.rs          # Abstract Syntax Tree
│   │   ├── transformers.rs # Code generation
│   │   ├── optimizer.rs    # Code optimization
│   │   └── runtime.rs      # Runtime support
│   └── Cargo.toml
├── tests/                  # Integration tests
├── examples/               # Example .uto files
├── docs/                   # Documentation
└── scripts/                # Build and test scripts
```

**Key Concepts:**
- **Lexer**: Converts source code into tokens
- **Parser**: Builds an Abstract Syntax Tree (AST) from tokens
- **Transformers**: Generate target language code from AST
- **Runtime**: Provides cross-language call support

### **3. Find an Issue to Work On**

**Browse Issues:**
- [Good First Issues](https://github.com/WinsonEdwards/Utopia/labels/good%20first%20issue) - Perfect for newcomers
- [Help Wanted](https://github.com/WinsonEdwards/Utopia/labels/help%20wanted) - Community contributions needed
- [Bug Reports](https://github.com/WinsonEdwards/Utopia/labels/bug) - Fix existing issues
- [Enhancements](https://github.com/WinsonEdwards/Utopia/labels/enhancement) - Add new features

**Comment on the Issue:**
Let us know you're working on it to avoid duplicate efforts!

---

## 💻 **Development Workflow**

### **1. Create a Feature Branch**
```bash
# Stay up to date
git checkout main
git pull upstream main

# Create a new branch
git checkout -b feature/your-feature-name
# or
git checkout -b fix/bug-description
```

### **2. Make Your Changes**

**Coding Standards:**
- Follow Rust conventions (`cargo fmt`)
- Run `cargo clippy` to catch common issues
- Add documentation for public APIs
- Write tests for new functionality
- Keep commits focused and atomic

**Testing:**
```bash
# Run unit tests
cargo test

# Run integration tests  
./scripts/quick_test.sh

# Run full test suite
./scripts/run_tests.sh

# Test specific language target
utopia compile tests/basic_syntax_test.uto --target python
```

### **3. Commit Your Changes**

**Commit Message Format:**
```
type(scope): short description

Longer description if needed.

Fixes #123
```

**Types:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `test`: Adding or updating tests
- `refactor`: Code refactoring
- `perf`: Performance improvements
- `style`: Code style changes

**Examples:**
```bash
git commit -m "feat(parser): add support for function declarations"
git commit -m "fix(lexer): handle edge case in string parsing"
git commit -m "docs(readme): improve installation instructions"
```

### **4. Submit a Pull Request**

**Before Submitting:**
```bash
# Ensure your branch is up to date
git checkout main
git pull upstream main
git checkout your-feature-branch
git rebase main

# Run final tests
cargo test
./scripts/quick_test.sh

# Push your changes
git push origin your-feature-branch
```

**Pull Request Guidelines:**
- Use our [PR template](https://github.com/WinsonEdwards/Utopia/blob/main/.github/pull_request_template.md)
- Link to relevant issues
- Describe what you changed and why
- Include screenshots for UI changes
- Update documentation if needed

---

## 🧪 **Testing Guidelines**

### **Writing Tests**

**Unit Tests (Rust):**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer_tokenizes_numbers() {
        let mut lexer = Lexer::new("42");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].kind, TokenKind::Number);
    }
}
```

**Integration Tests (.uto files):**
```utopia
// tests/your_feature_test.uto
@lang main

// Test your new feature
let result = your_new_function(42)
println("Result:", result)
```

**Test Organization:**
- Unit tests: Test individual functions/modules
- Integration tests: Test complete compilation pipeline
- Performance tests: Benchmark compilation speed
- Cross-platform tests: Ensure compatibility

### **Running Tests**

```bash
# Quick validation (recommended during development)
./scripts/quick_test.sh

# Full test suite (before submitting PR)
./scripts/run_tests.sh

# Specific test category
cargo test parser
cargo test --test integration_tests

# With verbose output
cargo test -- --nocapture
```

---

## 🏗️ **Architecture Guidelines**

### **Adding a New Language Target**

1. **Create the Transformer:**
```rust
// In src/transformers.rs
pub struct YourLanguageTransformer {
    // Configuration
}

impl Transformer for YourLanguageTransformer {
    fn transform(&self, program: &Program) -> Result<String> {
        // Implement code generation
    }
    
    fn target_name(&self) -> &str { "your_language" }
    fn file_extension(&self) -> &str { ".your_ext" }
}
```

2. **Register the Transformer:**
```rust
// In src/transformers.rs
pub fn get_transformer(target: &str) -> Option<Box<dyn Transformer>> {
    match target {
        // ... existing transformers
        "your_language" => Some(Box::new(YourLanguageTransformer::new())),
        _ => None,
    }
}
```

3. **Add Tests:**
```rust
#[test]
fn test_your_language_basic_syntax() {
    let source = r#"
        @lang main
        println("Hello from your language!")
    "#;
    
    let result = compile_to_target(source, "your_language").unwrap();
    assert!(result.contains("your expected output"));
}
```

### **Parser Extensions**

When adding new syntax:
1. Update the lexer for new tokens
2. Extend the AST with new node types
3. Update the parser to handle new syntax
4. Update all transformers to handle new AST nodes
5. Add comprehensive tests

---

## 📚 **Resources for Contributors**

### **Learning Materials**
- 📖 [Crafting Interpreters](https://craftinginterpreters.com/) - Great intro to language implementation
- 📖 [The Rust Programming Language](https://doc.rust-lang.org/book/) - Official Rust book
- 📖 [Engineering a Compiler](https://www.amazon.com/Engineering-Compiler-Keith-Cooper/dp/012088478X) - Advanced compiler techniques

### **Rust-Specific**
- 🦀 [Rust By Example](https://doc.rust-lang.org/rust-by-example/)
- 🦀 [The Rustonomicon](https://doc.rust-lang.org/nomicon/) - Advanced Rust
- 🦀 [Rust Compiler Team](https://forge.rust-lang.org/) - How Rust is built

### **Compiler Design**
- 🔧 [LLVM Tutorial](https://llvm.org/docs/tutorial/) - Building languages with LLVM
- 🔧 [Modern Compiler Implementation](https://www.cs.princeton.edu/~appel/modern/) - Tiger book
- 🔧 [AST Explorer](https://astexplorer.net/) - Visualize ASTs for different languages

---

## 🎯 **Contributor Recognition**

We believe in recognizing our contributors:

### **Ways We Say Thank You**
- 🏆 **Contributors List** in README and releases
- 🎖️ **Special GitHub Badges** for significant contributions
- 📢 **Shoutouts** on social media and blog posts
- 🎁 **Swag** for major contributors (stickers, t-shirts)
- 📧 **Reference Letters** for job applications (upon request)

### **Contribution Levels**
- **🌱 First-Time Contributor**: Your first merged PR
- **🌿 Regular Contributor**: 5+ merged PRs
- **🌳 Core Contributor**: 20+ merged PRs or major features
- **🏆 Maintainer**: Trusted with repository access

---

## ❓ **Getting Help**

Stuck? Don't worry, we're here to help!

### **Communication Channels**
- 💬 **GitHub Discussions**: [Ask questions, share ideas](https://github.com/WinsonEdwards/Utopia/discussions)
- 🐛 **GitHub Issues**: [Technical problems](https://github.com/WinsonEdwards/Utopia/issues)
- 📧 **Email**: For sensitive matters, contact the maintainers
- 📱 **Community Chat**: Coming soon!

### **What to Include When Asking for Help**
- What you're trying to do
- What you've tried so far
- Full error messages
- Your environment (OS, Rust version, etc.)
- Relevant code snippets

### **Response Times**
- Issues and PRs: Usually within 24-48 hours
- Discussions: Within a week
- Email: Within a week for non-urgent matters

---

## 📜 **Code of Conduct**

We are committed to providing a welcoming and inclusive environment for all contributors. Please read and follow our [Code of Conduct](CODE_OF_CONDUCT.md).

**In Summary:**
- Be respectful and inclusive
- Welcome newcomers and help them learn
- Focus on constructive feedback
- Respect different opinions and approaches
- Report unacceptable behavior to the maintainers

---

## 🙏 **Thank You!**

Every contribution, no matter how small, makes Utopia better. Whether you're fixing a typo, adding a new feature, or helping other contributors, you're part of building something amazing.

**Ready to contribute?** [Check out the open issues](https://github.com/WinsonEdwards/Utopia/issues) and jump in!

---

<div align="center">

**Happy Coding! 🚀**

*Together, we're building the future of programming.*

</div> 