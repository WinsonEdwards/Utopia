//! Utopia Multi-Language Compiler
//! 
//! A revolutionary programming language that allows mixing multiple languages
//! in a single file with cross-language function calls and unified optimization.

use colored::*;

fn main() {
    // Initialize logging
    env_logger::init();
    
    // Print banner
    println!("{}", "🚀 Utopia Multi-Language Compiler v0.3.0".bright_blue().bold());
    println!("{}", "Ultimate Performance Edition - Built with Rust".bright_green());
    println!();
    
    // Debug mode for testing
    if std::env::args().any(|arg| arg == "--debug-tokens") {
        let input = "let x = 5;";
        println!("Debug: Tokenizing '{}'", input);
        
        // Show the input character by character
        println!("Input chars:");
        for (i, ch) in input.char_indices() {
            println!("  [{}] '{}' (ASCII: {})", i, ch, ch as u32);
        }
        println!();
        
        let mut lexer = utopia::Lexer::new(input);
        match lexer.tokenize() {
            Ok(tokens) => {
                for (i, token) in tokens.iter().enumerate() {
                    println!("Token {}: {:?}", i, token);
                }
            }
            Err(e) => {
                println!("Lexer error: {}", e);
            }
        }
        return;
    }
    
    // Debug mode for specific string
    if std::env::args().any(|arg| arg == "--debug-equals") {
        let inputs = vec!["=", "==", "= ", " =", "x=5", "x = 5"];
        for input in inputs {
            println!("Testing input: '{}'", input);
            let mut lexer = utopia::Lexer::new(input);
            match lexer.tokenize() {
                Ok(tokens) => {
                    for token in tokens {
                        println!("  {:?}", token);
                    }
                }
                Err(e) => println!("  Error: {}", e),
            }
            println!();
        }
        return;
    }

    // Debug mode for colon tokenization
    if std::env::args().any(|arg| arg == "--debug-colons") {
        let inputs = vec![":", "::", "python::helper", "a:b", "a::b"];
        for input in inputs {
            println!("Testing input: '{}'", input);
            let mut lexer = utopia::Lexer::new(input);
            match lexer.tokenize() {
                Ok(tokens) => {
                    for token in tokens {
                        println!("  {:?}", token);
                    }
                }
                Err(e) => println!("  Error: {}", e),
            }
            println!();
        }
        return;
    }
    
    // Run normal CLI
    if let Err(e) = utopia::cli::run_cli() {
        eprintln!("{}", format!("Error: {}", e).bright_red());
        std::process::exit(1);
    }
}
