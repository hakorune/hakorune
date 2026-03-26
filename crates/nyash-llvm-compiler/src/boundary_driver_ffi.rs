use std::ffi::{CStr, CString};
use std::fs;
use std::path::Path;

use anyhow::{bail, Context, Result};
use libloading::Library;
use std::os::raw::{c_char, c_int, c_void};

extern "C" {
    fn free(ptr: *mut c_void);
}

type CompileFn = unsafe extern "C" fn(*const c_char, *const c_char, *mut *mut c_char) -> c_int;
type LinkFn =
    unsafe extern "C" fn(*const c_char, *const c_char, *const c_char, *mut *mut c_char) -> c_int;

pub(super) fn emit_object_from_json(input: &Path, out: &Path) -> Result<()> {
    ensure_output_parent(out);
    call_compile_symbol(input, out)
}

pub(super) fn link_object_to_exe(
    obj: &Path,
    out_exe: &Path,
    nyrt_dir: Option<&Path>,
    extra_libs: Option<&str>,
) -> Result<()> {
    ensure_output_parent(out_exe);
    call_link_symbol(obj, out_exe, nyrt_dir, extra_libs)
}

fn ensure_output_parent(path: &Path) {
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
}

unsafe fn with_compile_symbol<T, F>(action: F) -> Result<T>
where
    F: FnOnce(CompileFn) -> Result<T>,
{
    let lib = open_ffi_library()?;
    let recipe_env = std::env::var("HAKO_BACKEND_COMPILE_RECIPE").ok();
    let replay_env = std::env::var("HAKO_BACKEND_COMPAT_REPLAY").ok();
    let legacy_capi_pure = std::env::var("HAKO_CAPI_PURE").ok();
    let (compile_recipe, compat_replay) =
        super::boundary_driver_defaults::boundary_codegen_request_defaults(
            recipe_env.as_deref(),
            replay_env.as_deref(),
        );
    let compile_symbol = super::boundary_driver_defaults::boundary_compile_symbol(
        compile_recipe.as_deref(),
        legacy_capi_pure.as_deref(),
    );
    emit_compile_route_trace(
        compile_recipe.as_deref(),
        compat_replay.as_deref(),
        compile_symbol,
    );
    let func: CompileFn = *lib
        .get(compile_symbol)
        .context("missing symbol hako_llvmc_compile_json{_pure_first}")?;
    with_env_override("HAKO_BACKEND_COMPILE_RECIPE", compile_recipe.as_deref(), || {
        with_env_override("HAKO_BACKEND_COMPAT_REPLAY", compat_replay.as_deref(), || {
            action(func)
        })
    })
}

unsafe fn with_link_symbol<T, F>(action: F) -> Result<T>
where
    F: FnOnce(LinkFn) -> Result<T>,
{
    let lib = open_ffi_library()?;
    let func: LinkFn = *lib
        .get(b"hako_llvmc_link_obj\0")
        .context("missing symbol hako_llvmc_link_obj")?;
    action(func)
}

fn call_compile_symbol(input: &Path, out: &Path) -> Result<()> {
    let cin =
        CString::new(input.to_string_lossy().as_bytes()).context("invalid input path for C ABI")?;
    let cout =
        CString::new(out.to_string_lossy().as_bytes()).context("invalid output path for C ABI")?;
    let mut err_ptr: *mut c_char = std::ptr::null_mut();
    unsafe {
        with_compile_symbol(|func| {
            let rc = func(
                cin.as_ptr(),
                cout.as_ptr(),
                &mut err_ptr as *mut *mut c_char,
            );
            interpret_result(rc, err_ptr, out, "object not produced")
        })
    }
}

fn call_link_symbol(
    obj: &Path,
    out_exe: &Path,
    nyrt_dir: Option<&Path>,
    extra_libs: Option<&str>,
) -> Result<()> {
    let cobj =
        CString::new(obj.to_string_lossy().as_bytes()).context("invalid object path for C ABI")?;
    let cexe = CString::new(out_exe.to_string_lossy().as_bytes())
        .context("invalid executable path for C ABI")?;
    let libs_owned = extra_libs
        .filter(|value| !value.trim().is_empty())
        .map(CString::new)
        .transpose()
        .context("invalid linker flags for C ABI")?;
    let libs_ptr = libs_owned
        .as_ref()
        .map(|value| value.as_ptr())
        .unwrap_or(std::ptr::null());
    let nyrt_owned = nyrt_dir
        .map(|path| path.to_string_lossy().to_string())
        .filter(|value| !value.trim().is_empty());
    let mut err_ptr: *mut c_char = std::ptr::null_mut();
    unsafe {
        with_link_symbol(|func| {
            with_env_override("NYASH_EMIT_EXE_NYRT", nyrt_owned.as_deref(), || {
                let rc = func(
                    cobj.as_ptr(),
                    cexe.as_ptr(),
                    libs_ptr,
                    &mut err_ptr as *mut *mut c_char,
                );
                interpret_result(rc, err_ptr, out_exe, "exe not produced")
            })
        })
    }
}

unsafe fn open_ffi_library() -> Result<Library> {
    let lib_path = super::boundary_driver_defaults::resolve_ffi_library()?;
    Library::new(lib_path).context("dlopen failed")
}

fn interpret_result(
    rc: c_int,
    err_ptr: *mut c_char,
    output_path: &Path,
    missing_output_message: &str,
) -> Result<()> {
    if rc != 0 {
        let message = error_string_or(err_ptr, "C ABI route failed");
        unsafe {
            if !err_ptr.is_null() {
                free(err_ptr as *mut c_void);
            }
        }
        bail!("{}", message);
    }
    if !output_path.exists() {
        bail!("{}", missing_output_message);
    }
    unsafe {
        if !err_ptr.is_null() {
            free(err_ptr as *mut c_void);
        }
    }
    Ok(())
}

fn error_string_or(err_ptr: *mut c_char, fallback: &str) -> String {
    if err_ptr.is_null() {
        fallback.to_string()
    } else {
        unsafe { CStr::from_ptr(err_ptr).to_string_lossy().to_string() }
    }
}

fn with_env_override<T, F>(key: &str, value: Option<&str>, action: F) -> T
where
    F: FnOnce() -> T,
{
    let prev = std::env::var(key).ok();
    match value {
        Some(value) => std::env::set_var(key, value),
        None => std::env::remove_var(key),
    }
    let result = action();
    match prev {
        Some(prev) => std::env::set_var(key, prev),
        None => std::env::remove_var(key),
    }
    result
}

fn llvm_route_trace_enabled() -> bool {
    matches!(
        std::env::var("NYASH_LLVM_ROUTE_TRACE").ok().as_deref(),
        Some("1" | "on" | "true" | "yes")
    )
}

fn emit_compile_route_trace(
    compile_recipe: Option<&str>,
    compat_replay: Option<&str>,
    compile_symbol: &[u8],
) {
    if !llvm_route_trace_enabled() {
        return;
    }
    let symbol = CStr::from_bytes_with_nul(compile_symbol)
        .ok()
        .and_then(|value| value.to_str().ok())
        .unwrap_or("unknown");
    eprintln!(
        "[llvm-route/select] owner=boundary recipe={} compat_replay={} symbol={}",
        compile_recipe.unwrap_or("unset"),
        compat_replay.unwrap_or("unset"),
        symbol
    );
}

#[cfg(test)]
mod tests {
    use super::{llvm_route_trace_enabled, with_env_override};

    #[test]
    fn with_env_override_restores_previous_value_after_action() {
        std::env::set_var("NYASH_BOUNDARY_DRIVER_TEST_ENV", "before");
        let observed = with_env_override("NYASH_BOUNDARY_DRIVER_TEST_ENV", Some("inside"), || {
            std::env::var("NYASH_BOUNDARY_DRIVER_TEST_ENV").ok()
        });
        assert_eq!(observed.as_deref(), Some("inside"));
        assert_eq!(
            std::env::var("NYASH_BOUNDARY_DRIVER_TEST_ENV")
                .ok()
                .as_deref(),
            Some("before")
        );
        std::env::remove_var("NYASH_BOUNDARY_DRIVER_TEST_ENV");
    }

    #[test]
    fn llvm_route_trace_enabled_accepts_explicit_truthy_values_only() {
        std::env::remove_var("NYASH_LLVM_ROUTE_TRACE");
        assert!(!llvm_route_trace_enabled());
        std::env::set_var("NYASH_LLVM_ROUTE_TRACE", "1");
        assert!(llvm_route_trace_enabled());
        std::env::set_var("NYASH_LLVM_ROUTE_TRACE", "yes");
        assert!(llvm_route_trace_enabled());
        std::env::set_var("NYASH_LLVM_ROUTE_TRACE", "0");
        assert!(!llvm_route_trace_enabled());
        std::env::remove_var("NYASH_LLVM_ROUTE_TRACE");
    }
}
