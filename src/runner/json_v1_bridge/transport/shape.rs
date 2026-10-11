//! Strict wire primitives shared only by the data reader.
use crate::mir::{EffectMask, MirType};
use serde_json::Value;

pub(super) fn error(reason: &str) -> String {
    format!("[freeze:contract][json-v1/canonical-transport/{reason}]")
}
pub(super) fn fields(value: &Value, required: &[&str], optional: &[&str]) -> Result<(), String> {
    let object = value.as_object().ok_or_else(|| error("expected-object"))?;
    if required.iter().any(|key| !object.contains_key(*key))
        || object
            .keys()
            .any(|key| !required.contains(&key.as_str()) && !optional.contains(&key.as_str()))
    {
        return Err(error("missing-or-unknown-field"));
    }
    Ok(())
}
pub(super) fn string<'a>(value: &'a Value, key: &str) -> Result<&'a str, String> {
    value
        .get(key)
        .and_then(Value::as_str)
        .ok_or_else(|| error("expected-string"))
}
pub(super) fn nullable_string(value: &Value, key: &str) -> Result<(), String> {
    if value[key].is_null() || value[key].is_string() {
        Ok(())
    } else {
        Err(error("nullable-string"))
    }
}
pub(super) fn boolean(value: &Value, key: &str) -> Result<bool, String> {
    value
        .get(key)
        .and_then(Value::as_bool)
        .ok_or_else(|| error("expected-boolean"))
}
pub(super) fn array<'a>(value: &'a Value, key: &str) -> Result<&'a Vec<Value>, String> {
    value
        .get(key)
        .and_then(Value::as_array)
        .ok_or_else(|| error("expected-array"))
}
pub(super) fn id_value(value: &Value) -> Result<u32, String> {
    value
        .as_u64()
        .and_then(|id| u32::try_from(id).ok())
        .filter(|id| *id != u32::MAX)
        .ok_or_else(|| error("id-range"))
}
pub(super) fn id(value: &Value, key: &str) -> Result<u32, String> {
    id_value(&value[key])
}
pub(super) fn ty(value: &Value) -> Result<MirType, String> {
    match value.as_str() {
        Some("i64") => Ok(MirType::Integer),
        Some("i1") => Ok(MirType::Bool),
        Some("void") => Ok(MirType::Void),
        Some("unknown") => Ok(MirType::Unknown),
        _ => Err(error("type-outside-profile")),
    }
}
pub(super) fn effects(value: &Value) -> Result<EffectMask, String> {
    let bits = value["effects_bits"]
        .as_u64()
        .filter(|bits| *bits <= u16::MAX as u64)
        .ok_or_else(|| error("effects-bits"))?;
    Ok(EffectMask::from_bits(bits as u16))
}
pub(super) fn key(value: &Value) -> Result<(&str, &str, &str, u32), String> {
    fields(value, &["namespace", "owner", "name", "arity"], &[])?;
    let namespace = string(value, "namespace")?;
    let owner = string(value, "owner")?;
    let name = string(value, "name")?;
    if name.is_empty()
        || !matches!(
            namespace,
            "free_function" | "static_box_method" | "instance_box_method" | "birth_constructor"
        )
        || (namespace == "free_function") != owner.is_empty()
    {
        return Err(error("canonical-key"));
    }
    Ok((namespace, owner, name, id(value, "arity")?))
}

pub(super) fn layout(value: &Value) -> Result<crate::mir::edge_args::JumpArgsLayout, String> {
    match value.as_str() {
        Some("carriers_only") => Ok(crate::mir::edge_args::JumpArgsLayout::CarriersOnly),
        Some("expr_result_plus_carriers") => {
            Ok(crate::mir::edge_args::JumpArgsLayout::ExprResultPlusCarriers)
        }
        _ => Err(error("edge-layout")),
    }
}
pub(super) fn edge(value: &Value) -> Result<Option<crate::mir::edge_args::EdgeArgs>, String> {
    if value.is_null() {
        return Ok(None);
    }
    fields(value, &["layout", "values"], &[])?;
    Ok(Some(crate::mir::edge_args::EdgeArgs {
        layout: layout(&value["layout"])?,
        values: array(value, "values")?
            .iter()
            .map(|v| id_value(v).map(crate::mir::ValueId::new))
            .collect::<Result<_, _>>()?,
    }))
}
