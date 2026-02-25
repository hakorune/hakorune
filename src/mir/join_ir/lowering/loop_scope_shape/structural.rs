//! LoopStructuralAnalysis - ループの構造的性質を解析する箱
//!
//! # Phase 48-5.5: 箱化モジュール化
//!
//! Case-A/B/C 判定などの構造解析を統一的に扱う。
//! ExitAnalysis と LoopScopeShape を組み合わせて、ループの構造的性質を判定する。
//!
//! ## 責務
//!
//! - ループの構造的性質の解析（出口数、非局所 exit、progress_carrier など）
//! - Case-A minimal 判定（1出口 & 非局所なし & progress_carrier あり）
//! - 将来の Case-B/C 拡張の基盤
//!
//! ## 設計原則（箱理論）
//!
//! - **単一責務**: ループの構造解析のみ
//! - **質問箱**: is_case_a_minimal() で判定結果を提供
//! - **依存最小**: ExitAnalysis と LoopScopeShape のみに依存

use crate::mir::control_form::{analyze_exits, ExitAnalysis, LoopId};
use crate::mir::loop_form::LoopForm;

use super::shape::LoopScopeShape;

/// ループの構造的性質を解析した結果
///
/// # Phase 48-5.5: 箱化の利点
///
/// - ExitAnalysis と progress_carrier を組み合わせた判定を一箇所に集約
/// - Case-A/B/C 判定の拡張が容易
/// - テスト容易（ExitAnalysis + progress_carrier を渡すだけ）
#[derive(Debug, Clone)]
pub struct LoopStructuralAnalysis {
    /// 出口辺の解析結果
    exit_analysis: ExitAnalysis,
    /// progress_carrier の有無
    has_progress_carrier: bool,
}

impl LoopStructuralAnalysis {
    /// LoopForm と LoopScopeShape から構造解析を生成
    ///
    /// # Example
    ///
    /// ```ignore
    /// let analysis = LoopStructuralAnalysis::from_loop_scope(loop_form, scope);
    /// if analysis.is_case_a_minimal() {
    ///     // Case-A ループとして処理
    /// }
    /// ```
    pub fn from_loop_scope(loop_form: &LoopForm, scope: &LoopScopeShape) -> Self {
        let loop_id = LoopId(0);
        let exit_edges = loop_form.to_exit_edges(loop_id);
        let exit_analysis = analyze_exits(&exit_edges);

        Self {
            exit_analysis,
            has_progress_carrier: scope.progress_carrier.is_some(),
        }
    }

    /// Case-A minimal: 1出口 & 非局所なし & progress_carrier あり
    ///
    /// # 必須条件
    ///
    /// 1. ループ外出口が 1 グループのみ（`is_single_exit_group()`）
    /// 2. 非局所 exit がない（Return/Throw なし）
    /// 3. progress_carrier が存在する
    ///
    /// # Phase 48-5: 構造ベース判定
    ///
    /// 従来の関数名ハードコードに依存せず、ループの構造的性質のみで判定する。
    pub fn is_case_a_minimal(&self) -> bool {
        self.exit_analysis.is_single_exit_group() && self.has_progress_carrier
    }

    /// ExitAnalysis への参照を取得（詳細情報アクセス用）
    ///
    /// # 用途
    ///
    /// - 出口グループ数の確認
    /// - 非局所 exit の有無確認
    /// - 出口先ブロックの取得
    #[allow(dead_code)]
    pub fn exit_analysis(&self) -> &ExitAnalysis {
        &self.exit_analysis
    }

    /// progress_carrier の有無
    #[allow(dead_code)]
    pub fn has_progress_carrier(&self) -> bool {
        self.has_progress_carrier
    }

    // 将来の拡張用（Phase 48-6+ で実装予定）
    //
    // /// Case-B: 複数出口 & 非局所なし
    // pub fn is_case_b_multiple_exits(&self) -> bool {
    //     self.exit_analysis.loop_exit_groups.len() > 1
    //         && self.exit_analysis.nonlocal_exits.is_empty()
    // }
    //
    // /// Case-C: 非局所 exit あり
    // pub fn is_case_c_nonlocal(&self) -> bool {
    //     !self.exit_analysis.nonlocal_exits.is_empty()
    // }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::control_form::{ExitEdge, ExitKind, LoopId};
    use crate::mir::BasicBlockId;
    use std::collections::{BTreeMap, BTreeSet};

    fn make_single_exit_analysis() -> ExitAnalysis {
        let exit_edges = vec![ExitEdge {
            id: crate::mir::control_form::ExitEdgeId(0),
            loop_id: LoopId(0),
            from: BasicBlockId::new(4),
            to: BasicBlockId::new(100),
            kind: ExitKind::ConditionFalse,
        }];
        analyze_exits(&exit_edges)
    }

    fn make_multiple_exit_analysis() -> ExitAnalysis {
        let exit_edges = vec![
            ExitEdge {
                id: crate::mir::control_form::ExitEdgeId(0),
                loop_id: LoopId(0),
                from: BasicBlockId::new(4),
                to: BasicBlockId::new(100),
                kind: ExitKind::ConditionFalse,
            },
            ExitEdge {
                id: crate::mir::control_form::ExitEdgeId(1),
                loop_id: LoopId(0),
                from: BasicBlockId::new(5),
                to: BasicBlockId::new(101),
                kind: ExitKind::Break { label: None },
            },
        ];
        analyze_exits(&exit_edges)
    }

    fn make_scope_with_carrier() -> LoopScopeShape {
        LoopScopeShape {
            header: BasicBlockId::new(2),
            body: BasicBlockId::new(3),
            latch: BasicBlockId::new(4),
            exit: BasicBlockId::new(100),
            pinned: vec!["s".to_string()].into_iter().collect(),
            carriers: vec!["i".to_string()].into_iter().collect(),
            body_locals: BTreeSet::new(),
            exit_live: vec!["s".to_string(), "i".to_string()].into_iter().collect(),
            progress_carrier: Some("i".to_string()),
            variable_definitions: BTreeMap::new(),
        }
    }

    fn make_scope_without_carrier() -> LoopScopeShape {
        LoopScopeShape {
            header: BasicBlockId::new(2),
            body: BasicBlockId::new(3),
            latch: BasicBlockId::new(4),
            exit: BasicBlockId::new(100),
            pinned: vec!["s".to_string()].into_iter().collect(),
            carriers: BTreeSet::new(),
            body_locals: BTreeSet::new(),
            exit_live: vec!["s".to_string()].into_iter().collect(),
            progress_carrier: None,
            variable_definitions: BTreeMap::new(),
        }
    }

    #[test]
    fn test_case_a_minimal_positive() {
        // 1出口 & 非局所なし & progress_carrier あり → Case-A
        let exit_analysis = make_single_exit_analysis();
        let scope = make_scope_with_carrier();

        let analysis = LoopStructuralAnalysis {
            exit_analysis,
            has_progress_carrier: scope.progress_carrier.is_some(),
        };

        assert!(analysis.is_case_a_minimal());
        assert!(analysis.has_progress_carrier());
    }

    #[test]
    fn test_case_a_minimal_no_carrier() {
        // 1出口 & 非局所なし だが progress_carrier なし → Case-A ではない
        let exit_analysis = make_single_exit_analysis();
        let scope = make_scope_without_carrier();

        let analysis = LoopStructuralAnalysis {
            exit_analysis,
            has_progress_carrier: scope.progress_carrier.is_some(),
        };

        assert!(!analysis.is_case_a_minimal());
        assert!(!analysis.has_progress_carrier());
    }

    #[test]
    fn test_case_a_minimal_multiple_exits() {
        // 複数出口 → Case-A ではない
        let exit_analysis = make_multiple_exit_analysis();
        let scope = make_scope_with_carrier();

        let analysis = LoopStructuralAnalysis {
            exit_analysis,
            has_progress_carrier: scope.progress_carrier.is_some(),
        };

        assert!(!analysis.is_case_a_minimal());
        assert!(analysis.has_progress_carrier());
    }

    #[test]
    fn test_exit_analysis_access() {
        let exit_analysis = make_single_exit_analysis();
        let scope = make_scope_with_carrier();

        let analysis = LoopStructuralAnalysis {
            exit_analysis,
            has_progress_carrier: scope.progress_carrier.is_some(),
        };

        // ExitAnalysis へのアクセス確認
        assert!(analysis.exit_analysis().is_single_exit_group());
        assert_eq!(analysis.exit_analysis().loop_exit_groups.len(), 1);
        assert_eq!(analysis.exit_analysis().nonlocal_exits.len(), 0);
    }
}
