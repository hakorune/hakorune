/*!
 * CoreExecutor — loaded `MirModule` execution under Gate-C/Core policy.
 *
 * Responsibility
 * - Terminal execution owner after a JSON artifact has been lowered to `MirModule`.
 * - Narrow direct-MIR owner for already-materialized `MIR(JSON)` payloads.
 * - Keeps direct-core opt-in and OOB strict rc mapping.
 * - Does not own Program(JSON v0) import-bundle or artifact-family classification.
 */

use super::NyashRunner;
use std::io::Write;

// Artifact-family convergence entry.
// Classification stays here; callers that already know they hold MIR(JSON) should use
// `execute_mir_json_text(...)` or `execute_loaded_mir_module(...)`.
pub fn execute_json_artifact(runner: &NyashRunner, json: &str) -> i32 {
    if let Some(rc) = maybe_try_core_direct_for_mir_json(runner, json) {
        return rc;
    }
    match crate::runner::json_artifact::load_json_artifact_to_module(runner, json) {
        Ok(module) => execute_loaded_mir_module(runner, &module),
        Err(error) => {
            eprintln!("❌ {}", error);
            1
        }
    }
}

// Direct MIR(JSON) handoff for already-materialized MIR text.
// Keep this free from Program(JSON) fallback ownership and artifact-family classification.
pub(crate) fn execute_mir_json_text(
    runner: &NyashRunner,
    json: &str,
    source_label: &str,
) -> Result<i32, String> {
    if let Some(rc) = maybe_try_core_direct_for_mir_json(runner, json) {
        return Ok(rc);
    }
    let module = crate::runner::json_artifact::parse_direct_mir_json_text_with_v0_fallback(
        json,
        source_label,
    )?;
    Ok(execute_loaded_mir_module(runner, &module))
}

// Terminal in-proc execution owner after JSON/compat lowering is already done.
pub(crate) fn execute_loaded_mir_module(
    runner: &NyashRunner,
    module: &crate::mir::MirModule,
) -> i32 {
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

fn maybe_try_core_direct_for_mir_json(runner: &NyashRunner, json: &str) -> Option<i32> {
    // Optional direct-core execution probe.
    // This remains an internal opt-in under the direct MIR owner; it does not create a new
    // external route owner for Program(JSON) or shell compat lanes.
    // Toggle: HAKO_CORE_DIRECT=1 (alias: NYASH_CORE_DIRECT)
    let core_direct = std::env::var("HAKO_CORE_DIRECT").ok().as_deref() == Some("1")
        || std::env::var("NYASH_CORE_DIRECT").ok().as_deref() == Some("1");
    if !core_direct || !looks_like_mir_json_text(json) {
        return None;
    }

    // In-proc prototype (opt-in): HAKO_CORE_DIRECT_INPROC=1 (alias NYASH_CORE_DIRECT_INPROC)
    let core_direct_inproc = std::env::var("HAKO_CORE_DIRECT_INPROC").ok().as_deref() == Some("1")
        || std::env::var("NYASH_CORE_DIRECT_INPROC").ok().as_deref() == Some("1");
    if core_direct_inproc {
        if let Some(rc) = try_run_core_direct_inproc(runner, json) {
            return Some(rc);
        }
        crate::runtime::get_global_ring0()
            .log
            .warn("[core-exec] direct Core (inproc) failed; trying child wrapper");
    }
    if let Some(rc) = try_run_core_direct(json) {
        return Some(rc);
    }
    crate::runtime::get_global_ring0()
        .log
        .warn("[core-exec] direct Core (child) failed; falling back to VM interpreter");
    None
}

fn looks_like_mir_json_text(json: &str) -> bool {
    json.contains("\"functions\"") && json.contains("\"blocks\"")
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

#[cfg(test)]
mod tests {
    use super::execute_mir_json_text;
    use super::NyashRunner;
    use std::sync::{Mutex, OnceLock};

    struct EnvGuard {
        saved: Vec<(&'static str, Option<String>)>,
    }

    impl EnvGuard {
        fn set(vars: &[(&'static str, &'static str)]) -> Self {
            let mut saved = Vec::with_capacity(vars.len());
            for (k, v) in vars {
                saved.push((*k, std::env::var(k).ok()));
                std::env::set_var(k, v);
            }
            Self { saved }
        }
    }

    impl Drop for EnvGuard {
        fn drop(&mut self) {
            for (k, old) in self.saved.drain(..) {
                if let Some(v) = old {
                    std::env::set_var(k, v);
                } else {
                    std::env::remove_var(k);
                }
            }
        }
    }

    fn env_lock() -> &'static Mutex<()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(()))
    }

    fn core_direct_env_off() -> EnvGuard {
        EnvGuard::set(&[
            ("HAKO_CORE_DIRECT", "0"),
            ("NYASH_CORE_DIRECT", "0"),
            ("HAKO_CORE_DIRECT_INPROC", "0"),
            ("NYASH_CORE_DIRECT_INPROC", "0"),
        ])
    }

    fn direct_mir_fixture() -> &'static str {
        r#"{
            "kind":"MIR",
            "schema_version":"1.0",
            "functions":[
                {
                    "name":"main",
                    "blocks":[
                        {
                            "id":0,
                            "instructions":[
                                {"op":"const","dst":1,"value":{"type":"i64","value":42}},
                                {"op":"ret","value":1}
                            ]
                        }
                    ]
                }
            ]
        }"#
    }

    fn program_json_fixture() -> &'static str {
        r#"{
            "version":0,
            "kind":"Program",
            "body":[
                {"type":"Return","expr":{"type":"Int","value":42}}
            ]
        }"#
    }

    #[test]
    fn execute_mir_json_text_accepts_direct_mir_fixture() {
        let _lock = env_lock().lock().expect("env lock poisoned");
        let _env = core_direct_env_off();
        let runner = NyashRunner::new(crate::cli::CliConfig::default());

        let rc = execute_mir_json_text(&runner, direct_mir_fixture(), "<inline-mir>")
            .expect("direct MIR(JSON) should execute");

        assert_eq!(rc, 42, "direct MIR handoff must preserve terminal rc");
    }

    #[test]
    fn execute_mir_json_text_rejects_program_json_direct_input() {
        let _lock = env_lock().lock().expect("env lock poisoned");
        let _env = core_direct_env_off();
        let runner = NyashRunner::new(crate::cli::CliConfig::default());

        let err = execute_mir_json_text(&runner, program_json_fixture(), "<inline-program>")
            .expect_err("Program(JSON) must not be accepted on direct MIR handoff");

        assert!(
            err.contains("unsupported shape (<inline-program>)"),
            "unexpected direct handoff error: {}",
            err
        );
    }
}
