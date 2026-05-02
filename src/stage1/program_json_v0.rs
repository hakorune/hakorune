//! Stage1 Program(JSON v0) façade.
//!
//! Layout SSOT:
//! - `routing.rs`: source-shape and build-route policy
//! - `authority.rs`: strict source authority
//! - `extract.rs`: source observation / helper extraction
//! - `record_payload.rs`: shared enum compat payload boxification helpers
//! - `lowering.rs`: AST subset -> Program(JSON v0) lowering
//!
//! Cross-crate surface:
//! - allowed: `emit_program_json_v0_for_strict_authority_source(...)`,
//!   `emit_program_json_v0_for_current_stage1_build_box_mode(...)`
//! - forbidden: route/source-shape internals, parse/lower orchestration

#[path = "program_json_v0/authority.rs"]
mod authority;
#[path = "program_json_v0/extract.rs"]
mod extract;
#[path = "program_json_v0/lowering.rs"]
mod lowering;
#[path = "program_json_v0/record_payload.rs"]
mod record_payload;
#[path = "program_json_v0/routing.rs"]
mod routing;

#[cfg(test)]
use routing::strict_authority_program_json_v0_source_rejection;

fn trace_enabled() -> bool {
    std::env::var("HAKO_STAGE1_PROGRAM_JSON_TRACE")
        .ok()
        .as_deref()
        == Some("1")
}

const STAGE1_PROGRAM_JSON_V0_FREEZE_TAG: &str = "[freeze:contract][stage1_program_json_v0]";

fn current_stage1_build_box_strict_authority_mode() -> bool {
    crate::config::env::stage1::emit_program_json()
}

// Public entry surface

/// Explicit compatibility keep for launcher/dev-local alias sugar.
fn source_to_program_json_v0_relaxed(source_text: &str) -> Result<String, String> {
    authority::source_to_program_json_v0_relaxed(source_text)
}

/// Explicit strict parse entry kept owner-local to this cluster.
fn source_to_program_json_v0_strict(source_text: &str) -> Result<String, String> {
    authority::source_to_program_json_v0_strict(source_text)
}

/// Explicit authority helper for current `stage1-env-mir-source`.
pub fn emit_program_json_v0_for_strict_authority_source(
    source_text: &str,
) -> Result<String, String> {
    authority::emit_program_json_v0_for_strict_authority_source(source_text)
}

/// Crate-local helper for the future-retire Rust Stage1 bridge emit-program route.
pub(crate) fn emit_program_json_v0_for_stage1_bridge_emit_program_json(
    source_text: &str,
) -> Result<String, String> {
    authority::source_to_program_json_v0_strict(source_text)
        .map_err(|error_text| format!("emit-program-json-v0: {}", error_text))
}

fn format_stage1_program_json_v0_freeze(error_text: String) -> String {
    format!("{STAGE1_PROGRAM_JSON_V0_FREEZE_TAG} {}", error_text)
}

/// Owner-local explicit build-box helper.
fn emit_program_json_v0_for_stage1_build_box(
    source_text: &str,
    strict_authority_mode: bool,
) -> Result<String, String> {
    routing::emit_stage1_build_box_program_json(source_text, strict_authority_mode)
        .map(|emission| emission.into_program_json())
        .map_err(format_stage1_program_json_v0_freeze)
}

/// Cross-crate build-box helper that follows the current stage1 mode contract.
pub fn emit_program_json_v0_for_current_stage1_build_box_mode(
    source_text: &str,
) -> Result<String, String> {
    emit_program_json_v0_for_stage1_build_box(
        source_text,
        current_stage1_build_box_strict_authority_mode(),
    )
}

#[cfg(test)]
mod tests;
