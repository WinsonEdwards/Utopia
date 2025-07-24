//! Configuration for Utopia compiler

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub optimization_level: u8,
    pub target: String,
    pub debug: bool,
    pub language_configs: HashMap<String, LanguageConfig>,
}

impl Config {
    pub fn new() -> Self {
        Self {
            optimization_level: 2,
            target: "native".to_string(),
            debug: false,
            language_configs: HashMap::new(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageConfig {
    pub enabled: bool,
    pub options: HashMap<String, String>,
}

impl LanguageConfig {
    pub fn new() -> Self {
        Self {
            enabled: true,
            options: HashMap::new(),
        }
    }
}

impl Default for LanguageConfig {
    fn default() -> Self {
        Self::new()
    }
} 