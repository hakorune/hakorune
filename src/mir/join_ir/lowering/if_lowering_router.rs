//! # If-Expression JoinIR Lowering Router
//!
//! **Phase 33-12 Modularization**: Extracted from `mod.rs` (lines 201-423)
//!
//! ## Responsibility
//! Routes if-expressions to appropriate JoinIR lowering strategies.
//! This is the **main entry point** for if-expression → JoinIR lowering.
//!
//! ## Design Pattern
//! - **Input**: AST if-expression + MirBuilder
//! - **Output**: JoinModule (or error)
//! - **Side effects**: Modifies MirBuilder (adds blocks, values, instructions)
//!
//! ## Why Separate Router?
//! If and loop lowering are orthogonal concerns:
//! - If: Expression-level lowering (conditional value selection)
//! - Loop: Statement-level lowering (control flow patterns)
//! - Keeping separate: Easier to extend, test, and maintain
//!
//! ## Related Modules
//! - See also: `loop_route_router.rs` (loop lowering)
//! - Parent: `mod.rs` (central lowering dispatcher)
//!
//! # Routing Strategy
//!
//! This router tries multiple lowering strategies in order:
//! 1. **IfMerge**: For multiple-variable patterns (Phase 33-7)
//! 2. **IfSelect**: For single-variable patterns (Phase 33-2/33-3)
//!
//! # Control Flow
//!
//! - Checks toggle flags (`joinir_if_select_enabled()`)
//! - Validates function whitelist (test/Stage-1/explicit approvals)
//! - Excludes loop-lowered functions (Phase 33-9.1 separation)
//! - Falls back to traditional if_phi on pattern mismatch
//!
//! # Phase 61-1: If-in-Loop Support
//!
//! Context parameter enables if-in-loop lowering:
//! - `None`: Pure if-expression
//! - `Some(context)`: If-in-loop with carrier variables

use crate::mir::join_ir::JoinInst;
use crate::mir::{BasicBlockId, MirFunction};
use crate::runtime::get_global_ring0;

/// Phase 33-7: Try to lower if/else to JoinIR Select/IfMerge instruction
///
/// Scope:
/// - Only applies to whitelisted functions:
///   - IfSelectTest.* (Phase 33-2/33-3)
///   - IfMergeTest.* (Phase 33-7)
///   - JsonShapeToMap._read_value_from_pair/1 (Phase 33-4 Stage-1)
///   - Stage1JsonScannerBox.value_start_after_key_pos/2 (Phase 33-4 Stage-B)
/// - Requires JoinIR If-select toggle (HAKO_JOINIR_IF_SELECT / joinir_if_select_enabled)
/// - Falls back to traditional if_phi on pattern mismatch
///
/// Pattern selection:
/// - 1 variable → Select
/// - 2+ variables → IfMerge
///
/// Phase 61-1: If-in-loop support
/// - `context` parameter: If-in-loop context (carrier_names for loop variables)
/// - None = Pure If, Some(_) = If-in-loop
///
/// Returns Some(JoinInst::Select) or Some(JoinInst::IfMerge) if pattern matched, None otherwise.
pub fn try_lower_if_to_joinir(
    func: &MirFunction,
    block_id: BasicBlockId,
    debug: bool,
    context: Option<&crate::mir::join_ir::lowering::if_phi_context::IfPhiContext>, // Phase 61-1: If-in-loop context
) -> Option<JoinInst> {
    // 1. dev/Core トグルチェック
    //
    // - Core: joinir_if_select_enabled()（JoinIR は常時 ON）
    // - Dev:  joinir_dev_enabled()（詳細ログ等）
    //
    // 実際の挙動切り替えは joinir_if_select_enabled() に集約
    if !crate::config::env::joinir_if_select_enabled() {
        return None;
    }
    // Phase 185: strict check moved to caller (if_form.rs)
    // let strict_on = crate::config::env::joinir_strict_enabled();

    // Phase 33-9.1: Loop専任関数の除外（Loop/If責務分離）
    // Loop lowering対象関数はIf loweringの対象外にすることで、
    // 1関数につき1 loweringの原則を保証します
    if super::is_loop_lowered_function(&func.signature.name) {
        return None;
    }

    // Phase 33-8: デバッグログレベル取得（0-3）
    let debug_level = crate::config::env::joinir_debug_level();
    let _debug = debug || debug_level >= 1;

    // 2. Phase 33-8: 関数名ガード拡張（テスト + Stage-1 rollout + 明示承認）
    let is_allowed =
        // Test functions (always enabled)
        func.signature.name.starts_with("IfSelectTest.") ||
        func.signature.name.starts_with("IfSelectLocalTest.") || // Phase 33-10 test
        func.signature.name.starts_with("IfMergeTest.") ||
        func.signature.name.starts_with("IfToplevelTest.") || // Phase 61-4: loop-outside if test
        func.signature.name.starts_with("Stage1JsonScannerTestBox.") || // Phase 33-5 test

        // Stage-1 rollout (env-controlled)
        (crate::config::env::joinir_stage1_enabled() &&
         func.signature.name.starts_with("Stage1")) ||

        // Explicit approvals (Phase 33-4で検証済み, always on)
        matches!(func.signature.name.as_str(),
            "JsonShapeToMap._read_value_from_pair/1" |
            "Stage1JsonScannerBox.value_start_after_key_pos/2"
        );

    if !is_allowed {
        if debug_level >= 2 {
            get_global_ring0().log.debug(&format!(
                "[try_lower_if_to_joinir] skipping non-allowed function: {}",
                func.signature.name
            ));
        }
        return None;
    }
    // Phase 185: strict_allowed removed (strict check moved to caller: if_form.rs)

    if debug_level >= 1 {
        get_global_ring0().log.debug(&format!(
            "[try_lower_if_to_joinir] trying to lower {}",
            func.signature.name
        ));
    }

    // 3. Phase 33-7: IfMerge を優先的に試行（複数変数パターン）
    //    IfMerge が成功すればそれを返す、失敗したら Select を試行
    // Phase 61-1: context がある場合は with_context() を使用
    let if_merge_lowerer = if let Some(ctx) = context {
        crate::mir::join_ir::lowering::if_merge::IfMergeLowerer::with_context(
            debug_level,
            ctx.clone(),
        )
    } else {
        crate::mir::join_ir::lowering::if_merge::IfMergeLowerer::new(debug_level)
    };

    if if_merge_lowerer.can_lower_to_if_merge(func, block_id) {
        if let Some(result) = if_merge_lowerer.lower_if_to_if_merge(func, block_id) {
            if debug_level >= 1 {
                get_global_ring0().log.debug(&format!(
                    "[try_lower_if_to_joinir] ✅ IfMerge lowering used for {}",
                    func.signature.name
                ));
            }
            return Some(result);
        }
    }

    // 4. IfMerge が失敗したら Select を試行（単一変数パターン）
    // Phase 61-1: context がある場合は with_context() を使用
    let if_select_lowerer = if let Some(ctx) = context {
        crate::mir::join_ir::lowering::if_select::IfSelectLowerer::with_context(
            debug_level,
            ctx.clone(),
        )
    } else {
        crate::mir::join_ir::lowering::if_select::IfSelectLowerer::new(debug_level)
    };

    // Phase 185: Remove strict checks from lowerer (thin Result-returning box)
    // Strict mode panic should happen at caller level (if_form.rs), not here
    if !if_select_lowerer.can_lower_to_select(func, block_id) {
        if debug_level >= 1 {
            get_global_ring0().log.debug(&format!(
                "[try_lower_if_to_joinir] pattern not matched for {}",
                func.signature.name
            ));
        }
        return None;
    }

    let result = if_select_lowerer.lower_if_to_select(func, block_id);

    if result.is_some() && debug_level >= 1 {
        get_global_ring0().log.debug(&format!(
            "[try_lower_if_to_joinir] ✅ Select lowering used for {}",
            func.signature.name
        ));
    }

    result
}
