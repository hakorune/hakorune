//! Fallible canonical CFG edge/seal substrate.
//!
//! SSA-I1-T connects this module to one admitted trivial whole-owner route.
//! MIR terminators are the graph truth; cached successors/predecessors are
//! checked witnesses and are never repaired here.

mod branch;
mod error;
mod fragment;
mod instruction;
pub(in crate::mir::builder) use instruction::InstructionPlacementV1;
mod open_instruction_target;
mod pinned_text_finish;
mod predecessors;
mod session;

#[cfg(test)]
mod tests;

pub(in crate::mir::builder::resolved_lowering) use branch::PreparedCanonicalBranchV1;
pub(in crate::mir::builder) use error::CanonicalCfgErrorV1;
pub(in crate::mir::builder) use open_instruction_target::{
    CanonicalOpenInstructionTargetErrorV1, VerifiedCanonicalOpenInstructionTargetV1,
};
pub(in crate::mir::builder) use session::{CanonicalCfgSessionV1, VerifiedPredecessorsV1};
