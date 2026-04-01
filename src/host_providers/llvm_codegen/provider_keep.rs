use std::path::{Path, PathBuf};
use std::process::Command;

use super::normalize;
use super::route::{resolve_llvmlite_harness, resolve_ny_llvmc, resolve_python3};
use super::transport_io;
use super::transport_paths;
use super::Opts;

fn prepare_provider_io(mir_json: &str, opts: &Opts) -> Result<(PathBuf, PathBuf), String> {
    normalize::validate_backend_mir_shape(mir_json)?;
    let in_path = transport_io::prepare_backend_input_json_file(mir_json)?;
    let out_path = transport_paths::resolve_backend_object_output(opts);
    transport_io::ensure_backend_output_parent(&out_path);
    Ok((in_path, out_path))
}

fn ensure_object_output_exists(out_path: &Path) -> Result<PathBuf, String> {
    if !out_path.exists() {
        let tag = format!("[llvmemit/output/missing] {}", out_path.display());
        llvm_emit_error!("{}", tag);
        return Err(tag);
    }
    Ok(out_path.to_path_buf())
}

pub(super) fn mir_json_to_object_ny_llvmc(mir_json: &str, opts: &Opts) -> Result<PathBuf, String> {
    let (in_path, out_path) = prepare_provider_io(mir_json, opts)?;
    let ny_llvmc = resolve_ny_llvmc();
    if !ny_llvmc.exists() {
        let tag = format!("[llvmemit/ny-llvmc/not-found] path={}", ny_llvmc.display());
        llvm_emit_error!("{}", tag);
        return Err(tag);
    }

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
    ensure_object_output_exists(&out_path)
}

pub(super) fn mir_json_to_object_llvmlite(mir_json: &str, opts: &Opts) -> Result<PathBuf, String> {
    let (in_path, out_path) = prepare_provider_io(mir_json, opts)?;
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
    ensure_object_output_exists(&out_path)
}
