pub(crate) static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

struct EnvRestoreGuard {
    prev: Vec<(String, Option<String>)>,
}

impl EnvRestoreGuard {
    fn capture(pairs: &[(&str, &str)]) -> Self {
        let prev = pairs
            .iter()
            .map(|(k, _)| ((*k).to_string(), std::env::var(k).ok()))
            .collect();
        Self { prev }
    }
}

impl Drop for EnvRestoreGuard {
    fn drop(&mut self) {
        for (k, prev_v) in self.prev.drain(..) {
            if let Some(v) = prev_v {
                std::env::set_var(&k, v);
            } else {
                std::env::remove_var(&k);
            }
        }
    }
}

pub(crate) fn with_env_vars<F: FnOnce()>(pairs: &[(&str, &str)], f: F) {
    let _guard = match ENV_LOCK.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    let _restore = EnvRestoreGuard::capture(pairs);
    for (k, v) in pairs {
        std::env::set_var(k, v);
    }
    f();
}

pub(crate) fn with_env_var<F: FnOnce()>(key: &str, value: &str, f: F) {
    with_env_vars(&[(key, value)], f);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic::{catch_unwind, AssertUnwindSafe};

    #[test]
    fn with_env_vars_recovers_after_panic_without_poisoning_follow_on_calls() {
        const KEY: &str = "NYASH_TEST_ENV_LOCK_RECOVERY";

        std::env::remove_var(KEY);
        let panic_result = catch_unwind(AssertUnwindSafe(|| {
            with_env_var(KEY, "panic", || {
                assert_eq!(std::env::var(KEY).ok().as_deref(), Some("panic"));
                panic!("intentional env panic");
            });
        }));

        assert!(panic_result.is_err(), "panic should propagate to caller");
        assert_eq!(
            std::env::var(KEY).ok().as_deref(),
            None,
            "panic path must restore the original env state"
        );

        with_env_var(KEY, "after", || {
            assert_eq!(std::env::var(KEY).ok().as_deref(), Some("after"));
        });

        assert_eq!(
            std::env::var(KEY).ok().as_deref(),
            None,
            "follow-on call should run after a poisoned panic path"
        );
    }
}
