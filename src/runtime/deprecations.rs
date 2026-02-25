//! Deprecation warnings with "warn once" guards
use crate::runtime::get_global_ring0;
use std::sync::OnceLock;

fn warn_once(flag: &'static OnceLock<()>, msg: &str) {
    if flag.get().is_none() {
        let _ = flag.set(());
        get_global_ring0().log.warn(msg);
    }
}

static NYASH_TOML_WARN_ONCE: OnceLock<()> = OnceLock::new();

/// Warn once per process when nyash.toml is used while hako.toml is absent.
pub fn warn_nyash_toml_used_once() {
    warn_once(
        &NYASH_TOML_WARN_ONCE,
        "[deprecate] using nyash.toml; please rename to hako.toml",
    );
}
