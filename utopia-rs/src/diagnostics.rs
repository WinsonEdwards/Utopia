//! Diagnostics system for Utopia compiler
//!
//! This module provides comprehensive error reporting, warnings, and hints
//! for better developer experience.

use crate::Span;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Diagnostic severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiagnosticKind {
    Error,
    Warning,
    Info,
    Hint,
}

impl fmt::Display for DiagnosticKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DiagnosticKind::Error => write!(f, "error"),
            DiagnosticKind::Warning => write!(f, "warning"),
            DiagnosticKind::Info => write!(f, "info"),
            DiagnosticKind::Hint => write!(f, "hint"),
        }
    }
}

/// A diagnostic message with location and severity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Diagnostic {
    pub kind: DiagnosticKind,
    pub message: String,
    pub span: Span,
    pub code: Option<String>,
    pub suggestions: Vec<Suggestion>,
}

impl Diagnostic {
    pub fn error(message: String, span: Span) -> Self {
        Self {
            kind: DiagnosticKind::Error,
            message,
            span,
            code: None,
            suggestions: Vec::new(),
        }
    }

    pub fn warning(message: String, span: Span) -> Self {
        Self {
            kind: DiagnosticKind::Warning,
            message,
            span,
            code: None,
            suggestions: Vec::new(),
        }
    }

    pub fn info(message: String, span: Span) -> Self {
        Self {
            kind: DiagnosticKind::Info,
            message,
            span,
            code: None,
            suggestions: Vec::new(),
        }
    }

    pub fn hint(message: String, span: Span) -> Self {
        Self {
            kind: DiagnosticKind::Hint,
            message,
            span,
            code: None,
            suggestions: Vec::new(),
        }
    }

    pub fn with_code(mut self, code: String) -> Self {
        self.code = Some(code);
        self
    }

    pub fn with_suggestion(mut self, suggestion: Suggestion) -> Self {
        self.suggestions.push(suggestion);
        self
    }
}

/// A suggestion for fixing a diagnostic
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Suggestion {
    pub message: String,
    pub span: Span,
    pub replacement: Option<String>,
}

impl Suggestion {
    pub fn new(message: String, span: Span) -> Self {
        Self {
            message,
            span,
            replacement: None,
        }
    }

    pub fn with_replacement(mut self, replacement: String) -> Self {
        self.replacement = Some(replacement);
        self
    }
} 