//! Fallible canonical CFG edge/seal substrate.
//!
//! SSA-I1-T connects this module to one admitted trivial whole-owner route.
//! MIR terminators are the graph truth; cached successors/predecessors are
//! checked witnesses and are never repaired here.

mod error;
mod open_instruction_target;
mod pinned_text_finish;
mod predecessors;
mod session;

#[cfg(test)]
mod tests;

pub(in crate::mir::builder) use error::CanonicalCfgErrorV1;
pub(in crate::mir::builder::resolved_lowering) use open_instruction_target::{
    CanonicalOpenInstructionTargetErrorV1, VerifiedCanonicalOpenInstructionTargetV1,
};
pub(in crate::mir::builder) use session::{CanonicalCfgSessionV1, VerifiedPredecessorsV1};

/// Test-only observer seam for consumers that need the canonical CFG edge
/// witness without owning a session or repairing cached block metadata.
#[cfg(test)]
pub(in crate::mir::builder) fn verify_terminator_edges_for_test(
    function: &crate::mir::MirFunction,
) -> Result<(), CanonicalCfgErrorV1> {
    predecessors::derive_and_verify_predecessors(function).map(|_| ())
}
