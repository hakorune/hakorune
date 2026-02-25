use serde_json::{json, Map, Value};
use std::collections::{HashMap, HashSet};

use super::shape_contract::{
    collect_function_return_models, extract_string_handle_const, has_id_method_arg1_model,
    normalize_instruction_aliases, parse_print_arg_from_instruction, update_handle_bindings_from_const_or_copy,
    validate_two_arg_call_target, ReturnModel,
};
use super::{DYNAMIC_METHOD_BRIDGE_FUNC_ID, DYNAMIC_METHOD_FUNC_ID};

pub(super) fn extract_main_payload_json(json_text: &str) -> Result<String, String> {
    let root: Value = serde_json::from_str(json_text).map_err(|e| e.to_string())?;
    let functions = root
        .get("functions")
        .and_then(|v| v.as_array())
        .ok_or_else(|| "MissingFunctions".to_string())?;

    let main_func = functions
        .iter()
        .find(|f| f.get("name").and_then(|n| n.as_str()) == Some("main"))
        .or_else(|| functions.first())
        .ok_or_else(|| "EmptyFunctions".to_string())?;

    let entry_block = main_func
        .get("entry_block")
        .and_then(|v| v.as_i64())
        .unwrap_or(0);
    let blocks = main_func
        .get("blocks")
        .cloned()
        .ok_or_else(|| "MissingBlocks".to_string())?;
    let blocks = normalize_instruction_aliases(&blocks);
    let blocks = prune_dead_string_handle_consts(&blocks);
    let (blocks, used_dynamic_bridge) = normalize_dynamic_method_calls(&blocks);
    let blocks = normalize_print_calls(&blocks);
    let return_models = collect_function_return_models(functions);
    let (call0_const_map, mut call1_arg0_map, call2_arg1_map) =
        collect_call_maps(main_func, &return_models);
    if used_dynamic_bridge {
        call1_arg0_map
            .entry(DYNAMIC_METHOD_BRIDGE_FUNC_ID.to_string())
            .or_insert_with(|| json!(1));
    }
    let (boxcall0_const_map, boxcall1_arg0_map) = collect_boxcall_maps(functions, &return_models);

    let payload = json!({
        "entry_block": entry_block,
        "blocks": blocks,
        "call0_const_map": call0_const_map,
        "call1_arg0_map": call1_arg0_map,
        "call2_arg1_map": call2_arg1_map,
        "boxcall0_const_map": boxcall0_const_map,
        "boxcall1_arg0_map": boxcall1_arg0_map,
    });
    serde_json::to_string(&payload).map_err(|e| e.to_string())
}

fn collect_call_maps(
    main_func: &Value,
    models: &HashMap<String, ReturnModel>,
) -> (Map<String, Value>, Map<String, Value>, Map<String, Value>) {
    let mut call0_const_map = Map::new();
    let mut call1_arg0_map = Map::new();
    let mut call2_arg1_map = Map::new();

    let Some(blocks) = main_func.get("blocks").and_then(|v| v.as_array()) else {
        return (call0_const_map, call1_arg0_map, call2_arg1_map);
    };
    let allow_dynamic_method_arg1 = has_id_method_arg1_model(models);

    for block in blocks {
        let Some(insts) = block.get("instructions").and_then(|v| v.as_array()) else {
            continue;
        };
        let mut handle_by_reg: HashMap<u64, String> = HashMap::new();
        for inst in insts {
            if update_handle_bindings_from_const_or_copy(inst, &mut handle_by_reg) {
                continue;
            }
            if inst.get("op").and_then(|v| v.as_str()) != Some("call") {
                continue;
            }
            let Some(args) = inst.get("args").and_then(|v| v.as_array()) else {
                continue;
            };
            let Some(func_reg) = inst.get("func").and_then(|v| v.as_u64()) else {
                continue;
            };
            if args.len() == 2 {
                if validate_two_arg_call_target(
                    func_reg,
                    args,
                    &handle_by_reg,
                    allow_dynamic_method_arg1,
                )
                .is_ok()
                    && func_reg == DYNAMIC_METHOD_FUNC_ID
                {
                    call2_arg1_map
                        .entry(func_reg.to_string())
                        .or_insert_with(|| json!(1));
                    continue;
                }
            }
            let Some(func_name) = handle_by_reg.get(&func_reg) else {
                continue;
            };
            let Some(model) = resolve_return_model_for_handle(func_name, models) else {
                continue;
            };

            if args.is_empty() {
                if let ReturnModel::Const(v) = model {
                    call0_const_map
                        .entry(func_reg.to_string())
                        .or_insert_with(|| json!(v));
                }
                continue;
            }

            if args.len() == 1
                && args.first().and_then(|v| v.as_u64()).is_some()
                && matches!(model, ReturnModel::Arg(0))
            {
                call1_arg0_map
                    .entry(func_reg.to_string())
                    .or_insert_with(|| json!(1));
                continue;
            }

            if args.len() == 2
                && args.first().and_then(|v| v.as_u64()).is_some()
                && args.get(1).and_then(|v| v.as_u64()).is_some()
                && matches!(model, ReturnModel::Arg(1))
            {
                call2_arg1_map
                    .entry(func_reg.to_string())
                    .or_insert_with(|| json!(1));
            }
        }
    }

    (call0_const_map, call1_arg0_map, call2_arg1_map)
}

fn collect_boxcall_maps(
    functions: &[Value],
    models: &HashMap<String, ReturnModel>,
) -> (Map<String, Value>, Map<String, Value>) {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum Method0State {
        Const(i64),
        Invalid,
    }

    let mut method0_state: HashMap<String, Method0State> = HashMap::new();
    let mut method1_state: HashMap<String, i8> = HashMap::new(); // 1=arg0-only, -1=conflicted/non-arg0

    for func in functions {
        let Some(name) = func.get("name").and_then(|v| v.as_str()) else {
            continue;
        };
        let Some((method_name, arity)) = parse_method_name_and_arity(name) else {
            continue;
        };
        let model = models.get(name).copied();

        if arity == 0 {
            use Method0State::{Const, Invalid};
            match method0_state.entry(method_name) {
                std::collections::hash_map::Entry::Vacant(e) => {
                    if let Some(ReturnModel::Const(v)) = model {
                        e.insert(Const(v));
                    } else {
                        e.insert(Invalid);
                    }
                }
                std::collections::hash_map::Entry::Occupied(mut e) => {
                    let next = match (*e.get(), model) {
                        (Const(prev), Some(ReturnModel::Const(v))) if prev == v => Const(prev),
                        _ => Invalid,
                    };
                    e.insert(next);
                }
            }
            continue;
        }

        if arity == 1 {
            let is_arg0 = matches!(model, Some(ReturnModel::Arg(0)));
            let state = method1_state.entry(method_name).or_insert(0);
            if is_arg0 {
                if *state == 0 {
                    *state = 1;
                }
            } else {
                *state = -1;
            }
        }
    }

    let mut boxcall0_const_map = Map::new();
    for (method, state) in method0_state {
        if let Method0State::Const(v) = state {
            boxcall0_const_map.insert(method, json!(v));
        }
    }

    let mut boxcall1_arg0_map = Map::new();
    for (method, state) in method1_state {
        if state == 1 {
            boxcall1_arg0_map.insert(method, json!(1));
        }
    }

    (boxcall0_const_map, boxcall1_arg0_map)
}

fn parse_method_name_and_arity(name: &str) -> Option<(String, usize)> {
    let slash = name.rfind('/')?;
    let arity = name[(slash + 1)..].parse::<usize>().ok()?;
    let head = &name[..slash];
    let dot = head.rfind('.')?;
    if dot + 1 >= head.len() {
        return None;
    }
    Some((head[(dot + 1)..].to_string(), arity))
}

fn resolve_return_model_for_handle(
    handle: &str,
    models: &HashMap<String, ReturnModel>,
) -> Option<ReturnModel> {
    if let Some(model) = models.get(handle).copied() {
        return Some(model);
    }

    if let Some(model) = resolve_return_model_by_base_name(handle, models) {
        return Some(model);
    }

    if let Some(rest) = handle.strip_prefix("__method_") {
        if let Some(split) = rest.find('_') {
            let base = format!("{}.{}", &rest[..split], &rest[(split + 1)..]);
            if let Some(model) = resolve_return_model_by_base_name(&base, models) {
                return Some(model);
            }
        }
    }

    None
}

fn resolve_return_model_by_base_name(
    base: &str,
    models: &HashMap<String, ReturnModel>,
) -> Option<ReturnModel> {
    let mut matched: Option<ReturnModel> = None;
    for (name, model) in models {
        let Some(head) = name.split('/').next() else {
            continue;
        };
        if head != base {
            continue;
        }
        match matched {
            None => matched = Some(*model),
            Some(prev) if prev == *model => {}
            Some(_) => return None,
        }
    }
    matched
}

fn canonical_print_externcall(arg0: u64) -> Value {
    json!({
        "op": "externcall",
        "func": "nyash.console.log",
        "args": [arg0]
    })
}

fn prune_dead_string_handle_consts(blocks: &Value) -> Value {
    let Some(blocks_arr) = blocks.as_array() else {
        return blocks.clone();
    };

    let mut used_regs = HashSet::new();
    for block in blocks_arr {
        let Some(insts) = block.get("instructions").and_then(|v| v.as_array()) else {
            continue;
        };
        for inst in insts {
            for key in ["src", "lhs", "rhs", "cond", "value", "func", "box"] {
                if let Some(reg) = inst.get(key).and_then(|v| v.as_u64()) {
                    used_regs.insert(reg);
                }
            }
            if let Some(args) = inst.get("args").and_then(|v| v.as_array()) {
                for arg in args {
                    if let Some(reg) = arg.as_u64() {
                        used_regs.insert(reg);
                    }
                }
            }
            if let Some(incoming) = inst.get("incoming").and_then(|v| v.as_array()) {
                for entry in incoming {
                    let Some(pair) = entry.as_array() else {
                        continue;
                    };
                    if let Some(reg) = pair.first().and_then(|v| v.as_u64()) {
                        used_regs.insert(reg);
                    }
                }
            }
            if let Some(values) = inst.get("values").and_then(|v| v.as_array()) {
                for item in values {
                    if let Some(reg) = item.get("value").and_then(|v| v.as_u64()) {
                        used_regs.insert(reg);
                    } else if let Some(reg) = item.as_u64() {
                        used_regs.insert(reg);
                    }
                }
            }
        }
    }

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
        let mut filtered = Vec::new();
        for inst in insts {
            let keep = if extract_string_handle_const(inst).is_some() {
                let dst = inst.get("dst").and_then(|v| v.as_u64());
                dst.map(|r| used_regs.contains(&r)).unwrap_or(true)
            } else {
                true
            };
            if keep {
                filtered.push(inst.clone());
            }
        }
        obj.insert("instructions".to_string(), Value::Array(filtered));
        out_blocks.push(Value::Object(obj));
    }
    Value::Array(out_blocks)
}

fn normalize_dynamic_method_calls(blocks: &Value) -> (Value, bool) {
    let Some(blocks_arr) = blocks.as_array() else {
        return (blocks.clone(), false);
    };

    let mut changed = false;
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
            if inst.get("op").and_then(|v| v.as_str()) == Some("call")
                && inst.get("func").and_then(|v| v.as_u64()) == Some(DYNAMIC_METHOD_FUNC_ID)
            {
                if let Some(args) = inst.get("args").and_then(|v| v.as_array()) {
                    if args.len() == 2 {
                        if let Some(arg1) = args.get(1).and_then(|v| v.as_u64()) {
                            next["func"] = json!(DYNAMIC_METHOD_BRIDGE_FUNC_ID);
                            next["args"] = json!([arg1]);
                            changed = true;
                        }
                    }
                }
            }
            rewritten.push(next);
        }
        obj.insert("instructions".to_string(), Value::Array(rewritten));
        out_blocks.push(Value::Object(obj));
    }
    (Value::Array(out_blocks), changed)
}

fn normalize_print_calls(blocks: &Value) -> Value {
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
        let mut handle_by_reg: HashMap<u64, String> = HashMap::new();
        let mut rewritten = Vec::new();
        for inst in insts {
            if update_handle_bindings_from_const_or_copy(inst, &mut handle_by_reg) {
                rewritten.push(inst.clone());
                continue;
            }

            match parse_print_arg_from_instruction(inst, &handle_by_reg) {
                Ok(Some(arg0)) => rewritten.push(canonical_print_externcall(arg0)),
                _ => rewritten.push(inst.clone()),
            }
        }
        obj.insert("instructions".to_string(), Value::Array(rewritten));
        out_blocks.push(Value::Object(obj));
    }
    Value::Array(out_blocks)
}
