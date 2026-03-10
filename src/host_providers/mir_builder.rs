use crate::runner;
use serde_json::Value as JsonValue;
use std::collections::BTreeMap;
use std::fs;
// use std::io::Write; // kept for future pretty-print extensions

const FAILFAST_TAG: &str = "[freeze:contract][hako_mirbuilder]";
const TRACE_ENV: &str = "HAKO_STAGE1_MODULE_DISPATCH_TRACE";

fn trace_enabled() -> bool {
    std::env::var(TRACE_ENV).ok().as_deref() == Some("1")
}

fn trace_log(message: impl AsRef<str>) {
    if trace_enabled() {
        eprintln!("{}", message.as_ref());
    }
}

struct ScopedEnvVar {
    key: &'static str,
    prev: Option<String>,
}

impl ScopedEnvVar {
    fn set(key: &'static str, value: &'static str) -> Self {
        let prev = std::env::var(key).ok();
        std::env::set_var(key, value);
        Self { key, prev }
    }
}

impl Drop for ScopedEnvVar {
    fn drop(&mut self) {
        match self.prev.take() {
            Some(v) => std::env::set_var(self.key, v),
            None => std::env::remove_var(self.key),
        }
    }
}

struct Phase0MirJsonEnvGuard {
    _schema_v1: ScopedEnvVar,
    _unified_call: ScopedEnvVar,
}

impl Phase0MirJsonEnvGuard {
    fn new() -> Self {
        Self {
            _schema_v1: ScopedEnvVar::set("NYASH_JSON_SCHEMA_V1", "0"),
            _unified_call: ScopedEnvVar::set("NYASH_MIR_UNIFIED_CALL", "0"),
        }
    }
}

/// Convert Program(JSON v0) to MIR(JSON v0) and return it as a String.
/// Fail-Fast: prints stable tags on stderr and returns Err with the same tag text.
pub fn program_json_to_mir_json(program_json: &str) -> Result<String, String> {
    program_json_to_mir_json_with_imports(program_json, BTreeMap::new())
}

/// Convert source text through the existing stage1 Program(JSON v0) surrogate and
/// return both the transient Program(JSON) and MIR(JSON) while keeping that
/// boundary inside the provider.
pub fn source_to_program_and_mir_json(source_text: &str) -> Result<(String, String), String> {
    let program_json = crate::stage1::program_json_v0::source_to_program_json_v0(source_text)
        .map_err(|e| format!("{FAILFAST_TAG} {}", e))?;
    let mir_json = program_json_to_mir_json(&program_json)?;
    Ok((program_json, mir_json))
}

pub fn source_to_mir_json(source_text: &str) -> Result<String, String> {
    let (_, mir_json) = source_to_program_and_mir_json(source_text)?;
    Ok(mir_json)
}

/// Convert Program(JSON v0) to MIR(JSON v0) with using imports support.
pub fn program_json_to_mir_json_with_imports(
    program_json: &str,
    imports: BTreeMap<String, String>,
) -> Result<String, String> {
    // Phase-0 contract: MIR JSON v0 must be executable via `--mir-json-file` v0 loader.
    // That loader supports `externcall`/`boxcall` but not unified `mir_call` lowering.
    // Therefore we force unified-call OFF for both compilation and emission here.
    let _env_guard = Phase0MirJsonEnvGuard::new();

    // Parse JSON once and route by shape (Fail-Fast; no silent fallback).
    let parsed: JsonValue = serde_json::from_str(program_json).map_err(|e| {
        if trace_enabled() {
            let preview: String = program_json.chars().take(120).collect();
            let prefix_bytes: Vec<u8> = program_json.as_bytes().iter().take(16).copied().collect();
            trace_log(format!(
                "[stage1/mir_builder] invalid_json_preview={:?} prefix_bytes={:?}",
                preview, prefix_bytes
            ));
        }
        let tag = format!("{FAILFAST_TAG} invalid JSON: {}", e);
        crate::runtime::get_global_ring0().log.error(&tag);
        tag
    })?;

    // Route A: Stage-B Program(JSON v0) → MIR Module (via json_v0_bridge)
    let module = if parsed.get("version").is_some() && parsed.get("kind").is_some() {
        match runner::json_v0_bridge::parse_json_v0_to_module_with_imports(program_json, imports) {
            Ok(m) => m,
            Err(e) => {
                let tag = format!("{FAILFAST_TAG} {}", e);
                crate::runtime::get_global_ring0().log.error(&tag);
                return Err(tag);
            }
        }
    } else {
        // Route B (Phase-0 bridge): AST JSON (from `--emit-ast-json`, legacy: `--emit-program-json`) → MIR Module
        // This keeps Phase-0 runnable before Stage-1/Stage-B parser is fully green.
        let Some(ast) = crate::r#macro::ast_json::json_to_ast(&parsed) else {
            let tag = format!(
                "{FAILFAST_TAG} unsupported JSON input (expected Program(JSON v0) or AST JSON)"
            );
            crate::runtime::get_global_ring0().log.error(&tag);
            return Err(tag);
        };

        let mut builder = crate::mir::builder::MirBuilder::new();
        builder.build_module(ast).map_err(|e| {
            let tag = format!("{FAILFAST_TAG} {}", e);
            crate::runtime::get_global_ring0().log.error(&tag);
            tag
        })?
    };

    // Emit MIR(JSON) to a temporary file (reuse existing emitter), then read back
    let tmp_dir = std::env::temp_dir();
    let tmp_path = tmp_dir.join("hako_mirbuilder_out.json");
    let emit_res = runner::mir_json_emit::emit_mir_json_for_harness_bin(&module, &tmp_path);

    if let Err(e) = emit_res {
        let tag = format!("{FAILFAST_TAG} {}", e);
        crate::runtime::get_global_ring0().log.error(&tag);
        return Err(tag);
    }
    match fs::read_to_string(&tmp_path) {
        Ok(raw) => {
            // Best-effort cleanup
            let _ = fs::remove_file(&tmp_path);
            // Phase-0: return MIR JSON v0 as a single line.
            match serde_json::from_str::<JsonValue>(&raw) {
                Ok(v) => Ok(serde_json::to_string(&v).unwrap_or(raw)),
                Err(_) => Ok(raw),
            }
        }
        Err(e) => {
            let tag = format!("{FAILFAST_TAG} {}", e);
            crate::runtime::get_global_ring0().log.error(&tag);
            Err(tag)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_imports_resolution() {
        // Program JSON with MatI64.new(4, 4)
        let program_json = r#"{
            "version": 0,
            "kind": "Program",
            "body": [
                {
                    "type": "Local",
                    "name": "n",
                    "expr": {"type": "Int", "value": 4}
                },
                {
                    "type": "Local",
                    "name": "A",
                    "expr": {
                        "type": "Method",
                        "recv": {"type": "Var", "name": "MatI64"},
                        "method": "new",
                        "args": [
                            {"type": "Var", "name": "n"},
                            {"type": "Var", "name": "n"}
                        ]
                    }
                },
                {
                    "type": "Return",
                    "expr": {
                        "type": "Method",
                        "recv": {"type": "Var", "name": "A"},
                        "method": "at",
                        "args": [
                            {"type": "Int", "value": 0},
                            {"type": "Int", "value": 0}
                        ]
                    }
                }
            ]
        }"#;

        // Create imports map
        let mut imports = BTreeMap::new();
        imports.insert("MatI64".to_string(), "MatI64".to_string());

        // Call with imports
        let result = program_json_to_mir_json_with_imports(program_json, imports);

        // Should succeed
        assert!(result.is_ok(), "Failed with error: {:?}", result.err());

        let mir_json = result.unwrap();
        // MIR JSON should contain functions
        assert!(
            mir_json.contains("functions"),
            "MIR JSON should contain functions"
        );
    }

    #[test]
    fn test_stageb_program_json_with_stagebdriver_main_call() {
        let program_json = r#"{
            "body": [
                {
                    "expr": {
                        "args": [{"name": "args", "type": "Var"}],
                        "name": "StageBDriverBox.main",
                        "type": "Call"
                    },
                    "type": "Return"
                }
            ],
            "kind": "Program",
            "version": 0
        }"#;

        let result = program_json_to_mir_json(program_json);
        assert!(result.is_ok(), "Failed with error: {:?}", result.err());
        let mir_json = result.unwrap();
        assert!(mir_json.contains("functions"));
    }

    #[test]
    fn test_imported_alias_qualified_call_uses_json_imports() {
        let program_json = r#"{
            "version": 0,
            "kind": "Program",
            "imports": {
                "BuildBox": "lang.compiler.build.build_box"
            },
            "body": [
                {
                    "type": "Return",
                    "expr": {
                        "type": "Call",
                        "name": "BuildBox.emit_program_json_v0",
                        "args": [
                            {"type": "Str", "value": "box MainBox { method main() { return 1 } }"},
                            {"type": "Null"}
                        ]
                    }
                }
            ]
        }"#;

        let result = program_json_to_mir_json(program_json);
        assert!(result.is_ok(), "Failed with error: {:?}", result.err());

        let mir_json = result.unwrap();
        assert!(mir_json.contains("lang.compiler.build.build_box"));
        assert!(!mir_json.contains("\"BuildBox.emit_program_json_v0\""));
    }

    #[test]
    fn test_source_to_mir_json_handles_stage1_cli_env_source() {
        let source = include_str!("../../lang/src/runner/stage1_cli_env.hako");
        let result = source_to_mir_json(source);
        assert!(result.is_ok(), "Failed with error: {:?}", result.err());

        let mir_json = result.unwrap();
        assert!(mir_json.contains("functions"));
    }

    #[test]
    fn test_source_to_program_and_mir_json_returns_program_and_mir() {
        let source = include_str!("../../lang/src/runner/stage1_cli_env.hako");
        let result = source_to_program_and_mir_json(source);
        assert!(result.is_ok(), "Failed with error: {:?}", result.err());

        let (program_json, mir_json) = result.unwrap();
        assert!(program_json.contains("\"kind\":\"Program\""));
        assert!(mir_json.contains("functions"));
    }

    #[test]
    fn test_source_to_program_and_mir_json_handles_hello_simple_llvm_source() {
        let source = include_str!("../../apps/tests/hello_simple_llvm.hako");
        let result = source_to_program_and_mir_json(source);
        assert!(result.is_ok(), "Failed with error: {:?}", result.err());

        let (program_json, mir_json) = result.unwrap();
        assert!(program_json.contains("\"kind\":\"Program\""));
        assert!(program_json.contains("env.console.log"));
        assert!(mir_json.contains("\"functions\""));
    }
}
