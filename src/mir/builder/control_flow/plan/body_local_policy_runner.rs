use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::policies::PolicyDecision;
use crate::mir::builder::MirBuilder;
use crate::mir::join_ir::lowering::loop_scope_shape::LoopScopeShape;
use crate::mir::loop_route_detection::loop_body_cond_promoter::{
    ConditionPromotionResult, LoopBodyCondPromoter,
};
use crate::mir::loop_route_detection::loop_condition_scope::LoopConditionScope;

use super::body_local_policy_helpers::{route_promoted_body_local, route_unpromoted_body_local};
use super::body_local_policy_inputs::{
    build_condition_promotion_request, collect_body_local_condition_vars,
};
use super::body_local_policy_types::BodyLocalRoute;

pub(super) fn classify_body_local_policy_route(
    _builder: &MirBuilder,
    loop_var_name: &str,
    scope: &LoopScopeShape,
    break_condition_node: &ASTNode,
    cond_scope: &LoopConditionScope,
    body: &[ASTNode],
) -> PolicyDecision<BodyLocalRoute> {
    let vars = collect_body_local_condition_vars(cond_scope);
    let promotion_req = build_condition_promotion_request(
        loop_var_name,
        scope,
        break_condition_node,
        cond_scope,
        body,
    );

    match LoopBodyCondPromoter::try_promote_for_condition(promotion_req) {
        ConditionPromotionResult::Promoted {
            carrier_info: promoted_carrier,
            promoted_var,
            carrier_name,
        } => route_promoted_body_local(&vars, body, promoted_carrier, promoted_var, carrier_name),
        ConditionPromotionResult::CannotPromote { reason, .. } => {
            route_unpromoted_body_local(&vars, body, reason)
        }
    }
}
