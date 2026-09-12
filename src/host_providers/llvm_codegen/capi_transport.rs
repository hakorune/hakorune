use std::ffi::{CStr, CString};
use std::path::{Path, PathBuf};

use super::defaults;
use super::lifecycle_invocation::LifecycleInvocationInputV1;
use super::normalize;
use super::transport_io;
use super::transport_paths;
use super::Opts;

const COMPILE_OPTIONS_SYMBOL: &[u8] = b"hako_llvmc_compile_json_with_options_v1\0";

#[repr(C)]
struct PhysicalCompileContractCRowV1 {
    revision: u32,
    byte_size: u32,
    ingress_profile: u32,
    flags: u32,
    compile_recipe: *const std::os::raw::c_char,
    compat_replay: *const std::os::raw::c_char,
    opt_level: *const std::os::raw::c_char,
    opt_tool_path: *const std::os::raw::c_char,
    llc_tool_path: *const std::os::raw::c_char,
    llc_flags: *const std::os::raw::c_char,
    llvmc_path: *const std::os::raw::c_char,
}

struct OwnedPhysicalCompileContract {
    compile_recipe: CString,
    compat_replay: CString,
    opt_level: CString,
    opt_tool_path: Option<CString>,
    llc_tool_path: Option<CString>,
    llc_flags: Option<CString>,
}

impl OwnedPhysicalCompileContract {
    fn from_request(
        compile_recipe: Option<&str>,
        compat_replay: Option<&str>,
        opts: &Opts,
    ) -> Result<Self, String> {
        let compile_recipe = compile_recipe
            .filter(|value| *value == "pure-first")
            .ok_or_else(|| "[freeze:contract][compile-options/recipe] expected pure-first".to_string())?;
        let compat_replay = compat_replay
            .filter(|value| *value == "none")
            .ok_or_else(|| "[freeze:contract][compile-options/replay] expected none".to_string())?;
        let opt_level = explicit_opt_level(opts)?;
        validate_opt_level(&opt_level)?;
        let opt_tool_path = option_env("NYASH_NY_LLVM_OPT_TOOL")?;
        let llc_tool_path = option_env("NYASH_NY_LLVM_LLC_TOOL")?;
        let llc_flags = option_env("NYASH_NY_LLVM_LLC_FLAGS")?;
        validate_tool_path(opt_tool_path.as_deref(), "opt")?;
        validate_tool_path(llc_tool_path.as_deref(), "llc")?;
        Ok(Self {
            compile_recipe: cstring(compile_recipe, "compile recipe")?,
            compat_replay: cstring(compat_replay, "compat replay")?,
            opt_level: cstring(&opt_level, "opt level")?,
            opt_tool_path: opt_tool_path
                .as_deref()
                .map(|value| cstring(value, "opt tool"))
                .transpose()?,
            llc_tool_path: llc_tool_path
                .as_deref()
                .map(|value| cstring(value, "llc tool"))
                .transpose()?,
            llc_flags: llc_flags
                .as_deref()
                .map(|value| cstring(value, "llc flags"))
                .transpose()?,
        })
    }

    fn row_for_profile(&self, ingress_profile: u32) -> PhysicalCompileContractCRowV1 {
        fn pointer(value: Option<&CString>) -> *const std::os::raw::c_char {
            value.map_or(std::ptr::null(), |value| value.as_ptr())
        }
        PhysicalCompileContractCRowV1 {
            revision: 1,
            byte_size: std::mem::size_of::<PhysicalCompileContractCRowV1>() as u32,
            ingress_profile,
            flags: 0,
            compile_recipe: self.compile_recipe.as_ptr(),
            compat_replay: self.compat_replay.as_ptr(),
            opt_level: self.opt_level.as_ptr(),
            opt_tool_path: pointer(self.opt_tool_path.as_ref()),
            llc_tool_path: pointer(self.llc_tool_path.as_ref()),
            llc_flags: pointer(self.llc_flags.as_ref()),
            llvmc_path: std::ptr::null(),
        }
    }

    fn row(&self) -> PhysicalCompileContractCRowV1 {
        self.row_for_profile(1)
    }
}

fn cstring(value: &str, label: &str) -> Result<CString, String> {
    CString::new(value).map_err(|_| format!("[freeze:contract][compile-options/{label}] NUL"))
}

fn option_env(name: &str) -> Result<Option<String>, String> {
    std::env::var(name)
        .map(|value| (!value.is_empty()).then_some(value))
        .or_else(|error| {
            if error == std::env::VarError::NotPresent {
                Ok(None)
            } else {
                Err(format!("[freeze:contract][compile-options/{name}] invalid UTF-8"))
            }
        })
}

fn explicit_opt_level(opts: &Opts) -> Result<String, String> {
    if let Some(level) = opts.opt_level.as_deref() {
        return Ok(level.to_string());
    }
    let hako = std::env::var("HAKO_LLVM_OPT_LEVEL").ok();
    let nyash = std::env::var("NYASH_LLVM_OPT_LEVEL").ok();
    match (hako, nyash) {
        (Some(left), Some(right)) if left != right => Err(
            "[freeze:contract][compile-options/opt-level-alias-conflict]".to_string(),
        ),
        (Some(level), _) | (_, Some(level)) => Ok(level),
        (None, None) => Ok("0".to_string()),
    }
}

fn validate_opt_level(level: &str) -> Result<(), String> {
    if level.len() == 1 && matches!(level.as_bytes()[0], b'0'..=b'3') {
        Ok(())
    } else {
        Err("[freeze:contract][compile-options/opt-level] expected 0..3".to_string())
    }
}

fn validate_tool_path(path: Option<&str>, label: &str) -> Result<(), String> {
    let Some(path) = path else { return Ok(()); };
    if which::which(path).is_ok() {
        Ok(())
    } else {
        Err(format!("[freeze:contract][compile-options/{label}-tool] not found"))
    }
}

#[repr(C)]
struct LifecycleTargetSessionCRowV2 {
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
    map_size: u32,
    map_align: u32,
    map_revision: u32,
    key_size: u32,
    key_align: u32,
    key_revision: u32,
    outcome_size: u32,
    outcome_align: u32,
    outcome_revision: u32,
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

#[cfg(feature = "plugins")]
fn compile_via_capi_with_options(
    json_in: &Path,
    obj_out: &Path,
    compile_recipe: Option<&str>,
    compat_replay: Option<&str>,
    opts: &Opts,
) -> Result<(), String> {
    use std::os::raw::{c_char, c_int, c_void};

    extern "C" {
        fn free(ptr: *mut c_void);
    }

    let owned = OwnedPhysicalCompileContract::from_request(
        compile_recipe,
        compat_replay,
        opts,
    )?;
    let contract = owned.row();
    unsafe {
        let lib = load_ffi_library()?;
        type CompileFn = unsafe extern "C" fn(
            *const c_char,
            *const c_char,
            *const PhysicalCompileContractCRowV1,
            *mut *mut c_char,
        ) -> c_int;
        let func: libloading::Symbol<CompileFn> = lib
            .get(COMPILE_OPTIONS_SYMBOL)
            .map_err(|e| format!("dlsym failed for explicit compile options: {e}"))?;
        let cin = CString::new(json_in.to_string_lossy().as_bytes())
            .map_err(|_| "invalid json path".to_string())?;
        let cout = CString::new(obj_out.to_string_lossy().as_bytes())
            .map_err(|_| "invalid out path".to_string())?;
        let mut err_ptr: *mut c_char = std::ptr::null_mut();
        let rc = func(cin.as_ptr(), cout.as_ptr(), &contract, &mut err_ptr);
        if rc != 0 {
            let msg = if err_ptr.is_null() {
                "compile failed".to_string()
            } else {
                CStr::from_ptr(err_ptr).to_string_lossy().into_owned()
            };
            if !err_ptr.is_null() {
                free(err_ptr as *mut c_void);
            }
            return Err(msg);
        }
        transport_io::ensure_backend_artifact_written(obj_out, "object")
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
    if compile_recipe == Some("pure-first") && compat_replay == Some("none") {
        return compile_via_capi_with_options(
            json_in,
            obj_out,
            compile_recipe,
            compat_replay,
            opts,
        );
    }
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

#[cfg(feature = "plugins")]
pub(super) fn compile_published_lifecycle_physical_v4(
    input: &LifecycleInvocationInputV1<'_, '_>,
    obj_out: &Path,
) -> Result<(), String> {
    use std::os::raw::{c_char, c_int, c_void};
    extern "C" {
        fn free(ptr: *mut c_void);
    }
    let d = input.session().descriptor();
    let triple =
        CString::new(d.target_triple.as_str()).map_err(|_| "invalid lifecycle target triple")?;
    let row = LifecycleTargetSessionCRowV2 {
        revision: 2,
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
        map_size: d.map_size,
        map_align: d.map_align,
        map_revision: d.map_revision,
        key_size: d.key_size,
        key_align: d.key_align,
        key_revision: d.key_revision,
        outcome_size: d.outcome_size,
        outcome_align: d.outcome_align,
        outcome_revision: d.outcome_revision,
    };
    let json_in = transport_io::prepare_backend_input_json_file(&input.serialize()?)?;
    transport_io::ensure_backend_output_parent(obj_out);
    let result = (|| unsafe {
        let lib = load_ffi_library()?;
        type CompileFn = unsafe extern "C" fn(
            *const c_char,
            *const LifecycleTargetSessionCRowV2,
            *const c_char,
            *mut *mut c_char,
        ) -> c_int;
        let func: libloading::Symbol<CompileFn> = lib
            .get(b"hako_llvmc_compile_published_lifecycle_physical_v4\0")
            .map_err(|e| format!("dlsym failed for lifecycle V4 ingress: {e}"))?;
        let input =
            CString::new(json_in.to_string_lossy().as_bytes()).map_err(|_| "invalid json path")?;
        let output =
            CString::new(obj_out.to_string_lossy().as_bytes()).map_err(|_| "invalid out path")?;
        let mut error: *mut c_char = std::ptr::null_mut();
        if func(input.as_ptr(), &row, output.as_ptr(), &mut error) != 0 {
            let message = if error.is_null() {
                "published lifecycle V4 compile failed".into()
            } else {
                CStr::from_ptr(error).to_string_lossy().into_owned()
            };
            if !error.is_null() {
                free(error as *mut c_void);
            }
            return Err(message);
        }
        transport_io::ensure_backend_artifact_written(obj_out, "object")
    })();
    transport_io::remove_backend_temp_file(&json_in);
    result
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
pub(super) fn compile_published_lifecycle_physical_v4(
    _input: &LifecycleInvocationInputV1<'_, '_>,
    _obj_out: &Path,
) -> Result<(), String> {
    Err("capi not available (plugins feature disabled)".into())
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

#[path = "static_invocation.rs"]
mod static_invocation;
pub(super) use static_invocation::compile_published_static_v2;
