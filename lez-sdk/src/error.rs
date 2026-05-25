//! Error types for LEZ SDK programs.

use thiserror::Error;
use crate::output::SdkOutput;

/// Result type for LEZ program instruction handlers.
pub type SdkResult = Result<SdkOutput, SdkError>;

/// Structured error type for LEZ programs.
#[derive(Error, Debug)]
pub enum SdkError {
    /// Wrong number of accounts provided.
    #[error("expected {expected} accounts, got {actual}")]
    AccountCountMismatch { expected: usize, actual: usize },

    /// Failed to decode instruction arguments from raw bytes.
    #[error("failed to decode instruction arguments: {0}")]
    DecodeError(String),

    /// Unknown instruction discriminant.
    #[error("unknown instruction discriminant: {0}")]
    UnknownInstruction(u32),

    /// Authorization failure.
    #[error("unauthorized: {0}")]
    Unauthorized(String),

    /// Custom program error.
    #[error("program error {code}: {message}")]
    Custom { code: u32, message: String },
}

impl SdkError {
    /// Numeric error code for client-side handling.
    pub fn code(&self) -> u32 {
        match self {
            SdkError::AccountCountMismatch { .. } => 1000,
            SdkError::DecodeError(_) => 1001,
            SdkError::UnknownInstruction(_) => 1002,
            SdkError::Unauthorized(_) => 1003,
            SdkError::Custom { code, .. } => 6000 + code,
        }
    }
}
