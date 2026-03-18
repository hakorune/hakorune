use std::ffi::{CStr, CString};
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use libloading::Library;
use std::os::raw::{c_char, c_int, c_void};

extern "C" {
    fn free(ptr: *mut c_void);
}

type CompileFn = unsafe extern "C" fn(*const c_char, *const c_char, *mut *mut c_char) -> c_int;
type LinkFn =
    unsafe extern "C" fn(*const c_char, *const c_char, *const c_char, *mut *mut c_char) -> c_int;

const COMPILE_SYMBOL_DEFAULT: &[u8] = b"hako_llvmc_compile_json\0";
const COMPILE_SYMBOL_PURE_FIRST: &[u8] = b"hako_llvmc_compile_json_pure_first\0";
const BOUNDARY_DEFAULT_COMPILE_RECIPE: &str = "pure-first";
const BOUNDARY_DEFAULT_COMPAT_REPLAY: &str = "harness";

pub fn emit_dummy_object(out: &Path) -> Result<()> {
    let tmp = temporary_dummy_mir_path(out);
    fs::write(&tmp, build_dummy_mir_json())
        .with_context(|| format!("failed to write dummy MIR JSON: {}", tmp.display()))?;
    let result = emit_object_from_json(&tmp, out);
    let _ = fs::remove_file(&tmp);
    result
}

pub fn emit_object_from_json(input: &Path, out: &Path) -> Result<()> {
    ensure_output_parent(out);
    call_compile_symbol(input, out)
}

pub fn link_object_to_exe(
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

fn build_dummy_mir_json() -> String {
    r#"{"kind":"MIR","schema_version":"1.0","metadata":{"extern_c":[]},"functions":[{"name":"ny_main","blocks":[{"id":0,"instructions":[{"op":"const","dst":1,"value":{"type":"i64","value":0}},{"op":"ret","value":1}]}]}]}"#
        .to_string()
}

fn temporary_dummy_mir_path(out: &Path) -> PathBuf {
    let filename = out
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("nyllvmc_boundary_dummy");
    std::env::temp_dir().join(format!("{}.{}.mir.json", filename, std::process::id()))
}

fn ffi_library_filenames() -> &'static [&'static str] {
    if cfg!(target_os = "windows") {
        &["hako_llvmc_ffi.dll", "libhako_llvmc_ffi.dll"]
    } else if cfg!(target_os = "macos") {
        &[
            "libhako_llvmc_ffi.dylib",
            "hako_llvmc_ffi.dylib",
            "libhako_llvmc_ffi.so",
        ]
    } else {
        &[
            "libhako_llvmc_ffi.so",
            "hako_llvmc_ffi.so",
            "libhako_llvmc_ffi.dylib",
        ]
    }
}

fn ffi_library_default_candidates() -> Vec<PathBuf> {
    let mut out = Vec::new();
    for name in ffi_library_filenames() {
        out.push(PathBuf::from("target/release").join(name));
        out.push(PathBuf::from("lib").join(name));
    }
    out
}

fn resolve_ffi_library() -> Result<PathBuf> {
    let mut candidates = Vec::new();
    if let Ok(path) = std::env::var("HAKO_AOT_FFI_LIB") {
        let path = path.trim();
        if !path.is_empty() {
            candidates.push(PathBuf::from(path));
        }
    }
    candidates.extend(ffi_library_default_candidates());
    candidates
        .into_iter()
        .find(|path| path.exists())
        .ok_or_else(|| {
            anyhow::anyhow!(
                "FFI library not found (set HAKO_AOT_FFI_LIB or build libhako_llvmc_ffi)"
            )
        })
}

unsafe fn with_compile_symbol<T, F>(action: F) -> Result<T>
where
    F: FnOnce(CompileFn) -> Result<T>,
{
    let lib = open_ffi_library()?;
    let func: CompileFn = *lib
        .get(COMPILE_SYMBOL_PURE_FIRST)
        .or_else(|_| lib.get(COMPILE_SYMBOL_DEFAULT))
        .context("missing symbol hako_llvmc_compile_json{_pure_first}")?;
    action(func)
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
            with_boundary_default_route(|| {
                with_env_override(
                    "HAKO_BACKEND_COMPILE_RECIPE",
                    Some(BOUNDARY_DEFAULT_COMPILE_RECIPE),
                    || {
                        with_env_override(
                            "HAKO_BACKEND_COMPAT_REPLAY",
                            Some(BOUNDARY_DEFAULT_COMPAT_REPLAY),
                            || {
                                let rc = func(
                                    cin.as_ptr(),
                                    cout.as_ptr(),
                                    &mut err_ptr as *mut *mut c_char,
                                );
                                interpret_result(rc, err_ptr, out, "object not produced")
                            },
                        )
                    },
                )
            })
        })
    }
}

fn with_boundary_default_route<T, F>(action: F) -> T
where
    F: FnOnce() -> T,
{
    action()
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
    let lib_path = resolve_ffi_library()?;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dummy_mir_json_has_entry_function() {
        let text = build_dummy_mir_json();
        assert!(text.contains("\"name\":\"ny_main\""));
        assert!(text.contains("\"schema_version\":\"1.0\""));
    }
}
