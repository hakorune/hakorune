use std::collections::BTreeSet;

pub(super) fn program_json_to_mir_json_with_user_box_decls(
    program_json: &str,
) -> Result<String, String> {
    let mir_json = super::lowering::program_json_to_mir_json(program_json)?;
    inject_stage1_user_box_decls_from_program_json(program_json, &mir_json)
}

pub(super) fn inject_stage1_user_box_decls_from_program_json(
    program_json: &str,
    mir_json: &str,
) -> Result<String, String> {
    let mut mir_value: serde_json::Value =
        serde_json::from_str(mir_json).map_err(|error| format!("mir json parse error: {}", error))?;
    let mir_object = mir_value
        .as_object_mut()
        .ok_or_else(|| "mir json root must be object".to_string())?;
    mir_object.insert(
        "user_box_decls".to_string(),
        serde_json::Value::Array(stage1_user_box_decls_from_program_json(program_json)?),
    );
    serde_json::to_string(&mir_value)
        .map_err(|error| format!("mir json serialize error: {}", error))
}

fn stage1_user_box_decls_from_program_json(
    program_json: &str,
) -> Result<Vec<serde_json::Value>, String> {
    let program_value: serde_json::Value = serde_json::from_str(program_json)
        .map_err(|error| format!("program json parse error: {}", error))?;
    let mut seen = BTreeSet::new();
    seen.insert("Main".to_string());
    if let Some(defs) = program_value
        .get("defs")
        .and_then(serde_json::Value::as_array)
    {
        for def in defs {
            if let Some(box_name) = def.get("box").and_then(serde_json::Value::as_str) {
                if !box_name.is_empty() {
                    seen.insert(box_name.to_string());
                }
            }
        }
    }
    Ok(seen
        .into_iter()
        .map(|name| serde_json::json!({ "name": name, "fields": [] }))
        .collect())
}
