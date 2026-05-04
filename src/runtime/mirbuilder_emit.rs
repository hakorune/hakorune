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

    #[test]
    fn env_mirbuilder_emit_keeps_rune_attrs_on_selected_entry() {
        let _ = crate::runtime::ring0::ensure_global_ring0_initialized();
        let src = r#"
@rune Public
static box Main {
  @rune FfiSafe
  @rune ReturnsOwned
  @rune FreeWith("cleanup_main")
  @rune Symbol("main_sym")
  @rune CallConv("c")
  @rune Hint(inline)
  @rune Contract(no_alloc)
  @rune IntrinsicCandidate("Main.main/0")
  main() {
    return 0
  }
}
"#;
        let prev = std::env::var("NYASH_FEATURES").ok();
        std::env::set_var("NYASH_FEATURES", "rune");

        let result = (|| {
            let ast = NyashParser::parse_from_string(src).expect("parse");
            let ast_json = crate::r#macro::ast_json::ast_to_json_roundtrip(&ast).to_string();
            emit_program_json_to_mir_json_with_env_imports(&ast_json).expect("mir json from ast")
        })();

        match prev {
            Some(value) => std::env::set_var("NYASH_FEATURES", value),
            None => std::env::remove_var("NYASH_FEATURES"),
        }

        let value: serde_json::Value = serde_json::from_str(&result).expect("mir json must parse");
        let functions = value["functions"].as_array().expect("functions array");
        let main_fn = functions
            .iter()
            .find(|function| function["name"] == "main")
            .expect("selected entry function named main");
        let runes = main_fn["attrs"]["runes"]
            .as_array()
            .expect("attrs.runes array");
        assert_eq!(runes.len(), 8);
        assert_eq!(runes[0]["name"], "FfiSafe");
        assert_eq!(runes[0]["args"], serde_json::json!([]));
        assert_eq!(runes[1]["name"], "ReturnsOwned");
        assert_eq!(runes[1]["args"], serde_json::json!([]));
        assert_eq!(runes[2]["name"], "FreeWith");
        assert_eq!(runes[2]["args"], serde_json::json!(["cleanup_main"]));
        assert_eq!(runes[3]["name"], "Symbol");
        assert_eq!(runes[3]["args"], serde_json::json!(["main_sym"]));
        assert_eq!(runes[4]["name"], "CallConv");
        assert_eq!(runes[4]["args"], serde_json::json!(["c"]));
        assert_eq!(runes[5]["name"], "Hint");
        assert_eq!(runes[5]["args"], serde_json::json!(["inline"]));
        assert_eq!(runes[6]["name"], "Contract");
        assert_eq!(runes[6]["args"], serde_json::json!(["no_alloc"]));
        assert_eq!(runes[7]["name"], "IntrinsicCandidate");
        assert_eq!(runes[7]["args"], serde_json::json!(["Main.main/0"]));
    }
}
