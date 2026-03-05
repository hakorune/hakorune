//! Phase 282 P3: loop_simple_while extraction
//! (legacy label: Pattern1)
//! Phase 282 P9a: Integrated with common_helpers

use crate::ast::{ASTNode, BinaryOperator};

// Phase 282 P9a: Use common_helpers
use super::common_helpers::has_control_flow_statement as common_has_control_flow;

#[derive(Debug, Clone)]
pub(crate) struct Pattern1Parts {
    pub loop_var: String,
    // Note: condition/body は ctx から再利用（AST 丸コピー不要）
}

/// Extract loop_simple_while parts (legacy label: Pattern1)
///
/// # Detection Criteria (誤マッチ防止強化版)
///
/// 1. **Condition**: 比較演算（<, <=, >, >=, ==, !=）で左辺が変数
/// 2. **Body**: No break/continue/nested-loop/if-else-phi (return is allowed - it's not loop control flow)
/// 3. **Step**: 単純な増減パターン (i = i + 1, i = i - 1 など)
///
/// # Four-Phase Validation
///
/// **Phase 1**: Validate condition structure (比較 + 左が変数)
/// **Phase 2**: Validate body (control flow check)
/// **Phase 3**: Validate step pattern (単純増減のみ)
/// **Phase 4**: Extract loop variable
///
/// # Fail-Fast Rules
///
/// - `Ok(Some(parts))`: loop_simple_while match confirmed
/// - `Ok(None)`: Not loop_simple_while (構造不一致 or control flow)
/// - `Err(msg)`: Logic bug (malformed AST)
pub(crate) fn extract_simple_while_parts(
    condition: &ASTNode,
    body: &[ASTNode],
) -> Result<Option<Pattern1Parts>, String> {
    // Phase 1: Validate condition structure (比較 + 左が変数)
    let loop_var = match validate_condition_structure(condition) {
        Some(var) => var,
        None => return Ok(None), // 条件が loop_simple_while 形式でない
    };

    // Phase 2: Validate body (reject control flow)
    if has_control_flow_statement(body) {
        // Has break/continue/return → Not loop_simple_while
        return Ok(None);
    }

    // Phase 29ak: Reject nested loops.
    //
    // Nested loops are nested-minimal territory (legacy label: Pattern6NestedLoopMinimal);
    // letting loop_simple_while match them
    // causes routing to short-circuit before JoinIR can select the nested-loop lowerer.
    let counts = super::common_helpers::count_control_flow(
        body,
        super::common_helpers::ControlFlowDetector::default(),
    );
    if counts.has_nested_loop {
        return Ok(None);
    }

    // Phase 286 P2.6: Reject if-else statements (if_phi_join territory).
    // loop_simple_while allows simple if without else, but not if-else
    // (legacy label: Pattern3).
    if super::common_helpers::has_if_else_statement(body) {
        // Has if-else statement → if_phi_join (legacy label: Pattern3, if-phi merge)
        return Ok(None);
    }

    // Phase 3: Validate step pattern (単純増減のみ)
    if !has_simple_step_pattern(body, &loop_var) {
        // Step が複雑 or 存在しない → Not loop_simple_while
        return Ok(None);
    }

    // Phase 4: Return extracted info
    Ok(Some(Pattern1Parts { loop_var }))
}

/// Validate condition: 比較演算 (左辺が変数)
///
/// Exported for reuse by if_phi_join extractor (legacy label: Pattern3, Phase 282 P5)
pub(crate) fn validate_condition_structure(condition: &ASTNode) -> Option<String> {
    match condition {
        ASTNode::BinaryOp { operator, left, .. } => {
            // 比較演算子チェック
            if !matches!(
                operator,
                BinaryOperator::Less
                    | BinaryOperator::LessEqual
                    | BinaryOperator::Greater
                    | BinaryOperator::GreaterEqual
                    | BinaryOperator::Equal
                    | BinaryOperator::NotEqual
            ) {
                return None; // 比較でない（算術演算など）
            }

            // 左辺が変数であることを確認
            if let ASTNode::Variable { name, .. } = left.as_ref() {
                return Some(name.clone());
            }

            None // 左辺が変数でない
        }
        _ => None, // 比較演算でない
    }
}

/// Validate step: 単純増減パターン (i = i ± const)
fn has_simple_step_pattern(body: &[ASTNode], loop_var: &str) -> bool {
    if body.len() != 1 {
        return false;
    }
    for stmt in body {
        if let ASTNode::Assignment { target, value, .. } = stmt {
            // target が loop_var か確認
            if let ASTNode::Variable { name, .. } = target.as_ref() {
                if name != loop_var {
                    continue; // 他の変数への代入
                }

                // value が "loop_var ± const" 形式か確認
                if let ASTNode::BinaryOp {
                    operator,
                    left,
                    right,
                    ..
                } = value.as_ref()
                {
                    // 演算子が + or -
                    if !matches!(operator, BinaryOperator::Add | BinaryOperator::Subtract) {
                        return false; // 複雑な演算
                    }

                    // 左辺が loop_var
                    if let ASTNode::Variable { name: left_var, .. } = left.as_ref() {
                        if left_var != loop_var {
                            return false; // i = j + 1 みたいなパターン
                        }
                    } else {
                        return false; // 左辺が変数でない
                    }

                    // 右辺が定数
                    if !matches!(right.as_ref(), ASTNode::Literal { .. }) {
                        return false; // i = i + j みたいなパターン
                    }

                    return true; // ✅ 単純増減パターン確認
                }

                // value が他の形式（複雑な代入）
                return false;
            }
        }
    }

    // Step が見つからない → 単純ループでない
    false
}

/// Check if body has control flow statements
///
/// # Phase 282 P9a: Delegates to common_helpers::has_control_flow_statement
fn has_control_flow_statement(body: &[ASTNode]) -> bool {
    common_has_control_flow(body)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{LiteralValue, Span};

    #[test]
    fn test_extract_simple_while_success() {
        // loop(i < 10) { i = i + 1 }
        let condition = ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(ASTNode::Variable {
                name: "i".to_string(),
                span: Span::unknown(),
            }),
            right: Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(10),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };

        let body = vec![ASTNode::Assignment {
            target: Box::new(ASTNode::Variable {
                name: "i".to_string(),
                span: Span::unknown(),
            }),
            value: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Add,
                left: Box::new(ASTNode::Variable {
                    name: "i".to_string(),
                    span: Span::unknown(),
                }),
                right: Box::new(ASTNode::Literal {
                    value: LiteralValue::Integer(1),
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }];

        let result = extract_simple_while_parts(&condition, &body);
        assert!(result.is_ok());
        assert!(result.unwrap().is_some());
    }

    #[test]
    fn test_extract_with_break_returns_none() {
        let condition = ASTNode::Variable {
            name: "x".to_string(),
            span: Span::unknown(),
        };
        let body = vec![ASTNode::Break {
            span: Span::unknown(),
        }];

        let result = extract_simple_while_parts(&condition, &body);
        assert!(result.is_ok());
        assert!(result.unwrap().is_none()); // Has break → Ok(None)
    }

    #[test]
    fn test_extract_with_continue_returns_none() {
        let condition = ASTNode::Variable {
            name: "x".to_string(),
            span: Span::unknown(),
        };
        let body = vec![ASTNode::Continue {
            span: Span::unknown(),
        }];

        let result = extract_simple_while_parts(&condition, &body);
        assert!(result.is_ok());
        assert!(result.unwrap().is_none()); // Has continue → Ok(None)
    }
}
