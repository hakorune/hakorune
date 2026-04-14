//! Phase 92 P3: BodyLocal policy routing (Box)
//!
//! Purpose: make the "promotion vs read-only slot vs reject" decision explicit,
//! so loop-break routing code does not look like it "falls back" after failure.

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::policies::PolicyDecision;
use crate::mir::builder::MirBuilder;
use crate::mir::join_ir::lowering::carrier_info::CarrierInfo;
use crate::mir::join_ir::lowering::common::body_local_derived_slot_emitter::BodyLocalDerivedSlotRecipe;
use crate::mir::join_ir::lowering::common::body_local_slot::ReadOnlyBodyLocalSlot;
use crate::mir::join_ir::lowering::loop_scope_shape::LoopScopeShape;
use crate::mir::loop_route_detection::loop_body_cond_promoter::{
    ConditionPromotionRequest, ConditionPromotionResult, LoopBodyCondPromoter,
};
use crate::mir::loop_route_detection::loop_condition_scope::{CondVarScope, LoopConditionScope};

use super::body_local_policy_helpers::{route_promoted_body_local, route_unpromoted_body_local};

/// Explicit routing policy for LoopBodyLocal variables used in loop-break conditions.
///
/// This is a "route" decision (not a fallback): we choose exactly one of the supported
/// strategies and reject otherwise.
pub enum BodyLocalRoute {
    Promotion {
        promoted_carrier: CarrierInfo,
        promoted_var: String,
        carrier_name: String,
    },
    ReadOnlySlot(ReadOnlyBodyLocalSlot),
    DerivedSlot(BodyLocalDerivedSlotRecipe),
}

pub fn classify_loop_break_body_local_route(
    _builder: &MirBuilder,
    loop_var_name: &str,
    scope: &LoopScopeShape,
    break_condition_node: &ASTNode,
    cond_scope: &LoopConditionScope,
    body: &[ASTNode],
) -> PolicyDecision<BodyLocalRoute> {
    let vars: Vec<String> = cond_scope
        .vars
        .iter()
        .filter(|v| v.scope == CondVarScope::LoopBodyLocal)
        .map(|v| v.name.clone())
        .collect();

    let promotion_req = ConditionPromotionRequest {
        loop_param_name: loop_var_name,
        cond_scope,
        scope_shape: Some(scope),
        break_cond: Some(break_condition_node),
        continue_cond: None,
        loop_body: body,
    };

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
