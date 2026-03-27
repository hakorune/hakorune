use std::ffi::{CStr, CString};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

use serde_json::Value;

use super::{defaults, normalize, Opts};

pub(super) fn resolve_ny_llvmc() -> PathBuf {
    if let Some(s) = crate::config::env::ny_llvm_compiler_path() {
        return PathBuf::from(s);
    }
    if let Ok(p) = which::which("ny-llvmc") {
        return p;
    }
    PathBuf::from("target/release/ny-llvmc")
}

fn resolve_hakorune_bin() -> PathBuf {
    if let Ok(bin) = std::env::var("NYASH_BIN") {
        let trimmed = bin.trim();
        if !trimmed.is_empty() {
            return PathBuf::from(trimmed);
        }
    }
    if let Ok(cur) = std::env::current_exe() {
        return cur;
    }
    let hakorune = PathBuf::from("target/release/hakorune");
    if hakorune.exists() {
        return hakorune;
    }
    PathBuf::from("target/release/nyash")
}

fn resolve_llc_tool() -> Option<&'static str> {
    ["llc", "llc-18"].into_iter().find(|candidate| {
        Command::new(candidate)
            .arg("--version")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    })
}

fn resolve_opt_tool() -> Option<&'static str> {
    ["opt", "opt-18"].into_iter().find(|candidate| {
        Command::new(candidate)
            .arg("--version")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    })
}

fn compile_ll_to_object(ll_path: &Path, out_path: &Path) -> Result<(), String> {
    let llc = resolve_llc_tool().ok_or_else(|| "[llvmemit/hako-ll/llc-missing] llc not found".to_string())?;
    let output = Command::new(llc)
        .arg("-filetype=obj")
        .arg("-relocation-model=pic")
        .arg("-o")
        .arg(out_path)
        .arg(ll_path)
        .output()
        .map_err(|e| format!("[llvmemit/hako-ll/llc-spawn-failed] {}", e))?;
    if !output.status.success() {
        return Err(format!(
            "[llvmemit/hako-ll/llc-failed] stdout=`{}` stderr=`{}`",
            String::from_utf8_lossy(&output.stdout).trim(),
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    Ok(())
}

fn verify_ll_file(ll_path: &Path) -> Result<(), String> {
    let Some(opt) = resolve_opt_tool() else {
        return Ok(());
    };
    let output = Command::new(opt)
        .arg("-passes=verify")
        .arg("-disable-output")
        .arg(ll_path)
        .output()
        .map_err(|e| format!("[llvmemit/hako-ll/verify-spawn-failed] {}", e))?;
    if !output.status.success() {
        return Err(format!(
            "[llvmemit/hako-ll/verify-failed] stdout=`{}` stderr=`{}`",
            String::from_utf8_lossy(&output.stdout).trim(),
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    Ok(())
}

pub(super) fn build_backend_temp_input_path() -> PathBuf {
    std::env::temp_dir().join("hako_llvm_in.json")
}

pub(super) fn prepare_backend_input_json_file(mir_json: &str) -> Result<PathBuf, String> {
    let in_path = build_backend_temp_input_path();
    let mut f =
        fs::File::create(&in_path).map_err(|e| format!("[llvmemit/tmp/write-failed] {}", e))?;
    f.write_all(mir_json.as_bytes())
        .map_err(|e| format!("[llvmemit/tmp/write-failed] {}", e))?;
    Ok(in_path)
}

pub(super) fn resolve_backend_object_output(opts: &Opts) -> PathBuf {
    if let Some(p) = opts.out.clone() {
        p
    } else {
        std::env::temp_dir().join("hako_llvm_out.o")
    }
}

fn temporary_ll_output_path(out_path: &Path) -> PathBuf {
    let filename = out_path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("hako_ll_compare");
    std::env::temp_dir().join(format!("{}.{}.compare.ll", filename, std::process::id()))
}

fn temporary_hako_compare_source_path(out_path: &Path) -> PathBuf {
    let filename = out_path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("hako_ll_compare");
    std::env::temp_dir().join(format!("{}.{}.compare_driver.hako", filename, std::process::id()))
}

fn escape_hako_string_literal(text: &str) -> String {
    text.replace('\\', "\\\\").replace('"', "\\\"")
}

fn render_hako_embedded_value(
    value: &Value,
    counter: &mut usize,
    body: &mut String,
    indent: &str,
) -> Result<String, String> {
    match value {
        Value::Null => Ok("null".to_string()),
        Value::Bool(v) => Ok(if *v { "1" } else { "0" }.to_string()),
        Value::Number(num) => {
            if let Some(v) = num.as_i64() {
                Ok(v.to_string())
            } else if let Some(v) = num.as_u64() {
                if v <= i64::MAX as u64 {
                    Ok(v.to_string())
                } else {
                    Err(format!(
                        "[llvmemit/hako-ll/unsupported-number] unsigned out of i64 range: {}",
                        v
                    ))
                }
            } else {
                Err(format!(
                    "[llvmemit/hako-ll/unsupported-number] non-integer literal: {}",
                    num
                ))
            }
        }
        Value::String(text) => Ok(format!("\"{}\"", escape_hako_string_literal(text))),
        Value::Array(items) => {
            let var = format!("v{}", *counter);
            *counter += 1;
            body.push_str(&format!("{indent}local {var} = new ArrayBox()\n"));
            for item in items {
                let expr = render_hako_embedded_value(item, counter, body, indent)?;
                body.push_str(&format!("{indent}{var}.push({expr})\n"));
            }
            Ok(var)
        }
        Value::Object(map) => {
            let var = format!("v{}", *counter);
            *counter += 1;
            body.push_str(&format!("{indent}local {var} = new MapBox()\n"));
            for (key, item) in map {
                let expr = render_hako_embedded_value(item, counter, body, indent)?;
                body.push_str(&format!(
                    "{indent}{var}.set(\"{}\", {expr})\n",
                    escape_hako_string_literal(key)
                ));
            }
            Ok(var)
        }
    }
}

fn render_hako_compare_root_builder(mir_json: &str) -> Result<String, String> {
    let value: Value = serde_json::from_str(mir_json)
        .map_err(|e| format!("[llvmemit/hako-ll/json-parse-failed] {}", e))?;
    let mut counter = 0usize;
    let mut body = String::new();
    let root = render_hako_embedded_value(&value, &mut counter, &mut body, "    ")?;
    body.push_str(&format!("    return {root}\n"));
    Ok(body)
}

fn prepare_hako_compare_driver_source(mir_json: &str, out_path: &Path) -> Result<PathBuf, String> {
    let template = PathBuf::from("lang/src/shared/backend/ll_emit/driver.hako");
    let source = fs::read_to_string(&template).map_err(|e| {
        format!(
            "[llvmemit/hako-ll/template-read-failed] path={} error={}",
            template.display(),
            e
        )
    })?;
    let root_builder = render_hako_compare_root_builder(mir_json)?;
    let rendered = source
        .replace(
            "    // __HAKO_LL_COMPARE_ROOT_BUILDER__\n    return null\n",
            &root_builder,
        );
    let out = temporary_hako_compare_source_path(out_path);
    fs::write(&out, rendered).map_err(|e| {
        format!(
            "[llvmemit/hako-ll/template-write-failed] path={} error={}",
            out.display(),
            e
        )
    })?;
    Ok(out)
}

fn run_compare_driver_via_vm(hakorune: &Path, source_path: &Path) -> Result<String, String> {
    let output = Command::new(hakorune)
        .arg("--backend")
        .arg("vm")
        .arg(source_path)
        .output()
        .map_err(|e| format!("[llvmemit/hako-ll/vm-spawn-failed] {}", e))?;
    if !output.status.success() {
        return Err(format!(
            "[llvmemit/hako-ll/vm-failed] stdout=`{}` stderr=`{}`",
            String::from_utf8_lossy(&output.stdout).trim(),
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn extract_hako_compare_ll(stdout: &str) -> Result<String, String> {
    let begin = "[hako-ll/ll-begin]\n";
    let end = "\n[hako-ll/ll-end]";
    let start = stdout
        .find(begin)
        .ok_or_else(|| format!("[llvmemit/hako-ll/ll-begin-missing] stdout=`{}`", stdout.trim()))?;
    let content_start = start + begin.len();
    let tail = &stdout[content_start..];
    let end_offset = tail
        .find(end)
        .ok_or_else(|| format!("[llvmemit/hako-ll/ll-end-missing] stdout=`{}`", stdout.trim()))?;
    Ok(tail[..end_offset].to_string())
}

fn extract_hako_compare_contract_line(stdout: &str) -> Result<String, String> {
    stdout
        .lines()
        .find(|line| line.starts_with("[hako-ll/compare] "))
        .map(|line| line.to_string())
        .ok_or_else(|| {
            format!(
                "[llvmemit/hako-ll/compare-line-missing] stdout=`{}`",
                stdout.trim()
            )
        })
}

pub(super) fn ensure_backend_output_parent(out_path: &Path) {
    if let Some(parent) = out_path.parent() {
        let _ = fs::create_dir_all(parent);
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
    use libloading::Library;
    use std::os::raw::{c_char, c_int, c_void};

    extern "C" {
        fn free(ptr: *mut c_void);
    }

    unsafe {
        let mut candidates: Vec<PathBuf> = Vec::new();
        if let Some(p) = crate::config::env::aot_ffi_lib_path() {
            candidates.push(PathBuf::from(p));
        }
        candidates.extend(defaults::ffi_library_default_candidates());
        let lib_path = candidates
            .into_iter()
            .find(|p| p.exists())
            .ok_or_else(|| "FFI library not found (set HAKO_AOT_FFI_LIB)".to_string())?;
        let lib = Library::new(lib_path).map_err(|e| format!("dlopen failed: {}", e))?;
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

pub(super) fn resolve_python3() -> Option<PathBuf> {
    if let Ok(p) = which::which("python3") {
        return Some(p);
    }
    if let Ok(p) = which::which("python") {
        return Some(p);
    }
    None
}

pub(super) fn resolve_llvmlite_harness() -> Option<PathBuf> {
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
    let p2 = PathBuf::from("../tools/llvmlite_harness.py");
    if p2.exists() {
        return Some(p2);
    }
    None
}

pub(super) fn mir_json_to_object_ny_llvmc(mir_json: &str, opts: &Opts) -> Result<PathBuf, String> {
    normalize::validate_backend_mir_shape(mir_json)?;
    let ny_llvmc = resolve_ny_llvmc();
    if !ny_llvmc.exists() {
        let tag = format!("[llvmemit/ny-llvmc/not-found] path={}", ny_llvmc.display());
        llvm_emit_error!("{}", tag);
        return Err(tag);
    }

    let in_path = prepare_backend_input_json_file(&mir_json)?;
    let out_path = resolve_backend_object_output(opts);
    ensure_backend_output_parent(&out_path);

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

pub(super) fn mir_json_to_object_hako_ll_compare(
    mir_json: &str,
    opts: &Opts,
) -> Result<PathBuf, String> {
    normalize::validate_backend_mir_shape(mir_json)?;
    let hakorune = resolve_hakorune_bin();
    if !hakorune.exists() {
        return Err(format!(
            "[llvmemit/hako-ll/not-found] path={}",
            hakorune.display()
        ));
    }

    let out_path = resolve_backend_object_output(opts);
    ensure_backend_output_parent(&out_path);
    let ll_path = temporary_ll_output_path(&out_path);
    let source_path = prepare_hako_compare_driver_source(mir_json, &out_path)?;
    let stdout = run_compare_driver_via_vm(&hakorune, &source_path)?;
    let compare_line = extract_hako_compare_contract_line(&stdout)?;
    let ll_text = extract_hako_compare_ll(&stdout)?;
    fs::write(&ll_path, ll_text).map_err(|e| {
        format!(
            "[llvmemit/hako-ll/output-write-failed] path={} error={}",
            ll_path.display(),
            e
        )
    })?;
    println!("{}", compare_line);
    verify_ll_file(&ll_path)?;
    compile_ll_to_object(&ll_path, &out_path)?;
    let _ = fs::remove_file(&source_path);
    let _ = fs::remove_file(&ll_path);
    Ok(out_path)
}

#[cfg(feature = "plugins")]
pub(super) fn link_via_capi(
    obj_in: &Path,
    exe_out: &Path,
    extra_ldflags: Option<&str>,
) -> Result<(), String> {
    use libloading::Library;
    use std::os::raw::{c_char, c_int, c_void};

    extern "C" {
        fn free(ptr: *mut c_void);
    }

    unsafe {
        let mut candidates: Vec<PathBuf> = Vec::new();
        if let Some(p) = crate::config::env::aot_ffi_lib_path() {
            candidates.push(PathBuf::from(p));
        }
        candidates.extend(defaults::ffi_library_default_candidates());
        let lib_path = candidates
            .into_iter()
            .find(|p| p.exists())
            .ok_or_else(|| "FFI library not found (set HAKO_AOT_FFI_LIB)".to_string())?;
        let lib = Library::new(lib_path).map_err(|e| format!("dlopen failed: {}", e))?;
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

pub(super) fn mir_json_to_object_llvmlite(mir_json: &str, opts: &Opts) -> Result<PathBuf, String> {
    normalize::validate_backend_mir_shape(mir_json)?;
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
