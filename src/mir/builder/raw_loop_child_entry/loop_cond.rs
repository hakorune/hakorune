//! The existing LoopCond terminal borrows the SAME invocation capability.
//! Source selection, expression semantics and physical completion stay owned by
//! their existing issuers. No scope is removed to make this borrow possible.
use crate::mir::{MirBuilder, ValueId};
use crate::mir::builder::normal_callable_loop_source_facts::SourceLoopCondPhysicalInputV1;
use crate::mir::builder::normal_callable_semantic_lowering_state::CallableSemanticLoweringState;
use crate::mir::builder::recursive_child_lowering::RawInvocationChildPortV1;
use std::{cell::RefCell, rc::Rc};

pub(super) fn lower(builder: &mut MirBuilder, source: &SourceLoopCondPhysicalInputV1<'_, '_>,
    ledger: &Rc<RefCell<CallableSemanticLoweringState>>,
    mut invocation: Option<&mut RawInvocationChildPortV1<'_, '_>>,
) -> Result<ValueId, String> {
    let result = (|| {
        if let Some(port) = invocation.as_deref_mut() {
            if port.callable_loop_root_scope.is_none()
                || port.source_ledger.callable().is_none_or(|actual| !Rc::ptr_eq(actual, ledger))
                || port.source_input.is_none_or(|input| input.owner() != source.owner())
                || port.active_source.as_ref().is_none_or(|actual| actual.site() != source.parent_source().site()
                    || !actual.shares_root_lineage(source.parent_source())) {
                return Err("[freeze:contract][callable-loop/invocation-transport-drift]".into());
            }
        }
        source.validate_for_source_port()?;
        source.preflight_source_port()?;
        let port = invocation.as_deref_mut().ok_or_else(|| "[freeze:contract][source-loop/invocation-port-missing]".to_owned())?;
        let claims = ledger.borrow().ordinary_new_claim_ledger().cloned();
        source.open_source_lifecycle_prefix_v2(claims.as_deref())?;
        crate::mir::builder::control_flow::plan::lowerer::PlanLowerer::lower_checked_source_loop_v2(builder, source, port, ledger)
    })();
    result.map_err(|error| source.phi_session_v1().abort_error(builder, error))
}
