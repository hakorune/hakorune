use crate::mir::builder::MirBuilder;
use crate::mir::BasicBlockId;

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
            .function_state
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
