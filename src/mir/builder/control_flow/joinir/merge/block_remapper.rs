//! Block ID Remap SSOT
//!
//! Phase 284 P1: Consolidated block ID remapping logic.
//!
//! This module provides the SSOT for block ID remapping during JoinIR → MIR merge.
//! Previously, remapping logic was duplicated across instruction_rewriter.rs and terminator.rs.
//!
//! ## Priority Rule (Phase 284 P1 Fix)
//!
//! **local_block_map takes precedence over skipped_entry_redirects.**
//!
//! ### Why?
//!
//! Function-local block IDs may collide with skipped_entry_redirects (global IDs).
//!
//! Example scenario:
//! - `loop_step`'s bb4 (continue block) - local to loop_step function
//! - `k_exit`'s remapped entry (also bb4) - global redirect to exit_block_id
//!
//! If we check `skipped_entry_redirects` first, function-local blocks get incorrectly
//! redirected to `exit_block_id`, breaking the loop control flow.
//!
//! ### Solution
//!
//! ```text
//! 1. Check local_block_map (function-local blocks)
//! 2. Fallback to skipped_entry_redirects (cross-function references to skipped continuations)
//! 3. Final fallback to original block_id
//! ```
//!
//! ## Usage
//!
//! ```rust,ignore
//! use crate::mir::builder::control_flow::joinir::merge::block_remapper::remap_block_id;
//!
//! let remapped = remap_block_id(
//!     original_block_id,
//!     &local_block_map,
//!     &skipped_entry_redirects,
//! );
//! ```

use crate::mir::BasicBlockId;
use std::collections::BTreeMap;

/// Remap block ID with correct priority (Phase 284 P1 SSOT)
///
/// # Arguments
///
/// * `block_id` - Original block ID to remap
/// * `local_block_map` - Function-local block remapping (PRIORITY 1)
/// * `skipped_entry_redirects` - Global redirect map for skipped continuation entry blocks (PRIORITY 2)
///
/// # Returns
///
/// Remapped block ID, following the priority rule:
/// 1. local_block_map (function-local)
/// 2. skipped_entry_redirects (cross-function, skipped continuations)
/// 3. original block_id (fallback - no remapping needed)
///
/// # Phase 284 P1 Fix
///
/// This function consolidates the remapping logic that was previously scattered across:
/// - `instruction_rewriter.rs` (lines 496-527: Branch instruction remapping)
/// - `terminator.rs` (remap_jump, remap_branch functions)
///
/// The inline logic had the same structure but was duplicated. This SSOT ensures:
/// - Consistent priority rule across all remapping sites
/// - Single source of truth for future modifications
/// - Clear documentation of the WHY (collision prevention)
pub fn remap_block_id(
    block_id: BasicBlockId,
    local_block_map: &BTreeMap<BasicBlockId, BasicBlockId>,
    skipped_entry_redirects: &BTreeMap<BasicBlockId, BasicBlockId>,
) -> BasicBlockId {
    local_block_map
        .get(&block_id)
        .or_else(|| skipped_entry_redirects.get(&block_id))
        .copied()
        .unwrap_or(block_id)
}

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_local_map_priority() {
        let local_map = [(BasicBlockId(4), BasicBlockId(10))]
            .iter()
            .cloned()
            .collect();
        let skip_map = [(BasicBlockId(4), BasicBlockId(99))]
            .iter()
            .cloned()
            .collect();

        // local_block_map takes precedence
        let result = remap_block_id(BasicBlockId(4), &local_map, &skip_map);
        assert_eq!(result, BasicBlockId(10));
    }

    #[test]
    fn test_skipped_redirect_fallback() {
        let local_map = BTreeMap::new();
        let skip_map = [(BasicBlockId(4), BasicBlockId(99))]
            .iter()
            .cloned()
            .collect();

        // Falls back to skipped_entry_redirects
        let result = remap_block_id(BasicBlockId(4), &local_map, &skip_map);
        assert_eq!(result, BasicBlockId(99));
    }

    #[test]
    fn test_no_remap() {
        let local_map = BTreeMap::new();
        let skip_map = BTreeMap::new();

        // Returns original if not found in either map
        let result = remap_block_id(BasicBlockId(42), &local_map, &skip_map);
        assert_eq!(result, BasicBlockId(42));
    }

    #[test]
    fn test_collision_scenario() {
        // Simulate the Phase 284 P1 bug scenario:
        // - loop_step's bb4 (local) should map to bb10
        // - k_exit's bb4 (skipped) should map to bb99 (exit_block_id)
        let local_map = [(BasicBlockId(4), BasicBlockId(10))]
            .iter()
            .cloned()
            .collect();
        let skip_map = [(BasicBlockId(4), BasicBlockId(99))]
            .iter()
            .cloned()
            .collect();

        // When processing loop_step's bb4, local_map should win
        let result = remap_block_id(BasicBlockId(4), &local_map, &skip_map);
        assert_eq!(
            result,
            BasicBlockId(10),
            "local_block_map must take precedence to avoid collision"
        );
    }
}
