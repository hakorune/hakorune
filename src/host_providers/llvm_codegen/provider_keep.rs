use std::path::PathBuf;
#[cfg(feature = "llvmlite-compat")]
use std::process::Command;

#[cfg(feature = "llvmlite-compat")]
use super::normalize;
#[cfg(feature = "llvmlite-compat")]
use super::transport_io;
#[cfg(feature = "llvmlite-compat")]
use super::transport_paths;
use super::Opts;

#[cfg(feature = "llvmlite-compat")]
fn prepare_provider_io(
    mir_json: &str,
    opts: &Opts,
) -> Result<(transport_io::BackendInputJsonFile, PathBuf), String> {
    normalize::validate_backend_mir_shape(mir_json)?;
    let in_path = transport_io::prepare_backend_input_json_file(mir_json)?;
    let out_path = transport_paths::resolve_backend_object_output(opts);
    transport_io::ensure_backend_output_parent(&out_path);
    Ok((in_path, out_path))
}

#[cfg(feature = "llvmlite-compat")]
fn resolve_python3() -> Option<PathBuf> {
    if let Ok(p) = which::which("python3") {
        return Some(p);
    }
    if let Ok(p) = which::which("python") {
        return Some(p);
    }
    None
}

#[cfg(feature = "llvmlite-compat")]
fn resolve_llvmlite_harness() -> Option<PathBuf> {
    if let Some(root) = crate::config::env::hako_root() {
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

#[cfg(feature = "llvmlite-compat")]
pub(super) fn mir_json_to_object_llvmlite(mir_json: &str, opts: &Opts) -> Result<PathBuf, String> {
    let (input, out_path) = prepare_provider_io(mir_json, opts)?;
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
        .arg(input.path())
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
    transport_io::ensure_backend_artifact_written(&out_path, "object")?;
    Ok(out_path)
}

#[cfg(not(feature = "llvmlite-compat"))]
pub(super) fn mir_json_to_object_llvmlite(
    _mir_json: &str,
    _opts: &Opts,
) -> Result<PathBuf, String> {
    Err("[llvmemit/llvmlite/compat-disabled] build with --features llvmlite-compat for the explicit compatibility lane".to_string())
}
