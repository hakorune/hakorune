/*!
 * CoreExecutor — loaded `MirModule` execution under Gate-C/Core policy.
 *
 * Responsibility
 * - Terminal execution owner after a JSON artifact has been lowered to `MirModule`.
 * - Keeps direct-core opt-in and OOB strict rc mapping.
 * - Does not own Program(JSON v0) import-bundle or artifact-family classification.
 */

use super::NyashRunner;
use std::io::Write;

pub fn execute_json_artifact(runner: &NyashRunner, json: &str) -> i32 {
    // Optional: direct Core Dispatcher via child nyash (boxed)
    // Toggle: HAKO_CORE_DIRECT=1 (alias: NYASH_CORE_DIRECT)
    let core_direct = std::env::var("HAKO_CORE_DIRECT").ok().as_deref() == Some("1")
        || std::env::var("NYASH_CORE_DIRECT").ok().as_deref() == Some("1");
    if core_direct {
        // Only attempt Core-Direct when payload already looks like MIR(JSON v0)
        // i.e., has functions/blocks keys. Stage‑B Program(JSON v0) must go through bridge first.
        let looks_like_mir = json.contains("\"functions\"") && json.contains("\"blocks\"");
        if looks_like_mir {
            // In-proc prototype (opt-in): HAKO_CORE_DIRECT_INPROC=1 (alias NYASH_CORE_DIRECT_INPROC)
            let core_direct_inproc = std::env::var("HAKO_CORE_DIRECT_INPROC").ok().as_deref()
                == Some("1")
                || std::env::var("NYASH_CORE_DIRECT_INPROC").ok().as_deref() == Some("1");
            if core_direct_inproc {
                if let Some(rc) = try_run_core_direct_inproc(runner, json) {
                    return rc;
                }
                crate::runtime::get_global_ring0()
                    .log
                    .warn("[core-exec] direct Core (inproc) failed; trying child wrapper");
            }
            if let Some(rc) = try_run_core_direct(json) {
                return rc;
            }
            crate::runtime::get_global_ring0()
                .log
                .warn("[core-exec] direct Core (child) failed; falling back to VM interpreter");
        }
        // else: skip direct Core and continue to bridge/VM path
    }
    match crate::runner::json_artifact::load_json_artifact_to_module(runner, json) {
        Ok(module) => execute_loaded_mir_module(runner, &module),
        Err(error) => {
            eprintln!("❌ {}", error);
            1
        }
    }
}

fn execute_loaded_mir_module(runner: &NyashRunner, module: &crate::mir::MirModule) -> i32 {
    super::json_v0_bridge::maybe_dump_mir(module);
    crate::runner::child_env::pre_run_reset_oob_if_strict();
    let rc = runner.execute_mir_module_quiet_exit(module);
    if crate::config::env::oob_strict_fail() && crate::runtime::observe::oob_seen() {
        crate::runtime::get_global_ring0()
            .log
            .error("[gate-c][oob-strict] Out-of-bounds observed → exit(1)");
        return 1;
    }
    rc
}

fn try_run_core_direct(json: &str) -> Option<i32> {
    // Generate a temporary Hako program that includes the Core dispatcher
    // and calls NyVmDispatcher.run(json), printing the numeric result.
    let tmp_dir = std::path::Path::new("tmp");
    let _ = std::fs::create_dir_all(tmp_dir);
    let script_path = tmp_dir.join("core_exec_direct.hako");
    // Escape JSON into Hako string literal (simple backslash+quote escaping)
    let mut j = String::new();
    for ch in json.chars() {
        match ch {
            '\\' => j.push_str("\\\\"),
            '"' => j.push_str("\\\""),
            _ => j.push(ch),
        }
    }
    let code = format!(
        "include \"lang/src/vm/core/dispatcher.hako\"\nstatic box Main {{ method main(args) {{ local j=\"{}\"; local r=NyVmDispatcher.run(j); return r }} }}\n",
        j
    );
    if let Ok(mut f) = std::fs::File::create(&script_path) {
        let _ = f.write_all(code.as_bytes());
    } else {
        return None;
    }
    // Determine nyash binary (current executable)
    let exe = std::env::current_exe().ok()?;
    let mut cmd = std::process::Command::new(exe);
    crate::runner::child_env::apply_core_wrapper_env(&mut cmd);
    let out = cmd
        .args(["--backend", "vm", script_path.to_string_lossy().as_ref()])
        .output()
        .ok()?;
    if !out.stdout.is_empty() {
        let _ = std::io::stdout().write_all(&out.stdout);
    }
    let rc = out.status.code().unwrap_or(1);
    Some(rc)
}

fn try_run_core_direct_inproc(runner: &NyashRunner, json: &str) -> Option<i32> {
    // Parse direct MIR in-proc and execute via the terminal runner.
    match crate::runner::json_artifact::load_mir_json_to_module(json) {
        Ok(Some(module)) => Some(execute_loaded_mir_module(runner, &module)),
        Ok(None) | Err(_) => None,
    }
}
