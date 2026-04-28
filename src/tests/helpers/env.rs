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

pub fn with_env_var<R>(key: &'static str, value: &str, f: impl FnOnce() -> R) -> R {
    #[cfg(test)]
    let _lock = ENV_LOCK.lock().expect("test env lock poisoned");

    let _guard = EnvVarGuard::set(key, value);
    f()
}
