/*!
 * core_bridge.rs — NyVM wrapper bridge helpers
 *
 * Provides a JSON canonicalizer for NyVmDispatcher wrapper path.
 * Optional env toggles:
 *   - HAKO_BRIDGE_INJECT_SINGLETON / NYASH_BRIDGE_INJECT_SINGLETON:
 *       Rewrite ModuleFunction Array/Map len calls into Method form.
 *   - HAKO_BRIDGE_EARLY_PHI_MATERIALIZE / NYASH_BRIDGE_EARLY_PHI_MATERIALIZE:
 *       Move phi instructions to block head (order-preserving).
 * Dumps payload when `HAKO_DEBUG_NYVM_BRIDGE_DUMP` is set to a file path.
 */

use serde_json::Value;
use std::{env, fs};

pub fn canonicalize_module_json(input: &str) -> Result<String, String> {
    let mut output = input.to_string();

    if let Ok(path) = env::var("HAKO_DEBUG_NYVM_BRIDGE_DUMP") {
        if !path.trim().is_empty() {
            if let Err(e) = fs::write(&path, input.as_bytes()) {
                // Phase 98: ConsoleService if available, otherwise eprintln
                crate::console_println!("[bridge/dump] write error: {}", e);
            }
        }
    }

    let inject_singleton =
        env_flag("HAKO_BRIDGE_INJECT_SINGLETON") || env_flag("NYASH_BRIDGE_INJECT_SINGLETON");
    let materialize_phi = env_flag("HAKO_BRIDGE_EARLY_PHI_MATERIALIZE")
        || env_flag("NYASH_BRIDGE_EARLY_PHI_MATERIALIZE");
    let methodize = env_flag("HAKO_BRIDGE_METHODIZE") || env_flag("NYASH_BRIDGE_METHODIZE");

    if inject_singleton || materialize_phi || methodize {
        let mut json: Value = serde_json::from_str(input)
            .map_err(|e| format!("bridge canonicalize: invalid JSON ({})", e))?;
        let mut mutated = false;
        if inject_singleton {
            mutated |= inject_singleton_methods(&mut json)?;
        }
        if materialize_phi {
            mutated |= materialize_phi_blocks(&mut json)?;
        }
        if methodize {
            mutated |= methodize_calls(&mut json)?;
        }
        if mutated {
            output = serde_json::to_string(&json)
                .map_err(|e| format!("bridge canonicalize: serialize error ({})", e))?;
            // Optional: dump mutated JSON for diff-based tests
            if let Ok(path) = env::var("HAKO_DEBUG_NYVM_BRIDGE_DUMP_MUT") {
                if !path.trim().is_empty() {
                    if let Err(e) = fs::write(&path, output.as_bytes()) {
                        // Phase 98: ConsoleService if available, otherwise eprintln
                        crate::console_println!("[bridge/dump-mut] write error: {}", e);
                    }
                }
            }
        }
    }

    Ok(output)
}

fn env_flag(name: &str) -> bool {
    env::var(name)
        .ok()
        .map(|v| matches!(v.trim().to_ascii_lowercase().as_str(), "1" | "true" | "on"))
        .unwrap_or(false)
}

fn inject_singleton_methods(root: &mut Value) -> Result<bool, String> {
    let mut changed = false;
    let functions = match root.as_object_mut() {
        Some(obj) => obj.get_mut("functions"),
        None => return Err("bridge canonicalize: expected JSON object at root".into()),
    };
    let functions = match functions {
        Some(Value::Array(arr)) => arr,
        Some(_) => return Err("bridge canonicalize: functions must be array".into()),
        None => return Ok(false),
    };

    for func in functions.iter_mut() {
        let blocks = func.get_mut("blocks").and_then(Value::as_array_mut);
        let Some(blocks) = blocks else { continue };
        for block in blocks.iter_mut() {
            let insts = block.get_mut("instructions").and_then(Value::as_array_mut);
            let Some(insts) = insts else { continue };
            for inst in insts.iter_mut() {
                if transform_module_function(inst)? {
                    changed = true;
                }
            }
        }
    }

    Ok(changed)
}

/// Rewrite legacy call(func=reg) with Const string target "Box.method/N" into unified
/// mir_call(Method { box_name, method }, args)
fn methodize_calls(root: &mut Value) -> Result<bool, String> {
    let mut changed = false;
    let functions = match root.as_object_mut() {
        Some(obj) => obj.get_mut("functions"),
        None => return Err("bridge canonicalize: expected JSON object at root".into()),
    };
    let functions = match functions {
        Some(Value::Array(arr)) => arr,
        Some(_) => return Err("bridge canonicalize: functions must be array".into()),
        None => return Ok(false),
    };

    for func in functions.iter_mut() {
        let blocks = func.get_mut("blocks").and_then(Value::as_array_mut);
        let Some(blocks) = blocks else { continue };
        for block in blocks.iter_mut() {
            let insts_opt = block.get_mut("instructions").and_then(Value::as_array_mut);
            let Some(insts) = insts_opt else { continue };
            // First pass: collect const string targets reg -> name
            use std::collections::HashMap;
            let mut reg_name: HashMap<i64, String> = HashMap::new();
            for inst in insts.iter() {
                if let Some(obj) = inst.as_object() {
                    if obj.get("op").and_then(Value::as_str) == Some("const") {
                        if let (Some(dst), Some(val)) =
                            (obj.get("dst").and_then(Value::as_i64), obj.get("value"))
                        {
                            let mut s: Option<String> = None;
                            if let Some(st) = val.as_str() {
                                s = Some(st.to_string());
                            } else if let Some(vobj) = val.as_object() {
                                if let Some(Value::String(st)) = vobj.get("value") {
                                    s = Some(st.clone());
                                }
                            }
                            if let Some(name) = s {
                                // Accept only names with dot separator
                                if name.contains('.') {
                                    reg_name.insert(dst, name);
                                }
                            }
                        }
                    }
                }
            }
            // Second pass: rewrite calls
            for inst in insts.iter_mut() {
                let Some(obj) = inst.as_object_mut() else {
                    continue;
                };
                if obj.get("op").and_then(Value::as_str) != Some("call") {
                    continue;
                }
                let Some(func_reg) = obj.get("func").and_then(Value::as_i64) else {
                    continue;
                };
                let Some(name) = reg_name.get(&func_reg).cloned() else {
                    continue;
                };
                // Split Box.method[/N]
                let mut parts = name.split('.');
                let box_name = match parts.next() {
                    Some(x) => x,
                    None => continue,
                };
                let rest = match parts.next() {
                    Some(x) => x,
                    None => continue,
                };
                let method = rest.split('/').next().unwrap_or(rest);

                // Build mir_call object
                let args = obj.get("args").cloned().unwrap_or(Value::Array(vec![]));
                let dst = obj.get("dst").cloned();

                let mut callee = serde_json::Map::new();
                callee.insert("type".to_string(), Value::String("Method".into()));
                callee.insert("box_name".to_string(), Value::String(box_name.to_string()));
                callee.insert("method".to_string(), Value::String(method.to_string()));

                let mut mir_call = serde_json::Map::new();
                mir_call.insert("callee".to_string(), Value::Object(callee));
                mir_call.insert("args".to_string(), args);
                mir_call.insert("effects".to_string(), Value::Array(vec![]));

                obj.insert("op".to_string(), Value::String("mir_call".into()));
                if let Some(d) = dst {
                    obj.insert("dst".to_string(), d);
                }
                obj.remove("func");
                obj.insert("mir_call".to_string(), Value::Object(mir_call));
                changed = true;
            }
        }
    }
    Ok(changed)
}

fn transform_module_function(inst: &mut Value) -> Result<bool, String> {
    let obj = match inst.as_object_mut() {
        Some(map) => map,
        None => return Ok(false),
    };
    match obj.get("op").and_then(Value::as_str) {
        Some("mir_call") => {}
        _ => return Ok(false),
    }
    let mir_call = match obj.get_mut("mir_call").and_then(Value::as_object_mut) {
        Some(mc) => mc,
        None => return Ok(false),
    };
    let name_owned = {
        let callee_obj = match mir_call.get("callee").and_then(Value::as_object) {
            Some(c) => c,
            None => return Ok(false),
        };
        match callee_obj.get("type").and_then(Value::as_str) {
            Some("ModuleFunction") => {}
            _ => return Ok(false),
        }
        callee_obj
            .get("name")
            .and_then(Value::as_str)
            .ok_or_else(|| "bridge canonicalize: ModuleFunction missing name".to_string())?
            .to_string()
    };
    let name = name_owned.as_str();

    match name {
        "ArrayBox.len" | "MapBox.len" => {
            rewrite_array_like_length(mir_call, name)?;
            Ok(true)
        }
        _ => {
            if let Some((box_name, method)) = static_singleton_method(name) {
                rewrite_static_singleton_call(mir_call, box_name, method)?;
                Ok(true)
            } else {
                Ok(false)
            }
        }
    }
}

fn rewrite_array_like_length(
    mir_call: &mut serde_json::Map<String, Value>,
    name: &str,
) -> Result<(), String> {
    let receiver_value = mir_call
        .get_mut("args")
        .and_then(Value::as_array_mut)
        .ok_or_else(|| "bridge canonicalize: mir_call.args missing".to_string())?;
    if receiver_value.is_empty() {
        return Err(format!(
            "bridge canonicalize: {} requires receiver argument",
            name
        ));
    }
    let receiver_value = receiver_value.remove(0);
    let receiver = receiver_value
        .as_u64()
        .ok_or_else(|| format!("bridge canonicalize: {} receiver must be integer", name))?;

    let num = serde_json::Number::from_u128(receiver as u128)
        .ok_or_else(|| "bridge canonicalize: receiver out of range".to_string())?;

    let callee = mir_call
        .get_mut("callee")
        .and_then(Value::as_object_mut)
        .ok_or_else(|| "bridge canonicalize: callee missing".to_string())?;
    let (box_name, method) = match name {
        "ArrayBox.len" => ("ArrayBox", "size"),
        "MapBox.len" => ("MapBox", "len"),
        _ => unreachable!(),
    };
    callee.insert("type".to_string(), Value::String("Method".into()));
    callee.insert("method".to_string(), Value::String(method.into()));
    callee.insert("box_name".to_string(), Value::String(box_name.into()));
    callee.insert("receiver".to_string(), Value::Number(num));
    callee.remove("name");
    if !callee.contains_key("certainty") {
        callee.insert("certainty".to_string(), Value::String("Known".into()));
    }

    Ok(())
}

const STATIC_SINGLETON_METHODS: &[(&str, &str)] = &[
    ("PhiInst", "lower_phi"),
    ("ConstInst", "lower_const"),
    ("BinOpInst", "lower_binop"),
    ("CompareInst", "lower_compare"),
    ("BranchInst", "lower_branch"),
    ("JumpInst", "lower_jump"),
    ("ReturnInst", "lower_return"),
    ("LLVMPhiInstructionBox", "lower_phi"),
    ("LLVMConstInstructionBox", "lower_const"),
    ("LLVMBinOpInstructionBox", "lower_binop"),
    ("LLVMCompareInstructionBox", "lower_compare"),
    ("LLVMBranchInstructionBox", "lower_branch"),
    ("LLVMJumpInstructionBox", "lower_jump"),
    ("LLVMReturnInstructionBox", "lower_return"),
];

fn static_singleton_method(name: &str) -> Option<(&'static str, &'static str)> {
    STATIC_SINGLETON_METHODS
        .iter()
        .find(|(box_name, method)| name == format!("{}.{}", box_name, method))
        .copied()
}

fn rewrite_static_singleton_call(
    mir_call: &mut serde_json::Map<String, Value>,
    box_name: &'static str,
    method: &'static str,
) -> Result<(), String> {
    let callee = mir_call
        .get_mut("callee")
        .and_then(Value::as_object_mut)
        .ok_or_else(|| "bridge canonicalize: callee missing".to_string())?;

    callee.insert("type".to_string(), Value::String("Method".into()));
    callee.insert("box_name".to_string(), Value::String(box_name.into()));
    callee.insert("method".to_string(), Value::String(method.into()));
    callee.remove("name");
    callee.remove("module");
    callee.remove("receiver");
    if !callee.contains_key("certainty") {
        callee.insert("certainty".to_string(), Value::String("Known".into()));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    fn with_env<F: FnOnce() -> R, R>(key: &str, val: &str, f: F) -> R {
        let prev = env::var(key).ok();
        env::set_var(key, val);
        let result = f();
        if let Some(prev_val) = prev {
            env::set_var(key, prev_val);
        } else {
            env::remove_var(key);
        }
        result
    }

    #[test]
    fn singleton_injection_disabled_noop() {
        env::remove_var("HAKO_BRIDGE_INJECT_SINGLETON");
        env::remove_var("NYASH_BRIDGE_INJECT_SINGLETON");
        let input = r#"{"functions":[{"blocks":[{"instructions":[{"op":"mir_call","mir_call":{"callee":{"type":"ModuleFunction","name":"LLVMPhiInstructionBox.lower_phi"},"args":[1,2]}}]}]}]}"#;
        let output = canonicalize_module_json(input).expect("canonicalize");
        assert_eq!(output, input);
    }

    #[test]
    fn singleton_injection_enabled_rewrites_static_box() {
        let input = r#"{"functions":[{"blocks":[{"instructions":[{"op":"mir_call","mir_call":{"callee":{"type":"ModuleFunction","name":"LLVMPhiInstructionBox.lower_phi"},"args":[1,2]}}]}]}]}"#;
        let output = with_env("HAKO_BRIDGE_INJECT_SINGLETON", "1", || {
            canonicalize_module_json(input).expect("canonicalize")
        });
        let value: Value = serde_json::from_str(&output).expect("json");
        let callee = value["functions"][0]["blocks"][0]["instructions"][0]["mir_call"]["callee"]
            .as_object()
            .expect("callee object");
        assert_eq!(callee.get("type").and_then(Value::as_str), Some("Method"));
        assert_eq!(
            callee.get("box_name").and_then(Value::as_str),
            Some("LLVMPhiInstructionBox")
        );
        assert_eq!(
            callee.get("method").and_then(Value::as_str),
            Some("lower_phi")
        );
        assert!(callee.get("name").is_none());
    }
}

fn materialize_phi_blocks(root: &mut Value) -> Result<bool, String> {
    let mut changed = false;
    let functions = match root.as_object_mut() {
        Some(obj) => obj.get_mut("functions"),
        None => return Err("bridge canonicalize: expected JSON object at root".into()),
    };
    let functions = match functions {
        Some(Value::Array(arr)) => arr,
        Some(_) => return Err("bridge canonicalize: functions must be array".into()),
        None => return Ok(false),
    };

    for func in functions.iter_mut() {
        let blocks = func.get_mut("blocks").and_then(Value::as_array_mut);
        let Some(blocks) = blocks else { continue };
        for block in blocks.iter_mut() {
            let insts = block.get_mut("instructions").and_then(Value::as_array_mut);
            let Some(insts) = insts else { continue };
            if reorder_block_phi(insts)? {
                changed = true;
            }
        }
    }

    Ok(changed)
}

fn reorder_block_phi(insts: &mut Vec<Value>) -> Result<bool, String> {
    let mut seen_non_phi = false;
    let mut needs_reorder = false;
    for inst in insts.iter() {
        if is_phi(inst) {
            if seen_non_phi {
                needs_reorder = true;
                break;
            }
        } else {
            seen_non_phi = true;
        }
    }
    if !needs_reorder {
        return Ok(false);
    }

    let original = std::mem::take(insts);
    let mut phis = Vec::new();
    let mut others = Vec::new();
    for inst in original.into_iter() {
        if is_phi(&inst) {
            phis.push(inst);
        } else {
            others.push(inst);
        }
    }
    insts.extend(phis);
    insts.extend(others);
    Ok(true)
}

fn is_phi(inst: &Value) -> bool {
    inst.as_object()
        .and_then(|obj| obj.get("op"))
        .and_then(Value::as_str)
        .map(|op| op == "phi")
        .unwrap_or(false)
}
