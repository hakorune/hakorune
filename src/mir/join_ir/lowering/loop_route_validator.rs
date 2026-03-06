//! Phase 33-23: LoopRouteValidator - Loop構造検証箱
//!
//! LoopToJoinLowererから検証責務を分離した専用モジュール。
//!
//! ## 責務
//!
//! - **Exit構造検証**: 単一出口グループ + 非ローカル出口なし
//! - **Header構造検証**: Header successorが2つ（cond true/false）
//! - **Progress carrier検証**: 無限ループ防止チェック
//!
//! ## 設計思想
//!
//! - **単一責任**: 検証ロジックのみを集約
//! - **再利用性**: LoopToJoinLowerer以外からも利用可能
//! - **テスト容易性**: 独立したBoxで単体テスト可能

use crate::mir::control_form::{ExitEdge, LoopRegion};
use crate::mir::join_ir::lowering::loop_scope_shape::LoopScopeShape;
use crate::mir::MirFunction;
use crate::runtime::get_global_ring0;

/// Loop構造検証箱
///
/// LoopFormがCase-A loweringに適しているかを検証する。
pub struct LoopRouteValidator {
    /// デバッグモード（詳細ログ出力）
    debug: bool,
}

impl Default for LoopRouteValidator {
    fn default() -> Self {
        Self::new()
    }
}

impl LoopRouteValidator {
    /// 新しいLoopRouteValidatorを作成
    pub fn new() -> Self {
        let debug = std::env::var("NYASH_LOOPTOJOIN_DEBUG")
            .map(|v| v == "1")
            .unwrap_or(false);
        Self { debug }
    }

    /// Case-Aループとしてサポートされているかチェック
    ///
    /// # Case-Aの定義
    ///
    /// - **単一出口グループ**: 全ての出口辺が同じターゲットブロックに向かう
    /// - **非ローカル出口なし**: Return/Throwがない
    /// - ヘッダブロックのsuccessorが2つ（cond true/false）
    /// - ループ変数または固定変数が存在
    /// - Progress carrier あり（無限ループ防止）
    ///
    /// # Arguments
    ///
    /// - `func`: MIR関数（ヘッダsuccessorチェック用）
    /// - `region`: LoopRegion（ブロック構造）
    /// - `exit_edges`: ExitEdgeのリスト（グループ化分析用）
    /// - `scope`: LoopScopeShape（変数分類用）
    ///
    /// # Returns
    ///
    /// - `true`: Case-Aとしてlowering可能
    /// - `false`: 未サポート（フォールバック経路へ）
    pub fn is_supported_case_a(
        &self,
        func: &MirFunction,
        region: &LoopRegion,
        exit_edges: &[ExitEdge],
        scope: &LoopScopeShape,
    ) -> bool {
        // 1. Exit構造検証
        if !self.validate_exit_structure(exit_edges) {
            return false;
        }

        // 2. Header構造検証
        if !self.validate_header_structure(func, region) {
            return false;
        }

        // 3. Variable存在検証
        if !self.validate_variables_exist(scope) {
            return false;
        }

        // 4. Progress carrier検証
        if !self.validate_progress_carrier(scope, func, region) {
            return false;
        }

        true
    }

    /// Exit構造を検証
    ///
    /// # 検証項目
    ///
    /// - 単一出口グループ（複数ExitEdgeでも同じターゲットならOK）
    /// - 非ローカル出口なし（Return/Throw禁止）
    fn validate_exit_structure(&self, exit_edges: &[ExitEdge]) -> bool {
        use crate::mir::control_form::analyze_exits;

        let exit_analysis = analyze_exits(exit_edges);

        if !exit_analysis.is_single_exit_group() {
            if self.debug {
                let ring0 = get_global_ring0();
                ring0.log.debug(&format!(
                    "[LoopRouteValidator] rejected: not single exit group (groups={}, nonlocal={})",
                    exit_analysis.loop_exit_groups.len(),
                    exit_analysis.nonlocal_exits.len()
                ));
                // 詳細ログ: 各グループのターゲットを出力
                for (i, group) in exit_analysis.loop_exit_groups.iter().enumerate() {
                    ring0.log.debug(&format!(
                        "  group[{}]: target={:?}, edges={}, has_break={}",
                        i,
                        group.target,
                        group.edges.len(),
                        group.has_break
                    ));
                }
            }
            return false;
        }

        true
    }

    /// Header構造を検証
    ///
    /// # 検証項目
    ///
    /// - Headerブロックのsuccessorが2つ（cond true → body, cond false → exit）
    fn validate_header_structure(&self, func: &MirFunction, region: &LoopRegion) -> bool {
        if let Some(header_block) = func.blocks.get(&region.header) {
            let succ_count = header_block.successors.len();
            if succ_count != 2 {
                if self.debug {
                    get_global_ring0().log.debug(&format!(
                        "[LoopRouteValidator] rejected: header {:?} has {} successors (expected 2)",
                        region.header, succ_count
                    ));
                }
                return false;
            }
        } else {
            // ヘッダブロックが見つからない（異常ケース）
            if self.debug {
                get_global_ring0().log.debug(&format!(
                    "[LoopRouteValidator] rejected: header block {:?} not found",
                    region.header
                ));
            }
            return false;
        }

        true
    }

    /// ループ変数または固定変数の存在を検証
    ///
    /// # 検証項目
    ///
    /// - Carriers または Pinned vars が1つ以上存在（空ループ対象外）
    fn validate_variables_exist(&self, scope: &LoopScopeShape) -> bool {
        if scope.carriers.is_empty() && scope.pinned.is_empty() {
            if self.debug {
                get_global_ring0()
                    .log
                    .debug("[LoopRouteValidator] rejected: no carriers or pinned vars");
            }
            return false;
        }

        true
    }

    /// Progress carrierの安全性をチェック
    ///
    /// # 検証項目
    ///
    /// - Progress carrier が設定されている（無限ループ防止）
    ///
    /// # Phase 1実装（保守的）
    ///
    /// - `scope.progress_carrier.is_some()` をチェック
    /// - progress_carrierが設定されていればループは進捗すると仮定
    ///
    /// # Phase 2 (future)
    ///
    /// - MirQueryでheader→latch間にAdd命令があるかチェック
    /// - skip_ws verifierのロジックをMIRレベルで簡略化して適用
    fn validate_progress_carrier(
        &self,
        scope: &LoopScopeShape,
        _func: &MirFunction,  // Phase 2で使用予定
        _region: &LoopRegion, // Phase 2で使用予定
    ) -> bool {
        // Phase 1: 保守的チェック
        // progress_carrierが設定されていれば、ループは進捗すると仮定
        // （典型的には'i'のようなloop index）
        if scope.progress_carrier.is_none() {
            if self.debug {
                get_global_ring0().log.debug(&format!(
                    "[LoopRouteValidator] rejected: no safe progress carrier (progress_carrier={:?})",
                    scope.progress_carrier
                ));
            }
            return false;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validator_creation() {
        let validator = LoopRouteValidator::new();
        assert!(!validator.debug || validator.debug); // Just check it compiles
    }
}
