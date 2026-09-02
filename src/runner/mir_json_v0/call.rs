use crate::mir::definitions::call_unified::{CalleeBoxKind, TypeCertainty};
use crate::mir::{Callee, Effect, EffectMask, MirInstruction, ValueId};
use hakorune_mir_defs::CanonicalGlobalTargetV1;
use serde_json::{Map, Value};
use std::fmt;

use super::catalog::JsonV0FunctionCatalog;

#[derive(Debug)]
enum JsonV0CallInput {
    Explicit(Callee),
    LegacyName(Box<str>),
    LegacyFunc(ValueId),
}

#[derive(Debug)]
struct JsonV0CallInputError(String);

impl From<String> for JsonV0CallInputError {
    fn from(message: String) -> Self {
        Self(message)
    }
}

impl fmt::Display for JsonV0CallInputError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl JsonV0CallInput {
    fn resolve(
        self,
        catalog: &JsonV0FunctionCatalog,
        args_len: usize,
    ) -> Result<Callee, JsonV0CallInputError> {
        match self {
            Self::Explicit(Callee::Constructor { .. }) => Err(
                "[freeze:contract][mir-json-v0/constructor-call-requires-newbox]"
                    .to_string()
                    .into(),
            ),
            Self::Explicit(callee) => Ok(callee),
            Self::LegacyName(name) => Ok(Callee::Global(
                project_legacy_global_target(&name, args_len).map_err(JsonV0CallInputError)?,
            )),
            Self::LegacyFunc(value_id) => {
                let target = catalog.resolve(value_id).map_err(JsonV0CallInputError)?;
                Ok(Callee::Global(
                    project_legacy_global_target(target, args_len).map_err(JsonV0CallInputError)?,
                ))
            }
        }
    }
}

fn parse_call_input(node: &Value) -> Result<JsonV0CallInput, JsonV0CallInputError> {
    let args_len = node
        .get("args")
        .and_then(Value::as_array)
        .map_or(0, Vec::len);
    if let Some(callee_obj) = node.get("callee") {
        return Ok(JsonV0CallInput::Explicit(parse_explicit_callee(
            callee_obj, args_len,
        )?));
    }

    if node.get("name").is_some() {
        if node.get("func").is_some() {
            return Err("call legacy name and func cannot both be present"
                .to_string()
                .into());
        }
        let name = required_nonempty_string(node, "name", "call legacy name")?;
        return Ok(JsonV0CallInput::LegacyName(name.into_boxed_str()));
    }

    if node.get("func").is_some() {
        return Ok(JsonV0CallInput::LegacyFunc(parse_value_id_field(
            node,
            "func",
            "call legacy func",
        )?));
    }

    Err("call missing target: expected callee, name, or func"
        .to_string()
        .into())
}

fn parse_explicit_callee(callee_obj: &Value, args_len: usize) -> Result<Callee, String> {
    let callee_obj = callee_obj
        .as_object()
        .ok_or_else(|| "call callee must be an object".to_string())?;
    let callee_type = callee_obj
        .get("type")
        .and_then(Value::as_str)
        .ok_or_else(|| "call callee missing type".to_string())?;
    match callee_type {
        "Global" => Ok(Callee::Global(project_legacy_global_target(
            &required_object_string(callee_obj, "name", "call callee Global")?,
            args_len,
        )?)),
        "Extern" => Ok(Callee::Extern(required_object_string(
            callee_obj,
            "name",
            "call callee Extern",
        )?)),
        "Method" => {
            let method = required_alias_string(callee_obj, "method", "name", "call callee Method")?;
            let box_name = match callee_obj.get("box_name") {
                None => "RuntimeDataBox".to_string(),
                Some(_) => required_object_string(callee_obj, "box_name", "call callee Method")?,
            };
            let receiver = callee_obj
                .get("receiver")
                .map(|value| parse_value_id_value(value, "call callee Method receiver"))
                .transpose()?;
            let certainty = if box_name == "RuntimeDataBox" {
                TypeCertainty::Union
            } else {
                TypeCertainty::Known
            };
            Ok(Callee::Method {
                box_name,
                method,
                receiver,
                certainty,
                box_kind: CalleeBoxKind::RuntimeData,
            })
        }
        "Constructor" => Ok(Callee::Constructor {
            box_type: required_alias_string(
                callee_obj,
                "box_type",
                "name",
                "call callee Constructor",
            )?,
        }),
        "Value" => Ok(Callee::Value(parse_alias_value_id(
            callee_obj,
            "value",
            "func",
            "call callee Value",
        )?)),
        other => Err(format!("unsupported call callee.type '{}'", other)),
    }
}

fn required_nonempty_string(node: &Value, key: &str, context: &str) -> Result<String, String> {
    let object = node
        .as_object()
        .ok_or_else(|| format!("{} must be an object", context))?;
    required_object_string(object, key, context)
}

fn required_object_string(
    object: &Map<String, Value>,
    key: &str,
    context: &str,
) -> Result<String, String> {
    let value = object
        .get(key)
        .ok_or_else(|| format!("{} missing {}", context, key))?;
    let text = value
        .as_str()
        .ok_or_else(|| format!("{} {} must be a string", context, key))?;
    if text.is_empty() {
        return Err(format!("{} {} must not be empty", context, key));
    }
    Ok(text.to_string())
}

fn required_alias_string(
    object: &Map<String, Value>,
    primary_key: &str,
    alias_key: &str,
    context: &str,
) -> Result<String, String> {
    match (object.get(primary_key), object.get(alias_key)) {
        (Some(primary), Some(alias)) => {
            let primary_text = primary
                .as_str()
                .ok_or_else(|| format!("{} {} must be a string", context, primary_key))?;
            let alias_text = alias
                .as_str()
                .ok_or_else(|| format!("{} {} must be a string", context, alias_key))?;
            if primary_text != alias_text {
                return Err(format!(
                    "{} {} and {} conflict",
                    context, primary_key, alias_key
                ));
            }
            if primary_text.is_empty() {
                return Err(format!("{} {} must not be empty", context, primary_key));
            }
            Ok(primary_text.to_string())
        }
        (Some(value), None) => {
            let text = value
                .as_str()
                .ok_or_else(|| format!("{} {} must be a string", context, primary_key))?;
            if text.is_empty() {
                return Err(format!("{} {} must not be empty", context, primary_key));
            }
            Ok(text.to_string())
        }
        (None, Some(value)) => {
            let text = value
                .as_str()
                .ok_or_else(|| format!("{} {} must be a string", context, alias_key))?;
            if text.is_empty() {
                return Err(format!("{} {} must not be empty", context, alias_key));
            }
            Ok(text.to_string())
        }
        (None, None) => Err(format!("{} missing {}/{}", context, primary_key, alias_key)),
    }
}

fn parse_alias_value_id(
    object: &Map<String, Value>,
    primary_key: &str,
    alias_key: &str,
    context: &str,
) -> Result<ValueId, String> {
    match (object.get(primary_key), object.get(alias_key)) {
        (Some(primary), Some(alias)) => {
            let primary_id =
                parse_value_id_value(primary, &format!("{} {}", context, primary_key))?;
            let alias_id = parse_value_id_value(alias, &format!("{} {}", context, alias_key))?;
            if primary_id != alias_id {
                return Err(format!(
                    "{} {} and {} conflict",
                    context, primary_key, alias_key
                ));
            }
            Ok(primary_id)
        }
        (Some(value), None) => parse_value_id_value(value, &format!("{} {}", context, primary_key)),
        (None, Some(value)) => parse_value_id_value(value, &format!("{} {}", context, alias_key)),
        (None, None) => Err(format!("{} missing {}/{}", context, primary_key, alias_key)),
    }
}

fn parse_value_id_field(node: &Value, key: &str, context: &str) -> Result<ValueId, String> {
    let value = node
        .get(key)
        .ok_or_else(|| format!("{} missing {}", context, key))?;
    parse_value_id_value(value, &format!("{} {}", context, key))
}

fn parse_value_id_value(value: &Value, context: &str) -> Result<ValueId, String> {
    let raw = value
        .as_u64()
        .ok_or_else(|| format!("{} must be an integer value id", context))?;
    let id = u32::try_from(raw).map_err(|_| format!("{} is out of range: {}", context, raw))?;
    let value_id = ValueId::new(id);
    if value_id == ValueId::INVALID {
        return Err(format!("{} cannot be ValueId::INVALID", context));
    }
    Ok(value_id)
}

pub(super) fn build_call_instruction(
    inst: &Value,
    call_node: &Value,
    op_label: &str,
    catalog: &JsonV0FunctionCatalog,
) -> Result<(MirInstruction, Option<ValueId>), String> {
    let input = parse_call_input(call_node).map_err(|error| error.to_string())?;
    let dst_opt = inst
        .get("dst")
        .or_else(|| call_node.get("dst"))
        .and_then(Value::as_u64)
        .map(|v| ValueId::new(v as u32));
    let arg_ctx = format!("{} arg", op_label);
    let args = super::helpers::parse_value_id_array(call_node, "args", &arg_ctx)?;
    let callee = input
        .resolve(catalog, args.len())
        .map_err(|error| error.to_string())?;
    let effects = parse_call_effects(call_node)?;
    Ok((
        // JSON-v0 is an explicit compatibility ingress.  Preserve its
        // pre-R6 carrier until the R7 quarantine instead of silently
        // reclassifying the wire input as canonical MIR.
        MirInstruction::LegacyCallV0 {
            dst: dst_opt,
            func: ValueId::INVALID,
            callee: Some(callee),
            args,
            effects,
        },
        dst_opt,
    ))
}

/// JSON-v0 is an owner-local compatibility ingress.  Once the wire spelling
/// has been accepted, project it into the structural carrier exactly once;
/// this helper performs no catalog lookup, retry, or target repair.
fn project_legacy_global_target(
    name: &str,
    args_len: usize,
) -> Result<CanonicalGlobalTargetV1, String> {
    if name.is_empty() {
        return Err("call Global name must not be empty".to_string());
    }
    if name == "print" {
        return Ok(CanonicalGlobalTargetV1::builtin_print());
    }
    let (base, arity) = match name.rsplit_once('/') {
        Some((base, encoded)) => (
            base,
            encoded
                .parse::<u32>()
                .map_err(|_| format!("call Global name has malformed arity: {name}"))?,
        ),
        None => (name, args_len as u32),
    };
    if let Some((owner, method)) = base.rsplit_once('.') {
        return CanonicalGlobalTargetV1::new_static_box_method(owner.into(), method.into(), arity)
            .map_err(|error| format!("call Global target is invalid: {error:?}"));
    }
    CanonicalGlobalTargetV1::new_free_function(base.into(), arity)
        .map_err(|error| format!("call Global target is invalid: {error:?}"))
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
