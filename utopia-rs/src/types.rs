//! Type system for Utopia
//!
//! This module provides a unified type system that can represent types
//! from multiple programming languages and handle cross-language compatibility.

use crate::{ast::Program, diagnostics::Diagnostic, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Type representation in Utopia
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Type {
    // Primitive types
    Number,
    String,
    Boolean,
    Null,
    Void,
    
    // Composite types
    Array(Box<Type>),
    Object(HashMap<String, Type>),
    Function {
        parameters: Vec<Type>,
        return_type: Box<Type>,
    },
    
    // Language-specific types
    LanguageSpecific {
        language: String,
        type_name: String,
        generic_args: Vec<Type>,
    },
    
    // Generic types
    Generic(String),
    Union(Vec<Type>),
    Optional(Box<Type>),
    
    // Unknown type (for inference)
    Unknown,
}

impl Type {
    /// Check if this type is compatible with another type
    pub fn is_compatible_with(&self, other: &Type) -> bool {
        match (self, other) {
            // Same types are always compatible
            (a, b) if a == b => true,
            
            // Null is compatible with any optional type
            (Type::Null, Type::Optional(_)) => true,
            (Type::Optional(_), Type::Null) => true,
            
            // Check optional compatibility
            (Type::Optional(inner), other) => inner.is_compatible_with(other),
            (other, Type::Optional(inner)) => other.is_compatible_with(inner),
            
            // Union type compatibility
            (Type::Union(types), other) => types.iter().any(|t| t.is_compatible_with(other)),
            (other, Type::Union(types)) => types.iter().any(|t| other.is_compatible_with(t)),
            
            // Array compatibility
            (Type::Array(inner1), Type::Array(inner2)) => inner1.is_compatible_with(inner2),
            
            // Function compatibility
            (Type::Function { parameters: p1, return_type: r1 }, 
             Type::Function { parameters: p2, return_type: r2 }) => {
                p1.len() == p2.len() &&
                p1.iter().zip(p2.iter()).all(|(a, b)| a.is_compatible_with(b)) &&
                r1.is_compatible_with(r2)
            }
            
            // Object compatibility (structural)
            (Type::Object(fields1), Type::Object(fields2)) => {
                fields2.iter().all(|(name, type2)| {
                    fields1.get(name).map_or(false, |type1| type1.is_compatible_with(type2))
                })
            }
            
            // Unknown type is compatible with everything
            (Type::Unknown, _) | (_, Type::Unknown) => true,
            
            // Language-specific type compatibility
            (Type::LanguageSpecific { language: l1, type_name: t1, .. },
             Type::LanguageSpecific { language: l2, type_name: t2, .. }) => {
                l1 == l2 && t1 == t2
            }
            
            // Cross-language compatibility mappings
            _ => self.cross_language_compatible(other),
        }
    }

    /// Check cross-language type compatibility
    fn cross_language_compatible(&self, other: &Type) -> bool {
        // Define cross-language type mappings
        match (self, other) {
            // Python int/float <-> JavaScript number <-> C int/double
            (Type::LanguageSpecific { language: l1, type_name: t1, .. },
             Type::Number) if l1 == "python" && (t1 == "int" || t1 == "float") => true,
            
            (Type::Number,
             Type::LanguageSpecific { language: l2, type_name: t2, .. }) 
                if l2 == "python" && (t2 == "int" || t2 == "float") => true,
            
            (Type::LanguageSpecific { language: l1, type_name: t1, .. },
             Type::LanguageSpecific { language: l2, type_name: t2, .. }) => {
                self.language_type_mapping(l1, t1) == other.language_type_mapping(l2, t2)
            }
            
            _ => false,
        }
    }

    /// Map language-specific types to universal types
    fn language_type_mapping(&self, language: &str, type_name: &str) -> Option<&'static str> {
        match language {
            "python" => match type_name {
                "int" | "float" => Some("number"),
                "str" => Some("string"),
                "bool" => Some("boolean"),
                "list" => Some("array"),
                "dict" => Some("object"),
                _ => None,
            },
            "javascript" | "typescript" => match type_name {
                "number" => Some("number"),
                "string" => Some("string"),
                "boolean" => Some("boolean"),
                "Array" => Some("array"),
                "Object" => Some("object"),
                _ => None,
            },
            "java" => match type_name {
                "int" | "Integer" | "double" | "Double" | "float" | "Float" => Some("number"),
                "String" => Some("string"),
                "boolean" | "Boolean" => Some("boolean"),
                "List" | "ArrayList" => Some("array"),
                "Map" | "HashMap" => Some("object"),
                _ => None,
            },
            "c" | "cpp" => match type_name {
                "int" | "float" | "double" | "long" => Some("number"),
                "char*" | "string" => Some("string"),
                "bool" => Some("boolean"),
                _ => None,
            },
            _ => None,
        }
    }

    /// Get the size of this type in bytes (for optimization)
    pub fn size_hint(&self) -> Option<usize> {
        match self {
            Type::Boolean => Some(1),
            Type::Number => Some(8), // Assume 64-bit numbers
            Type::String => None, // Variable size
            Type::Null => Some(0),
            Type::Void => Some(0),
            Type::Array(_) => None, // Variable size
            Type::Object(_) => None, // Variable size
            Type::Function { .. } => Some(8), // Function pointer
            Type::Optional(inner) => inner.size_hint().map(|s| s + 1), // Add null flag
            _ => None,
        }
    }

    /// Create a language-specific type
    pub fn language_type(language: &str, type_name: &str) -> Self {
        Type::LanguageSpecific {
            language: language.to_string(),
            type_name: type_name.to_string(),
            generic_args: Vec::new(),
        }
    }

    /// Create a function type
    pub fn function(parameters: Vec<Type>, return_type: Type) -> Self {
        Type::Function {
            parameters,
            return_type: Box::new(return_type),
        }
    }

    /// Create an array type
    pub fn array(element_type: Type) -> Self {
        Type::Array(Box::new(element_type))
    }

    /// Create an optional type
    pub fn optional(inner_type: Type) -> Self {
        Type::Optional(Box::new(inner_type))
    }
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Number => write!(f, "number"),
            Type::String => write!(f, "string"),
            Type::Boolean => write!(f, "boolean"),
            Type::Null => write!(f, "null"),
            Type::Void => write!(f, "void"),
            Type::Array(inner) => write!(f, "{}[]", inner),
            Type::Object(fields) => {
                write!(f, "{{ ")?;
                for (i, (name, typ)) in fields.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}: {}", name, typ)?;
                }
                write!(f, " }}")
            }
            Type::Function { parameters, return_type } => {
                write!(f, "(")?;
                for (i, param) in parameters.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}", param)?;
                }
                write!(f, ") -> {}", return_type)
            }
            Type::LanguageSpecific { language, type_name, generic_args } => {
                write!(f, "{}::{}", language, type_name)?;
                if !generic_args.is_empty() {
                    write!(f, "<")?;
                    for (i, arg) in generic_args.iter().enumerate() {
                        if i > 0 { write!(f, ", ")?; }
                        write!(f, "{}", arg)?;
                    }
                    write!(f, ">")?;
                }
                Ok(())
            }
            Type::Generic(name) => write!(f, "{}", name),
            Type::Union(types) => {
                for (i, typ) in types.iter().enumerate() {
                    if i > 0 { write!(f, " | ")?; }
                    write!(f, "{}", typ)?;
                }
                Ok(())
            }
            Type::Optional(inner) => write!(f, "{}?", inner),
            Type::Unknown => write!(f, "unknown"),
        }
    }
}

/// Type system manager
pub struct TypeSystem {
    type_environments: HashMap<String, TypeEnvironment>,
    global_types: HashMap<String, Type>,
    language_adapters: HashMap<String, Box<dyn LanguageTypeAdapter>>,
}

impl TypeSystem {
    pub fn new() -> Self {
        let mut system = Self {
            type_environments: HashMap::new(),
            global_types: HashMap::new(),
            language_adapters: HashMap::new(),
        };
        
        // Register built-in language adapters
        system.register_language_adapter("python", Box::new(PythonTypeAdapter));
        system.register_language_adapter("javascript", Box::new(JavaScriptTypeAdapter));
        system.register_language_adapter("java", Box::new(JavaTypeAdapter));
        system.register_language_adapter("c", Box::new(CTypeAdapter));
        system.register_language_adapter("cpp", Box::new(CppTypeAdapter));
        
        system
    }

    pub fn register_language_adapter(&mut self, language: &str, adapter: Box<dyn LanguageTypeAdapter>) {
        self.language_adapters.insert(language.to_string(), adapter);
    }

    /// Check types for a program
    pub fn check(&self, program: &Program) -> Result<Program> {
        // This would implement full type checking
        // For now, return the program as-is
        Ok(program.clone())
    }

    /// Validate types in a program
    pub fn validate(&self, program: &Program) -> Result<Vec<Diagnostic>> {
        let mut diagnostics = Vec::new();
        
        // Type checking logic would go here
        // For now, return empty diagnostics
        
        Ok(diagnostics)
    }

    /// Infer the type of an expression
    pub fn infer_type(&self, expression: &crate::ast::Expression) -> Type {
        match expression {
            crate::ast::Expression::Literal { value, .. } => value.type_hint(),
            crate::ast::Expression::Identifier { .. } => Type::Unknown, // Would look up in environment
            crate::ast::Expression::Binary { left, operator, right, .. } => {
                use crate::ast::BinaryOperator;
                let left_type = self.infer_type(left);
                let right_type = self.infer_type(right);
                
                match operator {
                    BinaryOperator::Add | BinaryOperator::Subtract | 
                    BinaryOperator::Multiply | BinaryOperator::Divide | 
                    BinaryOperator::Modulo => {
                        if left_type == Type::Number && right_type == Type::Number {
                            Type::Number
                        } else {
                            Type::Unknown
                        }
                    }
                    BinaryOperator::Equal | BinaryOperator::NotEqual |
                    BinaryOperator::Less | BinaryOperator::LessEqual |
                    BinaryOperator::Greater | BinaryOperator::GreaterEqual => Type::Boolean,
                    BinaryOperator::And | BinaryOperator::Or => Type::Boolean,
                }
            }
            crate::ast::Expression::CrossCall { language, .. } => {
                // Would look up the function's return type in the target language
                Type::Unknown
            }
            _ => Type::Unknown,
        }
    }
}

impl Default for TypeSystem {
    fn default() -> Self {
        Self::new()
    }
}

/// Type environment for a scope
#[derive(Debug, Clone)]
pub struct TypeEnvironment {
    variables: HashMap<String, Type>,
    functions: HashMap<String, Type>,
    parent: Option<Box<TypeEnvironment>>,
}

impl TypeEnvironment {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            functions: HashMap::new(),
            parent: None,
        }
    }

    pub fn with_parent(parent: TypeEnvironment) -> Self {
        Self {
            variables: HashMap::new(),
            functions: HashMap::new(),
            parent: Some(Box::new(parent)),
        }
    }

    pub fn define_variable(&mut self, name: &str, typ: Type) {
        self.variables.insert(name.to_string(), typ);
    }

    pub fn define_function(&mut self, name: &str, typ: Type) {
        self.functions.insert(name.to_string(), typ);
    }

    pub fn lookup_variable(&self, name: &str) -> Option<&Type> {
        self.variables.get(name)
            .or_else(|| self.parent.as_ref().and_then(|p| p.lookup_variable(name)))
    }

    pub fn lookup_function(&self, name: &str) -> Option<&Type> {
        self.functions.get(name)
            .or_else(|| self.parent.as_ref().and_then(|p| p.lookup_function(name)))
    }
}

impl Default for TypeEnvironment {
    fn default() -> Self {
        Self::new()
    }
}

/// Language-specific type adapter trait
pub trait LanguageTypeAdapter {
    fn native_to_utopia(&self, native_type: &str) -> Option<Type>;
    fn utopia_to_native(&self, utopia_type: &Type) -> Option<String>;
    fn can_convert(&self, from: &Type, to: &Type) -> bool;
}

/// Python type adapter
pub struct PythonTypeAdapter;

impl LanguageTypeAdapter for PythonTypeAdapter {
    fn native_to_utopia(&self, native_type: &str) -> Option<Type> {
        match native_type {
            "int" | "float" => Some(Type::Number),
            "str" => Some(Type::String),
            "bool" => Some(Type::Boolean),
            "list" => Some(Type::Array(Box::new(Type::Unknown))),
            "dict" => Some(Type::Object(HashMap::new())),
            "None" => Some(Type::Null),
            _ => None,
        }
    }

    fn utopia_to_native(&self, utopia_type: &Type) -> Option<String> {
        match utopia_type {
            Type::Number => Some("float".to_string()),
            Type::String => Some("str".to_string()),
            Type::Boolean => Some("bool".to_string()),
            Type::Array(_) => Some("list".to_string()),
            Type::Object(_) => Some("dict".to_string()),
            Type::Null => Some("None".to_string()),
            _ => None,
        }
    }

    fn can_convert(&self, from: &Type, to: &Type) -> bool {
        from.is_compatible_with(to)
    }
}

/// JavaScript type adapter
pub struct JavaScriptTypeAdapter;

impl LanguageTypeAdapter for JavaScriptTypeAdapter {
    fn native_to_utopia(&self, native_type: &str) -> Option<Type> {
        match native_type {
            "number" => Some(Type::Number),
            "string" => Some(Type::String),
            "boolean" => Some(Type::Boolean),
            "Array" => Some(Type::Array(Box::new(Type::Unknown))),
            "Object" => Some(Type::Object(HashMap::new())),
            "null" | "undefined" => Some(Type::Null),
            _ => None,
        }
    }

    fn utopia_to_native(&self, utopia_type: &Type) -> Option<String> {
        match utopia_type {
            Type::Number => Some("number".to_string()),
            Type::String => Some("string".to_string()),
            Type::Boolean => Some("boolean".to_string()),
            Type::Array(_) => Some("Array".to_string()),
            Type::Object(_) => Some("Object".to_string()),
            Type::Null => Some("null".to_string()),
            _ => None,
        }
    }

    fn can_convert(&self, from: &Type, to: &Type) -> bool {
        from.is_compatible_with(to)
    }
}

/// Java type adapter
pub struct JavaTypeAdapter;

impl LanguageTypeAdapter for JavaTypeAdapter {
    fn native_to_utopia(&self, native_type: &str) -> Option<Type> {
        match native_type {
            "int" | "Integer" | "double" | "Double" | "float" | "Float" => Some(Type::Number),
            "String" => Some(Type::String),
            "boolean" | "Boolean" => Some(Type::Boolean),
            "List" | "ArrayList" => Some(Type::Array(Box::new(Type::Unknown))),
            "Map" | "HashMap" => Some(Type::Object(HashMap::new())),
            "null" => Some(Type::Null),
            _ => None,
        }
    }

    fn utopia_to_native(&self, utopia_type: &Type) -> Option<String> {
        match utopia_type {
            Type::Number => Some("double".to_string()),
            Type::String => Some("String".to_string()),
            Type::Boolean => Some("boolean".to_string()),
            Type::Array(_) => Some("List".to_string()),
            Type::Object(_) => Some("Map".to_string()),
            Type::Null => Some("null".to_string()),
            _ => None,
        }
    }

    fn can_convert(&self, from: &Type, to: &Type) -> bool {
        from.is_compatible_with(to)
    }
}

/// C type adapter
pub struct CTypeAdapter;

impl LanguageTypeAdapter for CTypeAdapter {
    fn native_to_utopia(&self, native_type: &str) -> Option<Type> {
        match native_type {
            "int" | "float" | "double" | "long" => Some(Type::Number),
            "char*" => Some(Type::String),
            "bool" => Some(Type::Boolean),
            "void" => Some(Type::Void),
            "NULL" => Some(Type::Null),
            _ => None,
        }
    }

    fn utopia_to_native(&self, utopia_type: &Type) -> Option<String> {
        match utopia_type {
            Type::Number => Some("double".to_string()),
            Type::String => Some("char*".to_string()),
            Type::Boolean => Some("bool".to_string()),
            Type::Void => Some("void".to_string()),
            Type::Null => Some("NULL".to_string()),
            _ => None,
        }
    }

    fn can_convert(&self, from: &Type, to: &Type) -> bool {
        from.is_compatible_with(to)
    }
}

/// C++ type adapter
pub struct CppTypeAdapter;

impl LanguageTypeAdapter for CppTypeAdapter {
    fn native_to_utopia(&self, native_type: &str) -> Option<Type> {
        match native_type {
            "int" | "float" | "double" | "long" => Some(Type::Number),
            "string" | "std::string" => Some(Type::String),
            "bool" => Some(Type::Boolean),
            "void" => Some(Type::Void),
            "nullptr" => Some(Type::Null),
            _ if native_type.starts_with("vector") => Some(Type::Array(Box::new(Type::Unknown))),
            _ if native_type.starts_with("map") => Some(Type::Object(HashMap::new())),
            _ => None,
        }
    }

    fn utopia_to_native(&self, utopia_type: &Type) -> Option<String> {
        match utopia_type {
            Type::Number => Some("double".to_string()),
            Type::String => Some("std::string".to_string()),
            Type::Boolean => Some("bool".to_string()),
            Type::Array(_) => Some("std::vector".to_string()),
            Type::Object(_) => Some("std::map".to_string()),
            Type::Void => Some("void".to_string()),
            Type::Null => Some("nullptr".to_string()),
            _ => None,
        }
    }

    fn can_convert(&self, from: &Type, to: &Type) -> bool {
        from.is_compatible_with(to)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_compatibility() {
        assert!(Type::Number.is_compatible_with(&Type::Number));
        assert!(Type::Null.is_compatible_with(&Type::Optional(Box::new(Type::String))));
        assert!(Type::Unknown.is_compatible_with(&Type::Number));
    }

    #[test]
    fn test_language_adapters() {
        let adapter = PythonTypeAdapter;
        assert_eq!(adapter.native_to_utopia("int"), Some(Type::Number));
        assert_eq!(adapter.utopia_to_native(&Type::String), Some("str".to_string()));
    }

    #[test]
    fn test_type_display() {
        assert_eq!(Type::Number.to_string(), "number");
        assert_eq!(Type::Array(Box::new(Type::String)).to_string(), "string[]");
        assert_eq!(Type::Optional(Box::new(Type::Number)).to_string(), "number?");
    }
} 