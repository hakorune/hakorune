//! Fallible canonical CFG edge/seal substrate.
//!
//! SSA-C1 keeps this module disconnected from production lowering. MIR
//! terminators are the graph truth; cached successors/predecessors are checked
//! witnesses and are never repaired here.

// SSA-C1 intentionally lands the complete substrate before its SSA-I1
// production connection. Remove this allowance with the first real caller.
#![allow(dead_code)]

mod error;
mod predecessors;
mod session;

#[cfg(test)]
mod tests;

#[allow(unused_imports)]
pub(in crate::mir::builder) use error::CanonicalCfgErrorV1;
// SSA-C1 intentionally exposes the complete disconnected facade before the
// first production caller lands in SSA-I1.
#[allow(unused_imports)]
pub(in crate::mir::builder) use session::{
    CanonicalCfgSessionV1, VerifiedCanonicalCfgV1, VerifiedPredecessorsV1,
};
