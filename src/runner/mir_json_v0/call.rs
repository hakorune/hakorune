use crate::mir::definitions::call_unified::{CalleeBoxKind, TypeCertainty};
use crate::mir::{Callee, Effect, EffectMask, MirInstruction, ValueId};
use serde_json::Value;

pub(super) fn parse_call_callee(inst: &Value) -> Result<Option<Callee>, String> {
    let callee_obj = match inst.get("callee") {
        Some(obj) => obj,
        None => return Ok(None),
    };
    let callee_type = callee_obj
        .get("type")
        .and_then(Value::as_str)
        .ok_or_else(|| "call callee missing type".to_string())?;
    match callee_type {
        "Global" => {
            let name = callee_obj
                .get("name")
                .and_then(Value::as_str)
                .ok_or_else(|| "call callee Global missing name".to_string())?
                .to_string();
            Ok(Some(Callee::Global(name)))
        }
        "Extern" => {
            let name = callee_obj
                .get("name")
                .and_then(Value::as_str)
                .ok_or_else(|| "call callee Extern missing name".to_string())?
                .to_string();
            Ok(Some(Callee::Extern(name)))
        }
        "Method" => {
            let method = callee_obj
                .get("method")
                .or_else(|| callee_obj.get("name"))
                .and_then(Value::as_str)
                .ok_or_else(|| "call callee Method missing method/name".to_string())?
                .to_string();
            let box_name = callee_obj
                .get("box_name")
                .and_then(Value::as_str)
                .unwrap_or("RuntimeDataBox")
                .to_string();
            let receiver = callee_obj
                .get("receiver")
                .and_then(Value::as_u64)
                .map(|v| ValueId::new(v as u32));
            let certainty = if box_name == "RuntimeDataBox" {
                TypeCertainty::Union
            } else {
                TypeCertainty::Known
            };
            Ok(Some(Callee::Method {
                box_name,
                method,
                receiver,
                certainty,
                box_kind: CalleeBoxKind::RuntimeData,
            }))
        }
        "Constructor" => {
            let box_type = callee_obj
                .get("box_type")
                .or_else(|| callee_obj.get("name"))
                .and_then(Value::as_str)
                .ok_or_else(|| "call callee Constructor missing box_type/name".to_string())?
                .to_string();
            Ok(Some(Callee::Constructor { box_type }))
        }
        "Value" => {
            let value_id = callee_obj
                .get("value")
                .or_else(|| callee_obj.get("func"))
                .and_then(Value::as_u64)
                .ok_or_else(|| "call callee Value missing value/func".to_string())?
                as u32;
            Ok(Some(Callee::Value(ValueId::new(value_id))))
        }
        other => Err(format!("unsupported call callee.type '{}'", other)),
    }
}

pub(super) fn build_call_instruction(
    inst: &Value,
    call_node: &Value,
    op_label: &str,
) -> Result<(MirInstruction, Option<ValueId>), String> {
    let callee = parse_call_callee(call_node)?;
    let func = if callee.is_some() {
        call_node
            .get("func")
            .and_then(Value::as_u64)
            .map(|v| ValueId::new(v as u32))
            .unwrap_or(ValueId::INVALID)
    } else {
        let ctx = format!("{} func", op_label);
        ValueId::new(super::helpers::require_u64(call_node, "func", &ctx)? as u32)
    };

    let dst_opt = inst
        .get("dst")
        .or_else(|| call_node.get("dst"))
        .and_then(Value::as_u64)
        .map(|v| ValueId::new(v as u32));
    let arg_ctx = format!("{} arg", op_label);
    let args = super::helpers::parse_value_id_array(call_node, "args", &arg_ctx)?;
    let effects = parse_call_effects(call_node)?;
    Ok((
        MirInstruction::Call {
            dst: dst_opt,
            func,
            callee,
            args,
            effects,
        },
        dst_opt,
    ))
}

fn parse_call_effects(node: &Value) -> Result<EffectMask, String> {
    let effects_v = match node.get("effects") {
        None => return Ok(EffectMask::READ),
        Some(v) => v,
    };
    if effects_v.is_null() {
        return Ok(EffectMask::READ);
    }

    if let Some(bits) = effects_v.as_u64() {
        if bits <= u16::MAX as u64 {
            let mask = EffectMask::from_bits(bits as u16);
            return Ok(if mask.bits() == 0 {
                EffectMask::READ
            } else {
                mask
            });
        }
        return Err(format!("call effects bits out of range: {}", bits));
    }

    let Some(arr) = effects_v.as_array() else {
        return Ok(EffectMask::READ);
    };
    if arr.is_empty() {
        return Ok(EffectMask::READ);
    }

    let mut mask = EffectMask::new();
    let mut parsed_any = false;
    for item in arr {
        if let Some(bits) = item.as_u64() {
            if bits <= u16::MAX as u64 {
                mask = mask.union(EffectMask::from_bits(bits as u16));
                parsed_any = true;
            }
            continue;
        }
        let Some(raw) = item.as_str() else {
            continue;
        };
        if let Some(mapped) = map_effect_name(raw) {
            mask = mask.union(mapped);
            parsed_any = true;
        }
    }

    if parsed_any {
        Ok(mask)
    } else {
        Ok(EffectMask::READ)
    }
}

fn map_effect_name(raw: &str) -> Option<EffectMask> {
    let lower = raw.to_ascii_lowercase();
    Some(match lower.as_str() {
        "pure" => EffectMask::PURE,
        "mut" => EffectMask::MUT,
        "io" => EffectMask::IO,
        "control" => EffectMask::CONTROL,
        "read" | "read_heap" => EffectMask::READ,
        "write" | "write_heap" => EffectMask::WRITE,
        "panic" => EffectMask::PANIC,
        "p2p" => EffectMask::P2P,
        "ffi" => EffectMask::from_bits(Effect::FFI as u16),
        "alloc" => EffectMask::from_bits(Effect::Alloc as u16),
        "global" => EffectMask::from_bits(Effect::Global as u16),
        "async" => EffectMask::from_bits(Effect::Async as u16),
        "unsafe" => EffectMask::from_bits(Effect::Unsafe as u16),
        "debug" => EffectMask::from_bits(Effect::Debug as u16),
        "barrier" => EffectMask::from_bits(Effect::Barrier as u16),
        _ => return None,
    })
}
