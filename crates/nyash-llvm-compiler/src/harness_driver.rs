use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{bail, Result};

use super::compile_input;

pub(super) fn run_harness_dummy(harness: Option<&PathBuf>, out: &Path) -> Result<()> {
    ensure_python()?;
    let harness = compile_input::resolve_harness_path(harness.cloned());
    let mut cmd = Command::new("python3");
    cmd.arg(&harness).arg("--out").arg(out);
    propagate_opt_level(&mut cmd);
    let status = cmd
        .status()
        .map_err(|_| anyhow::anyhow!("failed to execute python harness (dummy)"))?;
    if !status.success() {
        bail!("harness exited with status: {:?}", status.code());
    }
    Ok(())
}

pub(super) fn run_harness_in(harness: Option<&PathBuf>, input: &Path, out: &Path) -> Result<()> {
    ensure_python()?;
    let harness = compile_input::resolve_harness_path(harness.cloned());
    let mut cmd = Command::new("python3");
    cmd.arg(&harness)
        .arg("--in")
        .arg(input)
        .arg("--out")
        .arg(out);
    propagate_opt_level(&mut cmd);
    let status = cmd
        .status()
        .map_err(|_| anyhow::anyhow!("failed to execute python harness"))?;
    if !status.success() {
        bail!("harness exited with status: {:?}", status.code());
    }
    Ok(())
}

fn ensure_python() -> Result<()> {
    match Command::new("python3").arg("--version").output() {
        Ok(out) if out.status.success() => Ok(()),
        _ => bail!("python3 not found in PATH (required for compat llvmlite harness)"),
    }
}

fn propagate_opt_level(cmd: &mut Command) {
    let hako = env::var("HAKO_LLVM_OPT_LEVEL").ok();
    let nyash = env::var("NYASH_LLVM_OPT_LEVEL").ok();
    let level = nyash.clone().or(hako.clone());
    if let Some(level) = level {
        if hako.is_some() && nyash.is_none() {
            eprintln!(
                "[deprecate/env] 'HAKO_LLVM_OPT_LEVEL' is deprecated; use 'NYASH_LLVM_OPT_LEVEL'"
            );
        }
        cmd.env("HAKO_LLVM_OPT_LEVEL", &level);
        cmd.env("NYASH_LLVM_OPT_LEVEL", &level);
    }
}
