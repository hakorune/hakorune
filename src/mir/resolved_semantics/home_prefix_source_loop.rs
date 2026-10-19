//! Scoped inductive Header basis. This is source analysis, never a completed
//! Loop, Home After or physical Phi permission. Body frames retain only the
//! immutable basis, never a mutable exit collector or expression inventory.
use crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1;
use super::*;
use crate::mir::builder::{CallableLoopSourceBindingClassV1 as Role, RetainedCallableLoopSourceV1};
use crate::mir::resolved_semantics::source_value_flow::{
    SourceValueEdgeV1 as Edge, SourceValueJoinKindV1 as Join, SourceValueRefV1,
    VerifiedSourceValueFlowV1,
};
use std::{rc::Rc, sync::Arc};
#[path = "home_prefix_source_loop_ports.rs"]
mod ports;
#[path = "home_prefix_source_loop_control.rs"]
mod control;
pub(crate) use ports::{SourceLoopScanSummaryV1, SourceLoopPortKindV1};
pub(crate) use control::{CheckedSourceLoopBodyV1, CheckedSourceLoopStatementV1, SourceLoopReturnV1};

#[derive(Debug)]
pub(crate) struct LoopHeaderScalarBasisV1 {
    source: Rc<RetainedCallableLoopSourceV1>,
    flow: Arc<VerifiedSourceValueFlowV1>,
    binding: BindingRefV1,
    header: SourceValueRefV1,
    entry: SourceValueRefV1,
    entry_origin: SourceValueRefV1,
    entry_producer: CurrentScalarProducerV1,
}
// The immutable, non-Clone basis is minted once per selected source scope.
impl PartialEq for LoopHeaderScalarBasisV1 {
    fn eq(&self, other: &Self) -> bool { std::ptr::eq(self, other) }
}
impl Eq for LoopHeaderScalarBasisV1 {}
impl LoopHeaderScalarBasisV1 {
    pub(crate) fn source(&self) -> &Rc<RetainedCallableLoopSourceV1> { &self.source }
    pub(crate) fn binding(&self) -> BindingRefV1 { self.binding }
    pub(crate) fn header(&self) -> SourceValueRefV1 { self.header }
    pub(crate) fn entry(&self) -> SourceValueRefV1 { self.entry }
    pub(crate) fn entry_origin(&self) -> SourceValueRefV1 { self.entry_origin }
    pub(crate) fn entry_producer(&self) -> &CurrentScalarProducerV1 { &self.entry_producer }
}

pub(super) struct SourceLoopHeaderEnvironmentV1 {
    source: Rc<RetainedCallableLoopSourceV1>,
    headers: BTreeMap<BindingRefV1, Rc<LoopHeaderScalarBasisV1>>,
    ports: std::cell::RefCell<Vec<ports::SourceLoopPortSnapshotV1>>,
    refused: std::cell::Cell<bool>,
    control: std::cell::RefCell<control::SourceLoopControlDraftV1>,
}
impl SourceLoopHeaderEnvironmentV1 {
    pub(super) fn header_for_value(&self, flow: &Arc<VerifiedSourceValueFlowV1>, value: SourceValueRefV1)
        -> Option<&Rc<LoopHeaderScalarBasisV1>> {
        let mut matching = self.headers.values().filter(|basis|
            basis.header == value && Arc::ptr_eq(&basis.flow, flow)
                && Rc::ptr_eq(&basis.source, &self.source));
        let basis = matching.next()?;
        matching.next().is_none().then_some(basis)
    }
}

pub(in crate::mir::resolved_semantics::home_new_prefix) struct SourceLoopHeaderCheckpointV1 {
    active: Rc<SourceLoopHeaderEnvironmentV1>,
    previous: Option<Rc<SourceLoopHeaderEnvironmentV1>>,
}

pub(super) fn same_context(a: &Option<Rc<SourceLoopHeaderEnvironmentV1>>,
    b: &Option<Rc<SourceLoopHeaderEnvironmentV1>>) -> bool {
    match (a, b) { (None, None) => true, (Some(a), Some(b)) => Rc::ptr_eq(a, b), _ => false }
}

impl PrefixLocalFlow<'_> {
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn stage_source_loop_headers_v1(
        &mut self, source: Rc<RetainedCallableLoopSourceV1>, homes: &[LiveHomeV1],
        calls: &[super::super::LocalCallObservationV1],
    ) -> Option<SourceLoopHeaderCheckpointV1> {
        source.condition_source_v1(self.input)?;
        if source.bindings().owner() != self.input.owner()
            || source.bindings().loop_site() != source.site().node() { return None; }
        let flow = self.input.function().expression_source().value_flow()?;
        let mut headers = BTreeMap::new();
        for (binding, role) in source.bindings().bindings() {
            if role != Role::Carrier { continue; }
            if !self.can_rebind_integer_v1(binding, homes) { return None; }
            let (header, incoming) = flow.loop_join(source.site(), binding, Join::LoopHeader)?;
            let mut entries = incoming.iter().filter_map(|(edge, value)| (*edge == Edge::LoopEntry).then_some(*value));
            let entry = entries.next()?;
            if entries.next().is_some() || incoming.iter().any(|(edge, _)|
                !matches!(edge, Edge::LoopEntry | Edge::Backedge | Edge::Continue(_))) { return None; }
            let (entry_origin, entry_producer) = self.scalar_producer_for_value_v1(binding, entry, calls)?;
            let basis = Rc::new(LoopHeaderScalarBasisV1 {
                source: Rc::clone(&source), flow: Arc::clone(flow), binding,
                header, entry, entry_origin, entry_producer,
            });
            if headers.insert(binding, basis).is_some() { return None; }
        }
        if headers.is_empty() { return None; }
        let active = Rc::new(SourceLoopHeaderEnvironmentV1 { source, headers, ports: Default::default(), refused: Default::default(), control: Default::default() });
        let previous = self.source_loop.replace(Rc::clone(&active));
        Some(SourceLoopHeaderCheckpointV1 { active, previous })
    }

    /// Restore outer assumptions. The summary retains no expression inventory
    /// and grants no After; every source port still needs whole Recipe sealing.
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn finish_source_loop_headers_v1(
        &mut self, checkpoint: SourceLoopHeaderCheckpointV1,
        condition: Rc<super::super::CheckedBoolConditionFrameV1>,
        calls: &[super::super::LocalCallObservationV1],
    ) -> Option<SourceLoopScanSummaryV1> {
        if !self.source_loop.as_ref().is_some_and(|active| Rc::ptr_eq(active, &checkpoint.active)) { return None; }
        if !Rc::ptr_eq(condition.source(), checkpoint.active.source.condition_source_v1(self.input)?) { return None; }
        let snapshots = std::mem::take(&mut *checkpoint.active.ports.borrow_mut());
        let incoming = ports::verify_incoming(self, &checkpoint.active.source, &checkpoint.active.headers,
            &snapshots, checkpoint.active.refused.get(), calls);
        let prefix = checkpoint.active.source.site().node().segments();
        let body_expressions: Box<[_]> = self.expression_plans().into_iter().filter(|plan| {
            let segments = plan.source().root().statement().node().segments();
            segments.starts_with(prefix)
                && matches!(segments.get(prefix.len()), Some(crate::mir::resolved_semantics::SourcePathSegmentV1::LoopBody(_)))
        }).collect();
        let bool_expressions = std::iter::once(&condition)
            .chain(body_expressions.iter().filter_map(|plan| plan.condition_frame()))
            .map(|frame| crate::mir::loop_recipe_contract::project_source_loop_bool_v1(Rc::clone(frame)))
            .collect();
        let statement = self.input.source().exact_stmt(checkpoint.active.source.site()).ok()?;
        let body = self.input.source().child_body_from_stmt(&statement, crate::mir::resolved_semantics::BodyChildRoleV1::LoopBody).ok()?;
        let control = std::mem::take(&mut *checkpoint.active.control.borrow_mut()).finish(body.site());
        self.source_loop = checkpoint.previous;
        Some(SourceLoopScanSummaryV1 {
            condition, refused: checkpoint.active.refused.get(),
            source: Rc::clone(&checkpoint.active.source),
            headers: checkpoint.active.headers.clone(),
            ports: snapshots, incoming, body_expressions, bool_expressions, control,
        })
    }

    pub(in crate::mir::resolved_semantics::home_new_prefix) fn note_source_loop_availability_v1(&self, available: bool) {
        if !available { if let Some(context) = &self.source_loop { context.refused.set(true); } }
    }

    pub(in crate::mir::resolved_semantics::home_new_prefix) fn in_source_loop_v1(&self) -> bool {
        self.source_loop.is_some()
    }
}

#[cfg(test)]
pub(crate) use ports::tests::assert_original_loop_source_walk_for_test;

impl PrefixLocalFlow<'_> {
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn apply_source_loop_after_v2(&mut self,
        product: std::rc::Rc<crate::mir::loop_recipe_contract::VerifiedSourceLoopRecipeProductV2>,
        homes: &[LiveHomeV1],
    ) -> bool {
        if product.source.source.owner() != self.input.owner() || product.source.condition.prior_homes() != homes
            || product.source.headers.keys().any(|binding| !self.can_rebind_integer_v1(*binding, homes)) { return false; }
        let mut plans = self.expression_plans.borrow_mut();
        let indices: Vec<_> = plans.iter().enumerate().filter_map(|(index, plan)|
            std::rc::Rc::ptr_eq(plan.source(), product.source.condition.source()).then_some(index)).collect();
        if indices.len() != 1 { return false; }
        let updated: Vec<_> = product.source.headers.keys().copied().collect();
        if !plans[indices[0]].attach_loop_recipe_v2(product) { return false; }
        drop(plans);
        // All checks precede publication. Only original carried scalars change;
        // iteration-local Homes remain in the verified per-port discharge.
        for binding in updated { self.store(binding, StoredLocal::Trivial(Some(SourceScalarKind::Integer))); }
        true
    }
    pub(super) fn source_loop_after_for_value_v2(&self,
        flow: &Arc<VerifiedSourceValueFlowV1>, value: SourceValueRefV1,
    ) -> Option<CurrentScalarProducerV1> {
        let plans = self.expression_plans.borrow();
        let mut matching = plans.iter().filter_map(|plan| plan.loop_recipe_v2()).flat_map(|product| {
            product.source.incoming.as_ref().ok().into_iter().flat_map(move |rows| rows.iter())
                .filter(move |row| row.after == value && Arc::ptr_eq(&row.basis.flow, flow))
                .map(move |row| (product, row.basis.binding()))
        });
        let (product, binding) = matching.next()?;
        if matching.next().is_some() { return None; }
        Some(CurrentScalarProducerV1::LoopAfter { product: Rc::clone(product), binding })
    }
}
