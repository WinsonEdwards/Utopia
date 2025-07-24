//! # Utopia Multi-Language Compiler
//! 
//! A revolutionary programming language that allows mixing multiple languages
//! in a single file with cross-language function calls and unified optimization.
//! 
//! ## Features
//! 
//! - **15+ supported languages**: Python, JavaScript, TypeScript, Rust, Go, Java, C, C++, Assembly, CUDA, and more
//! - **Cross-language calls**: Call functions between different languages seamlessly
//! - **Unified optimization**: World-class compiler optimizations across all languages
//! - **Ultimate performance**: Faster than C through advanced optimization techniques
//! - **Multiple transformers**: Compile to Assembly, LLVM IR, WebAssembly, CUDA, Embedded C
//! - **Reverse compilation**: Convert any language to Utopia format for unified optimization
//! 
//! ## Example
//! 
//! ```utopia
//! @lang python
//! def ai_analysis(data):
//!     return machine_learning_model(data)
//! 
//! @lang cuda
//! kernel function gpu_compute(input, output, size) {
//!     int tid = getThreadId();
//!     output[tid] = input[tid] * input[tid];
//! }
//! 
//! @lang assembly
//! function critical_math(x, y) {
//!     mov rax, rdi
//!     imul rax, rsi
//!     ret
//! }
//! 
//! @lang javascript
//! function main() {
//!     let data = [1, 2, 3, 4, 5];
//!     let ai_result = python::ai_analysis(data);
//!     cuda::gpu_compute(data, result, data.length);
//!     let fast_math = assembly::critical_math(42, 13);
//! }
//! ```

#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

use serde::{Deserialize, Serialize};

pub mod ast;
pub mod transformers;
pub mod cli;
pub mod config;
pub mod diagnostics;
pub mod lexer;
pub mod optimizer;
pub mod parser;
pub mod reverse;
pub mod types;
pub mod utils;

// Re-export commonly used types
pub use ast::*;
pub use transformers::*;
pub use cli::*;
pub use config::*;
pub use diagnostics::*;
pub use lexer::*;
pub use optimizer::*;
pub use parser::*;
pub use reverse::*;
pub use types::*;

/// Result type used throughout the compiler
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Span representing a location in source code
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Span {
    pub start: usize,
    pub end: usize,
    pub line: usize,
    pub column: usize,
}

impl Span {
    pub fn new(start: usize, end: usize, line: usize, column: usize) -> Self {
        Self {
            start,
            end,
            line,
            column,
        }
    }

    pub fn len(&self) -> usize {
        self.end - self.start
    }

    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }
}

/// Main compiler structure
pub struct Compiler {
    config: Config,
    type_system: TypeSystem,
    optimizer: Optimizer,
    transformers: std::collections::HashMap<String, Box<dyn Transformer>>,
}

impl Compiler {
    /// Create a new compiler instance
    pub fn new(config: Config) -> Self {
        let type_system = TypeSystem::new();
        let optimizer = Optimizer::new();
        let transformers = std::collections::HashMap::new();

        Self {
            config,
            type_system,
            optimizer,
            transformers,
        }
    }

    /// Compile source code to target language
    pub fn compile(&mut self, source: &str, target: &str) -> Result<String> {
        // Lexical analysis
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize()?;

        // Syntax analysis
        let mut parser = Parser::new(tokens);
        let ast = parser.parse()?;

        // Type checking
        let typed_ast = self.type_system.check(&ast)?;

        // Optimization
        let optimized_ast = self.optimizer.optimize(typed_ast)?;

        // Code generation
        let transformer = self.transformers.get(target)
            .ok_or_else(|| format!("Unknown target: {}", target))?;
        
        transformer.transform(&optimized_ast)
    }

    /// Add a transformer for code generation
    pub fn add_transformer(&mut self, name: String, transformer: Box<dyn Transformer>) {
        self.transformers.insert(name, transformer);
    }

    /// Validate source code without compilation
    pub fn validate(&self, source: &str) -> Result<Vec<Diagnostic>> {
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize()?;

        let mut parser = Parser::new(tokens);
        let ast = parser.parse()?;

        self.type_system.validate(&ast)
    }

    /// Analyze source code and return metadata
    pub fn analyze(&self, source: &str) -> Result<ast::Metadata> {
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize()?;

        let mut parser = Parser::new(tokens);
        let ast = parser.parse()?;

        Ok(ast.metadata())
    }
}

/// Utopia compiler version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Utopia compiler name
pub const NAME: &str = env!("CARGO_PKG_NAME");

/// Utopia compiler description
pub const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION"); 