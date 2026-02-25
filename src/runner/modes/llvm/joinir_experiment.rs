//! JoinIR experiment for LLVM mode (Phase 32 L-4.3a)
//!
//! Handles JoinIR lowering experiment for fixing PHI issues (feature-gated).

use nyash_rust::mir::MirModule;

/// JoinIR experiment Box
///
/// **Responsibility**: Apply JoinIR lowering experiment when enabled (feature-gated)
/// **Input**: MirModule
/// **Output**: MirModule (converted if enabled, original otherwise)
pub struct JoinIrExperimentBox;

impl JoinIrExperimentBox {
    /// Apply JoinIR experiment if enabled
    ///
    /// Phase 32 L-4.3a: When NYASH_JOINIR_EXPERIMENT=1 and NYASH_JOINIR_LLVM_EXPERIMENT=1,
    /// try to lower MIR → JoinIR → MIR' for Main.skip/1 to fix PHI issues.
    /// JoinIR-converted functions are merged back into the original module.
    #[cfg(feature = "llvm-harness")]
    pub fn apply(module: MirModule) -> MirModule {
        if !crate::config::env::joinir_experiment_enabled()
            || !crate::config::env::joinir_llvm_experiment_enabled()
            || !crate::config::env::llvm_use_harness()
        {
            return module;
        }

        use nyash_rust::mir::join_ir::lower_skip_ws_to_joinir;
        use nyash_rust::mir::join_ir_vm_bridge::bridge_joinir_to_mir;

        crate::runtime::get_global_ring0()
            .log
            .debug("[joinir/llvm] Attempting JoinIR path for LLVM execution");

        // Try to lower Main.skip/1 to JoinIR
        if module.functions.contains_key("Main.skip/1") {
            match lower_skip_ws_to_joinir(&module) {
                Some(join_module) => {
                    crate::runtime::get_global_ring0().log.debug(&format!(
                        "[joinir/llvm] ✅ Lowered to JoinIR ({} functions)",
                        join_module.functions.len()
                    ));
                    // Convert JoinIR back to MIR' (with normalized PHI)
                    match bridge_joinir_to_mir(&join_module) {
                        Ok(mir_from_joinir) => {
                            crate::runtime::get_global_ring0().log.debug(&format!(
                                "[joinir/llvm] ✅ Converted to MIR' ({} functions)",
                                mir_from_joinir.functions.len()
                            ));
                            // Merge JoinIR functions into original module
                            // Strategy: Remove Main.skip/1 (PHI-problematic) and rename join_func_0 to Main.skip/1
                            let mut merged = module.clone();

                            // Remove the original PHI-problematic Main.skip/1
                            if merged.functions.remove("Main.skip/1").is_some() {
                                crate::runtime::get_global_ring0().log.debug("[joinir/llvm] Removed original Main.skip/1 (PHI-problematic)");
                            }

                            for (name, func) in mir_from_joinir.functions {
                                // Rename join_func_0 → Main.skip/1 to maintain call compatibility
                                let target_name = if name == "join_func_0" {
                                    crate::runtime::get_global_ring0().log.debug(&format!(
                                        "[joinir/llvm] Renaming {} → Main.skip/1",
                                        name
                                    ));
                                    "Main.skip/1".to_string()
                                } else {
                                    crate::runtime::get_global_ring0().log.debug(&format!(
                                        "[joinir/llvm] Adding JoinIR function: {}",
                                        name
                                    ));
                                    name
                                };
                                merged.functions.insert(target_name, func);
                            }
                            crate::runtime::get_global_ring0().log.debug(&format!(
                                "[joinir/llvm] ✅ Merged module ({} functions)",
                                merged.functions.len()
                            ));
                            merged
                        }
                        Err(e) => {
                            crate::runtime::get_global_ring0().log.debug(&format!(
                                "[joinir/llvm] ❌ JoinIR→MIR conversion failed: {:?}",
                                e
                            ));
                            crate::runtime::get_global_ring0()
                                .log
                                .debug("[joinir/llvm] Falling back to original MIR");
                            module
                        }
                    }
                }
                None => {
                    crate::runtime::get_global_ring0()
                        .log
                        .debug("[joinir/llvm] ❌ JoinIR lowering returned None");
                    crate::runtime::get_global_ring0()
                        .log
                        .debug("[joinir/llvm] Falling back to original MIR");
                    module
                }
            }
        } else {
            crate::runtime::get_global_ring0()
                .log
                .debug("[joinir/llvm] Main.skip/1 not found, using original MIR");
            module
        }
    }

    #[cfg(not(feature = "llvm-harness"))]
    pub fn apply(module: MirModule) -> MirModule {
        module
    }
}
