//! Nested-loop body profile (analysis-only, no AST rewrite).
//!
//! ## Cluster count SSOT (Phase 29bq clusterN consolidation)
//!
//! cluster3/4/5 等の「数だけで増える」箱の追加点を1箇所に集約する。
//! planner outcome は facts/recipe 契約のみを扱う（挙動不変）。

// ============================================================================
// Cluster count configuration SSOT
// ============================================================================

/// Cluster count configuration for loop_cond_break_continue variants.
#[derive(Debug, Clone, Copy)]
pub(in crate::mir::builder) struct ClusterCountProfile {
    /// Exact number of nested loops required for this cluster
    pub required_count: u8,
    /// Static rule name for planner logging
    pub rule: &'static str,
}

/// SSOT: Supported cluster configurations (優先順: 大きい数から)
///
/// cluster6+ 追加時は、この配列に1行追加するだけ。
pub(in crate::mir::builder) const CLUSTER_PROFILES: &[ClusterCountProfile] = &[
    ClusterCountProfile { required_count: 5, rule: "loop/loop_cond_break_continue_cluster5" },
    ClusterCountProfile { required_count: 4, rule: "loop/loop_cond_break_continue_cluster4" },
    ClusterCountProfile { required_count: 3, rule: "loop/loop_cond_break_continue_cluster3" },
];

/// Base profile rule (no cluster requirement)
pub(in crate::mir::builder) const BASE_RULE: &str = "loop/loop_cond_break_continue";

// ============================================================================
// Nested-loop body profile (existing functionality)
// ============================================================================

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::extractors::common_helpers::
    strip_trailing_continue_view;
use crate::mir::builder::control_flow::plan::facts::expr_bool::
    is_supported_bool_expr_with_canon;

#[derive(Debug, Clone, Copy)]
pub(in crate::mir::builder) struct NestedLoopBodyProfile {
    pub allow_calls: bool,
    pub require_call: bool,
    pub allow_break_in_if: bool,
    pub allow_continue_in_if: bool,
    pub allow_trailing_continue: bool,
}

#[derive(Debug, Clone, Copy)]
pub(in crate::mir::builder) struct NestedLoopBodyScan {
    pub seen_call: bool,
    pub has_trailing_continue: bool,
}

pub(in crate::mir::builder) fn scan_nested_loop_body(
    body: &[ASTNode],
    profile: NestedLoopBodyProfile,
    allow_extended: bool,
) -> Option<NestedLoopBodyScan> {
    let (trimmed_body, has_trailing_continue) = if profile.allow_trailing_continue {
        strip_trailing_continue_view(body)
    } else {
        (body, false)
    };

    let mut seen_call = false;
    if !scan_body(
        trimmed_body,
        false,
        profile,
        allow_extended,
        &mut seen_call,
    ) {
        return None;
    }

    if profile.require_call && !seen_call {
        return None;
    }

    Some(NestedLoopBodyScan {
        seen_call,
        has_trailing_continue,
    })
}

fn scan_body(
    body: &[ASTNode],
    in_if: bool,
    profile: NestedLoopBodyProfile,
    allow_extended: bool,
    seen_call: &mut bool,
) -> bool {
    for stmt in body {
        match stmt {
            ASTNode::Local { initial_values, .. } => {
                for init in initial_values {
                    let Some(init) = init.as_ref() else {
                        continue;
                    };
                    if expr_contains_call(init) {
                        if !profile.allow_calls {
                            return false;
                        }
                        *seen_call = true;
                    }
                }
            }
            ASTNode::Assignment { value, .. } => {
                if expr_contains_call(value) {
                    if !profile.allow_calls {
                        return false;
                    }
                    *seen_call = true;
                }
            }
            ASTNode::MethodCall { .. }
            | ASTNode::FunctionCall { .. }
            | ASTNode::Call { .. } => {
                if !profile.allow_calls {
                    return false;
                }
                *seen_call = true;
            }
            ASTNode::If {
                condition,
                then_body,
                else_body,
                ..
            } => {
                if !is_supported_bool_expr_with_canon(condition, allow_extended) {
                    return false;
                }
                if expr_contains_call(condition) {
                    if !profile.allow_calls {
                        return false;
                    }
                    *seen_call = true;
                }
                if !scan_body(then_body, true, profile, allow_extended, seen_call) {
                    return false;
                }
                if let Some(else_body) = else_body {
                    if !scan_body(else_body, true, profile, allow_extended, seen_call) {
                        return false;
                    }
                }
            }
            ASTNode::Program { statements, .. } => {
                if !scan_body(statements, in_if, profile, allow_extended, seen_call) {
                    return false;
                }
            }
            ASTNode::ScopeBox { body, .. } => {
                if !scan_body(body, in_if, profile, allow_extended, seen_call) {
                    return false;
                }
            }
            ASTNode::Break { .. } => {
                if !profile.allow_break_in_if || !in_if {
                    return false;
                }
            }
            ASTNode::Continue { .. } => {
                if !profile.allow_continue_in_if || !in_if {
                    return false;
                }
            }
            ASTNode::Return { .. }
            | ASTNode::Loop { .. }
            | ASTNode::While { .. }
            | ASTNode::ForRange { .. }
            | ASTNode::Print { .. } => return false,
            _ => return false,
        }
    }
    true
}

fn expr_contains_call(ast: &ASTNode) -> bool {
    match ast {
        ASTNode::MethodCall { .. } | ASTNode::FunctionCall { .. } | ASTNode::Call { .. } => true,
        ASTNode::BinaryOp { left, right, .. } => {
            expr_contains_call(left) || expr_contains_call(right)
        }
        ASTNode::UnaryOp { operand, .. } => expr_contains_call(operand),
        ASTNode::Variable { .. } | ASTNode::Literal { .. } => false,
        ASTNode::FieldAccess { object, .. } => expr_contains_call(object),
        ASTNode::Index { target, index, .. } => {
            expr_contains_call(target) || expr_contains_call(index)
        }
        ASTNode::New { arguments, .. } => arguments.iter().any(|arg| expr_contains_call(arg)),
        ASTNode::This { .. }
        | ASTNode::Me { .. }
        | ASTNode::ThisField { .. }
        | ASTNode::MeField { .. } => false,
        _ => false,
    }
}
