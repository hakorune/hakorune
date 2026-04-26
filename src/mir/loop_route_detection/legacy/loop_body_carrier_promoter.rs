//! Phase 171-C: LoopBodyCarrierPromoter Box
//!
//! body-local 変数を carrier に昇格させることで、
//! `loop_break` / `loop_continue_only` route で処理可能にするための箱。
//!
//! ## Design Philosophy
//!
//! - 入力: LoopScopeShape + LoopConditionScope + break 条件 AST
//! - 出力: 昇格成功なら CarrierInfo、失敗なら理由
//! - 役割: body-local を「評価済み bool carrier」に変換
//!
//! ## Implementation Scope
//!
//! ### Phase 171-C-1: スケルトン実装 ✅
//! - body-local の検出
//! - 定義の探索
//!
//! ### Phase 171-C-2: Trim route 昇格 ✅
//! - `local ch = s.substring(...)` route shape 検出
//! - `ch == " " || ch == "\t" ...` の等価比較検出
//! - `is_whitespace` bool carrier への変換情報生成

use crate::ast::ASTNode;
use crate::mir::join_ir::lowering::loop_scope_shape::LoopScopeShape;
use crate::mir::loop_route_detection::loop_condition_scope::LoopConditionScope;

/// 昇格リクエスト
pub struct PromotionRequest<'a> {
    /// ループのスコープ情報
    #[allow(dead_code)]
    pub(crate) scope: &'a LoopScopeShape,
    /// 条件変数のスコープ分類
    pub cond_scope: &'a LoopConditionScope,
    /// break 条件の AST（`loop_break` route の場合）
    pub break_cond: Option<&'a ASTNode>,
    /// ループ本体の AST
    pub loop_body: &'a [ASTNode],
}

/// Phase 171-C-2: 検出された Trim route 情報
#[derive(Debug, Clone)]
pub struct TrimRouteInfo {
    /// body-local 変数名（例: "ch"）
    pub var_name: String,
    /// 比較対象の文字列リテラル（例: [" ", "\t", "\n", "\r"]）
    pub comparison_literals: Vec<String>,
    /// 生成する carrier 名（例: "is_whitespace"）
    pub carrier_name: String,
}

impl TrimRouteInfo {
    /// Phase 171-C-4: Convert to CarrierInfo with a bool carrier for the route
    ///
    /// Creates a CarrierInfo containing a single bool carrier representing
    /// the Trim route match condition (e.g., "is_whitespace").
    ///
    /// # Arguments
    ///
    /// # Returns
    ///
    /// CarrierInfo with:
    /// - loop_var_name: The promoted carrier name (e.g., "is_ch_match")
    /// - loop_var_id: Placeholder ValueId(0) (will be remapped by JoinInlineBoundary)
    /// - carriers: Empty (the carrier itself is the loop variable)
    ///
    /// # Design Note
    ///
    /// The returned CarrierInfo uses a placeholder ValueId(0) because:
    /// - This is JoinIR-local ID space (not host ValueId space)
    /// - The actual host ValueId will be assigned during merge_joinir_mir_blocks
    /// - JoinInlineBoundary will handle the boundary mapping
    pub fn to_carrier_info(&self) -> crate::mir::join_ir::lowering::carrier_info::CarrierInfo {
        use super::trim_loop_helper::TrimLoopHelper;
        use crate::mir::join_ir::lowering::carrier_info::CarrierInfo;
        use crate::mir::ValueId;

        // Phase 171-C-4/5: Create CarrierInfo with promoted carrier as loop variable
        // and attach TrimLoopHelper for future lowering
        let mut carrier_info = CarrierInfo::with_carriers(
            self.carrier_name.clone(), // "is_ch_match" becomes the loop variable
            ValueId(0),                // Placeholder (will be remapped)
            vec![],                    // No additional carriers
        );

        // Phase 171-C-5: Attach TrimLoopHelper for trim-route lowering logic
        carrier_info.trim_helper = Some(TrimLoopHelper::from_route_info(self));

        // Phase 229: Record promoted variable (no need for condition_aliases)
        // Dynamic resolution uses promoted_body_locals + naming convention
        carrier_info
            .promoted_body_locals
            .push(self.var_name.clone());

        carrier_info
    }
}

/// 昇格結果
pub enum PromotionResult {
    /// 昇格成功: Trim route 情報を返す
    ///
    /// Phase 171-C-2: CarrierInfo の実際の更新は Phase 171-C-3 で実装
    Promoted {
        /// Phase 171-C-2: 検出された Trim route 情報
        trim_info: TrimRouteInfo,
    },

    /// 昇格不可: 理由を説明
    CannotPromote {
        reason: String,
        vars: Vec<String>, // 問題の body-local 変数
    },
}

/// Phase 171-C: LoopBodyCarrierPromoter Box
pub struct LoopBodyCarrierPromoter;

impl LoopBodyCarrierPromoter {
    /// body-local 変数を carrier に昇格できるか試行
    ///
    /// # Phase 171-C-2: Trim route 実装
    /// # Phase 79: Simplified using TrimDetector
    ///
    /// 現在の実装では:
    /// 1. body-local 変数を抽出
    /// 2. TrimDetector で純粋な検出ロジックを実行
    /// 3. 昇格可能なら TrimRouteInfo を返す
    pub fn try_promote(request: &PromotionRequest) -> PromotionResult {
        use super::trim_detector::TrimDetector;
        use crate::mir::loop_route_detection::loop_condition_scope::CondVarScope;

        // 1. body-local 変数を抽出
        let body_locals: Vec<&String> = request
            .cond_scope
            .vars
            .iter()
            .filter(|v| v.scope == CondVarScope::LoopBodyLocal)
            .map(|v| &v.name)
            .collect();

        if body_locals.is_empty() {
            // body-local 変数がなければ昇格不要
            return PromotionResult::CannotPromote {
                reason: "No LoopBodyLocal variables to promote".to_string(),
                vars: vec![],
            };
        }

        use crate::config::env::is_joinir_debug;
        if is_joinir_debug() || std::env::var("JOINIR_TEST_DEBUG").is_ok() {
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.debug(&format!(
                "[promoter/trim] Phase 171-C: Found {} body-local variables: {:?}",
                body_locals.len(),
                body_locals
            ));
        }

        // 2. break 条件を取得
        if request.break_cond.is_none() {
            return PromotionResult::CannotPromote {
                reason: "No break condition provided".to_string(),
                vars: body_locals.iter().map(|s| s.to_string()).collect(),
            };
        }

        let break_cond = request.break_cond.unwrap();

        // 3. 各 body-local 変数に対して TrimDetector で検出を試行
        for var_name in &body_locals {
            // Phase 79: Use TrimDetector for pure detection logic
            if let Some(detection) = TrimDetector::detect(break_cond, request.loop_body, var_name) {
                if is_joinir_debug() || std::env::var("JOINIR_TEST_DEBUG").is_ok() {
                    let ring0 = crate::runtime::get_global_ring0();
                    ring0.log.debug(&format!(
                        "[promoter/trim] trim route detected: var='{}', literals={:?}",
                        detection.match_var, detection.comparison_literals
                    ));
                }

                // 昇格成功！
                let trim_info = TrimRouteInfo {
                    var_name: detection.match_var,
                    comparison_literals: detection.comparison_literals,
                    carrier_name: detection.carrier_name,
                };

                // Phase 171-C-2: TrimRouteInfo を返す
                return PromotionResult::Promoted { trim_info };
            }
        }

        // 昇格 route に一致しない
        PromotionResult::CannotPromote {
            reason: "No promotable Trim route detected".to_string(),
            vars: body_locals.iter().map(|s| s.to_string()).collect(),
        }
    }

    // Phase 79: Helper methods removed - now in TrimDetector
    // - find_definition_in_body
    // - is_substring_method_call
    // - extract_equality_literals
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{BinaryOperator, LiteralValue, Span};
    use crate::mir::loop_route_detection::loop_condition_scope::{
        CondVarScope, LoopConditionScope,
    };
    use crate::mir::BasicBlockId;
    use std::collections::{BTreeMap, BTreeSet};

    fn minimal_scope() -> LoopScopeShape {
        LoopScopeShape {
            header: BasicBlockId(0),
            body: BasicBlockId(1),
            latch: BasicBlockId(2),
            exit: BasicBlockId(3),
            pinned: BTreeSet::new(),
            carriers: BTreeSet::new(),
            body_locals: BTreeSet::new(),
            exit_live: BTreeSet::new(),
            progress_carrier: None,
            variable_definitions: BTreeMap::new(),
        }
    }

    fn cond_scope_with_body_local(var_name: &str) -> LoopConditionScope {
        let mut scope = LoopConditionScope::new();
        scope.add_var(var_name.to_string(), CondVarScope::LoopBodyLocal);
        scope
    }

    // Helper: Create a Variable node
    fn var_node(name: &str) -> ASTNode {
        ASTNode::Variable {
            name: name.to_string(),
            span: Span::unknown(),
        }
    }

    // Helper: Create a String literal node
    fn str_literal(s: &str) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::String(s.to_string()),
            span: Span::unknown(),
        }
    }

    // Helper: Create an equality comparison (var == literal)
    fn eq_cmp(var_name: &str, literal: &str) -> ASTNode {
        ASTNode::BinaryOp {
            operator: BinaryOperator::Equal,
            left: Box::new(var_node(var_name)),
            right: Box::new(str_literal(literal)),
            span: Span::unknown(),
        }
    }

    // Helper: Create an Or expression
    fn or_expr(left: ASTNode, right: ASTNode) -> ASTNode {
        ASTNode::BinaryOp {
            operator: BinaryOperator::Or,
            left: Box::new(left),
            right: Box::new(right),
            span: Span::unknown(),
        }
    }

    // Helper: Create a MethodCall node
    fn method_call(object: &str, method: &str) -> ASTNode {
        ASTNode::MethodCall {
            object: Box::new(var_node(object)),
            method: method.to_string(),
            arguments: vec![],
            span: Span::unknown(),
        }
    }

    // Helper: Create an Assignment node
    fn assignment(target: &str, value: ASTNode) -> ASTNode {
        ASTNode::Assignment {
            target: Box::new(var_node(target)),
            value: Box::new(value),
            span: Span::unknown(),
        }
    }

    #[test]
    fn test_promoter_no_body_locals() {
        let scope = minimal_scope();
        let cond_scope = LoopConditionScope::new(); // Empty, no LoopBodyLocal

        let request = PromotionRequest {
            scope: &scope,
            cond_scope: &cond_scope,
            break_cond: None,
            loop_body: &[],
        };

        let result = LoopBodyCarrierPromoter::try_promote(&request);

        match result {
            PromotionResult::CannotPromote { reason, vars } => {
                assert!(vars.is_empty());
                assert!(reason.contains("No LoopBodyLocal"));
            }
            _ => panic!("Expected CannotPromote when no LoopBodyLocal variables"),
        }
    }

    #[test]
    fn test_promoter_body_local_no_definition() {
        // body-local 変数があるが、定義が見つからない場合
        // Phase 79: Now checks for break_cond first, so we provide a break_cond
        // but empty body so detection fails
        let scope = minimal_scope();
        let cond_scope = cond_scope_with_body_local("ch");

        let break_cond = eq_cmp("ch", " "); // Provide condition but no matching definition

        let request = PromotionRequest {
            scope: &scope,
            cond_scope: &cond_scope,
            break_cond: Some(&break_cond),
            loop_body: &[], // Empty body - no definition
        };

        let result = LoopBodyCarrierPromoter::try_promote(&request);

        match result {
            PromotionResult::CannotPromote { reason, vars } => {
                assert!(vars.contains(&"ch".to_string()));
                assert!(reason.contains("No promotable Trim route"));
            }
            _ => panic!("Expected CannotPromote when definition not found"),
        }
    }

    // ========================================================================
    // Phase 171-C-2: Trim route detection tests
    // ========================================================================

    // Phase 79: Tests removed - these methods are now in TrimDetector
    // The detector has its own comprehensive test suite in trim_detector.rs
    // See trim_detector::tests for equivalent test coverage:
    // - test_find_definition_in_body_simple
    // - test_find_definition_in_body_nested_if
    // - test_is_substring_method_call
    // - test_extract_equality_literals_single
    // - test_extract_equality_literals_or_chain
    // - test_extract_equality_literals_wrong_var

    #[test]
    fn test_trim_pattern_full_detection() {
        // Full Trim route test:
        // - body-local: ch
        // - Definition: ch = s.substring(...)
        // - Break condition: ch == " " || ch == "\t"

        let scope = minimal_scope();
        let cond_scope = cond_scope_with_body_local("ch");

        let loop_body = vec![assignment("ch", method_call("s", "substring"))];

        let break_cond = or_expr(eq_cmp("ch", " "), eq_cmp("ch", "\t"));

        let request = PromotionRequest {
            scope: &scope,
            cond_scope: &cond_scope,
            break_cond: Some(&break_cond),
            loop_body: &loop_body,
        };

        let result = LoopBodyCarrierPromoter::try_promote(&request);

        match result {
            PromotionResult::Promoted { trim_info } => {
                assert_eq!(trim_info.var_name, "ch");
                assert_eq!(trim_info.comparison_literals.len(), 2);
                assert!(trim_info.comparison_literals.contains(&" ".to_string()));
                assert!(trim_info.comparison_literals.contains(&"\t".to_string()));
                assert_eq!(trim_info.carrier_name, "is_ch_match");
            }
            PromotionResult::CannotPromote { reason, .. } => {
                panic!("Expected Promoted, got CannotPromote: {}", reason);
            }
        }
    }

    #[test]
    fn test_trim_pattern_with_4_whitespace_chars() {
        // Full whitespace pattern: " " || "\t" || "\n" || "\r"
        let scope = minimal_scope();
        let cond_scope = cond_scope_with_body_local("ch");

        let loop_body = vec![assignment("ch", method_call("s", "substring"))];

        let break_cond = or_expr(
            or_expr(eq_cmp("ch", " "), eq_cmp("ch", "\t")),
            or_expr(eq_cmp("ch", "\n"), eq_cmp("ch", "\r")),
        );

        let request = PromotionRequest {
            scope: &scope,
            cond_scope: &cond_scope,
            break_cond: Some(&break_cond),
            loop_body: &loop_body,
        };

        let result = LoopBodyCarrierPromoter::try_promote(&request);

        match result {
            PromotionResult::Promoted { trim_info } => {
                assert_eq!(trim_info.comparison_literals.len(), 4);
            }
            PromotionResult::CannotPromote { reason, .. } => {
                panic!("Expected Promoted, got CannotPromote: {}", reason);
            }
        }
    }
}
