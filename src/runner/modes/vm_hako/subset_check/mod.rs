use serde_json::Value;
use std::collections::HashMap;

use super::shape_contract::{
    call_callee_name, call_callee_type, canonical_barrier_kind, canonical_binop_operation,
    canonical_unop_operation, collect_function_return_models, has_id_method_arg1_model,
    normalize_aliases_in_root, parse_print_arg_from_instruction,
    update_handle_bindings_from_const_or_copy, validate_two_arg_call_target,
    vm_hako_supported_compare_operation,
};

mod boxcalls;
mod externcalls;
mod shapes;

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
        let mut box_type_by_reg: HashMap<u64, String> = HashMap::new();

        for inst in insts {
            let op = inst.get("op").and_then(|v| v.as_str()).unwrap_or("");
            match op {
                "const" => {
                    if let Err(reason) = shapes::validate_const_shape(inst) {
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
                "nop" => {}
                "keepalive" => {}
                "release_strong" => {}
                "debug" => {
                    if let Err(reason) = shapes::validate_debug_shape(inst) {
                        return Err((func_name.clone(), bb, reason));
                    }
                }
                "select" => {
                    if let Err(reason) = shapes::validate_select_shape(inst) {
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
                    if let Err(reason) = shapes::validate_load_shape(inst) {
                        return Err((func_name.clone(), bb, reason));
                    }
                }
                "array_get" => {
                    if let Err(reason) = shapes::validate_array_get_shape(inst) {
                        return Err((func_name.clone(), bb, reason));
                    }
                }
                "array_set" => {
                    if let Err(reason) = shapes::validate_array_set_shape(inst) {
                        return Err((func_name.clone(), bb, reason));
                    }
                }
                "store" => {
                    if let Err(reason) = shapes::validate_store_shape(inst) {
                        return Err((func_name.clone(), bb, reason));
                    }
                }
                "phi" => {
                    if let Err(reason) = shapes::validate_phi_shape(inst) {
                        return Err((func_name.clone(), bb, reason));
                    }
                }
                "typeop" => {
                    if let Err(reason) = shapes::validate_typeop_shape(inst) {
                        return Err((func_name.clone(), bb, reason));
                    }
                }
                "weak_new" => {
                    if let Err(reason) = shapes::validate_weak_new_shape(inst) {
                        return Err((func_name.clone(), bb, reason));
                    }
                }
                "weak_load" => {
                    if let Err(reason) = shapes::validate_weak_load_shape(inst) {
                        return Err((func_name.clone(), bb, reason));
                    }
                }
                "ref_new" => {
                    if let Err(reason) = shapes::validate_ref_new_shape(inst) {
                        return Err((func_name.clone(), bb, reason));
                    }
                }
                "future_new" => {
                    if let Err(reason) = shapes::validate_future_new_shape(inst) {
                        return Err((func_name.clone(), bb, reason));
                    }
                }
                "future_set" => {
                    if let Err(reason) = shapes::validate_future_set_shape(inst) {
                        return Err((func_name.clone(), bb, reason));
                    }
                }
                "await" => {
                    if let Err(reason) = shapes::validate_await_shape(inst) {
                        return Err((func_name.clone(), bb, reason));
                    }
                }
                "copy" => {
                    update_handle_bindings_from_const_or_copy(inst, &mut handle_by_reg);
                    if let (Some(dst), Some(src)) = (
                        inst.get("dst").and_then(|v| v.as_u64()),
                        inst.get("src").and_then(|v| v.as_u64()),
                    ) {
                        if let Some(box_type) = box_type_by_reg.get(&src).cloned() {
                            box_type_by_reg.insert(dst, box_type);
                        }
                    }
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
                        && box_type != "MapBox"
                        && box_type != "StringBox"
                        && box_type != "FileBox"
                        && box_type != "LlvmBackendBox"
                        && box_type != "OsVmCoreBox"
                        && box_type != "TlsCoreBox"
                        && box_type != "AtomicCoreBox"
                        && box_type != "GcCoreBox"
                        && box_type != "Main"
                    {
                        return Err((func_name.clone(), bb, format!("newbox({})", box_type)));
                    }
                    if let Some(dst) = inst.get("dst").and_then(|v| v.as_u64()) {
                        box_type_by_reg.insert(dst, box_type.to_string());
                    }
                }
                "boxcall" => {
                    if let Err(reason) = boxcalls::validate_boxcall_shape(inst) {
                        return Err((func_name.clone(), bb, reason));
                    }
                    let method = inst.get("method").and_then(|v| v.as_str()).unwrap_or("");
                    if let Some(box_reg) = inst.get("box").and_then(|v| v.as_u64()) {
                        if let Some(box_type) = box_type_by_reg.get(&box_reg) {
                            if box_type == "OsVmCoreBox" && method != "reserve_bytes_i64" {
                                return Err((
                                    func_name.clone(),
                                    bb,
                                    format!("boxcall(osvm:{})", method),
                                ));
                            }
                        }
                    }
                    continue;
                }
                "call" => {
                    let Some(args) = inst.get("args").and_then(|v| v.as_array()) else {
                        return Err((func_name.clone(), bb, "call(malformed)".to_string()));
                    };
                    let has_dst = inst.get("dst").and_then(|v| v.as_u64()).is_some();
                    let func_reg = inst.get("func").and_then(|v| v.as_u64());
                    if let Some(func_reg) = func_reg {
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
                                return Err((func_name.clone(), bb, reason.to_string()));
                            }
                        }
                        continue;
                    }

                    let callee_type = call_callee_type(inst).unwrap_or("");
                    if callee_type != "Global" {
                        return Err((func_name.clone(), bb, "call(missing-func)".to_string()));
                    }
                    let callee_name = call_callee_name(inst).unwrap_or("");
                    if callee_name.is_empty() {
                        return Err((func_name.clone(), bb, "call(global:missing-name)".to_string()));
                    }
                    if !has_dst {
                        match parse_print_arg_from_instruction(inst, &handle_by_reg) {
                            Ok(Some(_)) => continue,
                            Ok(None) => {}
                            Err(reason) => {
                                return Err((func_name.clone(), bb, reason.to_string()));
                            }
                        }
                        return Err((func_name.clone(), bb, "call(global:missing-dst)".to_string()));
                    }
                    if args.iter().any(|v| v.as_u64().is_none()) {
                        return Err((func_name.clone(), bb, "call(global:arg:non-reg)".to_string()));
                    }
                }
                "externcall" => {
                    let func = inst.get("func").and_then(|v| v.as_str()).unwrap_or("");
                    if func == "env.get" || func == "env.get/1" {
                        if let Err(reason) = externcalls::validate_env_get_externcall_shape(inst) {
                            return Err((func_name.clone(), bb, reason));
                        }
                        continue;
                    }
                    if func == "env.mirbuilder_emit"
                        || func == "env.mirbuilder_emit/1"
                        || func == "env.mirbuilder.emit"
                        || func == "env.mirbuilder.emit/1"
                    {
                        if let Err(reason) =
                            externcalls::validate_mirbuilder_emit_externcall_shape(inst)
                        {
                            return Err((func_name.clone(), bb, reason));
                        }
                        continue;
                    }
                    if func == "hako_last_error" || func == "hako_last_error/1" {
                        if let Err(reason) =
                            externcalls::validate_single_arg_externcall_shape(inst, "hako_last_error")
                        {
                            return Err((func_name.clone(), bb, reason));
                        }
                        continue;
                    }
                    if func == "nyash.box.from_i8_string" || func == "nyash.box.from_i8_string/1" {
                        if let Err(reason) = externcalls::validate_single_arg_externcall_shape(
                            inst,
                            "nyash.box.from_i8_string",
                        ) {
                            return Err((func_name.clone(), bb, reason));
                        }
                        continue;
                    }
                    if func == "hako_barrier_touch_i64" || func == "hako_barrier_touch_i64/1" {
                        if let Err(reason) = externcalls::validate_single_arg_externcall_shape(
                            inst,
                            "hako_barrier_touch_i64",
                        ) {
                            return Err((func_name.clone(), bb, reason));
                        }
                        continue;
                    }
                    if func == "hako_osvm_reserve_bytes_i64"
                        || func == "hako_osvm_reserve_bytes_i64/1"
                    {
                        if let Err(reason) = externcalls::validate_single_arg_externcall_shape(
                            inst,
                            "hako_osvm_reserve_bytes_i64",
                        ) {
                            return Err((func_name.clone(), bb, reason));
                        }
                        continue;
                    }
                    if func.starts_with("hako_osvm_") {
                        return Err((
                            func_name.clone(),
                            bb,
                            format!("externcall({})", func),
                        ));
                    }
                    if func == "nyash.gc.barrier_write" || func == "nyash.gc.barrier_write/1" {
                        if let Err(reason) = externcalls::validate_single_arg_externcall_shape(
                            inst,
                            "nyash.gc.barrier_write",
                        ) {
                            return Err((func_name.clone(), bb, reason));
                        }
                        continue;
                    }
                    match parse_print_arg_from_instruction(inst, &handle_by_reg) {
                        Ok(Some(_)) => {}
                        Ok(None) => {
                            return Err((func_name.clone(), bb, format!("externcall(func:{})", func)));
                        }
                        Err(reason) => {
                            return Err((func_name.clone(), bb, reason.to_string()));
                        }
                    }
                }
                "mir_call" => {
                    if let Err(reason) = boxcalls::validate_mir_call_shape(inst) {
                        return Err((func_name.clone(), bb, reason));
                    }
                    continue;
                }
                other => return Err((func_name.clone(), bb, other.to_string())),
            }
        }
    }
    Ok(())
}
