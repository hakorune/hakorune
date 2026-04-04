//! Exit reporter for LLVM mode (Phase 285LLVM-0)
//!
//! Handles leak report emission and process exit.

/// Exit reporter Box
///
/// **Responsibility**: Emit leak report and exit process
/// **Input**: exit code (i32)
/// **Output**: ! (never returns)
pub struct ExitReporterBox;

impl ExitReporterBox {
    /// Emit leak report and exit process
    ///
    /// Phase 285LLVM-0: Emit Rust-side leak report before exit (if enabled).
    /// Note: Only reports Rust VM-side roots (modules, host_handles, plugin_boxes).
    ///
    /// This function never returns.
    pub fn emit_and_exit(code: i32) -> ! {
        // Phase 285LLVM-0: Emit Rust-side leak report before exit (if enabled)
        // Note: Only reports Rust VM-side roots (modules, host_handles, plugin_boxes).
        crate::runtime::leak_tracker::emit_leak_report();

        std::process::exit(code);
    }
}
