//! Shared runtime-side `env.mirbuilder.emit` bridge helpers.
//!
//! This module keeps the runtime/plugin-side `Program(JSON v0) -> MIR(JSON)` bridge
//! on one owner so interpreter/provider paths and plugin-loader paths do not each
//! reimplement env-import parsing or direct lowering/emit glue.

use crate::mir::MirModule;
use serde_json::Value as JsonValue;
use std::collections::BTreeMap;

pub fn imports_from_env() -> BTreeMap<String, String> {
    if let Ok(imports_json) = std::env::var("HAKO_MIRBUILDER_IMPORTS") {
        match serde_json::from_str::<BTreeMap<String, String>>(&imports_json) {
            Ok(map) => map,
            Err(e) => {
                crate::runtime::get_global_ring0().log.error(&format!(
                    "[mirbuilder/imports] Failed to parse HAKO_MIRBUILDER_IMPORTS: {}",
                    e
                ));
                BTreeMap::new()
            }
        }
    } else {
        BTreeMap::new()
    }
}

pub fn emit_program_json_to_mir_json_with_env_imports(
    program_json: &str,
) -> Result<String, String> {
    let _env_guard = crate::host_providers::mir_builder::Phase0MirJsonEnvGuard::new();
    let module = lower_input_json_to_module(program_json, imports_from_env())?;
    crate::host_providers::mir_builder::module_to_mir_json(&module)
}

fn lower_input_json_to_module(
    input_json: &str,
    imports: BTreeMap<String, String>,
) -> Result<MirModule, String> {
    let parsed = parse_input_json(input_json)?;
    if parsed.get("version").is_some() && parsed.get("kind").is_some() {
        crate::runner::json_v0_bridge::parse_json_v0_to_module_with_imports(input_json, imports)
            .map_err(crate::host_providers::mir_builder::failfast_error)
    } else {
        lower_ast_json_to_module(&parsed)
    }
}

fn parse_input_json(input_json: &str) -> Result<JsonValue, String> {
    serde_json::from_str(input_json).map_err(|error| {
        crate::host_providers::mir_builder::failfast_error(format!("invalid JSON: {}", error))
    })
}

fn lower_ast_json_to_module(parsed: &JsonValue) -> Result<MirModule, String> {
    let Some(ast) = crate::r#macro::ast_json::json_to_ast(parsed) else {
        return Err(crate::host_providers::mir_builder::failfast_error(
            "unsupported JSON input (expected Program(JSON v0) or AST JSON)",
        ));
    };

    let mut builder = crate::mir::builder::MirBuilder::new();
    builder
        .build_module(ast)
        .map_err(crate::host_providers::mir_builder::failfast_error)
}

#[cfg(test)]
mod tests {
    use super::emit_program_json_to_mir_json_with_env_imports;
    use crate::parser::NyashParser;

    #[test]
    fn env_mirbuilder_emit_accepts_ast_json_roundtrip() {
        let _ = crate::runtime::ring0::ensure_global_ring0_initialized();
        let src = r#"
static box Main {
  main() {
    print(0)
    return 0
  }
}
"#;
        let ast = NyashParser::parse_from_string(src).expect("parse");
        let ast_json = crate::r#macro::ast_json::ast_to_json_roundtrip(&ast).to_string();

        let mir_json =
            emit_program_json_to_mir_json_with_env_imports(&ast_json).expect("mir json from ast");

        assert!(mir_json.contains("\"functions\""));
        assert!(!mir_json.is_empty());
    }
}
