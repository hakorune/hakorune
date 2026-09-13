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
        let llc_flags = option_env_preserve_empty("NYASH_NY_LLVM_LLC_FLAGS")?;
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

fn option_env_preserve_empty(name: &str) -> Result<Option<String>, String> {
    std::env::var(name)
        .map(Some)
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
    opts: &Opts,
) -> Result<(), String> {
    compile_via_capi_with_options(
        json_in,
        obj_out,
        opts.compile_recipe.as_deref(),
        opts.compat_replay.as_deref(),
        opts,
    )
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
        let input = CString::new(json_in.path().to_string_lossy().as_bytes())
            .map_err(|_| "invalid json path")?;
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
    result
}

pub(super) fn compile_via_capi_keep(
    mir_json: &str,
    opts: &Opts,
) -> Result<PathBuf, String> {
    normalize::validate_backend_mir_shape(mir_json)?;
    let input = transport_io::prepare_backend_input_json_file(mir_json)?;
    let out_path = transport_paths::resolve_backend_object_output(opts);
    transport_io::ensure_backend_output_parent(&out_path);
    compile_via_capi(input.path(), &out_path, opts)?;
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

#[cfg(all(test, feature = "plugins", unix))]
mod integration_tests {
    use super::{compile_via_capi_keep, compile_via_capi_with_options};
    use crate::host_providers::llvm_codegen::boundary_default_object_opts;
    use std::fs;
    use std::os::unix::fs::PermissionsExt;
    use std::path::{Path, PathBuf};
    use std::process::Command;

    fn executable(path: &Path, body: &str) {
        fs::write(path, body).expect("write CAPI test executable");
        let mut permissions = fs::metadata(path)
            .expect("CAPI test executable metadata")
            .permissions();
        permissions.set_mode(0o755);
        fs::set_permissions(path, permissions).expect("make CAPI test executable");
    }

    fn fake_llvm_tool(path: &Path) {
        executable(
            path,
            r#"#!/bin/sh
out=
while [ "$#" -gt 0 ]; do
  if [ "$1" = "-o" ]; then out="$2"; shift 2; continue; fi
  shift
done
case "${HAKO_CAPI_TOOL_MODE:-}" in
  fail-opt) [ "${0##*/}" = "opt" ] && exit 17 ;;
  fail-llc) [ "${0##*/}" = "llc" ] && exit 19 ;;
esac
[ -n "$out" ] && : > "$out"
exit 0
"#,
        );
    }

    fn build_missing_symbol_library(path: &Path) {
        let source = path.with_extension("c");
        fs::write(&source, "int unrelated_symbol(void) { return 0; }\n")
            .expect("write missing-symbol library source");
        crate::test_support::with_process_state_lock(|| {
            let status = Command::new("cc")
                .args(["-shared", "-fPIC", "-o"])
                .arg(path)
                .arg(&source)
                .status()
                .expect("invoke cc for missing-symbol library");
            assert!(status.success(), "missing-symbol library build failed");
        });
    }

    fn build_input_observer_library(path: &Path) {
        let source = path.with_extension("c");
        fs::write(
            &source,
            r#"#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int hako_llvmc_compile_json_with_options_v1(
    const char* input, const char* output, const void* options, char** error) {
  const char* record = getenv("HAKO_CAPI_RECORD_PATH");
  FILE* in;
  FILE* copy;
  FILE* out;
  char path[4096];
  int n;
  (void)options;
  if (!input || !output || !record) return -2;
  n = snprintf(path, sizeof(path), "%s.path", record);
  if (n <= 0 || (size_t)n >= sizeof(path)) return -3;
  copy = fopen(path, "wb");
  if (!copy) return -4;
  fputs(input, copy);
  if (fclose(copy) != 0) return -5;
  n = snprintf(path, sizeof(path), "%s.input", record);
  if (n <= 0 || (size_t)n >= sizeof(path)) return -6;
  in = fopen(input, "rb");
  copy = fopen(path, "wb");
  if (!in || !copy) return -7;
  {
    int ch;
    while ((ch = fgetc(in)) != EOF) fputc(ch, copy);
  }
  fclose(in);
  fclose(copy);
  out = fopen(output, "wb");
  if (!out) return -8;
  fputs("capi-observer-object", out);
  return fclose(out) == 0 ? 0 : -9;
}
"#,
        )
        .expect("write CAPI input observer source");
        crate::test_support::with_process_state_lock(|| {
            let status = Command::new("cc")
                .args(["-shared", "-fPIC", "-o"])
                .arg(path)
                .arg(&source)
                .status()
                .expect("invoke cc for CAPI input observer");
            assert!(status.success(), "CAPI input observer build failed");
        });
    }

    fn capi_opts(out: &Path) -> crate::host_providers::llvm_codegen::Opts {
        boundary_default_object_opts(Some(out.to_path_buf()), None, Some("0".into()), None)
    }

    fn run_with_capi_env<R>(
        ffi: &Path,
        opt: Option<&Path>,
        llc: Option<&Path>,
        mode: Option<&str>,
        f: impl FnOnce() -> R,
    ) -> R {
        let ffi = ffi.to_string_lossy().into_owned();
        let opt = opt.map(|path| path.to_string_lossy().into_owned());
        let llc = llc.map(|path| path.to_string_lossy().into_owned());
        crate::test_support::with_env_vars(
            &[
                ("HAKO_AOT_FFI_LIB", Some(ffi.as_str())),
                ("HAKO_LLVM_OPT_LEVEL", Some("0")),
                ("NYASH_LLVM_OPT_LEVEL", None),
                ("NYASH_NY_LLVM_OPT_TOOL", opt.as_deref()),
                ("NYASH_NY_LLVM_LLC_TOOL", llc.as_deref()),
                ("NYASH_NY_LLVM_LLC_FLAGS", None),
                ("HAKO_CAPI_TOOL_MODE", mode),
            ],
            f,
        )
    }

    fn run_with_capi_env_and_record<R>(
        ffi: &Path,
        record: &Path,
        f: impl FnOnce() -> R,
    ) -> R {
        let ffi = ffi.to_string_lossy().into_owned();
        let record = record.to_string_lossy().into_owned();
        crate::test_support::with_env_vars(
            &[
                ("HAKO_AOT_FFI_LIB", Some(ffi.as_str())),
                ("HAKO_LLVM_OPT_LEVEL", Some("0")),
                ("NYASH_LLVM_OPT_LEVEL", None),
                ("NYASH_NY_LLVM_OPT_TOOL", None),
                ("NYASH_NY_LLVM_LLC_TOOL", None),
                ("NYASH_NY_LLVM_LLC_FLAGS", None),
                ("HAKO_CAPI_TOOL_MODE", None),
                ("HAKO_CAPI_RECORD_PATH", Some(record.as_str())),
            ],
            f,
        )
    }

    #[test]
    fn capi_production_wrapper_exercises_contract_loader_child_and_success_paths() {
        std::thread::Builder::new()
            .name("llvm-capi-input-ownership".to_string())
            .stack_size(32 * 1024 * 1024)
            .spawn(capi_production_wrapper_exercises_contract_loader_child_and_success_paths_inner)
            .expect("CAPI integration thread should start")
            .join()
            .expect("CAPI integration thread should finish");
    }

    fn capi_production_wrapper_exercises_contract_loader_child_and_success_paths_inner() {
        let workspace = tempfile::tempdir().expect("CAPI integration tempdir");
        let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let ffi = manifest.join("target/release/libhako_llvmc_ffi.so");
        crate::test_support::with_process_state_lock(|| {
            let build = Command::new("bash")
                .arg(manifest.join("tools/build_hako_llvmc_ffi.sh"))
                .status()
                .expect("invoke CAPI library build");
            assert!(build.success(), "CAPI library build failed");
        });
        assert!(ffi.is_file(), "missing built CAPI library: {}", ffi.display());
        let mir_json = fs::read_to_string(
            manifest.join("apps/tests/mir_shape_guard/ret_const_min_v1.mir.json"),
        )
        .expect("read CAPI MIR fixture");

        // Rust-side contract validation runs before dlopen and before any
        // child can consume the invocation-owned input.
        let input = workspace.path().join("invalid-contract.json");
        let invalid_output = workspace.path().join("invalid-contract.o");
        fs::write(&input, &mir_json).expect("write invalid-contract input");
        let mut invalid = capi_opts(&invalid_output);
        invalid.compile_recipe = Some("wrong-recipe".into());
        let error = compile_via_capi_with_options(
            &input,
            &invalid_output,
            invalid.compile_recipe.as_deref(),
            invalid.compat_replay.as_deref(),
            &invalid,
        )
        .expect_err("invalid compile recipe must fail before loading CAPI");
        assert!(error.contains("compile-options/recipe"), "{error}");
        assert!(!invalid_output.exists());

        // A configured tool that is absent is rejected by the Rust contract
        // owner before the dynamic library or a child is reached.
        let missing_tool = workspace.path().join("missing-opt");
        let missing_output = workspace.path().join("missing-tool.o");
        run_with_capi_env(&ffi, Some(&missing_tool), None, None, || {
            let error = compile_via_capi_keep(&mir_json, &capi_opts(&missing_output))
                .expect_err("missing configured opt must fail");
            assert!(error.contains("compile-options/opt-tool"), "{error}");
            assert!(!missing_output.exists());
        });

        // dlopen succeeds but the production symbol is absent: this proves
        // the real loader/owner path reaches the missing-symbol terminal.
        let missing_symbol = workspace.path().join("libmissing_symbol.so");
        build_missing_symbol_library(&missing_symbol);
        let symbol_output = workspace.path().join("missing-symbol.o");
        run_with_capi_env(&missing_symbol, None, None, None, || {
            let error = compile_via_capi_keep(&mir_json, &capi_opts(&symbol_output))
                .expect_err("missing CAPI symbol must fail");
            assert!(
                error.contains("dlsym failed for explicit compile options"),
                "{error}"
            );
            assert!(!symbol_output.exists());
        });

        let opt = workspace.path().join("opt");
        let llc = workspace.path().join("llc");
        fake_llvm_tool(&opt);
        fake_llvm_tool(&llc);

        // The C child failure is exercised through compile_via_capi_keep,
        // with both explicit tools validated and then consumed by the C ABI.
        let child_output = workspace.path().join("child-failure.o");
        run_with_capi_env(&ffi, Some(&opt), Some(&llc), Some("fail-opt"), || {
            let error = compile_via_capi_keep(&mir_json, &capi_opts(&child_output))
                .expect_err("C child failure must be returned");
            assert!(!error.trim().is_empty());
            assert!(!child_output.exists());
        });

        // Both fake tools create their requested artifacts.  The returned
        // object proves the CAPI success terminal and the Rust owner drops its
        // temporary input after this synchronous consumer returns.
        let success_output = workspace.path().join("success.o");
        run_with_capi_env(&ffi, Some(&opt), Some(&llc), None, || {
            let result = compile_via_capi_keep(&mir_json, &capi_opts(&success_output))
                .expect("CAPI success should return an object");
            assert_eq!(result, success_output);
            assert!(success_output.is_file());
        });

        // A tiny C ABI observer records the exact owned input path and bytes.
        // The assertions after the call prove the consumer reopened the file
        // synchronously and the Rust owner removed it on return.
        let observer = workspace.path().join("libinput_observer.so");
        build_input_observer_library(&observer);
        let observer_record = workspace.path().join("capi-observer-record");
        let observer_output = workspace.path().join("capi-observer.o");
        run_with_capi_env_and_record(&observer, &observer_record, || {
            let result = compile_via_capi_keep(&mir_json, &capi_opts(&observer_output))
                .expect("CAPI observer success should return an object");
            assert_eq!(result, observer_output);
            assert_eq!(fs::read(&observer_output).unwrap(), b"capi-observer-object");
        });
        let observer_input_path = fs::read_to_string(observer_record.with_extension("path"))
            .expect("CAPI observer recorded input path");
        assert_eq!(
            fs::read(observer_record.with_extension("input")).unwrap(),
            mir_json.as_bytes()
        );
        assert!(!Path::new(&observer_input_path).exists());
    }
}
