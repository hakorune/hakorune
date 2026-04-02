use std::path::PathBuf;

use super::normalize;
use super::route;
use super::Opts;

/// Explicit no-helper compat text primitive.
///
/// This keeps the remaining Rust-side `emit_object(mir_json_text) -> object path`
/// contract off the legacy front-door helper while preserving the explicit
/// provider-keep / CAPI keep semantics that still back monitor-only proof lanes.
pub fn compile_object_from_mir_json_text_no_helper(
    mir_json: &str,
    opts: Opts,
) -> Result<PathBuf, String> {
    let mir_json = normalize::normalize_mir_json_for_backend(mir_json)?;
    if let Some(out_path) = route::try_compile_via_capi_keep(&mir_json, &opts)? {
        return Ok(out_path);
    }
    if let Some(out_path) = route::try_compile_via_explicit_provider_keep(&mir_json, &opts)? {
        return Ok(out_path);
    }
    if let Some(out_path) = route::try_compile_via_boundary_default(&mir_json, &opts)? {
        return Ok(out_path);
    }
    let tag = route::boundary_default_unavailable_tag();
    llvm_emit_error!("{}", tag);
    Err(tag)
}
