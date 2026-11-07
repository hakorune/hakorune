//! Tagged Dynamic read proofs: the original source SUM names the tagged carrier;
//! a proof carries no ownership, concrete payload view, or conversion.
use super::*;

/// One exact lexical read whose original source SUM is the tagged Dynamic
/// carrier. This proof preserves the runtime tag and carries no ownership or
/// concrete payload view.
#[derive(Debug, Clone)]
pub(crate) struct TaggedDynamicReadV1 {
    points: std::rc::Rc<crate::mir::normal_callable_semantic_package::SourceValueClassFactsV1>,
    read: SourceExprSiteV1,
    binding: BindingRefV1,
    value: crate::mir::resolved_semantics::source_value_flow::SourceValueRefV1,
}
impl PartialEq for TaggedDynamicReadV1 {
    fn eq(&self, other: &Self) -> bool {
        std::rc::Rc::ptr_eq(&self.points, &other.points)
            && self.read == other.read
            && self.binding == other.binding
            && self.value == other.value
    }
}
impl Eq for TaggedDynamicReadV1 {}

/// Exact source-flow point classified as tagged Dynamic by the original SUM.
/// It carries no concrete payload tag, conversion, or ownership permission.
#[derive(Debug, Clone)]
pub(crate) struct TaggedDynamicValueV1 {
    points: std::rc::Rc<crate::mir::normal_callable_semantic_package::SourceValueClassFactsV1>,
    flow: std::sync::Arc<crate::mir::resolved_semantics::source_value_flow::VerifiedSourceValueFlowV1>,
    binding: BindingRefV1,
    value: crate::mir::resolved_semantics::source_value_flow::SourceValueRefV1,
}
impl PartialEq for TaggedDynamicValueV1 {
    fn eq(&self, other: &Self) -> bool {
        std::rc::Rc::ptr_eq(&self.points, &other.points)
            && std::sync::Arc::ptr_eq(&self.flow, &other.flow)
            && self.binding == other.binding
            && self.value == other.value
    }
}
impl Eq for TaggedDynamicValueV1 {}
impl TaggedDynamicValueV1 {
    pub(crate) fn binding(&self) -> BindingRefV1 { self.binding }
    pub(crate) fn value(&self) -> crate::mir::resolved_semantics::source_value_flow::SourceValueRefV1 {
        self.value
    }
    pub(crate) fn expression_site_v1(&self) -> Option<&SourceExprSiteV1> {
        match self.flow.origin(self.value)? {
            crate::mir::resolved_semantics::source_value_flow::SourceValueOriginV1::Expression(site) => Some(site),
            _ => None,
        }
    }
    pub(crate) fn proves_source_value_v1(&self, binding: BindingRefV1,
        value: crate::mir::resolved_semantics::source_value_flow::SourceValueRefV1,
    ) -> bool {
        self.binding == binding && self.value == value && self.points.proves_value(&self.flow, value)
    }
    pub(crate) fn proves(
        &self,
        flow: &std::sync::Arc<crate::mir::resolved_semantics::source_value_flow::VerifiedSourceValueFlowV1>,
        binding: BindingRefV1,
        value: crate::mir::resolved_semantics::source_value_flow::SourceValueRefV1,
    ) -> bool {
        self.binding == binding
            && self.value == value
            && std::sync::Arc::ptr_eq(&self.flow, flow)
            && self.points.proves_value(flow, value)
    }
}
impl TaggedDynamicReadV1 {
    pub(crate) fn binding(&self) -> BindingRefV1 {
        self.binding
    }
}

impl<'source> PrefixLocalFlow<'source> {
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn tagged_dynamic_read_v1(
        &self,
        site: &SourceExprSiteV1,
    ) -> Option<TaggedDynamicReadV1> {
        let ResolvedLexicalRefV1::Local(binding) = self.input.function().variable_ref(site)? else {
            return None;
        };
        if !matches!(self.locals.get(&binding), Some(StoredLocal::TaggedDynamic)) {
            return None;
        }
        let flow = self.input.function().expression_source().value_flow()?;
        let points = self.source_value_class_facts.as_ref()?;
        let value = points.proves_read(flow, site, binding)?;
        Some(TaggedDynamicReadV1 {
            points: std::rc::Rc::clone(points),
            read: site.clone(),
            binding,
            value,
        })
    }

    pub(in crate::mir::resolved_semantics::home_new_prefix) fn tagged_dynamic_value_for_value_v1(
        &self,
        binding: BindingRefV1,
        value: crate::mir::resolved_semantics::source_value_flow::SourceValueRefV1,
        flow: &std::sync::Arc<crate::mir::resolved_semantics::source_value_flow::VerifiedSourceValueFlowV1>,
    ) -> Option<TaggedDynamicValueV1> {
        if binding.owner() != self.input.owner()
            || flow.owner() != binding.owner()
            || !matches!(self.locals.get(&binding), Some(StoredLocal::TaggedDynamic))
        {
            return None;
        }
        let points = self.source_value_class_facts.as_ref()?;
        points.proves_value(flow, value).then(|| TaggedDynamicValueV1 {
            points: std::rc::Rc::clone(points),
            flow: std::sync::Arc::clone(flow),
            binding,
            value,
        })
    }
}
