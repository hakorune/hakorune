use std::path::{Path, PathBuf};
use std::process::Command;

use super::ll_emit_compare_source;
use super::ll_tool_driver;
use super::normalize;
use super::transport_io;
use super::transport_paths;
use super::Opts;

const COMPARE_TAG: &str = "compare";

fn resolve_hakorune_bin() -> PathBuf {
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

fn run_driver_via_vm(hakorune: &Path, source_path: &Path) -> Result<String, String> {
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

fn extract_ll(stdout: &str) -> Result<String, String> {
    let begin = "[hako-ll/ll-begin]\n";
    let end = "\n[hako-ll/ll-end]";
    let start = stdout.find(begin).ok_or_else(|| {
        format!(
            "[llvmemit/hako-ll/ll-begin-missing] stdout=`{}`",
            stdout.trim()
        )
    })?;
    let content_start = start + begin.len();
    let tail = &stdout[content_start..];
    let end_offset = tail.find(end).ok_or_else(|| {
        format!(
            "[llvmemit/hako-ll/ll-end-missing] stdout=`{}`",
            stdout.trim()
        )
    })?;
    Ok(tail[..end_offset].to_string())
}

fn extract_contract_line(stdout: &str, lane_tag: &str) -> Result<String, String> {
    let prefix = format!("[hako-ll/{}] ", lane_tag);
    stdout
        .lines()
        .find(|line| line.starts_with(&prefix))
        .map(|line| line.to_string())
        .ok_or_else(|| {
            format!(
                "[llvmemit/hako-ll/contract-line-missing] lane={} stdout=`{}`",
                lane_tag,
                stdout.trim()
            )
        })
}

pub(super) fn mir_json_to_object_hako_ll_compare(
    mir_json: &str,
    opts: &Opts,
) -> Result<PathBuf, String> {
    normalize::validate_backend_mir_shape(mir_json)?;
    let hakorune = resolve_hakorune_bin();
    if !hakorune.exists() {
        return Err(format!(
            "[llvmemit/hako-ll/not-found] path={}",
            hakorune.display()
        ));
    }

    let out_path = super::transport_paths::resolve_backend_object_output(opts);
    super::transport_io::ensure_backend_output_parent(&out_path);
    let acceptance_case =
        crate::config::env::backend_acceptance_case().unwrap_or_else(|| "unset".to_string());
    let legacy_daily_allowed =
        crate::config::env::backend_legacy_daily_allowed().unwrap_or_else(|| "unknown".to_string());
    let source = ll_emit_compare_source::render_hako_driver_source(
        mir_json,
        COMPARE_TAG,
        &acceptance_case,
        &legacy_daily_allowed,
    )?;
    let source_path = transport_paths::build_backend_compare_source_path(&out_path, COMPARE_TAG);
    transport_io::write_backend_text_file(&source_path, &source)?;
    let result = (|| -> Result<PathBuf, String> {
        let stdout = run_driver_via_vm(&hakorune, &source_path)?;
        let contract_line = extract_contract_line(&stdout, COMPARE_TAG)?;
        let ll_text = extract_ll(&stdout)?;
        println!("{}", contract_line);
        ll_tool_driver::ll_text_to_object(&ll_text, &out_path, COMPARE_TAG)?;
        Ok(out_path)
    })();
    transport_io::remove_backend_temp_file(&source_path);
    result
}
