use std::path::PathBuf;

use super::normalize;
use super::route;
use super::Opts;

/// Compile MIR(JSON text) to an object through the explicit backend boundary.
///
/// This is the remaining Rust-side text object emission chokepoint for
/// monitor-only proof lanes. It normalizes input once, then delegates route
/// selection to the backend route layer without reviving legacy helper ownership.
pub fn compile_object_from_mir_json_text_boundary(
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
