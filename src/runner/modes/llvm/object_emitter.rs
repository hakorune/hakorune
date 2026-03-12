//! Object file emitter for LLVM mode
//!
//! Handles LLVM object file generation when requested (feature-gated).

#[cfg(feature = "llvm-harness")]
use crate::config::env;
use nyash_rust::mir::MirModule;

/// Object emitter Box
///
/// **Responsibility**: Emit LLVM object file if requested
/// **Input**: &MirModule
/// **Output**: Result<bool, String> (Ok(true) if emitted, Ok(false) if not requested, Err on failure)
#[allow(dead_code)]
pub struct ObjectEmitterBox;

impl ObjectEmitterBox {
    /// Emit LLVM object file if requested
    ///
    /// Checks NYASH_LLVM_OBJ_OUT environment variable.
    /// If set, emits object file and verifies it's not empty.
    #[cfg(feature = "llvm-harness")]
    pub fn try_emit(module: &MirModule) -> Result<bool, String> {
        let Some(out_path) = env::env_string("NYASH_LLVM_OBJ_OUT") else {
            return Ok(false); // Not requested
        };

        if crate::config::env::llvm_use_harness() {
            crate::runner::modes::common_util::exec::llvmlite_emit_object(
                module, &out_path, 20_000,
            )?;

            // Verify object file
            Self::verify_object(&out_path)?;
            return Ok(true);
        }

        // Verify object presence and size (>0)
        Self::verify_object(&out_path)?;
        Ok(true)
    }

    #[cfg(feature = "llvm-harness")]
    fn verify_object(path: &str) -> Result<(), String> {
        match std::fs::metadata(path) {
            Ok(meta) if meta.len() > 0 => {
                if env::cli_verbose_enabled() {
                    crate::console_println!(
                        "[LLVM] object emitted: {} ({} bytes)",
                        path,
                        meta.len()
                    );
                }
                Ok(())
            }
            Ok(_) => Err(format!("harness object is empty: {}", path)),
            Err(e) => Err(format!("harness output not found: {} ({})", path, e)),
        }
    }

    #[cfg(not(feature = "llvm-harness"))]
    #[allow(dead_code)]
    pub fn try_emit(_module: &MirModule) -> Result<bool, String> {
        Ok(false)
    }
}
