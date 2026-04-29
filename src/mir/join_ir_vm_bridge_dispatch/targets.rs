use crate::mir::MirModule;

/// JoinIR ブリッジの実行範囲を表す enum
///
/// - `Exec`: JoinIR→VM 実行まで対応。意味論を A/B 実証済みのものに限定。
/// - `LowerOnly`: JoinIR lowering / Bridge 構造検証専用。実行は VM Route A にフォールバック。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JoinIrBridgeKind {
    /// JoinIR→VM 実行まで対応（skip/trim など、意味論を A/B 実証済み）
    Exec,
    /// JoinIR lowering / Bridge 構造検証専用（Stage-1/Stage-B など）
    LowerOnly,
}

/// JoinIR ブリッジ対象の記述子
///
/// 関数名と実行範囲（Exec/LowerOnly）をペアで管理する。
#[derive(Debug, Clone, Copy)]
pub struct JoinIrTargetDesc {
    /// 対象関数名（MirModule.functions のキー）
    pub func_name: &'static str,
    /// 実行範囲
    pub kind: JoinIrBridgeKind,
    /// デフォルト有効化。
    ///
    /// `JOINIR_IF_TARGETS` では mainline 対象の判定に使う。Loop bridge
    /// (`JOINIR_TARGETS`) は常に明示 env でだけ VM bridge に入り、loop
    /// mainline 判定は登録行の有無だけを見る。
    pub default_enabled: bool,
}

/// JoinIR ループ lowering 対象テーブル（SSOT）
///
/// Phase 32 L-4: 全対象関数を一覧化し、Exec/LowerOnly の区分を明示する。
/// Phase 82: このテーブルが唯一の SSOT。is_loop_lowered_function() はここから参照。
/// Phase 182: representative loop paths inventory（LOOP ONLY）
/// Phase 184: Loop/If 分離を明文化（If は JOINIR_IF_TARGETS へ）
///
/// **重要**: このテーブルは LOOP lowering 専用です。
/// If lowering の関数を追加すると、is_loop_lowered_function() で除外され、
/// if-lowering が機能しなくなります。If 関数は JOINIR_IF_TARGETS で管理。
///
/// | 関数 | Kind | Bridge default | 備考 |
/// |-----|------|----------------|------|
/// | Main.skip/1 | Exec | No | PHI canary のため env 必須 |
/// | FuncScannerBox.trim/1 | Exec | No | A/B 実証済み。ただし VM bridge は env 必須 |
/// | FuncScannerBox.append_defs/2 | LowerOnly | No | Case-A loop lowering target。bridge exec route なし |
/// | Stage1UsingResolverBox.resolve_for_source/5 | LowerOnly | No | lowering/構造検証のみ。実行は VM Route A |
/// | StageBBodyExtractorBox.build_body_src/2 | LowerOnly | No | lowering/構造検証のみ。実行は VM Route A |
/// | StageBFuncScannerBox.scan_all_boxes/1 | LowerOnly | No | lowering/構造検証のみ。実行は VM Route A |
///
/// Phase 181/182 設計ドキュメント:
/// - docs/private/roadmap2/phases/phase-181/joinir-targets-mapping.md
/// - docs/private/roadmap2/phases/phase-181/representative-paths-finalized.md
/// - docs/private/roadmap2/phases/phase-182/FINDINGS.md (Phase 182 実装時発見事項)
pub const JOINIR_TARGETS: &[JoinIrTargetDesc] = &[
    // Loop Exec（実行対応）
    JoinIrTargetDesc {
        func_name: "Main.skip/1",
        kind: JoinIrBridgeKind::Exec,
        default_enabled: false, // PHI canary のため env 必須
    },
    JoinIrTargetDesc {
        func_name: "FuncScannerBox.trim/1",
        kind: JoinIrBridgeKind::Exec,
        default_enabled: false, // VM bridge は env 必須
    },
    JoinIrTargetDesc {
        func_name: "FuncScannerBox.append_defs/2",
        kind: JoinIrBridgeKind::LowerOnly,
        default_enabled: false,
    },
    // Stage-1/Stage-B infrastructure: lowering/structure verification only.
    JoinIrTargetDesc {
        func_name: "Stage1UsingResolverBox.resolve_for_source/5",
        kind: JoinIrBridgeKind::LowerOnly,
        default_enabled: false,
    },
    JoinIrTargetDesc {
        func_name: "StageBBodyExtractorBox.build_body_src/2",
        kind: JoinIrBridgeKind::LowerOnly,
        default_enabled: false,
    },
    JoinIrTargetDesc {
        func_name: "StageBFuncScannerBox.scan_all_boxes/1",
        kind: JoinIrBridgeKind::LowerOnly,
        default_enabled: false,
    },
];

/// Phase 32 L-4: テーブルから対象関数を探す
pub(crate) fn find_joinir_target(module: &MirModule) -> Option<&'static JoinIrTargetDesc> {
    JOINIR_TARGETS
        .iter()
        .find(|target| module.functions.contains_key(target.func_name))
}

// ============================================================================
// Phase 184: JoinIR If Lowering Targets (Separate from Loop Targets)
// ============================================================================

/// JoinIR If lowering 対象テーブル（SSOT）
///
/// Phase 184: Loop lowering（JOINIR_TARGETS）と分離した If lowering 専用テーブル。
/// Phase 182 で判明した設計制約（JOINIR_TARGETS は Loop 専用）に基づく責務分離。
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
        kind: JoinIrBridgeKind::Exec,
        default_enabled: true, // Simple return pattern (Phase 33-2/33-3)
    },
    JoinIrTargetDesc {
        func_name: "IfSelectLocalTest.main/0",
        kind: JoinIrBridgeKind::Exec,
        default_enabled: true, // Local variable pattern (Phase 33-10)
    },
    JoinIrTargetDesc {
        func_name: "IfMergeTest.simple_true/0",
        kind: JoinIrBridgeKind::Exec,
        default_enabled: true, // Multiple variables (Phase 33-7)
    },
    JoinIrTargetDesc {
        func_name: "IfMergeTest.simple_false/0",
        kind: JoinIrBridgeKind::Exec,
        default_enabled: true, // Multiple variables (Phase 33-7)
    },
    // Selfhost/Production functions (Phase 33-4 explicit approvals)
    JoinIrTargetDesc {
        func_name: "JsonShapeToMap._read_value_from_pair/1",
        kind: JoinIrBridgeKind::Exec,
        default_enabled: true, // Stage-1 実用関数
    },
    JoinIrTargetDesc {
        func_name: "Stage1JsonScannerBox.value_start_after_key_pos/2",
        kind: JoinIrBridgeKind::Exec,
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
    fn loop_bridge_non_exec_rows_are_marked_lower_only() {
        for func_name in [
            "FuncScannerBox.append_defs/2",
            "Stage1UsingResolverBox.resolve_for_source/5",
            "StageBBodyExtractorBox.build_body_src/2",
            "StageBFuncScannerBox.scan_all_boxes/1",
        ] {
            let target = JOINIR_TARGETS
                .iter()
                .find(|target| target.func_name == func_name)
                .expect("target row exists");
            assert_eq!(target.kind, JoinIrBridgeKind::LowerOnly);
            assert!(!target.default_enabled);
        }
    }
}
