//! Passive box-core crate scaffold.
//!
//! This crate owns dependency-free Box policy vocabulary. It must not depend on
//! the main crate, runtime, concrete Box implementations, providers, or config.
//!
//! Current scope is deliberately small: factory policy data only. Active
//! factory registry logic and concrete Box construction stay in the main crate.

pub mod policy;

pub use policy::{FactoryPolicy, FactoryType};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoxCoreBoundary;

impl BoxCoreBoundary {
    pub const fn name(self) -> &'static str {
        "hakorune-box-core"
    }
}
