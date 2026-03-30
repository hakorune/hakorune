use std::process::Command;

use super::driver_source::VM_HAKO_DRIVER_SOURCE;
use super::{temp_seed, VmHakoErr, VM_HAKO_PHASE};

pub(super) fn run_vm_hako_driver(filename: &str, payload_json: &str) -> Result<i32, VmHakoErr> {
    let driver_path = std::env::temp_dir().join(format!(
        "vm_hako_{}_driver_{}.hako",
        VM_HAKO_PHASE,
        temp_seed()
    ));
    if let Err(e) = std::fs::write(&driver_path, VM_HAKO_DRIVER_SOURCE) {
        return Err((
            "driver-write-error",
            format!("file={} message={}", filename, e),
        ));
    }

    let (payload_env, payload_path) = prepare_payload_transport(filename, payload_json)?;
    maybe_dump_payload_trace(payload_json);

    let exe = std::env::current_exe().unwrap_or_else(|_| std::path::PathBuf::from("hakorune"));
    let mut cmd = Command::new(&exe);
    cmd.arg("--backend")
        .arg("vm")
        .arg(&driver_path)
        // VM route cutover default is vm-hako on strict/dev.
        // Driver subprocess must run on rust-vm to execute MiniVmS0EntryBox without re-entering vm-hako subset checks.
        .env("NYASH_VM_HAKO_PREFER_STRICT_DEV", "0")
        // Driver compilation is internal plumbing; do not inherit strict+planner gate from parent smoke env.
        .env("HAKO_JOINIR_STRICT", "0")
        .env("NYASH_JOINIR_STRICT", "0")
        .env("HAKO_JOINIR_PLANNER_REQUIRED", "0")
        .env("NYASH_VERIFY_JSON", payload_env)
        .env("NYASH_PREINCLUDE", "1")
        .env("NYASH_USING_AST", "1")
        .env("NYASH_RESOLVE_FIX_BRACES", "1")
        .env("NYASH_FEATURES", "stage3")
        .env("NYASH_PARSER_ALLOW_SEMICOLON", "1")
        .env("NYASH_PARSER_SEAM_TOLERANT", "1")
        .env("NYASH_ENTRY_ALLOW_TOPLEVEL_MAIN", "1")
        .env("NYASH_ENABLE_USING", "1")
        .env("HAKO_ENABLE_USING", "1")
        .env("NYASH_DISABLE_NY_COMPILER", "1")
        .env("HAKO_DISABLE_NY_COMPILER", "1")
        .env("NYASH_USE_NY_COMPILER", "0")
        .env("HAKO_FAIL_FAST_ON_HAKO_IN_NYASH_VM", "0");
    let status = cmd.status();
    let _ = std::fs::remove_file(&driver_path);
    if let Some(path) = payload_path {
        let _ = std::fs::remove_file(path);
    }

    match status {
        Ok(st) => Ok(st.code().unwrap_or(1)),
        Err(e) => Err(("spawn-error", format!("file={} message={}", filename, e))),
    }
}

fn maybe_dump_payload_trace(payload_json: &str) {
    if std::env::var("NYASH_EMIT_MIR_TRACE").ok().as_deref() != Some("1") {
        return;
    }
    let dump_path = std::env::temp_dir().join(format!(
        "vm_hako_{}_payload_dump_{}.json",
        VM_HAKO_PHASE,
        temp_seed()
    ));
    if let Err(e) = std::fs::write(&dump_path, payload_json) {
        eprintln!(
            "[vm-hako/payload-trace] failed to dump payload JSON to {}: {}",
            dump_path.display(),
            e
        );
    } else {
        eprintln!(
            "[vm-hako/payload-trace] dumped payload JSON to {}",
            dump_path.display()
        );
    }
}

fn prepare_payload_transport(
    filename: &str,
    payload_json: &str,
) -> Result<(String, Option<std::path::PathBuf>), VmHakoErr> {
    const INLINE_PAYLOAD_LIMIT: usize = 16 * 1024;
    if payload_json.len() <= INLINE_PAYLOAD_LIMIT {
        return Ok((payload_json.to_string(), None));
    }

    let payload_path = std::env::temp_dir().join(format!(
        "vm_hako_{}_payload_{}.json",
        VM_HAKO_PHASE,
        temp_seed()
    ));
    if let Err(e) = std::fs::write(&payload_path, payload_json) {
        return Err((
            "driver-write-error",
            format!("file={} message={}", filename, e),
        ));
    }
    Ok((
        format!("@file:{}", payload_path.to_string_lossy()),
        Some(payload_path),
    ))
}
