//! Current scalar reads join original reaching definitions to the covered Prefix.
use super::super::{
    CheckedBindingRebindFrameV1, CheckedInitializerExpressionFrameV1, LocalCallObservationV1,
    LocalCallResultClassV1,
};
use super::*;
use crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1;
use crate::mir::resolved_semantics::source_value_flow::{
    SourceValueEdgeV1, SourceValueJoinKindV1, SourceValueOriginV1, SourceValueRefV1,
    VerifiedSourceValueFlowV1,
};
use std::{rc::Rc, sync::Arc};

/// Source scalar class retained from an exact literal or declaration contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SourceScalarKind {
    Integer,
    Bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum CurrentScalarProducerV1 {
    SignedEntry(BindingRefV1),
    Integer {
        site: SourceExprSiteV1,
        binding: BindingRefV1,
        value: i64,
    },
    Call(LocalCallObservationV1),
    FieldRead(Rc<super::super::field_read::ScalarFieldReadSourceV1>),
    Initializer(Rc<CheckedInitializerExpressionFrameV1>),
    BindingRebind(Rc<CheckedBindingRebindFrameV1>),
    BranchNormal(Rc<BranchNormalScalarSourceV1>),
    LoopHeader(Rc<super::source_loop::LoopHeaderValueBasisV1>),
    LoopAfter {
        product: Rc<crate::mir::loop_recipe_contract::VerifiedSourceLoopRecipeProductV2>,
        binding: BindingRefV1,
    },
}
/// Complete original BranchNormal inputs, qualified by the existing scalar
/// producer walk. Identity is the original immutable flow coordinate, not a
/// newly allocated wrapper; separate reads may borrow this same source value.
#[derive(Debug)]
pub(crate) struct BranchNormalScalarSourceV1 {
    flow: Arc<VerifiedSourceValueFlowV1>,
    value: SourceValueRefV1,
    binding: BindingRefV1,
    site: SourceStmtSiteV1,
    incoming: Box<[(SourceValueEdgeV1, SourceValueRefV1, CurrentScalarProducerV1)]>,
}
impl PartialEq for BranchNormalScalarSourceV1 {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.flow, &other.flow)
            && self.value == other.value
            && self.binding == other.binding
            && self.site == other.site
    }
}
impl Eq for BranchNormalScalarSourceV1 {}
impl BranchNormalScalarSourceV1 {
    pub(crate) fn binding(&self) -> BindingRefV1 {
        self.binding
    }
    pub(crate) fn site(&self) -> &SourceStmtSiteV1 {
        &self.site
    }
    pub(crate) fn incoming(
        &self,
    ) -> &[(SourceValueEdgeV1, SourceValueRefV1, CurrentScalarProducerV1)] {
        &self.incoming
    }
    pub(crate) fn corroborates_original(&self) -> bool {
        self.flow.is_complete()
            && self.flow.owner() == self.binding.owner()
            && matches!(self.flow.origin(self.value), Some(SourceValueOriginV1::Join {
                binding, site, kind: SourceValueJoinKindV1::BranchNormal, incoming
            }) if *binding == self.binding && site.node() == self.site.node()
                && incoming.len() == 2 && self.incoming.len() == 2
                && incoming.iter().zip(self.incoming.iter()).all(|((edge, value), (observed, original, _))|
                    edge == observed && value == original)
                && matches!((&incoming[0].0, &incoming[1].0),
                    (SourceValueEdgeV1::ThenNormal, SourceValueEdgeV1::ElseNormal)))
    }
}

/// A source read, not a physical operand or permission inferred from an I64 class.
#[derive(Debug, Clone)]
pub(crate) struct CurrentScalarReadV1 {
    read: SourceExprSiteV1,
    binding: BindingRefV1,
    kind: SourceScalarKind,
    value: SourceValueRefV1,
    origin: SourceValueRefV1,
    flow: Arc<VerifiedSourceValueFlowV1>,
    producer: CurrentScalarProducerV1,
}
impl PartialEq for CurrentScalarReadV1 {
    fn eq(&self, other: &Self) -> bool {
        self.read == other.read
            && self.binding == other.binding
            && self.value == other.value
            && self.origin == other.origin
            && Arc::ptr_eq(&self.flow, &other.flow)
            && match (&self.producer, &other.producer) {
                (
                    CurrentScalarProducerV1::Initializer(a),
                    CurrentScalarProducerV1::Initializer(b),
                ) => Rc::ptr_eq(a, b),
                (
                    CurrentScalarProducerV1::BindingRebind(a),
                    CurrentScalarProducerV1::BindingRebind(b),
                ) => Rc::ptr_eq(a, b),
                (
                    CurrentScalarProducerV1::FieldRead(a),
                    CurrentScalarProducerV1::FieldRead(b),
                ) => Rc::ptr_eq(a, b),
                (a, b) => a == b,
            }
    }
}
impl Eq for CurrentScalarReadV1 {}
impl CurrentScalarReadV1 {
    /// Borrow the SAME original point for the independent source SUM solver.
    pub(crate) fn point_for_original_flow_v1(
        &self,
        flow: &Arc<VerifiedSourceValueFlowV1>,
    ) -> Option<SourceValueRefV1> {
        (Arc::ptr_eq(flow, &self.flow)
            && flow.is_complete()
            && flow.owner() == self.binding.owner()
            && flow.read(&self.read, self.binding) == Some(self.value))
        .then_some(self.value)
    }
    pub(crate) fn read(&self) -> &SourceExprSiteV1 {
        &self.read
    }
    pub(crate) fn binding(&self) -> BindingRefV1 {
        self.binding
    }
    pub(crate) fn kind(&self) -> SourceScalarKind {
        self.kind
    }
    pub(crate) fn producer(&self) -> &CurrentScalarProducerV1 {
        &self.producer
    }
}
impl PrefixLocalFlow<'_> {
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn can_rebind_integer_v1(
        &self,
        binding: BindingRefV1,
        homes: &[LiveHomeV1],
    ) -> bool {
        binding.owner() == self.input.owner()
            && !homes.iter().any(|home| home.binding() == binding)
            && matches!(
                self.locals.get(&binding),
                Some(StoredLocal::Trivial(Some(SourceScalarKind::Integer)))
            )
    }
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn current_scalar_read_v1(
        &self,
        site: &SourceExprSiteV1,
        calls: &[LocalCallObservationV1],
    ) -> Option<CurrentScalarReadV1> {
        let observed = self.observe(site);
        let OrdinaryObservation::TrivialLocal(binding, Some(kind)) = observed? else {
            return None;
        };
        let function = self.input.function();
        let Some(flow) = function.expression_source().value_flow() else {
            if crate::config::env::builder_debug_enabled() { eprintln!("[scalar-read] {site:?} no value flow"); }
            return None;
        };
        let Some(value) = flow.read(site, binding) else {
            if crate::config::env::builder_debug_enabled() { eprintln!("[scalar-read] {site:?} binding={binding:?} kind={kind:?} has no flow read"); }
            return None;
        };
        let Some((origin, producer)) = self.scalar_producer_for_value_v1(binding, value, calls) else {
            if crate::config::env::builder_debug_enabled() {
                eprintln!("[scalar-read] {site:?} binding={binding:?} kind={kind:?} has no current producer");
            }
            return None;
        };
        Some(CurrentScalarReadV1 {
            read: site.clone(),
            binding,
            kind,
            value,
            origin,
            flow: Arc::clone(flow),
            producer,
        })
    }

    /// Resolve a proved current scalar's exact original value without inventing
    /// a read site. The Loop Header entry uses this same producer owner.
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn scalar_producer_for_value_v1(
        &self,
        binding: BindingRefV1,
        value: SourceValueRefV1,
        calls: &[LocalCallObservationV1],
    ) -> Option<(SourceValueRefV1, CurrentScalarProducerV1)> {
        self.scalar_producer_with_path_v1(binding, value, calls, &BTreeSet::new())
    }
    fn scalar_producer_with_path_v1(
        &self,
        binding: BindingRefV1,
        value: SourceValueRefV1,
        calls: &[LocalCallObservationV1],
        path: &BTreeSet<SourceValueRefV1>,
    ) -> Option<(SourceValueRefV1, CurrentScalarProducerV1)> {
        let mut path = path.clone();
        if !path.insert(value) {
            return None;
        }
        let Some(StoredLocal::Trivial(Some(kind))) = self.locals.get(&binding) else {
            return None;
        };
        let kind = *kind;
        if binding.owner() != self.input.owner() {
            return None;
        }
        // Only Integer reads have literal/field/call/rebind producers; an
        // exact Bool read is produced by its Local initializer frame alone.
        let frame_class = |frame: &crate::mir::resolved_semantics::home_new_prefix::CheckedInitializerExpressionFrameV1| match kind {
            SourceScalarKind::Integer => frame.is_i64(),
            SourceScalarKind::Bool => frame.is_bool(),
        };
        let function = self.input.function();
        let flow = function.expression_source().value_flow()?;
        // A completed source rebind is its own producer, including `x = x`.
        // Preserve it before following aliases; it is never an initializer.
        let rebind = |original: &SourceExprSiteV1| {
            let plans = self.expression_plans.borrow();
            let mut frames = plans
                .iter()
                .filter_map(|plan| plan.rebind_frame())
                .filter(|frame| {
                    frame.target().owner() == binding.owner()
                        && frame
                            .source()
                            .nodes()
                            .first()
                            .is_some_and(|node| node.site() == original)
                });
            let frame = frames.next()?;
            if frames.next().is_some() {
                return None;
            }
            Some(CurrentScalarProducerV1::BindingRebind(Rc::clone(frame)))
        };
        // A signed root-reference initializer records the original use/Copy,
        // but forwards its producer rather than creating another value root.
        let origin = flow.alias_terminal_with_expression_boundary(
            value,
            |site| function.variable_ref(site),
            |site| {
                rebind(site).is_some()
                    || self
                        .expression_plans
                        .borrow()
                        .iter()
                        .filter_map(|plan| plan.initializer_frame())
                        .any(|frame| {
                            frame_class(frame)
                                && frame.signed_root_sum().is_none()
                                && frame
                                    .source()
                                    .nodes()
                                    .first()
                                    .is_some_and(|node| node.site() == site)
                        })
            },
        )?;
        let producer = if let SourceValueOriginV1::Expression(site) = flow.origin(origin)? {
            rebind(site)
        } else {
            None
        };
        let producer = if let Some(rebind) = producer {
            rebind
        } else {
            match flow.origin(origin)? {
                SourceValueOriginV1::Join {
                    binding: original,
                    site,
                    kind: SourceValueJoinKindV1::BranchNormal,
                    incoming,
                } => {
                    if incoming.len() != 2
                        || !matches!(
                            (&incoming[0].0, &incoming[1].0),
                            (SourceValueEdgeV1::ThenNormal, SourceValueEdgeV1::ElseNormal)
                        )
                    {
                        return None;
                    }
                    let inputs = incoming
                        .iter()
                        .map(|(edge, value)| {
                            let (_, producer) =
                                self.scalar_producer_with_path_v1(*original, *value, calls, &path)?;
                            Some((edge.clone(), *value, producer))
                        })
                        .collect::<Option<Vec<_>>>()?;
                    let basis = Rc::new(BranchNormalScalarSourceV1 {
                        flow: Arc::clone(flow),
                        value: origin,
                        binding: *original,
                        site: SourceStmtSiteV1::from_node(site.node().clone()),
                        incoming: inputs.into_boxed_slice(),
                    });
                    if !basis.corroborates_original() {
                        return None;
                    }
                    CurrentScalarProducerV1::BranchNormal(basis)
                }
                SourceValueOriginV1::Join { .. } => match self
                    .source_loop
                    .as_ref()
                    .and_then(|active| active.header_for_value(flow, origin))
                {
                    Some(basis) if basis.entry_producer().scalar().is_some() => {
                        CurrentScalarProducerV1::LoopHeader(Rc::clone(basis))
                    }
                    Some(_) => return None,
                    None => self.source_loop_after_for_value_v2(flow, origin)?,
                },
                SourceValueOriginV1::Entry(root) if self.signed_formals.contains(root) => {
                    CurrentScalarProducerV1::SignedEntry(*root)
                }
                SourceValueOriginV1::Expression(original) => {
                    let mut definitions = function
                        .expression_source()
                        .initializers()
                        .filter(|row| row.initializer_site() == Some(original));
                    let initializer = definitions.next()?;
                    if definitions.next().is_some()
                        || initializer.binding().owner() != binding.owner()
                        || !matches!(
                            initializer.declaration_site(),
                            super::super::SourceBindingSiteV1::Local { .. }
                        )
                    {
                        return None;
                    }
                    if let Some(source) = self
                        .scalar_field_read_source_v1(initializer.binding(), original)
                    {
                        CurrentScalarProducerV1::FieldRead(source)
                    } else if let Some(ResolvedLiteralSourceV1::Integer(value)) =
                        function.expression_source().literal(original)
                    {
                        CurrentScalarProducerV1::Integer {
                            site: original.clone(),
                            binding: initializer.binding(),
                            value: *value,
                        }
                    } else {
                        let plans = self.expression_plans.borrow();
                        let mut frames = plans
                            .iter()
                            .filter_map(|plan| plan.initializer_frame())
                            .filter(|frame| {
                                frame_class(frame)
                                    && frame.binding() == initializer.binding()
                                    && frame
                                        .source()
                                        .nodes()
                                        .first()
                                        .is_some_and(|node| node.site() == original)
                            });
                        if let Some(frame) = frames.next() {
                            if frames.next().is_some() {
                                return None;
                            }
                            CurrentScalarProducerV1::Initializer(Rc::clone(frame))
                        } else {
                            let mut matching = calls.iter().filter(|call| {
                                call.owner() == self.input.owner()
                                    && call.site().site() == original
                                    && call.result() == LocalCallResultClassV1::I64
                                    && call.local_binding()
                                        == Some((
                                            initializer.declaration_site(),
                                            initializer.binding(),
                                        ))
                            });
                            let call = matching.next()?;
                            if matching.next().is_some() {
                                return None;
                            }
                            CurrentScalarProducerV1::Call(call.clone())
                        }
                    }
                }
                _ => return None,
            }
        };
        Some((origin, producer))
    }
}
