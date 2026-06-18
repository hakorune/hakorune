//! Passive AOT backend support boundary.
//!
//! This crate owns AOT configuration, errors, and executable packaging support.
//! It intentionally does not own MIR-to-WASM compilation.

mod config;
mod error;
mod executable;

pub use config::AotConfig;
pub use error::AotError;
pub use executable::ExecutableBuilder;
