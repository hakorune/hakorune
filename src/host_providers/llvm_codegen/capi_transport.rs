use std::ffi::{CStr, CString};
use std::path::{Path, PathBuf};

use super::defaults;
use super::Opts;

#[cfg(feature = "plugins")]
fn resolve_ffi_library_path() -> Result<PathBuf, String> {
    let mut candidates: Vec<PathBuf> = Vec::new();
    if let Some(p) = crate::config::env::aot_ffi_lib_path() {
        candidates.push(PathBuf::from(p));
    }
    candidates.extend(defaults::ffi_library_default_candidates());
    candidates
        .into_iter()
        .find(|p| p.exists())
        .ok_or_else(|| "FFI library not found (set HAKO_AOT_FFI_LIB)".to_string())
}

#[cfg(feature = "plugins")]
fn load_ffi_library() -> Result<libloading::Library, String> {
    let lib_path = resolve_ffi_library_path()?;
    unsafe { libloading::Library::new(lib_path).map_err(|e| format!("dlopen failed: {}", e)) }
}

#[cfg(feature = "plugins")]
pub(super) fn compile_via_capi(
    json_in: &Path,
    obj_out: &Path,
    compile_symbol: &[u8],
    compile_recipe: Option<&str>,
    compat_replay: Option<&str>,
    opts: &Opts,
) -> Result<(), String> {
    use std::os::raw::{c_char, c_int, c_void};

    extern "C" {
        fn free(ptr: *mut c_void);
    }

    unsafe {
        let lib = load_ffi_library()?;
        type CompileFn =
            unsafe extern "C" fn(*const c_char, *const c_char, *mut *mut c_char) -> c_int;
        let func: libloading::Symbol<CompileFn> = lib
            .get(compile_symbol)
            .or_else(|_| lib.get(defaults::COMPILE_SYMBOL_DEFAULT))
            .map_err(|e| format!("dlsym failed: {}", e))?;
        let cin = CString::new(json_in.to_string_lossy().as_bytes())
            .map_err(|_| "invalid json path".to_string())?;
        let cout = CString::new(obj_out.to_string_lossy().as_bytes())
            .map_err(|_| "invalid out path".to_string())?;
        let mut err_ptr: *mut c_char = std::ptr::null_mut();
        let prev_recipe = std::env::var("HAKO_BACKEND_COMPILE_RECIPE").ok();
        let prev_replay = std::env::var("HAKO_BACKEND_COMPAT_REPLAY").ok();
        let prev_hako_opt = std::env::var("HAKO_LLVM_OPT_LEVEL").ok();
        let prev_nyash_opt = std::env::var("NYASH_LLVM_OPT_LEVEL").ok();
        if let Some(value) = compile_recipe.as_deref() {
            std::env::set_var("HAKO_BACKEND_COMPILE_RECIPE", value);
        } else {
            std::env::remove_var("HAKO_BACKEND_COMPILE_RECIPE");
        }
        if let Some(value) = compat_replay.as_deref() {
            std::env::set_var("HAKO_BACKEND_COMPAT_REPLAY", value);
        } else {
            std::env::remove_var("HAKO_BACKEND_COMPAT_REPLAY");
        }
        if let Some(level) = opts.opt_level.as_ref() {
            std::env::set_var("HAKO_LLVM_OPT_LEVEL", level);
            std::env::set_var("NYASH_LLVM_OPT_LEVEL", level);
        } else {
            if crate::config::env::llvm_opt_level_envs().0.is_none() {
                std::env::set_var("HAKO_LLVM_OPT_LEVEL", "0");
            }
            if crate::config::env::llvm_opt_level_envs().1.is_none() {
                std::env::set_var("NYASH_LLVM_OPT_LEVEL", "0");
            }
        }

        if crate::config::env::cabi_trace() {
            let (hako_opt, nyash_opt) = crate::config::env::llvm_opt_level_envs();
            llvm_emit_debug!(
                "[llvmemit/capi/enter] HAKO_LLVM_OPT_LEVEL={:?} NYASH_LLVM_OPT_LEVEL={:?}",
                hako_opt,
                nyash_opt
            );
        }

        let rc = func(
            cin.as_ptr(),
            cout.as_ptr(),
            &mut err_ptr as *mut *mut c_char,
        );
        if let Some(v) = prev_recipe {
            std::env::set_var("HAKO_BACKEND_COMPILE_RECIPE", v);
        } else {
            std::env::remove_var("HAKO_BACKEND_COMPILE_RECIPE");
        }
        if let Some(v) = prev_replay {
            std::env::set_var("HAKO_BACKEND_COMPAT_REPLAY", v);
        } else {
            std::env::remove_var("HAKO_BACKEND_COMPAT_REPLAY");
        }
        if let Some(v) = prev_hako_opt {
            std::env::set_var("HAKO_LLVM_OPT_LEVEL", v);
        } else {
            std::env::remove_var("HAKO_LLVM_OPT_LEVEL");
        }
        if let Some(v) = prev_nyash_opt {
            std::env::set_var("NYASH_LLVM_OPT_LEVEL", v);
        } else {
            std::env::remove_var("NYASH_LLVM_OPT_LEVEL");
        }
        if rc != 0 {
            let msg = if !err_ptr.is_null() {
                CStr::from_ptr(err_ptr).to_string_lossy().to_string()
            } else {
                "compile failed".to_string()
            };
            if !err_ptr.is_null() {
                free(err_ptr as *mut c_void);
            }
            return Err(msg);
        }
        if !obj_out.exists() {
            return Err("object not produced".into());
        }
        Ok(())
    }
}

#[cfg(not(feature = "plugins"))]
pub(super) fn compile_via_capi(
    _json_in: &Path,
    _obj_out: &Path,
    _compile_symbol: &[u8],
    _compile_recipe: Option<&str>,
    _compat_replay: Option<&str>,
    _opts: &Opts,
) -> Result<(), String> {
    Err("capi not available (plugins feature disabled)".into())
}

#[cfg(feature = "plugins")]
pub(super) fn link_via_capi(
    obj_in: &Path,
    exe_out: &Path,
    extra_ldflags: Option<&str>,
) -> Result<(), String> {
    use std::os::raw::{c_char, c_int, c_void};

    extern "C" {
        fn free(ptr: *mut c_void);
    }

    unsafe {
        let lib = load_ffi_library()?;
        type LinkFn = unsafe extern "C" fn(
            *const c_char,
            *const c_char,
            *const c_char,
            *mut *mut c_char,
        ) -> c_int;
        let func: libloading::Symbol<LinkFn> = lib
            .get(b"hako_llvmc_link_obj\0")
            .map_err(|e| format!("dlsym failed: {}", e))?;
        let cobj = CString::new(obj_in.to_string_lossy().as_bytes())
            .map_err(|_| "invalid obj path".to_string())?;
        let cexe = CString::new(exe_out.to_string_lossy().as_bytes())
            .map_err(|_| "invalid exe path".to_string())?;
        let ldflags_owned;
        let cflags_ptr = if let Some(s) = extra_ldflags {
            ldflags_owned = CString::new(s).map_err(|_| "invalid ldflags".to_string())?;
            ldflags_owned.as_ptr()
        } else {
            std::ptr::null()
        };
        let mut err_ptr: *mut c_char = std::ptr::null_mut();
        let rc = func(
            cobj.as_ptr(),
            cexe.as_ptr(),
            cflags_ptr,
            &mut err_ptr as *mut *mut c_char,
        );
        if rc != 0 {
            let msg = if !err_ptr.is_null() {
                CStr::from_ptr(err_ptr).to_string_lossy().to_string()
            } else {
                "link failed".to_string()
            };
            if !err_ptr.is_null() {
                free(err_ptr as *mut c_void);
            }
            return Err(msg);
        }
        if !exe_out.exists() {
            return Err("exe not produced".into());
        }
        Ok(())
    }
}

#[cfg(not(feature = "plugins"))]
pub(super) fn link_via_capi(
    _obj_in: &Path,
    _exe_out: &Path,
    _extra: Option<&str>,
) -> Result<(), String> {
    Err("capi not available (plugins feature disabled)".into())
}
