use serde_json::Value;

pub(super) fn validate_env_get_externcall_shape(inst: &Value) -> Result<(), String> {
    let args = inst
        .get("args")
        .and_then(|v| v.as_array())
        .ok_or_else(|| "externcall(env.get:malformed)".to_string())?;
    if args.len() != 1 {
        return Err("externcall(env.get:args!=1)".to_string());
    }
    if args.first().and_then(|v| v.as_u64()).is_none() {
        return Err("externcall(env.get:arg0:non-reg)".to_string());
    }
    if let Some(dst) = inst.get("dst") {
        if !dst.is_null() && dst.as_u64().is_none() {
            return Err("externcall(env.get:dst:non-reg)".to_string());
        }
    }
    Ok(())
}

pub(super) fn validate_mirbuilder_emit_externcall_shape(inst: &Value) -> Result<(), String> {
    let args = inst
        .get("args")
        .and_then(|v| v.as_array())
        .ok_or_else(|| "externcall(env.mirbuilder.emit:malformed)".to_string())?;
    if args.len() != 1 {
        return Err("externcall(env.mirbuilder.emit:args!=1)".to_string());
    }
    if args.first().and_then(|v| v.as_u64()).is_none() {
        return Err("externcall(env.mirbuilder.emit:arg0:non-reg)".to_string());
    }
    if let Some(dst) = inst.get("dst") {
        if !dst.is_null() && dst.as_u64().is_none() {
            return Err("externcall(env.mirbuilder.emit:dst:non-reg)".to_string());
        }
    }
    Ok(())
}

pub(super) fn validate_single_arg_externcall_shape(inst: &Value, label: &str) -> Result<(), String> {
    let args = inst
        .get("args")
        .and_then(|v| v.as_array())
        .ok_or_else(|| format!("externcall({}:malformed)", label))?;
    if args.len() != 1 {
        return Err(format!("externcall({}:args!=1)", label));
    }
    if args.first().and_then(|v| v.as_u64()).is_none() {
        return Err(format!("externcall({}:arg0:non-reg)", label));
    }
    if let Some(dst) = inst.get("dst") {
        if !(dst.is_u64() || dst.is_null()) {
            return Err(format!("externcall({}:dst:non-reg)", label));
        }
    }
    Ok(())
}

pub(super) fn validate_two_arg_externcall_shape(inst: &Value, label: &str) -> Result<(), String> {
    let args = inst
        .get("args")
        .and_then(|v| v.as_array())
        .ok_or_else(|| format!("externcall({}:malformed)", label))?;
    if args.len() != 2 {
        return Err(format!("externcall({}:args!=2)", label));
    }
    if args.iter().any(|v| v.as_u64().is_none()) {
        return Err(format!("externcall({}:args:non-reg)", label));
    }
    if let Some(dst) = inst.get("dst") {
        if !(dst.is_u64() || dst.is_null()) {
            return Err(format!("externcall({}:dst:non-reg)", label));
        }
    }
    Ok(())
}
