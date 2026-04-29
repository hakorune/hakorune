use crate::ast::{ASTNode, BinaryOperator, Span};
use crate::mir::join_ir::lowering::error_tags;
use crate::mir::join_ir::lowering::loop_update_analyzer::{UpdateExpr, UpdateRhs};
use crate::mir::join_ir::BinOpKind;
use crate::mir::policies::post_loop_early_return_plan::PostLoopEarlyReturnPlan;
use std::collections::BTreeMap;

use super::super::PolicyDecision;
use super::ast_helpers::{eq_int, eq_str, var};
use super::extract::{extract_bounded_loop_counter, extract_depth_scan_shape};
use super::types::{BalancedDepthScanPolicyResult, BalancedDepthScanRecipe};

pub fn classify_balanced_depth_scan_array_end(
    condition: &ASTNode,
    body: &[ASTNode],
) -> PolicyDecision<BalancedDepthScanPolicyResult> {
    classify_balanced_depth_scan(condition, body, "[", "]")
}

pub fn classify_balanced_depth_scan_object_end(
    condition: &ASTNode,
    body: &[ASTNode],
) -> PolicyDecision<BalancedDepthScanPolicyResult> {
    classify_balanced_depth_scan(condition, body, "{", "}")
}

/// Decide balanced depth-scan family (SSOT ordering).
///
/// IMPORTANT: `Reject` means "close-but-unsupported" for that family, not "not this family".
/// We only return `Reject` if no other family matches.
pub fn decide(
    condition: &ASTNode,
    body: &[ASTNode],
) -> PolicyDecision<BalancedDepthScanPolicyResult> {
    let array = classify_balanced_depth_scan_array_end(condition, body);
    match array {
        PolicyDecision::Use(_) => array,
        PolicyDecision::Reject(_) => {
            let object = classify_balanced_depth_scan_object_end(condition, body);
            match object {
                PolicyDecision::Use(_) => object,
                PolicyDecision::Reject(_) => array,
                PolicyDecision::None => array,
            }
        }
        PolicyDecision::None => classify_balanced_depth_scan_object_end(condition, body),
    }
}

fn classify_balanced_depth_scan(
    condition: &ASTNode,
    body: &[ASTNode],
    open: &str,
    close: &str,
) -> PolicyDecision<BalancedDepthScanPolicyResult> {
    // bounded loop: loop(i < n)
    let (loop_counter_name, bound_name) = match extract_bounded_loop_counter(condition) {
        Some(v) => v,
        None => return PolicyDecision::None,
    };

    let summary = match extract_depth_scan_shape(body, &loop_counter_name, open, close) {
        Ok(Some(v)) => v,
        Ok(None) => return PolicyDecision::None,
        Err(reason) => return PolicyDecision::Reject(reason),
    };

    let depth_delta_name = "depth_delta".to_string();
    let depth_next_name = "depth_next".to_string();
    if summary.declared_locals.contains(&depth_delta_name)
        || summary.declared_locals.contains(&depth_next_name)
    {
        return PolicyDecision::Reject(error_tags::freeze(
            "[phase107/balanced_depth_scan/contract/name_conflict] 'depth_delta' or 'depth_next' is already declared in the loop body",
        ));
    }

    let break_condition_node = ASTNode::BinaryOp {
        operator: BinaryOperator::And,
        left: Box::new(eq_str(var(&summary.ch_name), close)),
        right: Box::new(eq_int(var(&depth_next_name), 0)),
        span: Span::unknown(),
    };

    // Carrier update override (SSOT): depth = depth + depth_delta, i = i + 1
    let mut carrier_updates_override: BTreeMap<String, UpdateExpr> = BTreeMap::new();
    carrier_updates_override.insert(loop_counter_name.clone(), UpdateExpr::Const(1));
    carrier_updates_override.insert(
        summary.depth_name.clone(),
        UpdateExpr::BinOp {
            lhs: summary.depth_name.clone(),
            op: BinOpKind::Add,
            rhs: UpdateRhs::Variable(depth_delta_name.clone()),
        },
    );

    PolicyDecision::Use(BalancedDepthScanPolicyResult {
        break_condition_node,
        allowed_body_locals_for_conditions: vec![summary.ch_name.clone(), depth_next_name.clone()],
        carrier_updates_override,
        derived_recipe: BalancedDepthScanRecipe {
            depth_var: summary.depth_name,
            ch_var: summary.ch_name,
            open: open.to_string(),
            close: close.to_string(),
            depth_delta_name,
            depth_next_name,
        },
        post_loop_early_return: PostLoopEarlyReturnPlan {
            cond: ASTNode::BinaryOp {
                operator: BinaryOperator::Less,
                left: Box::new(var(&loop_counter_name)),
                right: Box::new(var(&bound_name)),
                span: Span::unknown(),
            },
            ret_expr: var(&loop_counter_name),
        },
    })
}
