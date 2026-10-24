//! Source-backed Dynamic Loop entries, reads and reaching definitions.
use super::*;
use crate::mir::resolved_semantics::home_new_prefix::{CheckedInitializerExpressionFrameV1, CheckedBindingRebindFrameV1};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum SourceLoopDynamicProducerV1 {
    Source(TaggedDynamicValueV1),
    Initializer(Rc<CheckedInitializerExpressionFrameV1>),
    BindingRebind(Rc<CheckedBindingRebindFrameV1>),
    Header(Rc<LoopHeaderValueBasisV1>),
    After {
        product: Rc<crate::mir::loop_recipe_contract::VerifiedSourceLoopRecipeProductV2>,
        binding: BindingRefV1,
    },
}
impl SourceLoopDynamicProducerV1 {
    /// Original invocation Home supporting an opaque carrier read. Tagged SUM
    /// values retain their separate representation and grant no such view.
    pub(crate) fn invocation_home_v1(&self) -> Option<BindingRefV1> {
        match self {
            Self::Initializer(frame) => frame.dynamic_home_binding_v1(),
            Self::Header(basis) => match basis.entry_producer() {
                SourceLoopValueProducerV1::Dynamic(source) => source.invocation_home_v1(),
                _ => None,
            },
            Self::After { product, binding } => match product.source.headers.get(binding)?.entry_producer() {
                SourceLoopValueProducerV1::Dynamic(source) => source.invocation_home_v1(),
                _ => None,
            },
            Self::Source(_) | Self::BindingRebind(_) => None,
        }
    }
    pub(crate) fn matches_value_v1(
        &self, flow: &Arc<VerifiedSourceValueFlowV1>,
        binding: BindingRefV1, value: SourceValueRefV1,
    ) -> bool {
        use crate::mir::resolved_value_profile::expression_class::ExpressionValueClassV1 as Class;
        let expression = |source: &Rc<crate::mir::resolved_value_profile::expression_plan::ExpressionSourcePlanV1>| {
            source.owner() == flow.owner() && source.owner() == binding.owner()
                && source.nodes().first().is_some_and(|root|
                    matches!(flow.origin(value), Some(SourceValueOriginV1::Expression(site)) if site == root.site()))
        };
        match self {
            Self::Source(point) => point.proves(flow, binding, value),
            Self::Initializer(frame) => frame.binding() == binding
                && frame.is_dynamic_value() && expression(frame.source()),
            Self::BindingRebind(frame) => frame.target() == binding
                && frame.is_dynamic_loop_carrier_v1() && expression(frame.source()),
            Self::Header(basis) => basis.binding() == binding && Arc::ptr_eq(&basis.flow, flow)
                && basis.header == value && basis.entry_producer().class() == Class::Dynamic,
            Self::After { product, binding: original } => {
                let Some(basis) = product.source.headers.get(&binding) else { return false; };
                *original == binding && product.source.source.owner() == flow.owner()
                    && Arc::ptr_eq(&basis.flow, flow) && basis.entry_producer().class() == Class::Dynamic
                    && flow.loop_join(product.source.source.site(), binding, Join::LoopAfter)
                        .is_some_and(|(after, _)| after == value)
                    && product.source.incoming.as_ref().is_ok_and(|incoming| incoming.iter().filter(|row|
                        row.after == value && row.basis.binding() == binding
                            && Rc::ptr_eq(&row.basis, basis)).count() == 1)
            }
        }
    }
}

/// Exact source read and its reaching producer. Dynamic payload classification
/// comes from that producer; it never comes from a physical MIR type or tag.
#[derive(Debug, Clone)]
pub(crate) struct LoopDynamicReadV1 {
    flow: Arc<VerifiedSourceValueFlowV1>,
    read: SourceExprSiteV1,
    binding: BindingRefV1,
    value: SourceValueRefV1,
    producer: SourceLoopDynamicProducerV1,
}
impl PartialEq for LoopDynamicReadV1 {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.flow, &other.flow) && self.read == other.read
            && self.binding == other.binding && self.value == other.value && self.producer == other.producer
    }
}
impl Eq for LoopDynamicReadV1 {}
impl LoopDynamicReadV1 {
    pub(crate) fn binding(&self) -> BindingRefV1 { self.binding }
    pub(crate) fn read(&self) -> &SourceExprSiteV1 { &self.read }
    pub(crate) fn value(&self) -> SourceValueRefV1 { self.value }
    pub(crate) fn producer(&self) -> &SourceLoopDynamicProducerV1 { &self.producer }
    pub(crate) fn matches_source_node(
        &self, source: &crate::mir::resolved_value_profile::expression_plan::ExpressionSourcePlanV1,
        node: usize,
    ) -> bool {
        source.owner() == self.flow.owner() && self.binding.owner() == source.owner()
            && source.nodes().get(node).is_some_and(|original| original.site() == &self.read
                && matches!(original.source(), crate::mir::resolved_value_profile::expression_plan::ExpressionNodeSourceV1::Reference(
                    ResolvedLexicalRefV1::Local(binding)) if *binding == self.binding))
            && self.flow.read(&self.read, self.binding) == Some(self.value)
            && self.producer.matches_value_v1(&self.flow, self.binding, self.value)
    }
}
impl PrefixLocalFlow<'_> {
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn loop_header_dynamic_reference_v1(
        &self, site: &SourceExprSiteV1,
    ) -> Option<LoopDynamicReadV1> {
        let ResolvedLexicalRefV1::Local(binding) = self.input.function().variable_ref(site)? else { return None; };
        let flow = self.input.function().expression_source().value_flow()?;
        let value = flow.read(site, binding)?;
        let active = self.source_loop.as_ref()?;
        let prefix = active.source.site().node().segments();
        let segments = site.node().segments();
        if !segments.starts_with(prefix) || !matches!(segments.get(prefix.len()),
            Some(crate::mir::resolved_semantics::SourcePathSegmentV1::LoopCondition
                | crate::mir::resolved_semantics::SourcePathSegmentV1::LoopBody(_))) { return None; }
        let basis = active.header_for_value(flow, value)?;
        let producer = SourceLoopDynamicProducerV1::Header(Rc::clone(basis));
        producer.matches_value_v1(flow, binding, value).then(|| LoopDynamicReadV1 {
            flow: Arc::clone(flow), read: site.clone(), binding, value, producer,
        })
    }

    pub(in crate::mir::resolved_semantics::home_new_prefix) fn loop_dynamic_reference_v1(
        &self, site: &SourceExprSiteV1,
    ) -> Option<LoopDynamicReadV1> {
        if let Some(read) = self.loop_header_dynamic_reference_v1(site) { return Some(read); }
        let ResolvedLexicalRefV1::Local(binding) = self.input.function().variable_ref(site)? else { return None; };
        let flow = self.input.function().expression_source().value_flow()?;
        let value = flow.read(site, binding)?;
        let (_, producer) = self.dynamic_producer_for_value_v1(binding, value, flow)?;
        if !matches!(producer, SourceLoopDynamicProducerV1::Header(_) | SourceLoopDynamicProducerV1::After { .. }) {
            return None;
        }
        producer.matches_value_v1(flow, binding, value).then(|| LoopDynamicReadV1 {
            flow: Arc::clone(flow), read: site.clone(), binding, value, producer,
        })
    }

    /// Preserve each original producer independently from its ownership state.
    /// An initializer alias lends the original Home; a rebind may acquire a
    /// fresh carrier. The Loop lifecycle owner closes that transition.
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn dynamic_producer_for_value_v1(
        &self, binding: BindingRefV1, value: SourceValueRefV1,
        flow: &Arc<VerifiedSourceValueFlowV1>,
    ) -> Option<(SourceValueRefV1, SourceLoopDynamicProducerV1)> {
        if binding.owner() != self.input.owner() || flow.owner() != binding.owner() { return None; }
        if let Some(basis) = self.source_loop.as_ref().and_then(|active| active.header_for_value(flow, value)) {
            let producer = SourceLoopDynamicProducerV1::Header(Rc::clone(basis));
            if producer.matches_value_v1(flow, binding, value) { return Some((value, producer)); }
        }
        let plans = self.expression_plans.borrow();
        let mut candidates = plans.iter().filter_map(|plan| {
            if let Some(product) = plan.loop_recipe_v2() {
                let producer = SourceLoopDynamicProducerV1::After { product: Rc::clone(product), binding };
                if producer.matches_value_v1(flow, binding, value) { return Some(producer); }
            }
            let frame = plan.rebind_frame()?;
            let producer = SourceLoopDynamicProducerV1::BindingRebind(Rc::clone(frame));
            producer.matches_value_v1(flow, binding, value).then_some(producer)
        });
        if let Some(producer) = candidates.next() {
            return candidates.next().is_none().then_some((value, producer));
        }
        drop(candidates);
        drop(plans);
        if let Some(StoredLocal::DynamicInvocationResult { frame }) = self.locals.get(&binding) {
            let owner = frame.dynamic_home_binding_v1()?;
            if !matches!(self.locals.get(&owner), Some(StoredLocal::DynamicInvocationResult { frame })
                if frame.binding() == owner && frame.is_dynamic_invocation_result()) { return None; }
            let producer = SourceLoopDynamicProducerV1::Initializer(Rc::clone(frame));
            return producer.matches_value_v1(flow, binding, value).then_some((value, producer));
        }
        let source = self.tagged_dynamic_value_for_value_v1(binding, value, flow)?;
        Some((value, SourceLoopDynamicProducerV1::Source(source)))
    }
}
