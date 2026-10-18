use super::PreparedLocatedRawLoopChildEntryV1;
use crate::mir::builder::control_flow::plan::LoopFactsPolicyFrameV1;
#[cfg(test)]
use crate::mir::builder::module_invocation_session::UnpublishedCallableLoopRootScopeV1;
use crate::mir::builder::normal_callable_loop_source_route::CallableLoopSourceTargetProbeV1;
use crate::mir::builder::normal_callable_semantic_lowering_state::CallableSemanticLoweringState;
use crate::mir::builder::MirBuilder;
#[cfg(test)]
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::ValueId;
use std::cell::RefCell;
use std::rc::Rc;

impl<'source> PreparedLocatedRawLoopChildEntryV1<'source> {
    pub(in crate::mir::builder) fn lower_with_invocation_port_v2(
        self, builder: &mut MirBuilder, function_name: &str, debug: bool, in_static_box: bool,
        policy: LoopFactsPolicyFrameV1,
        invocation: &mut crate::mir::builder::recursive_child_lowering::RawInvocationChildPortV1<'_, '_>,
        source_target_probe: CallableLoopSourceTargetProbeV1,
    ) -> Result<ValueId, String> {
        let ledger = invocation.callable_ledger.clone()
            .ok_or("[freeze:contract][raw-loop-child-entry/callable-ledger-missing]")?;
        let locator = invocation.declared_instance_locator;
        let input = invocation.source_input.ok_or("[freeze:contract][raw-loop-child-entry/source-input-missing]")?;
        self.lower_with_callable_source_v2(builder, function_name, debug, in_static_box, policy,
            Some(invocation), Some(&ledger), locator, Some(input), source_target_probe)
    }

    #[cfg(test)]
    pub(in crate::mir::builder) fn lower_v1_with_root_scope_and_callable_ledger(
        self,
        builder: &mut MirBuilder,
        function_name: &str,
        debug: bool,
        in_static_box: bool,
        policy: LoopFactsPolicyFrameV1,
        _callable_loop_root_scope: &mut UnpublishedCallableLoopRootScopeV1,
        callable_ledger: &Rc<RefCell<CallableSemanticLoweringState>>,
        declared_instance_locator: Option<
            crate::mir::normal_callable_semantic_package::DeclaredInstanceCallLocatorScopeV1<'_>,
        >,
        source_input: Option<ResolvedFunctionLoweringInputV1<'_>>,
        source_target_probe: CallableLoopSourceTargetProbeV1,
    ) -> Result<ValueId, String> {
        self.lower_with_callable_source_v2(
            builder,
            function_name,
            debug,
            in_static_box,
            policy,
            None,
            Some(callable_ledger),
            declared_instance_locator,
            source_input,
            source_target_probe,
        )
    }
}

/// The existing semantic ledger owns the affine source-planner transport.
pub(super) fn take_preissued_source_plan_v1(
    ledger: Option<&Rc<RefCell<CallableSemanticLoweringState>>>,
    owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
    site: Option<&crate::mir::resolved_semantics::SourceNodeSiteV1>,
) -> Result<Option<crate::mir::builder::PreissuedCallableLoopSourcePlanV1>, String> {
    match (ledger, site) {
        (Some(ledger), Some(site)) => match ledger.borrow().ordinary_new_claim_ledger() {
            Some(claims) => claims.take_preissued_loop_source_handoff_v2(owner,
                &crate::mir::resolved_semantics::SourceStmtSiteV1::from_node(site.clone())),
            None => Ok(None),
        },
        _ => Ok(None),
    }
}

/// Respect a prior positive canonical selection before unrelated legacy probes.
/// The paired handoff is still consumed exactly once by the existing issuer.
pub(super) fn has_checked_source_selection_v2(
    ledger: Option<&Rc<RefCell<CallableSemanticLoweringState>>>,
    owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
    site: Option<&crate::mir::resolved_semantics::SourceNodeSiteV1>,
) -> Result<bool, String> {
    let (Some(ledger), Some(site)) = (ledger, site) else { return Ok(false) };
    let state = ledger.borrow();
    let selected = match state.ordinary_new_claim_ledger() {
        Some(claims) => claims.retained_source_loop_recipe_v2(owner,
            &crate::mir::resolved_semantics::SourceStmtSiteV1::from_node(site.clone()))?.is_some(),
        None => false,
    };
    if selected && state.owner() != owner { return Err("[freeze:contract][callable-loop/selected-source-owner]".into()); }
    Ok(selected)
}
