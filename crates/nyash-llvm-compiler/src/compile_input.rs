use std::env;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use serde_json::Value as JsonValue;

pub(super) fn resolve_harness_path(harness: Option<PathBuf>) -> PathBuf {
    harness.unwrap_or_else(|| PathBuf::from("tools/llvmlite_harness.py"))
}

pub(super) fn resolve_object_output_path(out: &Path, emit_exe: bool) -> PathBuf {
    if emit_exe {
        let mut path = out.to_path_buf();
        path.set_extension("o");
        path
    } else {
        out.to_path_buf()
    }
}

pub(super) fn prepare_input_json_path(
    infile: &str,
    canary_norm: bool,
) -> Result<(PathBuf, Option<PathBuf>)> {
    if infile == "-" {
        let value = read_input_json_value_from_stdin(canary_norm)?;
        let tmp = write_temp_input_json("ny_llvmc_stdin.json", &value)?;
        return Ok((tmp.clone(), Some(tmp)));
    }

    let input_path = PathBuf::from(infile);
    if canary_norm {
        let value = read_input_json_value_from_path(&input_path, canary_norm)?;
        let tmp = write_temp_input_json("ny_llvmc_in.json", &value)?;
        Ok((tmp.clone(), Some(tmp)))
    } else {
        Ok((input_path, None))
    }
}

pub(super) fn ensure_input_json_exists(input_path: &Path) -> Result<()> {
    if !input_path.exists() {
        bail!("input JSON not found: {}", input_path.display());
    }
    Ok(())
}

pub(super) fn maybe_dump_input_json(input_path: &Path) {
    if let Ok(dump_path) = env::var("NYASH_LLVM_DUMP_MIR_IN") {
        let _ = fs::copy(input_path, &dump_path);
        eprintln!("[ny-llvmc] dumped MIR input to {}", dump_path);
    }
}

pub(super) fn emit_preflight_shape_hint(input_path: &Path) {
    if let Some(hint) = read_shape_hint_from_path(input_path) {
        eprintln!("[ny-llvmc/hint] {}", hint);
    }
}

pub(super) fn maybe_emit_verbose_shape_hint(input_path: &Path, canary_norm: bool) {
    if env::var("NYASH_CLI_VERBOSE").ok().as_deref() == Some("1") && !canary_norm {
        if let Some(hint) = read_shape_hint_from_path(input_path) {
            eprintln!("[ny-llvmc/hint] {}", hint);
        }
    }
}

pub(super) fn cleanup_temp_input_json(temp_path: Option<PathBuf>) {
    if let Some(path) = temp_path {
        let _ = fs::remove_file(path);
    }
}

fn read_input_json_value_from_stdin(canary_norm: bool) -> Result<serde_json::Value> {
    let mut buf = String::new();
    std::io::stdin()
        .read_to_string(&mut buf)
        .context("reading MIR JSON from stdin")?;
    parse_input_json_value(&buf, "stdin does not contain valid JSON", canary_norm)
}

fn read_input_json_value_from_path(path: &Path, canary_norm: bool) -> Result<serde_json::Value> {
    let mut buf = String::new();
    File::open(path)
        .and_then(|mut f| f.read_to_string(&mut buf))
        .context("read input json")?;
    parse_input_json_value(&buf, "input is not valid JSON", canary_norm)
}

fn parse_input_json_value(
    input: &str,
    invalid_context: &'static str,
    canary_norm: bool,
) -> Result<serde_json::Value> {
    let mut value: serde_json::Value = serde_json::from_str(input).context(invalid_context)?;
    if canary_norm {
        value = normalize_canary_json(value);
    }
    Ok(value)
}

fn write_temp_input_json(filename: &str, value: &serde_json::Value) -> Result<PathBuf> {
    let tmp = env::temp_dir().join(filename);
    let mut file = File::create(&tmp).context("create temp json file")?;
    let serialized = serde_json::to_vec(value).context("serialize normalized json")?;
    file.write_all(&serialized).context("write temp json")?;
    Ok(tmp)
}

fn read_shape_hint_from_path(input_path: &Path) -> Option<String> {
    let input = fs::read_to_string(input_path).ok()?;
    let value = serde_json::from_str::<JsonValue>(&input).ok()?;
    shape_hint(&value)
}

/// Return a concise hint if the MIR JSON likely has a schema/shape mismatch for the Python harness.
fn shape_hint(v: &JsonValue) -> Option<String> {
    if let Some(sv) = v.get("schema_version") {
        if sv.is_number() {
            if sv.as_i64() == Some(1) {
                return Some(
                    "schema_version=1 detected; set to \"1.0\" or enable HAKO_LLVM_CANARY_NORMALIZE=1"
                        .into(),
                );
            }
        } else if sv.as_str() == Some("1") {
            return Some(
                "schema_version=\"1\" detected; prefer \"1.0\" or enable HAKO_LLVM_CANARY_NORMALIZE=1"
                    .into(),
            );
        }
    }

    if let Some(funcs) = v.get("functions") {
        if let Some(arr) = funcs.as_array() {
            for f in arr {
                if let Some(blocks) = f.get("blocks").and_then(|b| b.as_array()) {
                    for b in blocks {
                        if b.get("inst").is_some() && b.get("instructions").is_none() {
                            return Some(
                                "block key 'inst' found; rename to 'instructions' or enable HAKO_LLVM_CANARY_NORMALIZE=1"
                                    .into(),
                            );
                        }
                    }
                }
            }
        }
    }

    None
}

/// Normalize a very small canary JSON into the shape expected by the Python harness.
/// - Accepts schema_version as number or string; coerces to "1.0" when 1.
/// - Renames block key 'inst' -> 'instructions'.
/// - Converts const {"ty":"i64","value":N} into {"value":{"type":"i64","value":N}}
fn normalize_canary_json(mut v: serde_json::Value) -> serde_json::Value {
    use serde_json::{Map, Value};

    match v.get_mut("schema_version") {
        Some(Value::Number(n)) if n.as_i64() == Some(1) => {
            *v.get_mut("schema_version").unwrap() = Value::String("1.0".to_string());
        }
        Some(Value::String(s)) if s == "1" => {
            *v.get_mut("schema_version").unwrap() = Value::String("1.0".to_string());
        }
        _ => {}
    }

    if let Some(funcs) = v.get_mut("functions") {
        if let Value::Array(ref mut arr) = funcs {
            for func in arr.iter_mut() {
                if let Value::Object(ref mut fm) = func {
                    if let Some(blocks_v) = fm.get_mut("blocks") {
                        if let Value::Array(ref mut blks) = blocks_v {
                            for blk in blks.iter_mut() {
                                if let Value::Object(ref mut bm) = blk {
                                    if let Some(insts) = bm.remove("inst") {
                                        bm.insert("instructions".to_string(), insts);
                                    }
                                    if let Some(Value::Array(ref mut ins_arr)) =
                                        bm.get_mut("instructions")
                                    {
                                        for ins in ins_arr.iter_mut() {
                                            if let Value::Object(ref mut im) = ins {
                                                if im.get("op").and_then(|x| x.as_str())
                                                    == Some("const")
                                                {
                                                    if let (Some(ty), Some(val)) =
                                                        (im.remove("ty"), im.remove("value"))
                                                    {
                                                        let mut val_obj = Map::new();
                                                        if let Value::String(ts) = ty {
                                                            val_obj.insert(
                                                                "type".to_string(),
                                                                Value::String(ts),
                                                            );
                                                        } else {
                                                            val_obj.insert("type".to_string(), ty);
                                                        }
                                                        val_obj.insert("value".to_string(), val);
                                                        im.insert(
                                                            "value".to_string(),
                                                            Value::Object(val_obj),
                                                        );
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_harness_path_defaults_to_workspace_wrapper() {
        assert_eq!(
            resolve_harness_path(None),
            PathBuf::from("tools/llvmlite_harness.py")
        );
    }

    #[test]
    fn resolve_object_output_path_uses_o_suffix_for_exe() {
        assert_eq!(
            resolve_object_output_path(Path::new("/tmp/out.exe"), true),
            PathBuf::from("/tmp/out.o")
        );
        assert_eq!(
            resolve_object_output_path(Path::new("/tmp/out.o"), false),
            PathBuf::from("/tmp/out.o")
        );
    }
}
