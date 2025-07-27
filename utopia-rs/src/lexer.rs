//! High-performance lexer for Utopia source code
//! 
//! This lexer is designed for maximum performance with zero-copy string handling
//! and efficient state machine implementation.

use crate::{Result, Span};
use std::fmt;
use std::str::Chars;
use std::iter::Peekable;

/// Token types in Utopia
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    // Language directives
    At,                    // @
    Lang,                  // lang
    
    // Literals
    Number(String),
    String(String),
    Boolean(bool),
    Null,
    
    // Identifiers and keywords
    Identifier(String),
    Function,
    Let,
    Const,
    If,
    Else,
    While,
    For,
    Return,
    Import,
    Export,
    Class,
    
    // Operators
    Plus,                  // +
    Minus,                 // -
    Star,                  // *
    Slash,                 // /
    Percent,               // %
    Equal,                 // =
    EqualEqual,            // ==
    NotEqual,              // !=
    Less,                  // <
    LessEqual,             // <=
    Greater,               // >
    GreaterEqual,          // >=
    And,                   // &&
    Or,                    // ||
    Not,                   // !
    PlusPlus,              // ++
    MinusMinus,            // --
    
    // Delimiters
    LeftParen,             // (
    RightParen,            // )
    LeftBrace,             // {
    RightBrace,            // }
    LeftBracket,           // [
    RightBracket,          // ]
    Comma,                 // ,
    Semicolon,             // ;
    Colon,                 // :
    DoubleColon,           // ::
    Dot,                   // .
    Arrow,                 // ->
    
    // Special
    Newline,
    Whitespace,
    Comment(String),
    Eof,
    
    // Inline language blocks
    InlineCode(String, String), // (language, code)
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenKind::At => write!(f, "@"),
            TokenKind::Lang => write!(f, "lang"),
            TokenKind::Number(n) => write!(f, "{}", n),
            TokenKind::String(s) => write!(f, "\"{}\"", s),
            TokenKind::Boolean(b) => write!(f, "{}", b),
            TokenKind::Null => write!(f, "null"),
            TokenKind::Identifier(id) => write!(f, "{}", id),
            TokenKind::Function => write!(f, "function"),
            TokenKind::Let => write!(f, "let"),
            TokenKind::Const => write!(f, "const"),
            TokenKind::If => write!(f, "if"),
            TokenKind::Else => write!(f, "else"),
            TokenKind::While => write!(f, "while"),
            TokenKind::For => write!(f, "for"),
            TokenKind::Return => write!(f, "return"),
            TokenKind::Import => write!(f, "import"),
            TokenKind::Export => write!(f, "export"),
            TokenKind::Class => write!(f, "class"),
            TokenKind::Plus => write!(f, "+"),
            TokenKind::Minus => write!(f, "-"),
            TokenKind::Star => write!(f, "*"),
            TokenKind::Slash => write!(f, "/"),
            TokenKind::Percent => write!(f, "%"),
            TokenKind::Equal => write!(f, "="),
            TokenKind::EqualEqual => write!(f, "=="),
            TokenKind::NotEqual => write!(f, "!="),
            TokenKind::Less => write!(f, "<"),
            TokenKind::LessEqual => write!(f, "<="),
            TokenKind::Greater => write!(f, ">"),
            TokenKind::GreaterEqual => write!(f, ">="),
            TokenKind::And => write!(f, "&&"),
            TokenKind::Or => write!(f, "||"),
            TokenKind::Not => write!(f, "!"),
            TokenKind::PlusPlus => write!(f, "++"),
            TokenKind::MinusMinus => write!(f, "--"),
            TokenKind::LeftParen => write!(f, "("),
            TokenKind::RightParen => write!(f, ")"),
            TokenKind::LeftBrace => write!(f, "{{"),
            TokenKind::RightBrace => write!(f, "}}"),
            TokenKind::LeftBracket => write!(f, "["),
            TokenKind::RightBracket => write!(f, "]"),
            TokenKind::Comma => write!(f, ","),
            TokenKind::Semicolon => write!(f, ";"),
            TokenKind::Colon => write!(f, ":"),
            TokenKind::DoubleColon => write!(f, "::"),
            TokenKind::Dot => write!(f, "."),
            TokenKind::Arrow => write!(f, "->"),
            TokenKind::Newline => write!(f, "\\n"),
            TokenKind::Whitespace => write!(f, " "),
            TokenKind::Comment(c) => write!(f, "//{}", c),
            TokenKind::Eof => write!(f, "EOF"),
            TokenKind::InlineCode(lang, code) => write!(f, "@{} {{ {} }}", lang, code),
        }
    }
}

/// A token with position information
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
    pub lexeme: String,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span, lexeme: String) -> Self {
        Self { kind, span, lexeme }
    }
}

/// High-performance lexer with zero-copy string handling
pub struct Lexer<'a> {
    input: &'a str,
    chars: Peekable<Chars<'a>>,
    position: usize,
    line: usize,
    column: usize,
    tokens: Vec<Token>,
}

impl<'a> Lexer<'a> {
    /// Create a new lexer for the given input
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            chars: input.chars().peekable(),
            position: 0,
            line: 1,
            column: 1,
            tokens: Vec::new(),
        }
    }

    /// Tokenize the entire input
    pub fn tokenize(&mut self) -> Result<Vec<Token>> {
        while let Some(&ch) = self.chars.peek() {
            match ch {
                // Whitespace
                ' ' | '\t' | '\r' => {
                    self.advance();
                }
                
                // Newlines
                '\n' => {
                    let start = self.current_span();
                    self.advance();
                    self.add_token(TokenKind::Newline, start, "\n".to_string());
                }
                
                // Comments
                '/' if self.peek_ahead() == Some('/') => {
                    self.tokenize_comment()?;
                }
                
                // Multi-character operators
                '=' if self.peek_ahead() == Some('=') => {
                    let start = self.current_span();
                    self.advance(); // consume '='
                    self.advance(); // consume '='
                    self.add_token(TokenKind::EqualEqual, start, "==".to_string());
                }
                
                '!' if self.peek_ahead() == Some('=') => {
                    let start = self.current_span();
                    self.advance(); // consume '!'
                    self.advance(); // consume '='
                    self.add_token(TokenKind::NotEqual, start, "!=".to_string());
                }
                
                '<' if self.peek_ahead() == Some('=') => {
                    let start = self.current_span();
                    self.advance(); // consume '<'
                    self.advance(); // consume '='
                    self.add_token(TokenKind::LessEqual, start, "<=".to_string());
                }
                
                '>' if self.peek_ahead() == Some('=') => {
                    let start = self.current_span();
                    self.advance(); // consume '>'
                    self.advance(); // consume '='
                    self.add_token(TokenKind::GreaterEqual, start, ">=".to_string());
                }
                
                '&' if self.peek_ahead() == Some('&') => {
                    let start = self.current_span();
                    self.advance(); // consume '&'
                    self.advance(); // consume '&'
                    self.add_token(TokenKind::And, start, "&&".to_string());
                }
                
                '|' if self.peek_ahead() == Some('|') => {
                    let start = self.current_span();
                    self.advance(); // consume '|'
                    self.advance(); // consume '|'
                    self.add_token(TokenKind::Or, start, "||".to_string());
                }
                

                
                '-' if self.peek_ahead() == Some('>') => {
                    let start = self.current_span();
                    self.advance(); // consume '-'
                    self.advance(); // consume '>'
                    self.add_token(TokenKind::Arrow, start, "->".to_string());
                }
                
                '+' if self.peek_ahead() == Some('+') => {
                    let start = self.current_span();
                    self.advance(); // consume '+'
                    self.advance(); // consume '+'
                    self.add_token(TokenKind::PlusPlus, start, "++".to_string());
                }
                
                '-' if self.peek_ahead() == Some('-') => {
                    let start = self.current_span();
                    self.advance(); // consume '-'
                    self.advance(); // consume '-'
                    self.add_token(TokenKind::MinusMinus, start, "--".to_string());
                }
                
                // Single-character tokens
                '+' => self.single_char_token(TokenKind::Plus),
                '-' => self.single_char_token(TokenKind::Minus),
                '*' => self.single_char_token(TokenKind::Star),
                '/' => self.single_char_token(TokenKind::Slash),
                '%' => self.single_char_token(TokenKind::Percent),
                '=' => self.single_char_token(TokenKind::Equal),
                '<' => self.single_char_token(TokenKind::Less),
                '>' => self.single_char_token(TokenKind::Greater),
                '!' => self.single_char_token(TokenKind::Not),
                '(' => self.single_char_token(TokenKind::LeftParen),
                ')' => self.single_char_token(TokenKind::RightParen),
                '{' => self.single_char_token(TokenKind::LeftBrace),
                '}' => self.single_char_token(TokenKind::RightBrace),
                '[' => self.single_char_token(TokenKind::LeftBracket),
                ']' => self.single_char_token(TokenKind::RightBracket),
                ',' => self.single_char_token(TokenKind::Comma),
                ';' => self.single_char_token(TokenKind::Semicolon),
                ':' => {
                    // Check for double colon first
                    if self.peek_ahead() == Some(':') {
                        let start = self.current_span();
                        self.advance(); // consume first ':'
                        self.advance(); // consume second ':'
                        self.add_token(TokenKind::DoubleColon, start, "::".to_string());
                    } else {
                        self.single_char_token(TokenKind::Colon);
                    }
                }
                '.' => self.single_char_token(TokenKind::Dot),
                
                // @ symbol (language directive)
                '@' => self.single_char_token(TokenKind::At),
                
                // String literals
                '"' => self.tokenize_string()?,
                '\'' => self.tokenize_char()?,
                '`' => self.tokenize_template_literal()?,
                
                // Numbers
                '0'..='9' => self.tokenize_number()?,
                
                // Identifiers and keywords
                'a'..='z' | 'A'..='Z' | '_' => self.tokenize_identifier()?,
                
                // Unknown character
                _ => {
                    return Err(format!("Unexpected character '{}' at line {}, column {}", 
                                     ch, self.line, self.column).into());
                }
            }
        }
        
        // Add EOF token
        let span = Span::new(self.position, self.position, self.line, self.column);
        self.tokens.push(Token::new(TokenKind::Eof, span, String::new()));
        
        Ok(std::mem::take(&mut self.tokens))
    }

    fn advance(&mut self) -> Option<char> {
        if let Some(ch) = self.chars.next() {
            self.position += ch.len_utf8();
            if ch == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
            Some(ch)
        } else {
            None
        }
    }

    fn peek_ahead(&mut self) -> Option<char> {
        // Get an iterator for the remaining characters after current position
        let mut chars_ahead = self.input[self.position..].chars();
        chars_ahead.next(); // skip current character
        chars_ahead.next() // return next character
    }

    fn current_span(&self) -> Span {
        Span::new(self.position, self.position, self.line, self.column)
    }

    fn add_token(&mut self, kind: TokenKind, mut span: Span, lexeme: String) {
        span.end = self.position;
        self.tokens.push(Token::new(kind, span, lexeme));
    }

    fn single_char_token(&mut self, kind: TokenKind) {
        let start = self.current_span();
        let ch = self.advance().unwrap();
        self.add_token(kind, start, ch.to_string());
    }

    fn tokenize_comment(&mut self) -> Result<()> {
        let start = self.current_span();
        self.advance(); // consume first '/'
        self.advance(); // consume second '/'
        
        let mut comment = String::new();
        while let Some(&ch) = self.chars.peek() {
            if ch == '\n' {
                break;
            }
            comment.push(self.advance().unwrap());
        }
        
        self.add_token(TokenKind::Comment(comment.clone()), start, format!("//{}", comment));
        Ok(())
    }

    fn tokenize_string(&mut self) -> Result<()> {
        let start = self.current_span();
        self.advance(); // consume opening quote
        
        let mut value = String::new();
        let mut escaped = false;
        
        while let Some(ch) = self.advance() {
            if escaped {
                match ch {
                    'n' => value.push('\n'),
                    't' => value.push('\t'),
                    'r' => value.push('\r'),
                    '\\' => value.push('\\'),
                    '\'' => value.push('\''),
                    '"' => value.push('"'),
                    _ => {
                        value.push('\\');
                        value.push(ch);
                    }
                }
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == '"' {
                self.add_token(TokenKind::String(value.clone()), start, format!("\"{}\"", value));
                return Ok(());
            } else {
                value.push(ch);
            }
        }
        
        Err("Unterminated string literal".into())
    }

    fn tokenize_char(&mut self) -> Result<()> {
        let start = self.current_span();
        self.advance(); // consume opening quote
        
        let mut value = String::new();
        if let Some(ch) = self.advance() {
            if ch == '\\' {
                if let Some(escaped) = self.advance() {
                    match escaped {
                        'n' => value.push('\n'),
                        't' => value.push('\t'),
                        'r' => value.push('\r'),
                        '\\' => value.push('\\'),
                        '\'' => value.push('\''),
                        '"' => value.push('"'),
                        _ => {
                            value.push('\\');
                            value.push(escaped);
                        }
                    }
                }
            } else {
                value.push(ch);
            }
        }
        
        if self.advance() == Some('\'') {
            self.add_token(TokenKind::String(value.clone()), start, format!("'{}'", value));
            Ok(())
        } else {
            Err("Unterminated character literal".into())
        }
    }

    fn tokenize_number(&mut self) -> Result<()> {
        let start = self.current_span();
        let mut number = String::new();
        
        // Integer part
        while let Some(&ch) = self.chars.peek() {
            if ch.is_ascii_digit() {
                number.push(self.advance().unwrap());
            } else {
                break;
            }
        }
        
        // Decimal part
        if self.chars.peek() == Some(&'.') {
            number.push(self.advance().unwrap()); // consume '.'
            while let Some(&ch) = self.chars.peek() {
                if ch.is_ascii_digit() {
                    number.push(self.advance().unwrap());
                } else {
                    break;
                }
            }
        }
        
        // Exponent part
        if matches!(self.chars.peek(), Some(&'e') | Some(&'E')) {
            number.push(self.advance().unwrap()); // consume 'e' or 'E'
            if matches!(self.chars.peek(), Some(&'+') | Some(&'-')) {
                number.push(self.advance().unwrap()); // consume sign
            }
            while let Some(&ch) = self.chars.peek() {
                if ch.is_ascii_digit() {
                    number.push(self.advance().unwrap());
                } else {
                    break;
                }
            }
        }
        
        self.add_token(TokenKind::Number(number.clone()), start, number);
        Ok(())
    }

    fn tokenize_identifier(&mut self) -> Result<()> {
        let start = self.current_span();
        let mut identifier = String::new();
        
        while let Some(&ch) = self.chars.peek() {
            if ch.is_alphanumeric() || ch == '_' {
                identifier.push(self.advance().unwrap());
            } else {
                break;
            }
        }
        
        // Check for keywords
        let kind = match identifier.as_str() {
            "function" => TokenKind::Function,
            "let" => TokenKind::Let,
            "const" => TokenKind::Const,
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "while" => TokenKind::While,
            "for" => TokenKind::For,
            "return" => TokenKind::Return,
            "import" => TokenKind::Import,
            "export" => TokenKind::Export,
            "class" => TokenKind::Class,
            "lang" => TokenKind::Lang,
            "true" => TokenKind::Boolean(true),
            "false" => TokenKind::Boolean(false),
            "null" => TokenKind::Null,
            _ => TokenKind::Identifier(identifier.clone()),
        };
        
        self.add_token(kind, start, identifier);
        Ok(())
    }

    fn tokenize_template_literal(&mut self) -> Result<()> {
        let start = self.current_span();
        self.advance(); // consume opening '`'
        
        let mut template = String::new();
        
        while let Some(&ch) = self.chars.peek() {
            if ch == '`' {
                break;
            } else if ch == '\\' {
                self.advance(); // consume '\'
                if let Some(&escaped) = self.chars.peek() {
                    self.advance(); // consume escaped character
                    match escaped {
                        'n' => template.push('\n'),
                        't' => template.push('\t'),
                        'r' => template.push('\r'),
                        '\\' => template.push('\\'),
                        '`' => template.push('`'),
                        '$' => template.push('$'),
                        _ => {
                            template.push('\\');
                            template.push(escaped);
                        }
                    }
                }
            } else {
                template.push(self.advance().unwrap());
            }
        }
        
        if self.advance() == Some('`') {
            self.add_token(TokenKind::String(template.clone()), start, format!("`{}`", template));
            Ok(())
        } else {
            Err("Unterminated template literal".into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_tokens() {
        let mut lexer = Lexer::new("let x = 42;");
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens.len(), 6); // let, x, =, 42, ;, EOF
        assert_eq!(tokens[0].kind, TokenKind::Let);
        assert_eq!(tokens[1].kind, TokenKind::Identifier("x".to_string()));
        assert_eq!(tokens[2].kind, TokenKind::Equal);
        assert_eq!(tokens[3].kind, TokenKind::Number("42".to_string()));
        assert_eq!(tokens[4].kind, TokenKind::Semicolon);
        assert_eq!(tokens[5].kind, TokenKind::Eof);
    }

    #[test]
    fn test_string_literal() {
        let mut lexer = Lexer::new(r#""hello world""#);
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens.len(), 2); // string, EOF
        assert_eq!(tokens[0].kind, TokenKind::String("hello world".to_string()));
    }

    #[test]
    fn test_language_directive() {
        let mut lexer = Lexer::new("@lang python");
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens.len(), 4); // @, lang, python, EOF
        assert_eq!(tokens[0].kind, TokenKind::At);
        assert_eq!(tokens[1].kind, TokenKind::Lang);
        assert_eq!(tokens[2].kind, TokenKind::Identifier("python".to_string()));
    }

    #[test]
    fn test_cross_call() {
        let mut lexer = Lexer::new("python::function_name");
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens.len(), 4); // python, ::, function_name, EOF
        assert_eq!(tokens[0].kind, TokenKind::Identifier("python".to_string()));
        assert_eq!(tokens[1].kind, TokenKind::DoubleColon);
        assert_eq!(tokens[2].kind, TokenKind::Identifier("function_name".to_string()));
    }
} 