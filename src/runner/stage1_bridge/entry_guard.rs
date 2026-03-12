/*!
 * Stage-1 bridge entry guard helper.
 *
 * Owns child-recursion guard, top-level enablement guard, and bridge-entry
 * trace/debug logging so `mod.rs` stays focused on dispatch.
 */

use crate::config;
use crate::config::env::stage1;

pub(super) fn should_engage() -> bool {
    if config::env::cli_verbose_level() == 2 {
        let ring0 = crate::runtime::get_global_ring0();
        ring0.log.debug(&format!(
            "[stage1-bridge/trace] maybe_run_stage1_cli_stub invoked"
        ));
    }

    if stage1::child_invocation() {
        if config::env::cli_verbose_level() == 2 {
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.debug(&format!(
                "[stage1-bridge/trace] skip: NYASH_STAGE1_CLI_CHILD=1"
            ));
        }
        return false;
    }

    if !stage1::enabled() {
        if config::env::cli_verbose_level() == 2 {
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.debug(&format!(
                "[stage1-bridge/trace] skip: NYASH_USE_STAGE1_CLI!=1"
            ));
        }
        return false;
    }

    if config::env::cli_verbose() || config::env::cli_verbose_level() == 2 {
        let ring0 = crate::runtime::get_global_ring0();
        ring0.log.debug(&format!(
            "[stage1-bridge/debug] NYASH_USE_STAGE1_CLI=1 detected"
        ));
    }

    true
}

#[cfg(test)]
mod tests {
    use super::should_engage;
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

    #[test]
    fn should_engage_returns_false_for_child_invocation() {
        let _lock = test_support::env_lock().lock().expect("env lock");
        let _env = EnvGuard::set(&[
            ("NYASH_USE_STAGE1_CLI", "1"),
            ("NYASH_STAGE1_CLI_CHILD", "1"),
        ]);

        assert!(!should_engage());
    }

    #[test]
    fn should_engage_returns_false_when_bridge_disabled() {
        let _lock = test_support::env_lock().lock().expect("env lock");
        let _env = EnvGuard::clear(&[
            "NYASH_USE_STAGE1_CLI",
            "HAKO_STAGE1_ENABLE",
            "HAKO_EMIT_PROGRAM_JSON",
            "HAKO_EMIT_MIR_JSON",
            "NYASH_STAGE1_CLI_CHILD",
        ]);

        assert!(!should_engage());
    }

    #[test]
    fn should_engage_returns_true_for_enabled_non_child_lane() {
        let _lock = test_support::env_lock().lock().expect("env lock");
        let _env = EnvGuard::set(&[("NYASH_USE_STAGE1_CLI", "1")]);
        std::env::remove_var("NYASH_STAGE1_CLI_CHILD");

        assert!(should_engage());
    }
}
