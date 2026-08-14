//! Selected Boundary executor.
//!
//! This module owns only the selected Dynamic physical route. The
//! compatibility harness remains in `harness_executor`; both modules share
//! the existing process runner until the feature recut is complete.

use super::error::LlvmRunError;
use crate::config::env;
use nyash_rust::mir::MirModule;

/// Boundary executor for the selected Dynamic artifact lane.
pub struct BoundaryExecutorBox;

impl BoundaryExecutorBox {
    /// Emit, root-validate, launch, and clean one selected artifact bundle.
    ///
    /// The selected route is Boundary-only: failures are terminal and never
    /// fall through to the compatibility harness or mock executor.
    #[cfg(feature = "llvm-harness")]
    pub fn try_execute_selected_dynamic(module: &MirModule) -> Result<i32, LlvmRunError> {
        let exe_out = "tmp/nyash_llvm_run";
        let bundle = crate::runner::modes::common_util::selected_dynamic_artifact_bundle::
            selected_dynamic_bundle_path(exe_out);
        let nyrt_dir = crate::runner::modes::common_util::exec::selected_dynamic_nyrt_dir()
            .map_err(|error| {
                LlvmRunError::fatal(format!("selected Dynamic NyRt archive error: {error}"))
            })?;
        let libs = env::env_string("NYASH_LLVM_EXE_LIBS");
        let fence = crate::runner::modes::common_util::selected_dynamic_artifact_bundle::
            emit_selected_dynamic(
                module,
                bundle.to_string_lossy().as_ref(),
                Some(nyrt_dir.as_str()),
                libs.as_deref(),
            )
            .map_err(|error| {
                LlvmRunError::fatal(format!("selected Dynamic Boundary emit-exe error: {error}"))
            })?;
        fence
            .launch_and_cleanup(|program| {
                run_emitted_executable(program.to_string_lossy().as_ref())
                    .map_err(|error| error.msg)
            })
            .map_err(LlvmRunError::fatal)
    }

    #[cfg(not(feature = "llvm-harness"))]
    pub fn try_execute_selected_dynamic(_module: &MirModule) -> Result<i32, LlvmRunError> {
        Err(LlvmRunError::fatal(
            "selected Dynamic Boundary requires the LLVM runner feature",
        ))
    }
}

/// Run an already-published executable and preserve the existing output
/// contract shared by the explicit compatibility lane.
#[cfg(feature = "llvm-harness")]
pub(super) fn run_emitted_executable(exe_out: &str) -> Result<i32, LlvmRunError> {
    match crate::runner::modes::common_util::exec::run_executable(exe_out, &[], 20_000) {
        Ok((code, _timed_out, stdout_text)) => {
            if !stdout_text.is_empty() {
                print!("{}", stdout_text);
            }
            crate::console_println!("✅ LLVM (harness) execution completed (exit={})", code);
            Ok(code)
        }
        Err(e) => Err(LlvmRunError::fatal(format!("run executable error: {}", e))),
    }
}
