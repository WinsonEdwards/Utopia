//! Abstract Syntax Tree (AST) for Utopia
//!
//! This module defines the AST nodes that represent parsed Utopia source code.
//! The AST supports mixed-language constructs and cross-language function calls.

use crate::{Span, types::Type};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Metadata about a Utopia program
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    pub languages: Vec<String>,
    pub functions: Vec<FunctionInfo>,
    pub cross_calls: Vec<CrossCall>,
    pub imports: Vec<String>,
    pub exports: Vec<String>,
}

impl Metadata {
    pub fn new() -> Self {
        Self {
            languages: Vec::new(),
            functions: Vec::new(),
            cross_calls: Vec::new(),
            imports: Vec::new(),
            exports: Vec::new(),
        }
    }
}

impl Default for Metadata {
    fn default() -> Self {
        Self::new()
    }
}

/// Information about a function in the program
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionInfo {
    pub name: String,
    pub language: String,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<Type>,
    pub is_exported: bool,
    pub span: Span,
}

/// Function parameter information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Parameter {
    pub name: String,
    pub param_type: Option<Type>,
    pub default_value: Option<Expression>,
}

/// Cross-language function call information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossCall {
    pub target_language: String,
    pub function_name: String,
    pub call_site: Span,
}

/// Main AST node trait
pub trait AstNode {
    fn span(&self) -> Span;
    fn accept<V: Visitor>(&self, visitor: &mut V) -> V::Result;
}

/// Visitor pattern for traversing the AST
pub trait Visitor {
    type Result;

    fn visit_program(&mut self, program: &Program) -> Self::Result;
    fn visit_language_block(&mut self, block: &LanguageBlock) -> Self::Result;
    fn visit_function(&mut self, function: &Function) -> Self::Result;
    fn visit_statement(&mut self, statement: &Statement) -> Self::Result;
    fn visit_expression(&mut self, expression: &Expression) -> Self::Result;
}

/// Root node of the AST representing a complete Utopia program
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Program {
    pub language_blocks: Vec<LanguageBlock>,
    pub global_statements: Vec<Statement>,
    pub metadata: Metadata,
    pub span: Span,
}

impl Program {
    pub fn new(span: Span) -> Self {
        Self {
            language_blocks: Vec::new(),
            global_statements: Vec::new(),
            metadata: Metadata::new(),
            span,
        }
    }

    pub fn metadata(&self) -> Metadata {
        self.metadata.clone()
    }

    pub fn add_language_block(&mut self, block: LanguageBlock) {
        if !self.metadata.languages.contains(&block.language) {
            self.metadata.languages.push(block.language.clone());
        }
        self.language_blocks.push(block);
    }

    pub fn add_global_statement(&mut self, statement: Statement) {
        self.global_statements.push(statement);
    }
}

impl AstNode for Program {
    fn span(&self) -> Span {
        self.span
    }

    fn accept<V: Visitor>(&self, visitor: &mut V) -> V::Result {
        visitor.visit_program(self)
    }
}

/// A block of code in a specific language
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageBlock {
    pub language: String,
    pub functions: Vec<Function>,
    pub statements: Vec<Statement>,
    pub span: Span,
}

impl LanguageBlock {
    pub fn new(language: String, span: Span) -> Self {
        Self {
            language,
            functions: Vec::new(),
            statements: Vec::new(),
            span,
        }
    }

    pub fn add_function(&mut self, function: Function) {
        self.functions.push(function);
    }

    pub fn add_statement(&mut self, statement: Statement) {
        self.statements.push(statement);
    }
}

impl AstNode for LanguageBlock {
    fn span(&self) -> Span {
        self.span
    }

    fn accept<V: Visitor>(&self, visitor: &mut V) -> V::Result {
        visitor.visit_language_block(self)
    }
}

/// Function definition
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Function {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<Type>,
    pub body: Vec<Statement>,
    pub is_exported: bool,
    pub language: String,
    pub span: Span,
}

impl Function {
    pub fn new(name: String, language: String, span: Span) -> Self {
        Self {
            name,
            parameters: Vec::new(),
            return_type: None,
            body: Vec::new(),
            is_exported: false,
            language,
            span,
        }
    }
}

impl AstNode for Function {
    fn span(&self) -> Span {
        self.span
    }

    fn accept<V: Visitor>(&self, visitor: &mut V) -> V::Result {
        visitor.visit_function(self)
    }
}

/// Statements in the AST
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Statement {
    Expression {
        expression: Expression,
        span: Span,
    },
    VariableDeclaration {
        name: String,
        value: Option<Expression>,
        var_type: Option<Type>,
        is_const: bool,
        span: Span,
    },
    Assignment {
        target: Expression,
        value: Expression,
        span: Span,
    },
    If {
        condition: Expression,
        then_branch: Vec<Statement>,
        else_branch: Option<Vec<Statement>>,
        span: Span,
    },
    While {
        condition: Expression,
        body: Vec<Statement>,
        span: Span,
    },
    For {
        init: Option<Box<Statement>>,
        condition: Option<Expression>,
        update: Option<Expression>,
        body: Vec<Statement>,
        span: Span,
    },
    Return {
        value: Option<Expression>,
        span: Span,
    },
    Import {
        module: String,
        items: Vec<String>,
        span: Span,
    },
    Export {
        item: String,
        span: Span,
    },
    Block {
        statements: Vec<Statement>,
        span: Span,
    },
    FunctionDeclaration {
        name: String,
        parameters: Vec<Parameter>,
        return_type: Option<Type>,
        body: Vec<Statement>,
        span: Span,
    },
}

impl Statement {
    pub fn span(&self) -> Span {
        match self {
            Statement::Expression { span, .. } => *span,
            Statement::VariableDeclaration { span, .. } => *span,
            Statement::Assignment { span, .. } => *span,
            Statement::If { span, .. } => *span,
            Statement::While { span, .. } => *span,
            Statement::For { span, .. } => *span,
            Statement::Return { span, .. } => *span,
            Statement::Import { span, .. } => *span,
            Statement::Export { span, .. } => *span,
            Statement::Block { span, .. } => *span,
            Statement::FunctionDeclaration { span, .. } => *span,
        }
    }
}

impl AstNode for Statement {
    fn span(&self) -> Span {
        self.span()
    }

    fn accept<V: Visitor>(&self, visitor: &mut V) -> V::Result {
        visitor.visit_statement(self)
    }
}

/// Expressions in the AST
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Expression {
    Literal {
        value: LiteralValue,
        span: Span,
    },
    Identifier {
        name: String,
        span: Span,
    },
    Binary {
        left: Box<Expression>,
        operator: BinaryOperator,
        right: Box<Expression>,
        span: Span,
    },
    Unary {
        operator: UnaryOperator,
        operand: Box<Expression>,
        span: Span,
    },
    Postfix {
        operand: Box<Expression>,
        operator: PostfixOperator,
        span: Span,
    },
    Assignment {
        target: Box<Expression>,
        value: Box<Expression>,
        span: Span,
    },
    Call {
        callee: Box<Expression>,
        arguments: Vec<Expression>,
        span: Span,
    },
    CrossCall {
        language: String,
        function: String,
        arguments: Vec<Expression>,
        span: Span,
    },
    MemberAccess {
        object: Box<Expression>,
        property: String,
        span: Span,
    },
    ArrayAccess {
        array: Box<Expression>,
        index: Box<Expression>,
        span: Span,
    },
    Array {
        elements: Vec<Expression>,
        span: Span,
    },
    Object {
        properties: HashMap<String, Expression>,
        span: Span,
    },
    Lambda {
        parameters: Vec<Parameter>,
        body: Vec<Statement>,
        return_type: Option<Type>,
        span: Span,
    },
}

impl Expression {
    pub fn span(&self) -> Span {
        match self {
            Expression::Literal { span, .. } => *span,
            Expression::Identifier { span, .. } => *span,
            Expression::Binary { span, .. } => *span,
            Expression::Unary { span, .. } => *span,
            Expression::Call { span, .. } => *span,
            Expression::CrossCall { span, .. } => *span,
            Expression::MemberAccess { span, .. } => *span,
            Expression::ArrayAccess { span, .. } => *span,
            Expression::Array { span, .. } => *span,
            Expression::Object { span, .. } => *span,
            Expression::Lambda { span, .. } => *span,
            Expression::Postfix { span, .. } => *span,
            Expression::Assignment { span, .. } => *span,
        }
    }
}

impl AstNode for Expression {
    fn span(&self) -> Span {
        self.span()
    }

    fn accept<V: Visitor>(&self, visitor: &mut V) -> V::Result {
        visitor.visit_expression(self)
    }
}

/// Literal values
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LiteralValue {
    Number(f64),
    String(String),
    Boolean(bool),
    Null,
}

impl LiteralValue {
    pub fn type_hint(&self) -> Type {
        match self {
            LiteralValue::Number(_) => Type::Number,
            LiteralValue::String(_) => Type::String,
            LiteralValue::Boolean(_) => Type::Boolean,
            LiteralValue::Null => Type::Null,
        }
    }
}

/// Binary operators
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BinaryOperator {
    // Arithmetic
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    
    // Comparison
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    
    // Logical
    And,
    Or,
}

impl BinaryOperator {
    pub fn precedence(&self) -> u8 {
        match self {
            BinaryOperator::Or => 1,
            BinaryOperator::And => 2,
            BinaryOperator::Equal | BinaryOperator::NotEqual => 3,
            BinaryOperator::Less | BinaryOperator::LessEqual 
            | BinaryOperator::Greater | BinaryOperator::GreaterEqual => 4,
            BinaryOperator::Add | BinaryOperator::Subtract => 5,
            BinaryOperator::Multiply | BinaryOperator::Divide | BinaryOperator::Modulo => 6,
        }
    }

    pub fn is_left_associative(&self) -> bool {
        true // All binary operators are left-associative in Utopia
    }
}

/// Unary operators
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UnaryOperator {
    Not,
    Minus,
    Plus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PostfixOperator {
    Increment,
    Decrement,
}

/// Pretty printer for the AST
pub struct PrettyPrinter {
    indent: usize,
    output: String,
}

impl PrettyPrinter {
    pub fn new() -> Self {
        Self {
            indent: 0,
            output: String::new(),
        }
    }

    pub fn print(&mut self, program: &Program) -> String {
        program.accept(self);
        std::mem::take(&mut self.output)
    }

    fn indent_str(&self) -> String {
        "  ".repeat(self.indent)
    }

    fn write_line(&mut self, line: &str) {
        self.output.push_str(&self.indent_str());
        self.output.push_str(line);
        self.output.push('\n');
    }

    fn increase_indent(&mut self) {
        self.indent += 1;
    }

    fn decrease_indent(&mut self) {
        if self.indent > 0 {
            self.indent -= 1;
        }
    }
}

impl Default for PrettyPrinter {
    fn default() -> Self {
        Self::new()
    }
}

impl Visitor for PrettyPrinter {
    type Result = ();

    fn visit_program(&mut self, program: &Program) -> Self::Result {
        self.write_line("Program {");
        self.increase_indent();
        
        for block in &program.language_blocks {
            block.accept(self);
        }
        
        for statement in &program.global_statements {
            statement.accept(self);
        }
        
        self.decrease_indent();
        self.write_line("}");
    }

    fn visit_language_block(&mut self, block: &LanguageBlock) -> Self::Result {
        self.write_line(&format!("@lang {} {{", block.language));
        self.increase_indent();
        
        for function in &block.functions {
            function.accept(self);
        }
        
        for statement in &block.statements {
            statement.accept(self);
        }
        
        self.decrease_indent();
        self.write_line("}");
    }

    fn visit_function(&mut self, function: &Function) -> Self::Result {
        let params: Vec<String> = function.parameters.iter()
            .map(|p| format!("{}: {:?}", p.name, p.param_type))
            .collect();
        
        let return_type = function.return_type.as_ref()
            .map(|t| format!(" -> {:?}", t))
            .unwrap_or_default();
        
        self.write_line(&format!("function {}({}){} {{", 
                                function.name, 
                                params.join(", "),
                                return_type));
        self.increase_indent();
        
        for statement in &function.body {
            statement.accept(self);
        }
        
        self.decrease_indent();
        self.write_line("}");
    }

    fn visit_statement(&mut self, statement: &Statement) -> Self::Result {
        match statement {
            Statement::Expression { expression, .. } => {
                expression.accept(self);
                self.output.push_str(";\n");
            }
            Statement::VariableDeclaration { name, value, var_type, is_const, .. } => {
                let keyword = if *is_const { "const" } else { "let" };
                let type_annotation = var_type.as_ref()
                    .map(|t| format!(": {:?}", t))
                    .unwrap_or_default();
                let assignment = value.as_ref()
                    .map(|_| " = ".to_string())
                    .unwrap_or_default();
                
                self.write_line(&format!("{} {}{}{}", keyword, name, type_annotation, assignment));
                if let Some(value) = value {
                    value.accept(self);
                }
            }
            Statement::Return { value, .. } => {
                self.output.push_str(&self.indent_str());
                self.output.push_str("return");
                if let Some(value) = value {
                    self.output.push(' ');
                    value.accept(self);
                }
                self.output.push_str(";\n");
            }
            Statement::FunctionDeclaration { name, parameters, return_type, body, .. } => {
                let params_str = parameters.iter()
                    .map(|p| format!("{}: {:?}", p.name, p.param_type))
                    .collect::<Vec<_>>()
                    .join(", ");
                
                let return_type_str = return_type.as_ref()
                    .map(|t| format!(" -> {:?}", t))
                    .unwrap_or_default();
                
                self.write_line(&format!("function {}({}){} {{", name, params_str, return_type_str));
                self.indent += 1;
                
                for stmt in body {
                    stmt.accept(self);
                }
                
                self.indent -= 1;
                self.write_line("}");
            }
            // Add other statement types as needed
            _ => {
                self.write_line(&format!("{:?}", statement));
            }
        }
    }

    fn visit_expression(&mut self, expression: &Expression) -> Self::Result {
        match expression {
            Expression::Literal { value, .. } => {
                match value {
                    LiteralValue::Number(n) => self.output.push_str(&n.to_string()),
                    LiteralValue::String(s) => self.output.push_str(&format!("\"{}\"", s)),
                    LiteralValue::Boolean(b) => self.output.push_str(&b.to_string()),
                    LiteralValue::Null => self.output.push_str("null"),
                }
            }
            Expression::Identifier { name, .. } => {
                self.output.push_str(name);
            }
            Expression::Binary { left, operator, right, .. } => {
                left.accept(self);
                match operator {
                    BinaryOperator::Add => self.output.push_str(" + "),
                    BinaryOperator::Subtract => self.output.push_str(" - "),
                    BinaryOperator::Multiply => self.output.push_str(" * "),
                    BinaryOperator::Divide => self.output.push_str(" / "),
                    BinaryOperator::Modulo => self.output.push_str(" % "),
                    BinaryOperator::Equal => self.output.push_str(" == "),
                    BinaryOperator::NotEqual => self.output.push_str(" != "),
                    BinaryOperator::Less => self.output.push_str(" < "),
                    BinaryOperator::LessEqual => self.output.push_str(" <= "),
                    BinaryOperator::Greater => self.output.push_str(" > "),
                    BinaryOperator::GreaterEqual => self.output.push_str(" >= "),
                    BinaryOperator::And => self.output.push_str(" && "),
                    BinaryOperator::Or => self.output.push_str(" || "),
                }
                right.accept(self);
            }
            Expression::Unary { operator, operand, .. } => {
                match operator {
                    UnaryOperator::Not => self.output.push_str("!"),
                    UnaryOperator::Minus => self.output.push_str("-"),
                    UnaryOperator::Plus => self.output.push_str("+"),
                }
                operand.accept(self);
            }
            Expression::Postfix { operand, operator, .. } => {
                operand.accept(self);
                match operator {
                    PostfixOperator::Increment => self.output.push_str("++"),
                    PostfixOperator::Decrement => self.output.push_str("--"),
                }
            }
            Expression::Assignment { target, value, .. } => {
                target.accept(self);
                self.output.push_str(" = ");
                value.accept(self);
            }
            Expression::CrossCall { language, function, arguments, .. } => {
                self.output.push_str(&format!("{}::{}", language, function));
                self.output.push('(');
                for (i, arg) in arguments.iter().enumerate() {
                    if i > 0 {
                        self.output.push_str(", ");
                    }
                    arg.accept(self);
                }
                self.output.push(')');
            }
            // Add other expression types as needed
            _ => {
                self.output.push_str(&format!("{:?}", expression));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ast_creation() {
        let span = Span::new(0, 10, 1, 1);
        let mut program = Program::new(span);
        
        let mut lang_block = LanguageBlock::new("python".to_string(), span);
        let func = Function::new("test".to_string(), "python".to_string(), span);
        lang_block.add_function(func);
        
        program.add_language_block(lang_block);
        
        assert_eq!(program.language_blocks.len(), 1);
        assert_eq!(program.metadata.languages.len(), 1);
        assert_eq!(program.metadata.languages[0], "python");
    }

    #[test]
    fn test_pretty_printer() {
        let span = Span::new(0, 10, 1, 1);
        let program = Program::new(span);
        
        let mut printer = PrettyPrinter::new();
        let output = printer.print(&program);
        
        assert!(output.contains("Program {"));
        assert!(output.contains("}"));
    }
} 