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

mod capi_transport;
pub mod compat_text_primitive;
mod defaults;
mod ll_emit_compare_driver;
mod ll_emit_compare_source;
mod ll_tool_driver;
mod normalize;
mod provider_keep;
mod route;
mod transport_io;
mod transport_paths;
pub use defaults::boundary_default_object_opts;
/// Compile textual LLVM IR to an object file through the thin Rust tool boundary.
pub fn ll_text_to_object(ll_text: &str, opts: Opts) -> Result<PathBuf, String> {
    let out_path = transport_paths::resolve_backend_object_output(&opts);
    transport_io::ensure_backend_output_parent(&out_path);
    ll_tool_driver::ll_text_to_object(ll_text, &out_path, "ll-text")?;
    Ok(out_path)
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
    capi_transport::link_via_capi(obj_in, exe_out, extra_ldflags)
}

pub fn normalize_mir_json_for_backend(mir_json: &str) -> Result<String, String> {
    normalize::normalize_mir_json_for_backend(mir_json)
}
