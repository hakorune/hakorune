// Phase 190: Use modularized converter
use super::{JoinIrFunctionConverter, JoinIrVmBridgeError};
use crate::mir::join_ir::frontend::JoinFuncMetaMap;
use crate::mir::join_ir::{JoinFuncId, JoinModule};
use crate::mir::MirModule;
use std::collections::BTreeMap;

/// Phase 40-1実験用: JoinFuncMetaを使ったMIR変換
///
/// JoinFuncMetaを参照できるMIR変換入口。
///
/// # Role
/// The standard `run_joinir_via_vm()` path reaches this through
/// `bridge_joinir_to_mir()`. Tests may call it directly to verify metadata
/// handling.
///
/// # Architecture
/// JoinModule → MirModule変換において、JoinFuncMetaを観測する。
/// 現在はPHI拡張をここでは生成しない。
///
/// # Returns
/// - `Ok(MirModule)`: 変換済みMIRモジュール
pub fn convert_join_module_to_mir_with_meta(
    module: &JoinModule,
    meta: &JoinFuncMetaMap,
    boundary: Option<&crate::mir::join_ir::lowering::inline_boundary::JoinInlineBoundary>,
) -> Result<MirModule, JoinIrVmBridgeError> {
    // Phase 256 P1.5: boundary is now passed through, reserved for future ValueId remap logic
    let _boundary = boundary; // Suppress unused warning for now
    debug_log!("[Phase 40-1] convert_join_module_to_mir_with_meta");

    let mut mir_module = MirModule::new("joinir_bridge_with_meta".to_string());

    // Phase 256 P1.8: Build function name map for all functions in the module
    // This ensures Call instructions use actual names ("main", "loop_step", "k_exit")
    // instead of generated names ("join_func_0", "join_func_1", etc.)
    let func_name_map: BTreeMap<JoinFuncId, String> = module
        .functions
        .iter()
        .map(|(id, func)| (*id, func.name.clone()))
        .collect();

    // 1. 各関数を変換
    for (func_id, join_func) in &module.functions {
        debug_log!(
            "[Phase 40-1] Converting JoinFunction {} ({})",
            func_id.0,
            join_func.name
        );

        // 2. 基本のMIR変換（Phase 256 P1.8: with func_name_map）
        let mir_func = JoinIrFunctionConverter::convert_function_with_func_names(
            join_func,
            func_name_map.clone(),
        )?;

        // Phase 189 DEBUG: Dump MirFunction blocks to check PHI presence
        // Guarded to avoid polluting stdout/stderr in normal runs.
        debug_log!(
            "[joinir/meta] MirFunc '{}' has {} blocks after convert_function:",
            mir_func.signature.name,
            mir_func.blocks.len()
        );
        if crate::config::env::joinir_vm_bridge_debug() {
            for (block_id, block) in &mir_func.blocks {
                let phi_count = block
                    .instructions
                    .iter()
                    .filter(|i| matches!(i, crate::mir::MirInstruction::Phi { .. }))
                    .count();
                debug_log!(
                    "[joinir/meta]   Block {:?}: {} instructions ({} PHI), terminator={:?}",
                    block_id,
                    block.instructions.len(),
                    phi_count,
                    block
                        .terminator
                        .as_ref()
                        .map(|t| format!("{:?}", t).chars().take(40).collect::<String>())
                );
            }
        }

        // 3. Phase 40-1: if_modified_vars observation
        if let Some(m) = meta.get(func_id) {
            if let Some(if_vars) = &m.if_modified_vars {
                debug_log!(
                    "[Phase 40-1] Found if_modified_vars for func {:?}: {:?}",
                    func_id,
                    if_vars
                );

                // Metadata observation only. PHI generation belongs in an active
                // lowering contract with fixtures, not in this bridge helper.
            }
        }

        // Phase 256 P1.7: Use actual function name instead of join_func_name()
        // join_func_name() produces "join_func_{id}" but JoinFunction.name contains
        // the actual name ("main", "loop_step", "k_exit").
        // The merge code looks up functions by name, so we must use the actual name.
        mir_module
            .functions
            .insert(join_func.name.clone(), mir_func);
    }

    Ok(mir_module)
}
