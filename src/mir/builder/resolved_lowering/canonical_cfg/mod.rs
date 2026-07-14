//! Fallible canonical CFG edge/seal substrate.
//!
//! SSA-I1-T connects this module to one admitted trivial whole-owner route.
//! MIR terminators are the graph truth; cached successors/predecessors are
//! checked witnesses and are never repaired here.

mod error;
mod predecessors;
mod session;

#[cfg(test)]
mod tests;

#[cfg(test)]
pub(in crate::mir::builder) use error::CanonicalCfgErrorV1;
pub(in crate::mir::builder) use session::{CanonicalCfgSessionV1, VerifiedPredecessorsV1};
