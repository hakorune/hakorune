use super::build_shared::hakorune_cli_bin_path;
use std::path::Path;

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
    let status = std::process::Command::new(hakorune_cli_bin_path(cwd, profile))
        .args(["--backend", "vm", app])
        .status()
        .map_err(|e| format!("spawn hakorune jit-aot: {}", e))?;
    if !status.success() {
        return Err("Cranelift emit failed".into());
    }
    Ok(())
}
