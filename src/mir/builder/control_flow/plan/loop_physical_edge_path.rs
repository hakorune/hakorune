//! Explicit logical-edge to physical-CFG path witnesses for Loop lowering.
//!
//! This is a physical mapping product, not a semantic recipe or CFG owner.
//! The verified map seals its endpoint and predecessor laws before the PHI
//! materializer can consume it.

use crate::mir::loop_recipe_contract::{LoopJoinEdgeRoleV1, LoopNodeKeyV1};
use crate::mir::BasicBlockId;

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) struct LoopPhysicalEdgePathV1 {
    pub(in crate::mir::builder) loop_key: LoopNodeKeyV1,
    pub(in crate::mir::builder) role: LoopJoinEdgeRoleV1,
    pub(in crate::mir::builder) blocks: Box<[BasicBlockId]>,
    pub(in crate::mir::builder) terminal_predecessor: BasicBlockId,
}

impl LoopPhysicalEdgePathV1 {
    pub(in crate::mir::builder) fn from_parts(
        loop_key: LoopNodeKeyV1,
        role: LoopJoinEdgeRoleV1,
        blocks: Vec<BasicBlockId>,
        terminal_predecessor: BasicBlockId,
    ) -> Self {
        Self {
            loop_key,
            role,
            blocks: blocks.into_boxed_slice(),
            terminal_predecessor,
        }
    }
}
