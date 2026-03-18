use serde_json::json;

pub(super) fn validate_backend_mir_shape(mir_json: &str) -> Result<(), String> {
    if !mir_json.contains("\"functions\"") || !mir_json.contains("\"blocks\"") {
        let tag = "[llvmemit/input/invalid] missing functions/blocks keys";
        llvm_emit_error!("{}", tag);
        return Err(tag.into());
    }
    Ok(())
}

pub(super) fn normalize_mir_json_for_backend(mir_json: &str) -> Result<String, String> {
    let mut value: serde_json::Value = serde_json::from_str(mir_json)
        .map_err(|e| format!("[llvmemit/input/invalid-json] {}", e))?;
    let root = value
        .as_object_mut()
        .ok_or_else(|| "[llvmemit/input/invalid-root] expected object".to_string())?;
    root.insert(
        "kind".to_string(),
        serde_json::Value::String("MIR".to_string()),
    );
    root.insert(
        "schema_version".to_string(),
        serde_json::Value::String("1.0".to_string()),
    );
    match root.get_mut("metadata") {
        Some(serde_json::Value::Object(metadata)) => {
            metadata
                .entry("extern_c".to_string())
                .or_insert_with(|| json!([]));
        }
        _ => {
            root.insert("metadata".to_string(), json!({ "extern_c": [] }));
        }
    }
    serde_json::to_string(&value).map_err(|e| format!("[llvmemit/input/serialize-failed] {}", e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_backend_mir_shape_rejects_missing_keys() {
        assert!(validate_backend_mir_shape("{}").is_err());
    }

    #[test]
    fn normalize_mir_json_for_backend_sets_kind_schema_and_metadata() {
        let out = normalize_mir_json_for_backend(r#"{"functions":[]}"#).unwrap();
        let value: serde_json::Value = serde_json::from_str(&out).unwrap();
        assert_eq!(value["kind"], "MIR");
        assert_eq!(value["schema_version"], "1.0");
        assert_eq!(value["metadata"]["extern_c"], serde_json::json!([]));
    }
}
