//! Tiny env-gated logging helpers (quiet by default)

/// Returns true if the given env var is set to "1".
pub fn on(var: &str) -> bool {
    crate::config::env::env_string(var).as_deref() == Some("1")
}

/// Log a message to stderr if the env var is enabled.
pub fn log(var: &str, msg: &str) {
    if on(var) {
        crate::runtime::get_global_ring0().log.debug(msg);
    }
}

/// Log with formatting if the env var is enabled.
#[macro_export]
macro_rules! debug_logf {
    ($var:expr, $($arg:tt)*) => {{
        if crate::config::env::env_string($var).as_deref() == Some("1") {
            crate::runtime::get_global_ring0().log.debug(&format!($($arg)*));
        }
    }};
}
