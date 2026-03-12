/*!
 * Stage-1 bridge emit output-path helper.
 *
 * Centralizes bridge-local output-path resolution for stub emit and binary-only
 * direct emit routes.
 */

pub(super) fn resolve_mir_out_path(cli_path: Option<String>) -> Option<String> {
    cli_path
        .or_else(|| std::env::var("NYASH_STAGE1_EMIT_MIR_OUT").ok())
        .or_else(|| std::env::var("HAKO_STAGE1_EMIT_MIR_OUT").ok())
}

pub(super) fn resolve_program_json_out_path(cli_path: Option<String>) -> Option<String> {
    cli_path
        .or_else(|| std::env::var("NYASH_STAGE1_EMIT_PROGRAM_OUT").ok())
        .or_else(|| std::env::var("HAKO_STAGE1_EMIT_PROGRAM_OUT").ok())
}

#[cfg(test)]
mod tests {
    use super::{resolve_mir_out_path, resolve_program_json_out_path};
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

    #[test]
    fn resolve_mir_out_path_prefers_cli_over_env() {
        let _lock = test_support::env_lock().lock().expect("env lock");
        let _env = EnvGuard::set(&[
            ("NYASH_STAGE1_EMIT_MIR_OUT", "env-primary.json"),
            ("HAKO_STAGE1_EMIT_MIR_OUT", "env-legacy.json"),
        ]);

        assert_eq!(
            resolve_mir_out_path(Some("cli.json".to_string())),
            Some("cli.json".to_string())
        );
    }

    #[test]
    fn resolve_mir_out_path_falls_back_to_legacy_env_alias() {
        let _lock = test_support::env_lock().lock().expect("env lock");
        let _env = EnvGuard::set(&[("HAKO_STAGE1_EMIT_MIR_OUT", "legacy.json")]);

        assert_eq!(resolve_mir_out_path(None), Some("legacy.json".to_string()));
    }

    #[test]
    fn resolve_program_json_out_path_falls_back_to_legacy_env_alias() {
        let _lock = test_support::env_lock().lock().expect("env lock");
        let _env = EnvGuard::set(&[("HAKO_STAGE1_EMIT_PROGRAM_OUT", "legacy-program.json")]);

        assert_eq!(
            resolve_program_json_out_path(None),
            Some("legacy-program.json".to_string())
        );
    }
}
