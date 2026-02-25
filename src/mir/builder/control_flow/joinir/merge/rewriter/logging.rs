//! Logging utilities for instruction rewriter
//!
//! Phase 260 P0.1: Extracted from instruction_rewriter.rs
//! Provides centralized logging with verbose flag control (DEBUG-177 style).

use crate::mir::builder::control_flow::joinir::trace::JoinLoopTrace;

/// Log message conditionally based on enabled flag
///
/// Equivalent to the log! macro in instruction_rewriter.rs:
/// ```
/// macro_rules! log {
///     ($enabled:expr, $($arg:tt)*) => {
///         trace.stderr_if(&format!($($arg)*), $enabled);
///     };
/// }
/// ```
///
/// # Example
///
/// ```ignore
/// let trace = trace::trace();
/// log_if(&trace, true, "[DEBUG] Block {:?} processed", block_id);
/// ```
#[inline]
pub(super) fn log_if(trace: &JoinLoopTrace, enabled: bool, message: &str) {
    trace.stderr_if(message, enabled);
}

/// Macro version for format string convenience
///
/// Usage: `log!(trace, enabled, "format {}", arg);`
///
/// This macro forwards to log_if() with format!() handling.
#[macro_export]
macro_rules! rewriter_log {
    ($trace:expr, $enabled:expr, $($arg:tt)*) => {
        $crate::mir::builder::control_flow::joinir::merge::rewriter::logging::log_if(
            $trace,
            $enabled,
            &format!($($arg)*)
        )
    };
}

// Phase 260 P0.1: Keep local macro compatibility
//
// The original log! macro was defined locally in merge_and_rewrite().
// This module provides both function (log_if) and macro (rewriter_log!) forms.
//
// Migration strategy:
// 1. First, use logging::log_if() to replace some log! calls (verify)
// 2. Then, replace remaining log! with rewriter_log! macro
// 3. Finally, remove local log! macro from instruction_rewriter.rs
