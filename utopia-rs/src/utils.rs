//! Utility functions for Utopia compiler

use std::fs;
use std::path::Path;

/// Read a file to string
pub fn read_file<P: AsRef<Path>>(path: P) -> crate::Result<String> {
    fs::read_to_string(path).map_err(|e| e.into())
}

/// Write string to file
pub fn write_file<P: AsRef<Path>>(path: P, content: &str) -> crate::Result<()> {
    fs::write(path, content).map_err(|e| e.into())
}

/// Get file extension
pub fn get_extension<P: AsRef<Path>>(path: P) -> Option<String> {
    path.as_ref()
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_lowercase())
} 