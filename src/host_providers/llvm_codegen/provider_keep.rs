use std::path::PathBuf;
use std::process::Command;

use super::normalize;
use super::transport;
use super::Opts;

pub(super) fn resolve_ny_llvmc() -> PathBuf {
    if let Some(s) = crate::config::env::ny_llvm_compiler_path() {
        return PathBuf::from(s);
    }
    if let Ok(p) = which::which("ny-llvmc") {
        return p;
    }
    PathBuf::from("target/release/ny-llvmc")
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

    let in_path = transport::prepare_backend_input_json_file(mir_json)?;
    let out_path = transport::resolve_backend_object_output(opts);
    transport::ensure_backend_output_parent(&out_path);

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

pub(super) fn mir_json_to_object_llvmlite(
    mir_json: &str,
    opts: &Opts,
) -> Result<PathBuf, String> {
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

    let in_path = transport::prepare_backend_input_json_file(mir_json)?;
    let out_path = transport::resolve_backend_object_output(opts);
    transport::ensure_backend_output_parent(&out_path);

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
