use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{bail, Context, Result};
use serde_json::Value as JsonValue;

#[path = "native_ir.rs"]
mod native_ir;

pub fn emit_dummy_object(out: &Path) -> Result<()> {
    let ir = native_ir::build_dummy_ir();
    compile_ir_to_object(&ir, out)
}

pub fn emit_object_from_json(input: &Path, out: &Path) -> Result<()> {
    let text = fs::read_to_string(input)
        .with_context(|| format!("failed to read MIR JSON: {}", input.display()))?;
    let json: JsonValue = serde_json::from_str(&text)
        .with_context(|| format!("failed to parse MIR JSON: {}", input.display()))?;
    let ir = native_ir::build_ir_from_mir_json(&json)?;
    compile_ir_to_object(&ir, out)
}

fn compile_ir_to_object(ir: &str, out: &Path) -> Result<()> {
    if let Some(parent) = out.parent() {
        fs::create_dir_all(parent).ok();
    }
    let ll_path = temporary_ll_path(out);
    fs::write(&ll_path, ir)
        .with_context(|| format!("failed to write IR file: {}", ll_path.display()))?;
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
    fn emits_native_object_for_collapsed_min_fixture_when_llc_exists() {
        if resolve_llc().is_none() {
            return;
        }

        let fixture = include_str!("../../../apps/tests/mir_shape_guard/collapsed_min.mir.json");
        let tmp_in = std::env::temp_dir().join(format!(
            "nyllvmc_native_fixture_{}.mir.json",
            std::process::id()
        ));
        let tmp_out =
            std::env::temp_dir().join(format!("nyllvmc_native_fixture_{}.o", std::process::id()));
        std::fs::write(&tmp_in, fixture).unwrap();

        let result = emit_object_from_json(&tmp_in, &tmp_out);
        let _ = std::fs::remove_file(&tmp_in);
        let object_exists = tmp_out.exists();
        let _ = std::fs::remove_file(&tmp_out);

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
        std::fs::write(&tmp_in, fixture).unwrap();

        let result = emit_object_from_json(&tmp_in, &tmp_out);
        let _ = std::fs::remove_file(&tmp_in);
        let object_exists = tmp_out.exists();
        let _ = std::fs::remove_file(&tmp_out);

        assert!(result.is_ok(), "{:?}", result.err());
        assert!(object_exists);
    }
}
