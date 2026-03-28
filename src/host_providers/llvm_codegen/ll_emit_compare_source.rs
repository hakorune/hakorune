use std::fs;
use std::path::{Path, PathBuf};

use serde_json::Value;

fn temporary_hako_driver_source_path(out_path: &Path, lane_tag: &str) -> PathBuf {
    let filename = out_path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("hako_ll_bridge");
    std::env::temp_dir().join(format!(
        "{}.{}.{}.driver.hako",
        filename,
        std::process::id(),
        lane_tag
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

pub(super) fn prepare_hako_driver_source(
    mir_json: &str,
    out_path: &Path,
    lane_tag: &str,
    acceptance_case: &str,
    legacy_daily_allowed: &str,
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
        .replace("__HAKO_LL_LANE__", lane_tag)
        .replace("__HAKO_LL_ACCEPTANCE_CASE__", acceptance_case)
        .replace("__HAKO_LL_LEGACY_DAILY_ALLOWED__", legacy_daily_allowed);
    let out = temporary_hako_driver_source_path(out_path, lane_tag);
    fs::write(&out, rendered).map_err(|e| {
        format!(
            "[llvmemit/hako-ll/template-write-failed] path={} error={}",
            out.display(),
            e
        )
    })?;
    Ok(out)
}
