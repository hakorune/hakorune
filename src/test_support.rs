//! Shared test support helpers available to integration tests and unit tests.

use std::{
    collections::BTreeSet,
    ffi::OsString,
    sync::{Mutex, MutexGuard},
};

static PROCESS_STATE_LOCK: Mutex<()> = Mutex::new(());

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProcessStateSnapshot {
    values: Vec<(&'static str, Option<OsString>)>,
}

impl ProcessStateSnapshot {
    pub fn capture(keys: impl IntoIterator<Item = &'static str>) -> Self {
        let values = keys
            .into_iter()
            .collect::<BTreeSet<_>>()
            .into_iter()
            .map(|key| (key, std::env::var_os(key)))
            .collect();
        Self { values }
    }

    pub fn drift_from(&self, expected: &Self) -> ProcessStateDrift {
        let current = self
            .values
            .iter()
            .map(|(key, value)| (*key, value))
            .collect::<std::collections::BTreeMap<_, _>>();
        let baseline = expected
            .values
            .iter()
            .map(|(key, value)| (*key, value))
            .collect::<std::collections::BTreeMap<_, _>>();
        let changed_keys = current
            .keys()
            .chain(baseline.keys())
            .copied()
            .collect::<BTreeSet<_>>()
            .into_iter()
            .filter(|key| current.get(key) != baseline.get(key))
            .collect();
        ProcessStateDrift { changed_keys }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProcessStateDrift {
    pub changed_keys: Vec<&'static str>,
}

impl ProcessStateDrift {
    pub const fn stable_tag(&self) -> &'static str {
        "test/env_contamination_detected"
    }

    pub fn is_empty(&self) -> bool {
        self.changed_keys.is_empty()
    }
}

pub struct ScopedTestConfig {
    _lock: MutexGuard<'static, ()>,
    before: ProcessStateSnapshot,
}

impl ScopedTestConfig {
    pub fn apply(updates: &[(&'static str, Option<&str>)]) -> Self {
        let lock = PROCESS_STATE_LOCK
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let before = ProcessStateSnapshot::capture(updates.iter().map(|(key, _)| *key));
        for (key, value) in updates {
            match value {
                Some(value) => std::env::set_var(key, value),
                None => std::env::remove_var(key),
            }
        }
        Self {
            _lock: lock,
            before,
        }
    }

    pub fn set(key: &'static str, value: &str) -> Self {
        Self::apply(&[(key, Some(value))])
    }
}

impl Drop for ScopedTestConfig {
    fn drop(&mut self) {
        for (key, value) in self.before.values.iter().rev() {
            match value {
                Some(value) => std::env::set_var(key, value),
                None => std::env::remove_var(key),
            }
        }
        let restored =
            ProcessStateSnapshot::capture(self.before.values.iter().map(|(key, _)| *key));
        let drift = restored.drift_from(&self.before);
        if !drift.is_empty() && !std::thread::panicking() {
            panic!("[{}] {:?}", drift.stable_tag(), drift.changed_keys);
        }
    }
}

pub fn with_env_var<R>(key: &'static str, value: &str, f: impl FnOnce() -> R) -> R {
    let _config = ScopedTestConfig::set(key, value);
    f()
}

pub fn with_env_vars<R>(updates: &[(&'static str, Option<&str>)], f: impl FnOnce() -> R) -> R {
    let _config = ScopedTestConfig::apply(updates);
    f()
}

pub fn with_stage3_features<R>(f: impl FnOnce() -> R) -> R {
    with_env_vars(&[("NYASH_FEATURES", Some("stage3"))], f)
}

pub fn with_stage3_block_catch<R>(f: impl FnOnce() -> R) -> R {
    with_env_vars(
        &[
            ("NYASH_FEATURES", Some("stage3")),
            ("NYASH_BLOCK_CATCH", Some("1")),
        ],
        f,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic::{catch_unwind, AssertUnwindSafe};

    const RESTORE_KEY: &str = "NYASH_SCOPED_TEST_CONFIG_RESTORE_PROBE";
    const PANIC_KEY: &str = "NYASH_SCOPED_TEST_CONFIG_PANIC_PROBE";
    const DRIFT_KEY: &str = "NYASH_SCOPED_TEST_CONFIG_DRIFT_PROBE";

    #[test]
    fn scoped_config_restores_absent_and_existing_values() {
        std::env::remove_var(RESTORE_KEY);
        with_env_var(RESTORE_KEY, "temporary", || {
            assert_eq!(
                std::env::var(RESTORE_KEY).ok().as_deref(),
                Some("temporary")
            );
        });
        assert_eq!(std::env::var_os(RESTORE_KEY), None);

        std::env::set_var(RESTORE_KEY, "before");
        with_env_var(RESTORE_KEY, "temporary", || {
            assert_eq!(
                std::env::var(RESTORE_KEY).ok().as_deref(),
                Some("temporary")
            );
        });
        assert_eq!(std::env::var(RESTORE_KEY).ok().as_deref(), Some("before"));
        std::env::remove_var(RESTORE_KEY);
    }

    #[test]
    fn scoped_config_restores_after_panic_without_poisoning_follow_on_scope() {
        std::env::remove_var(PANIC_KEY);
        let panic_result = catch_unwind(AssertUnwindSafe(|| {
            with_env_var(PANIC_KEY, "panic", || panic!("intentional test panic"));
        }));

        assert!(panic_result.is_err());
        assert_eq!(std::env::var_os(PANIC_KEY), None);
        with_env_var(PANIC_KEY, "after", || {
            assert_eq!(std::env::var(PANIC_KEY).ok().as_deref(), Some("after"));
        });
        assert_eq!(std::env::var_os(PANIC_KEY), None);
    }

    #[test]
    fn snapshot_reports_unscoped_process_state_drift() {
        std::env::remove_var(DRIFT_KEY);
        let before = ProcessStateSnapshot::capture([DRIFT_KEY]);
        std::env::set_var(DRIFT_KEY, "leaked");
        let after = ProcessStateSnapshot::capture([DRIFT_KEY]);
        let drift = after.drift_from(&before);
        std::env::remove_var(DRIFT_KEY);

        assert_eq!(drift.changed_keys, vec![DRIFT_KEY]);
        assert_eq!(drift.stable_tag(), "test/env_contamination_detected");
    }
}
