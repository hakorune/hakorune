/*!
 * core_bridge.rs — NyVM wrapper bridge helpers
 *
 * Provides a JSON canonicalizer for NyVmDispatcher wrapper path.
 * Optional env toggle:
 *   - HAKO_BRIDGE_EARLY_PHI_MATERIALIZE / NYASH_BRIDGE_EARLY_PHI_MATERIALIZE:
 *       Move phi instructions to block head (order-preserving).
 *   - HAKO_BRIDGE_METHODIZE / NYASH_BRIDGE_METHODIZE:
 *       Retired; requests fail before JSON canonicalization.
 * Dumps payload when `HAKO_DEBUG_NYVM_BRIDGE_DUMP` is set to a file path.
 */

use serde_json::Value;
use std::{env, fs};

pub fn canonicalize_module_json(input: &str) -> Result<String, String> {
    ensure_methodize_disabled()?;
    ensure_singleton_injection_disabled()?;
    let mut output = input.to_string();

    if let Ok(path) = env::var("HAKO_DEBUG_NYVM_BRIDGE_DUMP") {
        if !path.trim().is_empty() {
            if let Err(e) = fs::write(&path, input.as_bytes()) {
                // Phase 98: ConsoleService if available, otherwise eprintln
                crate::console_println!("[bridge/dump] write error: {}", e);
            }
        }
    }

    let materialize_phi = env_flag("HAKO_BRIDGE_EARLY_PHI_MATERIALIZE")
        || env_flag("NYASH_BRIDGE_EARLY_PHI_MATERIALIZE");
    if materialize_phi {
        let mut json: Value = serde_json::from_str(input)
            .map_err(|e| format!("bridge canonicalize: invalid JSON ({})", e))?;
        let mut mutated = false;
        if materialize_phi {
            mutated |= materialize_phi_blocks(&mut json)?;
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

pub(crate) fn ensure_methodize_disabled() -> Result<(), String> {
    if env_flag("HAKO_BRIDGE_METHODIZE") || env_flag("NYASH_BRIDGE_METHODIZE") {
        return Err(
            "[freeze:contract][mir-json-bridge/methodize-retired] methodize compatibility reissuer is retired"
                .to_owned(),
        );
    }
    Ok(())
}

pub(crate) fn ensure_singleton_injection_disabled() -> Result<(), String> {
    if env_flag("HAKO_BRIDGE_INJECT_SINGLETON")
        || env_flag("NYASH_BRIDGE_INJECT_SINGLETON")
    {
        return Err(
            "[freeze:contract][mir-json-bridge/singleton-injection-retired] singleton compatibility reissuer is retired"
                .to_owned(),
        );
    }
    Ok(())
}

fn env_flag(name: &str) -> bool {
    env::var(name)
        .ok()
        .map(|v| matches!(v.trim().to_ascii_lowercase().as_str(), "1" | "true" | "on"))
        .unwrap_or(false)
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
        env::remove_var("HAKO_BRIDGE_METHODIZE");
        env::remove_var("NYASH_BRIDGE_METHODIZE");
        let input = r#"{"functions":[{"blocks":[{"instructions":[{"op":"mir_call","mir_call":{"callee":{"type":"ModuleFunction","name":"LLVMPhiInstructionBox.lower_phi"},"args":[1,2]}}]}]}]}"#;
        let output = canonicalize_module_json(input).expect("canonicalize");
        assert_eq!(output, input);

        let error = with_env("HAKO_BRIDGE_INJECT_SINGLETON", "1", || {
            canonicalize_module_json(input).expect_err("retired singleton must stop")
        });
        assert!(error.contains(
            "[freeze:contract][mir-json-bridge/singleton-injection-retired]"
        ));
    }

    #[test]
    fn singleton_injection_enabled_rewrites_static_box() {
        let input = r#"{"functions":[{"blocks":[{"instructions":[{"op":"mir_call","mir_call":{"callee":{"type":"ModuleFunction","name":"LLVMPhiInstructionBox.lower_phi"},"args":[1,2]}}]}]}]}"#;
        let error = with_env("NYASH_BRIDGE_INJECT_SINGLETON", "1", || {
            canonicalize_module_json(input).expect_err("retired singleton alias must stop")
        });
        assert!(error.contains(
            "[freeze:contract][mir-json-bridge/singleton-injection-retired]"
        ));
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
