//! Phase 30 F-3.1: Case-A minimal ターゲット判定
//!
//! Phase 48-5 で構造ベース判定を追加。
//! Phase 48-5.5 で LoopStructuralAnalysis 箱化モジュール化。

use crate::mir::loop_form::LoopForm;
use crate::runtime::get_global_ring0;

use super::shape::LoopScopeShape;
use super::structural::LoopStructuralAnalysis;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CaseAMinimalTargetKind {
    SkipWhitespace,
    Trim,
    AppendDefs,
    Stage1UsingResolver,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct CaseAMinimalTargetDesc {
    pub(crate) func_name: &'static str,
    pub(crate) kind: CaseAMinimalTargetKind,
}

const CASE_A_MINIMAL_TARGETS: &[CaseAMinimalTargetDesc] = &[
    CaseAMinimalTargetDesc {
        func_name: "Main.skip/1",
        kind: CaseAMinimalTargetKind::SkipWhitespace,
    },
    CaseAMinimalTargetDesc {
        func_name: "FuncScannerBox.trim/1",
        kind: CaseAMinimalTargetKind::Trim,
    },
    CaseAMinimalTargetDesc {
        func_name: "FuncScannerBox.append_defs/2",
        kind: CaseAMinimalTargetKind::AppendDefs,
    },
    CaseAMinimalTargetDesc {
        func_name: "Stage1UsingResolverBox.resolve_for_source/5",
        kind: CaseAMinimalTargetKind::Stage1UsingResolver,
    },
];

pub(crate) fn find_case_a_minimal_target(func_name: &str) -> Option<CaseAMinimalTargetDesc> {
    CASE_A_MINIMAL_TARGETS
        .iter()
        .copied()
        .find(|target| target.func_name == func_name)
}

/// 現在 JoinIR lowering でサポートしている Case-A minimal ループのみ true を返す。
/// これらは LoopScopeShape の新しい analyze_case_a パスを通る。
///
/// # Supported Targets
///
/// - `Main.skip/1`: minimal_ssa_skip_ws.hako
/// - `FuncScannerBox.trim/1`: funcscanner_trim_min.hako
/// - `FuncScannerBox.append_defs/2`: funcscanner_append_defs_min.hako
/// - `Stage1UsingResolverBox.resolve_for_source/5`: stage1_using_resolver minimal
///
/// # Phase 48-5: 構造ベース判定への移行
///
/// 名前ハードコードは後方互換性のために保持。
/// 構造判定は `validate_case_a_structural()` で検証される。
pub(crate) fn is_case_a_minimal_target(func_name: &str) -> bool {
    find_case_a_minimal_target(func_name).is_some()
}

/// Phase 48-5.5: analyze_case_a 内で構造判定を検証
///
/// 名前ハードコードで Case-A と判定されたループが、構造的にも Case-A の性質を持つか検証する。
/// 将来的に名前ハードコードを削除し、構造判定のみに移行するための準備。
///
/// # Phase 48-5.5: 箱化モジュール化
///
/// LoopStructuralAnalysis 箱を使用して構造判定を実行。
/// is_case_a_structural() は LoopStructuralAnalysis::is_case_a_minimal() に統合された。
///
/// # Returns
///
/// - `true`: 構造判定も通過（正常）
/// - `false`: 構造判定に失敗（警告ログ）
pub(crate) fn validate_case_a_structural(
    loop_form: &LoopForm,
    scope: &LoopScopeShape,
    func_name: &str,
) -> bool {
    let analysis = LoopStructuralAnalysis::from_loop_scope(loop_form, scope);
    let is_structural = analysis.is_case_a_minimal();

    if !is_structural {
        get_global_ring0().log.warn(&format!(
            "[case_a/warning] {} is marked as Case-A by name, but fails structural check",
            func_name
        ));
    }

    is_structural
}

#[cfg(test)]
mod tests {
    use super::{find_case_a_minimal_target, is_case_a_minimal_target, CaseAMinimalTargetKind};

    #[test]
    fn case_a_minimal_target_table_keeps_accepted_subset() {
        for (name, kind) in [
            ("Main.skip/1", CaseAMinimalTargetKind::SkipWhitespace),
            ("FuncScannerBox.trim/1", CaseAMinimalTargetKind::Trim),
            (
                "FuncScannerBox.append_defs/2",
                CaseAMinimalTargetKind::AppendDefs,
            ),
            (
                "Stage1UsingResolverBox.resolve_for_source/5",
                CaseAMinimalTargetKind::Stage1UsingResolver,
            ),
        ] {
            let target = find_case_a_minimal_target(name)
                .expect("Case-A minimal target should stay accepted");
            assert_eq!(target.kind, kind);
            assert!(is_case_a_minimal_target(name));
        }
    }

    #[test]
    fn case_a_minimal_target_table_rejects_non_subset_loop_targets() {
        for name in [
            "StageBBodyExtractorBox.build_body_src/2",
            "StageBFuncScannerBox.scan_all_boxes/1",
            "IfSelectTest.simple_return/0",
            "Main.main/0",
        ] {
            assert!(find_case_a_minimal_target(name).is_none());
            assert!(!is_case_a_minimal_target(name));
        }
    }
}
