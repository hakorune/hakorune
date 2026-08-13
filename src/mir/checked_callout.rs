//! Neutral MIR vocabulary for a checked call with canonical Normal/Fault CFG.
//!
//! This module is deliberately physical-only. It does not resolve a provider,
//! selector, runtime lease token, or backend function address. A function-local
//! site plan is admitted once and the canonical CFG/SSA sessions consume it.

mod census;
mod site_plan;

pub(crate) use census::*;
pub(crate) use site_plan::*;

#[cfg(test)]
mod tests;
