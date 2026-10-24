//! Context-free source exit snapshots. The collector never owns a Prefix clone
//! or the shared expression inventory, so completed Loop products cannot cycle.
use super::*;
use crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1;
use crate::mir::resolved_semantics::{
    ResolvedControlTransferV1 as Transfer, ResolvedExitOriginV1 as Origin, ResolvedExitSiteV1,
    ScopeId, ScopeKindV1, SourceStmtSiteV1,
};

#[derive(Clone, PartialEq, Eq)]
pub(crate) enum SourceLoopPortKindV1 {
    Backedge,
    Break(SourceStmtSiteV1),
    Continue(SourceStmtSiteV1),
}

#[derive(Clone)]
pub(crate) struct SourceLoopPortSnapshotV1 {
    pub(crate) kind: SourceLoopPortKindV1,
    // Exact path classes, not the mutable Loop collector or an After class.
    locals: BTreeMap<BindingRefV1, StoredLocal>,
    homes: Box<[LiveHomeV1]>,
    crossed: Box<[ScopeId]>,
    normal_ends: Box<[LiveHomeV1]>,
    covered: Box<[SourceStmtSiteV1]>,
    calls: BTreeSet<OwnedExprSiteV1>,
    pub(crate) unavailable: Option<super::super::super::HomePrefixUnavailableV1>,
}

impl SourceLoopPortSnapshotV1 {
    pub(crate) fn homes(&self) -> &[LiveHomeV1] {
        &self.homes
    }
    pub(crate) fn normal_ends(&self) -> &[LiveHomeV1] {
        &self.normal_ends
    }
    pub(crate) fn crossed_scopes(&self) -> &[ScopeId] {
        &self.crossed
    }
}

pub(crate) struct SourceLoopScanSummaryV1 {
    pub(crate) condition: Rc<super::super::super::CheckedBoolConditionFrameV1>,
    pub(crate) refused: bool,
    pub(crate) source: Rc<RetainedCallableLoopSourceV1>,
    pub(crate) headers: BTreeMap<BindingRefV1, Rc<LoopHeaderValueBasisV1>>,
    pub(crate) ports: Vec<SourceLoopPortSnapshotV1>,
    pub(crate) incoming: Result<Box<[CheckedSourceLoopBindingIncomingV1]>, &'static str>,
    // Original body expression roots, including roots with no carrier write.
    // Own immutable frames, never the shared inventory or this Loop's slot.
    pub(crate) body_expressions: Box<[super::super::super::HomeExpressionPlanV1]>,
    pub(crate) bool_expressions:
        Result<Box<[crate::mir::loop_recipe_contract::CheckedLoopBoolExpressionV1]>, String>,
    pub(crate) control: Result<super::control::CheckedSourceLoopBodyV1, &'static str>,
}

/// Complete original predecessor relations and their checked source producers.
/// These are inputs to Recipe construction, never a Home After/physical grant.
#[derive(Debug)]
pub(crate) struct CheckedSourceLoopBindingIncomingV1 {
    pub(crate) basis: Rc<LoopHeaderValueBasisV1>,
    pub(crate) after: SourceValueRefV1,
    pub(crate) header_incoming: Box<[CheckedSourceLoopIncomingValueV1]>,
    pub(crate) after_incoming: Box<[CheckedSourceLoopIncomingValueV1]>,
}

#[derive(Debug)]
pub(crate) struct CheckedSourceLoopIncomingValueV1 {
    pub(crate) edge: Edge,
    pub(crate) value: SourceValueRefV1,
    pub(crate) origin: SourceValueRefV1,
    pub(crate) producer: SourceLoopValueProducerV1,
}

fn port_edge(kind: &SourceLoopPortKindV1) -> Edge {
    match kind {
        SourceLoopPortKindV1::Backedge => Edge::Backedge,
        SourceLoopPortKindV1::Break(site) => Edge::Break(site.clone()),
        SourceLoopPortKindV1::Continue(site) => Edge::Continue(site.clone()),
    }
}

fn exact_edges(incoming: &[(Edge, SourceValueRefV1)], expected: &[Edge]) -> bool {
    incoming.len() == expected.len()
        && expected.iter().enumerate().all(|(index, edge)| {
            !expected[..index].contains(edge)
                && incoming.iter().filter(|(actual, _)| actual == edge).count() == 1
        })
}

fn producer_is_covered(
    producer: &CurrentScalarProducerV1,
    port: &SourceLoopPortSnapshotV1,
) -> bool {
    let evaluation = match producer {
        CurrentScalarProducerV1::Initializer(frame) => frame.evaluation(),
        CurrentScalarProducerV1::BindingRebind(frame) => frame.evaluation(),
        CurrentScalarProducerV1::Call(call) => return port.calls.contains(call.site()),
        CurrentScalarProducerV1::Integer { .. }
        | CurrentScalarProducerV1::SignedEntry(_)
        | CurrentScalarProducerV1::LoopHeader(_) => return true,
        CurrentScalarProducerV1::LoopAfter { product, .. } => {
            return port.covered.contains(product.source.source.site())
        }
        CurrentScalarProducerV1::BranchNormal(source) => {
            return source.corroborates_original()
                && port.covered.contains(source.site())
                && source
                    .incoming()
                    .iter()
                    .all(|(_, _, producer)| producer_is_covered(producer, port))
        }
    };
    port.covered
        .contains(evaluation.source().root().statement())
        && evaluation
            .calls()
            .iter()
            .all(|call| port.calls.contains(call.site()))
        && evaluation
            .references()
            .values()
            .all(|reference| match reference {
                super::super::super::CheckedExpressionReferenceV1::Scalar(reference) => reference
                    .original_read()
                    .is_some_and(|read| producer_is_covered(read.producer(), port)),
                _ => true,
            })
}

fn text_producer_is_covered(
    producer: &SourceLoopTextProducerV1,
    port: &SourceLoopPortSnapshotV1,
    source: &Rc<RetainedCallableLoopSourceV1>,
) -> bool {
    match producer {
        SourceLoopTextProducerV1::Static { .. } => true,
        SourceLoopTextProducerV1::Residence(acquisition) => port
            .covered
            .contains(acquisition.source().root().statement()),
        SourceLoopTextProducerV1::Header(basis) => Rc::ptr_eq(basis.source(), source),
        SourceLoopTextProducerV1::After { product, .. } => {
            port.covered.contains(product.source.source.site())
        }
        SourceLoopTextProducerV1::BranchNormal(branch) => {
            branch.corroborates_original() && port.covered.contains(branch.site())
        }
    }
}

fn dynamic_producer_is_covered(
    producer: &SourceLoopDynamicProducerV1,
    port: &SourceLoopPortSnapshotV1,
    source: &Rc<RetainedCallableLoopSourceV1>,
    flow: &Arc<VerifiedSourceValueFlowV1>,
) -> bool {
    match producer {
        SourceLoopDynamicProducerV1::Initializer(frame) => {
            frame.is_dynamic_value() && frame.source().owner() == source.owner()
                && port.covered.contains(frame.source().root().statement())
                && frame.dynamic_home_binding_v1().is_some_and(|owner| port.homes.contains(&LiveHomeV1::Stable(owner)))
        }
        SourceLoopDynamicProducerV1::BindingRebind(frame) => {
            frame.is_dynamic_loop_carrier_v1() && frame.source().owner() == source.owner()
                && port.covered.contains(frame.source().root().statement())
                && frame.evaluation().calls().iter().all(|call| port.calls.contains(call.site()))
        }
        SourceLoopDynamicProducerV1::Source(point) => {
            if !point.proves(flow, point.binding(), point.value()) {
                return false;
            }
            match flow.origin(point.value()) {
                Some(SourceValueOriginV1::Entry(binding)) => *binding == point.binding(),
                Some(SourceValueOriginV1::Expression(site)) => port.covered.iter().any(|statement| {
                    site.node()
                        .segments()
                        .starts_with(statement.node().segments())
                }),
                // A Loop join requires its source-owned Header/After producer;
                // a Dynamic class alone cannot choose an incoming edge.
                Some(SourceValueOriginV1::Join { .. }) | Some(SourceValueOriginV1::Unknown) | None => false,
            }
        }
        SourceLoopDynamicProducerV1::Header(basis) => {
            basis.binding().owner() == source.owner()
                && Rc::ptr_eq(basis.source(), source)
                && basis.flow.owner() == flow.owner()
                && Arc::ptr_eq(&basis.flow, flow)
                && basis.entry_producer().class()
                    == crate::mir::resolved_value_profile::expression_class::ExpressionValueClassV1::Dynamic
        }
        SourceLoopDynamicProducerV1::After { product, binding } => {
            let Some(basis) = product.source.headers.get(binding) else {
                return false;
            };
            basis.binding() == *binding
                && basis.flow.owner() == flow.owner()
                && Arc::ptr_eq(&basis.flow, flow)
                && basis.entry_producer().class()
                    == crate::mir::resolved_value_profile::expression_class::ExpressionValueClassV1::Dynamic
                && flow
                    .loop_join(product.source.source.site(), *binding, Join::LoopAfter)
                    .is_some_and(|(after, _)| {
                        product.source.incoming.as_ref().ok().is_some_and(|rows| {
                            rows.iter().filter(|row| {
                                row.after == after
                                    && row.basis.binding() == *binding
                                    && Rc::ptr_eq(&row.basis, basis)
                            }).count() == 1
                        })
                    })
                && port.covered.contains(product.source.source.site())
        }
    }
}

pub(crate) fn verify_incoming(
    locals: &PrefixLocalFlow<'_>,
    source: &Rc<RetainedCallableLoopSourceV1>,
    headers: &BTreeMap<BindingRefV1, Rc<LoopHeaderValueBasisV1>>,
    ports: &[SourceLoopPortSnapshotV1],
    refused: bool,
    calls: &[super::super::super::LocalCallObservationV1],
) -> Result<Box<[CheckedSourceLoopBindingIncomingV1]>, &'static str> {
    if refused || ports.iter().any(|port| port.unavailable.is_some()) {
        return Err("source Loop has an unavailable path");
    }
    let carriers: BTreeSet<_> = source
        .bindings()
        .bindings()
        .filter_map(|(binding, role)| (role == Role::Carrier).then_some(binding))
        .collect();
    if carriers.is_empty() || headers.keys().copied().collect::<BTreeSet<_>>() != carriers {
        return Err("source Loop carrier census mismatch");
    }
    let flow = locals
        .input
        .function()
        .expression_source()
        .value_flow()
        .ok_or("source Loop value flow missing")?;
    let mut header_edges = vec![Edge::LoopEntry];
    let mut after_edges = vec![Edge::ConditionFalse];
    for port in ports {
        let edge = port_edge(&port.kind);
        let edges = if matches!(edge, Edge::Break(_)) {
            &mut after_edges
        } else {
            &mut header_edges
        };
        if edges.contains(&edge) {
            return Err("duplicate source Loop port");
        }
        edges.push(edge);
    }
    let mut checked = Vec::new();
    for (&binding, basis) in headers {
        if basis.binding != binding
            || !Rc::ptr_eq(&basis.source, source)
            || !Arc::ptr_eq(&basis.flow, flow)
        {
            return Err("source Loop Header basis identity");
        }
        let (header, header_incoming) = flow
            .loop_join(source.site(), binding, Join::LoopHeader)
            .ok_or("source Loop Header missing")?;
        let (after, after_incoming) = flow
            .loop_join(source.site(), binding, Join::LoopAfter)
            .ok_or("source Loop After missing")?;
        if header != basis.header
            || !exact_edges(header_incoming, &header_edges)
            || !exact_edges(after_incoming, &after_edges)
        {
            return Err("source Loop predecessor census mismatch");
        }
        let qualify = |incoming: &[(Edge, SourceValueRefV1)]| -> Result<Box<[_]>, &'static str> {
            incoming
                .iter()
                .map(|(edge, value)| {
                    let (origin, producer) = match edge {
                        Edge::LoopEntry if *value == basis.entry => {
                            (basis.entry_origin, basis.entry_producer.clone())
                        }
                        Edge::ConditionFalse if *value == basis.header => (
                            basis.header,
                            match basis.entry_producer() {
                                SourceLoopValueProducerV1::Scalar(_) => {
                                    CurrentScalarProducerV1::LoopHeader(Rc::clone(basis)).into()
                                }
                                SourceLoopValueProducerV1::Text(_) => {
                                    SourceLoopValueProducerV1::Text(
                                        SourceLoopTextProducerV1::Header(Rc::clone(basis)),
                                    )
                                }
                                SourceLoopValueProducerV1::Dynamic(_) => {
                                    SourceLoopValueProducerV1::Dynamic(
                                        SourceLoopDynamicProducerV1::Header(Rc::clone(basis)),
                                    )
                                }
                            },
                        ),
                        Edge::Backedge | Edge::Break(_) | Edge::Continue(_) => {
                            let port = ports
                                .iter()
                                .find(|port| port_edge(&port.kind) == *edge)
                                .ok_or("source Loop port snapshot missing")?;
                            // A transient path loan shares the inventory; it never
                            // escapes into a snapshot or the immutable result.
                            let mut path = locals.clone();
                            path.locals = port.locals.clone();
                            let available_calls: Vec<_> = calls
                                .iter()
                                .filter(|call| port.calls.contains(call.site()))
                                .cloned()
                                .collect();
                            if basis.entry_producer().scalar().is_some() {
                                if !path.can_rebind_integer_v1(binding, &port.homes) {
                                    return Err("source Loop incoming scalar class");
                                }
                                let (origin, producer) = path
                                    .scalar_producer_for_value_v1(binding, *value, &available_calls)
                                    .ok_or("source Loop incoming original producer missing")?;
                                if !producer_is_covered(&producer, port) {
                                    return Err("source Loop incoming producer outside path");
                                }
                                (origin, producer.into())
                            } else if basis.entry_producer().class()
                                == crate::mir::resolved_value_profile::expression_class::ExpressionValueClassV1::String
                            {
                                let (origin, producer) = path
                                    .text_producer_for_value_v1(binding, *value, flow, &port.homes)
                                    .ok_or("source Loop incoming original producer missing")?;
                                if !text_producer_is_covered(&producer, port, source) {
                                    return Err("source Loop incoming producer outside path");
                                }
                                (origin, SourceLoopValueProducerV1::Text(producer))
                            } else if basis.entry_producer().class()
                                == crate::mir::resolved_value_profile::expression_class::ExpressionValueClassV1::Dynamic
                            {
                                let (origin, producer) = path
                                    .dynamic_producer_for_value_v1(binding, *value, flow)
                                    .ok_or("source Loop incoming original producer missing")?;
                                if !dynamic_producer_is_covered(&producer, port, source, flow) {
                                    return Err("source Loop incoming producer outside path");
                                }
                                (origin, SourceLoopValueProducerV1::Dynamic(producer))
                            } else {
                                return Err("source Loop incoming unsupported producer class");
                            }
                        }
                        _ => return Err("source Loop entry/condition value drift"),
                    };
                    Ok(CheckedSourceLoopIncomingValueV1 {
                        edge: edge.clone(),
                        value: *value,
                        origin,
                        producer,
                    })
                })
                .collect()
        };
        checked.push(CheckedSourceLoopBindingIncomingV1 {
            basis: Rc::clone(basis),
            after,
            header_incoming: qualify(header_incoming)?,
            after_incoming: qualify(after_incoming)?,
        });
    }
    Ok(checked.into_boxed_slice())
}
impl SourceLoopScanSummaryV1 {
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn all_ports_available(&self) -> bool {
        !self.refused && self.ports.iter().all(|port| port.unavailable.is_none())
    }
}

fn crossed_scopes(
    input: ResolvedFunctionLoweringInputV1<'_>,
    source: &RetainedCallableLoopSourceV1,
    start: ScopeId,
) -> Option<Box<[ScopeId]>> {
    let target = input
        .function()
        .loop_region_bundle(source.site())
        .ok()?
        .loop_pair()
        .scope();
    if input.function().scope(target)?.kind() != ScopeKindV1::LoopBody {
        return None;
    }
    let mut current = start;
    let mut crossed = Vec::new();
    let mut seen = BTreeSet::new();
    while seen.insert(current) {
        if current.owner() != input.owner() {
            return None;
        }
        crossed.push(current);
        if current == target {
            return Some(crossed.into_boxed_slice());
        }
        current = input.function().scope(current)?.parent()?;
    }
    None
}

impl PrefixLocalFlow<'_> {
    /// `None` is not a Loop transfer; `Some(false)` is a selected but invalid
    /// target/scope receipt. The scanner must terminate either selected path.
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn observe_source_loop_exit_v1(
        &self,
        site: &SourceStmtSiteV1,
        homes: &[LiveHomeV1],
        covered: &[SourceStmtSiteV1],
        calls: &BTreeSet<OwnedExprSiteV1>,
        unavailable: &Option<super::super::super::HomePrefixUnavailableV1>,
    ) -> Option<bool> {
        let context = self.source_loop.as_ref()?;
        let function = self.input.function();
        let record = function.resolved_exit(&ResolvedExitSiteV1::Statement(site.clone()))?;
        let (target, kind) = match (record.origin(), record.transfer()) {
            (Origin::ExplicitBreak, Transfer::Break { target_loop }) => {
                (target_loop, SourceLoopPortKindV1::Break(site.clone()))
            }
            (Origin::ExplicitContinue, Transfer::Continue { target_loop }) => {
                (target_loop, SourceLoopPortKindV1::Continue(site.clone()))
            }
            (_, Transfer::Return { .. }) => return None,
            _ => return Some(false),
        };
        let Some(bundle) = function.loop_region_bundle(context.source.site()).ok() else {
            return Some(false);
        };
        if target != bundle.loop_pair().region() {
            return Some(false);
        }
        let start = function
            .region(record.source_region())
            .and_then(|region| region.lexical_scope());
        let Some(start) =
            start.filter(|start| function.exact_scope_containing(site.node()) == Some(*start))
        else {
            return Some(false);
        };
        let port = context.ports.borrow().len();
        let available =
            self.record_source_loop_port(kind, start, homes, covered, calls, unavailable);
        if available {
            self.record_source_loop_control_exit_v1(site, *record, port);
        }
        Some(available)
    }

    pub(in crate::mir::resolved_semantics::home_new_prefix) fn observe_source_loop_backedge_v1(
        &self,
        homes: &[LiveHomeV1],
        covered: &[SourceStmtSiteV1],
        calls: &BTreeSet<OwnedExprSiteV1>,
        unavailable: &Option<super::super::super::HomePrefixUnavailableV1>,
    ) -> bool {
        let Some(context) = &self.source_loop else {
            return false;
        };
        let Some(bundle) = self
            .input
            .function()
            .loop_region_bundle(context.source.site())
            .ok()
        else {
            return false;
        };
        self.record_source_loop_port(
            SourceLoopPortKindV1::Backedge,
            bundle.loop_pair().scope(),
            homes,
            covered,
            calls,
            unavailable,
        )
    }

    fn record_source_loop_port(
        &self,
        kind: SourceLoopPortKindV1,
        start: ScopeId,
        homes: &[LiveHomeV1],
        covered: &[SourceStmtSiteV1],
        calls: &BTreeSet<OwnedExprSiteV1>,
        unavailable: &Option<super::super::super::HomePrefixUnavailableV1>,
    ) -> bool {
        let Some(context) = &self.source_loop else {
            return false;
        };
        let Some(crossed) = crossed_scopes(self.input, &context.source, start) else {
            return false;
        };
        let mut seen = BTreeSet::new();
        let mut normal_ends = Vec::new();
        for home in homes.iter().rev() {
            let Some(binding) = self.input.function().binding(home.binding()) else {
                return false;
            };
            if home.owner() != self.input.owner() || !seen.insert(home.binding()) {
                return false;
            }
            if crossed.contains(&binding.owner_scope()) {
                normal_ends.push(home.clone());
            }
        }
        context.ports.borrow_mut().push(SourceLoopPortSnapshotV1 {
            kind,
            locals: self.locals.clone(),
            homes: homes.to_vec().into_boxed_slice(),
            crossed,
            normal_ends: normal_ends.into_boxed_slice(),
            covered: covered.to_vec().into_boxed_slice(),
            calls: calls.clone(),
            unavailable: unavailable.clone(),
        });
        true
    }
}

#[cfg(test)]
#[path = "home_prefix_source_loop_tests.rs"]
pub(crate) mod tests;
