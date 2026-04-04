//! Fallback executor for LLVM mode (mock/legacy)
//!
//! Handles fallback execution when LLVM backends are not available.

use super::error::LlvmRunError;
use nyash_rust::{mir::MirInstruction, mir::MirModule};

/// Fallback executor Box
///
/// **Responsibility**: Execute fallback path (feature check + mock)
/// **Input**: &MirModule
/// **Output**: Result<i32, LlvmRunError> (Ok(exit_code) on success, Err on failure)
pub struct FallbackExecutorBox;

impl FallbackExecutorBox {
    /// Execute fallback path (feature check + mock)
    ///
    /// Fail-fast: if the user explicitly requested the llvmlite harness
    /// but this binary was built without the `llvm-harness` feature,
    /// do not silently fall back to mock.
    ///
    /// Otherwise, executes mock execution that inspects the MIR
    /// and returns a deterministic exit code based on Return instructions.
    pub fn execute(module: &MirModule) -> Result<i32, LlvmRunError> {
        // Fail-fast: if the user explicitly requested the llvmlite harness
        // but this binary was built without the `llvm-harness` feature,
        // do not silently fall back to mock.
        if crate::config::env::env_bool("NYASH_LLVM_USE_HARNESS") {
            return Err(LlvmRunError::fatal(
                "LLVM harness requested (NYASH_LLVM_USE_HARNESS=1), but this binary was built without `--features llvm` (llvm-harness).\n\
Fix:\n  cargo build --release -p nyash-rust --features llvm --bin hakorune\n\
Then ensure prerequisites:\n  cargo build --release -p nyash-llvm-compiler\n  cargo build --release -p nyash_kernel\n\
Tip: tools/run_llvm_harness.sh <program.hako>"
            ));
        }

        crate::console_println!("🔧 Mock LLVM Backend Execution:");
        crate::console_println!("   Build with --features llvm for real backend.");

        // NamingBox SSOT: Select entry (arity-aware, Main.main → main fallback)
        let entry =
            crate::runner::modes::common_util::entry_selection::select_entry_function(module);

        if let Some(main_func) = module.functions.get(&entry) {
            for (_bid, block) in &main_func.blocks {
                if let Some(term) = &block.terminator {
                    match term {
                        MirInstruction::Return { value: Some(_) } => {
                            crate::console_println!("✅ Mock exit code: 42");
                            return Ok(42);
                        }
                        MirInstruction::Return { value: None } => {
                            crate::console_println!("✅ Mock exit code: 0");
                            return Ok(0);
                        }
                        _ => {}
                    }
                }
            }
        }

        crate::console_println!("✅ Mock exit code: 0");
        Ok(0)
    }
}
