use serde_json::json;

use crate::mir::{BasicBlockId, MirInstruction, ValueId};

pub(crate) fn emit_branch(
    condition: &ValueId,
    then_bb: &BasicBlockId,
    else_bb: &BasicBlockId,
) -> serde_json::Value {
    json!({"op":"branch","cond": condition.as_u32(), "then": then_bb.as_u32(), "else": else_bb.as_u32()})
}

pub(crate) fn emit_jump(target: &BasicBlockId) -> serde_json::Value {
    json!({"op":"jump","target": target.as_u32()})
}

pub(crate) fn emit_return(value: Option<&ValueId>) -> serde_json::Value {
    json!({"op":"ret","value": value.map(|v| v.as_u32())})
}

pub(crate) fn emit_terminator(term: &MirInstruction) -> Result<serde_json::Value, String> {
    if !crate::mir::contracts::backend_core_ops::is_supported_mir_json_terminator(term) {
        return Err(format!(
            "MIR JSON emit contract violation: unsupported terminator {}",
            crate::mir::contracts::backend_core_ops::instruction_tag(term)
        ));
    }

    match term {
        MirInstruction::Return { value } => Ok(emit_return(value.as_ref())),
        MirInstruction::Jump { target, .. } => Ok(emit_jump(target)),
        MirInstruction::Branch {
            condition,
            then_bb,
            else_bb,
            ..
        } => Ok(emit_branch(condition, then_bb, else_bb)),
        _ => unreachable!("pre-checked by backend_core_ops allowlist"),
    }
}
