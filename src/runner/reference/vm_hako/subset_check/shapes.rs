use serde_json::Value;

use super::ensure_u64_fields;

pub(super) fn validate_debug_shape(inst: &Value) -> Result<(), String> {
    ensure_u64_fields(inst, &[("value", "debug(missing-value)")])?;
    if inst.get("message").and_then(|v| v.as_str()).is_none() {
        return Err("debug(missing-message)".to_string());
    }
    Ok(())
}

pub(super) fn validate_select_shape(inst: &Value) -> Result<(), String> {
    ensure_u64_fields(
        inst,
        &[
            ("dst", "select(missing-dst)"),
            ("cond", "select(missing-cond)"),
            ("then_val", "select(missing-then-val)"),
            ("else_val", "select(missing-else-val)"),
        ],
    )?;
    Ok(())
}

pub(super) fn validate_load_shape(inst: &Value) -> Result<(), String> {
    ensure_u64_fields(
        inst,
        &[("dst", "load(missing-dst)"), ("ptr", "load(missing-ptr)")],
    )?;
    Ok(())
}

pub(super) fn validate_array_get_shape(inst: &Value) -> Result<(), String> {
    ensure_u64_fields(
        inst,
        &[
            ("dst", "array_get(missing-dst)"),
            ("array", "array_get(missing-array)"),
            ("index", "array_get(missing-index)"),
        ],
    )?;
    Ok(())
}

pub(super) fn validate_array_set_shape(inst: &Value) -> Result<(), String> {
    ensure_u64_fields(
        inst,
        &[
            ("array", "array_set(missing-array)"),
            ("index", "array_set(missing-index)"),
            ("value", "array_set(missing-value)"),
        ],
    )?;
    Ok(())
}

pub(super) fn validate_store_shape(inst: &Value) -> Result<(), String> {
    ensure_u64_fields(
        inst,
        &[("ptr", "store(missing-ptr)"), ("value", "store(missing-value)")],
    )?;
    Ok(())
}

pub(super) fn validate_phi_shape(inst: &Value) -> Result<(), String> {
    ensure_u64_fields(inst, &[("dst", "phi(missing-dst)")])?;
    let incoming = inst
        .get("incoming")
        .and_then(|v| v.as_array())
        .ok_or_else(|| "phi(missing-incoming)".to_string())?;
    if incoming.is_empty() {
        return Err("phi(empty-incoming)".to_string());
    }
    for entry in incoming {
        let pair = entry
            .as_array()
            .ok_or_else(|| "phi(entry-not-array)".to_string())?;
        if pair.len() != 2 {
            return Err("phi(entry-len!=2)".to_string());
        }
        if pair[0].as_u64().is_none() {
            return Err("phi(value-non-reg)".to_string());
        }
        if pair[1].as_u64().is_none() {
            return Err("phi(block-non-int)".to_string());
        }
    }
    Ok(())
}

pub(super) fn validate_typeop_shape(inst: &Value) -> Result<(), String> {
    ensure_u64_fields(inst, &[("dst", "typeop(missing-dst)")])?;
    if inst
        .get("src")
        .or_else(|| inst.get("value"))
        .and_then(|v| v.as_u64())
        .is_none()
    {
        return Err("typeop(missing-src)".to_string());
    }

    let op_raw = inst
        .get("operation")
        .or_else(|| inst.get("op_kind"))
        .and_then(|v| v.as_str())
        .ok_or_else(|| "typeop(missing-operation)".to_string())?;
    let op = super::super::shape_contract::canonical_typeop_operation(op_raw);
    if op != "check" && op != "cast" {
        return Err(format!("typeop(operation:{})", op_raw));
    }

    let ty = inst
        .get("target_type")
        .or_else(|| inst.get("ty"))
        .and_then(|v| v.as_str())
        .ok_or_else(|| "typeop(missing-target-type)".to_string())?;
    if ty.is_empty() {
        return Err("typeop(empty-target-type)".to_string());
    }
    Ok(())
}

pub(super) fn validate_weak_new_shape(inst: &Value) -> Result<(), String> {
    ensure_u64_fields(
        inst,
        &[
            ("dst", "weak_new(missing-dst)"),
            ("box_val", "weak_new(missing-box-val)"),
        ],
    )?;
    Ok(())
}

pub(super) fn validate_weak_load_shape(inst: &Value) -> Result<(), String> {
    ensure_u64_fields(
        inst,
        &[
            ("dst", "weak_load(missing-dst)"),
            ("weak_ref", "weak_load(missing-weak-ref)"),
        ],
    )?;
    Ok(())
}

pub(super) fn validate_ref_new_shape(inst: &Value) -> Result<(), String> {
    ensure_u64_fields(
        inst,
        &[
            ("dst", "ref_new(missing-dst)"),
            ("box_val", "ref_new(missing-box-val)"),
        ],
    )?;
    Ok(())
}

pub(super) fn validate_future_new_shape(inst: &Value) -> Result<(), String> {
    ensure_u64_fields(
        inst,
        &[
            ("dst", "future_new(missing-dst)"),
            ("value", "future_new(missing-value)"),
        ],
    )?;
    Ok(())
}

pub(super) fn validate_future_set_shape(inst: &Value) -> Result<(), String> {
    ensure_u64_fields(
        inst,
        &[
            ("future", "future_set(missing-future)"),
            ("value", "future_set(missing-value)"),
        ],
    )?;
    Ok(())
}

pub(super) fn validate_await_shape(inst: &Value) -> Result<(), String> {
    ensure_u64_fields(
        inst,
        &[("dst", "await(missing-dst)"), ("future", "await(missing-future)")],
    )?;
    Ok(())
}

pub(super) fn validate_const_shape(inst: &Value) -> Result<(), String> {
    let value = inst
        .get("value")
        .ok_or_else(|| "const(malformed)".to_string())?;
    let ty = value
        .get("type")
        .ok_or_else(|| "const(malformed)".to_string())?;

    let is_i64_or_bool = ty.as_str().map(|s| s == "i64" || s == "bool").unwrap_or(false);
    let is_void = ty.as_str().map(|s| s == "void").unwrap_or(false);
    let is_string_handle = ty
        .as_object()
        .map(|obj| {
            obj.get("box_type").and_then(|v| v.as_str()) == Some("StringBox")
                && obj.get("kind").and_then(|v| v.as_str()) == Some("handle")
        })
        .unwrap_or(false);

    if !is_i64_or_bool && !is_void && !is_string_handle {
        let label = if let Some(s) = ty.as_str() {
            s.to_string()
        } else {
            "<object>".to_string()
        };
        return Err(format!("const(non-i64-bool-void-handle:{})", label));
    }

    Ok(())
}
