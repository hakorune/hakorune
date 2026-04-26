//! Phase 33-23: LoopViewBuilder - Loop lowering ディスパッチ箱
//!
//! LoopToJoinLowererからlowering選択責務を分離した専用モジュール。
//!
//! ## 責務
//!
//! - **Route検出**: LoopSimpleWhile route 検出
//! - **Shape検出**: CaseALoweringShape検出
//! - **Lowerer選択**: Shape/名前ベースでlowerer選択
//! - **Lowerer呼び出し**: 適切なlowererに委譲
//!
//! ## 設計思想
//!
//! - **単一責任**: Lowering選択ロジックのみを集約
//! - **拡張性**: 新しいroute追加が容易
//! - **テスト容易性**: 独立したBoxで単体テスト可能

use crate::mir::join_ir::lowering::canonical_names;
use crate::mir::join_ir::lowering::generic_case_a;
use crate::mir::join_ir::lowering::loop_scope_shape::{
    find_case_a_minimal_target, CaseALoweringShape, CaseAMinimalTargetKind, LoopScopeShape,
};
use crate::mir::join_ir::JoinModule;
use crate::mir::naming::StaticMethodId;
use crate::runtime::get_global_ring0;

fn is_simple_while_main_route_candidate(name: &str) -> bool {
    if name == canonical_names::MAIN {
        return true;
    }

    StaticMethodId::parse(name)
        .map(|id| id.method == canonical_names::MAIN)
        .unwrap_or(false)
}

/// Loop lowering ディスパッチ箱
///
/// LoopScopeShapeからrouteを検出し、適切なlowererを選択する。
pub struct LoopViewBuilder {
    /// デバッグモード（詳細ログ出力）
    debug: bool,
}

impl Default for LoopViewBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl LoopViewBuilder {
    /// 新しいLoopViewBuilderを作成
    pub fn new() -> Self {
        let debug = std::env::var("NYASH_LOOPTOJOIN_DEBUG")
            .map(|v| v == "1")
            .unwrap_or(false);
        Self { debug }
    }

    /// LoopScopeShapeからJoinModuleを生成
    ///
    /// # 選択戦略（Phase 170-A-2: Structure-based + Name fallback）
    ///
    /// 1. LoopSimpleWhile route検出
    /// 2. Shape検出（CaseALoweringShape）
    /// 3. Shape別ディスパッチ
    /// 4. 名前ベースフォールバック（Generic/NotCaseAの場合）
    ///
    /// # Arguments
    ///
    /// - `scope`: LoopScopeShape（変数分類・構造情報）
    /// - `func_name`: 関数名（名前ベースフォールバック用）
    ///
    /// # Returns
    ///
    /// - `Some(JoinModule)`: Lowering成功
    /// - `None`: 未サポート形（フォールバック経路へ）
    pub fn build(&self, scope: LoopScopeShape, func_name: Option<&str>) -> Option<JoinModule> {
        let name = func_name.unwrap_or("");

        // Phase 188-Impl-1: LoopSimpleWhile route detection
        // Try LoopSimpleWhile route FIRST for main function (loop_min_while.hako target)
        if let Some(result) = self.try_loop_simple_while(&scope, name) {
            return Some(result);
        }

        // Phase 170-A-2: Structure-based routing with CaseALoweringShape.
        // Name-only carrier data is not an update-kind proof.
        let carrier_count = scope.carriers.len();
        let stub_features = crate::mir::loop_route_detection::LoopFeatures {
            carrier_count,
            ..Default::default()
        };

        let has_progress_carrier = scope.progress_carrier.is_some();

        let shape = CaseALoweringShape::detect_from_features(
            &stub_features,
            carrier_count,
            has_progress_carrier,
        );

        if self.debug {
            get_global_ring0().log.debug(&format!(
                "[LoopViewBuilder] Phase 170-C-2b: shape={:?}, name={:?}, carriers={:?}",
                shape.name(),
                name,
                scope.carriers_ordered()
            ));
        }

        // Shape-based dispatch
        self.dispatch_by_shape(shape, scope, name)
    }

    /// LoopSimpleWhile route 検出・lowering試行
    ///
    /// # 検出条件
    ///
    /// - canonical JoinIR `main` or a static method whose method name is `main`
    /// - Pinned vars がない
    /// - Carriers が1つ以上
    ///
    /// # Phase 202-A: JoinValueSpace Integration
    fn try_loop_simple_while(&self, scope: &LoopScopeShape, name: &str) -> Option<JoinModule> {
        if !is_simple_while_main_route_candidate(name) {
            return None;
        }

        if scope.pinned.is_empty() && !scope.carriers.is_empty() {
            if self.debug {
                get_global_ring0().log.debug(&format!(
                    "[LoopViewBuilder] Trying LoopSimpleWhile route lowering for {:?}",
                    name
                ));
            }

            // Phase 202-A: Create JoinValueSpace for LoopSimpleWhile route
            use super::join_value_space::JoinValueSpace;
            let mut join_value_space = JoinValueSpace::new();

            if let Some(result) = super::simple_while_minimal::lower_simple_while_minimal(
                scope.clone(),
                &mut join_value_space,
            ) {
                if self.debug {
                    get_global_ring0().log.debug(&format!(
                        "[LoopViewBuilder] LoopSimpleWhile route lowering succeeded for {:?}",
                        name
                    ));
                }
                return Some(result);
            }

            if self.debug {
                get_global_ring0()
                    .log
                    .debug(
                        "[LoopViewBuilder] LoopSimpleWhile route lowering failed, trying other lowerers",
                    );
            }
        }

        None
    }

    /// Shape別にlowererをディスパッチ
    ///
    /// # Shape種別
    ///
    /// - **StringExamination**: skip_ws lowerer
    /// - **ArrayAccumulation**: append_defs lowerer
    /// - **IterationWithAccumulation**: stage1 lowerer
    /// - **Generic/NotCaseA**: 名前ベースフォールバック
    fn dispatch_by_shape(
        &self,
        shape: CaseALoweringShape,
        scope: LoopScopeShape,
        name: &str,
    ) -> Option<JoinModule> {
        match shape {
            CaseALoweringShape::StringExamination => {
                if self.debug {
                    get_global_ring0()
                        .log
                        .debug("[LoopViewBuilder] Shape: StringExamination → skip_ws lowerer");
                }
                generic_case_a::lower_case_a_skip_ws_with_scope(scope)
            }
            CaseALoweringShape::ArrayAccumulation => {
                if self.debug {
                    get_global_ring0()
                        .log
                        .debug("[LoopViewBuilder] Shape: ArrayAccumulation → append_defs lowerer");
                }
                generic_case_a::lower_case_a_append_defs_with_scope(scope)
            }
            CaseALoweringShape::IterationWithAccumulation => {
                if self.debug {
                    get_global_ring0().log.debug(
                        "[LoopViewBuilder] Shape: IterationWithAccumulation → stage1 lowerer",
                    );
                }
                generic_case_a::lower_case_a_stage1_usingresolver_with_scope(scope)
            }
            CaseALoweringShape::Generic | CaseALoweringShape::NotCaseA => {
                if self.debug {
                    get_global_ring0().log.debug(&format!(
                        "[LoopViewBuilder] Shape: {:?} → name-based fallback",
                        shape.name()
                    ));
                }
                self.dispatch_by_name(scope, name)
            }
        }
    }

    /// 名前ベースフォールバック（Legacy）
    ///
    /// # Phase 170-A-2 設計
    ///
    /// Shape検出で未分類のループを名前で振り分ける。
    /// 将来的にはShape検出を強化してこのフォールバックを削減する。
    fn dispatch_by_name(&self, scope: LoopScopeShape, name: &str) -> Option<JoinModule> {
        let Some(target) = find_case_a_minimal_target(name) else {
            // No shape match AND no whitelist match
            if self.debug && self.generic_case_a_enabled() {
                get_global_ring0().log.debug(&format!(
                    "[LoopViewBuilder] generic Case-A candidate: {:?} (no lowerer yet)",
                    name
                ));
            }
            return None;
        };

        match target.kind {
            CaseAMinimalTargetKind::SkipWhitespace => {
                if self.debug {
                    get_global_ring0()
                        .log
                        .debug("[LoopViewBuilder] [fallback] dispatching to skip_ws lowerer");
                }
                generic_case_a::lower_case_a_skip_ws_with_scope(scope)
            }
            CaseAMinimalTargetKind::Trim => {
                if self.debug {
                    get_global_ring0()
                        .log
                        .debug("[LoopViewBuilder] [fallback] dispatching to trim lowerer");
                }
                generic_case_a::lower_case_a_trim_with_scope(scope)
            }
            CaseAMinimalTargetKind::AppendDefs => {
                if self.debug {
                    get_global_ring0()
                        .log
                        .debug("[LoopViewBuilder] [fallback] dispatching to append_defs lowerer");
                }
                generic_case_a::lower_case_a_append_defs_with_scope(scope)
            }
            CaseAMinimalTargetKind::Stage1UsingResolver => {
                if self.debug {
                    get_global_ring0()
                        .log
                        .debug("[LoopViewBuilder] [fallback] dispatching to stage1 lowerer");
                }
                generic_case_a::lower_case_a_stage1_usingresolver_with_scope(scope)
            }
        }
    }

    /// Phase 32 L-1.2: 汎用Case-A loweringが有効かどうか
    fn generic_case_a_enabled(&self) -> bool {
        crate::mir::join_ir::env_flag_is_1("NYASH_JOINIR_LOWER_GENERIC")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_creation() {
        let builder = LoopViewBuilder::new();
        assert!(!builder.debug || builder.debug); // Just check it compiles
    }

    #[test]
    fn simple_while_main_route_gate_uses_static_method_identity() {
        for name in ["main", "Main.main/0", "Main.main/1", "main.main/0"] {
            assert!(is_simple_while_main_route_candidate(name));
        }

        for name in [
            "",
            "domain_loop/0",
            "Main.remaining/0",
            "Main.not_main/0",
            "FuncScannerBox.trim/1",
        ] {
            assert!(!is_simple_while_main_route_candidate(name));
        }
    }
}
