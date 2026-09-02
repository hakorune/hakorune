/*!
 * CoreExecutor — loaded `MirModule` execution under Gate-C/Core policy.
 *
 * Responsibility
 * - Terminal execution owner after a JSON artifact has been lowered to `MirModule`.
 * - Narrow direct-MIR owner for already-materialized `MIR(JSON)` payloads.
 * - Keeps one direct-core grace tombstone and OOB strict rc mapping.
 * - Does not own Program(JSON v0) import-bundle or artifact-family classification.
 */

use super::NyashRunner;

// Artifact-family convergence entry.
// Classification stays here; callers that already know they hold MIR(JSON) should use
// `execute_mir_json_text(...)` or `execute_loaded_mir_module(...)`.
pub fn execute_json_artifact(runner: &NyashRunner, json: &str) -> i32 {
    match crate::runner::json_artifact::load_json_artifact_to_module(runner, json) {
        Ok(module) => {
            if core_direct_requested() {
                return core_direct_retired();
            }
            execute_loaded_mir_module(runner, &module)
        }
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
    let module = crate::runner::json_artifact::parse_direct_mir_json_text_with_v0_fallback(
        json,
        source_label,
    )?;
    if core_direct_requested() {
        return Ok(core_direct_retired());
    }
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

fn core_direct_requested() -> bool {
    // Grace-period tombstone intent. Both historical spellings share one terminal;
    // no route selection or execution is performed from this flag.
    crate::config::env::env_bool("HAKO_CORE_DIRECT")
        || crate::config::env::env_bool("NYASH_CORE_DIRECT")
}

fn core_direct_retired() -> i32 {
    eprintln!("[core-direct/retired]");
    1
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
        EnvGuard::set(&[("HAKO_CORE_DIRECT", "0"), ("NYASH_CORE_DIRECT", "0")])
    }

    fn core_direct_env_on() -> EnvGuard {
        EnvGuard::set(&[("HAKO_CORE_DIRECT", "1"), ("NYASH_CORE_DIRECT", "0")])
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

    #[cfg(feature = "vm-reference")]
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

    #[test]
    fn execute_mir_json_text_core_direct_is_one_state_post_decode_terminal() {
        let _lock = env_lock().lock().expect("env lock poisoned");
        let _env = core_direct_env_on();
        let runner = NyashRunner::new(crate::cli::CliConfig::default());

        let rc = execute_mir_json_text(&runner, direct_mir_fixture(), "<inline-mir>")
            .expect("valid direct MIR should reach the retired terminal");

        assert_eq!(rc, 1, "CoreDirect tombstone must return stable rc=1");
    }

    #[test]
    fn execute_mir_json_text_core_direct_does_not_relabel_wrong_entrance() {
        let _lock = env_lock().lock().expect("env lock poisoned");
        let _env = core_direct_env_on();
        let runner = NyashRunner::new(crate::cli::CliConfig::default());

        let err = execute_mir_json_text(&runner, program_json_fixture(), "<inline-program>")
            .expect_err("Program(JSON) must remain a wrong-entrance error");

        assert!(
            !err.contains("core-direct/retired"),
            "wrong entrance must not be relabeled as CoreDirect terminal: {}",
            err
        );
    }
}
