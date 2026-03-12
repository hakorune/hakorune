//! PyVM harness executor (dev/test helper)
//!
//! Handles execution via PyVM harness when requested for development/testing.

use super::error::LlvmRunError;
use nyash_rust::mir::MirModule;

/// PyVM executor Box
///
/// **Responsibility**: Execute via PyVM harness when requested (dev/test helper)
/// **Input**: &MirModule
/// **Output**: Result<i32, LlvmRunError> (Ok(exit_code) on success, Err if failed or not requested)
///
/// **IMPORTANT**: This Box is used by 8 JSON AST smoke tests. DO NOT REMOVE!
pub struct PyVmExecutorBox;

impl PyVmExecutorBox {
    /// Execute via PyVM harness if requested
    ///
    /// This function checks the SMOKES_USE_PYVM environment variable.
    /// If set to "1", it executes the module via PyVM harness and returns the exit code.
    ///
    /// Returns Ok(exit_code) on success, Err(LlvmRunError) on failure.
    /// If SMOKES_USE_PYVM is not set, returns Err with a special "not requested" message.
    pub fn try_execute(module: &MirModule) -> Result<i32, LlvmRunError> {
        if std::env::var("SMOKES_USE_PYVM").ok().as_deref() != Some("1") {
            return Err(LlvmRunError::new(0, "PyVM not requested"));
        }

        super::super::common_util::legacy::pyvm::run_pyvm_harness_lib(module, "llvm-ast")
            .map_err(|e| LlvmRunError::fatal(format!("PyVM harness error: {}", e)))
    }
}
