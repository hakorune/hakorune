//! Phase 287 P4: Entry function resolution logic
//!
//! Extracted from plan.rs lines 122-155
//!
//! Responsibilities:
//! - Resolve entry function name (loop header) from boundary or heuristic
//! - Reverse lookup function name by entry block
//! - Entry-like判定は tail_call_policy に集約

use crate::mir::{BasicBlockId, MirFunction};
use crate::mir::join_ir::lowering::inline_boundary::JoinInlineBoundary;
use crate::mir::join_ir::lowering::canonical_names;
use std::collections::{BTreeMap, BTreeSet};

/// Determine entry function (loop header)
///
/// Phase 287 P2: Prefer boundary SSOT (loop_header_func_name) over heuristic.
///
/// Returns Option<String> (not &str) to avoid lifetime issues.
pub(super) fn resolve_entry_func_name(
    functions_merge: &Vec<(&String, &MirFunction)>,
    boundary: Option<&JoinInlineBoundary>,
    continuation_candidates: &BTreeSet<String>,
) -> Option<String> {
    boundary
        .and_then(|b| b.loop_header_func_name.clone())
        .or_else(|| {
            functions_merge
                .iter()
                .find(|(name, _)| {
                    let name_str = name.as_str();
                    let is_continuation = continuation_candidates.contains(*name);
                    let is_main = name_str == canonical_names::MAIN;
                    !is_continuation && !is_main
                })
                .map(|(name, _)| (*name).clone())
        })
}

/// Reverse lookup: find function name by target block
///
/// Searches function_entry_map to find which function has the given block as entry.
pub(super) fn resolve_target_func_name<'a>(
    function_entry_map: &'a BTreeMap<String, BasicBlockId>,
    target_block: BasicBlockId,
) -> Option<&'a str> {
    function_entry_map
        .iter()
        .find_map(|(fname, &entry_block)| (entry_block == target_block).then(|| fname.as_str()))
}

// Entry-like判定は tail_call_policy.rs に集約
