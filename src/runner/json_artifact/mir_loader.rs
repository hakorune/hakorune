use crate::mir::MirModule;

pub(super) fn load_mir_json_to_module(text: &str) -> Result<Option<MirModule>, String> {
    if text.contains("\"schema_version\"") {
        match crate::runner::json_v1_bridge::try_parse_v1_to_module(text) {
            Ok(Some(module)) => return Ok(Some(module)),
            Ok(None) => {}
            Err(error) => return Err(format!("JSON v1 bridge error: {}", error)),
        }
    }

    if looks_like_mir_v0(text) {
        return crate::runner::mir_json_v0::parse_mir_v0_to_module(text)
            .map(Some)
            .map_err(|error| format!("MIR JSON v0 parse error: {}", error));
    }

    Ok(None)
}

pub(super) fn parse_direct_mir_json_text_with_v0_fallback(
    text: &str,
    path: &str,
) -> Result<MirModule, String> {
    let looks_like_v0 = looks_like_mir_v0(text);
    match crate::runner::json_v1_bridge::try_parse_v1_to_module(text) {
        Ok(Some(module)) => Ok(module),
        Ok(None) => {
            if looks_like_v0 {
                crate::runner::mir_json_v0::parse_mir_v0_to_module(text)
                    .map_err(|error| format!("v0({}): {}", path, error))
            } else {
                Err(format!("unsupported shape ({})", path))
            }
        }
        Err(error_v1) => {
            if looks_like_v0 {
                match crate::runner::mir_json_v0::parse_mir_v0_to_module(text) {
                    Ok(module) => Ok(module),
                    Err(error_v0) => Err(format!(
                        "v1({}): {}; v0({}): {}",
                        path, error_v1, path, error_v0
                    )),
                }
            } else {
                Err(format!("v1({}): {}", path, error_v1))
            }
        }
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
}
