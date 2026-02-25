//! Runner tracing helpers (verbose-guarded)

use crate::runtime::get_global_ring0;

/// Return whether CLI verbose logging is enabled
#[allow(dead_code)]
pub fn cli_verbose() -> bool {
    crate::config::env::cli_verbose()
}

#[macro_export]
macro_rules! cli_v {
    ($($arg:tt)*) => {{
        if crate::config::env::cli_verbose() {
            crate::runtime::get_global_ring0().log.debug(&format!($($arg)*));
        }
    }};
}

/// Unstructured trace output function used by pipeline helpers
pub fn log<S: AsRef<str>>(msg: S) {
    get_global_ring0().log.debug(msg.as_ref());
}
