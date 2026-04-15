use crate::ast::ASTNode;
use crate::mir::builder::control_flow::cleanup::policies::body_local_derived_slot::extract_derived_slot_for_conditions;
use crate::mir::builder::control_flow::cleanup::policies::PolicyDecision;
use crate::mir::builder::control_flow::plan::body_local_policy::BodyLocalRoute;
use crate::mir::join_ir::lowering::carrier_info::CarrierInfo;
use crate::mir::join_ir::lowering::common::body_local_slot::{
    ReadOnlyBodyLocalSlot, ReadOnlyBodyLocalSlotBox,
};

pub(super) fn route_promoted_body_local(
    vars: &[String],
    body: &[ASTNode],
    promoted_carrier: CarrierInfo,
    promoted_var: String,
    carrier_name: String,
) -> PolicyDecision<BodyLocalRoute> {
    match extract_derived_slot_for_conditions(vars, body) {
        Ok(Some(recipe)) => PolicyDecision::Use(BodyLocalRoute::DerivedSlot(recipe)),
        Ok(None) => PolicyDecision::Use(BodyLocalRoute::Promotion {
            promoted_carrier,
            promoted_var,
            carrier_name,
        }),
        Err(slot_err) => PolicyDecision::Reject(format!(
            "[loop_break/body_local_policy] derived-slot check failed: {slot_err}"
        )),
    }
}

pub(super) fn route_unpromoted_body_local(
    vars: &[String],
    body: &[ASTNode],
    reason: String,
) -> PolicyDecision<BodyLocalRoute> {
    match extract_derived_slot_for_conditions(vars, body) {
        Ok(Some(recipe)) => PolicyDecision::Use(BodyLocalRoute::DerivedSlot(recipe)),
        Ok(None) => match extract_body_local_inits_for_conditions(vars, body) {
            Ok(Some(slot)) => PolicyDecision::Use(BodyLocalRoute::ReadOnlySlot(slot)),
            Ok(None) => PolicyDecision::Reject(reason),
            Err(slot_err) => {
                PolicyDecision::Reject(format!("{reason}; read-only-slot rejected: {slot_err}"))
            }
        },
        Err(slot_err) => {
            PolicyDecision::Reject(format!("{reason}; derived-slot rejected: {slot_err}"))
        }
    }
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
