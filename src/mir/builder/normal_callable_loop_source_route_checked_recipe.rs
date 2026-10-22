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
            if !product.delegates_call_v2(&owned) || !seen.insert(site)
                || relation.target().is_none_or(|target|
                    target.namespace() != hakorune_mir_defs::SameModuleCallableNamespaceV1::StaticBoxMethod
                        || target.arity() != item.arity())
                || relation.requirement().is_none_or(|requirement| Some(requirement.representation()) != expected.as_ref()) {
                return Err(CallableLoopSourceRouteRejectV1::SourceTargetRequirementMismatch);
            }
        }
        if self.instance_methods.iter().any(|item|
            !items.contains(item) || !seen.insert(item.call_site()) || !(product.delegates_call_v2(
                &crate::mir::resolved_semantics::OwnedExprSiteV1::new(product.source.source.owner(), item.call_site().clone()))
                || product.schedule().blocks_array_appends_v2().any(|observation|
                matches!(observation,
                    crate::mir::resolved_semantics::home_new_prefix::NamedArrayHomeObservationV1::Append { candidate, source, call, .. }
                        if candidate.site() == item.call_site() && source.append_candidate_is_null_v1(call) == Some(false))))) {
            return Err(CallableLoopSourceRouteRejectV1::SourceCallResidualEvidence);
        }
        for item in &self.core_methods {
            if !items.contains(item) || !seen.insert(item.call_site()) {
                return Err(CallableLoopSourceRouteRejectV1::SourceItemForeign);
            }
        }
        // The original Append packet owns its outer call; its allocation
        // candidate is covered above by the ordinary instance bucket. Do not
        // demand a second generic Call owner for this already checked effect.
        for observation in product.schedule().blocks_array_appends_v2() {
            let crate::mir::resolved_semantics::home_new_prefix::NamedArrayHomeObservationV1::Append { call, .. }
                = observation else { return Err(CallableLoopSourceRouteRejectV1::SourceItemForeign); };
            if !items.iter().any(|item| item.call_site() == call.site()) || !seen.insert(call.site()) {
                return Err(CallableLoopSourceRouteRejectV1::SourceCallResidualEvidence);
            }
        }
        // Expression Core loans are owned by the SAME checked frames, rather
        // than the older staged Core table. Normalize their exact resolver rows
        // here; a source-site match alone never grants call coverage.
        let mut expression_cores = std::collections::BTreeSet::new();
        for frame in product.schedule().frames() {
            use crate::mir::resolved_semantics::home_new_prefix::LifecyclePrefixSourceV1 as Prefix;
            let evaluation = match frame.prefix() {
                Prefix::BoolCondition { frame, .. } => frame.evaluation(),
                Prefix::InitializerExpression { frame, .. } => frame.evaluation(),
                Prefix::BindingRebind { frame, .. } => frame.evaluation(),
                _ => return Err(CallableLoopSourceRouteRejectV1::SourceItemForeign),
            };
            for (node, core) in evaluation.cores() {
                if !core.matches_plan(frame.source(), *node) {
                    return Err(CallableLoopSourceRouteRejectV1::SourceItemForeign);
                }
                let original = CallableLoopSourceItemBindingV1::from_resolved(
                    product.source.source.owner(), core.source().source())?;
                let item = items.iter().find(|item| **item == original)
                    .ok_or(CallableLoopSourceRouteRejectV1::SourceItemForeign)?;
                if !expression_cores.insert(item.call_site())
                    || (!seen.insert(item.call_site()) && !self.core_methods.contains(item)) {
                    return Err(CallableLoopSourceRouteRejectV1::SourceItemDuplicate { call_site: item.call_site().clone() });
                }
            }
        }
        if seen != items.iter().map(|item| item.call_site()).collect() {
            return Err(CallableLoopSourceRouteRejectV1::SourceCallResidualEvidence);
        }
        Ok(())
    }
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
