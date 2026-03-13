mod ast_json;
mod program_json;

use crate::mir::MirModule;
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

    let parsed = parse_input_json(program_json)?;

    let module = if parsed.get("version").is_some() && parsed.get("kind").is_some() {
        program_json::lower_program_json_to_module(program_json, imports)?
    } else {
        ast_json::lower_ast_json_to_module(&parsed)?
    };

    let tmp_path = emit_module_to_temp_mir_json(&module)?;
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

fn parse_input_json(program_json: &str) -> Result<JsonValue, String> {
    serde_json::from_str(program_json).map_err(|error| {
        if trace_enabled() {
            let preview: String = program_json.chars().take(120).collect();
            let prefix_bytes: Vec<u8> = program_json.as_bytes().iter().take(16).copied().collect();
            trace_log(format!(
                "[stage1/mir_builder] invalid_json_preview={:?} prefix_bytes={:?}",
                preview, prefix_bytes
            ));
        }
        let tag = format!("{FAILFAST_TAG} invalid JSON: {}", error);
        crate::runtime::get_global_ring0().log.error(&tag);
        tag
    })
}

fn emit_module_to_temp_mir_json(module: &MirModule) -> Result<std::path::PathBuf, String> {
    let tmp_path = unique_mir_json_tmp_path();
    match runner::mir_json_emit::emit_mir_json_for_harness_bin(module, &tmp_path) {
        Ok(()) => Ok(tmp_path),
        Err(error) => {
            let tag = format!("{FAILFAST_TAG} {}", error);
            crate::runtime::get_global_ring0().log.error(&tag);
            Err(tag)
        }
    }
}
