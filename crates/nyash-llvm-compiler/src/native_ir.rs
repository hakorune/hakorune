use anyhow::{bail, Context, Result};
use serde_json::Value as JsonValue;

pub(super) fn build_dummy_ir() -> String {
    "; ModuleID = \"nyash_native\"\n\ndefine i64 @ny_main() {\nentry:\n  ret i64 0\n}\n".to_string()
}

pub(super) fn build_ir_from_mir_json(json: &JsonValue) -> Result<String> {
    let func = find_entry_function(json)
        .context("native driver currently requires an entry function named `main` or `ny_main`")?;
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
                        bail!(
                            "native driver currently supports only i64 const, got {}",
                            ty
                        );
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
        prologue.push(
            "@.fmt_i64 = private unnamed_addr constant [5 x i8] c\"%ld\\0A\\00\", align 1"
                .to_string(),
        );
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
}
