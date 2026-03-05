//! Phase 92 P3: BodyLocal policy routing (Box)
//!
//! Purpose: make the "promotion vs read-only slot vs reject" decision explicit,
//! so Pattern2 code does not look like it "falls back" after failure.

use crate::ast::ASTNode;
use crate::mir::builder::MirBuilder;
use crate::mir::builder::control_flow::plan::policies::PolicyDecision;
use crate::mir::builder::control_flow::plan::pattern2::contracts::derived_slot::extract_derived_slot_for_conditions;
use crate::mir::join_ir::lowering::carrier_info::CarrierInfo;
use crate::mir::join_ir::lowering::common::body_local_derived_slot_emitter::BodyLocalDerivedSlotRecipe;
use crate::mir::join_ir::lowering::common::body_local_slot::{
    ReadOnlyBodyLocalSlot, ReadOnlyBodyLocalSlotBox,
};
use crate::mir::join_ir::lowering::loop_scope_shape::LoopScopeShape;
use crate::mir::loop_pattern_detection::loop_body_cond_promoter::{
    ConditionPromotionRequest, ConditionPromotionResult, LoopBodyCondPromoter,
};
use crate::mir::loop_pattern_detection::loop_condition_scope::{CondVarScope, LoopConditionScope};

/// Explicit routing policy for LoopBodyLocal variables used in Pattern2 conditions.
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
        #[cfg(feature = "normalized_dev")]
        binding_map: Some(&_builder.binding_map),
    };

    match LoopBodyCondPromoter::try_promote_for_condition(promotion_req) {
        ConditionPromotionResult::Promoted {
            carrier_info: promoted_carrier,
            promoted_var,
            carrier_name,
        } => match extract_derived_slot_for_conditions(&vars, body) {
            Ok(Some(recipe)) => PolicyDecision::Use(BodyLocalRoute::DerivedSlot(recipe)),
            Ok(None) => PolicyDecision::Use(BodyLocalRoute::Promotion {
                promoted_carrier,
                promoted_var,
                carrier_name,
            }),
            Err(slot_err) => PolicyDecision::Reject(format!(
                "[loop_break/body_local_policy] derived-slot check failed: {slot_err}"
            )),
        },
        ConditionPromotionResult::CannotPromote { reason, .. } => {
            match extract_derived_slot_for_conditions(&vars, body) {
                Ok(Some(recipe)) => PolicyDecision::Use(BodyLocalRoute::DerivedSlot(recipe)),
                Ok(None) => match extract_body_local_inits_for_conditions(&vars, body) {
                    Ok(Some(slot)) => PolicyDecision::Use(BodyLocalRoute::ReadOnlySlot(slot)),
                    Ok(None) => PolicyDecision::Reject(reason),
                    Err(slot_err) => PolicyDecision::Reject(format!(
                        "{reason}; read-only-slot rejected: {slot_err}"
                    )),
                },
                Err(slot_err) => PolicyDecision::Reject(format!(
                    "{reason}; derived-slot rejected: {slot_err}"
                )),
            }
        }
    }
}

#[allow(dead_code)]
pub fn classify_for_pattern2(
    builder: &MirBuilder,
    loop_var_name: &str,
    scope: &LoopScopeShape,
    break_condition_node: &ASTNode,
    cond_scope: &LoopConditionScope,
    body: &[ASTNode],
) -> PolicyDecision<BodyLocalRoute> {
    classify_loop_break_body_local_route(
        builder,
        loop_var_name,
        scope,
        break_condition_node,
        cond_scope,
        body,
    )
}

fn extract_body_local_inits_for_conditions(
    body_local_names_in_conditions: &[String],
    body: &[ASTNode],
) -> Result<Option<ReadOnlyBodyLocalSlot>, String> {
    if body_local_names_in_conditions.is_empty() {
        return Ok(None);
    }
    Ok(Some(ReadOnlyBodyLocalSlotBox::extract_single(
        body_local_names_in_conditions,
        body,
    )?))
}
