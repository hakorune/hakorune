//! Error reporting for LLVM mode
//!
//! This is the ONLY place that calls ExitReporterBox::emit_and_exit.

use super::error::LlvmRunError;
use super::exit_reporter::ExitReporterBox;

/// Emit error and exit process
///
/// This function:
/// 1. Prints error message
/// 2. Calls ExitReporterBox::emit_and_exit (which includes leak report)
///
/// This is the SINGLE exit point for all LLVM mode errors.
pub fn emit_error_and_exit(err: LlvmRunError) -> ! {
    crate::console_println!("❌ {}", err.msg);
    ExitReporterBox::emit_and_exit(err.code);
}
