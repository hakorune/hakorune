use crate::mir::{BasicBlockId, ValueId};

/// Ephemeral bridge proof for one source-issued Program-v0 call anchor.
///
/// The bridge owns this receipt only for the source-artifact handoff. It is not
/// serialized into MIR and cannot be used by the body-only compatibility path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Stage1ProgramJsonCallAnchorReceiptV1 {
    pub(crate) anchor: u32,
    pub(crate) function_name: String,
    pub(crate) block: BasicBlockId,
    pub(crate) instruction_index: usize,
    pub(crate) dst: Option<ValueId>,
}
