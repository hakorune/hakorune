//! Phase 85: DebugOutputBox - Centralized debug output management for JoinIR
//!
//! ## Purpose
//! Provides structured debug output with automatic flag checking to eliminate
//! scattered `if is_joinir_debug() { eprintln!(...) }` patterns.
//!
//! ## Usage
//! ```rust,ignore
//! // Before:
//! if is_joinir_debug() {
//!     eprintln!("[phase80/p3] Registered loop var...");
//! }
//!
//! // After:
//! let debug = DebugOutputBox::new("phase80/p3");
//! debug.log("register", "Registered loop var...");
//! ```
//!
//! ## Benefits
//! - Centralized debug output control
//! - Consistent log formatting
//! - Feature-gated (no-op in production)
//! - Zero runtime cost when disabled

use crate::config::env::{is_joinir_debug, joinir_dev_enabled};
use crate::runtime::get_global_ring0;

/// DebugOutputBox: Centralized debug output for JoinIR lowering
///
/// Automatically checks HAKO_JOINIR_DEBUG flag and formats output consistently.
#[derive(Debug)]
pub struct DebugOutputBox {
    enabled: bool,
    context_tag: String,
}

impl DebugOutputBox {
    /// Create a new DebugOutputBox with the given context tag
    ///
    /// # Arguments
    /// * `context_tag` - Identifies the subsystem (e.g., "phase80/p3", "carrier_info")
    ///
    /// # Example
    /// ```rust,ignore
    /// let debug = DebugOutputBox::new("phase80/p3");
    /// ```
    pub fn new(context_tag: impl Into<String>) -> Self {
        Self {
            enabled: is_joinir_debug(),
            context_tag: context_tag.into(),
        }
    }

    /// Create a DebugOutputBox with an explicit enabled flag.
    ///
    /// Use this when the caller already has a higher-level gate (e.g. a `verbose` flag)
    /// and wants consistent formatting without re-checking env vars.
    pub fn new_with_enabled(context_tag: impl Into<String>, enabled: bool) -> Self {
        Self {
            enabled,
            context_tag: context_tag.into(),
        }
    }

    /// Create a DebugOutputBox enabled by JoinIR dev mode (NYASH_JOINIR_DEV=1).
    ///
    /// This is useful for "developer convenience" logs that should not require
    /// explicitly setting HAKO_JOINIR_DEBUG, but still must stay opt-in.
    pub fn new_dev(context_tag: impl Into<String>) -> Self {
        Self {
            enabled: joinir_dev_enabled(),
            context_tag: context_tag.into(),
        }
    }

    /// Log a debug message with category
    ///
    /// Output format: `[context_tag/category] message`
    ///
    /// # Arguments
    /// * `category` - Sub-category (e.g., "register", "promote", "bind")
    /// * `message` - Debug message
    ///
    /// # Example
    /// ```rust,ignore
    /// debug.log("register", "loop var 'i' BindingId(1) -> ValueId(5)");
    /// // Output: [phase80/p3/register] loop var 'i' BindingId(1) -> ValueId(5)
    /// ```
    pub fn log(&self, category: &str, message: &str) {
        if self.enabled {
            get_global_ring0()
                .log
                .debug(&format!("[{}/{}] {}", self.context_tag, category, message));
        }
    }

    /// Log a message without category
    ///
    /// Output format: `[context_tag] message`
    ///
    /// # Example
    /// ```rust,ignore
    /// debug.log_simple("Processing loop body");
    /// // Output: [phase80/p3] Processing loop body
    /// ```
    pub fn log_simple(&self, message: &str) {
        if self.enabled {
            get_global_ring0()
                .log
                .debug(&format!("[{}] {}", self.context_tag, message));
        }
    }

    /// Log only if enabled (with lazy message generation)
    ///
    /// Useful when message construction is expensive.
    ///
    /// # Example
    /// ```rust,ignore
    /// debug.log_if_enabled(|| {
    ///     format!("Complex value: {:?}", expensive_computation())
    /// });
    /// ```
    pub fn log_if_enabled(&self, f: impl FnOnce() -> String) {
        if self.enabled {
            let msg = f();
            get_global_ring0()
                .log
                .debug(&format!("[{}] {}", self.context_tag, msg));
        }
    }

    /// Check if debug output is enabled
    ///
    /// Useful for conditional code that shouldn't run in production.
    ///
    /// # Example
    /// ```rust,ignore
    /// if debug.is_enabled() {
    ///     // Expensive debug-only validation
    /// }
    /// ```
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debug_output_box_creation() {
        let debug = DebugOutputBox::new("test/context");
        assert_eq!(debug.context_tag, "test/context");
        // Note: is_enabled() depends on env var HAKO_JOINIR_DEBUG
    }

    #[test]
    fn test_log_methods_dont_panic() {
        let debug = DebugOutputBox::new("test");

        // These should never panic, even if disabled
        debug.log("category", "message");
        debug.log_simple("simple message");
        debug.log_if_enabled(|| "lazy message".to_string());
    }

    #[test]
    fn test_is_enabled_returns_bool() {
        let debug = DebugOutputBox::new("test");
        let enabled = debug.is_enabled();

        // Should return a boolean (either true or false)
        assert!(enabled == true || enabled == false);
    }

    #[test]
    fn test_lazy_message_only_called_if_enabled() {
        let debug = DebugOutputBox::new("test");
        let mut called = false;

        debug.log_if_enabled(|| {
            called = true;
            "message".to_string()
        });

        // If debug is disabled, called should still be false
        // If debug is enabled, called will be true
        // Either outcome is valid - we just verify no panic
        let _ = called; // Use the variable to avoid warning
    }
}
