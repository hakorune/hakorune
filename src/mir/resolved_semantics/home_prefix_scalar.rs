//! Current scalar reads join original reaching definitions to the covered Prefix.
use super::*;
use super::super::{CheckedInitializerExpressionFrameV1, LocalCallObservationV1, LocalCallResultClassV1};
use crate::mir::resolved_semantics::source_value_flow::{SourceValueRefV1, SourceValueOriginV1, VerifiedSourceValueFlowV1};
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
    Integer { site: SourceExprSiteV1, binding: BindingRefV1, value: i64 },
    Call(LocalCallObservationV1),
    Initializer(Rc<CheckedInitializerExpressionFrameV1>),
}
/// A source read, not a physical operand or permission inferred from an I64 class.
#[derive(Debug, Clone)]
pub(crate) struct CurrentScalarReadV1 {
    read: SourceExprSiteV1,
    binding: BindingRefV1,
    value: SourceValueRefV1,
    origin: SourceValueRefV1,
    flow: Arc<VerifiedSourceValueFlowV1>,
    producer: CurrentScalarProducerV1,
}
impl PartialEq for CurrentScalarReadV1 {
    fn eq(&self, other: &Self) -> bool {
        self.read == other.read && self.binding == other.binding && self.value == other.value
            && self.origin == other.origin && Arc::ptr_eq(&self.flow, &other.flow)
            && match (&self.producer, &other.producer) {
                (CurrentScalarProducerV1::Initializer(a), CurrentScalarProducerV1::Initializer(b)) => Rc::ptr_eq(a, b),
                (a, b) => a == b,
            }
    }
}
impl Eq for CurrentScalarReadV1 {}
impl CurrentScalarReadV1 {
    pub(crate) fn read(&self) -> &SourceExprSiteV1 { &self.read }
    pub(crate) fn binding(&self) -> BindingRefV1 { self.binding }
    pub(crate) fn kind(&self) -> SourceScalarKind { SourceScalarKind::Integer }
    pub(crate) fn producer(&self) -> &CurrentScalarProducerV1 { &self.producer }
}
impl PrefixLocalFlow<'_> {
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn current_scalar_read_v1(
        &self, site: &SourceExprSiteV1, calls: &[LocalCallObservationV1],
    ) -> Option<CurrentScalarReadV1> {
        let OrdinaryObservation::TrivialLocal(binding, Some(SourceScalarKind::Integer)) = self.observe(site)? else { return None };
        let function = self.input.function();
        let flow = function.expression_source().value_flow()?;
        let value = flow.read(site, binding)?;
        let origin = flow.alias_origin(value, |site| function.variable_ref(site))?;
        let producer = match flow.origin(origin)? {
            SourceValueOriginV1::Entry(root) if self.signed_formals.contains(root) => CurrentScalarProducerV1::SignedEntry(*root),
            SourceValueOriginV1::Expression(original) => {
                let mut definitions = function.expression_source().initializers().filter(|row| row.initializer_site() == Some(original));
                let initializer = definitions.next()?;
                if definitions.next().is_some() || initializer.binding().owner() != binding.owner()
                    || !matches!(initializer.declaration_site(), super::super::SourceBindingSiteV1::Local { .. }) { return None; }
                if let Some(ResolvedLiteralSourceV1::Integer(value)) = function.expression_source().literal(original) {
                    CurrentScalarProducerV1::Integer { site: original.clone(), binding: initializer.binding(), value: *value }
                } else {
                    let plans = self.expression_plans.borrow();
                    let mut frames = plans.iter().filter_map(|plan| plan.initializer_frame()).filter(|frame|
                        frame.is_i64() && frame.binding() == initializer.binding()
                            && frame.source().nodes().first().is_some_and(|node| node.site() == original));
                    if let Some(frame) = frames.next() {
                        if frames.next().is_some() { return None; }
                        CurrentScalarProducerV1::Initializer(Rc::clone(frame))
                    } else {
                        let mut matching = calls.iter().filter(|call| call.owner() == self.input.owner()
                            && call.site().site() == original && call.result() == LocalCallResultClassV1::I64
                            && call.local_binding() == Some((initializer.declaration_site(), initializer.binding())));
                        let call = matching.next()?;
                        if matching.next().is_some() { return None; }
                        CurrentScalarProducerV1::Call(call.clone())
                    }
                }
            }
            _ => return None,
        };
        Some(CurrentScalarReadV1 { read: site.clone(), binding, value, origin, flow: Arc::clone(flow), producer })
    }
}
