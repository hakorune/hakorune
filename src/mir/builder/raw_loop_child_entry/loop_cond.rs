//! The existing LoopCond terminal borrows the SAME invocation capability.
//! Source selection, expression semantics and physical completion stay owned by
//! their existing issuers. No scope is removed to make this borrow possible.
use crate::mir::{MirBuilder, ValueId};
use crate::mir::builder::normal_callable_loop_source_facts::SourceLoopCondPhysicalInputV1;
use crate::mir::builder::normal_callable_semantic_lowering_state::CallableSemanticLoweringState;
use crate::mir::builder::recursive_child_lowering::RawInvocationChildPortV1;
use crate::mir::builder::control_flow::plan::lowering_context::PlanLoweringContext;
use std::{cell::RefCell, rc::Rc};

pub(super) fn lower(builder: &mut MirBuilder, source: &SourceLoopCondPhysicalInputV1<'_, '_>,
    context: &dyn PlanLoweringContext, ledger: &Rc<RefCell<CallableSemanticLoweringState>>,
    mut invocation: Option<&mut RawInvocationChildPortV1<'_, '_>>,
) -> Result<ValueId, String> {
    let result = (|| {
        if let Some(port) = invocation.as_deref_mut() {
            if port.callable_loop_root_scope.is_none()
                || port.callable_ledger.as_ref().is_none_or(|actual| !Rc::ptr_eq(actual, ledger))
                || port.source_input.is_none_or(|input| input.owner() != source.owner())
                || port.active_source.as_ref().is_none_or(|actual| actual.site() != source.parent_source().site()
                    || !actual.shares_root_lineage(source.parent_source())) {
                return Err("[freeze:contract][callable-loop/invocation-transport-drift]".into());
            }
        }
        source.validate_for_source_port()?;
        source.preflight_source_port()?;
        if source.checked_source_recipe_v2().is_some() {
            let port = invocation.as_deref_mut().ok_or_else(|| "[freeze:contract][source-loop/invocation-port-missing]".to_owned())?;
            let claims = ledger.borrow().ordinary_new_claim_ledger().cloned();
            source.open_source_lifecycle_prefix_v2(claims.as_deref())?;
            return crate::mir::builder::control_flow::plan::lowerer::PlanLowerer::lower_checked_source_loop_v2(builder, source, port, ledger);
        }
        let claims = ledger.borrow().ordinary_new_claim_ledger().cloned();
        source.open_source_lifecycle_prefix_v2(claims.as_deref())?;
        let plan = crate::mir::builder::control_flow::plan::features::loop_cond_bc_source::lower_loop_cond_break_continue_source(builder, source)?;
        crate::mir::builder::control_flow::verify::PlanVerifier::verify(&plan)
            .map_err(|error| format!("[freeze:contract][callable-loop/verify] {error}"))?;
        let (result, after_role) = crate::mir::builder::control_flow::plan::lowerer::PlanLowerer::lower_loop_cond_with_source_publication(
            builder, plan, context, Rc::clone(ledger), source,
        ).map_err(|error| format!("[freeze:contract][callable-loop/lower] {error}"))?;
        let result = result.ok_or_else(|| "[freeze:contract][callable-loop/lower-no-value]".to_owned())?;
        if let Some(role) = after_role {
            if let Some(claims) = ledger.borrow().ordinary_new_claim_ledger() { claims.record_loop_cond_after_role(role)?; }
        }
        Ok(result)
    })();
    result.map_err(|error| source.phi_session_v1().abort_error(builder, error))
}
