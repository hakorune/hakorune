//! Phase 33-17: Merge Result Data Structure
//!
//! Contains all information needed for exit PHI construction.
//!
//! Extracted from instruction_rewriter.rs (lines 71-81) for single responsibility.

use crate::mir::{BasicBlockId, ValueId};
use std::collections::BTreeMap;

/// Phase 131 P1 Task 1: Contract requirements for JoinIR merge operations
///
/// This is the SSOT for merge contracts and invariants that must be verified.
/// Consolidating these in a single type makes the contract requirements visible
/// and reduces the need to pass individual slices/vectors around.
#[derive(Debug, Clone)]
pub struct MergeContracts {
    /// Block IDs that are allowed to be missing from the function when verifying
    /// terminator targets. This typically includes the exit_block_id which is
    /// allocated but may not be inserted yet at verification time.
    pub allowed_missing_jump_targets: Vec<BasicBlockId>,
}

/// Phase 33-13: Return type for merge_and_rewrite
///
/// Contains all information needed for exit PHI construction.
pub struct MergeResult {
    /// The ID of the exit block where all Returns jump to
    pub exit_block_id: BasicBlockId,
    /// Vec of (from_block, return_value) for expr result PHI generation
    pub exit_phi_inputs: Vec<(BasicBlockId, ValueId)>,
    /// Map of carrier_name → Vec of (from_block, exit_value) for carrier PHI generation
    pub carrier_inputs: BTreeMap<String, Vec<(BasicBlockId, ValueId)>>,
    /// Phase 131 P1.5: Remapped exit values (JoinIR → Host ValueId)
    ///
    /// This is the SSOT for exit value remapping. The merge box owns the remapper,
    /// so it's responsible for converting JoinIR exit values to host ValueIds.
    ///
    /// Key: carrier_name (from exit_bindings)
    /// Value: host ValueId (remapper.get_value(binding.join_exit_value))
    ///
    /// Used by DirectValue mode to update variable_map without PHI generation.
    pub remapped_exit_values: BTreeMap<String, ValueId>,
}
