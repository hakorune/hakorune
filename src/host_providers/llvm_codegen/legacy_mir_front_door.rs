use std::path::PathBuf;

use super::normalize;
use super::route;
use super::Opts;

/// Explicit compat/archive MIR(JSON) object helper.
///
/// The root module stays thin: daily code stops at `ll_text_to_object(...)` while
/// the remaining legacy JSON knot stays explicit in this sibling module.
pub fn compile_object_from_legacy_mir_json(
    mir_json: &str,
    opts: Opts,
) -> Result<PathBuf, String> {
    let mir_json = normalize::normalize_mir_json_for_backend(mir_json)?;
    if let Some(out_path) = route::try_compile_via_hako_ll_bridge(&mir_json, &opts)? {
        return Ok(out_path);
    }
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

pub fn normalize_mir_json_for_backend(mir_json: &str) -> Result<String, String> {
    normalize::normalize_mir_json_for_backend(mir_json)
}
