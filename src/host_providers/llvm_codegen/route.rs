use std::path::PathBuf;

use super::normalize::validate_backend_mir_shape;
use super::transport::{
    compile_via_capi, ensure_backend_output_parent, prepare_backend_input_json_file,
    resolve_backend_object_output,
};
use super::Opts;

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
    let compile_recipe = opts.compile_recipe.clone();
    let compat_replay = opts.compat_replay.clone();
    let compile_symbol = compile_symbol_for_keep_recipe(compile_recipe.as_deref());
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
    let compile_recipe = opts.compile_recipe.clone();
    let compat_replay = opts.compat_replay.clone();
    let compile_symbol = compile_symbol_for_keep_recipe(compile_recipe.as_deref());
    match compile_via_capi(
        &in_path,
        &out_path,
        compile_symbol,
        compile_recipe.as_deref(),
        compat_replay.as_deref(),
        opts,
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

fn compile_symbol_for_keep_recipe(recipe: Option<&str>) -> &'static [u8] {
    // Keep lanes may still reuse the historical generic export.
    // Daily pure-first callers should already be explicit before reaching here.
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

#[cfg(test)]
mod tests {
    use super::{compile_symbol_for_keep_recipe, COMPILE_SYMBOL_PURE_FIRST};

    #[test]
    fn keep_recipe_prefers_pure_first_symbol_when_explicit() {
        assert_eq!(
            compile_symbol_for_keep_recipe(Some("pure-first")),
            COMPILE_SYMBOL_PURE_FIRST
        );
    }

    #[test]
    fn keep_recipe_uses_generic_symbol_for_missing_or_compat_values() {
        assert_eq!(
            compile_symbol_for_keep_recipe(None),
            super::super::COMPILE_SYMBOL_DEFAULT
        );
        assert_eq!(
            compile_symbol_for_keep_recipe(Some("harness")),
            super::super::COMPILE_SYMBOL_DEFAULT
        );
    }
}
