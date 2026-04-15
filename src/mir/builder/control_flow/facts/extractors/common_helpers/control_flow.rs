use crate::ast::ASTNode;
use crate::mir::builder::control_flow::facts::stmt_walk::walk_stmt_list;

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
