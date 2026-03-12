use std::ffi::{CStr, CString};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

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

/// Compile MIR(JSON v0) to an object file (.o) using ny-llvmc. Returns the output path.
/// Fail‑Fast: prints stable tags and returns Err with the same message.
pub fn mir_json_to_object(mir_json: &str, opts: Opts) -> Result<PathBuf, String> {
    // Optional provider selection (C-API) — guarded by env flags
    // NYASH_LLVM_USE_CAPI=1 and HAKO_V1_EXTERN_PROVIDER_C_ABI=1
    if crate::config::env::llvm_use_capi() && crate::config::env::extern_provider_c_abi() {
        // Basic shape check first
        if !mir_json.contains("\"functions\"") || !mir_json.contains("\"blocks\"") {
            let tag = "[llvmemit/input/invalid] missing functions/blocks keys";
            llvm_emit_error!("{}", tag);
            return Err(tag.into());
        }
        // Write input to a temp file
        let tmp_dir = std::env::temp_dir();
        let in_path = tmp_dir.join("hako_llvm_in.json");
        {
            let mut f = fs::File::create(&in_path)
                .map_err(|e| format!("[llvmemit/tmp/write-failed] {}", e))?;
            f.write_all(mir_json.as_bytes())
                .map_err(|e| format!("[llvmemit/tmp/write-failed] {}", e))?;
        }
        let out_path = if let Some(p) = opts.out.clone() {
            p
        } else {
            tmp_dir.join("hako_llvm_out.o")
        };
        if let Some(parent) = out_path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        match compile_via_capi(&in_path, &out_path) {
            Ok(()) => return Ok(out_path),
            Err(e) => {
                llvm_emit_error!("[llvmemit/capi/failed] {}", e);
                // Fall through to other providers only when explicitly allowed; by default fail-fast
                return Err(format!("[llvmemit/capi/failed] {}", e));
            }
        }
    }
    // Optional provider selection (default: ny-llvmc)
    match crate::config::env::llvm_emit_provider().as_deref() {
        Some("llvmlite") => return mir_json_to_object_llvmlite(mir_json, &opts),
        _ => {}
    }
    // Basic shape check for MIR(JSON v0)
    if !mir_json.contains("\"functions\"") || !mir_json.contains("\"blocks\"") {
        let tag = "[llvmemit/input/invalid] missing functions/blocks keys";
        llvm_emit_error!("{}", tag);
        return Err(tag.into());
    }

    let ny_llvmc = resolve_ny_llvmc();
    if !ny_llvmc.exists() {
        let tag = format!("[llvmemit/ny-llvmc/not-found] path={}", ny_llvmc.display());
        llvm_emit_error!("{}", tag);
        return Err(tag);
    }

    // Write MIR JSON to temp
    let tmp_dir = std::env::temp_dir();
    let in_path = tmp_dir.join("hako_llvm_in.json");
    {
        let mut f =
            fs::File::create(&in_path).map_err(|e| format!("[llvmemit/tmp/write-failed] {}", e))?;
        f.write_all(mir_json.as_bytes())
            .map_err(|e| format!("[llvmemit/tmp/write-failed] {}", e))?;
    }

    // Output path
    let out_path = if let Some(p) = opts.out.clone() {
        p
    } else {
        tmp_dir.join("hako_llvm_out.o")
    };
    if let Some(parent) = out_path.parent() {
        let _ = fs::create_dir_all(parent);
    }

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

#[cfg(feature = "plugins")]
fn compile_via_capi(json_in: &Path, obj_out: &Path) -> Result<(), String> {
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
        // Symbol: int hako_llvmc_compile_json(const char*, const char*, char**)
        type CompileFn =
            unsafe extern "C" fn(*const c_char, *const c_char, *mut *mut c_char) -> c_int;
        let func: libloading::Symbol<CompileFn> = lib
            .get(b"hako_llvmc_compile_json\0")
            .map_err(|e| format!("dlsym failed: {}", e))?;
        let cin = CString::new(json_in.to_string_lossy().as_bytes())
            .map_err(|_| "invalid json path".to_string())?;
        let cout = CString::new(obj_out.to_string_lossy().as_bytes())
            .map_err(|_| "invalid out path".to_string())?;
        let mut err_ptr: *mut c_char = std::ptr::null_mut();
        // Avoid recursive FFI-in-FFI: force inner AOT to use CLI path
        let prev = crate::config::env::aot_use_ffi_env();
        std::env::set_var("HAKO_AOT_USE_FFI", "0");

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
        if let Some(v) = prev {
            std::env::set_var("HAKO_AOT_USE_FFI", v);
        } else {
            std::env::remove_var("HAKO_AOT_USE_FFI");
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
fn compile_via_capi(_json_in: &Path, _obj_out: &Path) -> Result<(), String> {
    Err("capi not available (plugins feature disabled)".into())
}

/// Link an object to an executable via C-API FFI bundle.
pub fn link_object_capi(
    obj_in: &Path,
    exe_out: &Path,
    extra_ldflags: Option<&str>,
) -> Result<(), String> {
    // Compute effective ldflags
    let mut eff: Option<String> = extra_ldflags.map(|s| s.to_string());
    let empty = eff.as_deref().map(|s| s.trim().is_empty()).unwrap_or(true);
    if empty {
        if let Some(s) = crate::config::env::aot_ldflags() {
            eff = Some(s);
        }
    }
    if eff.is_none() {
        // Try to auto-detect NyRT static lib; append common libs
        let candidates = [
            // New kernel name
            "target/release/libnyash_kernel.a",
            "crates/nyash_kernel/target/release/libnyash_kernel.a",
            "dist/lib/libnyash_kernel.a",
            // Legacy names (fallback)
            "target/release/libnyrt.a",
            "crates/nyrt/target/release/libnyrt.a",
            "dist/lib/libnyrt.a",
        ];
        for c in candidates.iter() {
            let p = PathBuf::from(c);
            if p.exists() {
                eff = Some(format!("{} -ldl -lpthread -lm", p.to_string_lossy()));
                break;
            }
        }
    }
    if crate::config::env::cabi_trace() {
        llvm_emit_debug!("[hb:link:ldflags] {}", eff.as_deref().unwrap_or("<none>"));
    }
    link_via_capi(obj_in, exe_out, eff.as_deref())
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
        // Avoid recursive FFI-in-FFI
        let prev = crate::config::env::aot_use_ffi_env();
        std::env::set_var("HAKO_AOT_USE_FFI", "0");
        let rc = func(
            cobj.as_ptr(),
            cexe.as_ptr(),
            cflags_ptr,
            &mut err_ptr as *mut *mut c_char,
        );
        if let Some(v) = prev {
            std::env::set_var("HAKO_AOT_USE_FFI", v);
        } else {
            std::env::remove_var("HAKO_AOT_USE_FFI");
        }
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
    if !mir_json.contains("\"functions\"") || !mir_json.contains("\"blocks\"") {
        let tag = "[llvmemit/input/invalid] missing functions/blocks keys";
        llvm_emit_error!("{}", tag);
        return Err(tag.into());
    }
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

    // Write MIR JSON to temp
    let tmp_dir = std::env::temp_dir();
    let in_path = tmp_dir.join("hako_llvm_in.json");
    {
        let mut f =
            fs::File::create(&in_path).map_err(|e| format!("[llvmemit/tmp/write-failed] {}", e))?;
        f.write_all(mir_json.as_bytes())
            .map_err(|e| format!("[llvmemit/tmp/write-failed] {}", e))?;
    }
    let out_path = if let Some(p) = opts.out.clone() {
        p
    } else {
        tmp_dir.join("hako_llvm_out.o")
    };
    if let Some(parent) = out_path.parent() {
        let _ = fs::create_dir_all(parent);
    }

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
