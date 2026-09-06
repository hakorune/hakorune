//! Shared helpers for env-backed backend selection.
//!
//! This keeps the backend-selection pattern consistent across plugin and export
//! modules without forcing each caller to reimplement the cache + env read
//! boilerplate.

use std::sync::OnceLock;

pub(crate) fn cached_env_choice<T: Copy>(
    cache: &'static OnceLock<T>,
    env_key: &'static str,
    resolve: impl FnOnce(Option<&str>) -> T,
) -> T {
    *cache.get_or_init(|| {
        let value = std::env::var(env_key).ok();
        resolve(value.as_deref())
    })
}

pub(crate) fn panic_unsupported_env_value(
    context: &'static str,
    env_key: &'static str,
    value: &str,
) -> ! {
    panic!("[freeze:contract][{context}] unsupported {env_key}={value}");
}

/// Fallible startup admission sharing the existing cache, not a second profile
/// owner. Invalid/non-Unicode settings do not initialize it or panic across C.
pub(crate) fn try_cached_env_choice<T: Copy>(
    cache: &'static OnceLock<T>, env_key: &'static str,
    resolve: impl FnOnce(Option<&str>) -> Result<T, ()>,
) -> Result<T, ()> {
    if let Some(value) = cache.get() { return Ok(*value); }
    let value = match std::env::var(env_key) {
        Ok(value) => Some(value),
        Err(std::env::VarError::NotPresent) => None,
        Err(std::env::VarError::NotUnicode(_)) => return Err(()),
    };
    let selected = resolve(value.as_deref())?;
    let _ = cache.set(selected);
    Ok(*cache.get().expect("successful set or concurrent initializer"))
}
