use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{Context, Result, bail};
use serde_json::Value as JsonValue;

pub fn emit_dummy_object(out: &Path) -> Result<()> {
    let ir = build_dummy_ir();
    compile_ir_to_object(&ir, out)
}

pub fn emit_object_from_json(input: &Path, out: &Path) -> Result<()> {
    let text = fs::read_to_string(input)
        .with_context(|| format!("failed to read MIR JSON: {}", input.display()))?;
    let json: JsonValue = serde_json::from_str(&text)
        .with_context(|| format!("failed to parse MIR JSON: {}", input.display()))?;
    let ir = build_ir_from_mir_json(&json)?;
    compile_ir_to_object(&ir, out)
}

fn build_dummy_ir() -> String {
    "; ModuleID = \"nyash_native\"\n\ndefine i64 @ny_main() {\nentry:\n  ret i64 0\n}\n".to_string()
}

fn build_ir_from_mir_json(json: &JsonValue) -> Result<String> {
    let func = find_entry_function(json).context("native driver currently requires an entry function named `main` or `ny_main`")?;
    let blocks = func
        .get("blocks")
        .and_then(JsonValue::as_array)
        .context("entry function blocks[] missing")?;

    let mut prologue = vec!["; ModuleID = \"nyash_native\"".to_string()];
    let mut body = vec![String::new(), "define i64 @ny_main() {".to_string()];
    let mut needs_printf = false;

    for block in blocks {
        let block_id = block
            .get("id")
            .and_then(JsonValue::as_i64)
            .context("block.id missing or non-integer")?;
        body.push(format!("bb{}:", block_id));

        let instructions = block
            .get("instructions")
            .and_then(JsonValue::as_array)
            .context("block.instructions missing")?;
        for instruction in instructions {
            let op = instruction
                .get("op")
                .and_then(JsonValue::as_str)
                .context("instruction.op missing")?;
            match op {
                "const" => {
                    let dst = instruction
                        .get("dst")
                        .and_then(JsonValue::as_i64)
                        .context("const.dst missing")?;
                    let value = instruction
                        .get("value")
                        .and_then(JsonValue::as_object)
                        .context("const.value missing")?;
                    let ty = value
                        .get("type")
                        .and_then(JsonValue::as_str)
                        .unwrap_or("i64");
                    if ty != "i64" {
                        bail!("native driver currently supports only i64 const, got {}", ty);
                    }
                    let int_value = value
                        .get("value")
                        .and_then(JsonValue::as_i64)
                        .context("const.value.value missing")?;
                    body.push(format!("  %v{} = add i64 0, {}", dst, int_value));
                }
                "mir_call" => {
                    let call = instruction
                        .get("mir_call")
                        .and_then(JsonValue::as_object)
                        .context("mir_call payload missing")?;
                    let callee = call
                        .get("callee")
                        .and_then(JsonValue::as_object)
                        .context("mir_call.callee missing")?;
                    let callee_name = callee
                        .get("name")
                        .and_then(JsonValue::as_str)
                        .context("mir_call.callee.name missing")?;
                    if callee_name != "print" {
                        bail!(
                            "native driver currently supports only Global print mir_call, got {}",
                            callee_name
                        );
                    }
                    let args = call
                        .get("args")
                        .and_then(JsonValue::as_array)
                        .context("mir_call.args missing")?;
                    if args.len() != 1 {
                        bail!(
                            "native driver currently supports only print/1, got arity {}",
                            args.len()
                        );
                    }
                    let arg_id = args[0]
                        .as_i64()
                        .context("mir_call print arg must be integer value id")?;
                    needs_printf = true;
                    body.push(format!(
                        "  %print_call_{} = call i32 (ptr, ...) @printf(ptr getelementptr inbounds ([5 x i8], ptr @.fmt_i64, i64 0, i64 0), i64 %v{})",
                        arg_id, arg_id
                    ));
                }
                "ret" => {
                    let value_id = instruction
                        .get("value")
                        .and_then(JsonValue::as_i64)
                        .context("ret.value missing")?;
                    body.push(format!("  ret i64 %v{}", value_id));
                }
                other => bail!(
                    "native driver currently supports only const/ret/print canary ops, got {}",
                    other
                ),
            }
        }
    }

    if needs_printf {
        prologue.push("@.fmt_i64 = private unnamed_addr constant [5 x i8] c\"%ld\\0A\\00\", align 1".to_string());
        prologue.push("declare i32 @printf(ptr, ...)".to_string());
    }

    body.push("}".to_string());
    prologue.extend(body);
    Ok(prologue.join("\n") + "\n")
}

fn find_entry_function<'a>(json: &'a JsonValue) -> Option<&'a JsonValue> {
    let functions = json.get("functions")?.as_array()?;
    functions.iter().find(|function| {
        matches!(
            function.get("name").and_then(JsonValue::as_str),
            Some("ny_main" | "main")
        )
    })
}

fn compile_ir_to_object(ir: &str, out: &Path) -> Result<()> {
    if let Some(parent) = out.parent() {
        fs::create_dir_all(parent).ok();
    }
    let ll_path = temporary_ll_path(out);
    fs::write(&ll_path, ir).with_context(|| format!("failed to write IR file: {}", ll_path.display()))?;
    let llc = resolve_llc().context("llc not found in PATH for native driver")?;

    let output = Command::new(&llc)
        .arg("-filetype=obj")
        .arg("-relocation-model=pic")
        .arg("-o")
        .arg(out)
        .arg(&ll_path)
        .output()
        .with_context(|| format!("failed to invoke {}", llc))?;
    let _ = fs::remove_file(&ll_path);
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        bail!(
            "native driver llc failed (tool={}): stdout=`{}` stderr=`{}`",
            llc,
            stdout.trim(),
            stderr.trim()
        );
    }
    Ok(())
}

fn resolve_llc() -> Option<&'static str> {
    ["llc", "llc-18"].into_iter().find(|candidate| {
        Command::new(candidate)
            .arg("--version")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    })
}

fn temporary_ll_path(out: &Path) -> PathBuf {
    let filename = out
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("nyash_native");
    std::env::temp_dir().join(format!("{}.{}.ll", filename, std::process::id()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_ir_for_collapsed_min_fixture() {
        let fixture = include_str!("../../../apps/tests/mir_shape_guard/collapsed_min.mir.json");
        let json: JsonValue = serde_json::from_str(fixture).unwrap();
        let ir = build_ir_from_mir_json(&json).unwrap();
        assert!(ir.contains("define i64 @ny_main()"));
        assert!(ir.contains("%v1 = add i64 0, 0"));
        assert!(ir.contains("ret i64 %v1"));
    }

    #[test]
    fn builds_ir_for_hello_simple_fixture_with_printf() {
        let fixture = include_str!("../../../apps/tests/hello_simple_llvm_native_probe.mir.json");
        let json: JsonValue = serde_json::from_str(fixture).unwrap();
        let ir = build_ir_from_mir_json(&json).unwrap();
        assert!(ir.contains("declare i32 @printf(ptr, ...)"));
        assert!(ir.contains("@.fmt_i64"));
        assert!(ir.contains("call i32 (ptr, ...) @printf"));
        assert!(ir.contains("ret i64 %v3"));
    }

    #[test]
    fn emits_native_object_for_collapsed_min_fixture_when_llc_exists() {
        if resolve_llc().is_none() {
            return;
        }

        let fixture = include_str!("../../../apps/tests/mir_shape_guard/collapsed_min.mir.json");
        let tmp_in = std::env::temp_dir().join(format!(
            "nyllvmc_native_fixture_{}.mir.json",
            std::process::id()
        ));
        let tmp_out = std::env::temp_dir().join(format!(
            "nyllvmc_native_fixture_{}.o",
            std::process::id()
        ));
        fs::write(&tmp_in, fixture).unwrap();

        let result = emit_object_from_json(&tmp_in, &tmp_out);
        let _ = fs::remove_file(&tmp_in);
        let object_exists = tmp_out.exists();
        let _ = fs::remove_file(&tmp_out);

        assert!(result.is_ok(), "{:?}", result.err());
        assert!(object_exists);
    }

    #[test]
    fn emits_native_object_for_hello_simple_fixture_when_llc_exists() {
        if resolve_llc().is_none() {
            return;
        }

        let fixture = include_str!("../../../apps/tests/hello_simple_llvm_native_probe.mir.json");
        let tmp_in = std::env::temp_dir().join(format!(
            "nyllvmc_native_hello_fixture_{}.mir.json",
            std::process::id()
        ));
        let tmp_out = std::env::temp_dir().join(format!(
            "nyllvmc_native_hello_fixture_{}.o",
            std::process::id()
        ));
        fs::write(&tmp_in, fixture).unwrap();

        let result = emit_object_from_json(&tmp_in, &tmp_out);
        let _ = fs::remove_file(&tmp_in);
        let object_exists = tmp_out.exists();
        let _ = fs::remove_file(&tmp_out);

        assert!(result.is_ok(), "{:?}", result.err());
        assert!(object_exists);
    }
}
