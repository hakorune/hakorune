#[cfg(test)]
mod ast_json;

#[cfg(test)]
use crate::mir::MirModule;
#[cfg(test)]
use crate::runner;
#[cfg(test)]
use serde_json::Value as JsonValue;
#[cfg(test)]
use std::collections::BTreeMap;

#[cfg(test)]
use super::{trace_enabled, trace_log, Phase0MirJsonEnvGuard};

/// Convert Program(JSON v0) to MIR(JSON v0) and return it as a String.
/// Fail-Fast: prints stable tags on stderr and returns Err with the same tag text.
#[cfg(test)]
pub(super) fn program_json_to_mir_json(program_json: &str) -> Result<String, String> {
    program_json_to_mir_json_impl(program_json)
}

/// Convert Program(JSON v0) to MIR(JSON v0) with using imports support.
#[cfg(test)]
pub(super) fn program_json_to_mir_json_with_imports(
    program_json: &str,
    imports: BTreeMap<String, String>,
) -> Result<String, String> {
    let _env_guard = Phase0MirJsonEnvGuard::new();
    let parsed = parse_input_json(program_json)?;
    let module = if parsed.get("version").is_some() && parsed.get("kind").is_some() {
        lower_program_json_to_module_with_imports(program_json, imports)?
    } else {
        ast_json::lower_ast_json_to_module(&parsed)?
    };
    super::module_to_mir_json(&module)
}

#[cfg(test)]
fn program_json_to_mir_json_impl(program_json: &str) -> Result<String, String> {
    // Phase-0 contract: MIR JSON v0 must be executable via `--mir-json-file` v0 loader.
    // That loader supports `externcall`/`boxcall` but not unified `mir_call` lowering.
    // Therefore we force unified-call OFF for both compilation and emission here.
    let _env_guard = Phase0MirJsonEnvGuard::new();

    let parsed = parse_input_json(program_json)?;

    let module = if parsed.get("version").is_some() && parsed.get("kind").is_some() {
        match runner::json_v0_bridge::parse_json_v0_to_module(program_json) {
            Ok(module) => module,
            Err(error) => return Err(super::failfast_error(error)),
        }
    } else {
        ast_json::lower_ast_json_to_module(&parsed)?
    };

    super::module_to_mir_json(&module)
}

#[cfg(test)]
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
        super::failfast_error(format!("invalid JSON: {}", error))
    })
}

#[cfg(test)]
fn lower_program_json_to_module_with_imports(
    program_json: &str,
    imports: BTreeMap<String, String>,
) -> Result<MirModule, String> {
    match runner::json_v0_bridge::parse_json_v0_to_module_with_imports(program_json, imports) {
        Ok(module) => Ok(module),
        Err(error) => Err(super::failfast_error(error)),
    }
}
