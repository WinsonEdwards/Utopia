//! High-performance parser for Utopia using nom
//!
//! This parser transforms tokens into an Abstract Syntax Tree (AST)
//! with support for mixed-language constructs and cross-language calls.

use crate::{
    ast::*, 
    lexer::{Token, TokenKind}, 
    types::Type, 
    Result, 
    Span
};
use nom::{
    branch::alt,
    combinator::{map, opt, value},
    multi::{many0, separated_list0, separated_list1},
    sequence::{delimited, pair, preceded, terminated, tuple},
    IResult,
};
use std::collections::HashMap;

/// Parser state
pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

    /// Parse tokens into a Program AST
    pub fn parse(&mut self) -> Result<Program> {
        // Skip any leading comments
        self.skip_comments();
        
        let span = if self.tokens.is_empty() {
            Span::new(0, 0, 1, 1)
        } else {
            Span::new(
                self.tokens[0].span.start,
                self.tokens.last().unwrap().span.end,
                self.tokens[0].span.line,
                self.tokens[0].span.column,
            )
        };
        
        let mut program = Program::new(span);
        
        while !self.is_at_end() {
            // Skip newlines at the top level
            if self.check(&TokenKind::Newline) {
                self.advance();
                continue;
            }
            
            // Parse language blocks
            if self.check(&TokenKind::At) && self.check_next(&TokenKind::Lang) {
                let lang_block = self.parse_language_block()?;
                program.add_language_block(lang_block);
            } else {
                // Parse global statements
                let statement = self.parse_statement()?;
                program.add_global_statement(statement);
            }
        }
        
        Ok(program)
    }

    fn parse_language_block(&mut self) -> Result<LanguageBlock> {
        let start_span = self.current_token().span;
        
        // Consume '@'
        self.consume(&TokenKind::At, "Expected '@'")?;
        
        // Consume 'lang'
        self.consume(&TokenKind::Lang, "Expected 'lang'")?;
        
        // Get language name
        let language = if let TokenKind::Identifier(name) = &self.current_token().kind {
            let lang = name.clone();
            self.advance();
            lang
        } else {
            return Err("Expected language identifier".into());
        };
        
        // Optional newline
        if self.check(&TokenKind::Newline) {
            self.advance();
        }
        
        // Consume '{'
        self.consume(&TokenKind::LeftBrace, "Expected '{'")?;
        
        let mut lang_block = LanguageBlock::new(language, start_span);
        
        // Parse functions and statements
        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            // Skip newlines
            if self.check(&TokenKind::Newline) {
                self.advance();
                continue;
            }
            
            // Check for function declaration
            if self.check(&TokenKind::Function) {
                let function = self.parse_function(&lang_block.language)?;
                lang_block.add_function(function);
            } else {
                // Parse regular statement
                let statement = self.parse_statement()?;
                lang_block.add_statement(statement);
            }
        }
        
        // Consume '}'
        self.consume(&TokenKind::RightBrace, "Expected '}'")?;
        
        Ok(lang_block)
    }

    fn parse_function(&mut self, language: &str) -> Result<Function> {
        let start_span = self.current_token().span;
        
        // Consume 'function'
        self.consume(&TokenKind::Function, "Expected 'function'")?;
        
        // Get function name
        let name = if let TokenKind::Identifier(name) = &self.current_token().kind {
            let func_name = name.clone();
            self.advance();
            func_name
        } else {
            return Err("Expected function name".into());
        };
        
        let mut function = Function::new(name, language.to_string(), start_span);
        
        // Parse parameters
        self.consume(&TokenKind::LeftParen, "Expected '('")?;
        
        if !self.check(&TokenKind::RightParen) {
            loop {
                let param = self.parse_parameter()?;
                function.parameters.push(param);
                
                if self.check(&TokenKind::Comma) {
                    self.advance();
                } else {
                    break;
                }
            }
        }
        
        self.consume(&TokenKind::RightParen, "Expected ')'")?;
        
        // Optional return type
        if self.check(&TokenKind::Arrow) {
            self.advance();
            function.return_type = Some(self.parse_type()?);
        }
        
        // Optional newline
        if self.check(&TokenKind::Newline) {
            self.advance();
        }
        
        // Parse function body
        self.consume(&TokenKind::LeftBrace, "Expected '{'")?;
        
        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            // Skip newlines
            if self.check(&TokenKind::Newline) {
                self.advance();
                continue;
            }
            
            let statement = self.parse_statement()?;
            function.body.push(statement);
        }
        
        self.consume(&TokenKind::RightBrace, "Expected '}'")?;
        
        Ok(function)
    }

    fn parse_parameter(&mut self) -> Result<Parameter> {
        // Get parameter name
        let name = if let TokenKind::Identifier(name) = &self.current_token().kind {
            let param_name = name.clone();
            self.advance();
            param_name
        } else {
            return Err("Expected parameter name".into());
        };
        
        // Optional type annotation
        let param_type = if self.check(&TokenKind::Colon) {
            self.advance();
            Some(self.parse_type()?)
        } else {
            None
        };
        
        // Optional default value
        let default_value = if self.check(&TokenKind::Equal) {
            self.advance();
            Some(self.parse_expression()?)
        } else {
            None
        };
        
        Ok(Parameter {
            name,
            param_type,
            default_value,
        })
    }

    fn parse_type(&mut self) -> Result<Type> {
        match &self.current_token().kind {
            TokenKind::Identifier(name) => {
                let type_name = name.clone();
                self.advance();
                
                match type_name.as_str() {
                    "number" => Ok(Type::Number),
                    "string" => Ok(Type::String),
                    "boolean" => Ok(Type::Boolean),
                    "void" => Ok(Type::Void),
                    "null" => Ok(Type::Null),
                    _ => Ok(Type::LanguageSpecific {
                        language: "utopia".to_string(),
                        type_name,
                        generic_args: Vec::new(),
                    }),
                }
            }
            _ => Err("Expected type identifier".into()),
        }
    }

    fn parse_statement(&mut self) -> Result<Statement> {
        let start_span = self.current_token().span;
        
        match &self.current_token().kind {
            TokenKind::Let | TokenKind::Const => self.parse_variable_declaration(),
            TokenKind::If => self.parse_if_statement(),
            TokenKind::While => self.parse_while_statement(),
            TokenKind::For => self.parse_for_statement(),
            TokenKind::Return => self.parse_return_statement(),
            TokenKind::Import => self.parse_import_statement(),
            TokenKind::Export => self.parse_export_statement(),
            TokenKind::LeftBrace => self.parse_block_statement(),
            _ => {
                // Expression statement
                let expression = self.parse_expression()?;
                
                // Check for assignment
                if self.check(&TokenKind::Equal) {
                    self.advance();
                    let value = self.parse_expression()?;
                    
                    // Consume optional semicolon
                    if self.check(&TokenKind::Semicolon) {
                        self.advance();
                    }
                    
                    Ok(Statement::Assignment {
                        target: expression,
                        value,
                        span: start_span,
                    })
                } else {
                    // Consume optional semicolon
                    if self.check(&TokenKind::Semicolon) {
                        self.advance();
                    }
                    
                    Ok(Statement::Expression {
                        expression,
                        span: start_span,
                    })
                }
            }
        }
    }

    fn parse_variable_declaration(&mut self) -> Result<Statement> {
        let start_span = self.current_token().span;
        
        let is_const = match &self.current_token().kind {
            TokenKind::Let => {
                self.advance();
                false
            }
            TokenKind::Const => {
                self.advance();
                true
            }
            _ => return Err("Expected 'let' or 'const'".into()),
        };
        
        // Get variable name
        let name = if let TokenKind::Identifier(name) = &self.current_token().kind {
            let var_name = name.clone();
            self.advance();
            var_name
        } else {
            return Err("Expected variable name".into());
        };
        
        // Optional type annotation
        let var_type = if self.check(&TokenKind::Colon) {
            self.advance();
            Some(self.parse_type()?)
        } else {
            None
        };
        
        // Optional initialization
        let value = if self.check(&TokenKind::Equal) {
            self.advance();
            Some(self.parse_expression()?)
        } else {
            None
        };
        
        // Consume optional semicolon
        if self.check(&TokenKind::Semicolon) {
            self.advance();
        }
        
        Ok(Statement::VariableDeclaration {
            name,
            value,
            var_type,
            is_const,
            span: start_span,
        })
    }

    fn parse_if_statement(&mut self) -> Result<Statement> {
        let start_span = self.current_token().span;
        
        self.consume(&TokenKind::If, "Expected 'if'")?;
        self.consume(&TokenKind::LeftParen, "Expected '('")?;
        
        let condition = self.parse_expression()?;
        
        self.consume(&TokenKind::RightParen, "Expected ')'")?;
        
        let then_branch = self.parse_block()?;
        
        let else_branch = if self.check(&TokenKind::Else) {
            self.advance();
            Some(self.parse_block()?)
        } else {
            None
        };
        
        Ok(Statement::If {
            condition,
            then_branch,
            else_branch,
            span: start_span,
        })
    }

    fn parse_while_statement(&mut self) -> Result<Statement> {
        let start_span = self.current_token().span;
        
        self.consume(&TokenKind::While, "Expected 'while'")?;
        self.consume(&TokenKind::LeftParen, "Expected '('")?;
        
        let condition = self.parse_expression()?;
        
        self.consume(&TokenKind::RightParen, "Expected ')'")?;
        
        let body = self.parse_block()?;
        
        Ok(Statement::While {
            condition,
            body,
            span: start_span,
        })
    }

    fn parse_for_statement(&mut self) -> Result<Statement> {
        let start_span = self.current_token().span;
        
        self.consume(&TokenKind::For, "Expected 'for'")?;
        self.consume(&TokenKind::LeftParen, "Expected '('")?;
        
        // Initialize
        let init = if self.check(&TokenKind::Semicolon) {
            None
        } else {
            Some(Box::new(self.parse_statement()?))
        };
        
        self.consume(&TokenKind::Semicolon, "Expected ';'")?;
        
        // Condition
        let condition = if self.check(&TokenKind::Semicolon) {
            None
        } else {
            Some(self.parse_expression()?)
        };
        
        self.consume(&TokenKind::Semicolon, "Expected ';'")?;
        
        // Update
        let update = if self.check(&TokenKind::RightParen) {
            None
        } else {
            Some(self.parse_expression()?)
        };
        
        self.consume(&TokenKind::RightParen, "Expected ')'")?;
        
        let body = self.parse_block()?;
        
        Ok(Statement::For {
            init,
            condition,
            update,
            body,
            span: start_span,
        })
    }

    fn parse_return_statement(&mut self) -> Result<Statement> {
        let start_span = self.current_token().span;
        
        self.consume(&TokenKind::Return, "Expected 'return'")?;
        
        let value = if self.check(&TokenKind::Semicolon) || self.check(&TokenKind::Newline) {
            None
        } else {
            Some(self.parse_expression()?)
        };
        
        // Consume optional semicolon
        if self.check(&TokenKind::Semicolon) {
            self.advance();
        }
        
        Ok(Statement::Return {
            value,
            span: start_span,
        })
    }

    fn parse_import_statement(&mut self) -> Result<Statement> {
        let start_span = self.current_token().span;
        
        self.consume(&TokenKind::Import, "Expected 'import'")?;
        
        // For now, simple import syntax: import module
        let module = if let TokenKind::Identifier(name) = &self.current_token().kind {
            let module_name = name.clone();
            self.advance();
            module_name
        } else {
            return Err("Expected module name".into());
        };
        
        // Consume optional semicolon
        if self.check(&TokenKind::Semicolon) {
            self.advance();
        }
        
        Ok(Statement::Import {
            module,
            items: Vec::new(), // TODO: Parse specific imports
            span: start_span,
        })
    }

    fn parse_export_statement(&mut self) -> Result<Statement> {
        let start_span = self.current_token().span;
        
        self.consume(&TokenKind::Export, "Expected 'export'")?;
        
        let item = if let TokenKind::Identifier(name) = &self.current_token().kind {
            let item_name = name.clone();
            self.advance();
            item_name
        } else {
            return Err("Expected export item".into());
        };
        
        // Consume optional semicolon
        if self.check(&TokenKind::Semicolon) {
            self.advance();
        }
        
        Ok(Statement::Export {
            item,
            span: start_span,
        })
    }

    fn parse_block_statement(&mut self) -> Result<Statement> {
        let start_span = self.current_token().span;
        let statements = self.parse_block()?;
        
        Ok(Statement::Block {
            statements,
            span: start_span,
        })
    }

    fn parse_block(&mut self) -> Result<Vec<Statement>> {
        self.consume(&TokenKind::LeftBrace, "Expected '{'")?;
        
        let mut statements = Vec::new();
        
        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            // Skip newlines
            if self.check(&TokenKind::Newline) {
                self.advance();
                continue;
            }
            
            statements.push(self.parse_statement()?);
        }
        
        self.consume(&TokenKind::RightBrace, "Expected '}'")?;
        
        Ok(statements)
    }

    fn parse_expression(&mut self) -> Result<Expression> {
        self.parse_logical_or()
    }

    fn parse_logical_or(&mut self) -> Result<Expression> {
        let mut expr = self.parse_logical_and()?;
        
        while self.check(&TokenKind::Or) {
            let start = expr.span().start;
            self.advance();
            let right = self.parse_logical_and()?;
            let span = Span::new(start, right.span().end, expr.span().line, expr.span().column);
            
            expr = Expression::Binary {
                left: Box::new(expr),
                operator: BinaryOperator::Or,
                right: Box::new(right),
                span,
            };
        }
        
        Ok(expr)
    }

    fn parse_logical_and(&mut self) -> Result<Expression> {
        let mut expr = self.parse_equality()?;
        
        while self.check(&TokenKind::And) {
            let start = expr.span().start;
            self.advance();
            let right = self.parse_equality()?;
            let span = Span::new(start, right.span().end, expr.span().line, expr.span().column);
            
            expr = Expression::Binary {
                left: Box::new(expr),
                operator: BinaryOperator::And,
                right: Box::new(right),
                span,
            };
        }
        
        Ok(expr)
    }

    fn parse_equality(&mut self) -> Result<Expression> {
        let mut expr = self.parse_comparison()?;
        
        while matches!(self.current_token().kind, TokenKind::EqualEqual | TokenKind::NotEqual) {
            let start = expr.span().start;
            let operator = match self.current_token().kind {
                TokenKind::EqualEqual => BinaryOperator::Equal,
                TokenKind::NotEqual => BinaryOperator::NotEqual,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_comparison()?;
            let span = Span::new(start, right.span().end, expr.span().line, expr.span().column);
            
            expr = Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
                span,
            };
        }
        
        Ok(expr)
    }

    fn parse_comparison(&mut self) -> Result<Expression> {
        let mut expr = self.parse_term()?;
        
        while matches!(self.current_token().kind, 
                      TokenKind::Greater | TokenKind::GreaterEqual | 
                      TokenKind::Less | TokenKind::LessEqual) {
            let start = expr.span().start;
            let operator = match self.current_token().kind {
                TokenKind::Greater => BinaryOperator::Greater,
                TokenKind::GreaterEqual => BinaryOperator::GreaterEqual,
                TokenKind::Less => BinaryOperator::Less,
                TokenKind::LessEqual => BinaryOperator::LessEqual,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_term()?;
            let span = Span::new(start, right.span().end, expr.span().line, expr.span().column);
            
            expr = Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
                span,
            };
        }
        
        Ok(expr)
    }

    fn parse_term(&mut self) -> Result<Expression> {
        let mut expr = self.parse_factor()?;
        
        while matches!(self.current_token().kind, TokenKind::Plus | TokenKind::Minus) {
            let start = expr.span().start;
            let operator = match self.current_token().kind {
                TokenKind::Plus => BinaryOperator::Add,
                TokenKind::Minus => BinaryOperator::Subtract,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_factor()?;
            let span = Span::new(start, right.span().end, expr.span().line, expr.span().column);
            
            expr = Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
                span,
            };
        }
        
        Ok(expr)
    }

    fn parse_factor(&mut self) -> Result<Expression> {
        let mut expr = self.parse_unary()?;
        
        while matches!(self.current_token().kind, TokenKind::Star | TokenKind::Slash | TokenKind::Percent) {
            let start = expr.span().start;
            let operator = match self.current_token().kind {
                TokenKind::Star => BinaryOperator::Multiply,
                TokenKind::Slash => BinaryOperator::Divide,
                TokenKind::Percent => BinaryOperator::Modulo,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_unary()?;
            let span = Span::new(start, right.span().end, expr.span().line, expr.span().column);
            
            expr = Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
                span,
            };
        }
        
        Ok(expr)
    }

    fn parse_unary(&mut self) -> Result<Expression> {
        if matches!(self.current_token().kind, TokenKind::Not | TokenKind::Minus | TokenKind::Plus) {
            let span = self.current_token().span;
            let operator = match self.current_token().kind {
                TokenKind::Not => UnaryOperator::Not,
                TokenKind::Minus => UnaryOperator::Minus,
                TokenKind::Plus => UnaryOperator::Plus,
                _ => unreachable!(),
            };
            self.advance();
            let operand = self.parse_call()?;
            
            Ok(Expression::Unary {
                operator,
                operand: Box::new(operand),
                span,
            })
        } else {
            self.parse_call()
        }
    }

    fn parse_call(&mut self) -> Result<Expression> {
        let mut expr = self.parse_primary()?;
        
        loop {
            if self.check(&TokenKind::LeftParen) {
                // Function call
                let start = expr.span().start;
                self.advance();
                
                let mut arguments = Vec::new();
                if !self.check(&TokenKind::RightParen) {
                    loop {
                        arguments.push(self.parse_expression()?);
                        if self.check(&TokenKind::Comma) {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                }
                
                self.consume(&TokenKind::RightParen, "Expected ')'")?;
                let span = Span::new(start, self.previous().span.end, expr.span().line, expr.span().column);
                
                expr = Expression::Call {
                    callee: Box::new(expr),
                    arguments,
                    span,
                };
            } else if self.check(&TokenKind::Dot) {
                // Member access
                let start = expr.span().start;
                self.advance();
                
                let property = if let TokenKind::Identifier(name) = &self.current_token().kind {
                    let prop_name = name.clone();
                    self.advance();
                    prop_name
                } else {
                    return Err("Expected property name".into());
                };
                
                let span = Span::new(start, self.previous().span.end, expr.span().line, expr.span().column);
                
                expr = Expression::MemberAccess {
                    object: Box::new(expr),
                    property,
                    span,
                };
            } else if self.check(&TokenKind::LeftBracket) {
                // Array access
                let start = expr.span().start;
                self.advance();
                
                let index = self.parse_expression()?;
                
                self.consume(&TokenKind::RightBracket, "Expected ']'")?;
                let span = Span::new(start, self.previous().span.end, expr.span().line, expr.span().column);
                
                expr = Expression::ArrayAccess {
                    array: Box::new(expr),
                    index: Box::new(index),
                    span,
                };
            } else {
                break;
            }
        }
        
        Ok(expr)
    }

    fn parse_primary(&mut self) -> Result<Expression> {
        let token = self.current_token().clone();
        let span = token.span;
        
        match &token.kind {
            TokenKind::Number(n) => {
                let number_str = n.clone();
                self.advance();
                let value = number_str.parse::<f64>().map_err(|_| "Invalid number")?;
                Ok(Expression::Literal {
                    value: LiteralValue::Number(value),
                    span,
                })
            }
            TokenKind::String(s) => {
                let string_val = s.clone();
                self.advance();
                Ok(Expression::Literal {
                    value: LiteralValue::String(string_val),
                    span,
                })
            }
            TokenKind::Boolean(b) => {
                let bool_val = *b;
                self.advance();
                Ok(Expression::Literal {
                    value: LiteralValue::Boolean(bool_val),
                    span,
                })
            }
            TokenKind::Null => {
                self.advance();
                Ok(Expression::Literal {
                    value: LiteralValue::Null,
                    span,
                })
            }
            TokenKind::Identifier(name) => {
                let identifier = name.clone();
                self.advance();
                
                // Check for cross-language call (language::function)
                if self.check(&TokenKind::DoubleColon) {
                    self.advance(); // consume '::'
                    
                    let current = self.current_token();
                    if let TokenKind::Identifier(function_name) = &current.kind {
                        let func_name = function_name.clone();
                        self.advance();
                        
                        // Parse arguments
                        self.consume(&TokenKind::LeftParen, "Expected '('")?;
                        
                        let mut arguments = Vec::new();
                        if !self.check(&TokenKind::RightParen) {
                            loop {
                                arguments.push(self.parse_expression()?);
                                if self.check(&TokenKind::Comma) {
                                    self.advance();
                                } else {
                                    break;
                                }
                            }
                        }
                        
                        self.consume(&TokenKind::RightParen, "Expected ')'")?;
                        
                        Ok(Expression::CrossCall {
                            language: identifier,
                            function: func_name,
                            arguments,
                            span,
                        })
                    } else {
                        Err("Expected function name after '::'".into())
                    }
                } else {
                    Ok(Expression::Identifier {
                        name: identifier,
                        span,
                    })
                }
            }
            TokenKind::LeftParen => {
                self.advance();
                let expr = self.parse_expression()?;
                self.consume(&TokenKind::RightParen, "Expected ')'")?;
                Ok(expr)
            }
            TokenKind::LeftBracket => {
                self.advance();
                let mut elements = Vec::new();
                
                if !self.check(&TokenKind::RightBracket) {
                    loop {
                        elements.push(self.parse_expression()?);
                        if self.check(&TokenKind::Comma) {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                }
                
                self.consume(&TokenKind::RightBracket, "Expected ']'")?;
                
                Ok(Expression::Array {
                    elements,
                    span,
                })
            }
            TokenKind::LeftBrace => {
                self.advance();
                let mut properties = HashMap::new();
                
                if !self.check(&TokenKind::RightBrace) {
                    loop {
                        // Parse key
                        let current = self.current_token();
                        let key = if let TokenKind::Identifier(name) = &current.kind {
                            let key_name = name.clone();
                            self.advance();
                            key_name
                        } else {
                            return Err("Expected property name".into());
                        };
                        
                        self.consume(&TokenKind::Colon, "Expected ':'")?;
                        
                        let value = self.parse_expression()?;
                        properties.insert(key, value);
                        
                        if self.check(&TokenKind::Comma) {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                }
                
                self.consume(&TokenKind::RightBrace, "Expected '}'")?;
                
                Ok(Expression::Object {
                    properties,
                    span,
                })
            }
            _ => Err(format!("Unexpected token: {:?}", token.kind).into()),
        }
    }

    // Helper methods
    fn current_token(&self) -> Token {
        if self.position >= self.tokens.len() {
            // Return EOF token
            Token::new(TokenKind::Eof, Span::new(0, 0, 1, 1), String::new())
        } else {
            self.tokens[self.position].clone()
        }
    }

    fn previous(&self) -> Token {
        if self.position == 0 {
            self.tokens[0].clone()
        } else {
            self.tokens[self.position - 1].clone()
        }
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.position += 1;
        }
        // Skip comments
        self.skip_comments();
        self.previous()
    }
    
    fn skip_comments(&mut self) {
        while !self.is_at_end() {
            if let TokenKind::Comment(_) = self.current_token().kind {
                self.position += 1;
            } else {
                break;
            }
        }
    }

    fn is_at_end(&self) -> bool {
        self.position >= self.tokens.len() || 
        matches!(self.current_token().kind, TokenKind::Eof)
    }

    fn check(&self, kind: &TokenKind) -> bool {
        if self.is_at_end() {
            false
        } else {
            let current = self.current_token();
            std::mem::discriminant(&current.kind) == std::mem::discriminant(kind)
        }
    }

    fn check_next(&self, kind: &TokenKind) -> bool {
        if self.position + 1 >= self.tokens.len() {
            false
        } else {
            std::mem::discriminant(&self.tokens[self.position + 1].kind) == std::mem::discriminant(kind)
        }
    }

    fn consume(&mut self, kind: &TokenKind, message: &str) -> Result<Token> {
        if self.check(kind) {
            Ok(self.advance())
        } else {
            let current = self.current_token();
            Err(format!("{} at line {}, column {}", 
                       message, 
                       current.span.line, 
                       current.span.column).into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    #[test]
    fn test_parse_simple_expression() {
        let mut lexer = Lexer::new("42 + 13");
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();
        
        assert_eq!(program.global_statements.len(), 1);
    }

    #[test]
    fn test_parse_language_block() {
        let mut lexer = Lexer::new("@lang python { function test() { return 42; } }");
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();
        
        assert_eq!(program.language_blocks.len(), 1);
        assert_eq!(program.language_blocks[0].language, "python");
        assert_eq!(program.language_blocks[0].functions.len(), 1);
        assert_eq!(program.language_blocks[0].functions[0].name, "test");
    }

    #[test]
    fn test_parse_cross_call() {
        let mut lexer = Lexer::new("python::math_function(42, 13)");
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();
        
        assert_eq!(program.global_statements.len(), 1);
        if let Statement::Expression { expression, .. } = &program.global_statements[0] {
            if let Expression::CrossCall { language, function, arguments, .. } = expression {
                assert_eq!(language, "python");
                assert_eq!(function, "math_function");
                assert_eq!(arguments.len(), 2);
            } else {
                panic!("Expected cross call expression");
            }
        } else {
            panic!("Expected expression statement");
        }
    }
} 