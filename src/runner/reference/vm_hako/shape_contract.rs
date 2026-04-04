use serde_json::{json, Value};
use std::collections::HashMap;

use super::{DYNAMIC_METHOD_BRIDGE_FUNC_ID, DYNAMIC_METHOD_FUNC_ID};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum ReturnModel {
    Const(i64),
    Arg(usize),
}

pub(super) fn collect_function_return_models(functions: &[Value]) -> HashMap<String, ReturnModel> {
    let mut out = HashMap::new();
    for func in functions {
        let Some(name) = func.get("name").and_then(|v| v.as_str()) else {
            continue;
        };
        let Some(model) = extract_return_model(func) else {
            continue;
        };
        out.insert(name.to_string(), model);
    }
    out
}

pub(super) fn has_id_method_arg1_model(models: &HashMap<String, ReturnModel>) -> bool {
    models
        .iter()
        .any(|(name, model)| name.ends_with(".id/1") && *model == ReturnModel::Arg(1))
}

fn extract_return_model(func: &Value) -> Option<ReturnModel> {
    let params = func.get("params").and_then(|v| v.as_array())?;
    let blocks = func.get("blocks").and_then(|v| v.as_array())?;
    if blocks.len() != 1 {
        return None;
    }
    let insts = blocks
        .first()?
        .get("instructions")
        .and_then(|v| v.as_array())?;

    let ret_reg = insts.iter().find_map(|inst| {
        if inst.get("op").and_then(|v| v.as_str()) != Some("ret") {
            return None;
        }
        inst.get("value").and_then(|v| v.as_u64())
    })?;

    if params.is_empty() {
        let v = extract_const_for_dst(insts, ret_reg)?;
        return Some(ReturnModel::Const(v));
    }

    for (idx, param) in params.iter().enumerate() {
        let Some(pid) = param.as_u64() else {
            continue;
        };
        if ret_reg == pid {
            return Some(ReturnModel::Arg(idx));
        }
        if let Some(src) = extract_copy_src_for_dst(insts, ret_reg) {
            if src == pid {
                return Some(ReturnModel::Arg(idx));
            }
        }
    }

    None
}

fn extract_copy_src_for_dst(insts: &[Value], dst_reg: u64) -> Option<u64> {
    insts.iter().find_map(|inst| {
        if inst.get("op").and_then(|v| v.as_str()) != Some("copy") {
            return None;
        }
        let dst = inst.get("dst").and_then(|v| v.as_u64())?;
        if dst != dst_reg {
            return None;
        }
        inst.get("src").and_then(|v| v.as_u64())
    })
}

fn extract_const_for_dst(insts: &[Value], dst_reg: u64) -> Option<i64> {
    insts.iter().find_map(|inst| {
        if inst.get("op").and_then(|v| v.as_str()) != Some("const") {
            return None;
        }
        let dst = inst.get("dst").and_then(|v| v.as_u64())?;
        if dst != dst_reg {
            return None;
        }
        extract_const_i64_or_bool(inst.get("value")?)
    })
}

fn extract_const_i64_or_bool(value: &Value) -> Option<i64> {
    let ty = value.get("type")?.as_str()?;
    match ty {
        "i64" => value.get("value").and_then(|v| v.as_i64()),
        "bool" => value
            .get("value")
            .and_then(|v| v.as_bool())
            .map(|v| if v { 1 } else { 0 }),
        _ => None,
    }
}

pub(super) fn is_method_like_handle(name: &str) -> bool {
    name.starts_with("__method_") || (name.contains('.') && name.contains('/'))
}

pub(super) fn update_handle_bindings_from_const_or_copy(
    inst: &Value,
    handle_by_reg: &mut HashMap<u64, String>,
) -> bool {
    if let Some((dst, name)) = extract_string_handle_const(inst) {
        handle_by_reg.insert(dst, name);
        return true;
    }

    if inst.get("op").and_then(|v| v.as_str()) == Some("copy") {
        if let (Some(dst), Some(src)) = (
            inst.get("dst").and_then(|v| v.as_u64()),
            inst.get("src").and_then(|v| v.as_u64()),
        ) {
            if let Some(name) = handle_by_reg.get(&src).cloned() {
                handle_by_reg.insert(dst, name);
            }
        }
        return true;
    }

    false
}

pub(super) fn validate_two_arg_call_target(
    func_reg: u64,
    args: &[Value],
    handle_by_reg: &HashMap<u64, String>,
    allow_dynamic_method_arg1: bool,
) -> Result<(), &'static str> {
    if args.len() != 2 {
        return Err("call(args!=2)");
    }
    if args.first().and_then(|v| v.as_u64()).is_none()
        || args.get(1).and_then(|v| v.as_u64()).is_none()
    {
        return Err("call(args2:non-reg)");
    }

    if func_reg == DYNAMIC_METHOD_FUNC_ID {
        if allow_dynamic_method_arg1 {
            return Ok(());
        }
        return Err("call(args2:dynamic-no-id1-model)");
    }

    let Some(handle_name) = handle_by_reg.get(&func_reg) else {
        return Err("call(args2:missing-handle)");
    };
    if !is_method_like_handle(handle_name) {
        return Err("call(args2:non-method-handle)");
    }
    Ok(())
}

fn is_print_like_handle(name: &str) -> bool {
    name == "print"
        || name.ends_with(".print/1")
        || name.ends_with(".println/1")
        || name.starts_with("__fn_print")
        || name.starts_with("__global_print")
}

fn call_callee_payload(inst: &Value) -> Option<&Value> {
    inst.get("callee")
        .or_else(|| inst.get("mir_call").and_then(|v| v.get("callee")))
}

pub(super) fn call_callee_type(inst: &Value) -> Option<&str> {
    call_callee_payload(inst)
        .and_then(|v| v.get("type"))
        .and_then(|v| v.as_str())
}

pub(super) fn call_callee_name(inst: &Value) -> Option<&str> {
    call_callee_payload(inst)
        .and_then(|v| v.get("name"))
        .and_then(|v| v.as_str())
}

fn is_print_like_global_callee(inst: &Value) -> bool {
    matches!(call_callee_type(inst), Some("Global"))
        && matches!(call_callee_name(inst), Some("print" | "println"))
}

pub(super) fn parse_print_arg_from_instruction(
    inst: &Value,
    handle_by_reg: &HashMap<u64, String>,
) -> Result<Option<u64>, &'static str> {
    let op = inst.get("op").and_then(|v| v.as_str()).unwrap_or("");
    match op {
        "externcall" => parse_print_arg_from_externcall(inst),
        "mir_call" => Err("mir_call(legacy-removed)"),
        "call" => parse_print_arg_from_dynamic_call(inst, handle_by_reg),
        _ => Ok(None),
    }
}

fn parse_print_arg_from_externcall(inst: &Value) -> Result<Option<u64>, &'static str> {
    let Some(func) = inst.get("func").and_then(|v| v.as_str()) else {
        return Err("externcall(missing-func)");
    };
    if func != "nyash.console.log" {
        return Ok(None);
    }
    let Some(args) = inst.get("args").and_then(|v| v.as_array()) else {
        return Err("externcall(malformed)");
    };
    if args.len() != 1 {
        return Err("externcall(args!=1)");
    }
    let Some(arg0) = args.first().and_then(|v| v.as_u64()) else {
        return Err("externcall(arg0:non-reg)");
    };
    if let Some(dst) = inst.get("dst") {
        if !dst.is_null() && dst.as_u64().is_none() {
            return Err("externcall(dst:non-reg)");
        }
    }
    Ok(Some(arg0))
}

fn parse_print_arg_from_dynamic_call(
    inst: &Value,
    handle_by_reg: &HashMap<u64, String>,
) -> Result<Option<u64>, &'static str> {
    if inst.get("dst").and_then(|v| v.as_u64()).is_some() {
        return Ok(None);
    }
    let Some(args) = inst.get("args").and_then(|v| v.as_array()) else {
        return Err("call(malformed)");
    };
    let func_reg = inst.get("func").and_then(|v| v.as_u64());
    let is_print_route = if let Some(func_reg) = func_reg {
        func_reg == DYNAMIC_METHOD_FUNC_ID
            || func_reg == DYNAMIC_METHOD_BRIDGE_FUNC_ID
            || handle_by_reg
                .get(&func_reg)
                .map(|name| is_print_like_handle(name))
                .unwrap_or(false)
    } else {
        is_print_like_global_callee(inst)
    };
    if !is_print_route {
        return Ok(None);
    }
    if args.len() != 1 {
        return Err("call(print:args!=1)");
    }
    let Some(arg0) = args.first().and_then(|v| v.as_u64()) else {
        return Err("call(print:arg0:non-reg)");
    };
    Ok(Some(arg0))
}

pub(super) fn extract_string_handle_const(inst: &Value) -> Option<(u64, String)> {
    if inst.get("op").and_then(|v| v.as_str()) != Some("const") {
        return None;
    }
    let dst = inst.get("dst").and_then(|v| v.as_u64())?;
    let value = inst.get("value")?;
    let value_obj = value.as_object()?;
    let ty = value_obj.get("type")?;
    let is_string_handle = match ty {
        Value::Object(obj) => {
            obj.get("box_type").and_then(|v| v.as_str()) == Some("StringBox")
                && obj.get("kind").and_then(|v| v.as_str()) == Some("handle")
        }
        _ => false,
    };
    if !is_string_handle {
        return None;
    }
    let name = value_obj.get("value")?.as_str()?.to_string();
    Some((dst, name))
}

pub(super) fn canonical_binop_operation(inst: &Value) -> Option<&'static str> {
    let raw = inst
        .get("operation")
        .or_else(|| inst.get("op_kind"))
        .and_then(|v| v.as_str())?;
    if raw == "+" || raw.eq_ignore_ascii_case("add") {
        return Some("+");
    }
    if raw == "-" || raw.eq_ignore_ascii_case("sub") {
        return Some("-");
    }
    if raw == "*" || raw.eq_ignore_ascii_case("mul") {
        return Some("*");
    }
    if raw == "/" || raw.eq_ignore_ascii_case("div") {
        return Some("/");
    }
    if raw == "%" || raw.eq_ignore_ascii_case("mod") {
        return Some("%");
    }
    None
}

fn canonical_compare_operation(inst: &Value) -> Option<&'static str> {
    let raw = inst
        .get("operation")
        .or_else(|| inst.get("op_kind"))
        .and_then(|v| v.as_str())?;
    if raw == "==" || raw.eq_ignore_ascii_case("eq") {
        return Some("==");
    }
    if raw == "!=" || raw.eq_ignore_ascii_case("ne") {
        return Some("!=");
    }
    if raw == "<" || raw.eq_ignore_ascii_case("lt") {
        return Some("<");
    }
    if raw == "<=" || raw.eq_ignore_ascii_case("le") {
        return Some("<=");
    }
    if raw == ">" || raw.eq_ignore_ascii_case("gt") {
        return Some(">");
    }
    if raw == ">=" || raw.eq_ignore_ascii_case("ge") {
        return Some(">=");
    }
    None
}

// Keep in sync with compare handling in `lang/src/vm/boxes/mir_vm_s0_exec_dispatch.hako`.
// Drift is guarded by `vm_hako_runtime_compare_contract_is_in_sync` test.
const VM_HAKO_COMPARE_ALLOWLIST: [&str; 6] = ["==", "!=", "<", "<=", ">", ">="];

pub(super) fn vm_hako_supported_compare_operation(inst: &Value) -> Option<&'static str> {
    let sym = canonical_compare_operation(inst)?;
    if VM_HAKO_COMPARE_ALLOWLIST.contains(&sym) {
        return Some(sym);
    }
    None
}

pub(super) fn canonical_unop_operation(inst: &Value) -> Option<&'static str> {
    let raw = inst
        .get("operation")
        .or_else(|| inst.get("op_kind"))
        .and_then(|v| v.as_str())?;
    if raw.eq_ignore_ascii_case("neg") || raw == "-" {
        return Some("neg");
    }
    if raw.eq_ignore_ascii_case("not") || raw == "!" {
        return Some("not");
    }
    None
}

pub(super) fn canonical_barrier_kind(inst: &Value) -> Option<&'static str> {
    let raw = inst
        .get("kind")
        .or_else(|| inst.get("op_kind"))
        .and_then(|v| v.as_str())?;
    if raw.eq_ignore_ascii_case("read") {
        return Some("read");
    }
    if raw.eq_ignore_ascii_case("write") {
        return Some("write");
    }
    None
}

pub(super) fn canonical_typeop_operation(raw: &str) -> &'static str {
    if raw.eq_ignore_ascii_case("check") || raw.eq_ignore_ascii_case("is") {
        return "check";
    }
    if raw.eq_ignore_ascii_case("cast") || raw.eq_ignore_ascii_case("as") {
        return "cast";
    }
    "unknown"
}

pub(super) fn normalize_instruction_aliases(blocks: &Value) -> Value {
    let Some(blocks_arr) = blocks.as_array() else {
        return blocks.clone();
    };

    let mut out_blocks = Vec::new();
    for block in blocks_arr {
        let Some(mut obj) = block.as_object().cloned() else {
            out_blocks.push(block.clone());
            continue;
        };
        let Some(insts) = block.get("instructions").and_then(|v| v.as_array()) else {
            out_blocks.push(block.clone());
            continue;
        };
        let mut rewritten = Vec::new();
        for inst in insts {
            let mut next = inst.clone();
            let op = inst.get("op").and_then(|v| v.as_str()).unwrap_or("");
            match op {
                "binop" => {
                    if let Some(sym) = canonical_binop_operation(inst) {
                        next["operation"] = json!(sym);
                    }
                }
                "compare" => {
                    if let Some(sym) = canonical_compare_operation(inst) {
                        next["operation"] = json!(sym);
                    }
                }
                "unop" => {
                    if let Some(sym) = canonical_unop_operation(inst) {
                        next["operation"] = json!(sym);
                    }
                }
                "barrier" => {
                    if let Some(kind) = canonical_barrier_kind(inst) {
                        next["kind"] = json!(kind);
                    }
                }
                "typeop" => {
                    if let Some(raw) = inst
                        .get("operation")
                        .or_else(|| inst.get("op_kind"))
                        .and_then(|v| v.as_str())
                    {
                        let op = canonical_typeop_operation(raw);
                        if op != "unknown" {
                            next["operation"] = json!(op);
                        }
                    }
                    if next.get("src").and_then(|v| v.as_u64()).is_none() {
                        if let Some(src) = next.get("value").and_then(|v| v.as_u64()) {
                            next["src"] = json!(src);
                        }
                    }
                    if next.get("target_type").and_then(|v| v.as_str()).is_none() {
                        if let Some(ty) = next.get("ty").and_then(|v| v.as_str()) {
                            next["target_type"] = json!(ty);
                        }
                    }
                }
                _ => {}
            }
            rewritten.push(next);
        }
        obj.insert("instructions".to_string(), Value::Array(rewritten));
        out_blocks.push(Value::Object(obj));
    }
    Value::Array(out_blocks)
}

pub(super) fn normalize_aliases_in_root(root: &mut Value) {
    let Some(functions) = root.get_mut("functions").and_then(|v| v.as_array_mut()) else {
        return;
    };
    for func in functions.iter_mut() {
        let Some(blocks) = func.get("blocks").cloned() else {
            continue;
        };
        let normalized = normalize_instruction_aliases(&blocks);
        if let Some(obj) = func.as_object_mut() {
            obj.insert("blocks".to_string(), normalized);
        }
    }
}
