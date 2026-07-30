//! JoinIR Lowering Functions
//!
//! Phase 27.9: Modular separation of MIR → JoinIR lowering implementations.
//! Loop lowering and the two explicit VM-reference observations live here.
//!
//! このモジュールは各種 MIR 関数を JoinIR に変換する lowering 関数を提供します。
//!
//! ## 構成:
//! - `common.rs`: CFG sanity checks と lowering 共通ユーティリティ（Phase 27.10）
//! - `value_id_ranges.rs`: ValueId 範囲管理（Phase 27.13+）
//! - `skip_ws.rs`: Main.skip/1 の空白スキップ lowering（手書き版＋MIR自動解析版）
//! - `funcscanner_trim.rs`: FuncScannerBox.trim/1 の trim lowering

pub mod canonical_names; // Phase 256 P1.7: SSOT for JoinIR function names (k_exit, loop_step, main)
pub mod carrier_info; // Phase 196: Carrier metadata for loop lowering
pub(crate) mod common; // Internal lowering utilities
pub mod condition_env; // Phase 171-fix: Condition expression environment
pub mod error_tags; // Phase 86: Centralized error message formatting
pub(crate) mod exit_args_resolver; // Internal exit argument resolution
pub mod funcscanner_trim;
pub(crate) mod generic_case_a; // Phase 192: Modularized Case A lowering
pub mod inline_boundary; // Phase 188-Impl-3: JoinIR→Host boundary
#[cfg(test)]
pub mod inline_boundary_builder; // Test-only builder pattern for JoinInlineBoundary
pub mod join_value_space; // Phase 201: Unified JoinIR ValueId allocation
pub(crate) mod loop_form_intake; // Internal loop form intake
pub(crate) mod loop_route_validator; // Phase 33-23: Loop structure validation
pub mod loop_scope_shape;
pub(crate) mod loop_target_policy;
pub mod loop_to_join;
pub mod loop_update_analyzer; // Phase 197: Update expression analyzer for carrier semantics
pub(crate) mod loop_view_builder; // Phase 33-23: Loop lowering dispatch
pub mod simple_while_minimal; // Phase 188-Impl-1: loop_simple_while minimal lowerer
pub mod skip_ws;
pub mod type_inference; // Phase 65-2-A
pub(crate) mod value_id_ranges; // Internal ValueId range management

// Re-export public lowering functions
pub use funcscanner_trim::lower_funcscanner_trim_to_joinir;
#[cfg(test)]
pub use inline_boundary_builder::JoinInlineBoundaryBuilder;
// Phase 31: LoopToJoinLowerer 統一箱
pub use loop_to_join::LoopToJoinLowerer;
pub use skip_ws::lower_skip_ws_to_joinir;

/// Phase 33-9.1: Loop lowering対象関数の判定
///
/// これらの関数は Phase 32/33 で LoopToJoinLowerer によって処理されます。
/// If lowering (Select/IfMerge) の対象から除外することで、Loop/If の責務を明確に分離します。
///
/// Classification SSOT: `loop_target_policy`.
///
/// ## 対象関数（5本）
/// - Main.skip/1: 空白スキップループ
/// - FuncScannerBox.trim/1: 前後空白削除ループ
/// - Stage1UsingResolverBox.resolve_for_source/5: lower-resolver compatibility using解析ループ
/// - StageBBodyExtractorBox.build_body_src/2: mode-B compatibility本体抽出ループ
/// - StageBFuncScannerBox.scan_all_boxes/1: mode-B compatibility Box走査ループ
///
/// ## 将来の拡張
/// NYASH_JOINIR_LOWER_GENERIC=1 で汎用 Case-A ループにも拡張可能
pub(crate) fn is_loop_lowered_function(name: &str) -> bool {
    loop_target_policy::is_loop_lowering_target(name)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Phase 33-9.1: is_loop_lowered_function() の動作確認
    #[test]
    fn test_is_loop_lowered_function() {
        // Loop 専任関数（5本）は true を返す
        assert!(is_loop_lowered_function("Main.skip/1"));
        assert!(is_loop_lowered_function("FuncScannerBox.trim/1"));
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
        assert!(!is_loop_lowered_function("FuncScannerBox.append_defs/2"));

        // 一般的な関数も false を返す
        assert!(!is_loop_lowered_function("SomeBox.some_method/3"));
        assert!(!is_loop_lowered_function("Main.main/0"));
    }
}
