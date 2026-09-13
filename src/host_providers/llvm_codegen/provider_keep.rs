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

#[cfg(all(test, feature = "llvmlite-compat"))]
mod tests {
    use super::{mir_json_to_object_llvmlite, resolve_python3};
    use crate::host_providers::llvm_codegen::boundary_default_object_opts;
    use std::fs;
    use std::path::{Path, PathBuf};

    const MIR_JSON: &str = r#"{"functions":[],"blocks":[]}"#;

    fn provider_opts(out: &Path) -> crate::host_providers::llvm_codegen::Opts {
        boundary_default_object_opts(Some(out.to_path_buf()), None, None, None)
    }

    #[test]
    fn provider_wrapper_exercises_missing_tool_invalid_input_child_failure_and_success() {
        let workspace = tempfile::tempdir().expect("provider integration tempdir");
        let root = workspace.path().join("hako-root");
        let tools = root.join("tools");
        let empty_bin = workspace.path().join("empty-bin");
        fs::create_dir_all(&tools).expect("provider tools directory");
        fs::create_dir_all(&empty_bin).expect("provider empty bin directory");
        let interpreter = resolve_python3().expect("Python is required for provider acceptance");
        let interpreter_dir = interpreter
            .parent()
            .expect("provider interpreter has a parent")
            .to_path_buf();
        fs::write(
            tools.join("llvmlite_harness.py"),
            r#"import os
import pathlib
import sys

args = iter(sys.argv[1:])
input_path = None
output_path = None
for arg in args:
    if arg == "--in":
        input_path = pathlib.Path(next(args))
    elif arg == "--out":
        output_path = pathlib.Path(next(args))
if input_path is None or output_path is None:
    raise SystemExit(61)
record = pathlib.Path(os.environ["HAKO_PROVIDER_RECORD_PATH"])
(record.with_suffix(".path")).write_text(str(input_path), encoding="utf-8")
(record.with_suffix(".input")).write_bytes(input_path.read_bytes())
if os.environ.get("HAKO_PROVIDER_MODE") == "fail":
    raise SystemExit(23)
if os.environ.get("HAKO_PROVIDER_MODE") == "success":
    output_path.write_bytes(b"provider-object")
    raise SystemExit(0)
raise SystemExit(61)
"#,
        )
        .expect("provider harness fixture");

        let record = workspace.path().join("provider-record");
        let root_text = root.to_string_lossy().into_owned();
        let interpreter_dir_text = interpreter_dir.to_string_lossy().into_owned();
        let empty_bin_text = empty_bin.to_string_lossy().into_owned();
        let record_text = record.to_string_lossy().into_owned();

        // The resolver must reject before a child is launched when no Python
        // executable is discoverable, while the scoped owner still cleans up.
        crate::test_support::with_env_vars(
            &[
                ("PATH", Some(empty_bin_text.as_str())),
                ("HAKO_ROOT", Some(root_text.as_str())),
                ("HAKO_PROVIDER_RECORD_PATH", Some(record_text.as_str())),
            ],
            || {
                let out = workspace.path().join("missing-tool.o");
                let error = mir_json_to_object_llvmlite(MIR_JSON, &provider_opts(&out))
                    .expect_err("missing provider tool must fail before launch");
                assert!(error.contains("python-not-found"), "{error}");
                assert!(!out.exists());
                assert!(!record.with_extension("path").exists());
            },
        );

        // Shape validation is the provider's input contract and must precede
        // tool discovery or child execution.
        crate::test_support::with_env_vars(
            &[
                ("PATH", Some(interpreter_dir_text.as_str())),
                ("HAKO_ROOT", Some(root_text.as_str())),
                ("HAKO_PROVIDER_RECORD_PATH", Some(record_text.as_str())),
                ("HAKO_PROVIDER_MODE", Some("success")),
            ],
            || {
                let out = workspace.path().join("invalid-input.o");
                let error = mir_json_to_object_llvmlite("{}", &provider_opts(&out))
                    .expect_err("invalid provider input must fail before child");
                assert!(error.contains("llvmemit/input/invalid"), "{error}");
                assert!(!out.exists());
                assert!(!record.with_extension("path").exists());
            },
        );

        // A child failure still has to observe the owned input, and the
        // owner must remove that exact path after the wrapper returns.
        crate::test_support::with_env_vars(
            &[
                ("PATH", Some(interpreter_dir_text.as_str())),
                ("HAKO_ROOT", Some(root_text.as_str())),
                ("HAKO_PROVIDER_RECORD_PATH", Some(record_text.as_str())),
                ("HAKO_PROVIDER_MODE", Some("fail")),
            ],
            || {
                let out = workspace.path().join("child-failure.o");
                let error = mir_json_to_object_llvmlite(MIR_JSON, &provider_opts(&out))
                    .expect_err("provider child failure must be returned");
                assert!(error.contains("status=23"), "{error}");
                assert!(!out.exists());
                let input_path = fs::read_to_string(record.with_extension("path"))
                    .expect("provider child recorded input path");
                assert_eq!(
                    fs::read(record.with_extension("input")).unwrap(),
                    MIR_JSON.as_bytes()
                );
                assert!(!PathBuf::from(input_path).exists());
            },
        );

        // Successful consumption validates the object terminal and the same
        // invocation-owned input is gone only after the child returned.
        crate::test_support::with_env_vars(
            &[
                ("PATH", Some(interpreter_dir_text.as_str())),
                ("HAKO_ROOT", Some(root_text.as_str())),
                ("HAKO_PROVIDER_RECORD_PATH", Some(record_text.as_str())),
                ("HAKO_PROVIDER_MODE", Some("success")),
            ],
            || {
                let out = workspace.path().join("success.o");
                let result = mir_json_to_object_llvmlite(MIR_JSON, &provider_opts(&out))
                    .expect("provider success should return object");
                assert_eq!(result, out);
                assert!(out.is_file());
                let input_path = fs::read_to_string(record.with_extension("path"))
                    .expect("provider success recorded input path");
                assert_eq!(
                    fs::read(record.with_extension("input")).unwrap(),
                    MIR_JSON.as_bytes()
                );
                assert!(!PathBuf::from(input_path).exists());
            },
        );
    }
}
