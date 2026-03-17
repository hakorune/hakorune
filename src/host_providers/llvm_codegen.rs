use serde_json::json;
use std::ffi::{CStr, CString};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

const COMPILE_SYMBOL_DEFAULT: &[u8] = b"hako_llvmc_compile_json\0";
const COMPILE_SYMBOL_PURE_FIRST: &[u8] = b"hako_llvmc_compile_json_pure_first\0";

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

pub fn boundary_default_object_opts(
    out: Option<PathBuf>,
    nyrt: Option<PathBuf>,
    opt_level: Option<String>,
    timeout_ms: Option<u64>,
) -> Opts {
    Opts {
        out,
        nyrt,
        opt_level,
        timeout_ms,
        compile_recipe: Some("pure-first".to_string()),
        compat_replay: Some("harness".to_string()),
    }
}

fn resolve_ny_llvmc() -> PathBuf {
    if let Some(s) = crate::config::env::ny_llvm_compiler_path() {
        return PathBuf::from(s);
    }
    if let Ok(p) = which::which("ny-llvmc") {
        return p;
    }
    PathBuf::from("target/release/ny-llvmc")
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

fn validate_backend_mir_shape(mir_json: &str) -> Result<(), String> {
    if !mir_json.contains("\"functions\"") || !mir_json.contains("\"blocks\"") {
        let tag = "[llvmemit/input/invalid] missing functions/blocks keys";
        llvm_emit_error!("{}", tag);
        return Err(tag.into());
    }
    Ok(())
}

fn build_backend_temp_input_path() -> PathBuf {
    std::env::temp_dir().join("hako_llvm_in.json")
}

fn prepare_backend_input_json_file(mir_json: &str) -> Result<PathBuf, String> {
    let in_path = build_backend_temp_input_path();
    let mut f =
        fs::File::create(&in_path).map_err(|e| format!("[llvmemit/tmp/write-failed] {}", e))?;
    f.write_all(mir_json.as_bytes())
        .map_err(|e| format!("[llvmemit/tmp/write-failed] {}", e))?;
    Ok(in_path)
}

fn resolve_backend_object_output(opts: &Opts) -> PathBuf {
    if let Some(p) = opts.out.clone() {
        p
    } else {
        std::env::temp_dir().join("hako_llvm_out.o")
    }
}

fn ensure_backend_output_parent(out_path: &Path) {
    if let Some(parent) = out_path.parent() {
        let _ = fs::create_dir_all(parent);
    }
}

fn try_compile_via_capi_keep(mir_json: &str, opts: &Opts) -> Result<Option<PathBuf>, String> {
    if !(crate::config::env::llvm_use_capi() && crate::config::env::extern_provider_c_abi()) {
        return Ok(None);
    }
    validate_backend_mir_shape(mir_json)?;
    let in_path = prepare_backend_input_json_file(mir_json)?;
    let out_path = resolve_backend_object_output(opts);
    ensure_backend_output_parent(&out_path);
    match compile_via_capi(&in_path, &out_path, opts) {
        Ok(()) => Ok(Some(out_path)),
        Err(e) => {
            llvm_emit_error!("[llvmemit/capi/failed] {}", e);
            Err(format!("[llvmemit/capi/failed] {}", e))
        }
    }
}

fn try_compile_via_explicit_provider_keep(
    mir_json: &str,
    opts: &Opts,
) -> Result<Option<PathBuf>, String> {
    match crate::config::env::llvm_emit_provider().as_deref() {
        Some("llvmlite") => mir_json_to_object_llvmlite(mir_json, opts).map(Some),
        Some("ny-llvmc") => mir_json_to_object_ny_llvmc(mir_json, opts).map(Some),
        _ => Ok(None),
    }
}

fn capi_boundary_unavailable(error: &str) -> bool {
    error.contains("FFI library not found")
        || error.contains("capi not available")
        || error.contains("dlopen failed")
        || error.contains("dlsym failed")
}

fn try_compile_via_boundary_default(
    mir_json: &str,
    opts: &Opts,
) -> Result<Option<PathBuf>, String> {
    validate_backend_mir_shape(mir_json)?;
    let in_path = prepare_backend_input_json_file(mir_json)?;
    let out_path = resolve_backend_object_output(opts);
    ensure_backend_output_parent(&out_path);
    let boundary_opts = with_boundary_default_route(opts);
    match compile_via_capi(&in_path, &out_path, &boundary_opts) {
        Ok(()) => Ok(Some(out_path)),
        Err(error) if capi_boundary_unavailable(&error) => Ok(None),
        Err(error) => {
            llvm_emit_error!("[llvmemit/capi/default-failed] {}", error);
            Err(format!("[llvmemit/capi/default-failed] {}", error))
        }
    }
}

fn boundary_default_unavailable_tag() -> String {
    "[llvmemit/capi/default-unavailable] build libhako_llvmc_ffi.so or set HAKO_LLVM_EMIT_PROVIDER=llvmlite".into()
}

pub fn normalize_mir_json_for_backend(mir_json: &str) -> Result<String, String> {
    let mut value: serde_json::Value = serde_json::from_str(mir_json)
        .map_err(|e| format!("[llvmemit/input/invalid-json] {}", e))?;
    let root = value
        .as_object_mut()
        .ok_or_else(|| "[llvmemit/input/invalid-root] expected object".to_string())?;
    root.insert(
        "kind".to_string(),
        serde_json::Value::String("MIR".to_string()),
    );
    root.insert(
        "schema_version".to_string(),
        serde_json::Value::String("1.0".to_string()),
    );
    match root.get_mut("metadata") {
        Some(serde_json::Value::Object(metadata)) => {
            metadata
                .entry("extern_c".to_string())
                .or_insert_with(|| json!([]));
        }
        _ => {
            root.insert("metadata".to_string(), json!({ "extern_c": [] }));
        }
    }
    serde_json::to_string(&value).map_err(|e| format!("[llvmemit/input/serialize-failed] {}", e))
}

pub fn mir_json_file_to_object(input_json_path: &Path, opts: Opts) -> Result<PathBuf, String> {
    let mir_json = fs::read_to_string(input_json_path)
        .map_err(|e| format!("[llvmemit/input/read-failed] {}", e))?;
    mir_json_to_object(&mir_json, opts)
}

/// Compile MIR(JSON v0) to an object file (.o) using ny-llvmc. Returns the output path.
/// Fail‑Fast: prints stable tags and returns Err with the same message.
pub fn mir_json_to_object(mir_json: &str, opts: Opts) -> Result<PathBuf, String> {
    let mir_json = normalize_mir_json_for_backend(mir_json)?;
    if let Some(out_path) = try_compile_via_capi_keep(&mir_json, &opts)? {
        return Ok(out_path);
    }
    if let Some(out_path) = try_compile_via_explicit_provider_keep(&mir_json, &opts)? {
        return Ok(out_path);
    }
    if let Some(out_path) = try_compile_via_boundary_default(&mir_json, &opts)? {
        return Ok(out_path);
    }
    let tag = boundary_default_unavailable_tag();
    llvm_emit_error!("{}", tag);
    Err(tag)
}

/// Compile via ny-llvmc wrapper (explicit compat keep). Returns output path or tagged error.
fn mir_json_to_object_ny_llvmc(mir_json: &str, opts: &Opts) -> Result<PathBuf, String> {
    validate_backend_mir_shape(mir_json)?;
    let ny_llvmc = resolve_ny_llvmc();
    if !ny_llvmc.exists() {
        let tag = format!("[llvmemit/ny-llvmc/not-found] path={}", ny_llvmc.display());
        llvm_emit_error!("{}", tag);
        return Err(tag);
    }

    let in_path = prepare_backend_input_json_file(&mir_json)?;
    let out_path = resolve_backend_object_output(&opts);
    ensure_backend_output_parent(&out_path);

    // Build command: ny-llvmc --in <json> --emit obj --out <out>
    let mut cmd = Command::new(&ny_llvmc);
    cmd.arg("--in")
        .arg(&in_path)
        .arg("--emit")
        .arg("obj")
        .arg("--out")
        .arg(&out_path);
    if let Some(nyrt) = opts.nyrt.as_ref() {
        cmd.arg("--nyrt").arg(nyrt);
    }
    if let Some(level) = opts.opt_level.as_ref() {
        cmd.env("HAKO_LLVM_OPT_LEVEL", level);
        cmd.env("NYASH_LLVM_OPT_LEVEL", level);
    }

    let status = cmd
        .status()
        .map_err(|e| format!("[llvmemit/spawn/error] {}", e))?;
    if !status.success() {
        let code = status.code().unwrap_or(1);
        let tag = format!("[llvmemit/ny-llvmc/failed status={}]", code);
        llvm_emit_error!("{}", tag);
        return Err(tag);
    }
    if !out_path.exists() {
        let tag = format!("[llvmemit/output/missing] {}", out_path.display());
        llvm_emit_error!("{}", tag);
        return Err(tag);
    }
    Ok(out_path)
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
        _ => COMPILE_SYMBOL_DEFAULT,
    }
}

#[cfg(feature = "plugins")]
fn compile_via_capi(json_in: &Path, obj_out: &Path, opts: &Opts) -> Result<(), String> {
    use libloading::Library;
    use std::os::raw::{c_char, c_int, c_void};

    // Declare libc free for error string cleanup
    extern "C" {
        fn free(ptr: *mut c_void);
    }

    unsafe {
        // Resolve library path
        let mut candidates: Vec<PathBuf> = Vec::new();
        if let Some(p) = crate::config::env::aot_ffi_lib_path() {
            candidates.push(PathBuf::from(p));
        }
        candidates.extend(ffi_library_default_candidates());
        let lib_path = candidates
            .into_iter()
            .find(|p| p.exists())
            .ok_or_else(|| "FFI library not found (set HAKO_AOT_FFI_LIB)".to_string())?;
        let lib = Library::new(lib_path).map_err(|e| format!("dlopen failed: {}", e))?;
        // Recipe-aware daily callers prefer the explicit pure-first export so
        // route selection stays outside the generic C shim surface.
        type CompileFn =
            unsafe extern "C" fn(*const c_char, *const c_char, *mut *mut c_char) -> c_int;
        let compile_recipe = requested_compile_recipe(opts);
        let compat_replay = requested_compat_replay(opts);
        let func: libloading::Symbol<CompileFn> = lib
            .get(compile_symbol_for_recipe(compile_recipe.as_deref()))
            .or_else(|_| lib.get(COMPILE_SYMBOL_DEFAULT))
            .map_err(|e| format!("dlsym failed: {}", e))?;
        let cin = CString::new(json_in.to_string_lossy().as_bytes())
            .map_err(|_| "invalid json path".to_string())?;
        let cout = CString::new(obj_out.to_string_lossy().as_bytes())
            .map_err(|_| "invalid out path".to_string())?;
        let mut err_ptr: *mut c_char = std::ptr::null_mut();
        let prev_recipe = std::env::var("HAKO_BACKEND_COMPILE_RECIPE").ok();
        let prev_replay = std::env::var("HAKO_BACKEND_COMPAT_REPLAY").ok();
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

        // Inject opt_level defaults for Python harness (insurance against None)
        if crate::config::env::llvm_opt_level_envs().0.is_none() {
            std::env::set_var("HAKO_LLVM_OPT_LEVEL", "0");
        }
        if crate::config::env::llvm_opt_level_envs().1.is_none() {
            std::env::set_var("NYASH_LLVM_OPT_LEVEL", "0");
        }

        // Optional trace for debugging (HAKO_CABI_TRACE=1)
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
        if rc != 0 {
            let msg = if !err_ptr.is_null() {
                CStr::from_ptr(err_ptr).to_string_lossy().to_string()
            } else {
                "compile failed".to_string()
            };
            // Free error string (allocated by C side)
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
fn compile_via_capi(_json_in: &Path, _obj_out: &Path, _opts: &Opts) -> Result<(), String> {
    Err("capi not available (plugins feature disabled)".into())
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
    link_via_capi(obj_in, exe_out, extra_ldflags)
}

#[cfg(feature = "plugins")]
fn link_via_capi(obj_in: &Path, exe_out: &Path, extra_ldflags: Option<&str>) -> Result<(), String> {
    use libloading::Library;
    use std::os::raw::{c_char, c_int, c_void};

    // Declare libc free for error string cleanup
    extern "C" {
        fn free(ptr: *mut c_void);
    }

    unsafe {
        let mut candidates: Vec<PathBuf> = Vec::new();
        if let Some(p) = crate::config::env::aot_ffi_lib_path() {
            candidates.push(PathBuf::from(p));
        }
        candidates.extend(ffi_library_default_candidates());
        let lib_path = candidates
            .into_iter()
            .find(|p| p.exists())
            .ok_or_else(|| "FFI library not found (set HAKO_AOT_FFI_LIB)".to_string())?;
        let lib = Library::new(lib_path).map_err(|e| format!("dlopen failed: {}", e))?;
        // int hako_llvmc_link_obj(const char*, const char*, const char*, char**)
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
fn link_via_capi(_obj_in: &Path, _exe_out: &Path, _extra: Option<&str>) -> Result<(), String> {
    Err("capi not available (plugins feature disabled)".into())
}

fn resolve_python3() -> Option<PathBuf> {
    if let Ok(p) = which::which("python3") {
        return Some(p);
    }
    if let Ok(p) = which::which("python") {
        return Some(p);
    }
    None
}

fn resolve_llvmlite_harness() -> Option<PathBuf> {
    if let Some(root) = crate::config::env::nyash_root() {
        let p = PathBuf::from(root).join("tools/llvmlite_harness.py");
        if p.exists() {
            return Some(p);
        }
    }
    let p = PathBuf::from("tools/llvmlite_harness.py");
    if p.exists() {
        return Some(p);
    }
    // Also try repo-relative (target may run elsewhere)
    let p2 = PathBuf::from("../tools/llvmlite_harness.py");
    if p2.exists() {
        return Some(p2);
    }
    None
}

/// Compile via llvmlite harness (opt-in provider). Returns output path or tagged error.
fn mir_json_to_object_llvmlite(mir_json: &str, opts: &Opts) -> Result<PathBuf, String> {
    validate_backend_mir_shape(mir_json)?;
    let py = resolve_python3().ok_or_else(|| {
        let tag = String::from("[llvmemit/llvmlite/python-not-found]");
        llvm_emit_error!("{}", tag);
        tag
    })?;
    let harness = resolve_llvmlite_harness().ok_or_else(|| {
        let tag = String::from("[llvmemit/llvmlite/harness-not-found] tools/llvmlite_harness.py");
        llvm_emit_error!("{}", tag);
        tag
    })?;

    let in_path = prepare_backend_input_json_file(mir_json)?;
    let out_path = resolve_backend_object_output(opts);
    ensure_backend_output_parent(&out_path);

    // Run: python3 tools/llvmlite_harness.py --in <json> --out <out>
    let status = Command::new(&py)
        .arg(&harness)
        .arg("--in")
        .arg(&in_path)
        .arg("--out")
        .arg(&out_path)
        .status()
        .map_err(|e| format!("[llvmemit/llvmlite/spawn/error] {}", e))?;
    if !status.success() {
        let code = status.code().unwrap_or(1);
        let tag = format!("[llvmemit/llvmlite/failed status={}]", code);
        llvm_emit_error!("{}", tag);
        return Err(tag);
    }
    if !out_path.exists() {
        let tag = format!("[llvmemit/output/missing] {}", out_path.display());
        llvm_emit_error!("{}", tag);
        return Err(tag);
    }
    Ok(out_path)
}
