use std::path::{Path, PathBuf};

pub(super) fn build_engineering_artifact(
    cwd: &Path,
    profile: &str,
    app: &str,
    obj_dir: &Path,
) -> Result<(), String> {
    emit_engineering_object(cwd, profile, app, obj_dir)
}

fn emit_engineering_object(
    cwd: &Path,
    profile: &str,
    app: &str,
    obj_dir: &Path,
) -> Result<(), String> {
    std::env::set_var("NYASH_AOT_OBJECT_OUT", obj_dir);
    println!(
        "[emit] Cranelift object → {} (directory)",
        obj_dir.display()
    );
    let status = std::process::Command::new(nyash_bin_path(cwd, profile))
        .args(["--backend", "vm", app])
        .status()
        .map_err(|e| format!("spawn nyash jit-aot: {}", e))?;
    if !status.success() {
        return Err("Cranelift emit failed".into());
    }
    Ok(())
}

fn nyash_bin_path(cwd: &Path, profile: &str) -> PathBuf {
    cwd.join("target")
        .join(profile)
        .join(if cfg!(windows) { "nyash.exe" } else { "nyash" })
}
