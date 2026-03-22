use crate::mir::builder::MirBuilder;
use crate::mir::{BasicBlockId, ValueId};
use std::collections::BTreeMap;

/// Standard 5-block layout for simple/control/scan loops
///
/// CFG: preheader → header → body → step → header (back-edge)
///                      ↓
///                   after
#[derive(Debug, Clone, Copy)]
pub(in crate::mir::builder) struct LoopBlocksStandard5 {
    pub(in crate::mir::builder) preheader_bb: BasicBlockId,
    pub(in crate::mir::builder) header_bb: BasicBlockId,
    pub(in crate::mir::builder) body_bb: BasicBlockId,
    pub(in crate::mir::builder) step_bb: BasicBlockId,
    pub(in crate::mir::builder) after_bb: BasicBlockId,
}

impl LoopBlocksStandard5 {
    /// Allocate 5 blocks for a standard loop
    pub(in crate::mir::builder) fn allocate(builder: &mut MirBuilder) -> Result<Self, String> {
        let preheader_bb = builder
            .current_block
            .ok_or_else(|| "[normalizer] No current block for loop entry".to_string())?;
        let header_bb = builder.next_block_id();
        let body_bb = builder.next_block_id();
        let step_bb = builder.next_block_id();
        let after_bb = builder.next_block_id();
        Ok(Self {
            preheader_bb,
            header_bb,
            body_bb,
            step_bb,
            after_bb,
        })
    }
}

/// Extended 8-block layout for if-phi-join loops
///
/// CFG: preheader → header → body → then/else → merge → step → header
///                      ↓
///                   after
#[derive(Debug, Clone, Copy)]
pub(super) struct LoopBlocksWithIfPhi {
    pub(super) preheader_bb: BasicBlockId,
    pub(super) header_bb: BasicBlockId,
    pub(super) body_bb: BasicBlockId,
    pub(super) then_bb: BasicBlockId,
    pub(super) else_bb: BasicBlockId,
    pub(super) merge_bb: BasicBlockId,
    pub(super) step_bb: BasicBlockId,
    pub(super) after_bb: BasicBlockId,
}

impl LoopBlocksWithIfPhi {
    /// Allocate 8 blocks for an if-phi loop
    pub(super) fn allocate(builder: &mut MirBuilder) -> Result<Self, String> {
        let preheader_bb = builder
            .current_block
            .ok_or_else(|| "[normalizer] No current block for loop entry".to_string())?;
        let header_bb = builder.next_block_id();
        let body_bb = builder.next_block_id();
        let then_bb = builder.next_block_id();
        let else_bb = builder.next_block_id();
        let merge_bb = builder.next_block_id();
        let step_bb = builder.next_block_id();
        let after_bb = builder.next_block_id();
        Ok(Self {
            preheader_bb,
            header_bb,
            body_bb,
            then_bb,
            else_bb,
            merge_bb,
            step_bb,
            after_bb,
        })
    }
}

/// Create phi_bindings map from variable name-ValueId pairs
///
/// phi_bindings are used to override variable_map lookups during AST lowering,
/// ensuring loop variables reference PHI destinations instead of initial values.
pub(in crate::mir::builder) fn create_phi_bindings(
    bindings: &[(&str, ValueId)],
) -> BTreeMap<String, ValueId> {
    bindings
        .iter()
        .map(|(name, id)| (name.to_string(), *id))
        .collect()
}
