use serde_json::Value;

use super::ensure_u64_fields;

fn validate_boxcall_birth_shape(args: &[Value]) -> Result<(), String> {
    if !args.is_empty() {
        return Err("boxcall(birth:args>0)".to_string());
    }
    Ok(())
}

fn validate_boxcall_push_shape(inst: &Value, args: &[Value]) -> Result<(), String> {
    if args.len() != 1 {
        return Err("boxcall(push:args!=1)".to_string());
    }
    if args.first().and_then(|v| v.as_u64()).is_none() {
        return Err("boxcall(push:arg0:non-reg)".to_string());
    }
    ensure_u64_fields(inst, &[("box", "boxcall(missing-box)")])?;
    Ok(())
}

fn validate_boxcall_open_shape(inst: &Value, args: &[Value]) -> Result<(), String> {
    if args.len() != 2 && args.len() != 3 {
        return Err("boxcall(open:args!=2or3)".to_string());
    }
    let args_ok = if args.len() == 2 {
        args.first().and_then(|v| v.as_u64()).is_some()
            && args.get(1).and_then(|v| v.as_u64()).is_some()
    } else {
        args.first().and_then(|v| v.as_u64()).is_some()
            && args.get(1).and_then(|v| v.as_u64()).is_some()
            && args.get(2).and_then(|v| v.as_u64()).is_some()
    };
    if !args_ok {
        return Err("boxcall(open:args:non-reg)".to_string());
    }
    ensure_u64_fields(
        inst,
        &[
            ("dst", "boxcall(missing-dst)"),
            ("box", "boxcall(missing-box)"),
        ],
    )?;
    Ok(())
}

fn validate_boxcall_substring_shape(inst: &Value, args: &[Value]) -> Result<(), String> {
    if args.len() != 2 {
        return Err("boxcall(substring:args!=2)".to_string());
    }
    let args_ok = args.first().and_then(|v| v.as_u64()).is_some()
        && args.get(1).and_then(|v| v.as_u64()).is_some();
    if !args_ok {
        return Err("boxcall(substring:args:non-reg)".to_string());
    }
    ensure_u64_fields(
        inst,
        &[
            ("dst", "boxcall(missing-dst)"),
            ("box", "boxcall(missing-box)"),
        ],
    )?;
    Ok(())
}

fn validate_boxcall_set_shape(inst: &Value, args: &[Value], method: &str) -> Result<(), String> {
    if args.len() != 2 {
        return Err(format!("boxcall({}:args!=2)", method));
    }
    let args_ok = args.iter().all(|v| v.as_u64().is_some());
    if !args_ok {
        return Err(format!("boxcall({}:args:non-reg)", method));
    }
    ensure_u64_fields(
        inst,
        &[
            ("dst", "boxcall(missing-dst)"),
            ("box", "boxcall(missing-box)"),
        ],
    )?;
    Ok(())
}

fn validate_boxcall_link_exe_shape(inst: &Value, args: &[Value]) -> Result<(), String> {
    if args.len() != 3 {
        return Err("boxcall(link_exe:args!=3)".to_string());
    }
    let args_ok = args.iter().all(|v| v.as_u64().is_some());
    if !args_ok {
        return Err("boxcall(link_exe:args:non-reg)".to_string());
    }
    ensure_u64_fields(
        inst,
        &[
            ("dst", "boxcall(missing-dst)"),
            ("box", "boxcall(missing-box)"),
        ],
    )?;
    Ok(())
}

pub(super) fn validate_mir_call_shape(inst: &Value) -> Result<(), String> {
    let callee_type = super::call_callee_type(inst).unwrap_or("");
    let callee_name = super::call_callee_name(inst).unwrap_or("");
    if callee_type.is_empty() {
        return Err("mir_call(missing-callee-type)".to_string());
    }
    if callee_name.is_empty() {
        return Err("mir_call(missing-callee-name)".to_string());
    }

    let args = inst
        .get("mir_call")
        .and_then(|v| v.get("args"))
        .or_else(|| inst.get("args"))
        .and_then(|v| v.as_array())
        .ok_or_else(|| "mir_call(missing-args)".to_string())?;
    if args.iter().any(|v| v.as_u64().is_none()) {
        return Err("mir_call(args:non-reg)".to_string());
    }

    Ok(())
}

fn validate_boxcall_noarg_shape(inst: &Value, args: &[Value], method: &str) -> Result<(), String> {
    if !args.is_empty() {
        return Err(format!("boxcall({}:args!=0)", method));
    }
    ensure_u64_fields(
        inst,
        &[
            ("dst", "boxcall(missing-dst)"),
            ("box", "boxcall(missing-box)"),
        ],
    )?;
    Ok(())
}

fn validate_boxcall_zero_or_one_reg_shape(
    inst: &Value,
    args: &[Value],
    method: &str,
) -> Result<(), String> {
    if args.len() > 1 {
        return Err(format!("boxcall({}:args>1)", method));
    }
    if args.len() == 1 && args.first().and_then(|v| v.as_u64()).is_none() {
        return Err(format!("boxcall({}:arg0:non-reg)", method));
    }
    ensure_u64_fields(
        inst,
        &[
            ("dst", "boxcall(missing-dst)"),
            ("box", "boxcall(missing-box)"),
        ],
    )?;
    Ok(())
}

fn validate_boxcall_two_reg_shape(
    inst: &Value,
    args: &[Value],
    method: &str,
) -> Result<(), String> {
    if args.len() != 2 {
        return Err(format!("boxcall({}:args!=2)", method));
    }
    if args.iter().any(|v| v.as_u64().is_none()) {
        return Err(format!("boxcall({}:args:non-reg)", method));
    }
    ensure_u64_fields(
        inst,
        &[
            ("dst", "boxcall(missing-dst)"),
            ("box", "boxcall(missing-box)"),
        ],
    )?;
    Ok(())
}

const BOXCALL_INDEXOF_ARGS_TAG: &str = "boxcall(indexOf:args!=1or2)";
const BOXCALL_INDEXOF_ARG0_NON_REG_TAG: &str = "boxcall(indexOf:arg0:non-reg)";
const BOXCALL_INDEXOF_ARG1_NON_REG_TAG: &str = "boxcall(indexOf:arg1:non-reg)";

fn validate_boxcall_indexof_shape(inst: &Value, args: &[Value]) -> Result<(), String> {
    if args.is_empty() || args.len() > 2 {
        return Err(BOXCALL_INDEXOF_ARGS_TAG.to_string());
    }
    if args.first().and_then(|v| v.as_u64()).is_none() {
        return Err(BOXCALL_INDEXOF_ARG0_NON_REG_TAG.to_string());
    }
    if args.len() == 2 && args.get(1).and_then(|v| v.as_u64()).is_none() {
        return Err(BOXCALL_INDEXOF_ARG1_NON_REG_TAG.to_string());
    }
    ensure_u64_fields(
        inst,
        &[
            ("dst", "boxcall(missing-dst)"),
            ("box", "boxcall(missing-box)"),
        ],
    )?;
    Ok(())
}

fn validate_boxcall_default_shape(inst: &Value, args: &[Value]) -> Result<(), String> {
    let method = inst.get("method").and_then(|v| v.as_str()).unwrap_or("");
    if args.len() > 1 {
        return Err(format!("boxcall({}:args>1)", method));
    }
    if args.len() == 1 && args.first().and_then(|v| v.as_u64()).is_none() {
        return Err(format!("boxcall({}:arg0:non-reg)", method));
    }
    ensure_u64_fields(
        inst,
        &[
            ("dst", "boxcall(missing-dst)"),
            ("box", "boxcall(missing-box)"),
        ],
    )?;
    Ok(())
}

pub(super) fn validate_boxcall_shape(inst: &Value) -> Result<(), String> {
    let method = inst.get("method").and_then(|v| v.as_str()).unwrap_or("");
    let args = inst
        .get("args")
        .and_then(|v| v.as_array())
        .ok_or_else(|| "boxcall(malformed)".to_string())?;
    match method {
        "birth" => validate_boxcall_birth_shape(args),
        "push" => validate_boxcall_push_shape(inst, args),
        "open" => validate_boxcall_open_shape(inst, args),
        "link_exe" => validate_boxcall_link_exe_shape(inst, args),
        "commit_bytes_i64" | "decommit_bytes_i64" => {
            validate_boxcall_two_reg_shape(inst, args, method)
        }
        "set" | "setField" => validate_boxcall_set_shape(inst, args, method),
        "read" | "close" => validate_boxcall_zero_or_one_reg_shape(inst, args, method),
        "length" => validate_boxcall_noarg_shape(inst, args, method),
        "indexOf" => validate_boxcall_indexof_shape(inst, args),
        "substring" => validate_boxcall_substring_shape(inst, args),
        _ => validate_boxcall_default_shape(inst, args),
    }
}
