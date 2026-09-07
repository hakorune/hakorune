//! Source-selected Array control bindings; no reconstruction from finished MIR.
use crate::mir::builder::normal_script_source_continuation::{
    ArrayLocalRecipeV1, ArrayReleaseRoleV1,
};
use crate::mir::{ArrayWriteSiteId, BasicBlockId, MirBuilder, MirInstruction, ValueId};

#[path = "script_array_control_emission.rs"]
pub(in crate::mir::builder) mod control;

#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct ArrayInvokeSite {
    pub origin: BasicBlockId,
    pub normal: BasicBlockId,
    pub fault: BasicBlockId,
}
#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct ArrayCleanupEmission {
    pub block: BasicBlockId,
    pub releases: Vec<(ArrayReleaseRoleV1, ValueId)>,
}
#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct ArrayLiteralEmission {
    pub recipe: ArrayLocalRecipeV1,
    pub entry: BasicBlockId,
    pub frame: ValueId,
    pub allocation: ValueId,
    pub allocation_site: (BasicBlockId, usize),
    pub allocation_control: ArrayInvokeSite,
    pub allocation_fault: ArrayCleanupEmission,
    pub acquired_fault: ArrayCleanupEmission,
    pub claim: String,
    pub claim_control: ArrayInvokeSite,
    pub elements: Vec<ArrayElementEmission>,
}
#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct ArrayElementEmission {
    pub value: ValueId,
    pub definition: MirInstruction,
    pub definition_site: (BasicBlockId, usize),
    pub write: ArrayWriteSiteId,
    pub control: ArrayInvokeSite,
}

pub(in crate::mir::builder) fn last_instruction(
    builder: &MirBuilder,
) -> Result<((BasicBlockId, usize), &MirInstruction), String> {
    let function = builder
        .function_state
        .current_function
        .as_ref()
        .ok_or_else(|| fault("function-missing"))?;
    let block_id = builder
        .function_state
        .current_block
        .ok_or_else(|| fault("block-missing"))?;
    let block = function
        .blocks
        .get(&block_id)
        .ok_or_else(|| fault("block-missing"))?;
    let index = block
        .instructions
        .len()
        .checked_sub(1)
        .ok_or_else(|| fault("instruction-missing"))?;
    Ok(((block_id, index), &block.instructions[index]))
}
fn fault(reason: &str) -> String {
    format!("[freeze:contract][array-emission/{reason}]")
}
