use serde_json::json;

use crate::mir::{BasicBlockId, EffectMask, MirInstruction, ValueId};

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

pub(crate) fn emit_checked_callout(
    site_id: &crate::mir::checked_callout::CheckedCallOutSiteIdV1,
    receiver: &ValueId,
    arguments: &[ValueId],
    normal_landing: &BasicBlockId,
    fault_landing: &BasicBlockId,
    effects: &EffectMask,
) -> serde_json::Value {
    json!({
        "op": "checked_callout",
        "site_id": site_id.0,
        "receiver": receiver.as_u32(),
        "args": arguments.iter().map(|value| value.as_u32()).collect::<Vec<_>>(),
        "normal": normal_landing.as_u32(),
        "fault": fault_landing.as_u32(),
        "effects": effects.bits(),
    })
}

pub(crate) fn emit_checked_callout_fault(
    site_id: &crate::mir::checked_callout::CheckedCallOutSiteIdV1,
) -> serde_json::Value {
    json!({"op":"checked_callout_fault", "site_id": site_id.0})
}

pub(crate) fn emit_checked_callout_normal_result(
    site_id: &crate::mir::checked_callout::CheckedCallOutSiteIdV1,
    dst: &ValueId,
) -> serde_json::Value {
    json!({"op":"checked_callout_normal_result", "site_id": site_id.0, "dst": dst.as_u32()})
}

pub(crate) fn emit_checked_callout_end(
    site_id: &crate::mir::checked_callout::CheckedCallOutSiteIdV1,
    lease_slot: &crate::mir::checked_callout::CheckedCallOutLeaseSlotIdV1,
) -> serde_json::Value {
    json!({"op":"checked_callout_end", "site_id": site_id.0, "lease_slot": lease_slot.0})
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
        MirInstruction::CheckedCallOut {
            site_id,
            receiver,
            arguments,
            normal_landing,
            fault_landing,
            effects,
        } => Ok(emit_checked_callout(
            site_id,
            receiver,
            arguments,
            normal_landing,
            fault_landing,
            effects,
        )),
        MirInstruction::CheckedCallOutFault { site_id } => Ok(emit_checked_callout_fault(site_id)),
        _ => unreachable!("pre-checked by backend_core_ops allowlist"),
    }
}
