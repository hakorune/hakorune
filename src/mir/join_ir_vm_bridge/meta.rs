// Phase 190: Use modularized converter
use super::{JoinIrFunctionConverter, JoinIrVmBridgeError};
use crate::mir::join_ir::frontend::JoinFuncMetaMap;
use crate::mir::join_ir::{JoinFuncId, JoinModule};
use crate::mir::{MirFunction, MirModule};
use std::collections::BTreeMap;

/// Phase 40-1実験用: JoinFuncMetaを使ったMIR変換
///
/// 既存の run_joinir_via_vm() を拡張し、
/// if_modified_varsがあればloop exit PHIを生成する。
///
/// # Phase 40-1専用
/// この関数はPhase 40-1 A/Bテスト専用。
/// 本番パスでは使わない（従来のrun_joinir_via_vm()を使う）。
///
/// # Architecture
/// JoinModule → MirModule変換において、JoinFuncMetaを参照してPHI生成を拡張
///
/// # Returns
/// - `Ok(MirModule)`: 変換済みMIRモジュール（PHI拡張版）
pub fn convert_join_module_to_mir_with_meta(
    module: &JoinModule,
    meta: &JoinFuncMetaMap,
    boundary: Option<&crate::mir::join_ir::lowering::inline_boundary::JoinInlineBoundary>,
) -> Result<MirModule, JoinIrVmBridgeError> {
    // Phase 256 P1.5: boundary is now passed through, reserved for future ValueId remap logic
    let _boundary = boundary;  // Suppress unused warning for now
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

        // 3. Phase 40-1: if_modified_varsがあればloop exit PHI生成
        if let Some(m) = meta.get(func_id) {
            if let Some(if_vars) = &m.if_modified_vars {
                debug_log!(
                    "[Phase 40-1] Found if_modified_vars for func {:?}: {:?}",
                    func_id,
                    if_vars
                );

                // TODO(Phase 40-1.2): emit_loop_exit_phi_for_if_modified()実装後に有効化
                // emit_loop_exit_phi_for_if_modified(&mut mir_func, join_func, if_vars)?;
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

/// if-in-loop modified varsに対するloop exit PHI生成
///
/// # Purpose
/// JoinIR Frontendで検出されたif-in-loop修正変数に対して、
/// loop exit blockにPHI命令を追加する。
///
/// # Arguments
/// - `mir_func`: 変換済みMIR関数（ミュータブル）
/// - `join_func`: 元のJoinIR関数（メタデータ参照用）
/// - `if_modified_vars`: if-in-loop修正変数名のセット
///
/// # Implementation Note
/// 現在の実装では、JoinIRのloop_step関数は単一ブロックベースであり、
/// exit blockの特定が困難。Phase 40-1では**ログ出力のみ**を行い、
/// 実際のPHI生成はPhase 40-2以降で実装する。
///
/// # TODO(Phase 40-2)
/// - exit block特定ロジック実装
/// - PHI incoming value特定（header vs loop body）
/// - PHI命令生成とブロックへの挿入
#[allow(dead_code)]
pub(crate) fn emit_loop_exit_phi_for_if_modified(
    _mir_func: &mut MirFunction,
    join_func: &crate::mir::join_ir::JoinFunction,
    if_modified_vars: &std::collections::HashSet<String>,
) -> Result<(), JoinIrVmBridgeError> {
    debug_log!(
        "[Phase 40-1] emit_loop_exit_phi_for_if_modified: func={}, vars={:?}",
        join_func.name,
        if_modified_vars
    );

    // Phase 40-1 minimal implementation: ログ出力のみ
    // 理由: JoinIRのloop_step関数はtail-recursiveで、exit blockが明示的でない
    // TODO(Phase 40-2): JoinIR構造を拡張してexit block情報を保持

    if !if_modified_vars.is_empty() {
        debug_log!(
            "[Phase 40-1] Would generate {} loop exit PHIs for: {:?}",
            if_modified_vars.len(),
            if_modified_vars
        );
    }

    Ok(())
}
