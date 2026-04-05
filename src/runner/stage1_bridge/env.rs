/*!
 * Stage-1 CLI bridge - child environment facade
 *
 * Keeps one child-env entrypoint for the bridge while delegating each policy
 * section to a focused helper module:
 * - runtime defaults
 * - Stage-1 alias propagation
 * - parser / Stage-B toggles
 */

mod parser_stageb;
mod runtime_defaults;
mod stage1_aliases;

use super::modules::Stage1ModuleEnvLists;
use std::process::Command;

pub(super) struct Stage1ChildEnvConfig<'a> {
    pub(super) entry_path: Option<&'a str>,
    pub(super) entry_fn: &'a str,
    pub(super) backend_hint: Option<&'a str>,
    pub(super) module_env_lists: Stage1ModuleEnvLists,
}

/// Configure environment variables for Stage-1 CLI child process.
///
/// Child env policy stays split by responsibility so callers keep a single
/// bridge-local entrypoint without reintroducing mixed mode/backend parsing.
pub(super) fn configure_stage1_env(cmd: &mut Command, config: Stage1ChildEnvConfig<'_>) {
    stage1_aliases::apply(cmd, &config);
    runtime_defaults::apply(cmd);
    parser_stageb::apply(cmd, &config);
}

#[cfg(test)]
mod tests {
    use super::{configure_stage1_env, Stage1ChildEnvConfig};
    use crate::runner::stage1_bridge::modules::Stage1ModuleEnvLists;
    use crate::runner::stage1_bridge::test_support;
    use std::collections::BTreeMap;
    use std::process::Command;

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

    fn command_env_map(cmd: &Command) -> BTreeMap<String, String> {
        cmd.get_envs()
            .filter_map(|(key, value)| {
                Some((
                    key.to_string_lossy().into_owned(),
                    value?.to_string_lossy().into_owned(),
                ))
            })
            .collect()
    }

    fn configure_fixture(config: Stage1ChildEnvConfig<'_>) -> BTreeMap<String, String> {
        let mut cmd = Command::new("true");
        configure_stage1_env(&mut cmd, config);
        command_env_map(&cmd)
    }

    #[test]
    fn configure_stage1_env_keeps_parent_backend_aliases_out_of_child_env() {
        let _lock = test_support::env_lock().lock().unwrap();
        let _clear = EnvGuard::clear(&[
            "HAKO_STAGE1_MODE",
            "NYASH_STAGE1_MODE",
            "HAKO_EMIT_PROGRAM_JSON",
            "STAGE1_EMIT_PROGRAM_JSON",
            "HAKO_STAGE1_INPUT",
            "NYASH_STAGE1_INPUT",
            "STAGE1_SOURCE",
            "STAGE1_INPUT",
            "HAKO_STAGE1_PROGRAM_JSON",
            "NYASH_STAGE1_PROGRAM_JSON",
            "STAGE1_PROGRAM_JSON",
            "HAKO_STAGE1_BACKEND",
            "NYASH_STAGE1_BACKEND",
            "NYASH_ENTRY",
        ]);
        let _set = EnvGuard::set(&[
            ("STAGE1_EMIT_PROGRAM_JSON", "1"),
            ("STAGE1_SOURCE", "fixtures/demo.hako"),
            ("STAGE1_PROGRAM_JSON", "target/demo.program.json"),
            ("STAGE1_BACKEND", "llvm"),
        ]);

        let envs = configure_fixture(Stage1ChildEnvConfig {
            entry_path: Some("lang/src/runner/stage1_cli_env.hako"),
            entry_fn: "Main.main/0",
            backend_hint: None,
            module_env_lists: Stage1ModuleEnvLists::default(),
        });

        assert_eq!(envs.get("NYASH_STAGE1_CLI_CHILD"), Some(&"1".to_string()));
        assert_eq!(
            envs.get("NYASH_STAGE1_MODE"),
            Some(&"emit-program".to_string())
        );
        assert_eq!(
            envs.get("NYASH_STAGE1_INPUT"),
            Some(&"fixtures/demo.hako".to_string())
        );
        assert_eq!(
            envs.get("NYASH_STAGE1_PROGRAM_JSON"),
            Some(&"target/demo.program.json".to_string())
        );
        assert!(!envs.contains_key("NYASH_STAGE1_BACKEND"));
        assert!(!envs.contains_key("STAGE1_BACKEND"));
        assert_eq!(
            envs.get("STAGE1_CLI_ENTRY"),
            Some(&"lang/src/runner/stage1_cli_env.hako".to_string())
        );
        assert_eq!(
            envs.get("HAKORUNE_STAGE1_ENTRY"),
            Some(&"lang/src/runner/stage1_cli_env.hako".to_string())
        );
    }

    #[test]
    fn configure_stage1_env_sets_runtime_and_stageb_defaults() {
        let _lock = test_support::env_lock().lock().unwrap();
        let _clear = EnvGuard::clear(&[
            "NYASH_NYRT_SILENT_RESULT",
            "NYASH_DISABLE_PLUGINS",
            "NYASH_FILEBOX_MODE",
            "NYASH_BOX_FACTORY_POLICY",
            "HAKO_MIR_BUILDER_METHODIZE",
            "NYASH_MIR_UNIFIED_CALL",
            "HAKO_STAGEB_APPLY_USINGS",
            "NYASH_ENABLE_USING",
            "HAKO_ENABLE_USING",
            "NYASH_FEATURES",
            "NYASH_PARSER_STAGE3",
            "HAKO_PARSER_STAGE3",
            "HAKO_STAGEB_MODULES_LIST",
            "HAKO_STAGEB_MODULE_ROOTS_LIST",
            "STAGE1_CLI_ENTRY",
            "HAKORUNE_STAGE1_ENTRY",
            "NYASH_ENTRY",
            "STAGE1_BACKEND",
            "HAKO_STAGE1_BACKEND",
            "NYASH_STAGE1_BACKEND",
        ]);

        let envs = configure_fixture(Stage1ChildEnvConfig {
            entry_path: Some("lang/src/runner/stage1_cli_env.hako"),
            entry_fn: "Main.main/0",
            backend_hint: Some("vm"),
            module_env_lists: Stage1ModuleEnvLists {
                modules_list: Some("core=lang/core".into()),
                module_roots_list: Some("core=lang".into()),
            },
        });

        assert_eq!(envs.get("NYASH_NYRT_SILENT_RESULT"), Some(&"1".to_string()));
        assert_eq!(envs.get("NYASH_DISABLE_PLUGINS"), Some(&"0".to_string()));
        assert_eq!(envs.get("NYASH_FILEBOX_MODE"), Some(&"auto".to_string()));
        assert_eq!(
            envs.get("NYASH_BOX_FACTORY_POLICY"),
            Some(&"builtin_first".to_string())
        );
        assert_eq!(
            envs.get("HAKO_MIR_BUILDER_METHODIZE"),
            Some(&"1".to_string())
        );
        assert_eq!(
            envs.get("NYASH_MIR_UNIFIED_CALL"),
            Some(&"1".to_string())
        );
        assert_eq!(
            envs.get("HAKO_SELFHOST_NO_DELEGATE"),
            Some(&"1".to_string())
        );
        assert_eq!(
            envs.get("HAKO_MIR_BUILDER_DELEGATE"),
            Some(&"0".to_string())
        );
        assert_eq!(envs.get("HAKO_STAGEB_APPLY_USINGS"), Some(&"0".to_string()));
        assert_eq!(envs.get("NYASH_ENABLE_USING"), Some(&"1".to_string()));
        assert_eq!(envs.get("HAKO_ENABLE_USING"), Some(&"1".to_string()));
        assert_eq!(envs.get("NYASH_FEATURES"), Some(&"stage3".to_string()));
        assert_eq!(
            envs.get("HAKO_STAGEB_MODULES_LIST"),
            Some(&"core=lang/core".to_string())
        );
        assert_eq!(
            envs.get("HAKO_STAGEB_MODULE_ROOTS_LIST"),
            Some(&"core=lang".to_string())
        );
        assert_eq!(envs.get("NYASH_ENTRY"), Some(&"Main.main/0".to_string()));
        assert_eq!(
            envs.get("STAGE1_CLI_ENTRY"),
            Some(&"lang/src/runner/stage1_cli_env.hako".to_string())
        );
        assert_eq!(
            envs.get("HAKORUNE_STAGE1_ENTRY"),
            Some(&"lang/src/runner/stage1_cli_env.hako".to_string())
        );
        assert_eq!(envs.get("NYASH_STAGE1_BACKEND"), Some(&"vm".to_string()));
        assert_eq!(envs.get("STAGE1_BACKEND"), Some(&"vm".to_string()));
    }

    #[test]
    fn configure_stage1_env_preserves_parent_overrides_and_merges_stage3() {
        let _lock = test_support::env_lock().lock().unwrap();
        let _clear = EnvGuard::clear(&[
            "NYASH_NYRT_SILENT_RESULT",
            "HAKO_MIR_BUILDER_METHODIZE",
            "NYASH_MIR_UNIFIED_CALL",
            "HAKO_STAGEB_APPLY_USINGS",
            "NYASH_ENABLE_USING",
            "HAKO_ENABLE_USING",
            "NYASH_FEATURES",
            "NYASH_ENTRY",
            "STAGE1_CLI_ENTRY",
            "HAKORUNE_STAGE1_ENTRY",
        ]);
        let _set = EnvGuard::set(&[
            ("NYASH_NYRT_SILENT_RESULT", "0"),
            ("HAKO_MIR_BUILDER_METHODIZE", "0"),
            ("NYASH_MIR_UNIFIED_CALL", "0"),
            ("HAKO_STAGEB_APPLY_USINGS", "1"),
            ("NYASH_ENABLE_USING", "0"),
            ("HAKO_ENABLE_USING", "0"),
            ("NYASH_FEATURES", "macro"),
            ("NYASH_ENTRY", "Custom.main/0"),
        ]);

        let envs = configure_fixture(Stage1ChildEnvConfig {
            entry_path: Some("lang/src/runner/stage1_cli_env.hako"),
            entry_fn: "Main.main/0",
            backend_hint: None,
            module_env_lists: Stage1ModuleEnvLists::default(),
        });

        assert!(!envs.contains_key("NYASH_NYRT_SILENT_RESULT"));
        assert_eq!(
            envs.get("HAKO_MIR_BUILDER_METHODIZE"),
            Some(&"1".to_string())
        );
        assert_eq!(
            envs.get("NYASH_MIR_UNIFIED_CALL"),
            Some(&"1".to_string())
        );
        assert!(!envs.contains_key("HAKO_STAGEB_APPLY_USINGS"));
        assert!(!envs.contains_key("NYASH_ENABLE_USING"));
        assert!(!envs.contains_key("HAKO_ENABLE_USING"));
        assert!(!envs.contains_key("NYASH_ENTRY"));
        assert_eq!(
            envs.get("STAGE1_CLI_ENTRY"),
            Some(&"lang/src/runner/stage1_cli_env.hako".to_string())
        );
        assert_eq!(
            envs.get("HAKORUNE_STAGE1_ENTRY"),
            Some(&"lang/src/runner/stage1_cli_env.hako".to_string())
        );
        assert_eq!(
            envs.get("NYASH_FEATURES"),
            Some(&"macro,stage3".to_string())
        );
    }
}
