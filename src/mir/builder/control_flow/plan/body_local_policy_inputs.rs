use crate::ast::ASTNode;
use crate::mir::join_ir::lowering::loop_scope_shape::LoopScopeShape;
use crate::mir::loop_route_detection::loop_body_cond_promoter::ConditionPromotionRequest;
use crate::mir::loop_route_detection::support::condition_scope::{
    CondVarScope, LoopConditionScope,
};

pub(super) fn collect_body_local_condition_vars(cond_scope: &LoopConditionScope) -> Vec<String> {
    cond_scope
        .vars
        .iter()
        .filter(|v| v.scope == CondVarScope::LoopBodyLocal)
        .map(|v| v.name.clone())
        .collect()
}

pub(super) fn build_condition_promotion_request<'a>(
    loop_var_name: &'a str,
    scope: &'a LoopScopeShape,
    break_condition_node: &'a ASTNode,
    cond_scope: &'a LoopConditionScope,
    body: &'a [ASTNode],
) -> ConditionPromotionRequest<'a> {
    ConditionPromotionRequest {
        loop_param_name: loop_var_name,
        cond_scope,
        scope_shape: Some(scope),
        break_cond: Some(break_condition_node),
        continue_cond: None,
        loop_body: body,
    }
}
