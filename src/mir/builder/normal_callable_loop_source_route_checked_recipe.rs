//! Checked Recipe coverage borrows source Calls and their existing publication relations.
use super::*;

impl CallableLoopSourceCallCoverageV1 {
    pub(in crate::mir::builder) fn validate_checked_recipe_v2(
        product: &crate::mir::loop_recipe_contract::VerifiedSourceLoopRecipeProductV2,
        owner: FunctionOwnerIdV1, parent: &crate::mir::resolved_semantics::SourceNodeSiteV1,
        items: &[CallableLoopSourceItemBindingV1],
    ) -> Result<(), CallableLoopSourceRouteRejectV1> {
        use crate::mir::resolved_value_profile::expression_plan::ExpressionNodeSourceV1;
        if product.source.source.owner() != owner {
            return Err(CallableLoopSourceRouteRejectV1::SourceCoverageForeign);
        }
        if product.source.source.site().node() != parent {
            return Err(CallableLoopSourceRouteRejectV1::SourceCoverageSiteMismatch);
        }
        let mut expected = std::collections::BTreeSet::new();
        let mut frame_calls = std::collections::BTreeSet::new();
        for node in product.schedule().frames().iter().flat_map(|frame| frame.source().nodes()) {
            if matches!(node.source(), ExpressionNodeSourceV1::MethodCall) {
                if !frame_calls.insert(node.site()) {
                    return Err(CallableLoopSourceRouteRejectV1::SourceItemDuplicate { call_site: node.site().clone() });
                }
                expected.insert(node.site());
            }
        }
        // Argument frames are checked subviews of the SAME scheduled Call.
        // Nested source calls therefore overlap their enclosing expression
        // frame by site; union those views while retaining duplicate checks
        // for the final resolver-owned item inventory below.
        for frame in product.scheduled_call_arguments_v2() {
            for node in frame.source().nodes() {
                if matches!(node.source(), ExpressionNodeSourceV1::MethodCall) {
                    expected.insert(node.site());
                }
            }
        }
        let mut scheduled_calls = std::collections::BTreeSet::new();
        for step in product.schedule().calls_v2() {
            if let crate::mir::loop_recipe_contract::SourceLoopScheduledStepV2::Call { observation, .. } = step {
                if !scheduled_calls.insert(observation.site().site()) {
                    return Err(CallableLoopSourceRouteRejectV1::SourceItemDuplicate { call_site: observation.site().site().clone() });
                }
                expected.insert(observation.site().site());
            }
        }
        for observation in product.schedule().blocks_array_appends_v2() {
            let crate::mir::resolved_semantics::home_new_prefix::NamedArrayHomeObservationV1::Append {
                source, call, candidate, ..
            } = observation else { return Err(CallableLoopSourceRouteRejectV1::SourceItemForeign); };
            if !expected.insert(call.site()) {
                return Err(CallableLoopSourceRouteRejectV1::SourceItemDuplicate { call_site: call.site().clone() });
            }
            match source.append_candidate_is_null_v1(call) {
                Some(true) => {},
                Some(false) if expected.insert(candidate.site()) => {},
                _ => return Err(CallableLoopSourceRouteRejectV1::SourceItemForeign),
            }
        }
        let mut observed = std::collections::BTreeSet::new();
        for item in items {
            if !observed.insert(item.call_site()) {
                return Err(CallableLoopSourceRouteRejectV1::SourceItemDuplicate { call_site: item.call_site().clone() });
            }
        }
        if expected != observed { return Err(CallableLoopSourceRouteRejectV1::SourceItemForeign); }
        Ok(())
    }
}

impl CallableLoopSourceTargetProbeV1 {
    pub(in crate::mir::builder) fn consume_for_checked_recipe_v2(
        self, items: &[CallableLoopSourceItemBindingV1],
        product: &crate::mir::loop_recipe_contract::VerifiedSourceLoopRecipeProductV2,
    ) -> Result<(), CallableLoopSourceRouteRejectV1> {
        if self.requirement_mismatch { return Err(CallableLoopSourceRouteRejectV1::SourceTargetRequirementMismatch); }
        if !self.uncovered.is_empty() { return Err(CallableLoopSourceRouteRejectV1::SourceTargetUnselected { call_sites: self.uncovered }); }
        let mut seen = std::collections::BTreeSet::new();
        for relation in &self.selected {
            let site = relation.call_site();
            let owned = crate::mir::resolved_semantics::OwnedExprSiteV1::new(product.source.source.owner(), site.clone());
            let item = items.iter().find(|item| item.call_site() == site)
                .ok_or(CallableLoopSourceRouteRejectV1::SourceItemForeign)?;
            let expected = delegated_result(product, site);
            if !product.delegates_call_v2(&owned) || !seen.insert(site.clone())
                || relation.target().is_none_or(|target|
                    target.namespace() != hakorune_mir_defs::SameModuleCallableNamespaceV1::StaticBoxMethod
                        || target.arity() != item.arity())
                || relation.requirement().is_none_or(|requirement| Some(requirement.representation()) != expected.as_ref()) {
                return Err(CallableLoopSourceRouteRejectV1::SourceTargetRequirementMismatch);
            }
        }
        if self.instance_methods.iter().any(|item|
            !items.contains(item) || !seen.insert(item.call_site().clone()) || !(product.delegates_call_v2(
                &crate::mir::resolved_semantics::OwnedExprSiteV1::new(product.source.source.owner(), item.call_site().clone()))
                || product.schedule().blocks_array_appends_v2().any(|observation|
                matches!(observation,
                    crate::mir::resolved_semantics::home_new_prefix::NamedArrayHomeObservationV1::Append { candidate, source, call, .. }
                        if candidate.site() == item.call_site() && source.append_candidate_is_null_v1(call) == Some(false))))) {
            return Err(CallableLoopSourceRouteRejectV1::SourceCallResidualEvidence);
        }
        for item in &self.core_methods {
            if !items.contains(item) || !seen.insert(item.call_site().clone()) {
                return Err(CallableLoopSourceRouteRejectV1::SourceItemForeign);
            }
        }
        // The original Append packet owns its outer call; its allocation
        // candidate is covered above by the ordinary instance bucket. Do not
        // demand a second generic Call owner for this already checked effect.
        for observation in product.schedule().blocks_array_appends_v2() {
            let crate::mir::resolved_semantics::home_new_prefix::NamedArrayHomeObservationV1::Append { call, .. }
                = observation else { return Err(CallableLoopSourceRouteRejectV1::SourceItemForeign); };
            if !items.iter().any(|item| item.call_site() == call.site()) || !seen.insert(call.site().clone()) {
                return Err(CallableLoopSourceRouteRejectV1::SourceCallResidualEvidence);
            }
        }
        // Expression Core loans are owned by the SAME checked frames, rather
        // than the older staged Core table. Normalize their exact resolver rows
        // here; a source-site match alone never grants call coverage.
        let mut expression_cores = std::collections::BTreeMap::new();
        let mut expression_dynamic = std::collections::BTreeMap::new();
        for evaluation in product.checked_expression_evaluations_v2() {
            for (node, core) in evaluation.cores() {
                if !core.matches_plan(evaluation.source(), *node) {
                    return Err(CallableLoopSourceRouteRejectV1::SourceItemForeign);
                }
                let original = CallableLoopSourceItemBindingV1::from_resolved(
                    product.source.source.owner(), core.source().source())?;
                let item = items.iter().find(|item| **item == original)
                    .ok_or(CallableLoopSourceRouteRejectV1::SourceItemForeign)?;
                if let Some(previous) = expression_cores.get(item.call_site()) {
                    if *previous != item {
                        return Err(CallableLoopSourceRouteRejectV1::SourceItemForeign);
                    }
                    continue;
                }
                let first_core_site = seen.insert(item.call_site().clone());
                if !first_core_site && !self.core_methods.contains(item) {
                    if crate::config::env::joinir_dev::debug_enabled() {
                        eprintln!(
                            "[checked-loop-core-duplicate] site={:?} expression_core=true initial_core={} dynamic={} instance={}",
                            item.call_site(),
                            self.core_methods.contains(item),
                            self.dynamic_invocations.iter().any(|source| source.call_site() == item.call_site()),
                            self.instance_methods.iter().any(|source| source.call_site() == item.call_site()),
                        );
                    }
                    return Err(CallableLoopSourceRouteRejectV1::SourceItemDuplicate { call_site: item.call_site().clone() });
                }
                expression_cores.insert(item.call_site().clone(), item);
            }
            for call in evaluation.calls() {
                observe_dynamic_call_v2(
                    call,
                    &self.dynamic_invocations,
                    items,
                    &self.core_methods,
                    &expression_cores,
                    &mut seen,
                    &mut expression_dynamic,
                )?;
            }
        }
        if expression_dynamic.len() != self.dynamic_invocations.len() {
            return Err(CallableLoopSourceRouteRejectV1::SourceItemForeign);
        }
        if seen != items.iter().map(|item| item.call_site().clone()).collect() {
            return Err(CallableLoopSourceRouteRejectV1::SourceCallResidualEvidence);
        }
        Ok(())
    }
}

fn observe_dynamic_call_v2(
    call: &crate::mir::resolved_semantics::home_new_prefix::LocalCallObservationV1,
    dynamic_invocations: &[CallableLoopDynamicInvocationSourceV1],
    items: &[CallableLoopSourceItemBindingV1],
    core_methods: &[CallableLoopSourceItemBindingV1],
    expression_cores: &std::collections::BTreeMap<
        SourceExprSiteV1,
        &CallableLoopSourceItemBindingV1,
    >,
    seen: &mut std::collections::BTreeSet<SourceExprSiteV1>,
    expression_dynamic: &mut std::collections::BTreeMap<
        SourceExprSiteV1,
        crate::mir::resolved_semantics::home_new_prefix::LocalCallObservationV1,
    >,
) -> Result<(), CallableLoopSourceRouteRejectV1> {
    if call.dynamic_invocation_result_lifecycle_v1().is_none() {
        return Ok(());
    }
    let site = call.site().site();
    if let Some(original) = expression_dynamic.get(site) {
        return if original == call {
            Ok(())
        } else {
            Err(CallableLoopSourceRouteRejectV1::SourceItemForeign)
        };
    }
    let source = dynamic_invocations
        .iter()
        .find(|source| source.call_site() == site && source.matches_checked_observation(call))
        .ok_or(CallableLoopSourceRouteRejectV1::SourceItemForeign)?;
    let exact_core_observation = core_methods.contains(source.item())
        || expression_cores
            .get(site)
            .is_some_and(|core_item| *core_item == source.item());
    let first_dynamic_site = seen.insert(source.call_site().clone());
    if !items.contains(source.item()) || (!first_dynamic_site && !exact_core_observation) {
        if crate::config::env::joinir_dev::debug_enabled() {
            eprintln!(
                "[checked-loop-dynamic-duplicate] site={:?} first={} exact_core={} initial_core={} expression_core={}",
                source.call_site(),
                first_dynamic_site,
                exact_core_observation,
                core_methods.contains(source.item()),
                expression_cores.contains_key(site),
            );
        }
        return Err(CallableLoopSourceRouteRejectV1::SourceItemDuplicate {
            call_site: source.call_site().clone(),
        });
    }
    expression_dynamic.insert(site.clone(), call.clone());
    Ok(())
}

fn delegated_result(product: &crate::mir::loop_recipe_contract::VerifiedSourceLoopRecipeProductV2,
    site: &SourceExprSiteV1,
) -> Option<VerifiedCallableResultRepresentationV1> {
    use crate::mir::resolved_semantics::home_new_prefix::LocalCallResultClassV1 as Result;
    use crate::mir::resolved_value_profile::{expression_class::ExpressionValueClassV1 as Class,
        expression_projection::ExpressionLeafClassV1 as Leaf};
    for step in product.schedule().calls_v2() {
        if let crate::mir::loop_recipe_contract::SourceLoopScheduledStepV2::Call { observation, .. } = step {
            if observation.site().site() == site { return match observation.result() {
                Result::I64 => Some(VerifiedCallableResultRepresentationV1::ExactI64),
                Result::Bool => Some(VerifiedCallableResultRepresentationV1::ExactBool),
                _ => None,
            }; }
        }
    }
    product.scheduled_call_arguments_v2().find_map(|frame| frame.source().nodes().iter().enumerate()
        .find(|(_, node)| node.site() == site).and_then(|(node, _)| match frame.evaluation().class(node) {
            Some(Leaf::Value(Class::I64)) => Some(VerifiedCallableResultRepresentationV1::ExactI64),
            Some(Leaf::Value(Class::Bool)) => Some(VerifiedCallableResultRepresentationV1::ExactBool),
            Some(Leaf::Value(Class::String)) => Some(VerifiedCallableResultRepresentationV1::ExactString),
            _ => None,
        }))
}
