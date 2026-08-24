use serde_json::{json, Value};

pub(super) fn normalize_program_json_bridge_backend_shape(
    mir_json: &str,
) -> Result<String, String> {
    let mut root: Value = serde_json::from_str(mir_json)
        .map_err(|error| format!("bridge backend-shape json parse failed: {error}"))?;
    let changed = normalize_console_print_externcalls(&mut root)?;
    if !changed {
        return Ok(mir_json.to_string());
    }
    serde_json::to_string(&root)
        .map_err(|error| format!("bridge backend-shape json serialize failed: {error}"))
}

fn normalize_console_print_externcalls(root: &mut Value) -> Result<bool, String> {
    let functions = root
        .get_mut("functions")
        .and_then(Value::as_array_mut)
        .ok_or_else(|| "bridge backend-shape missing functions array".to_string())?;
    let mut changed = false;
    for (function_index, function) in functions.iter_mut().enumerate() {
        let blocks = function
            .get_mut("blocks")
            .and_then(Value::as_array_mut)
            .ok_or_else(|| {
                format!("bridge backend-shape function[{function_index}] missing blocks array")
            })?;
        for (block_index, block) in blocks.iter_mut().enumerate() {
            let instructions = block
                .get_mut("instructions")
                .and_then(Value::as_array_mut)
                .ok_or_else(|| {
                    format!(
                        "bridge backend-shape function[{function_index}] block[{block_index}] missing instructions array"
                    )
                })?;
            for (instruction_index, inst) in instructions.iter_mut().enumerate() {
                if normalize_console_print_externcall(inst).map_err(|error| {
                    format!(
                        "bridge backend-shape function[{function_index}] block[{block_index}] instruction[{instruction_index}]: {error}"
                    )
                })? {
                    changed = true;
                }
            }
        }
    }
    Ok(changed)
}

fn normalize_console_print_externcall(inst: &mut Value) -> Result<bool, String> {
    let Some(obj) = inst.as_object() else {
        return Err("instruction must be an object".to_string());
    };
    let Some(op) = obj.get("op").and_then(Value::as_str) else {
        return Err("instruction missing string op".to_string());
    };
    if op != "externcall" {
        return Ok(false);
    }
    let Some(func) = obj.get("func").and_then(Value::as_str) else {
        return Err("externcall missing string func".to_string());
    };
    let Some(dst) = obj.get("dst") else {
        return Err("externcall missing dst".to_string());
    };
    if !dst.is_null() && dst.as_u64().is_none() {
        return Err("externcall dst must be an integer or null".to_string());
    }
    let Some(args) = obj.get("args").and_then(Value::as_array) else {
        return Err("externcall args must be an array".to_string());
    };
    if args.iter().any(|arg| arg.as_u64().is_none()) {
        return Err("externcall args must contain only integers".to_string());
    }
    for key in obj.keys() {
        if !matches!(key.as_str(), "op" | "func" | "args" | "dst") {
            return Err(format!("externcall has unsupported field '{key}'"));
        }
    }
    if func != "nyash.console.log" && func != "env.console.log" {
        return Ok(false);
    }

    let dst = dst.clone();
    let args = Value::Array(args.to_vec());
    *inst = json!({
        "op": "mir_call",
        "dst": dst,
        "mir_call": {
            "callee": { "type": "Global", "name": "print" },
            "args": args,
            "effects": ["IO"],
            "flags": {}
        }
    });
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::normalize_program_json_bridge_backend_shape;

    #[test]
    fn normalizes_console_log_externcall_to_global_print_mir_call() {
        let input = r#"{
          "functions": [{
            "name": "main",
            "blocks": [{
              "id": 0,
              "instructions": [
                {"op":"const","dst":1,"value":{"type":"i64","value":42}},
                {"op":"externcall","func":"nyash.console.log","args":[1],"dst":null},
                {"op":"ret","value":1}
              ]
            }]
          }]
        }"#;

        let out = normalize_program_json_bridge_backend_shape(input).expect("normalize");
        let parsed: serde_json::Value = serde_json::from_str(&out).expect("json");
        let inst = &parsed["functions"][0]["blocks"][0]["instructions"][1];
        assert_eq!(inst["op"].as_str(), Some("mir_call"));
        assert_eq!(
            inst["mir_call"]["callee"],
            serde_json::json!({"type": "Global", "name": "print"})
        );
        assert_eq!(inst["mir_call"]["args"], serde_json::json!([1]));
        assert_eq!(inst["mir_call"]["effects"], serde_json::json!(["IO"]));
    }

    #[test]
    fn leaves_non_console_externcall_unchanged() {
        let input = r#"{"functions":[{"blocks":[{"instructions":[{"op":"externcall","func":"env.get","args":[1],"dst":2}]}]}]}"#;

        let out = normalize_program_json_bridge_backend_shape(input).expect("normalize");
        let parsed: serde_json::Value = serde_json::from_str(&out).expect("json");
        let inst = &parsed["functions"][0]["blocks"][0]["instructions"][0];
        assert_eq!(inst["op"].as_str(), Some("externcall"));
        assert_eq!(inst["func"].as_str(), Some("env.get"));
    }

    #[test]
    fn rejects_console_externcall_with_defaultable_fields_missing() {
        let missing_dst = r#"{"functions":[{"blocks":[{"instructions":[{"op":"externcall","func":"nyash.console.log","args":[1]}]}]}]}"#;
        let missing_args = r#"{"functions":[{"blocks":[{"instructions":[{"op":"externcall","func":"nyash.console.log","dst":null}]}]}]}"#;

        for input in [missing_dst, missing_args] {
            let error = normalize_program_json_bridge_backend_shape(input)
                .expect_err("missing adapter fields must reject");
            assert!(
                error.contains("externcall missing") || error.contains("args must be"),
                "unexpected error: {error}"
            );
        }
    }

    #[test]
    fn rejects_console_externcall_with_malformed_values() {
        let input = r#"{"functions":[{"blocks":[{"instructions":[{"op":"externcall","func":"nyash.console.log","args":["not-a-value"],"dst":null}]}]}]}"#;

        let error = normalize_program_json_bridge_backend_shape(input)
            .expect_err("non-numeric args must reject");
        assert!(error.contains("args must contain only integers"), "{error}");
    }

    #[test]
    fn rejects_externcall_with_unowned_extra_fields() {
        let input = r#"{"functions":[{"blocks":[{"instructions":[{"op":"externcall","func":"env.get","args":[1],"dst":2,"effects":["IO"]}]}]}]}"#;

        let error = normalize_program_json_bridge_backend_shape(input)
            .expect_err("extra fields must reject");
        assert!(error.contains("unsupported field 'effects'"), "{error}");
    }

    #[test]
    fn rejects_malformed_bridge_containers() {
        let input = r#"{"functions":[{"blocks":[{}]}]}"#;

        let error = normalize_program_json_bridge_backend_shape(input)
            .expect_err("missing instructions must reject");
        assert!(error.contains("missing instructions array"), "{error}");
    }
}
