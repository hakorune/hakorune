use std::fs;
use std::path::{Path, PathBuf};

macro_rules! llvm_emit_error {
    ($($arg:tt)*) => {{
        if crate::config::env::cli_verbose_enabled() {
            crate::runtime::get_global_ring0()
                .log
                .error(&format!($($arg)*));
        }
    }};
}

macro_rules! llvm_emit_debug {
    ($($arg:tt)*) => {{
        crate::runtime::get_global_ring0()
            .log
            .debug(&format!($($arg)*));
    }};
}

pub struct Opts {
    pub out: Option<PathBuf>,
    pub nyrt: Option<PathBuf>,
    pub opt_level: Option<String>,
    pub timeout_ms: Option<u64>,
    pub compile_recipe: Option<String>,
    pub compat_replay: Option<String>,
}

mod defaults;
mod ll_emit_bridge;
mod normalize;
mod route;
mod transport;
pub use defaults::boundary_default_object_opts;

pub fn mir_json_file_to_object(input_json_path: &Path, opts: Opts) -> Result<PathBuf, String> {
    let mir_json = fs::read_to_string(input_json_path)
        .map_err(|e| format!("[llvmemit/input/read-failed] {}", e))?;
    mir_json_to_object(&mir_json, opts)
}

/// Compile MIR(JSON v0) to an object file (.o) using ny-llvmc. Returns the output path.
/// Fail‑Fast: prints stable tags and returns Err with the same message.
pub fn mir_json_to_object(mir_json: &str, opts: Opts) -> Result<PathBuf, String> {
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

/// Link an object to an executable via C-API FFI bundle.
pub fn link_object_capi(
    obj_in: &Path,
    exe_out: &Path,
    extra_ldflags: Option<&str>,
) -> Result<(), String> {
    if crate::config::env::cabi_trace() {
        llvm_emit_debug!("[hb:link:ldflags] {}", extra_ldflags.unwrap_or("<none>"));
    }
    transport::link_via_capi(obj_in, exe_out, extra_ldflags)
}

pub fn normalize_mir_json_for_backend(mir_json: &str) -> Result<String, String> {
    normalize::normalize_mir_json_for_backend(mir_json)
}
