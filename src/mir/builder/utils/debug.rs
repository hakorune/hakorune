//! Builder debug logging utilities
//!
//! Optional debug output controlled by environment variables:
//! - NYASH_BUILDER_DEBUG: Enable debug output
//! - NYASH_BUILDER_DEBUG_LIMIT: Cap the number of debug lines (default: unlimited)

use std::sync::atomic::{AtomicUsize, Ordering};

/// Check if builder debug logging is enabled
pub(in crate::mir::builder) fn builder_debug_enabled() -> bool {
    crate::config::env::builder_debug_enabled()
}

static BUILDER_DEBUG_COUNT: AtomicUsize = AtomicUsize::new(0);

/// Log a debug message (if enabled)
pub(in crate::mir::builder) fn builder_debug_log(msg: &str) {
    if builder_debug_enabled() {
        // Optional cap: limit the number of builder debug lines to avoid flooding the terminal.
        // Set via env: NYASH_BUILDER_DEBUG_LIMIT=<N> (default: unlimited)
        if let Some(cap) = crate::config::env::builder_debug_limit() {
            let n = BUILDER_DEBUG_COUNT.fetch_add(1, Ordering::Relaxed);
            if n >= cap {
                return;
            }
        }
        let ring0 = crate::runtime::get_global_ring0();
        ring0.log.debug(&format!("[BUILDER] {}", msg));
    }
}
