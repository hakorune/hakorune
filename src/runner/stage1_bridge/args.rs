/*!
 * Stage-1 CLI bridge - args builder
 *
 * Constructs stage1_args based on execution mode (emit_program / emit_mir / run).
 * The resulting raw/subcmd lane is a non-authority future-retire target;
 * current green selfhost authority lives in stage1_cli_env.hako.
 */

use crate::cli::CliGroups;
use crate::config::env::stage1;
use serde_json;
use std::process;

/// Stage-1 args construction result
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Stage1ArgsMode {
    EmitProgramJsonV0,
    EmitMirJson,
    Run,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Stage1StubEmitMode {
    MirJson,
    ProgramJsonV0,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Stage1StubExecPlan {
    EmitCapture(Stage1StubEmitMode),
    DelegateStatus,
}

/// Stage-1 args construction result
#[derive(Debug)]
pub(super) struct Stage1Args {
    pub mode: Stage1ArgsMode,
    pub args: Vec<String>,
    pub env_script_args: Option<String>,
    pub source_env: Option<String>,
    pub progjson_env: Option<String>,
}

impl Stage1Args {
    pub(super) fn backend_cli_hint(&self) -> Option<&str> {
        if self.mode != Stage1ArgsMode::Run {
            return None;
        }
        self.args
            .windows(2)
            .find(|window| window[0] == "--backend")
            .map(|window| window[1].as_str())
    }

    pub(super) fn stub_exec_plan(&self) -> Stage1StubExecPlan {
        match self.mode {
            Stage1ArgsMode::EmitMirJson => {
                Stage1StubExecPlan::EmitCapture(Stage1StubEmitMode::MirJson)
            }
            Stage1ArgsMode::EmitProgramJsonV0 => {
                Stage1StubExecPlan::EmitCapture(Stage1StubEmitMode::ProgramJsonV0)
            }
            Stage1ArgsMode::Run => Stage1StubExecPlan::DelegateStatus,
        }
    }
}

/// Build stage1_args based on execution mode
///
/// # Modes
/// - emit_program: emit program-json <source.hako>
/// - emit_mir: emit mir-json (<source.hako> or STAGE1_PROGRAM_JSON)
/// - run: run --backend <backend> <source.hako>
pub(super) fn build_stage1_args(groups: &CliGroups) -> Stage1Args {
    // Prefer new env (NYASH_STAGE1_*) and fall back to legacy names to keep compatibility.
    let source = stage1::input_path().or_else(|| groups.input.file.as_ref().cloned());

    let emit_program = stage1::emit_program_json();
    let emit_mir = stage1::emit_mir_json();

    let mut args: Vec<String> = Vec::new();
    let mut source_env: Option<String> = None;
    let mut progjson_env: Option<String> = None;

    let mode = if emit_program {
        let src = source.as_ref().cloned().unwrap_or_else(|| {
            let ring0 = crate::runtime::ring0::get_global_ring0();
            ring0
                .log
                .error("[stage1-cli] STAGE1_EMIT_PROGRAM_JSON=1 but no input file provided");
            process::exit(97);
        });
        args.push("emit".into());
        args.push("program-json".into());
        args.push(src);
        source_env = args.last().cloned();
        Stage1ArgsMode::EmitProgramJsonV0
    } else if emit_mir {
        if let Some(pjson) = stage1::program_json_path() {
            args.push("emit".into());
            args.push("mir-json".into());
            args.push("--from-program-json".into());
            args.push(pjson);
            progjson_env = args.last().cloned();
        } else {
            let src = source.as_ref().cloned().unwrap_or_else(|| {
                let ring0 = crate::runtime::ring0::get_global_ring0();
                ring0
                    .log
                    .error("[stage1-cli] STAGE1_EMIT_MIR_JSON=1 but no input file provided");
                process::exit(97);
            });
            args.push("emit".into());
            args.push("mir-json".into());
            args.push(src);
            source_env = args.last().cloned();
        }
        Stage1ArgsMode::EmitMirJson
    } else {
        let src = source.as_ref().cloned().unwrap_or_else(|| {
            let ring0 = crate::runtime::ring0::get_global_ring0();
            ring0
                .log
                .error("[stage1-cli] NYASH_USE_STAGE1_CLI=1 requires an input file to run");
            process::exit(97);
        });
        args.push("run".into());
        let backend = stage1::backend_hint().unwrap_or_else(|| groups.backend.backend.clone());
        args.push("--backend".into());
        args.push(backend);
        args.push(src);
        source_env = args.last().cloned();
        Stage1ArgsMode::Run
    };

    // Forward script args provided to the parent process (via -- arg1 arg2 ...)
    if let Ok(json) = std::env::var("NYASH_SCRIPT_ARGS_JSON") {
        if let Ok(mut extras) = serde_json::from_str::<Vec<String>>(&json) {
            args.append(&mut extras);
        }
    }

    // Also pass args via env to guarantee argv is well-defined in the stub.
    let env_script_args = if std::env::var("NYASH_SCRIPT_ARGS_JSON").is_err() {
        serde_json::to_string(&args).ok()
    } else {
        None
    };

    Stage1Args {
        mode,
        args,
        env_script_args,
        source_env,
        progjson_env,
    }
}

#[cfg(test)]
mod tests {
    use super::{
        build_stage1_args, Stage1Args, Stage1ArgsMode, Stage1StubEmitMode, Stage1StubExecPlan,
    };
    use crate::cli::{
        BackendConfig, BuildConfig, CliGroups, DebugConfig, EmitConfig, InputConfig, JitConfig,
        ParserPipeConfig,
    };
    use crate::runner::stage1_bridge::test_support;

    struct EnvGuard {
        saved: Vec<(&'static str, Option<String>)>,
    }

    impl EnvGuard {
        fn set(vars: &[(&'static str, &'static str)]) -> Self {
            let mut saved = Vec::with_capacity(vars.len());
            for (key, value) in vars {
                saved.push((*key, std::env::var(key).ok()));
                std::env::set_var(key, value);
            }
            Self { saved }
        }

        fn clear(keys: &[&'static str]) -> Self {
            let mut saved = Vec::with_capacity(keys.len());
            for key in keys {
                saved.push((*key, std::env::var(key).ok()));
                std::env::remove_var(key);
            }
            Self { saved }
        }
    }

    impl Drop for EnvGuard {
        fn drop(&mut self) {
            for (key, old_value) in self.saved.drain(..) {
                if let Some(value) = old_value {
                    std::env::set_var(key, value);
                } else {
                    std::env::remove_var(key);
                }
            }
        }
    }

    fn groups_fixture(file: Option<&str>) -> CliGroups {
        CliGroups {
            input: InputConfig {
                file: file.map(str::to_string),
                cli_usings: vec![],
            },
            debug: DebugConfig {
                debug_fuel: None,
                dump_ast: false,
                dump_mir: false,
                verify_mir: false,
                mir_verbose: false,
                mir_verbose_effects: false,
                cli_verbose: false,
            },
            backend: BackendConfig {
                backend: "vm".to_string(),
                vm_stats: false,
                vm_stats_json: false,
                jit: JitConfig {
                    exec: false,
                    stats: false,
                    stats_json: false,
                    dump: false,
                    events: false,
                    events_compile: false,
                    events_runtime: false,
                    events_path: None,
                    threshold: None,
                    phi_min: false,
                    hostcall: false,
                    handle_debug: false,
                    native_f64: false,
                    native_bool: false,
                    only: false,
                    direct: false,
                },
            },
            build: BuildConfig {
                path: None,
                app: None,
                out: None,
                aot: None,
                profile: None,
                target: None,
            },
            emit: EmitConfig {
                emit_cfg: None,
                emit_mir_json: None,
                emit_mir_json_minimal: None,
                emit_wat: None,
                emit_ast_json: None,
                emit_program_json_v0: None,
                hako_emit_mir_json: false,
                hako_run: false,
                emit_exe: None,
                emit_exe_nyrt: None,
                emit_exe_libs: None,
            },
            parser: ParserPipeConfig {
                ny_parser_pipe: false,
                json_file: None,
                mir_json_file: None,
            },
            gc_mode: None,
            compile_wasm: false,
            compile_native: false,
            output_file: None,
            benchmark: false,
            iterations: 1,
            run_task: None,
            load_ny_plugins: false,
        }
    }

    fn args_fixture(mode: Stage1ArgsMode) -> Stage1Args {
        Stage1Args {
            mode,
            args: vec![],
            env_script_args: None,
            source_env: None,
            progjson_env: None,
        }
    }

    #[test]
    fn build_stage1_args_marks_emit_program_mode() {
        let _lock = test_support::env_lock().lock().expect("env lock");
        let _env = EnvGuard::set(&[("STAGE1_EMIT_PROGRAM_JSON", "1")]);
        let groups = groups_fixture(Some("fixture.hako"));

        let args = build_stage1_args(&groups);

        assert_eq!(args.mode, Stage1ArgsMode::EmitProgramJsonV0);
        assert_eq!(args.args, vec!["emit", "program-json", "fixture.hako"]);
    }

    #[test]
    fn build_stage1_args_marks_emit_mir_mode_from_program_json_input() {
        let _lock = test_support::env_lock().lock().expect("env lock");
        let _env = EnvGuard::set(&[
            ("STAGE1_EMIT_MIR_JSON", "1"),
            ("STAGE1_PROGRAM_JSON", "fixture.program.json"),
        ]);
        let groups = groups_fixture(None);

        let args = build_stage1_args(&groups);

        assert_eq!(args.mode, Stage1ArgsMode::EmitMirJson);
        assert_eq!(
            args.args,
            vec![
                "emit",
                "mir-json",
                "--from-program-json",
                "fixture.program.json"
            ]
        );
        assert_eq!(args.progjson_env.as_deref(), Some("fixture.program.json"));
    }

    #[test]
    fn build_stage1_args_marks_run_mode_by_default() {
        let _lock = test_support::env_lock().lock().expect("env lock");
        let _env = EnvGuard::clear(&[
            "STAGE1_EMIT_PROGRAM_JSON",
            "STAGE1_EMIT_MIR_JSON",
            "NYASH_STAGE1_MODE",
            "HAKO_STAGE1_MODE",
        ]);
        let groups = groups_fixture(Some("fixture.hako"));

        let args = build_stage1_args(&groups);

        assert_eq!(args.mode, Stage1ArgsMode::Run);
        assert_eq!(args.args, vec!["run", "--backend", "vm", "fixture.hako"]);
        assert_eq!(args.backend_cli_hint(), Some("vm"));
    }

    #[test]
    fn backend_cli_hint_is_none_for_non_run_modes() {
        let _lock = test_support::env_lock().lock().expect("env lock");
        let _env = EnvGuard::set(&[("STAGE1_EMIT_PROGRAM_JSON", "1")]);
        let groups = groups_fixture(Some("fixture.hako"));

        let args = build_stage1_args(&groups);

        assert_eq!(args.mode, Stage1ArgsMode::EmitProgramJsonV0);
        assert_eq!(args.backend_cli_hint(), None);
    }

    #[test]
    fn stub_exec_plan_uses_capture_for_emit_mir_mode() {
        assert_eq!(
            args_fixture(Stage1ArgsMode::EmitMirJson).stub_exec_plan(),
            Stage1StubExecPlan::EmitCapture(Stage1StubEmitMode::MirJson)
        );
    }

    #[test]
    fn stub_exec_plan_uses_capture_for_emit_program_mode() {
        assert_eq!(
            args_fixture(Stage1ArgsMode::EmitProgramJsonV0).stub_exec_plan(),
            Stage1StubExecPlan::EmitCapture(Stage1StubEmitMode::ProgramJsonV0)
        );
    }

    #[test]
    fn stub_exec_plan_uses_delegate_status_for_run_mode() {
        assert_eq!(
            args_fixture(Stage1ArgsMode::Run).stub_exec_plan(),
            Stage1StubExecPlan::DelegateStatus
        );
    }
}
