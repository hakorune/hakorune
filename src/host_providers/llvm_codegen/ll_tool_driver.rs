use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn resolve_llc_tool() -> Option<&'static str> {
    ["llc", "llc-18"].into_iter().find(|candidate| {
        Command::new(candidate)
            .arg("--version")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    })
}

fn resolve_opt_tool() -> Option<&'static str> {
    ["opt", "opt-18"].into_iter().find(|candidate| {
        Command::new(candidate)
            .arg("--version")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    })
}

pub(crate) fn temporary_ll_output_path(out_path: &Path, lane_tag: &str) -> PathBuf {
    let filename = out_path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("hako_ll_bridge");
    std::env::temp_dir().join(format!(
        "{}.{}.{}.ll",
        filename,
        std::process::id(),
        lane_tag
    ))
}

pub(crate) fn verify_ll_file(ll_path: &Path) -> Result<(), String> {
    let Some(opt) = resolve_opt_tool() else {
        return Ok(());
    };
    let output = Command::new(opt)
        .arg("-passes=verify")
        .arg("-disable-output")
        .arg(ll_path)
        .output()
        .map_err(|e| format!("[llvmemit/ll-tool/verify-spawn-failed] {}", e))?;
    if !output.status.success() {
        return Err(format!(
            "[llvmemit/ll-tool/verify-failed] stdout=`{}` stderr=`{}`",
            String::from_utf8_lossy(&output.stdout).trim(),
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    Ok(())
}

pub(crate) fn compile_ll_file_to_object(ll_path: &Path, out_path: &Path) -> Result<(), String> {
    let llc = resolve_llc_tool()
        .ok_or_else(|| "[llvmemit/ll-tool/llc-missing] llc not found".to_string())?;
    let output = Command::new(llc)
        .arg("-filetype=obj")
        .arg("-relocation-model=pic")
        .arg("-o")
        .arg(out_path)
        .arg(ll_path)
        .output()
        .map_err(|e| format!("[llvmemit/ll-tool/llc-spawn-failed] {}", e))?;
    if !output.status.success() {
        return Err(format!(
            "[llvmemit/ll-tool/llc-failed] stdout=`{}` stderr=`{}`",
            String::from_utf8_lossy(&output.stdout).trim(),
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    Ok(())
}

pub(crate) fn ll_text_to_object(
    ll_text: &str,
    out_path: &Path,
    lane_tag: &str,
) -> Result<(), String> {
    if ll_text.trim().is_empty() {
        return Err("[llvmemit/ll-tool/ll-text-empty] ll_text is empty".to_string());
    }
    let ll_path = temporary_ll_output_path(out_path, lane_tag);
    fs::write(&ll_path, ll_text).map_err(|e| {
        format!(
            "[llvmemit/ll-tool/output-write-failed] path={} error={}",
            ll_path.display(),
            e
        )
    })?;
    let result = (|| {
        verify_ll_file(&ll_path)?;
        compile_ll_file_to_object(&ll_path, out_path)
    })();
    let _ = fs::remove_file(&ll_path);
    result
}
