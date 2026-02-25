//! Exception handling implementation.
//!
//! This module provides exception handling control flow primitives:
//! - Try/catch/finally blocks
//! - Throw statements
//!
//! # Architecture
//!
//! Exception handling integrates with the MIR builder's cleanup and
//! deferred return mechanisms to ensure proper resource management
//! and control flow even in the presence of exceptions.
//!
//! ## Try/Catch/Finally
//!
//! The try/catch/finally implementation creates multiple basic blocks:
//! - Try block: Normal execution path
//! - Catch block: Exception handler
//! - Finally block: Cleanup code (optional)
//! - Exit block: Continuation after exception handling
//!
//! ## Cleanup Blocks
//!
//! Cleanup blocks (finally blocks) have special restrictions:
//! - Return statements require `NYASH_CLEANUP_ALLOW_RETURN=1`
//! - Throw statements require `NYASH_CLEANUP_ALLOW_THROW=1`
//!
//! # Modules
//!
//! - `try_catch`: Try/catch/finally block implementation
//! - `throw`: Throw statement implementation

mod throw;
mod try_catch;

pub(in crate::mir::builder) use throw::cf_throw;
pub(in crate::mir::builder) use try_catch::cf_try_catch;
