//! Error types for the core module

use thiserror::Error;

/// Core module errors
#[derive(Error, Debug)]
pub enum Error {
    #[error("Database error: {0}")]
    Database(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Storage error: {0}")]
    Storage(String),
}

/// Result type alias for core operations
pub type Result<T> = std::result::Result<T, Error>;
