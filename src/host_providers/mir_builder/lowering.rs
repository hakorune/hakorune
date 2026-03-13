use crate::runner;
use serde_json::Value as JsonValue;
use std::collections::BTreeMap;
use std::fs;

use super::{
    trace_enabled, trace_log, unique_mir_json_tmp_path, Phase0MirJsonEnvGuard, FAILFAST_TAG,
};

/// Convert Program(JSON v0) to MIR(JSON v0) and return it as a String.
/// Fail-Fast: prints stable tags on stderr and returns Err with the same tag text.
pub(super) fn program_json_to_mir_json(program_json: &str) -> Result<String, String> {
    program_json_to_mir_json_with_imports(program_json, BTreeMap::new())
}

/// Convert Program(JSON v0) to MIR(JSON v0) with using imports support.
pub(super) fn program_json_to_mir_json_with_imports(
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

    // Route A: Stage-B Program(JSON v0) -> MIR Module (via json_v0_bridge)
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
        // Route B (Phase-0 bridge): AST JSON (from `--emit-ast-json`, legacy: `--emit-program-json`) -> MIR Module
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

    // Emit MIR(JSON) to a temporary file (reuse existing emitter), then read back.
    let tmp_path = unique_mir_json_tmp_path();
    let emit_res = runner::mir_json_emit::emit_mir_json_for_harness_bin(&module, &tmp_path);

    if let Err(e) = emit_res {
        let tag = format!("{FAILFAST_TAG} {}", e);
        crate::runtime::get_global_ring0().log.error(&tag);
        return Err(tag);
    }
    match fs::read_to_string(&tmp_path) {
        Ok(raw) => {
            let _ = fs::remove_file(&tmp_path);
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
