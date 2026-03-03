use serde_json::Value;
use std::collections::HashMap;

use super::shape_contract::{
    canonical_barrier_kind, canonical_binop_operation, canonical_typeop_operation,
    canonical_unop_operation, collect_function_return_models, has_id_method_arg1_model,
    normalize_aliases_in_root, parse_print_arg_from_instruction,
    update_handle_bindings_from_const_or_copy, validate_two_arg_call_target,
    vm_hako_supported_compare_operation,
};
fn ensure_u64_field(inst: &Value, key: &str, err_tag: &'static str) -> Result<(), String> {
    if inst.get(key).and_then(|v| v.as_u64()).is_none() {
        return Err(err_tag.to_string());
    }
    Ok(())
}

fn ensure_u64_fields(inst: &Value, fields: &[(&str, &'static str)]) -> Result<(), String> {
    for (key, tag) in fields {
        ensure_u64_field(inst, key, tag)?;
    }
    Ok(())
}

fn validate_debug_shape(inst: &Value) -> Result<(), String> {
    ensure_u64_fields(inst, &[("value", "debug(missing-value)")])?;
    if inst.get("message").and_then(|v| v.as_str()).is_none() {
        return Err("debug(missing-message)".to_string());
    }
    Ok(())
}

fn validate_select_shape(inst: &Value) -> Result<(), String> {
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

fn validate_load_shape(inst: &Value) -> Result<(), String> {
    ensure_u64_fields(
        inst,
        &[("dst", "load(missing-dst)"), ("ptr", "load(missing-ptr)")],
    )?;
    Ok(())
}

fn validate_array_get_shape(inst: &Value) -> Result<(), String> {
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

fn validate_array_set_shape(inst: &Value) -> Result<(), String> {
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

fn validate_store_shape(inst: &Value) -> Result<(), String> {
    ensure_u64_fields(
        inst,
        &[
            ("ptr", "store(missing-ptr)"),
            ("value", "store(missing-value)"),
        ],
    )?;
    Ok(())
}

fn validate_phi_shape(inst: &Value) -> Result<(), String> {
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

fn validate_typeop_shape(inst: &Value) -> Result<(), String> {
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
    let op = canonical_typeop_operation(op_raw);
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

fn validate_weak_new_shape(inst: &Value) -> Result<(), String> {
    ensure_u64_fields(
        inst,
        &[
            ("dst", "weak_new(missing-dst)"),
            ("box_val", "weak_new(missing-box-val)"),
        ],
    )?;
    Ok(())
}

fn validate_weak_load_shape(inst: &Value) -> Result<(), String> {
    ensure_u64_fields(
        inst,
        &[
            ("dst", "weak_load(missing-dst)"),
            ("weak_ref", "weak_load(missing-weak-ref)"),
        ],
    )?;
    Ok(())
}

fn validate_ref_new_shape(inst: &Value) -> Result<(), String> {
    ensure_u64_fields(
        inst,
        &[
            ("dst", "ref_new(missing-dst)"),
            ("box_val", "ref_new(missing-box-val)"),
        ],
    )?;
    Ok(())
}

fn validate_future_new_shape(inst: &Value) -> Result<(), String> {
    ensure_u64_fields(
        inst,
        &[
            ("dst", "future_new(missing-dst)"),
            ("value", "future_new(missing-value)"),
        ],
    )?;
    Ok(())
}

fn validate_future_set_shape(inst: &Value) -> Result<(), String> {
    ensure_u64_fields(
        inst,
        &[
            ("future", "future_set(missing-future)"),
            ("value", "future_set(missing-value)"),
        ],
    )?;
    Ok(())
}

fn validate_await_shape(inst: &Value) -> Result<(), String> {
    ensure_u64_fields(
        inst,
        &[
            ("dst", "await(missing-dst)"),
            ("future", "await(missing-future)"),
        ],
    )?;
    Ok(())
}

fn validate_const_shape(inst: &Value) -> Result<(), String> {
    let value = inst
        .get("value")
        .ok_or_else(|| "const(malformed)".to_string())?;
    let ty = value
        .get("type")
        .ok_or_else(|| "const(malformed)".to_string())?;

    let is_i64_or_bool = ty
        .as_str()
        .map(|s| s == "i64" || s == "bool")
        .unwrap_or(false);
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
    ensure_u64_field(inst, "box", "boxcall(missing-box)")?;
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

fn validate_boxcall_indexof_shape(inst: &Value, args: &[Value]) -> Result<(), String> {
    if args.is_empty() || args.len() > 2 {
        return Err("boxcall(indexOf:args!=1or2)".to_string());
    }
    if args.first().and_then(|v| v.as_u64()).is_none() {
        return Err("boxcall(indexOf:arg0:non-reg)".to_string());
    }
    if args.len() == 2 && args.get(1).and_then(|v| v.as_u64()).is_none() {
        return Err("boxcall(indexOf:arg1:non-reg)".to_string());
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
    if args.len() > 1 {
        return Err("boxcall(args>1)".to_string());
    }
    if args.len() == 1 && args.first().and_then(|v| v.as_u64()).is_none() {
        return Err("boxcall(arg0:non-reg)".to_string());
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

// SSOT: docs/development/current/main/phases/phase-29y/82-VM-HAKO-BOXCALL-CONTRACT-SSOT.md
fn validate_boxcall_shape(inst: &Value) -> Result<(), String> {
    let method = inst.get("method").and_then(|v| v.as_str()).unwrap_or("");
    let args = inst
        .get("args")
        .and_then(|v| v.as_array())
        .ok_or_else(|| "boxcall(malformed)".to_string())?;
    match method {
        "birth" => validate_boxcall_birth_shape(args),
        "push" => validate_boxcall_push_shape(inst, args),
        "open" => validate_boxcall_open_shape(inst, args),
        "read" | "close" => validate_boxcall_zero_or_one_reg_shape(inst, args, method),
        "length" => validate_boxcall_noarg_shape(inst, args, method),
        "indexOf" => validate_boxcall_indexof_shape(inst, args),
        "substring" => validate_boxcall_substring_shape(inst, args),
        _ => validate_boxcall_default_shape(inst, args),
    }
}

fn validate_env_get_externcall_shape(inst: &Value) -> Result<(), String> {
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

pub(super) fn check_vm_hako_subset_json(json_text: &str) -> Result<(), (String, u32, String)> {
    let mut root: Value = serde_json::from_str(json_text)
        .map_err(|_| ("<json>".to_string(), 0, "InvalidJson".to_string()))?;
    normalize_aliases_in_root(&mut root);
    let functions = root
        .get("functions")
        .and_then(|v| v.as_array())
        .ok_or_else(|| ("<json>".to_string(), 0, "MissingFunctions".to_string()))?;
    let return_models = collect_function_return_models(functions);
    let allow_dynamic_method_arg1 = has_id_method_arg1_model(&return_models);

    let main_func = functions
        .iter()
        .find(|f| f.get("name").and_then(|n| n.as_str()) == Some("main"))
        .or_else(|| functions.first())
        .ok_or_else(|| ("<json>".to_string(), 0, "EmptyFunctions".to_string()))?;

    let func_name = main_func
        .get("name")
        .and_then(|n| n.as_str())
        .unwrap_or("main")
        .to_string();

    let blocks = main_func
        .get("blocks")
        .and_then(|v| v.as_array())
        .ok_or_else(|| (func_name.clone(), 0, "MissingBlocks".to_string()))?;

    for block in blocks {
        let bb = block
            .get("id")
            .and_then(|v| v.as_u64())
            .map(|v| v as u32)
            .unwrap_or(0);
        let insts = block
            .get("instructions")
            .and_then(|v| v.as_array())
            .ok_or_else(|| (func_name.clone(), bb, "MissingInstructions".to_string()))?;
        let mut handle_by_reg: HashMap<u64, String> = HashMap::new();

        for inst in insts {
            let op = inst.get("op").and_then(|v| v.as_str()).unwrap_or("");
            match op {
                "const" => {
                    if let Err(reason) = validate_const_shape(inst) {
                        return Err((func_name.clone(), bb, reason));
                    }
                    update_handle_bindings_from_const_or_copy(inst, &mut handle_by_reg);
                }
                "binop" => {
                    let raw = inst
                        .get("operation")
                        .or_else(|| inst.get("op_kind"))
                        .and_then(|v| v.as_str())
                        .unwrap_or("");
                    if canonical_binop_operation(inst).is_none() {
                        return Err((func_name.clone(), bb, format!("binop({})", raw)));
                    }
                }
                "compare" => {
                    let raw = inst
                        .get("operation")
                        .or_else(|| inst.get("op_kind"))
                        .and_then(|v| v.as_str())
                        .unwrap_or("");
                    if vm_hako_supported_compare_operation(inst).is_none() {
                        return Err((func_name.clone(), bb, format!("compare({})", raw)));
                    }
                }
                "branch" => {
                    if inst.get("cond").and_then(|v| v.as_u64()).is_none()
                        || inst.get("then").and_then(|v| v.as_u64()).is_none()
                        || inst.get("else").and_then(|v| v.as_u64()).is_none()
                    {
                        return Err((func_name.clone(), bb, "branch(malformed)".to_string()));
                    }
                }
                "jump" => {
                    if inst.get("target").and_then(|v| v.as_u64()).is_none() {
                        return Err((func_name.clone(), bb, "jump(malformed)".to_string()));
                    }
                }
                "safepoint" => {}
                // Retired op: accepted for backward compatibility and treated as no-op.
                "nop" => {}
                "keepalive" => {}
                "release_strong" => {}
                "debug" => {
                    if let Err(reason) = validate_debug_shape(inst) {
                        return Err((func_name.clone(), bb, reason));
                    }
                }
                "select" => {
                    if let Err(reason) = validate_select_shape(inst) {
                        return Err((func_name.clone(), bb, reason));
                    }
                }
                "barrier" => {
                    if inst.get("ptr").and_then(|v| v.as_u64()).is_none() {
                        return Err((func_name.clone(), bb, "barrier(missing-ptr)".to_string()));
                    }
                    let raw = inst
                        .get("kind")
                        .or_else(|| inst.get("op_kind"))
                        .and_then(|v| v.as_str())
                        .unwrap_or("");
                    if canonical_barrier_kind(inst).is_none() {
                        return Err((func_name.clone(), bb, format!("barrier(kind:{})", raw)));
                    }
                }
                "load" => {
                    if let Err(reason) = validate_load_shape(inst) {
                        return Err((func_name.clone(), bb, reason));
                    }
                }
                "array_get" => {
                    if let Err(reason) = validate_array_get_shape(inst) {
                        return Err((func_name.clone(), bb, reason));
                    }
                }
                "array_set" => {
                    if let Err(reason) = validate_array_set_shape(inst) {
                        return Err((func_name.clone(), bb, reason));
                    }
                }
                "store" => {
                    if let Err(reason) = validate_store_shape(inst) {
                        return Err((func_name.clone(), bb, reason));
                    }
                }
                "phi" => {
                    if let Err(reason) = validate_phi_shape(inst) {
                        return Err((func_name.clone(), bb, reason));
                    }
                }
                "typeop" => {
                    if let Err(reason) = validate_typeop_shape(inst) {
                        return Err((func_name.clone(), bb, reason));
                    }
                }
                "weak_new" => {
                    if let Err(reason) = validate_weak_new_shape(inst) {
                        return Err((func_name.clone(), bb, reason));
                    }
                }
                "weak_load" => {
                    if let Err(reason) = validate_weak_load_shape(inst) {
                        return Err((func_name.clone(), bb, reason));
                    }
                }
                "ref_new" => {
                    if let Err(reason) = validate_ref_new_shape(inst) {
                        return Err((func_name.clone(), bb, reason));
                    }
                }
                "future_new" => {
                    if let Err(reason) = validate_future_new_shape(inst) {
                        return Err((func_name.clone(), bb, reason));
                    }
                }
                "future_set" => {
                    if let Err(reason) = validate_future_set_shape(inst) {
                        return Err((func_name.clone(), bb, reason));
                    }
                }
                "await" => {
                    if let Err(reason) = validate_await_shape(inst) {
                        return Err((func_name.clone(), bb, reason));
                    }
                }
                "copy" => {
                    update_handle_bindings_from_const_or_copy(inst, &mut handle_by_reg);
                }
                "unop" => {
                    let raw = inst
                        .get("operation")
                        .or_else(|| inst.get("op_kind"))
                        .and_then(|v| v.as_str())
                        .unwrap_or("");
                    if canonical_unop_operation(inst).is_none() {
                        return Err((func_name.clone(), bb, format!("unop({})", raw)));
                    }
                }
                "ret" => {}
                "newbox" => {
                    let box_type = inst.get("type").and_then(|v| v.as_str()).unwrap_or("");
                    if box_type != "ArrayBox"
                        && box_type != "StringBox"
                        && box_type != "FileBox"
                        && box_type != "Main"
                    {
                        return Err((func_name.clone(), bb, format!("newbox({})", box_type)));
                    }
                }
                "boxcall" => {
                    if let Err(reason) = validate_boxcall_shape(inst) {
                        return Err((func_name.clone(), bb, reason));
                    }
                    continue;
                }
                "call" => {
                    let Some(args) = inst.get("args").and_then(|v| v.as_array()) else {
                        return Err((func_name.clone(), bb, "call(malformed)".to_string()));
                    };
                    let Some(func_reg) = inst.get("func").and_then(|v| v.as_u64()) else {
                        return Err((func_name.clone(), bb, "call(missing-func)".to_string()));
                    };
                    let has_dst = inst.get("dst").and_then(|v| v.as_u64()).is_some();
                    if !has_dst {
                        match parse_print_arg_from_instruction(inst, &handle_by_reg) {
                            Ok(Some(_)) => continue,
                            Ok(None) => {}
                            Err(reason) => {
                                return Err((func_name.clone(), bb, reason.to_string()));
                            }
                        }
                        return Err((func_name.clone(), bb, "call(missing-dst)".to_string()));
                    }
                    if args.len() > 2 {
                        return Err((func_name.clone(), bb, "call(args>2)".to_string()));
                    }
                    if args.len() == 1 && args.first().and_then(|v| v.as_u64()).is_none() {
                        return Err((func_name.clone(), bb, "call(arg0:non-reg)".to_string()));
                    }
                    if args.len() == 2 {
                        if let Err(reason) = validate_two_arg_call_target(
                            func_reg,
                            args,
                            &handle_by_reg,
                            allow_dynamic_method_arg1,
                        ) {
                            return Err((
                                func_name.clone(),
                                bb,
                                reason.to_string(),
                            ));
                        }
                    }
                }
                "externcall" => {
                    let func = inst.get("func").and_then(|v| v.as_str()).unwrap_or("");
                    if func == "env.get" || func == "env.get/1" {
                        if let Err(reason) = validate_env_get_externcall_shape(inst) {
                            return Err((func_name.clone(), bb, reason));
                        }
                        continue;
                    }
                    match parse_print_arg_from_instruction(inst, &handle_by_reg) {
                        Ok(Some(_)) => {}
                        Ok(None) => {
                            return Err((
                                func_name.clone(),
                                bb,
                                format!("externcall(func:{})", func),
                            ));
                        }
                        Err(reason) => {
                            return Err((func_name.clone(), bb, reason.to_string()));
                        }
                    }
                }
                "mir_call" => match parse_print_arg_from_instruction(inst, &handle_by_reg) {
                    Ok(Some(_)) => {}
                    Ok(None) => {
                        let callee_name = inst
                            .get("mir_call")
                            .and_then(|v| v.get("callee"))
                            .and_then(|v| v.get("name"))
                            .and_then(|v| v.as_str())
                            .unwrap_or("");
                        return Err((
                            func_name.clone(),
                            bb,
                            format!("mir_call(callee:{})", callee_name),
                        ));
                    }
                    Err(reason) => {
                        return Err((func_name.clone(), bb, reason.to_string()));
                    }
                },
                other => return Err((func_name.clone(), bb, other.to_string())),
            }
        }
    }
    Ok(())
}
