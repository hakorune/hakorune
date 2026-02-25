//! Phase 287 P0.2: Value ID remapping (pure helper)
//!
//! Allocates new ValueIds for all JoinIR values, avoiding conflicts with:
//! - Host function's existing ValueIds
//! - Reserved PHI dst ValueIds (allocated by LoopHeaderPhiBuilder)
//!
//! This is Phase 3 of the merge pipeline.

use crate::mir::ValueId;
use std::collections::{BTreeSet, HashSet};

/// Phase 3: Allocate new ValueIds for all collected values
///
/// Phase 201-A: Accept reserved ValueIds that must not be reused.
/// These are PHI dst ValueIds that will be created by LoopHeaderPhiBuilder.
/// We must skip these IDs to prevent carrier value corruption.
pub(super) fn remap_values(
    builder: &mut crate::mir::builder::MirBuilder,
    used_values: &BTreeSet<ValueId>,
    remapper: &mut crate::mir::builder::joinir_id_remapper::JoinIrIdRemapper,
    reserved_ids: &HashSet<ValueId>,
    debug: bool,
) -> Result<(), String> {
    let trace = super::trace::trace();
    trace.stderr_if(
        &format!(
            "[cf_loop/joinir] Phase 3: Remapping {} ValueIds (reserved: {})",
            used_values.len(),
            reserved_ids.len()
        ),
        debug,
    );

    for old_value in used_values {
        // Phase 201-A: Allocate new ValueId, skipping reserved PHI dsts
        let new_value = loop {
            let candidate = builder.next_value_id();
            if !reserved_ids.contains(&candidate) {
                break candidate;
            }
            // Skip reserved ID - will try next one
            trace.stderr_if(
                &format!(
                    "[cf_loop/joinir] Phase 201-A: Skipping reserved PHI dst {:?}",
                    candidate
                ),
                debug,
            );
        };

        remapper.set_value(*old_value, new_value);
        trace.stderr_if(
            &format!(
                "[cf_loop/joinir]   Value remap: {:?} → {:?}",
                old_value, new_value
            ),
            debug,
        );
    }

    Ok(())
}
