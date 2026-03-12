pub(crate) const PYVM_RUNNER_PATH: &str = "tools/historical/pyvm/pyvm_runner.py";

pub(crate) fn resolve_runner_path() -> Option<std::path::PathBuf> {
    let runner = std::path::PathBuf::from(PYVM_RUNNER_PATH);
    if runner.exists() {
        return Some(runner);
    }
    if let Ok(root) = std::env::var("NYASH_ROOT") {
        let alt = std::path::Path::new(&root).join(PYVM_RUNNER_PATH);
        if alt.exists() {
            return Some(alt);
        }
    }
    None
}

/// Run PyVM harness over a nyash_rust (lib) MIR module.
/// This legacy route is used only by LLVM diagnostics (`SMOKES_USE_PYVM=1`).
pub(crate) fn run_pyvm_harness_lib(
    module: &nyash_rust::mir::MirModule,
    tag: &str,
) -> Result<i32, String> {
    let py3 = which::which("python3").map_err(|e| format!("python3 not found: {}", e))?;
    let runner_buf = match resolve_runner_path() {
        Some(path) => path,
        None => {
            return Err(format!(
                "PyVM runner not found: {} (cwd) or $NYASH_ROOT/{}",
                PYVM_RUNNER_PATH, PYVM_RUNNER_PATH
            ));
        }
    };
    let tmp_dir = std::path::Path::new("tmp");
    let _ = std::fs::create_dir_all(tmp_dir);
    let mir_json_path = tmp_dir.join("nyash_pyvm_mir.json");
    crate::runner::mir_json_emit::emit_mir_json_for_harness(module, &mir_json_path)
        .map_err(|e| format!("PyVM MIR JSON emit error: {}", e))?;
    crate::cli_v!(
        "[Runner] using PyVM ({} ) → {}",
        tag,
        mir_json_path.display()
    );
    // NamingBox SSOT: Select entry (arity-aware, Main.main → main fallback)
    let entry = super::super::entry_selection::select_entry_function(&module);
    let mut cmd = std::process::Command::new(py3);
    crate::runner::child_env::apply_core_wrapper_env(&mut cmd);
    if std::env::var("NYASH_MINIVM_READ_STDIN").ok().as_deref() == Some("1") {
        use std::io::Read;
        let mut buf = String::new();
        let _ = std::io::stdin().read_to_string(&mut buf);
        let arg_json = serde_json::json!([buf]).to_string();
        cmd.env("NYASH_SCRIPT_ARGS_JSON", arg_json);
    }
    let status = cmd
        .args([
            runner_buf.to_string_lossy().as_ref(),
            "--in",
            &mir_json_path.display().to_string(),
            "--entry",
            &entry,
            "--args-env",
            "NYASH_SCRIPT_ARGS_JSON",
        ])
        .status()
        .map_err(|e| format!("spawn pyvm: {}", e))?;
    let code = status.code().unwrap_or(1);
    if !status.success() {
        crate::cli_v!("❌ PyVM ({}) failed (status={})", tag, code);
    }
    Ok(code)
}
