use std::path::{Path, PathBuf};

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
    let status = std::process::Command::new(nyash_bin_path(cwd, profile))
        .args(["--backend", "llvm", app])
        .status()
        .map_err(|e| format!("spawn nyash llvm: {}", e))?;
    if !status.success() {
        return Err("LLVM emit failed".into());
    }
    Ok(())
}

fn nyash_bin_path(cwd: &Path, profile: &str) -> PathBuf {
    cwd.join("target")
        .join(profile)
        .join(if cfg!(windows) { "nyash.exe" } else { "nyash" })
}
