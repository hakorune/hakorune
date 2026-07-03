use super::build_shared::hakorune_cli_bin_path;
use std::path::Path;

pub(super) fn build_product_artifact(
    cwd: &Path,
    profile: &str,
    app: &str,
    obj_path: &Path,
) -> Result<(), String> {
    emit_llvm_object(cwd, profile, app, obj_path)
}

fn emit_llvm_object(cwd: &Path, profile: &str, app: &str, obj_path: &Path) -> Result<(), String> {
    std::env::set_var("NYASH_LLVM_OBJ_OUT", obj_path);
    println!("[emit] LLVM object → {}", obj_path.display());
    let status = std::process::Command::new(hakorune_cli_bin_path(cwd, profile))
        .args(["--backend", "llvm", app])
        .status()
        .map_err(|e| format!("spawn hakorune llvm: {}", e))?;
    if !status.success() {
        return Err("LLVM emit failed".into());
    }
    Ok(())
}
