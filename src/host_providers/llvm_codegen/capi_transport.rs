use std::ffi::{CStr, CString};
use std::path::{Path, PathBuf};

use super::defaults;
use super::normalize;
use super::runtime_abi_descriptor::LifecycleRuntimeSessionV1;
use super::transport_io;
use super::transport_paths;
use super::Opts;
use crate::mir::function::{
    PublishedLifecycleBodySiteCRowV1, PublishedLifecycleCFrameHeaderV2,
    PublishedStaticMethodCallCRowV1,
};

#[repr(C)]
struct LifecycleTargetSessionCRowV1 {
    revision: u32,
    target_triple: *const std::os::raw::c_char,
    endian: u32,
    pointer_width: u32,
    fault_abi_version: u32,
    status_abi_version: u32,
    diagnostic_size: u32,
    diagnostic_align: u32,
    diagnostic_site_offset: u32,
    diagnostic_details_offset: u32,
    diagnostic_message_offset: u32,
    frame_size: u32,
    frame_align: u32,
    frame_primary_offset: u32,
    frame_suppressed_offset: u32,
}

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

/// Validates the one final-view-issued lifecycle physical input before the
/// selected lifecycle body ingress observes any temporary body transport.
pub(super) fn validate_published_lifecycle_physical_v1(json_in: &Path) -> Result<(), String> {
    use std::os::raw::{c_char, c_int, c_void};

    extern "C" {
        fn free(ptr: *mut c_void);
    }

    unsafe {
        let lib = load_ffi_library()?;
        type ValidateFn = unsafe extern "C" fn(*const c_char, *mut *mut c_char) -> c_int;
        let validate: libloading::Symbol<ValidateFn> = lib
            .get(b"hako_llvmc_validate_published_lifecycle_physical_v1\0")
            .map_err(|error| format!("dlsym failed for lifecycle physical parser: {error}"))?;
        let input = CString::new(json_in.to_string_lossy().as_bytes())
            .map_err(|_| "invalid lifecycle physical JSON path".to_owned())?;
        let mut error: *mut c_char = std::ptr::null_mut();
        let rc = validate(input.as_ptr(), &mut error);
        if rc == 0 {
            return Ok(());
        }
        let message = if error.is_null() {
            "published lifecycle physical parser rejected input".to_owned()
        } else {
            CStr::from_ptr(error).to_string_lossy().into_owned()
        };
        if !error.is_null() {
            free(error as *mut c_void);
        }
        Err(message)
    }
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
            .map_err(|e| format!("dlsym failed for requested compile symbol: {}", e))?;
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
        transport_io::ensure_backend_artifact_written(obj_out, "object")?;
        Ok(())
    }
}

/// Compile a published module through the typed published-call ingress.
/// Unlike `compile_via_capi`, this entry passes the selected call-site rows
/// directly to the C consumer; it never selects a JSON/name fallback route.
#[cfg(feature = "plugins")]
pub(super) fn compile_published_static_method_v1(
    json_in: &Path,
    obj_out: &Path,
    rows: &[PublishedStaticMethodCallCRowV1],
    opts: &Opts,
) -> Result<(), String> {
    use std::os::raw::{c_char, c_int, c_void};

    extern "C" {
        fn free(ptr: *mut c_void);
    }

    if rows.is_empty() {
        return Err("published-call ingress requires at least one row".to_owned());
    }

    unsafe {
        let lib = load_ffi_library()?;
        type CompileFn = unsafe extern "C" fn(
            *const c_char,
            *const PublishedStaticMethodCallCRowV1,
            usize,
            *const c_char,
            *mut *mut c_char,
        ) -> c_int;
        let func: libloading::Symbol<CompileFn> = lib
            .get(b"hako_llvmc_compile_published_static_method_v1\0")
            .map_err(|e| format!("dlsym failed for typed published MIR ingress: {}", e))?;
        let cin = CString::new(json_in.to_string_lossy().as_bytes())
            .map_err(|_| "invalid json path".to_owned())?;
        let cout = CString::new(obj_out.to_string_lossy().as_bytes())
            .map_err(|_| "invalid out path".to_owned())?;
        let mut err_ptr: *mut c_char = std::ptr::null_mut();
        let prev_recipe = std::env::var("HAKO_BACKEND_COMPILE_RECIPE").ok();
        let prev_replay = std::env::var("HAKO_BACKEND_COMPAT_REPLAY").ok();
        let prev_hako_opt = std::env::var("HAKO_LLVM_OPT_LEVEL").ok();
        let prev_nyash_opt = std::env::var("NYASH_LLVM_OPT_LEVEL").ok();
        if let Some(value) = opts.compile_recipe.as_deref() {
            std::env::set_var("HAKO_BACKEND_COMPILE_RECIPE", value);
        } else {
            std::env::remove_var("HAKO_BACKEND_COMPILE_RECIPE");
        }
        if let Some(value) = opts.compat_replay.as_deref() {
            std::env::set_var("HAKO_BACKEND_COMPAT_REPLAY", value);
        } else {
            std::env::remove_var("HAKO_BACKEND_COMPAT_REPLAY");
        }
        if let Some(level) = opts.opt_level.as_ref() {
            std::env::set_var("HAKO_LLVM_OPT_LEVEL", level);
            std::env::set_var("NYASH_LLVM_OPT_LEVEL", level);
        }

        let rc = func(
            cin.as_ptr(),
            rows.as_ptr(),
            rows.len(),
            cout.as_ptr(),
            &mut err_ptr as *mut *mut c_char,
        );

        if let Some(value) = prev_recipe {
            std::env::set_var("HAKO_BACKEND_COMPILE_RECIPE", value);
        } else {
            std::env::remove_var("HAKO_BACKEND_COMPILE_RECIPE");
        }
        if let Some(value) = prev_replay {
            std::env::set_var("HAKO_BACKEND_COMPAT_REPLAY", value);
        } else {
            std::env::remove_var("HAKO_BACKEND_COMPAT_REPLAY");
        }
        if let Some(value) = prev_hako_opt {
            std::env::set_var("HAKO_LLVM_OPT_LEVEL", value);
        } else {
            std::env::remove_var("HAKO_LLVM_OPT_LEVEL");
        }
        if let Some(value) = prev_nyash_opt {
            std::env::set_var("NYASH_LLVM_OPT_LEVEL", value);
        } else {
            std::env::remove_var("NYASH_LLVM_OPT_LEVEL");
        }

        if rc != 0 {
            let msg = if !err_ptr.is_null() {
                CStr::from_ptr(err_ptr).to_string_lossy().into_owned()
            } else {
                "typed published MIR compile failed".to_owned()
            };
            if !err_ptr.is_null() {
                free(err_ptr as *mut c_void);
            }
            return Err(msg);
        }
        transport_io::ensure_backend_artifact_written(obj_out, "object")?;
        Ok(())
    }
}

#[cfg(feature = "plugins")]
pub(super) fn compile_published_lifecycle_v2(
    frame: &PublishedLifecycleCFrameHeaderV2,
    obj_out: &Path,
) -> Result<(), String> {
    use std::os::raw::{c_char, c_int, c_void};
    extern "C" {
        fn free(ptr: *mut c_void);
    }
    unsafe {
        let lib = load_ffi_library()?;
        type CompileFn = unsafe extern "C" fn(
            *const PublishedLifecycleCFrameHeaderV2,
            *const c_char,
            *mut *mut c_char,
        ) -> c_int;
        let func: libloading::Symbol<CompileFn> = lib
            .get(b"hako_llvmc_compile_published_lifecycle_v2\0")
            .map_err(|error| format!("dlsym failed for published lifecycle V2 ingress: {error}"))?;
        let output = CString::new(obj_out.to_string_lossy().as_bytes())
            .map_err(|_| "invalid out path".to_owned())?;
        let mut err_ptr: *mut c_char = std::ptr::null_mut();
        let rc = func(frame, output.as_ptr(), &mut err_ptr);
        if rc != 0 {
            let message = if err_ptr.is_null() {
                "published lifecycle V2 compile failed".to_owned()
            } else {
                CStr::from_ptr(err_ptr).to_string_lossy().into_owned()
            };
            if !err_ptr.is_null() {
                free(err_ptr as *mut c_void);
            }
            return Err(message);
        }
        transport_io::ensure_backend_artifact_written(obj_out, "object")
    }
}

#[cfg(feature = "plugins")]
pub(super) fn compile_published_lifecycle_body_v2(
    json_in: &Path,
    frame: &PublishedLifecycleCFrameHeaderV2,
    sites: &[PublishedLifecycleBodySiteCRowV1],
    obj_out: &Path,
) -> Result<(), String> {
    use std::os::raw::{c_char, c_int, c_void};
    extern "C" {
        fn free(ptr: *mut c_void);
    }
    if sites.is_empty() {
        return Err("published lifecycle body requires NewBox sites".into());
    }
    unsafe {
        let lib = load_ffi_library()?;
        type CompileFn = unsafe extern "C" fn(
            *const c_char,
            *const PublishedLifecycleCFrameHeaderV2,
            *const PublishedLifecycleBodySiteCRowV1,
            usize,
            *const c_char,
            *mut *mut c_char,
        ) -> c_int;
        let func: libloading::Symbol<CompileFn> = lib
            .get(b"hako_llvmc_compile_published_lifecycle_body_v2\0")
            .map_err(|error| {
                format!("dlsym failed for published lifecycle body V2 ingress: {error}")
            })?;
        let input = CString::new(json_in.to_string_lossy().as_bytes())
            .map_err(|_| "invalid json path".to_owned())?;
        let output = CString::new(obj_out.to_string_lossy().as_bytes())
            .map_err(|_| "invalid out path".to_owned())?;
        let mut err_ptr: *mut c_char = std::ptr::null_mut();
        let rc = func(
            input.as_ptr(),
            frame,
            sites.as_ptr(),
            sites.len(),
            output.as_ptr(),
            &mut err_ptr,
        );
        if rc != 0 {
            let message = if err_ptr.is_null() {
                "published lifecycle body V2 compile failed".into()
            } else {
                CStr::from_ptr(err_ptr).to_string_lossy().into_owned()
            };
            if !err_ptr.is_null() {
                free(err_ptr as *mut c_void);
            }
            return Err(message);
        }
        transport_io::ensure_backend_artifact_written(obj_out, "object")
    }
}

pub(super) fn compile_published_lifecycle_body_v3(
    json_in: &Path,
    frame: &PublishedLifecycleCFrameHeaderV2,
    sites: &[PublishedLifecycleBodySiteCRowV1],
    session: &LifecycleRuntimeSessionV1,
    obj_out: &Path,
) -> Result<(), String> {
    use std::os::raw::{c_char, c_int, c_void};
    extern "C" {
        fn free(ptr: *mut c_void);
    }
    if sites.is_empty() {
        return Err("published lifecycle body requires NewBox sites".into());
    }
    let d = session.descriptor();
    let triple =
        CString::new(d.target_triple.as_str()).map_err(|_| "invalid lifecycle target triple")?;
    let row = LifecycleTargetSessionCRowV1 {
        revision: 1,
        target_triple: triple.as_ptr(),
        endian: d.endian,
        pointer_width: d.pointer_width,
        fault_abi_version: d.fault_abi_version,
        status_abi_version: d.status_abi_version,
        diagnostic_size: d.diagnostic_size,
        diagnostic_align: d.diagnostic_align,
        diagnostic_site_offset: d.diagnostic_site_offset,
        diagnostic_details_offset: d.diagnostic_details_offset,
        diagnostic_message_offset: d.diagnostic_message_offset,
        frame_size: d.frame_size,
        frame_align: d.frame_align,
        frame_primary_offset: d.frame_primary_offset,
        frame_suppressed_offset: d.frame_suppressed_offset,
    };
    unsafe {
        let lib = load_ffi_library()?;
        type CompileFn = unsafe extern "C" fn(
            *const c_char,
            *const PublishedLifecycleCFrameHeaderV2,
            *const PublishedLifecycleBodySiteCRowV1,
            usize,
            *const LifecycleTargetSessionCRowV1,
            *const c_char,
            *mut *mut c_char,
        ) -> c_int;
        let func: libloading::Symbol<CompileFn> = lib
            .get(b"hako_llvmc_compile_published_lifecycle_body_v3\0")
            .map_err(|e| format!("dlsym failed for lifecycle V3 ingress: {e}"))?;
        let input =
            CString::new(json_in.to_string_lossy().as_bytes()).map_err(|_| "invalid json path")?;
        let output =
            CString::new(obj_out.to_string_lossy().as_bytes()).map_err(|_| "invalid out path")?;
        let mut error: *mut c_char = std::ptr::null_mut();
        if func(
            input.as_ptr(),
            frame,
            sites.as_ptr(),
            sites.len(),
            &row,
            output.as_ptr(),
            &mut error,
        ) != 0
        {
            let message = if error.is_null() {
                "published lifecycle V3 compile failed".into()
            } else {
                CStr::from_ptr(error).to_string_lossy().into_owned()
            };
            if !error.is_null() {
                free(error as *mut c_void);
            }
            return Err(message);
        }
        transport_io::ensure_backend_artifact_written(obj_out, "object")
    }
}

pub(super) fn compile_via_capi_keep(
    mir_json: &str,
    compile_symbol: &[u8],
    compile_recipe: Option<&str>,
    compat_replay: Option<&str>,
    opts: &Opts,
) -> Result<PathBuf, String> {
    normalize::validate_backend_mir_shape(mir_json)?;
    let in_path = transport_io::prepare_backend_input_json_file(mir_json)?;
    let out_path = transport_paths::resolve_backend_object_output(opts);
    transport_io::ensure_backend_output_parent(&out_path);
    compile_via_capi(
        &in_path,
        &out_path,
        compile_symbol,
        compile_recipe,
        compat_replay,
        opts,
    )?;
    Ok(out_path)
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

#[cfg(not(feature = "plugins"))]
pub(super) fn compile_published_static_method_v1(
    _json_in: &Path,
    _obj_out: &Path,
    _rows: &[PublishedStaticMethodCallCRowV1],
    _opts: &Opts,
) -> Result<(), String> {
    Err("capi not available (plugins feature disabled)".into())
}

#[cfg(not(feature = "plugins"))]
pub(super) fn compile_published_lifecycle_v2(
    _frame: &PublishedLifecycleCFrameHeaderV2,
    _obj_out: &Path,
) -> Result<(), String> {
    Err("capi not available (plugins feature disabled)".into())
}

#[cfg(not(feature = "plugins"))]
pub(super) fn compile_published_lifecycle_body_v2(
    _json_in: &Path,
    _frame: &PublishedLifecycleCFrameHeaderV2,
    _sites: &[PublishedLifecycleBodySiteCRowV1],
    _obj_out: &Path,
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
        transport_io::ensure_backend_artifact_written(exe_out, "exe")?;
        Ok(())
    }
}

#[cfg(feature = "plugins")]
pub(super) fn link_via_capi_v2(
    obj_in: &Path,
    exe_out: &Path,
    runtime_archive: &Path,
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
            *const c_char,
            *mut *mut c_char,
        ) -> c_int;
        let func: libloading::Symbol<LinkFn> = lib
            .get(b"hako_llvmc_link_obj_v2\0")
            .map_err(|e| format!("dlsym failed for explicit link: {}", e))?;
        let cobj = CString::new(obj_in.to_string_lossy().as_bytes())
            .map_err(|_| "invalid obj path".to_owned())?;
        let cexe = CString::new(exe_out.to_string_lossy().as_bytes())
            .map_err(|_| "invalid exe path".to_owned())?;
        let car = CString::new(runtime_archive.to_string_lossy().as_bytes())
            .map_err(|_| "invalid runtime archive path".to_owned())?;
        let cflags = extra_ldflags
            .map(|flags| CString::new(flags).map_err(|_| "invalid ldflags".to_owned()))
            .transpose()?;
        let mut err_ptr: *mut c_char = std::ptr::null_mut();
        let rc = func(
            cobj.as_ptr(),
            cexe.as_ptr(),
            car.as_ptr(),
            cflags.as_ref().map_or(std::ptr::null(), |v| v.as_ptr()),
            &mut err_ptr as *mut *mut c_char,
        );
        if rc != 0 {
            let msg = if !err_ptr.is_null() {
                CStr::from_ptr(err_ptr).to_string_lossy().into_owned()
            } else {
                "explicit link failed".to_owned()
            };
            if !err_ptr.is_null() {
                free(err_ptr as *mut c_void);
            }
            return Err(msg);
        }
        transport_io::ensure_backend_artifact_written(exe_out, "exe")
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

#[cfg(not(feature = "plugins"))]
pub(super) fn link_via_capi_v2(
    _obj_in: &Path,
    _exe_out: &Path,
    _runtime_archive: &Path,
    _extra_ldflags: Option<&str>,
) -> Result<(), String> {
    Err("capi not available (plugins feature disabled)".into())
}
