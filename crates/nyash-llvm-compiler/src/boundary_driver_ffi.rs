use std::ffi::{CStr, CString};
use std::fs;
use std::path::Path;

use anyhow::{bail, Context, Result};
use libloading::Library;
use std::os::raw::{c_char, c_int, c_void};
use std::sync::OnceLock;

extern "C" {
    fn free(ptr: *mut c_void);
}

const COMPILE_OPTIONS_SYMBOL: &[u8] = b"hako_llvmc_compile_json_with_options_v1\0";
const CONTRACT_REVISION: u32 = 1;
const BOUNDARY_PURE_FIRST_PROFILE: u32 = 1;

#[repr(C)]
struct PhysicalCompileContractV1 {
    revision: u32,
    byte_size: u32,
    ingress_profile: u32,
    flags: u32,
    compile_recipe: *const c_char,
    compat_replay: *const c_char,
    opt_level: *const c_char,
    opt_tool_path: *const c_char,
    llc_tool_path: *const c_char,
    llc_flags: *const c_char,
    llvmc_path: *const c_char,
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
    fn from_environment() -> Result<Self> {
        let legacy_alias = environment_value("HAKO_CAPI_PURE")?;
        if legacy_capi_pure_alias_enabled(legacy_alias.as_deref()) {
            warn_legacy_capi_pure_alias_once();
            bail!(
                "[freeze:contract][env/hako_capi_pure_retired] HAKO_CAPI_PURE is retired; use HAKO_BACKEND_COMPILE_RECIPE=pure-first"
            );
        }
        let recipe = environment_value("HAKO_BACKEND_COMPILE_RECIPE")?;
        let replay = environment_value("HAKO_BACKEND_COMPAT_REPLAY")?;
        let (recipe, replay) = admit_boundary_route(recipe.as_deref(), replay.as_deref())?;
        let opt_level = environment_opt_level()?;
        validate_opt_level(&opt_level)?;
        let opt_tool_path = optional_environment_value("NYASH_NY_LLVM_OPT_TOOL")?;
        let llc_tool_path = optional_environment_value("NYASH_NY_LLVM_LLC_TOOL")?;
        let llc_flags = optional_environment_value("NYASH_NY_LLVM_LLC_FLAGS")?;
        Ok(Self {
            compile_recipe: cstring(recipe, "compile recipe")?,
            compat_replay: cstring(replay, "compat replay")?,
            opt_level: cstring(&opt_level, "opt level")?,
            opt_tool_path: optional_cstring(opt_tool_path, "opt tool")?,
            llc_tool_path: optional_cstring(llc_tool_path, "llc tool")?,
            llc_flags: optional_cstring(llc_flags, "llc flags")?,
        })
    }

    fn row(&self) -> PhysicalCompileContractV1 {
        fn pointer(value: Option<&CString>) -> *const c_char {
            value.map_or(std::ptr::null(), |value| value.as_ptr())
        }
        PhysicalCompileContractV1 {
            revision: CONTRACT_REVISION,
            byte_size: std::mem::size_of::<PhysicalCompileContractV1>() as u32,
            ingress_profile: BOUNDARY_PURE_FIRST_PROFILE,
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
}

type CompileOptionsFn = unsafe extern "C" fn(
    *const c_char,
    *const c_char,
    *const PhysicalCompileContractV1,
    *mut *mut c_char,
) -> c_int;
type LinkFnV2 = unsafe extern "C" fn(
    *const c_char,
    *const c_char,
    *const c_char,
    *const c_char,
    *mut *mut c_char,
) -> c_int;

pub(super) fn emit_object_from_json(input: &Path, out: &Path) -> Result<()> {
    call_compile_options(input, out)
}

pub(super) fn link_object_to_exe(
    obj: &Path,
    out_exe: &Path,
    nyrt_dir: Option<&Path>,
    extra_libs: Option<&str>,
) -> Result<()> {
    ensure_output_parent(out_exe);
    let archive = require_explicit_nyrt_archive(nyrt_dir)?;
    link_object_to_exe_with_archive(obj, out_exe, &archive, extra_libs)
}

pub(super) fn link_object_to_exe_with_archive(
    obj: &Path,
    out_exe: &Path,
    runtime_archive: &Path,
    extra_libs: Option<&str>,
) -> Result<()> {
    ensure_output_parent(out_exe);
    if !runtime_archive.is_file() {
        bail!(
            "explicit runtime archive is missing: {}",
            runtime_archive.display()
        );
    }
    call_link_symbol_v2(obj, out_exe, runtime_archive, extra_libs)
}

fn ensure_output_parent(path: &Path) {
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
}

unsafe fn with_link_symbol_v2<T, F>(action: F) -> Result<T>
where
    F: FnOnce(LinkFnV2) -> Result<T>,
{
    let lib = open_ffi_library()?;
    let func: LinkFnV2 = *lib
        .get(b"hako_llvmc_link_obj_v2\0")
        .context("missing symbol hako_llvmc_link_obj_v2")?;
    action(func)
}

fn call_compile_options(input: &Path, out: &Path) -> Result<()> {
    let owned = OwnedPhysicalCompileContract::from_environment()?;
    let contract = owned.row();
    let cin =
        CString::new(input.to_string_lossy().as_bytes()).context("invalid input path for C ABI")?;
    let cout =
        CString::new(out.to_string_lossy().as_bytes()).context("invalid output path for C ABI")?;
    ensure_output_parent(out);
    let mut err_ptr: *mut c_char = std::ptr::null_mut();
    unsafe {
        let lib = open_ffi_library()?;
        let func: CompileOptionsFn = *lib
            .get(COMPILE_OPTIONS_SYMBOL)
            .context("missing symbol hako_llvmc_compile_json_with_options_v1")?;
        emit_compile_route_trace("pure-first", "none");
        let rc = func(
            cin.as_ptr(),
            cout.as_ptr(),
            &contract,
            &mut err_ptr as *mut *mut c_char,
        );
        interpret_result(rc, err_ptr, out, "object not produced")
    }
}

fn call_link_symbol_v2(
    obj: &Path,
    out_exe: &Path,
    runtime_archive: &Path,
    extra_libs: Option<&str>,
) -> Result<()> {
    let cobj =
        CString::new(obj.to_string_lossy().as_bytes()).context("invalid object path for C ABI")?;
    let cexe = CString::new(out_exe.to_string_lossy().as_bytes())
        .context("invalid executable path for C ABI")?;
    let carchive = CString::new(runtime_archive.to_string_lossy().as_bytes())
        .context("invalid runtime archive path for C ABI")?;
    let libs_owned = extra_libs
        .filter(|value| !value.trim().is_empty())
        .map(CString::new)
        .transpose()
        .context("invalid linker flags for C ABI")?;
    let libs_ptr = libs_owned
        .as_ref()
        .map(|value| value.as_ptr())
        .unwrap_or(std::ptr::null());
    let mut err_ptr: *mut c_char = std::ptr::null_mut();
    unsafe {
        with_link_symbol_v2(|func| {
            let rc = func(
                cobj.as_ptr(),
                cexe.as_ptr(),
                carchive.as_ptr(),
                libs_ptr,
                &mut err_ptr as *mut *mut c_char,
            );
            interpret_result(rc, err_ptr, out_exe, "exe not produced")
        })
    }
}

fn require_explicit_nyrt_archive(nyrt_dir: Option<&Path>) -> Result<std::path::PathBuf> {
    let dir = nyrt_dir.context("explicit --nyrt <DIR> is required for Boundary exe linking")?;
    let archive = dir.join("libnyash_kernel.a");
    if !archive.is_file() {
        bail!(
            "libnyash_kernel.a not found in {} (Boundary v2 requires an explicit archive path)",
            dir.display()
        );
    }
    Ok(archive)
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

fn llvm_route_trace_enabled() -> bool {
    matches!(
        std::env::var("NYASH_LLVM_ROUTE_TRACE").ok().as_deref(),
        Some("1" | "on" | "true" | "yes")
    )
}

fn legacy_capi_pure_alias_enabled(value: Option<&str>) -> bool {
    matches!(value, Some("1" | "on" | "true" | "yes"))
}

fn warn_legacy_capi_pure_alias_once() {
    static WARNED: OnceLock<()> = OnceLock::new();
    if WARNED.set(()).is_ok() {
        eprintln!(
            "[deprecate/env] 'HAKO_CAPI_PURE' is deprecated; use 'HAKO_BACKEND_COMPILE_RECIPE=pure-first'"
        );
    }
}

fn emit_compile_route_trace(compile_recipe: &str, compat_replay: &str) {
    if !llvm_route_trace_enabled() {
        return;
    }
    eprintln!(
        "[llvm-route/select] owner=boundary recipe={} compat_replay={} entry=hako_llvmc_compile_json_with_options_v1",
        compile_recipe, compat_replay
    );
}

fn environment_value(name: &str) -> Result<Option<String>> {
    match std::env::var(name) {
        Ok(value) => Ok(Some(value)),
        Err(std::env::VarError::NotPresent) => Ok(None),
        Err(std::env::VarError::NotUnicode(_)) => {
            bail!("[freeze:contract][compile-options/{name}] invalid UTF-8")
        }
    }
}

fn optional_environment_value(name: &str) -> Result<Option<String>> {
    Ok(environment_value(name)?.filter(|value| !value.is_empty()))
}

fn admit_boundary_route<'a>(
    recipe: Option<&'a str>,
    replay: Option<&'a str>,
) -> Result<(&'a str, &'a str)> {
    let recipe = recipe.unwrap_or("pure-first");
    if recipe != "pure-first" {
        bail!("[freeze:contract][compile-options/recipe] expected pure-first");
    }
    let replay = replay.unwrap_or("none");
    if replay != "none" {
        bail!("[freeze:contract][compile-options/replay] expected none");
    }
    Ok((recipe, replay))
}

fn environment_opt_level() -> Result<String> {
    let hako = environment_value("HAKO_LLVM_OPT_LEVEL")?;
    let nyash = environment_value("NYASH_LLVM_OPT_LEVEL")?;
    match (hako, nyash) {
        (Some(left), Some(right)) if left != right => {
            bail!("[freeze:contract][compile-options/opt-level-alias-conflict]")
        }
        (Some(level), _) | (_, Some(level)) => Ok(level),
        (None, None) => Ok("0".to_string()),
    }
}

fn validate_opt_level(level: &str) -> Result<()> {
    if level.len() == 1 && matches!(level.as_bytes()[0], b'0'..=b'3') {
        Ok(())
    } else {
        bail!("[freeze:contract][compile-options/opt-level] expected 0..3");
    }
}

fn cstring(value: &str, label: &str) -> Result<CString> {
    CString::new(value).with_context(|| format!("[freeze:contract][compile-options/{label}] NUL"))
}

fn optional_cstring(value: Option<String>, label: &str) -> Result<Option<CString>> {
    value.map(|value| cstring(&value, label)).transpose()
}

#[cfg(test)]
mod tests {
    use super::{admit_boundary_route, llvm_route_trace_enabled};

    #[test]
    fn boundary_route_defaults_to_pure_first_and_none() {
        assert_eq!(
            admit_boundary_route(None, None).unwrap(),
            ("pure-first", "none")
        );
    }

    #[test]
    fn boundary_route_rejects_compatibility_recipe_and_replay() {
        assert!(admit_boundary_route(Some("harness"), None).is_err());
        assert!(admit_boundary_route(None, Some("harness")).is_err());
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
