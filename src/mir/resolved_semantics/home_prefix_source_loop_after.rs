//! Atomic source Loop After publication and its retained product lookup.
use super::*;
use crate::mir::resolved_semantics::home_new_prefix::{CheckedExpressionEvaluationV1, CheckedExpressionReferenceV1};
use crate::mir::resolved_semantics::SourcePathSegmentV1;

impl PrefixLocalFlow<'_> {
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn apply_source_loop_after_v2(
        &mut self,
        product: std::rc::Rc<crate::mir::loop_recipe_contract::VerifiedSourceLoopRecipeProductV2>,
        homes: &[LiveHomeV1],
    ) -> bool {
        if product.source.source.owner() != self.input.owner()
            || product.source.condition.prior_homes() != homes
        {
            return false;
        }
        let flow = match self.input.function().expression_source().value_flow() {
            Some(flow) => flow,
            None => return false,
        };
        #[derive(Clone, Copy)]
        enum AfterClass { Scalar, Text, Dynamic }
        let updated: Vec<_> = product.source.headers.iter().map(|(binding, basis)| {
            if basis.entry_producer().scalar().is_some() {
                self.can_rebind_integer_v1(*binding, homes).then_some((*binding, AfterClass::Scalar))
            } else if basis.entry_producer().class()
                == crate::mir::resolved_value_profile::expression_class::ExpressionValueClassV1::String {
                self.text_producer_for_value_v1(*binding, basis.entry(), flow, homes)
                    .filter(|(origin, producer)| *origin == basis.entry_origin()
                        && matches!(basis.entry_producer(), SourceLoopValueProducerV1::Text(expected) if expected == producer))
                    .map(|_| (*binding, AfterClass::Text))
            } else if basis.entry_producer().class()
                == crate::mir::resolved_value_profile::expression_class::ExpressionValueClassV1::Dynamic {
                let no_home = !homes.iter().any(|home| home.binding() == *binding);
                let Some((after, _)) = flow.loop_join(product.source.source.site(), *binding, Join::LoopAfter) else {
                    return None;
                };
                let incoming = product.source.incoming.as_ref().ok()?;
                let mut rows = incoming.iter().filter(|row| row.after == after
                    && row.basis.binding() == *binding && Rc::ptr_eq(&row.basis, basis));
                (no_home && rows.next().is_some() && rows.next().is_none())
                    .then_some((*binding, AfterClass::Dynamic))
            } else {
                None
            }
        }).collect::<Option<Vec<_>>>().unwrap_or_default();
        if updated.len() != product.source.headers.len() || updated.is_empty() {
            return false;
        }
        let mut plans = self.expression_plans.borrow_mut();
        let indices: Vec<_> = plans
            .iter()
            .enumerate()
            .filter_map(|(index, plan)| {
                std::rc::Rc::ptr_eq(plan.source(), product.source.condition.source())
                    .then_some(index)
            })
            .collect();
        if indices.len() != 1 {
            return false;
        }
        if !plans[indices[0]].attach_loop_recipe_v2(Rc::clone(&product)) {
            return false;
        }
        drop(plans);
        // All checks precede publication. Text keeps the exact product as its
        // conditional Static/Owned source; it does not mint a Home here.
        for (binding, class) in updated {
            match class {
                AfterClass::Text => self.store(
                    binding,
                    StoredLocal::LoopText { product: Rc::clone(&product) },
                ),
                AfterClass::Dynamic => self.store(binding, StoredLocal::TaggedDynamic),
                AfterClass::Scalar => self.store(
                    binding,
                    StoredLocal::Trivial(Some(SourceScalarKind::Integer)),
                ),
            }
        }
        true
    }
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn source_loop_after_for_value_v2(
        &self,
        flow: &Arc<VerifiedSourceValueFlowV1>,
        value: SourceValueRefV1,
    ) -> Option<CurrentScalarProducerV1> {
        let plans = self.expression_plans.borrow();
        let mut matching = plans
            .iter()
            .filter_map(|plan| plan.loop_recipe_v2())
            .flat_map(|product| {
                product
                    .source
                    .incoming
                    .as_ref()
                    .ok()
                    .into_iter()
                    .flat_map(move |rows| rows.iter())
                    .filter(move |row| {
                        row.after == value
                            && row.basis.entry_producer().scalar().is_some()
                            && Arc::ptr_eq(&row.basis.flow, flow)
                    })
                    .map(move |row| (product, row.basis.binding()))
            });
        let (product, binding) = matching.next()?;
        if matching.next().is_some() {
            return None;
        }
        Some(CurrentScalarProducerV1::LoopAfter {
            product: Rc::clone(product),
            binding,
        })
    }
}

impl CheckedExpressionEvaluationV1 {
    /// Lend one exact After obligation read by this Return. Repeated reads of
    /// the same source generation share it; distinct obligations need the
    /// source exit plan's acquisition order, never reference-map key order.
    pub(crate) fn return_text_after_read_v2(&self) -> Result<Option<(usize, &LoopTextReadV1)>, &'static str> {
        use crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1;
        use crate::mir::resolved_value_profile::expression_root::StatementExpressionRoleV1;
        let mut selected: Option<(usize, &LoopTextReadV1)> = None;
        for (&node, reference) in self.references() {
            let CheckedExpressionReferenceV1::LoopText(read) = reference else { continue; };
            let SourceLoopTextProducerV1::After { product, binding } = read.producer() else { continue; };
            if !read.matches_source_node(self.source(), node) {
                return Err("source-loop/text-point-binding");
            }
            if let Some((_, previous)) = selected {
                if !matches!(previous.producer(), SourceLoopTextProducerV1::After { product: prior, binding: old }
                    if Rc::ptr_eq(prior, product) && old == binding) {
                    return Err("source-loop/text-point-binding");
                }
            } else { selected = Some((node, read)); }
        }
        if selected.is_none() { return Ok(None); }
        if !matches!(self.source().root(), ExpressionPlanRootV1::Statement {
            role: StatementExpressionRoleV1::ReturnValue, .. }) {
            return Err("expression/residence-original-pair");
        }
        // This profile crosses only ordinary lexical/control bodies. Try and
        // cleanup ancestry require a later source cleanup continuation: it may
        // still read the local after the Return expression has been evaluated.
        if self.source().root().statement().node().segments().is_empty()
            || !self.source().root().statement().node().segments().iter().all(|segment|
                matches!(segment, SourcePathSegmentV1::Body(_) | SourcePathSegmentV1::ScopeBody(_)
                    | SourcePathSegmentV1::IfThen(_) | SourcePathSegmentV1::IfElse(_)
                    | SourcePathSegmentV1::LoopBody(_) | SourcePathSegmentV1::FunctionBody
                    | SourcePathSegmentV1::ScopeBodyRoot | SourcePathSegmentV1::IfThenBody
                    | SourcePathSegmentV1::IfElseBody | SourcePathSegmentV1::LoopBodyRoot)) {
            return Err("expression/residence-original-pair");
        }
        Ok(selected)
    }
}
