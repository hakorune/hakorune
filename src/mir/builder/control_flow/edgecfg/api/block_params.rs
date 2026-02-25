use crate::mir::join_ir::lowering::inline_boundary::JumpArgsLayout;
use crate::mir::ValueId;

/// Join block params (same layout as EdgeArgs).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockParams {
    pub layout: JumpArgsLayout,
    pub params: Vec<ValueId>,
}
