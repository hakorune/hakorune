//! Trim pattern policy box (判定専用).
//!
//! 目的: Trim 形状かどうかを判定し、ConditionScope を返すだけに責務を絞る。
//! 生成（lowering）は従来通り `TrimLoopLowerer` 側が担当する。

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::loop_break::contracts::derived_slot::extract_body_local_derived_slot;
use crate::mir::join_ir::lowering::loop_scope_shape::LoopScopeShape;
use crate::mir::loop_route_detection::loop_condition_scope::{
    CondVarInfo, CondVarScope, LoopConditionScope, LoopConditionScopeBox,
};

use super::PolicyDecision;

/// 判定結果（生成に必要な最小情報だけ運ぶ）
#[derive(Debug, Clone)]
pub struct TrimPolicyResult {
    pub cond_scope: LoopConditionScope,
    pub condition_body_locals: Vec<CondVarInfo>,
}

pub fn classify_trim_like_loop(
    scope: &LoopScopeShape,
    loop_cond: &ASTNode,
    break_cond: &ASTNode,
    body: &[ASTNode],
    loop_var_name: &str,
) -> PolicyDecision<TrimPolicyResult> {
    let cond_scope =
        LoopConditionScopeBox::analyze(loop_var_name, &[loop_cond, break_cond], Some(scope));

    if !cond_scope.has_loop_body_local() {
        return PolicyDecision::None;
    }

    let condition_body_locals: Vec<_> = cond_scope
        .vars
        .iter()
        .filter(|v| v.scope == CondVarScope::LoopBodyLocal)
        .filter(|v| is_var_used_in_condition(&v.name, break_cond))
        .cloned()
        .collect();

    if condition_body_locals.is_empty() {
        return PolicyDecision::None;
    }

    if condition_body_locals.len() == 1 {
        match extract_body_local_derived_slot(&condition_body_locals[0].name, body) {
            Ok(Some(_)) => return PolicyDecision::None,
            Ok(None) => {}
            Err(reason) => {
                return PolicyDecision::Reject(format!(
                    "[trim_policy] derived-slot check failed: {reason}"
                ));
            }
        }
    }

    PolicyDecision::Use(TrimPolicyResult {
        cond_scope,
        condition_body_locals,
    })
}

fn is_var_used_in_condition(var_name: &str, cond_node: &ASTNode) -> bool {
    match cond_node {
        ASTNode::Variable { name, .. } => name == var_name,
        ASTNode::BinaryOp { left, right, .. } => {
            is_var_used_in_condition(var_name, left) || is_var_used_in_condition(var_name, right)
        }
        ASTNode::UnaryOp { operand, .. } => is_var_used_in_condition(var_name, operand),
        ASTNode::MethodCall {
            object, arguments, ..
        } => {
            is_var_used_in_condition(var_name, object)
                || arguments
                    .iter()
                    .any(|arg| is_var_used_in_condition(var_name, arg))
        }
        _ => false,
    }
}
