use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use serde_json::Value;

use super::transport::{ensure_backend_output_parent, resolve_backend_object_output};
use super::{normalize, Opts};

enum BridgeLane {
    Daily,
    Compare,
}

impl BridgeLane {
    fn tag(&self) -> &'static str {
        match self {
            Self::Daily => "daily",
            Self::Compare => "compare",
        }
    }
}

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

fn compile_ll_to_object(ll_path: &Path, out_path: &Path) -> Result<(), String> {
    let llc =
        resolve_llc_tool().ok_or_else(|| "[llvmemit/hako-ll/llc-missing] llc not found".to_string())?;
    let output = Command::new(llc)
        .arg("-filetype=obj")
        .arg("-relocation-model=pic")
        .arg("-o")
        .arg(out_path)
        .arg(ll_path)
        .output()
        .map_err(|e| format!("[llvmemit/hako-ll/llc-spawn-failed] {}", e))?;
    if !output.status.success() {
        return Err(format!(
            "[llvmemit/hako-ll/llc-failed] stdout=`{}` stderr=`{}`",
            String::from_utf8_lossy(&output.stdout).trim(),
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    Ok(())
}

fn verify_ll_file(ll_path: &Path) -> Result<(), String> {
    let Some(opt) = resolve_opt_tool() else {
        return Ok(());
    };
    let output = Command::new(opt)
        .arg("-passes=verify")
        .arg("-disable-output")
        .arg(ll_path)
        .output()
        .map_err(|e| format!("[llvmemit/hako-ll/verify-spawn-failed] {}", e))?;
    if !output.status.success() {
        return Err(format!(
            "[llvmemit/hako-ll/verify-failed] stdout=`{}` stderr=`{}`",
            String::from_utf8_lossy(&output.stdout).trim(),
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    Ok(())
}

fn temporary_ll_output_path(out_path: &Path, lane: &BridgeLane) -> PathBuf {
    let filename = out_path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("hako_ll_bridge");
    std::env::temp_dir().join(format!(
        "{}.{}.{}.ll",
        filename,
        std::process::id(),
        lane.tag()
    ))
}

fn temporary_hako_driver_source_path(out_path: &Path, lane: &BridgeLane) -> PathBuf {
    let filename = out_path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("hako_ll_bridge");
    std::env::temp_dir().join(format!(
        "{}.{}.{}.driver.hako",
        filename,
        std::process::id(),
        lane.tag()
    ))
}

fn escape_hako_string_literal(text: &str) -> String {
    text.replace('\\', "\\\\").replace('"', "\\\"")
}

fn render_hako_embedded_value(
    value: &Value,
    counter: &mut usize,
    body: &mut String,
    indent: &str,
) -> Result<String, String> {
    match value {
        Value::Null => Ok("null".to_string()),
        Value::Bool(v) => Ok(if *v { "1" } else { "0" }.to_string()),
        Value::Number(num) => {
            if let Some(v) = num.as_i64() {
                Ok(v.to_string())
            } else if let Some(v) = num.as_u64() {
                if v <= i64::MAX as u64 {
                    Ok(v.to_string())
                } else {
                    Err(format!(
                        "[llvmemit/hako-ll/unsupported-number] unsigned out of i64 range: {}",
                        v
                    ))
                }
            } else {
                Err(format!(
                    "[llvmemit/hako-ll/unsupported-number] non-integer literal: {}",
                    num
                ))
            }
        }
        Value::String(text) => Ok(format!("\"{}\"", escape_hako_string_literal(text))),
        Value::Array(items) => {
            let var = format!("v{}", *counter);
            *counter += 1;
            body.push_str(&format!("{indent}local {var} = new ArrayBox()\n"));
            for item in items {
                let expr = render_hako_embedded_value(item, counter, body, indent)?;
                body.push_str(&format!("{indent}{var}.push({expr})\n"));
            }
            Ok(var)
        }
        Value::Object(map) => {
            let var = format!("v{}", *counter);
            *counter += 1;
            body.push_str(&format!("{indent}local {var} = new MapBox()\n"));
            for (key, item) in map {
                let expr = render_hako_embedded_value(item, counter, body, indent)?;
                body.push_str(&format!(
                    "{indent}{var}.set(\"{}\", {expr})\n",
                    escape_hako_string_literal(key)
                ));
            }
            Ok(var)
        }
    }
}

fn render_hako_root_builder(mir_json: &str) -> Result<String, String> {
    let value: Value = serde_json::from_str(mir_json)
        .map_err(|e| format!("[llvmemit/hako-ll/json-parse-failed] {}", e))?;
    let mut counter = 0usize;
    let mut body = String::new();
    let root = render_hako_embedded_value(&value, &mut counter, &mut body, "    ")?;
    body.push_str(&format!("    return {root}\n"));
    Ok(body)
}

fn prepare_hako_driver_source(
    mir_json: &str,
    out_path: &Path,
    lane: &BridgeLane,
) -> Result<PathBuf, String> {
    let template = PathBuf::from("lang/src/shared/backend/ll_emit/driver.hako");
    let source = fs::read_to_string(&template).map_err(|e| {
        format!(
            "[llvmemit/hako-ll/template-read-failed] path={} error={}",
            template.display(),
            e
        )
    })?;
    let root_builder = render_hako_root_builder(mir_json)?;
    let rendered = source
        .replace(
            "    // __HAKO_LL_COMPARE_ROOT_BUILDER__\n    return null\n",
            &root_builder,
        )
        .replace("__HAKO_LL_LANE__", lane.tag());
    let out = temporary_hako_driver_source_path(out_path, lane);
    fs::write(&out, rendered).map_err(|e| {
        format!(
            "[llvmemit/hako-ll/template-write-failed] path={} error={}",
            out.display(),
            e
        )
    })?;
    Ok(out)
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
    let start = stdout
        .find(begin)
        .ok_or_else(|| format!("[llvmemit/hako-ll/ll-begin-missing] stdout=`{}`", stdout.trim()))?;
    let content_start = start + begin.len();
    let tail = &stdout[content_start..];
    let end_offset = tail
        .find(end)
        .ok_or_else(|| format!("[llvmemit/hako-ll/ll-end-missing] stdout=`{}`", stdout.trim()))?;
    Ok(tail[..end_offset].to_string())
}

fn extract_contract_line(stdout: &str, lane: &BridgeLane) -> Result<String, String> {
    let prefix = format!("[hako-ll/{}] ", lane.tag());
    stdout
        .lines()
        .find(|line| line.starts_with(&prefix))
        .map(|line| line.to_string())
        .ok_or_else(|| {
            format!(
                "[llvmemit/hako-ll/contract-line-missing] lane={} stdout=`{}`",
                lane.tag(),
                stdout.trim()
            )
        })
}

fn mir_json_to_object_hako_ll(
    mir_json: &str,
    opts: &Opts,
    lane: BridgeLane,
) -> Result<PathBuf, String> {
    normalize::validate_backend_mir_shape(mir_json)?;
    let hakorune = resolve_hakorune_bin();
    if !hakorune.exists() {
        return Err(format!(
            "[llvmemit/hako-ll/not-found] path={}",
            hakorune.display()
        ));
    }

    let out_path = resolve_backend_object_output(opts);
    ensure_backend_output_parent(&out_path);
    let ll_path = temporary_ll_output_path(&out_path, &lane);
    let source_path = prepare_hako_driver_source(mir_json, &out_path, &lane)?;
    let stdout = run_driver_via_vm(&hakorune, &source_path)?;
    let contract_line = extract_contract_line(&stdout, &lane)?;
    let ll_text = extract_ll(&stdout)?;
    fs::write(&ll_path, ll_text).map_err(|e| {
        format!(
            "[llvmemit/hako-ll/output-write-failed] path={} error={}",
            ll_path.display(),
            e
        )
    })?;
    println!("{}", contract_line);
    verify_ll_file(&ll_path)?;
    compile_ll_to_object(&ll_path, &out_path)?;
    let _ = fs::remove_file(&source_path);
    let _ = fs::remove_file(&ll_path);
    Ok(out_path)
}

pub(super) fn mir_json_to_object_hako_ll_daily(
    mir_json: &str,
    opts: &Opts,
) -> Result<PathBuf, String> {
    mir_json_to_object_hako_ll(mir_json, opts, BridgeLane::Daily)
}

pub(super) fn mir_json_to_object_hako_ll_compare(
    mir_json: &str,
    opts: &Opts,
) -> Result<PathBuf, String> {
    mir_json_to_object_hako_ll(mir_json, opts, BridgeLane::Compare)
}
