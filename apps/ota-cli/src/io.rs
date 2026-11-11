//! Filesystem helpers shared by the subcommands.

use std::fs;
use std::path::Path;

use anyhow::{Context, Result};

/// Read a file into memory.
///
/// # Errors
///
/// Returns an error when the file cannot be read.
pub fn read(path: &Path) -> Result<Vec<u8>> {
    fs::read(path).with_context(|| format!("failed to read {}", path.display()))
}

/// Read a file into a UTF-8 string.
///
/// # Errors
///
/// Returns an error when the file cannot be read or is not valid UTF-8.
pub fn read_to_string(path: &Path) -> Result<String> {
    fs::read_to_string(path).with_context(|| format!("failed to read {}", path.display()))
}

/// Write bytes to a file, creating or truncating it.
///
/// # Errors
///
/// Returns an error when the file cannot be written.
pub fn write(path: &Path, bytes: &[u8]) -> Result<()> {
    fs::write(path, bytes).with_context(|| format!("failed to write {}", path.display()))
}
