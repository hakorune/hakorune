//! LLVM mode error types

use std::fmt;

/// LLVM mode execution error
#[derive(Debug)]
pub struct LlvmRunError {
    pub code: i32,
    pub msg: String,
}

impl LlvmRunError {
    /// Create new error
    pub fn new(code: i32, msg: impl Into<String>) -> Self {
        Self {
            code,
            msg: msg.into(),
        }
    }

    /// Create error with exit code 1
    pub fn fatal(msg: impl Into<String>) -> Self {
        Self::new(1, msg)
    }
}

impl fmt::Display for LlvmRunError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl std::error::Error for LlvmRunError {}
