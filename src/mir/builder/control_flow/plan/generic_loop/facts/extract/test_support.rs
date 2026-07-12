use std::ffi::OsString;
use std::sync::Mutex;

static ENV_LOCK: Mutex<()> = Mutex::new(());

struct EnvRestore {
    joinir_dev: Option<OsString>,
    planner_required: Option<OsString>,
    strict: Option<OsString>,
}

impl EnvRestore {
    fn set(joinir_dev: Option<&str>, planner_required: Option<&str>, strict: Option<&str>) -> Self {
        let restore = Self {
            joinir_dev: std::env::var_os("NYASH_JOINIR_DEV"),
            planner_required: std::env::var_os("HAKO_JOINIR_PLANNER_REQUIRED"),
            strict: std::env::var_os("HAKO_JOINIR_STRICT"),
        };
        set_or_remove("NYASH_JOINIR_DEV", joinir_dev);
        set_or_remove("HAKO_JOINIR_PLANNER_REQUIRED", planner_required);
        set_or_remove("HAKO_JOINIR_STRICT", strict);
        restore
    }
}

impl Drop for EnvRestore {
    fn drop(&mut self) {
        restore_var("NYASH_JOINIR_DEV", self.joinir_dev.take());
        restore_var("HAKO_JOINIR_PLANNER_REQUIRED", self.planner_required.take());
        restore_var("HAKO_JOINIR_STRICT", self.strict.take());
    }
}

fn set_or_remove(name: &str, value: Option<&str>) {
    match value {
        Some(value) => std::env::set_var(name, value),
        None => std::env::remove_var(name),
    }
}

fn restore_var(name: &str, value: Option<OsString>) {
    match value {
        Some(value) => std::env::set_var(name, value),
        None => std::env::remove_var(name),
    }
}

pub(super) fn with_joinir_env<T>(
    joinir_dev: Option<&str>,
    planner_required: Option<&str>,
    f: impl FnOnce() -> T,
) -> T {
    with_joinir_env_inner(joinir_dev, planner_required, None, f)
}

pub(super) fn with_strict_joinir_env<T>(f: impl FnOnce() -> T) -> T {
    with_joinir_env_inner(Some("1"), Some("1"), Some("1"), f)
}

fn with_joinir_env_inner<T>(
    joinir_dev: Option<&str>,
    planner_required: Option<&str>,
    strict: Option<&str>,
    f: impl FnOnce() -> T,
) -> T {
    let _lock = ENV_LOCK
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let _restore = EnvRestore::set(joinir_dev, planner_required, strict);
    f()
}
