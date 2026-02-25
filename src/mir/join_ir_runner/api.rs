use crate::backend::mir_interpreter::MirInterpreter;
use crate::mir::join_ir::{JoinFuncId, JoinModule};

use super::exec::execute_function;
use super::{JoinRuntimeError, JoinValue};

#[cfg(feature = "normalized_dev")]
use crate::config::env::joinir_dev::{current_joinir_mode, JoinIrMode};
#[cfg(feature = "normalized_dev")]
use crate::mir::join_ir::normalized::{dev_env, normalized_dev_roundtrip_structured, shape_guard};

pub fn run_joinir_function(
    vm: &mut MirInterpreter,
    module: &JoinModule,
    entry: JoinFuncId,
    args: &[JoinValue],
) -> Result<JoinValue, JoinRuntimeError> {
    #[cfg(feature = "normalized_dev")]
    {
        // Canonical shapes always go through Normalized roundtrip regardless of mode/env.
        let canonical_shapes = shape_guard::canonical_shapes(module);
        if !canonical_shapes.is_empty() {
            let args_vec = args.to_vec();
            return dev_env::with_dev_env_if_unset(|| {
                let structured = normalized_dev_roundtrip_structured(module).map_err(|msg| {
                    JoinRuntimeError::new(format!(
                        "[joinir/normalized-dev/runner] canonical roundtrip failed: {}",
                        msg
                    ))
                })?;
                if dev_env::normalized_dev_logs_enabled() {
                    crate::runtime::get_global_ring0().log.debug(&format!(
                        "[joinir/normalized-dev/runner] canonical normalized roundtrip (shapes={:?}, functions={})",
                        canonical_shapes,
                        structured.functions.len()
                    ));
                }
                execute_function(vm, &structured, entry, args_vec)
            });
        }
    }

    #[cfg(feature = "normalized_dev")]
    match current_joinir_mode() {
        JoinIrMode::NormalizedDev => {
            return run_joinir_function_normalized_dev(vm, module, entry, args);
        }
        _ => {
            // Structured-only path (default)
        }
    }

    execute_function(vm, module, entry, args.to_vec())
}

#[cfg(feature = "normalized_dev")]
fn run_joinir_function_normalized_dev(
    vm: &mut MirInterpreter,
    module: &JoinModule,
    entry: JoinFuncId,
    args: &[JoinValue],
) -> Result<JoinValue, JoinRuntimeError> {
    // JoinIrMode::NormalizedDev path: Structured→Normalized→Structured roundtrip
    // Keep dev path opt-in and fail-fast: only Structured P1/P2 minis are supported.
    dev_env::with_dev_env_if_unset(|| {
        let debug = dev_env::normalized_dev_logs_enabled();
        let args_vec = args.to_vec();

        let shapes = shape_guard::supported_shapes(module);
        if shapes.is_empty() {
            if debug {
                crate::runtime::get_global_ring0().log.debug(
                    "[joinir/normalized-dev/runner] shape unsupported; staying on Structured path"
                );
            }
            return execute_function(vm, module, entry, args_vec);
        }

        let structured_roundtrip = normalized_dev_roundtrip_structured(module).map_err(|msg| {
            JoinRuntimeError::new(format!("[joinir/normalized-dev/runner] {}", msg))
        })?;

        if debug {
            crate::runtime::get_global_ring0().log.debug(&format!(
                "[joinir/normalized-dev/runner] normalized roundtrip succeeded (shapes={:?}, functions={})",
                shapes,
                structured_roundtrip.functions.len()
            ));
        }

        execute_function(vm, &structured_roundtrip, entry, args_vec)
    })
}
