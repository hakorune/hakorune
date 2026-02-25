//! Core analysis functions for function scope capture

mod v1;
mod v2;
#[cfg(test)]
mod tests;

#[allow(unused_imports)]
pub(crate) use v1::analyze_captured_vars;
pub(crate) use v2::analyze_captured_vars_v2;
