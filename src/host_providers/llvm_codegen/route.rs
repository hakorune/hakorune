use std::path::PathBuf;

use super::normalize::validate_backend_mir_shape;
use super::transport::{
    compile_via_capi, ensure_backend_output_parent, prepare_backend_input_json_file,
    resolve_backend_object_output,
};
use super::{boundary_default_object_opts, Opts};

const COMPILE_SYMBOL_PURE_FIRST: &[u8] = b"hako_llvmc_compile_json_pure_first\0";

pub(super) fn try_compile_via_capi_keep(
    mir_json: &str,
    opts: &Opts,
) -> Result<Option<PathBuf>, String> {
    if !(crate::config::env::llvm_use_capi() && crate::config::env::extern_provider_c_abi()) {
        return Ok(None);
    }
    validate_backend_mir_shape(mir_json)?;
    let in_path = prepare_backend_input_json_file(mir_json)?;
    let out_path = resolve_backend_object_output(opts);
    ensure_backend_output_parent(&out_path);
    let compile_recipe = requested_compile_recipe(opts);
    let compat_replay = requested_compat_replay(opts);
    let compile_symbol = compile_symbol_for_recipe(compile_recipe.as_deref());
    match compile_via_capi(
        &in_path,
        &out_path,
        compile_symbol,
        compile_recipe.as_deref(),
        compat_replay.as_deref(),
        opts,
    ) {
        Ok(()) => Ok(Some(out_path)),
        Err(e) => {
            llvm_emit_error!("[llvmemit/capi/failed] {}", e);
            Err(format!("[llvmemit/capi/failed] {}", e))
        }
    }
}

pub(super) fn try_compile_via_explicit_provider_keep(
    mir_json: &str,
    opts: &Opts,
) -> Result<Option<PathBuf>, String> {
    match crate::config::env::llvm_emit_provider().as_deref() {
        Some("llvmlite") => super::transport::mir_json_to_object_llvmlite(mir_json, opts).map(Some),
        Some("ny-llvmc") => super::transport::mir_json_to_object_ny_llvmc(mir_json, opts).map(Some),
        _ => Ok(None),
    }
}

pub(super) fn try_compile_via_boundary_default(
    mir_json: &str,
    opts: &Opts,
) -> Result<Option<PathBuf>, String> {
    validate_backend_mir_shape(mir_json)?;
    let in_path = prepare_backend_input_json_file(mir_json)?;
    let out_path = resolve_backend_object_output(opts);
    ensure_backend_output_parent(&out_path);
    let boundary_opts = with_boundary_default_route(opts);
    let compile_symbol = compile_symbol_for_recipe(boundary_opts.compile_recipe.as_deref());
    match compile_via_capi(
        &in_path,
        &out_path,
        compile_symbol,
        boundary_opts.compile_recipe.as_deref(),
        boundary_opts.compat_replay.as_deref(),
        &boundary_opts,
    ) {
        Ok(()) => Ok(Some(out_path)),
        Err(error) if capi_boundary_unavailable(&error) => Ok(None),
        Err(error) => {
            llvm_emit_error!("[llvmemit/capi/default-failed] {}", error);
            Err(format!("[llvmemit/capi/default-failed] {}", error))
        }
    }
}

pub(super) fn boundary_default_unavailable_tag() -> String {
    "[llvmemit/capi/default-unavailable] build libhako_llvmc_ffi.so or set HAKO_LLVM_EMIT_PROVIDER=llvmlite".into()
}

fn requested_compile_recipe(opts: &Opts) -> Option<String> {
    opts.compile_recipe
        .clone()
        .or_else(crate::config::env::backend_compile_recipe)
}

fn requested_compat_replay(opts: &Opts) -> Option<String> {
    opts.compat_replay
        .clone()
        .or_else(crate::config::env::backend_compat_replay)
}

fn with_boundary_default_route(opts: &Opts) -> Opts {
    let mut route = boundary_default_object_opts(
        opts.out.clone(),
        opts.nyrt.clone(),
        opts.opt_level.clone(),
        opts.timeout_ms,
    );
    route.compile_recipe = opts.compile_recipe.clone().or(route.compile_recipe);
    route.compat_replay = opts.compat_replay.clone().or(route.compat_replay);
    route
}

fn compile_symbol_for_recipe(recipe: Option<&str>) -> &'static [u8] {
    match recipe {
        Some("pure-first") => COMPILE_SYMBOL_PURE_FIRST,
        _ => super::COMPILE_SYMBOL_DEFAULT,
    }
}

fn capi_boundary_unavailable(error: &str) -> bool {
    error.contains("FFI library not found")
        || error.contains("capi not available")
        || error.contains("dlopen failed")
        || error.contains("dlsym failed")
}
