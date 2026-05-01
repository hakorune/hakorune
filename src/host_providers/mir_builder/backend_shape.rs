use serde_json::{json, Value};

pub(super) fn normalize_program_json_bridge_backend_shape(
    mir_json: &str,
) -> Result<String, String> {
    let mut root: Value = serde_json::from_str(mir_json)
        .map_err(|error| format!("bridge backend-shape json parse failed: {error}"))?;
    let changed = normalize_console_print_externcalls(&mut root);
    if !changed {
        return Ok(mir_json.to_string());
    }
    serde_json::to_string(&root)
        .map_err(|error| format!("bridge backend-shape json serialize failed: {error}"))
}

fn normalize_console_print_externcalls(root: &mut Value) -> bool {
    let Some(functions) = root.get_mut("functions").and_then(Value::as_array_mut) else {
        return false;
    };
    let mut changed = false;
    for function in functions {
        let Some(blocks) = function.get_mut("blocks").and_then(Value::as_array_mut) else {
            continue;
        };
        for block in blocks {
            let Some(instructions) = block.get_mut("instructions").and_then(Value::as_array_mut)
            else {
                continue;
            };
            for inst in instructions {
                if normalize_console_print_externcall(inst) {
                    changed = true;
                }
            }
        }
    }
    changed
}

fn normalize_console_print_externcall(inst: &mut Value) -> bool {
    let Some(obj) = inst.as_object() else {
        return false;
    };
    if obj.get("op").and_then(Value::as_str) != Some("externcall") {
        return false;
    }
    let Some(func) = obj.get("func").and_then(Value::as_str) else {
        return false;
    };
    if func != "nyash.console.log" && func != "env.console.log" {
        return false;
    }

    let dst = obj.get("dst").cloned().unwrap_or(Value::Null);
    let args = obj
        .get("args")
        .cloned()
        .unwrap_or_else(|| Value::Array(Vec::new()));
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
    true
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
}
