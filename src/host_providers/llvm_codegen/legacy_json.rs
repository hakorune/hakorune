use std::fs;
use std::path::{Path, PathBuf};

use super::normalize;
use super::route;
use super::Opts;

pub fn mir_json_file_to_object(input_json_path: &Path, opts: Opts) -> Result<PathBuf, String> {
    let mir_json = fs::read_to_string(input_json_path)
        .map_err(|e| format!("[llvmemit/input/read-failed] {}", e))?;
    mir_json_to_object(&mir_json, opts)
}

/// Compile MIR(JSON v0) to an object file (.o) using ny-llvmc. Returns the output path.
/// Fail-Fast: prints stable tags and returns Err with the same message.
pub fn mir_json_to_object(mir_json: &str, opts: Opts) -> Result<PathBuf, String> {
    emit_object_from_mir_json(mir_json, opts)
}

/// Archive-later helper for runtime callers that still need the legacy JSON front door.
pub fn emit_object_from_mir_json(mir_json: &str, opts: Opts) -> Result<PathBuf, String> {
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
