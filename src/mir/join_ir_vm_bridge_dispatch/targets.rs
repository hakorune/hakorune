use crate::mir::MirModule;

/// JoinIR ブリッジ対象の記述子
#[derive(Debug, Clone, Copy)]
pub struct JoinIrTargetDesc {
    /// 対象関数名（MirModule.functions のキー）
    pub func_name: &'static str,
    /// デフォルト有効化。
    ///
    /// `JOINIR_IF_TARGETS` では mainline 対象の判定に使う。Loop bridge
    /// (`JOINIR_VM_EXEC_TARGETS`) は常に明示 env でだけ VM bridge に入る。
    pub default_enabled: bool,
}

/// JoinIR VM bridgeの実行対象テーブル。
///
/// Loop/If/strict classification is the separate neutral
/// `join_ir::lowering::loop_target_policy`.
pub const JOINIR_VM_EXEC_TARGETS: &[JoinIrTargetDesc] = &[
    JoinIrTargetDesc {
        func_name: crate::mir::join_ir::lowering::loop_target_policy::MAIN_SKIP,
        default_enabled: false, // PHI canary のため env 必須
    },
    JoinIrTargetDesc {
        func_name: crate::mir::join_ir::lowering::loop_target_policy::FUNCSCANNER_TRIM,
        default_enabled: false, // VM bridge は env 必須
    },
];

/// VM実行テーブルから対象関数を探す。
pub(crate) fn find_joinir_target(module: &MirModule) -> Option<&'static JoinIrTargetDesc> {
    JOINIR_VM_EXEC_TARGETS
        .iter()
        .find(|target| module.functions.contains_key(target.func_name))
}

// ============================================================================
// Phase 184: JoinIR If Lowering Targets (Separate from Loop Targets)
// ============================================================================

/// JoinIR If lowering 対象テーブル（SSOT）
///
/// Phase 184: Loop lowering policyと分離した If lowering 専用テーブル。
///
/// **責務**:
/// - If/Else → Select/IfMerge lowering の対象関数を一覧化
/// - Loop lowering と独立して管理（1関数につき1 lowering の原則）
///
/// **使用箇所**:
/// - `is_if_mainline_target()`: Core ON 時の本線化判定
/// - `try_lower_if_to_joinir()`: If lowering 試行時のホワイトリスト
///
/// | 関数 | Kind | デフォルト有効 | 備考 |
/// |-----|------|---------------|------|
/// | IfSelectTest.test/1 | Exec | Yes | Phase 33-2/33-3 simple return pattern |
/// | IfSelectLocalTest.main/0 | Exec | Yes | Phase 33-10 local variable pattern |
/// | IfMergeTest.simple_true/0 | Exec | Yes | Phase 33-7 multiple variables (IfMerge) |
/// | IfMergeTest.simple_false/0 | Exec | Yes | Phase 33-7 multiple variables (IfMerge) |
/// | JsonShapeToMap._read_value_from_pair/1 | Exec | Yes | Phase 33-4 Stage-1 実用関数 |
/// | Stage1JsonScannerBox.value_start_after_key_pos/2 | Exec | Yes | Phase 33-4 Stage-B 実用関数 |
///
/// Phase 184 設計ドキュメント:
/// - docs/private/roadmap2/phases/phase-184/if_lowering_inventory.md
/// - docs/private/roadmap2/phases/phase-184/README.md
pub const JOINIR_IF_TARGETS: &[JoinIrTargetDesc] = &[
    // Test functions (Phase 33 series)
    JoinIrTargetDesc {
        func_name: "IfSelectTest.test/1",
        default_enabled: true, // Simple return pattern (Phase 33-2/33-3)
    },
    JoinIrTargetDesc {
        func_name: "IfSelectLocalTest.main/0",
        default_enabled: true, // Local variable pattern (Phase 33-10)
    },
    JoinIrTargetDesc {
        func_name: "IfMergeTest.simple_true/0",
        default_enabled: true, // Multiple variables (Phase 33-7)
    },
    JoinIrTargetDesc {
        func_name: "IfMergeTest.simple_false/0",
        default_enabled: true, // Multiple variables (Phase 33-7)
    },
    // Selfhost/Production functions (Phase 33-4 explicit approvals)
    JoinIrTargetDesc {
        func_name: "JsonShapeToMap._read_value_from_pair/1",
        default_enabled: true, // Stage-1 実用関数
    },
    JoinIrTargetDesc {
        func_name: "Stage1JsonScannerBox.value_start_after_key_pos/2",
        default_enabled: true, // Stage-B 実用関数
    },
];

/// Phase 184: If lowering 対象関数の判定
///
/// JOINIR_IF_TARGETS テーブルから対象関数を検索し、
/// default_enabled が true の関数のみを本線対象とする。
///
/// **用途**:
/// - `is_if_mainline_target()`: Core ON 時の本線化判定
/// - `should_try_joinir_mainline(func_name, is_loop=false)` 経由で使用
pub fn is_if_lowered_function(name: &str) -> bool {
    JOINIR_IF_TARGETS
        .iter()
        .any(|t| t.func_name == name && t.default_enabled)
}

/// Prefix-based If lowering rollout policy.
///
/// Exact targets belong to `JOINIR_IF_TARGETS`. This helper owns only the
/// historical prefix families that are still intentionally broader than the
/// table rows.
pub fn is_if_lowering_prefix_target(name: &str, stage1_enabled: bool) -> bool {
    name.starts_with("IfSelectTest.")
        || name.starts_with("IfSelectLocalTest.")
        || name.starts_with("IfMergeTest.")
        || name.starts_with("IfToplevelTest.")
        || name.starts_with("Stage1JsonScannerTestBox.")
        || (stage1_enabled && name.starts_with("Stage1"))
}

/// Prefix subset used by the toplevel-if entry check.
///
/// This intentionally stays narrower than `is_if_lowering_prefix_target` to
/// preserve the existing toplevel behavior while centralizing the strings.
pub fn is_if_toplevel_prefix_target(name: &str) -> bool {
    name.starts_with("IfSelectTest.")
        || name.starts_with("IfToplevelTest.")
        || name.starts_with("IfMergeTest.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vm_exec_targets_are_exactly_skip_and_trim() {
        let names = JOINIR_VM_EXEC_TARGETS
            .iter()
            .map(|target| target.func_name)
            .collect::<Vec<_>>();
        assert_eq!(
            names,
            [
                crate::mir::join_ir::lowering::loop_target_policy::MAIN_SKIP,
                crate::mir::join_ir::lowering::loop_target_policy::FUNCSCANNER_TRIM,
            ]
        );
        assert!(JOINIR_VM_EXEC_TARGETS
            .iter()
            .all(|target| !target.default_enabled));
    }

    #[test]
    fn lower_only_observation_names_are_not_vm_targets() {
        for func_name in [
            crate::mir::join_ir::lowering::loop_target_policy::STAGE1_USING_RESOLVER,
            crate::mir::join_ir::lowering::loop_target_policy::STAGEB_BODY_EXTRACTOR,
            crate::mir::join_ir::lowering::loop_target_policy::STAGEB_FUNC_SCANNER,
            "FuncScannerBox.append_defs/2",
        ] {
            assert!(JOINIR_VM_EXEC_TARGETS
                .iter()
                .all(|target| target.func_name != func_name));
        }
    }
}
