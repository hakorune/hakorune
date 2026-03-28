use std::path::{Path, PathBuf};
use std::process::Command;

pub(super) fn resolve_hakorune_bin() -> PathBuf {
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

pub(super) fn run_driver_via_vm(hakorune: &Path, source_path: &Path) -> Result<String, String> {
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
