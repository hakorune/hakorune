//! Canonical fallible transport preserves the selected call, not a display name.
use crate::mir::{
    definitions::Callee,
    instruction::{InvokeCallResultKind, InvokeOperation},
    MirInstruction,
};
use hakorune_mir_defs::{CanonicalGlobalTargetV1, CanonicalSameModuleGlobalTargetV1};
use serde_json::{json, Value};

pub(super) fn call_payload(call: &crate::mir::definitions::MirCall) -> Result<Value, String> {
    let Callee::Global(CanonicalGlobalTargetV1::SameModule(
        CanonicalSameModuleGlobalTargetV1::StaticBoxMethod {
            owner,
            method,
            arity,
        },
    )) = &call.callee
    else {
        return Err("[freeze:contract][mir-json/call-target-outside-profile]".into());
    };
    Ok(json!({"dst":call.dst.map(|v|v.as_u32()),
        "target":{"namespace":"static_box_method","owner":owner,"name":method,"arity":arity},
        "args":call.args.iter().map(|v|v.as_u32()).collect::<Vec<_>>(),
        "flags":{"tail_call":call.flags.tail_call,"no_return":call.flags.no_return,"can_inline":call.flags.can_inline,"is_constructor":call.flags.is_constructor},
        "effects_bits":call.effects.bits()}))
}

pub(super) fn emit(inst: &MirInstruction, selected: bool) -> Result<Value, String> {
    if !selected {
        return Err("[freeze:contract][mir-json/invoke-requires-canonical-transport]".into());
    }
    Ok(match inst {
        MirInstruction::Invoke {
            operation:
                InvokeOperation::Call {
                    call,
                    result: InvokeCallResultKind::I64,
                },
            fault_frame,
            normal_landing,
            fault_landing,
        } => {
            if call.dst.is_some() {
                return Err("[freeze:contract][mir-json/invoke-embedded-result]".into());
            }
            json!({"op":"invoke", "operation":{"kind":"call","result":"i64","call":call_payload(call)?},
                "fault_frame":fault_frame.as_u32(), "normal":normal_landing.as_u32(), "fault":fault_landing.as_u32()})
        }
        MirInstruction::InvokeNormalResult { invoke_block, dst } => {
            json!({"op": "invoke_normal_result", "invoke_block": invoke_block.as_u32(), "dst": dst.as_u32()})
        }
        MirInstruction::ReturnFault { fault_frame } => {
            json!({"op": "return_fault", "fault_frame": fault_frame.as_u32()})
        }
        _ => return Err("[freeze:contract][mir-json/invoke-operation-outside-profile]".into()),
    })
}
