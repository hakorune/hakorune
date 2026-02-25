//! Macro logging SSOT (guarded by env flags)

/// Macro log guard (trace or CLI verbose).
pub fn enabled() -> bool {
    crate::config::env::macro_trace() || crate::config::env::macro_cli_verbose()
}

#[macro_export]
macro_rules! macro_log {
    ($($arg:tt)*) => {
        if $crate::r#macro::log::enabled() {
            $crate::runtime::get_global_ring0()
                .log
                .debug(&format!($($arg)*));
        }
    };
}
