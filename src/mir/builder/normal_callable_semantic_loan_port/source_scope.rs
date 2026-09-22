use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::normal_callable_semantic_package::{
    LoopBreakSourcePackageLoanV1, LoopBreakSourcePackageTakeHandle, OrdinaryNewClaimLedgerV1,
};
use crate::parser::CallableMethodSourceObservationV1;

use super::super::normal_callable_semantic_lowering_state::CallableSemanticLoweringState;
use super::super::raw_invocation_source_transport::{
    RawInvocationRootLineageV1, RawInvocationSourceTransportV1,
};
use super::super::recursive_child_lowering::RawInvocationChildPortV1;
use super::SelectedCallableLoweringInputRefV1;
use crate::mir::normal_callable_semantic_package::SelectedSourceCoreMethodCallV1;

pub(super) fn with_selected_source_scope<'port, 'collector, R>(
    inner: &mut RawInvocationChildPortV1<'port, 'collector>,
    lineage: RawInvocationRootLineageV1,
    input: SelectedCallableLoweringInputRefV1<'_>,
    core_method_calls: BTreeMap<
        crate::mir::resolved_semantics::SourceExprSiteV1,
        SelectedSourceCoreMethodCallV1,
    >,
    named_array_emissions: Rc<
        crate::mir::normal_callable_semantic_package::NamedArrayEmissionCollectorV1,
    >,
    ordinary_new_claim_ledger: Rc<OrdinaryNewClaimLedgerV1>,
    loop_break_take: Option<LoopBreakSourcePackageTakeHandle<'_>>,
    execute: impl FnOnce(
        &mut RawInvocationChildPortV1<'port, 'collector>,
        RawInvocationSourceTransportV1<()>,
    ) -> Result<R, String>,
) -> Result<R, String> {
    for row in core_method_calls.values() {
        row.require_selected(input.selected_key(), input.source().owner())?;
    }
    let dynamic_source = match input.semantic() {
        crate::mir::normal_callable_semantic_package::SelectedCallableSemanticRefV1::Dynamic {
            source,
            ..
        } => Some(Rc::clone(source)),
        crate::mir::normal_callable_semantic_package::SelectedCallableSemanticRefV1::Ordinary => {
            None
        }
    };
    with_callable_source_scope(
        inner,
        lineage,
        input.source(),
        dynamic_source,
        core_method_calls,
        input.method_source_observation().cloned(),
        Some(named_array_emissions),
        ordinary_new_claim_ledger,
        loop_break_take
            .map(|take| take.take_for_owner(input.source().owner()))
            .transpose()?,
        execute,
    )
}

pub(super) fn with_callable_source_scope<'port, 'collector, R>(
    inner: &mut RawInvocationChildPortV1<'port, 'collector>,
    lineage: RawInvocationRootLineageV1,
    input: ResolvedFunctionLoweringInputV1<'_>,
    dynamic_source: Option<Rc<crate::mir::builder::VerifiedSourceBackedDynamicCallableV1>>,
    core_method_calls: BTreeMap<
        crate::mir::resolved_semantics::SourceExprSiteV1,
        SelectedSourceCoreMethodCallV1,
    >,
    observation: Option<CallableMethodSourceObservationV1>,
    named_array_emissions: Option<
        Rc<crate::mir::normal_callable_semantic_package::NamedArrayEmissionCollectorV1>,
    >,
    ordinary_new_claim_ledger: Rc<OrdinaryNewClaimLedgerV1>,
    loop_break_source: Option<LoopBreakSourcePackageLoanV1>,
    execute: impl FnOnce(
        &mut RawInvocationChildPortV1<'port, 'collector>,
        RawInvocationSourceTransportV1<()>,
    ) -> Result<R, String>,
) -> Result<R, String> {
    let transport = RawInvocationSourceTransportV1::root((), lineage);
    let state = CallableSemanticLoweringState::from_exact_source_with_dynamic_source_and_core_methods_and_loop_break_source(
        input,
        dynamic_source,
        core_method_calls,
        loop_break_source,
    )?;
    let state = Rc::new(RefCell::new(state));
    let script_ledger = inner.semantic_ledger.take();
    let parent_callable = inner.callable_ledger.replace(state.clone());
    let parent_ordinary_new_claim_ledger = inner
        .ordinary_new_claim_ledger
        .replace(ordinary_new_claim_ledger);
    let result = inner
        .with_callable_method_source_observation(observation, |inner| execute(inner, transport));
    inner.callable_ledger = parent_callable;
    inner.ordinary_new_claim_ledger = parent_ordinary_new_claim_ledger;
    inner.semantic_ledger = script_ledger;
    match result {
        Ok(value) => {
            let rows = Rc::try_unwrap(state)
                .map_err(|_| "[freeze:contract][mir/callable-semantic/ledger-loan]".to_owned())?
                .into_inner()
                .finish_with_named_arrays()?;
            match named_array_emissions {
                Some(collector) => collector.hand_back(rows)?,
                None if rows.is_empty() => {}
                None => return Err("[freeze:contract][named-array/collector-missing]".into()),
            }
            Ok(value)
        }
        Err(error) => Err(error),
    }
}
