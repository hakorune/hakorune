//! Error generation utilities for MIR Interpreter
//!
//! Purpose: Centralize error message generation to reduce duplication
//! and ensure consistent error formatting across ~95 error sites.
//!
//! Phase 3 refactoring: 200-300 lines saved

use crate::backend::vm_types::VMError;

/// Error message builder utilities
///
/// Provides standardized error generation methods to replace
/// scattered `VMError::InvalidInstruction(...)` calls.
pub struct ErrorBuilder;

impl ErrorBuilder {
    /// General invalid instruction error
    ///
    /// Use for simple error messages without specific patterns.
    ///
    /// # Example
    /// ```ignore
    /// return Err(ErrorBuilder::invalid_instruction("push expects 1 arg"));
    /// ```
    #[inline]
    pub fn invalid_instruction(msg: impl Into<String>) -> VMError {
        VMError::InvalidInstruction(msg.into())
    }

    /// Type mismatch error with consistent formatting
    ///
    /// # Arguments
    /// * `method` - Method or operation name
    /// * `expected` - Expected type description
    /// * `actual` - Actual type received
    ///
    /// # Example
    /// ```ignore
    /// ErrorBuilder::type_mismatch("get", "Integer", "String")
    /// // => "get expects Integer type, got String"
    /// ```
    #[inline]
    #[allow(dead_code)]
    pub fn type_mismatch(method: &str, expected: &str, actual: &str) -> VMError {
        VMError::InvalidInstruction(format!(
            "{} expects {} type, got {}",
            method, expected, actual
        ))
    }

    /// Index out of bounds error
    ///
    /// # Arguments
    /// * `method` - Method or operation name
    /// * `index` - Attempted index
    /// * `len` - Actual length/size
    ///
    /// # Example
    /// ```ignore
    /// ErrorBuilder::out_of_bounds("get", 5, 3)
    /// // => "get index out of bounds: 5 >= 3"
    /// ```
    #[inline]
    #[allow(dead_code)]
    pub fn out_of_bounds(method: &str, index: usize, len: usize) -> VMError {
        VMError::InvalidInstruction(format!(
            "{} index out of bounds: {} >= {}",
            method, index, len
        ))
    }

    /// Unsupported operation error
    ///
    /// # Example
    /// ```ignore
    /// ErrorBuilder::unsupported_operation("divide by string")
    /// // => "divide by string operation not supported"
    /// ```
    #[inline]
    pub fn unsupported_operation(operation: &str) -> VMError {
        VMError::InvalidInstruction(format!("{} operation not supported", operation))
    }

    /// Method not found on box type
    ///
    /// # Example
    /// ```ignore
    /// ErrorBuilder::method_not_found("StringBox", "push")
    /// // => "Unknown method 'push' on StringBox"
    /// ```
    #[inline]
    pub fn method_not_found(box_type: &str, method: &str) -> VMError {
        VMError::InvalidInstruction(format!("Unknown method '{}' on {}", method, box_type))
    }

    /// Receiver type error
    ///
    /// # Example
    /// ```ignore
    /// ErrorBuilder::receiver_type_error("ArrayBox")
    /// // => "receiver must be ArrayBox"
    /// ```
    #[inline]
    #[allow(dead_code)]
    pub fn receiver_type_error(expected: &str) -> VMError {
        VMError::InvalidInstruction(format!("receiver must be {}", expected))
    }

    /// Argument count mismatch
    ///
    /// # Example
    /// ```ignore
    /// ErrorBuilder::arg_count_mismatch("push", 1, 0)
    /// // => "push expects 1 arg, got 0"
    /// ```
    #[inline]
    pub fn arg_count_mismatch(method: &str, expected: usize, actual: usize) -> VMError {
        VMError::InvalidInstruction(format!(
            "{} expects {} arg{}, got {}",
            method,
            expected,
            if expected == 1 { "" } else { "s" },
            actual
        ))
    }

    /// Minimum argument count error
    ///
    /// # Example
    /// ```ignore
    /// ErrorBuilder::arg_count_min("link_object", 1, 0)
    /// // => "link_object expects at least 1 arg, got 0"
    /// ```
    #[inline]
    #[allow(dead_code)]
    pub fn arg_count_min(method: &str, min: usize, actual: usize) -> VMError {
        VMError::InvalidInstruction(format!(
            "{} expects at least {} arg{}, got {}",
            method,
            min,
            if min == 1 { "" } else { "s" },
            actual
        ))
    }

    /// Custom formatted error with context
    ///
    /// # Example
    /// ```ignore
    /// ErrorBuilder::with_context("emit_object", "invalid JSON format")
    /// // => "emit_object: invalid JSON format"
    /// ```
    #[inline]
    pub fn with_context(operation: &str, detail: &str) -> VMError {
        VMError::InvalidInstruction(format!("{}: {}", operation, detail))
    }

    /// Error from another error type
    ///
    /// # Example
    /// ```ignore
    /// ErrorBuilder::from_error("link_object", &parse_error)
    /// // => "link_object: <error message>"
    /// ```
    #[inline]
    #[allow(dead_code)]
    pub fn from_error(operation: &str, error: &dyn std::error::Error) -> VMError {
        VMError::InvalidInstruction(format!("{}: {}", operation, error))
    }
}

// Convenience methods on MirInterpreter to make error generation even shorter
impl super::super::MirInterpreter {
    /// General invalid instruction error (shortest form)
    ///
    /// # Example
    /// ```ignore
    /// return Err(self.err_invalid("push expects 1 arg"));
    /// ```
    #[inline]
    pub(crate) fn err_invalid(&self, msg: impl Into<String>) -> VMError {
        ErrorBuilder::invalid_instruction(msg)
    }

    /// Type mismatch error
    ///
    /// # Example
    /// ```ignore
    /// return Err(self.err_type_mismatch("get", "Integer", actual_type));
    /// ```
    #[inline]
    #[allow(dead_code)]
    pub(crate) fn err_type_mismatch(&self, method: &str, expected: &str, actual: &str) -> VMError {
        ErrorBuilder::type_mismatch(method, expected, actual)
    }

    /// Index out of bounds error
    ///
    /// # Example
    /// ```ignore
    /// return Err(self.err_out_of_bounds("get", idx, len));
    /// ```
    #[inline]
    #[allow(dead_code)]
    pub(crate) fn err_out_of_bounds(&self, method: &str, index: usize, len: usize) -> VMError {
        ErrorBuilder::out_of_bounds(method, index, len)
    }

    /// Unsupported operation error
    ///
    /// # Example
    /// ```ignore
    /// return Err(self.err_unsupported("divide by zero"));
    /// ```
    #[inline]
    pub(crate) fn err_unsupported(&self, operation: &str) -> VMError {
        ErrorBuilder::unsupported_operation(operation)
    }

    /// Method not found error
    ///
    /// # Example
    /// ```ignore
    /// return Err(self.err_method_not_found("StringBox", method_name));
    /// ```
    #[inline]
    pub(crate) fn err_method_not_found(&self, box_type: &str, method: &str) -> VMError {
        ErrorBuilder::method_not_found(box_type, method)
    }

    /// Argument count mismatch error
    ///
    /// # Example
    /// ```ignore
    /// return Err(self.err_arg_count("push", 1, args.len()));
    /// ```
    #[inline]
    pub(crate) fn err_arg_count(&self, method: &str, expected: usize, actual: usize) -> VMError {
        ErrorBuilder::arg_count_mismatch(method, expected, actual)
    }

    /// Error with context
    ///
    /// # Example
    /// ```ignore
    /// return Err(self.err_with_context("emit_object", "parse failed"));
    /// ```
    #[inline]
    pub(crate) fn err_with_context(&self, operation: &str, detail: &str) -> VMError {
        ErrorBuilder::with_context(operation, detail)
    }
}
