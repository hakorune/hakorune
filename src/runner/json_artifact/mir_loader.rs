use crate::mir::MirModule;

pub(super) fn load_mir_json_to_module(text: &str) -> Result<Option<MirModule>, String> {
    match crate::runner::json_v1_bridge::try_parse_v1_to_module(text) {
        Ok(Some(module)) => return Ok(Some(module)),
        Ok(None) => {}
        Err(error) => return Err(format!("JSON v1 bridge error: {}", error)),
    }

    if looks_like_mir_v0(text) {
        return crate::runner::mir_json_v0::parse_mir_v0_to_module(text)
            .map(Some)
            .map_err(|error| format!("MIR JSON v0 parse error: {}", error));
    }

    Ok(None)
}

pub(super) fn parse_direct_mir_json_text(text: &str, path: &str) -> Result<MirModule, String> {
    match crate::runner::json_v1_bridge::try_parse_v1_to_module(text) {
        Ok(Some(module)) => Ok(module),
        Ok(None) => {
            if looks_like_mir_v0(text) {
                crate::runner::mir_json_v0::parse_mir_v0_to_module(text)
                    .map_err(|error| format!("v0({}): {}", path, error))
            } else {
                Err(format!("unsupported shape ({})", path))
            }
        }
        Err(error_v1) => Err(format!("v1({}): {}", path, error_v1)),
    }
}

fn looks_like_mir_v0(text: &str) -> bool {
    text.contains("\"functions\"") && text.contains("\"blocks\"")
}

#[cfg(test)]
mod tests {
    use super::load_mir_json_to_module;

    #[test]
    fn load_mir_json_to_module_returns_none_for_program_json_v0() {
        let program_json = r#"{
            "version": 0,
            "kind": "Program",
            "body": [
                {"type": "Return", "expr": {"type": "Int", "value": 1}}
            ]
        }"#;

        let result = load_mir_json_to_module(program_json).expect("program json should not error");
        assert!(result.is_none());
    }

    #[test]
    fn load_mir_json_to_module_accepts_mir_json_v0() {
        let mir_json = r#"{
            "functions": [
                {
                    "name": "main",
                    "blocks": [
                        {
                            "id": 0,
                            "instructions": [
                                {"op": "ret"}
                            ]
                        }
                    ]
                }
            ]
        }"#;

        let result = load_mir_json_to_module(mir_json).expect("mir json should parse");
        assert!(result.is_some());
    }

    #[test]
    fn load_mir_json_to_module_rejects_escaped_declared_schema_before_v0() {
        let payload = r#"{
            "\u0073chema_version": "2.0",
            "functions": [{"name": "main", "blocks": [{"id": 0, "instructions": []}]}]
        }"#;

        let error = load_mir_json_to_module(payload)
            .expect_err("escaped declared schemas must be terminal");
        assert!(error.contains("JSON v1 bridge error"));
        assert!(error.contains("unsupported schema_version"));
    }

    #[test]
    fn direct_mir_json_rejects_declared_v1_boxcall_without_v0_retry() {
        let payload = r#"{
            "schema_version": "1.0",
            "functions": [{
                "name": "main",
                "blocks": [{
                    "id": 0,
                    "instructions": [{"op": "boxcall", "box": 0, "method": "run", "args": []}]
                }]
            }]
        }"#;

        let error = super::parse_direct_mir_json_text(payload, "<declared-v1>")
            .expect_err("declared v1 errors must not re-enter v0");
        assert!(error.contains("v1(<declared-v1>)"));
        assert!(error.contains("unsupported"), "unexpected error: {error}");
    }

    #[test]
    fn direct_mir_json_rejects_unsupported_declared_schema() {
        let payload = r#"{
            "schema_version": "2.0",
            "functions": [{"name": "main", "blocks": [{"id": 0, "instructions": []}]}]
        }"#;

        let error = super::parse_direct_mir_json_text(payload, "<schema-2>")
            .expect_err("unsupported declared schemas must be terminal");
        assert!(error.contains("unsupported schema_version"));
    }

    #[test]
    fn direct_mir_json_rejects_non_string_schema() {
        let payload = r#"{
            "schema_version": 1,
            "functions": [{"name": "main", "blocks": [{"id": 0, "instructions": []}]}]
        }"#;

        let error = super::parse_direct_mir_json_text(payload, "<schema-type>")
            .expect_err("non-string schemas must be terminal");
        assert!(error.contains("expected schema_version string"));
    }

    #[test]
    fn direct_mir_json_rejects_malformed_json_before_v0() {
        let error = super::parse_direct_mir_json_text(
            r#"{"schema_version":"1.0","functions":["#,
            "<malformed>",
        )
        .expect_err("malformed JSON must not enter v0");
        assert!(error.contains("invalid JSON"));
    }
}
