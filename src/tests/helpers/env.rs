//! Shared test ENV helpers.

#[cfg(test)]
use std::sync::Mutex;

#[cfg(test)]
static ENV_LOCK: Mutex<()> = Mutex::new(());

struct EnvVarGuard {
    key: &'static str,
    old: Option<String>,
}

impl EnvVarGuard {
    fn set(key: &'static str, value: &str) -> Self {
        let old = std::env::var(key).ok();
        std::env::set_var(key, value);
        Self { key, old }
    }
}

impl Drop for EnvVarGuard {
    fn drop(&mut self) {
        if let Some(value) = &self.old {
            std::env::set_var(self.key, value);
        } else {
            std::env::remove_var(self.key);
        }
    }
}

struct EnvVarsGuard {
    old: Vec<(&'static str, Option<String>)>,
}

impl EnvVarsGuard {
    fn apply(updates: &[(&'static str, Option<&str>)]) -> Self {
        let old = updates
            .iter()
            .map(|(key, _)| (*key, std::env::var(key).ok()))
            .collect();
        for (key, value) in updates {
            match value {
                Some(value) => std::env::set_var(key, value),
                None => std::env::remove_var(key),
            }
        }
        Self { old }
    }
}

impl Drop for EnvVarsGuard {
    fn drop(&mut self) {
        for (key, value) in self.old.iter().rev() {
            match value {
                Some(value) => std::env::set_var(key, value),
                None => std::env::remove_var(key),
            }
        }
    }
}

pub fn with_env_var<R>(key: &'static str, value: &str, f: impl FnOnce() -> R) -> R {
    #[cfg(test)]
    let _lock = ENV_LOCK
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());

    let _guard = EnvVarGuard::set(key, value);
    f()
}

pub fn with_env_vars<R>(updates: &[(&'static str, Option<&str>)], f: impl FnOnce() -> R) -> R {
    #[cfg(test)]
    let _lock = ENV_LOCK
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());

    let _guard = EnvVarsGuard::apply(updates);
    f()
}
