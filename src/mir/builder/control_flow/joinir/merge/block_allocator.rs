//! JoinIR Block ID Allocator
//!
//! Allocates new BasicBlockIds for all blocks in JoinIR functions
//! to avoid ID conflicts with the host MIR builder.
//!
//! Phase 4 Extraction: Separated from merge_joinir_mir_blocks (lines 159-194)

use super::super::trace;
use crate::mir::builder::joinir_id_remapper::JoinIrIdRemapper;
use crate::mir::MirModule;

/// Phase 1: Allocate new block IDs for ALL functions (Phase 189)
///
/// DETERMINISM: Sort functions and blocks by name/ID to ensure consistent iteration order
///
/// Returns: (JoinIrIdRemapper, exit_block_id)
pub(super) fn allocate_blocks(
    builder: &mut crate::mir::builder::MirBuilder,
    mir_module: &MirModule,
    _debug: bool,
) -> Result<(JoinIrIdRemapper, crate::mir::BasicBlockId), String> {
    let mut remapper = JoinIrIdRemapper::new();

    // Phase 177-3: Allocate exit block FIRST to ensure it doesn't conflict with JoinIR blocks
    // This exit_block_id will be returned and used by instruction_rewriter and exit_phi_builder
    let exit_block_id = builder.next_block_id();

    trace::trace().dev(
        "cf_loop/joinir/block_allocator",
        &format!(
            "Phase 177-3: Allocated exit_block_id = {:?}",
            exit_block_id
        ),
    );

    // Phase 195: Use unified trace
    trace::trace().blocks(
        "allocator",
        "Phase 189: Allocating block IDs for all functions",
    );

    // DETERMINISM FIX: Sort functions by name to ensure consistent iteration order
    let mut functions: Vec<_> = mir_module.functions.iter().collect();
    functions.sort_by_key(|(name, _)| name.as_str());

    for (func_name, func) in functions {
        // Phase 195: Use unified trace
        trace::trace().blocks("allocator", &format!("Function: {}", func_name));

        // DETERMINISM FIX: Sort blocks by ID to ensure consistent iteration order
        let mut blocks: Vec<_> = func.blocks.iter().collect();
        blocks.sort_by_key(|(id, _)| id.0);

        for (old_block_id, _) in blocks {
            let new_block_id = builder.next_block_id();
            // Use remapper to store composite key mapping
            remapper.set_block(func_name.clone(), *old_block_id, new_block_id);

            // Phase 195: Use unified trace
            trace::trace().blocks(
                "allocator",
                &format!(
                    "Block remap: {}:{:?} → {:?}",
                    func_name, old_block_id, new_block_id
                ),
            );
        }

        // Map function entry blocks for Call→Jump conversion (stored in remapper for later use)
        let entry_block_new = remapper
            .get_block(func_name, func.entry_block)
            .ok_or_else(|| format!("Entry block not found for {}", func_name))?;

        // Phase 195: Use unified trace
        trace::trace().blocks(
            "allocator",
            &format!("Entry map: {} → {:?}", func_name, entry_block_new),
        );
    }

    // Phase 177-3: Return both remapper and exit_block_id
    Ok((remapper, exit_block_id))
}
