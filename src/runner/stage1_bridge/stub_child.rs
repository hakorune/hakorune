/*!
 * Stage-1 bridge stub child command builder.
 *
 * Keeps Stage1 stub entry resolution and child command/env construction outside
 * `mod.rs` so the future-retire bridge root stays focused on route dispatch.
 */

use super::args::Stage1Args;
use super::modules::Stage1ModuleEnvLists;
use crate::cli::CliGroups;
use crate::config::env::stage1;
use std::path::{Path, PathBuf};
use std::process::Command;

// Default embedded child entry stays on the env authority so the stage1 stub
// preserves payload materialization instead of falling back to the raw lane.
const EMBEDDED_STAGE1_ENTRY_SRC: &str =
    include_str!("../../../lang/src/runner/stage1_cli_env.hako");
const EMBEDDED_STAGE1_ENTRY_FILE: &str = "stage1_cli.embedded.hako";

pub(super) struct PreparedStage1StubChild {
    pub(super) command: Command,
    pub(super) trace_summary: String,
}

pub(super) fn prepare(
    groups: &CliGroups,
    args_result: &Stage1Args,
) -> Result<PreparedStage1StubChild, String> {
    let entry = resolve_stage1_entry_path()?;
    let module_env_lists = super::modules::collect_module_env_lists();
    Ok(prepare_with(
        groups,
        args_result,
        entry,
        module_env_lists,
        current_exe_fallback(),
    ))
}

pub(super) fn prepare_or_log(
    groups: &CliGroups,
    args_result: &Stage1Args,
) -> Result<PreparedStage1StubChild, i32> {
    prepare(groups, args_result).map_err(|message| {
        crate::runtime::get_global_ring0().log.error(&message);
        97
    })
}

fn prepare_with(
    groups: &CliGroups,
    args_result: &Stage1Args,
    entry: String,
    module_env_lists: Stage1ModuleEnvLists,
    executable: PathBuf,
) -> PreparedStage1StubChild {
    let entry_fn = std::env::var("NYASH_ENTRY").unwrap_or_else(|_| "Main.main/0".to_string());
    let backend_hint = if stage1::entry_override().is_some() {
        args_result.backend_cli_hint()
    } else {
        None
    };
    let mut cmd = Command::new(executable);
    cmd.arg(&entry).arg("--");
    for arg in &args_result.args {
        cmd.arg(arg);
    }
    set_args_env(&mut cmd, args_result);
    set_source_text_env(&mut cmd, groups, args_result);
    super::env::configure_stage1_env(
        &mut cmd,
        super::env::Stage1ChildEnvConfig {
            entry_path: Some(entry.as_str()),
            entry_fn: &entry_fn,
            backend_hint,
            module_env_lists,
        },
    );
    PreparedStage1StubChild {
        command: cmd,
        trace_summary: format!("{} -- {}", entry, args_result.args.join(" ")),
    }
}

fn set_args_env(cmd: &mut Command, args_result: &Stage1Args) {
    if let Some(json) = args_result.env_script_args.as_ref() {
        cmd.env("NYASH_SCRIPT_ARGS_JSON", json);
    }
    if let Some(src) = args_result.source_env.as_ref() {
        cmd.env("STAGE1_SOURCE", src);
    }
    if let Some(program_json) = args_result.progjson_env.as_ref() {
        cmd.env("STAGE1_PROGRAM_JSON", program_json);
    }
}

fn set_source_text_env(cmd: &mut Command, groups: &CliGroups, args_result: &Stage1Args) {
    if args_result.source_env.is_none() {
        if let Some(src_path) = groups.input.file.as_ref() {
            set_source_text_env_from_path(cmd, src_path);
        }
        return;
    }

    if let Some(src_path) = args_result.source_env.as_ref() {
        set_source_text_env_from_path(cmd, src_path);
    }
}

fn set_source_text_env_from_path(cmd: &mut Command, src_path: &str) {
    if let Ok(text) = std::fs::read_to_string(src_path) {
        cmd.env("STAGE1_SOURCE_TEXT", text);
    }
}

fn current_exe_fallback() -> PathBuf {
    std::env::current_exe().unwrap_or_else(|_| PathBuf::from("target/release/nyash"))
}

fn resolve_stage1_entry_path() -> Result<String, String> {
    if let Some(entry) = stage1::entry_override() {
        if Path::new(&entry).exists() {
            return Ok(entry);
        }
        return Err(format!("[stage1-cli] entry not found: {}", entry));
    }

    let base = std::env::var("NYASH_STAGE1_EMBED_DIR")
        .ok()
        .filter(|value| !value.trim().is_empty())
        .map(PathBuf::from)
        .unwrap_or_else(|| std::env::temp_dir().join("hakorune_stage1_embedded"));

    std::fs::create_dir_all(&base).map_err(|error| {
        format!(
            "[stage1-cli] embedded entry mkdir failed: {} ({})",
            base.display(),
            error
        )
    })?;

    let entry_path = base.join(EMBEDDED_STAGE1_ENTRY_FILE);
    match std::fs::read_to_string(&entry_path) {
        Ok(existing) if existing == EMBEDDED_STAGE1_ENTRY_SRC => {}
        _ => {
            let tmp_path = base.join(format!(
                "{}.tmp-{}",
                EMBEDDED_STAGE1_ENTRY_FILE,
                std::process::id()
            ));
            std::fs::write(&tmp_path, EMBEDDED_STAGE1_ENTRY_SRC).map_err(|error| {
                format!(
                    "[stage1-cli] embedded entry write failed: {} ({})",
                    tmp_path.display(),
                    error
                )
            })?;
            std::fs::rename(&tmp_path, &entry_path)
                .or_else(|_| {
                    let _ = std::fs::remove_file(&entry_path);
                    std::fs::rename(&tmp_path, &entry_path)
                })
                .map_err(|error| {
                    format!(
                        "[stage1-cli] embedded entry install failed: {} ({})",
                        entry_path.display(),
                        error
                    )
                })?;
        }
    }
    Ok(entry_path.to_string_lossy().to_string())
}

#[cfg(test)]
mod tests {
    use super::super::args::{Stage1Args, Stage1ArgsMode};
    use super::{prepare_with, resolve_stage1_entry_path};
    use crate::cli::{
        BackendConfig, CliGroups, DebugConfig, EmitConfig, InputConfig, JitConfig, ParserPipeConfig,
    };
    use crate::runner::stage1_bridge::test_support;
    use std::ffi::OsStr;
    use std::ffi::OsString;
    use tempfile::tempdir;

    struct EnvGuard {
        saved: Vec<(&'static str, Option<String>)>,
    }

    impl EnvGuard {
        fn set_var(key: &'static str, value: &str) -> Self {
            let saved = vec![(key, std::env::var(key).ok())];
            std::env::set_var(key, value);
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

    fn groups_fixture(file: Option<String>) -> CliGroups {
        CliGroups {
            input: InputConfig {
                file,
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
            build: crate::cli::BuildConfig {
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
                emit_wat: None,
                emit_ast_json: None,
                emit_program_json_v0: None,
                hako_emit_program_json: false,
                hako_emit_mir_json: false,
                hako_run: false,
                program_json_to_mir: None,
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

    fn args_fixture(source_env: Option<String>, program_json_env: Option<String>) -> Stage1Args {
        Stage1Args {
            mode: Stage1ArgsMode::EmitMirJson,
            args: vec![
                "emit".to_string(),
                "mir-json".to_string(),
                "fixture.hako".to_string(),
            ],
            env_script_args: Some("[\"emit\",\"mir-json\"]".to_string()),
            source_env,
            progjson_env: program_json_env,
        }
    }

    fn run_args_fixture(backend: &str, source_env: Option<String>) -> Stage1Args {
        Stage1Args {
            mode: Stage1ArgsMode::Run,
            args: vec![
                "run".to_string(),
                "--backend".to_string(),
                backend.to_string(),
                "fixture.hako".to_string(),
            ],
            env_script_args: Some("[\"run\",\"--backend\"]".to_string()),
            source_env,
            progjson_env: None,
        }
    }

    fn env_map(command: &std::process::Command) -> Vec<(OsString, Option<OsString>)> {
        command
            .get_envs()
            .map(|(key, value)| (key.to_os_string(), value.map(OsStr::to_os_string)))
            .collect()
    }

    fn env_value(command: &std::process::Command, key: &str) -> Option<String> {
        env_map(command)
            .into_iter()
            .find(|(env_key, _)| env_key == key)
            .and_then(|(_, value)| value)
            .map(|value| value.to_string_lossy().to_string())
    }

    #[test]
    fn prepare_with_injects_source_text_from_source_env_path() {
        let _lock = test_support::env_lock().lock().expect("env lock");
        let _env = EnvGuard::clear(&["NYASH_ENTRY", "NYASH_STAGE1_INPUT"]);
        let dir = tempdir().expect("tempdir");
        let source_path = dir.path().join("fixture.hako");
        std::fs::write(&source_path, "static box Main { main() { return 0 } }")
            .expect("write source");

        let groups = groups_fixture(None);
        let args = args_fixture(Some(source_path.to_string_lossy().to_string()), None);
        let prepared = prepare_with(
            &groups,
            &args,
            "stage1_entry.hako".to_string(),
            super::super::modules::Stage1ModuleEnvLists::default(),
            std::path::PathBuf::from("target/release/nyash"),
        );

        assert_eq!(
            env_value(&prepared.command, "STAGE1_SOURCE_TEXT"),
            Some("static box Main { main() { return 0 } }".to_string())
        );
        assert_eq!(
            env_value(&prepared.command, "STAGE1_SOURCE"),
            Some(source_path.to_string_lossy().to_string())
        );
    }

    #[test]
    fn prepare_with_prefers_cli_input_file_when_source_env_absent() {
        let _lock = test_support::env_lock().lock().expect("env lock");
        let _env = EnvGuard::clear(&["NYASH_ENTRY", "NYASH_STAGE1_INPUT"]);
        let dir = tempdir().expect("tempdir");
        let source_path = dir.path().join("cli_input.hako");
        std::fs::write(&source_path, "print(0)").expect("write source");

        let groups = groups_fixture(Some(source_path.to_string_lossy().to_string()));
        let args = args_fixture(None, None);
        let prepared = prepare_with(
            &groups,
            &args,
            "stage1_entry.hako".to_string(),
            super::super::modules::Stage1ModuleEnvLists::default(),
            std::path::PathBuf::from("target/release/nyash"),
        );

        assert_eq!(
            env_value(&prepared.command, "STAGE1_SOURCE_TEXT"),
            Some("print(0)".to_string())
        );
        assert_eq!(
            prepared.trace_summary,
            "stage1_entry.hako -- emit mir-json fixture.hako"
        );
    }

    #[test]
    fn prepare_with_preserves_script_args_and_program_json_env() {
        let _lock = test_support::env_lock().lock().expect("env lock");
        let _env = EnvGuard::clear(&["NYASH_ENTRY"]);

        let groups = groups_fixture(None);
        let args = args_fixture(None, Some("payload.program.json".to_string()));
        let prepared = prepare_with(
            &groups,
            &args,
            "stage1_entry.hako".to_string(),
            super::super::modules::Stage1ModuleEnvLists::default(),
            std::path::PathBuf::from("target/release/nyash"),
        );

        assert_eq!(
            env_value(&prepared.command, "NYASH_SCRIPT_ARGS_JSON"),
            Some("[\"emit\",\"mir-json\"]".to_string())
        );
        assert_eq!(
            env_value(&prepared.command, "STAGE1_PROGRAM_JSON"),
            Some("payload.program.json".to_string())
        );
    }

    #[test]
    fn prepare_with_default_entry_does_not_forward_backend_hint() {
        let _lock = test_support::env_lock().lock().expect("env lock");
        let _env = EnvGuard::clear(&[
            "NYASH_ENTRY",
            "STAGE1_CLI_ENTRY",
            "HAKORUNE_STAGE1_ENTRY",
            "NYASH_STAGE1_BACKEND",
            "STAGE1_BACKEND",
        ]);

        let groups = groups_fixture(Some("fixture.hako".to_string()));
        let args = run_args_fixture("vm", Some("fixture.hako".to_string()));
        let prepared = prepare_with(
            &groups,
            &args,
            "stage1_cli.embedded.hako".to_string(),
            super::super::modules::Stage1ModuleEnvLists::default(),
            std::path::PathBuf::from("target/release/nyash"),
        );

        assert_eq!(env_value(&prepared.command, "NYASH_STAGE1_BACKEND"), None);
        assert_eq!(env_value(&prepared.command, "STAGE1_BACKEND"), None);
    }

    #[test]
    fn resolve_stage1_entry_path_prefers_explicit_override_when_present() {
        let _lock = test_support::env_lock().lock().expect("env lock");
        let dir = tempdir().expect("tempdir");
        let entry_path = dir.path().join("stage1_override.hako");
        std::fs::write(&entry_path, "static box Main { main() { return 0 } }")
            .expect("write entry");
        let entry_env = entry_path.to_string_lossy().to_string();
        let _env = EnvGuard::set_var("STAGE1_CLI_ENTRY", &entry_env);

        assert_eq!(
            resolve_stage1_entry_path().expect("entry path"),
            entry_path.to_string_lossy().to_string()
        );
    }
}
