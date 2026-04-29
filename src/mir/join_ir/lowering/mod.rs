//! JoinIR Lowering Functions
//!
//! Phase 27.9: Modular separation of MIR → JoinIR lowering implementations.
//! Phase 33-12: Router-based If/Loop lowering organization.
//!
//! このモジュールは各種 MIR 関数を JoinIR に変換する lowering 関数を提供します。
//!
//! ## 構成:
//! - `common.rs`: CFG sanity checks と lowering 共通ユーティリティ（Phase 27.10）
//! - `value_id_ranges.rs`: ValueId 範囲管理（Phase 27.13+）
//! - `min_loop.rs`: JoinIrMin.main/0 専用の最小ループ lowering
//! - `skip_ws.rs`: Main.skip/1 の空白スキップ lowering（手書き版＋MIR自動解析版）
//! - `funcscanner_trim.rs`: FuncScannerBox.trim/1 の trim lowering
//! - `stage1_using_resolver.rs`: Stage1UsingResolverBox.resolve_for_source entries loop lowering（Phase 27.12）
//! - `funcscanner_append_defs.rs`: FuncScannerBox._append_defs/2 の配列結合 lowering（Phase 27.14）
//! - `if_select.rs`: Phase 33 If/Else → Select lowering
//! - `if_dry_runner.rs`: Phase 33-10 If lowering dry-run スキャナー（箱化版）
//! - `if_lowering_router.rs`: Phase 33-12 If-expression routing (Select/IfMerge dispatcher)

#![allow(dead_code)]

pub mod canonical_names; // Phase 256 P1.7: SSOT for JoinIR function names (k_exit, loop_step, main)
pub mod carrier_info; // Phase 196: Carrier metadata for loop lowering
pub(crate) mod common; // Internal lowering utilities
pub mod complex_addend_normalizer; // Phase 192: Complex addend normalization (AST preprocessing)
pub mod condition_env; // Phase 171-fix: Condition expression environment
pub(crate) mod condition_lowerer; // Phase 171-fix: Core condition lowering logic
pub mod condition_lowering_box; // Phase 244: Unified condition lowering interface (trait-based)
pub mod condition_pattern; // Phase 219-fix: If condition pattern detection (simple vs complex)
pub mod condition_to_joinir; // Phase 169: JoinIR condition lowering orchestrator (refactored)
pub(crate) mod condition_var_extractor; // Phase 171-fix: Variable extraction from condition AST
pub mod continue_branch_normalizer; // Phase 33-19: Continue branch normalization for continue-only route shape
pub mod debug_output_box; // Phase 85: Centralized debug output management
pub mod digitpos_condition_normalizer; // Phase 224-E: DigitPos condition normalizer (digit_pos < 0 → !is_digit_pos)
pub mod error_tags; // Phase 86: Centralized error message formatting
pub(crate) mod exit_args_resolver; // Internal exit argument resolution
pub mod exit_meta_builder; // Phase 118: ExitMeta builder box for if_phi_join route shape
pub mod expr_lowerer; // Phase 231: Unified expression lowering with scope management
pub mod funcscanner_append_defs;
pub mod funcscanner_trim;
pub(crate) mod generic_case_a; // Phase 192: Modularized Case A lowering
pub mod generic_type_resolver; // Phase 66: P3-C ジェネリック型推論箱
pub mod if_dry_runner; // Phase 33-10.0
pub(crate) mod if_lowering_router; // Phase 33-12: If-expression routing (re-exported)
pub mod if_merge; // Phase 33-7
pub mod if_phi_context; // Phase 61-1
pub mod if_phi_spec; // Phase 61-2
pub(crate) mod if_select; // Phase 33: Internal If/Select lowering
pub mod inline_boundary; // Phase 188-Impl-3: JoinIR→Host boundary
pub mod inline_boundary_builder; // Phase 200-2: Builder pattern for JoinInlineBoundary
pub mod join_value_space; // Phase 201: Unified JoinIR ValueId allocation
pub mod loop_body_local_env; // Phase 184: Body-local variable environment
pub mod loop_body_local_init; // Phase 186: Body-local init expression lowering
pub(crate) mod loop_form_intake; // Internal loop form intake
pub(crate) mod loop_route_validator; // Phase 33-23: Loop structure validation
pub mod loop_scope_shape;
pub mod loop_to_join;
pub mod loop_update_analyzer; // Phase 197: Update expression analyzer for carrier semantics
pub(crate) mod loop_view_builder; // Phase 33-23: Loop lowering dispatch
pub mod method_call_lowerer; // Phase 224-B: MethodCall lowering (metadata-driven)
pub mod method_return_hint; // Phase 83: P3-D 既知メソッド戻り値型推論箱
pub(crate) mod return_collector; // Phase 284 P1: Return statement collector SSOT
pub mod scope_manager; // Phase 231: Unified variable scope management // Phase 195: loop_continue_only minimal lowerer support
pub mod user_method_policy; // Phase 252: User-defined method policy (SSOT for static box method whitelists) // Phase 47-A: Generic step scheduler for loop_break/if_phi_join // Phase 73: BindingId-based scope PoC (dev-only)
                            // Phase 242-EX-A: loop_with_if_phi_minimal removed - replaced by loop_with_if_phi_if_sum
pub mod loop_with_if_phi_if_sum; // Phase 213: if_phi_join AST-based if-sum lowerer (Phase 242-EX-A: supports complex conditions)
pub mod min_loop;
pub mod scan_bool_predicate_minimal; // Phase 259 P0: bool_predicate_scan minimal lowerer (is_integer/is_valid boolean predicate scan)
pub mod scan_with_init_minimal; // Phase 254 P1: scan_with_init minimal lowerer (index_of/find/contains)
pub mod scan_with_init_reverse; // Phase 257 P0: scan_with_init reverse scan lowerer (last_index_of)
pub mod simple_while_minimal; // Phase 188-Impl-1: loop_simple_while minimal lowerer
pub mod skip_ws;
pub mod split_scan_minimal; // Phase 256 P0: split_scan minimal lowerer (split/tokenization with variable step)
pub mod stage1_using_resolver;
pub mod stageb_body;
pub mod stageb_funcscanner;
pub mod type_hint_policy; // Phase 65.5: 型ヒントポリシー箱化
pub mod type_inference; // Phase 65-2-A
pub mod update_env; // Phase 184: Unified variable resolution for update expressions
pub(crate) mod value_id_ranges; // Internal ValueId range management

// Re-export public lowering functions
pub use funcscanner_append_defs::lower_funcscanner_append_defs_to_joinir;
pub use funcscanner_trim::lower_funcscanner_trim_to_joinir;
// Phase 200-2: Builder pattern
pub use inline_boundary_builder::JoinInlineBoundaryBuilder;
// Phase 31: LoopToJoinLowerer 統一箱
pub use loop_to_join::LoopToJoinLowerer;
// Phase 30 F-3: 旧 lower_case_a_loop_to_joinir_for_minimal_skip_ws は _with_scope に置き換え済みのため削除
pub use min_loop::lower_min_loop_to_joinir;
pub use skip_ws::lower_skip_ws_to_joinir;
pub use stage1_using_resolver::lower_stage1_usingresolver_to_joinir;
pub use stageb_body::lower_stageb_body_to_joinir;
pub use stageb_funcscanner::lower_stageb_funcscanner_to_joinir;

pub use if_lowering_router::try_lower_if_to_joinir;

/// Phase 33-9.1: Loop lowering対象関数の判定
///
/// これらの関数は Phase 32/33 で LoopToJoinLowerer によって処理されます。
/// If lowering (Select/IfMerge) の対象から除外することで、Loop/If の責務を明確に分離します。
///
/// Phase 82 SSOT: JOINIR_TARGETS テーブルから Exec 対象を参照
/// （テーブルは vm_bridge_dispatch/targets.rs で一元管理）
///
/// ## 対象関数（6本）
/// - Main.skip/1: 空白スキップループ
/// - FuncScannerBox.trim/1: 前後空白削除ループ
/// - FuncScannerBox.append_defs/2: 配列結合ループ
/// - Stage1UsingResolverBox.resolve_for_source/5: using解析ループ
/// - StageBBodyExtractorBox.build_body_src/2: Stage-B本体抽出ループ
/// - StageBFuncScannerBox.scan_all_boxes/1: Stage-B Box走査ループ
///
/// ## 将来の拡張
/// NYASH_JOINIR_LOWER_GENERIC=1 で汎用 Case-A ループにも拡張可能
pub(crate) fn is_loop_lowered_function(name: &str) -> bool {
    // Phase 82 SSOT: vm_bridge_dispatch テーブルから Loop 関数を抽出
    // Phase 33-9.1: If lowering の除外対象は、JOINIR_TARGETS に登録されたすべての関数
    // （Exec/LowerOnly 問わず、ループ専任関数として Loop lowering で処理）
    crate::mir::join_ir_vm_bridge_dispatch::JOINIR_TARGETS
        .iter()
        .any(|t| t.func_name == name)
}

// ============================================================================
// Phase 80: JoinIR Mainline Unification - Core ON 時の本線化判定
// ============================================================================

/// Phase 80: JoinIR 本線化対象（Loop）の判定（JoinIR は常時 ON）
pub fn is_loop_mainline_target(name: &str) -> bool {
    is_loop_lowered_function(name)
}

/// Phase 80/184: JoinIR 本線化対象（If）の判定（JoinIR は常時 ON）
///
/// Phase 184: JOINIR_IF_TARGETS テーブルからの参照に変更
pub fn is_if_mainline_target(name: &str) -> bool {
    crate::mir::join_ir_vm_bridge_dispatch::is_if_lowered_function(name)
}

/// Phase 80: JoinIR を本線として試行すべきか判定（Core 常時 ON）
pub fn should_try_joinir_mainline(func_name: &str, is_loop: bool) -> bool {
    if is_loop {
        is_loop_mainline_target(func_name)
    } else {
        is_if_mainline_target(func_name)
    }
}

/// Phase 80/81: Strict モードで JoinIR lowering 失敗時にパニックすべきか判定
pub fn should_panic_on_joinir_failure(func_name: &str, is_loop: bool) -> bool {
    if !crate::config::env::joinir_strict_enabled() {
        return false;
    }
    should_try_joinir_mainline(func_name, is_loop)
}

/// Phase 61-4/184: ループ外 If の JoinIR 対象関数判定
///
/// HAKO_JOINIR_IF_TOPLEVEL=1 有効時に、ループ外 if の JoinIR 経路を試行する関数。
/// Phase 184: JOINIR_IF_TARGETS テーブルに統一（SSOT化）
///
/// ## 対象関数（テーブル管理）
/// - IfSelectTest.*: テスト専用関数群
/// - IfMergeTest.*: 複数変数テスト（Phase 33-7）
/// - IfToplevelTest.*: ループ外 if テスト専用（Phase 61-4）
/// - JsonShapeToMap._read_value_from_pair/1: Phase 33-4 Stage-1 実用関数
/// - Stage1JsonScannerBox.value_start_after_key_pos/2: Phase 33-4 Stage-B 実用関数
///
/// ## 使用方法
/// if_form.rs から呼び出され、関数名がテーブルに含まれる場合のみ
/// JoinIR 経路を試行する。
///
/// Phase 184: テーブル参照に変更（プレフィックス判定は併用）
pub fn is_joinir_if_toplevel_target(name: &str) -> bool {
    // Phase 184: JOINIR_IF_TARGETS テーブルから参照（exact match）
    if crate::mir::join_ir_vm_bridge_dispatch::JOINIR_IF_TARGETS
        .iter()
        .any(|t| t.func_name == name)
    {
        return true;
    }

    if crate::mir::join_ir_vm_bridge_dispatch::is_if_toplevel_prefix_target(name) {
        return true;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Phase 33-9.1: is_loop_lowered_function() の動作確認
    #[test]
    fn test_is_loop_lowered_function() {
        // Loop 専任関数（6本）は true を返す
        assert!(is_loop_lowered_function("Main.skip/1"));
        assert!(is_loop_lowered_function("FuncScannerBox.trim/1"));
        assert!(is_loop_lowered_function("FuncScannerBox.append_defs/2"));
        assert!(is_loop_lowered_function(
            "Stage1UsingResolverBox.resolve_for_source/5"
        ));
        assert!(is_loop_lowered_function(
            "StageBBodyExtractorBox.build_body_src/2"
        ));
        assert!(is_loop_lowered_function(
            "StageBFuncScannerBox.scan_all_boxes/1"
        ));

        // If lowering 対象関数は false を返す
        assert!(!is_loop_lowered_function("IfSelectTest.simple_return/0"));
        assert!(!is_loop_lowered_function("IfMergeTest.multiple_true/0"));
        assert!(!is_loop_lowered_function(
            "JsonShapeToMap._read_value_from_pair/1"
        ));
        assert!(!is_loop_lowered_function(
            "Stage1JsonScannerBox.value_start_after_key_pos/2"
        ));

        // 一般的な関数も false を返す
        assert!(!is_loop_lowered_function("SomeBox.some_method/3"));
        assert!(!is_loop_lowered_function("Main.main/0"));
    }
}
