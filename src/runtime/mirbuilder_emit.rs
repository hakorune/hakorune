//! Shared runtime-side `env.mirbuilder.emit` Program(JSON v0) bridge.
//!
//! Interpreter/provider and plugin-loader callers share one JSON discriminator,
//! imports snapshot, direct Program lowering, metadata refresh, and MIR emission.
//! Arbitrary AST JSON is outside this runtime API.

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
    let mut module = lower_input_json_to_module(program_json, imports_from_env())?;
    crate::host_providers::mir_builder::refresh_bridge_semantic_metadata(&mut module);
    let mir_json = crate::host_providers::mir_builder::module_to_mir_json(&module)?;
    crate::host_providers::mir_builder::normalize_program_json_bridge_backend_shape(&mir_json)
}

fn lower_input_json_to_module(
    input_json: &str,
    imports: BTreeMap<String, String>,
) -> Result<MirModule, String> {
    let parsed = parse_input_json(input_json)?;
    if parsed.get("version").is_none() || parsed.get("kind").is_none() {
        return Err(crate::host_providers::mir_builder::failfast_error(
            "unsupported JSON input (expected Program(JSON v0))",
        ));
    }
    crate::runner::json_v0_bridge::parse_json_v0_to_module_with_imports(input_json, imports)
        .map_err(crate::host_providers::mir_builder::failfast_error)
}

fn parse_input_json(input_json: &str) -> Result<JsonValue, String> {
    serde_json::from_str(input_json).map_err(|error| {
        crate::host_providers::mir_builder::failfast_error(format!("invalid JSON: {}", error))
    })
}

#[cfg(test)]
mod tests {
    use super::emit_program_json_to_mir_json_with_env_imports;

    #[test]
    fn env_mirbuilder_emit_rejects_ast_json_before_builder() {
        let ast_json = r#"{"schema":"ast_json_roundtrip_v1","schema_version":1,"kind":"Program","statements":[]}"#;
        let error = emit_program_json_to_mir_json_with_env_imports(ast_json)
            .expect_err("AST JSON compatibility is retired");
        assert!(error.contains("expected Program(JSON v0)"), "{error}");
    }

    #[test]
    fn env_mirbuilder_emit_normalizes_console_print_for_backend_boundary() {
        let _ = crate::runtime::ring0::ensure_global_ring0_initialized();
        let program_json = r#"{
            "version": 0,
            "kind": "Program",
            "body": [
                {
                    "type": "Extern",
                    "iface": "env.console",
                    "method": "log",
                    "args": [{ "type": "Int", "value": 42 }]
                },
                { "type": "Return", "expr": { "type": "Int", "value": 0 } }
            ]
        }"#;

        let mir_json =
            emit_program_json_to_mir_json_with_env_imports(program_json).expect("mir json");
        let parsed: serde_json::Value = serde_json::from_str(&mir_json).expect("mir json parses");
        let instructions = parsed["functions"][0]["blocks"][0]["instructions"]
            .as_array()
            .expect("instructions array");

        let console_externcalls = instructions
            .iter()
            .filter(|inst| {
                inst["op"] == "externcall" && inst["func"].as_str() == Some("nyash.console.log")
            })
            .count();
        assert_eq!(console_externcalls, 0);

        let print_calls = instructions
            .iter()
            .filter(|inst| {
                inst["op"] == "mir_call"
                    && inst["mir_call"]["callee"]
                        == serde_json::json!({"type": "Global", "name": "print"})
            })
            .count();
        assert_eq!(print_calls, 1);
    }

    #[test]
    fn env_mirbuilder_emit_refreshes_global_call_routes() {
        let _ = crate::runtime::ring0::ensure_global_ring0_initialized();
        let program_json = r#"{
            "version": 0,
            "kind": "Program",
            "defs": [
                {
                    "box": "HelperBox",
                    "name": "label",
                    "params": [],
                    "body": {"version":0,"kind":"Program","body":[{"type":"Return","expr":{"type":"Str","value":"ok"}}]}
                }
            ],
            "body": [
                {
                    "type": "Return",
                    "expr": {"type": "Call", "name": "HelperBox.label", "args": []}
                }
            ]
        }"#;

        let mir_json =
            emit_program_json_to_mir_json_with_env_imports(program_json).expect("mir json");
        let parsed: serde_json::Value = serde_json::from_str(&mir_json).expect("mir json parses");
        let main_fn = parsed["functions"]
            .as_array()
            .and_then(|functions| {
                functions
                    .iter()
                    .find(|function| function["name"].as_str() == Some("main"))
            })
            .expect("main function");
        let routes = main_fn["metadata"]["global_call_routes"]
            .as_array()
            .expect("global_call_routes array");
        let plans = main_fn["metadata"]["lowering_plan"]
            .as_array()
            .expect("lowering_plan array");

        assert_eq!(routes.len(), 1);
        assert_eq!(routes[0]["target_exists"].as_bool(), Some(true));
        assert_eq!(
            routes[0]["target_shape"].as_str(),
            Some("generic_pure_string_body")
        );
        assert_eq!(plans.len(), 1);
        assert_eq!(plans[0]["source"].as_str(), Some("global_call_routes"));
    }
}
