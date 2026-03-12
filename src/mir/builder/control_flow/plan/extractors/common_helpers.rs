//! Common extraction helpers for core loop routes.
//!
//! Phase 282 P9a: Extracted from loop route extractors
//! to eliminate common duplication.
//! Phase 29ai P10: Moved to plan-layer SSOT (JoinIR keeps a wrapper path).
//!
//! # Design Principles
//!
//! - **Pure Functions**: No side effects, no builder mutations
//! - **Fail-Fast**: Err for logic bugs, Ok(None) for non-matches
//! - **Configurability**: ControlFlowDetector for route-specific behavior
//! - **Scope-Limited**: Common detection only, route-specific logic excluded
//!
//! # Groups (P9a Scope-Limited)
//!
//! 1. Control Flow Counting (count_control_flow) - Universal counter
//! 2. Control Flow Detection (has_break_statement, has_continue_statement, etc.) - Common detection
//! 3. Condition Validation (extract_loop_variable, is_true_literal) - Condition helpers
//! 4. Loop Increment Extraction (extract_loop_increment_plan) - Common plan helper
//! 5. loop_true_early_exit-specific helpers
//!    validate_continue_at_end, validate_break_in_simple_if) - NOT generalized
//!
//! **IMPORTANT**: route-specific interpretation logic (e.g., if_phi_join nested_if)
//! is EXCLUDED.
//! Such logic remains in individual extractor files to maintain clear SSOT boundaries.

#![allow(dead_code)]

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::canon::generic_loop::canon_update_for_loop_var;

/// Walk a statement list and flatten Program/ScopeBox wrappers.
///
/// This is analysis-only: it does not rewrite or mutate AST nodes.
/// The visitor returns `true` to stop traversal early.
pub(crate) fn walk_stmt_list<'a, F>(body: &'a [ASTNode], mut visit: F) -> bool
where
    F: FnMut(&'a ASTNode) -> bool,
{
    fn walk_node<'a, F>(node: &'a ASTNode, visit: &mut F) -> bool
    where
        F: FnMut(&'a ASTNode) -> bool,
    {
        match node {
            ASTNode::Program { statements, .. } => {
                for stmt in statements {
                    if walk_node(stmt, visit) {
                        return true;
                    }
                }
                false
            }
            ASTNode::ScopeBox { body, .. } => {
                for stmt in body {
                    if walk_node(stmt, visit) {
                        return true;
                    }
                }
                false
            }
            _ => visit(node),
        }
    }

    for stmt in body {
        if walk_node(stmt, &mut visit) {
            return true;
        }
    }
    false
}

pub(crate) fn flatten_stmt_list<'a>(body: &'a [ASTNode]) -> Vec<&'a ASTNode> {
    let mut out = Vec::new();
    walk_stmt_list(body, |stmt| {
        out.push(stmt);
        false
    });
    out
}

/// View-only helper: drop a trailing top-level `continue` (no AST rewrite).
pub(crate) fn strip_trailing_continue_view<'a>(body: &'a [ASTNode]) -> (&'a [ASTNode], bool) {
    match body.last() {
        Some(ASTNode::Continue { .. }) => (&body[..body.len().saturating_sub(1)], true),
        _ => (body, false),
    }
}

/// ============================================================
/// Group 1: Control Flow Counting (汎用カウンター)
/// ============================================================

#[derive(Debug, Clone, Default)]
pub(crate) struct ControlFlowCounts {
    pub break_count: usize,
    pub continue_count: usize,
    pub return_count: usize,
    pub has_nested_loop: bool,
}

/// Control flow detection options
///
/// # P9a Scope-Limited
/// - `detect_nested_if` removed (if_phi_join-specific, deferred to P9b)
/// - Only common detection options included
#[derive(Debug, Clone)]
pub(crate) struct ControlFlowDetector {
    /// Skip break/continue inside nested loops?
    pub skip_nested_control_flow: bool,
    /// Count return statements?
    pub count_returns: bool,
}

impl Default for ControlFlowDetector {
    fn default() -> Self {
        Self {
            skip_nested_control_flow: true,
            count_returns: false,
        }
    }
}

/// Universal control flow counter
///
/// # Examples (P9a Scope-Limited)
/// - loop_simple_while: default (skip_nested=true, count_returns=false)
/// - loop_break: default (skip_nested=true, count_returns=false)
/// - loop_continue_only: default (skip_nested=true, count_returns=false)
/// - loop_true_early_exit: count_returns=true
///   (returns Err if return found)
pub(crate) fn count_control_flow(
    body: &[ASTNode],
    detector: ControlFlowDetector,
) -> ControlFlowCounts {
    let mut counts = ControlFlowCounts::default();

    fn scan_node(
        node: &ASTNode,
        counts: &mut ControlFlowCounts,
        detector: &ControlFlowDetector,
        depth: usize,
    ) {
        match node {
            ASTNode::Break { .. } => {
                counts.break_count += 1;
            }
            ASTNode::Continue { .. } => {
                counts.continue_count += 1;
            }
            ASTNode::Return { .. } if detector.count_returns => {
                counts.return_count += 1;
            }
            ASTNode::Loop { body, .. }
            | ASTNode::While { body, .. }
            | ASTNode::ForRange { body, .. } => {
                counts.has_nested_loop = true;
                // Skip nested loop bodies if configured
                if detector.skip_nested_control_flow {
                    return;
                }
                walk_stmt_list(body, |stmt| {
                    scan_node(stmt, counts, detector, depth + 1);
                    false
                });
            }
            ASTNode::If {
                then_body,
                else_body,
                ..
            } => {
                // Recurse into if/else bodies
                walk_stmt_list(then_body, |stmt| {
                    scan_node(stmt, counts, detector, depth + 1);
                    false
                });
                if let Some(else_b) = else_body {
                    walk_stmt_list(else_b, |stmt| {
                        scan_node(stmt, counts, detector, depth + 1);
                        false
                    });
                }
            }
            _ => {}
        }
    }

    walk_stmt_list(body, |stmt| {
        scan_node(stmt, &mut counts, &detector, 0);
        false
    });

    counts
}

/// ============================================================
/// Group 2: Control Flow Detection (真偽値判定)
/// ============================================================

/// Check if body has ANY break statement
pub(crate) fn has_break_statement(body: &[ASTNode]) -> bool {
    count_control_flow(body, ControlFlowDetector::default()).break_count > 0
}

/// Check if body has ANY continue statement
pub(crate) fn has_continue_statement(body: &[ASTNode]) -> bool {
    count_control_flow(body, ControlFlowDetector::default()).continue_count > 0
}

/// Check if body has ANY return statement
pub(crate) fn has_return_statement(body: &[ASTNode]) -> bool {
    let mut detector = ControlFlowDetector::default();
    detector.count_returns = true;
    count_control_flow(body, detector).return_count > 0
}

/// Check if body has ANY break or continue
pub(crate) fn has_control_flow_statement(body: &[ASTNode]) -> bool {
    let counts = count_control_flow(body, ControlFlowDetector::default());
    counts.break_count > 0 || counts.continue_count > 0
}

/// Phase 286 P2.6: Check if body has ANY if statement (recursive)
///
/// This is a supplementary helper for loop_simple_while extraction to prevent
/// loop_simple_while from incorrectly matching if_phi_join fixtures.
pub(crate) fn has_if_statement(body: &[ASTNode]) -> bool {
    walk_stmt_list(body, |node| match node {
        ASTNode::If { .. } => true,
        ASTNode::Loop { body, .. } => has_if_statement(body),
        _ => false,
    })
}

/// Phase 286 P2.6: Check if body has ANY if-else statement (recursive)
///
/// This is more specific than has_if_statement - it only detects if statements
/// with else branches, which are if_phi_join territory.
pub(crate) fn has_if_else_statement(body: &[ASTNode]) -> bool {
    walk_stmt_list(body, |node| match node {
        ASTNode::If {
            else_body: Some(_), ..
        } => true,
        ASTNode::Loop { body, .. } => has_if_else_statement(body),
        _ => false,
    })
}

/// Phase 286: Find first if-else statement in loop body (non-recursive)
pub(crate) fn find_if_else_statement(body: &[ASTNode]) -> Option<&ASTNode> {
    body.iter().find(|stmt| {
        matches!(
            stmt,
            ASTNode::If {
                else_body: Some(_),
                ..
            }
        )
    })
}

/// ============================================================
/// Group 3: Condition Validation (比較演算検証)
/// ============================================================
use crate::ast::BinaryOperator;

/// Validate condition: 比較演算 (左辺が変数)
pub(crate) fn extract_loop_variable(condition: &ASTNode) -> Option<String> {
    match condition {
        ASTNode::BinaryOp { operator, left, .. } => {
            if !matches!(
                operator,
                BinaryOperator::Less
                    | BinaryOperator::LessEqual
                    | BinaryOperator::Greater
                    | BinaryOperator::GreaterEqual
                    | BinaryOperator::Equal
                    | BinaryOperator::NotEqual
            ) {
                return None;
            }

            if let ASTNode::Variable { name, .. } = left.as_ref() {
                return Some(name.clone());
            }

            None
        }
        _ => None,
    }
}

/// Check if condition is true literal
pub(crate) fn is_true_literal(condition: &ASTNode) -> bool {
    use crate::ast::LiteralValue;

    matches!(
        condition,
        ASTNode::Literal {
            value: LiteralValue::Bool(true),
            ..
        }
    )
}

/// ============================================================
/// Group 4: Loop Increment Extraction (Common for Plan line)
/// ============================================================

/// Phase 286 P2.2: Extract loop increment for Plan line patterns
///
/// Supports `<var> = <var> ( + | - | * | / ) <int_lit>` pattern only (PoC safety).
pub(crate) fn extract_loop_increment_plan(
    body: &[ASTNode],
    loop_var: &str,
) -> Result<Option<ASTNode>, String> {
    fn extract_increment_value(stmt: &ASTNode, loop_var: &str) -> Option<ASTNode> {
        let ASTNode::Assignment { target, value, .. } = stmt else {
            return None;
        };
        let ASTNode::Variable { name, .. } = target.as_ref() else {
            return None;
        };
        if name != loop_var {
            return None;
        }
        if canon_update_for_loop_var(stmt, loop_var).is_some() {
            return Some(value.as_ref().clone());
        }
        let ASTNode::BinaryOp {
            operator,
            left,
            right,
            ..
        } = value.as_ref()
        else {
            return None;
        };
        match operator {
            BinaryOperator::Add => {
                if let (ASTNode::Variable { name: lname, .. }, ASTNode::Literal { .. }) =
                    (left.as_ref(), right.as_ref())
                {
                    if lname == loop_var {
                        return Some(value.as_ref().clone());
                    }
                }
                if let (ASTNode::Literal { .. }, ASTNode::Variable { name: rname, .. }) =
                    (left.as_ref(), right.as_ref())
                {
                    if rname == loop_var {
                        return Some(value.as_ref().clone());
                    }
                }
                None
            }
            BinaryOperator::Subtract | BinaryOperator::Multiply | BinaryOperator::Divide => {
                let ASTNode::Variable { name: lname, .. } = left.as_ref() else {
                    return None;
                };
                if lname != loop_var {
                    return None;
                }
                if !matches!(right.as_ref(), ASTNode::Literal { .. }) {
                    return None;
                }
                Some(value.as_ref().clone())
            }
            _ => None,
        }
    }

    for stmt in body {
        if let Some(increment) = extract_increment_value(stmt, loop_var) {
            return Ok(Some(increment));
        }
    }

    // Fallback contract:
    // - Only the last top-level statement may supply this fallback step.
    // - The statement must be an assignment to the current loop var.
    // - This is used when canonical +/-/*// literal forms are absent,
    //   mainly for selfhost release-route loops with computed step values.
    if let Some(tail_value) = extract_tail_loop_assignment_value(body, loop_var) {
        return Ok(Some(tail_value));
    }

    let mut found: Option<ASTNode> = None;
    for stmt in body {
        let ASTNode::If {
            then_body,
            else_body,
            ..
        } = stmt
        else {
            continue;
        };
        let is_continue_tail =
            matches!(then_body.last(), Some(ASTNode::Continue { .. })) && else_body.is_none();
        let is_break_else = else_body
            .as_ref()
            .is_some_and(|body| body.len() == 1 && matches!(body[0], ASTNode::Break { .. }));
        if !is_continue_tail && !is_break_else {
            continue;
        }
        for inner in then_body {
            if let Some(increment) = extract_increment_value(inner, loop_var) {
                if found.is_some() {
                    return Ok(None);
                }
                found = Some(increment);
            }
        }
    }
    Ok(found)
}

fn extract_tail_loop_assignment_value(body: &[ASTNode], loop_var: &str) -> Option<ASTNode> {
    let ASTNode::Assignment { target, value, .. } = body.last()? else {
        return None;
    };
    let ASTNode::Variable { name, .. } = target.as_ref() else {
        return None;
    };
    if name != loop_var {
        return None;
    }
    Some(value.as_ref().clone())
}

/// ============================================================
/// Group 5: loop_true_early_exit-specific helpers (NOT generalized)
/// ============================================================
///
/// **IMPORTANT**: These helpers are loop_true_early_exit-specific and intentionally NOT generalized.

/// Validate continue is at body end (loop_true_early_exit specific)
pub(crate) fn validate_continue_at_end(body: &[ASTNode]) -> bool {
    matches!(body.last(), Some(ASTNode::Continue { .. }))
}

/// Validate break is in simple if pattern (loop_true_early_exit specific)
pub(crate) fn validate_break_in_simple_if(body: &[ASTNode]) -> bool {
    for stmt in body {
        if let ASTNode::If {
            then_body,
            else_body,
            ..
        } = stmt
        {
            if then_body.len() == 1
                && matches!(then_body[0], ASTNode::Break { .. })
                && else_body.is_none()
            {
                return true;
            }
        }
    }
    false
}

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};

    fn make_break() -> ASTNode {
        ASTNode::Break {
            span: Span::unknown(),
        }
    }

    fn make_continue() -> ASTNode {
        ASTNode::Continue {
            span: Span::unknown(),
        }
    }

    fn make_return() -> ASTNode {
        ASTNode::Return {
            value: None,
            span: Span::unknown(),
        }
    }

    fn make_if_with_break() -> ASTNode {
        ASTNode::If {
            condition: Box::new(ASTNode::Variable {
                name: "cond".to_string(),
                span: Span::unknown(),
            }),
            then_body: vec![make_break()],
            else_body: None,
            span: Span::unknown(),
        }
    }

    fn make_nested_loop() -> ASTNode {
        ASTNode::Loop {
            condition: Box::new(ASTNode::Variable {
                name: "cond".to_string(),
                span: Span::unknown(),
            }),
            body: vec![make_break()],
            span: Span::unknown(),
        }
    }

    #[test]
    fn test_count_control_flow_break() {
        let body = vec![make_break()];
        let counts = count_control_flow(&body, ControlFlowDetector::default());
        assert_eq!(counts.break_count, 1);
        assert_eq!(counts.continue_count, 0);
    }

    #[test]
    fn test_has_break_statement() {
        let body = vec![make_if_with_break()];
        assert!(has_break_statement(&body));
    }

    #[test]
    fn test_has_continue_statement() {
        let body = vec![make_continue()];
        assert!(has_continue_statement(&body));
    }

    #[test]
    fn test_has_continue_statement_false() {
        let body = vec![make_break()];
        assert!(!has_continue_statement(&body));
    }

    #[test]
    fn test_has_return_statement() {
        let body = vec![make_return()];
        assert!(has_return_statement(&body));
    }

    #[test]
    fn test_has_return_statement_false() {
        let body = vec![make_break()];
        assert!(!has_return_statement(&body));
    }

    #[test]
    fn test_has_control_flow_statement_break() {
        let body = vec![make_if_with_break()];
        assert!(has_control_flow_statement(&body));
    }

    #[test]
    fn test_has_control_flow_statement_continue() {
        let body = vec![make_continue()];
        assert!(has_control_flow_statement(&body));
    }

    #[test]
    fn test_has_control_flow_statement_false() {
        let body = vec![ASTNode::Variable {
            name: "x".to_string(),
            span: Span::unknown(),
        }];
        assert!(!has_control_flow_statement(&body));
    }

    #[test]
    fn test_count_control_flow_detects_nested_loop_at_top_level() {
        let body = vec![make_nested_loop()];
        let counts = count_control_flow(&body, ControlFlowDetector::default());
        assert!(counts.has_nested_loop);
    }

    #[test]
    fn test_count_control_flow_detects_nested_loop_in_if() {
        let body = vec![ASTNode::If {
            condition: Box::new(ASTNode::Variable {
                name: "cond".to_string(),
                span: Span::unknown(),
            }),
            then_body: vec![make_nested_loop()],
            else_body: None,
            span: Span::unknown(),
        }];
        let counts = count_control_flow(&body, ControlFlowDetector::default());
        assert!(counts.has_nested_loop);
    }

    #[test]
    fn test_has_control_flow_statement_break_in_scopebox() {
        let body = vec![ASTNode::ScopeBox {
            body: vec![make_break()],
            span: Span::unknown(),
        }];
        assert!(has_control_flow_statement(&body));
    }

    #[test]
    fn test_extract_loop_variable_success() {
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

        assert_eq!(extract_loop_variable(&condition), Some("i".to_string()));
    }

    #[test]
    fn test_extract_loop_variable_not_comparison() {
        let condition = ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
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

        assert_eq!(extract_loop_variable(&condition), None);
    }

    #[test]
    fn test_extract_loop_variable_not_variable() {
        let condition = ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(5),
                span: Span::unknown(),
            }),
            right: Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(10),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };

        assert_eq!(extract_loop_variable(&condition), None);
    }

    #[test]
    fn test_is_true_literal_success() {
        let condition = ASTNode::Literal {
            value: LiteralValue::Bool(true),
            span: Span::unknown(),
        };

        assert!(is_true_literal(&condition));
    }

    #[test]
    fn test_is_true_literal_false() {
        let condition = ASTNode::Literal {
            value: LiteralValue::Bool(false),
            span: Span::unknown(),
        };

        assert!(!is_true_literal(&condition));
    }

    #[test]
    fn test_is_true_literal_not_literal() {
        let condition = ASTNode::Variable {
            name: "x".to_string(),
            span: Span::unknown(),
        };

        assert!(!is_true_literal(&condition));
    }

    #[test]
    fn test_validate_continue_at_end_success() {
        let body = vec![make_break(), make_continue()];
        assert!(validate_continue_at_end(&body));
    }

    #[test]
    fn test_validate_continue_at_end_false() {
        let body = vec![make_continue(), make_break()];
        assert!(!validate_continue_at_end(&body));
    }

    #[test]
    fn test_validate_break_in_simple_if_success() {
        let body = vec![make_if_with_break()];
        assert!(validate_break_in_simple_if(&body));
    }

    #[test]
    fn test_validate_break_in_simple_if_with_else() {
        let body = vec![ASTNode::If {
            condition: Box::new(ASTNode::Variable {
                name: "done".to_string(),
                span: Span::unknown(),
            }),
            then_body: vec![make_break()],
            else_body: Some(vec![make_continue()]),
            span: Span::unknown(),
        }];
        assert!(!validate_break_in_simple_if(&body));
    }

    #[test]
    fn test_validate_break_in_simple_if_multiple_statements() {
        let body = vec![ASTNode::If {
            condition: Box::new(ASTNode::Variable {
                name: "done".to_string(),
                span: Span::unknown(),
            }),
            then_body: vec![make_break(), make_continue()],
            else_body: None,
            span: Span::unknown(),
        }];
        assert!(!validate_break_in_simple_if(&body));
    }

    fn var(name: &str) -> ASTNode {
        ASTNode::Variable {
            name: name.to_string(),
            span: Span::unknown(),
        }
    }

    fn lit_i(v: i64) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Integer(v),
            span: Span::unknown(),
        }
    }

    fn assign(name: &str, value: ASTNode) -> ASTNode {
        ASTNode::Assignment {
            target: Box::new(var(name)),
            value: Box::new(value),
            span: Span::unknown(),
        }
    }

    #[test]
    fn test_extract_loop_increment_plan_uses_tail_assignment_fallback() {
        let body = vec![
            ASTNode::MethodCall {
                object: Box::new(var("arr")),
                method: "push".to_string(),
                arguments: vec![var("i")],
                span: Span::unknown(),
            },
            assign(
                "i",
                ASTNode::MethodCall {
                    object: Box::new(var("arr")),
                    method: "length".to_string(),
                    arguments: vec![],
                    span: Span::unknown(),
                },
            ),
        ];

        let inc = extract_loop_increment_plan(&body, "i")
            .expect("no error")
            .expect("must extract fallback step");
        assert!(matches!(inc, ASTNode::MethodCall { .. }));
    }

    #[test]
    fn test_extract_loop_increment_plan_ignores_non_tail_assignment() {
        let body = vec![
            assign("i", lit_i(1)),
            ASTNode::MethodCall {
                object: Box::new(var("arr")),
                method: "push".to_string(),
                arguments: vec![var("i")],
                span: Span::unknown(),
            },
        ];

        let inc = extract_loop_increment_plan(&body, "i").expect("no error");
        assert!(inc.is_none());
    }

    #[test]
    fn test_extract_loop_increment_plan_ignores_tail_other_var() {
        let body = vec![
            ASTNode::MethodCall {
                object: Box::new(var("arr")),
                method: "push".to_string(),
                arguments: vec![var("i")],
                span: Span::unknown(),
            },
            assign("j", lit_i(1)),
        ];

        let inc = extract_loop_increment_plan(&body, "i").expect("no error");
        assert!(inc.is_none());
    }
}
