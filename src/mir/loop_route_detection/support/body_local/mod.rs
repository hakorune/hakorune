//! Loop-body-local promotion support for JoinIR route planning.
//!
//! This family owns carrier, condition, digit-position promotion, and their
//! private detector helpers.

/// Carrier promotion support.
pub mod carrier;

/// Condition promotion support.
pub mod condition;

mod digitpos;
mod digitpos_detector;
mod trim_detector;
