//! CFG-cache support for the disconnected legacy PHI repair candidate.

use super::edge_verifier::{derive_terminator_cfg_view_v1, PhiEdgeVerificationErrorV1};
use crate::mir::{BasicBlockId, MirFunction};

pub(super) fn rebuild_cfg_caches_from_terminators_v1(
    function: &mut MirFunction,
) -> Result<(), Vec<PhiEdgeVerificationErrorV1>> {
    let view = derive_terminator_cfg_view_v1(function)?;
    for block_id in sorted_block_ids(function) {
        let block = function
            .blocks
            .get_mut(&block_id)
            .expect("sorted block key remains present");
        block.successors = block.successors_from_terminator();
        block.predecessors = view.predecessors(block_id);
        block.reachable = view.is_reachable(block_id);
    }
    Ok(())
}

pub(super) fn sorted_block_ids(function: &MirFunction) -> Vec<BasicBlockId> {
    let mut block_ids = function.blocks.keys().copied().collect::<Vec<_>>();
    block_ids.sort_by_key(|block_id| block_id.0);
    block_ids
}
