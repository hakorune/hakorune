//! Corroborate SAME child packets against the source writer's ordered frames.
use super::*;
use crate::mir::builder::EmittedSourceLoopGraphV2;

impl EmittedSourceLoopProjectionV2 {
    pub(super) fn validate_frames_v2(&self, graph: &EmittedSourceLoopGraphV2,
        context: CallValidationContextV1<'_>, function: &MirFunction,
        finishing: Option<&crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit::physical_boundary::FinishedBindings>,
    ) -> Result<Vec<RootLifecyclePrefixBindingGroupV1>, String> {
        let children = self.snapshot();
        if !Rc::ptr_eq(self.product(), &graph.product) || children.len() != self.expected.len()
            || children.len() != graph.frames.len() + graph.effects.len() { return Err(freeze("source-loop/frame-census")); }
        let mut index = 0;
        for (child, expected) in children.iter().zip(&self.expected) {
            if let LifecyclePrefixSourceV1::Call { site, statement } = expected {
                let mut rows = graph.effects.iter().filter(|row| row.packet.site() == site);
                let row = rows.next().ok_or_else(|| freeze("source-loop/call-source"))?;
                let mut calls = graph.product.schedule().calls_v2().filter_map(|step| match step {
                    crate::mir::loop_recipe_contract::SourceLoopScheduledStepV2::Call { site: original, observation, result, .. }
                        if original == statement && observation.site() == site => Some((observation, result)),
                    _ => None,
                });
                let (observation, result) = calls.next().ok_or_else(|| freeze("source-loop/call-source"))?;
                if calls.next().is_some() || rows.next().is_some() || !child.matches_source(expected)
                    || !child.same_effect_packet_v2(&row.packet)
                    || row.result.as_ref().map(|(key, _)| key) != result.as_ref() {
                    return Err(freeze("source-loop/call-source"));
                }
                child.corroborate_loop_call_v2(observation, row.entry, row.end,
                    row.result.as_ref().map(|(_, relation)| relation.values().0), context, function, finishing.is_none())?;
                child.check_physical_bindings_v1(function, finishing)?;
                child.validate_recorded_in_context(context, function, finishing)?;
                continue;
            }
            if let LifecyclePrefixSourceV1::OwnedArrayAppend { site, holder, candidate, .. } = expected {
                let packet = child.owned_array_append().ok_or_else(|| freeze("source-loop/append-packet"))?;
                let mut rows = graph.effects.iter().filter(|row| row.packet.site() == site);
                let row = rows.next().ok_or_else(|| freeze("source-loop/append-source"))?;
                if rows.next().is_some() || !child.matches_source(expected)
                    || !child.same_effect_packet_v2(&row.packet) || !packet.source().matches_original_v1(holder, candidate) {
                    return Err(freeze("source-loop/append-source"));
                }
                child.check_physical_bindings_v1(function, finishing)?;
                child.validate_recorded_in_context(context, function, finishing)?;
                continue;
            }
            let observed = graph.frames.get(index).ok_or_else(|| freeze("source-loop/frame-census"))?;
            if index != observed.index || !child.matches_source(expected) { return Err(freeze("source-loop/frame-source")); }
            match expected {
                LifecyclePrefixSourceV1::InitializerExpression { .. } => {
                    child.initializer().ok_or_else(|| freeze("source-loop/initializer-packet"))?
                        .corroborate_loop_frame_v2(observed.entry, observed.end, observed.value)?;
                }
                LifecyclePrefixSourceV1::BindingRebind { .. } => {
                    child.rebind().ok_or_else(|| freeze("source-loop/rebind-packet"))?
                        .corroborate_loop_frame_v2(observed.entry, observed.end, observed.value)?;
                }
                LifecyclePrefixSourceV1::BoolCondition { .. } => {
                    let materialization = observed.condition.as_ref().ok_or_else(|| freeze("source-loop/condition-materialization"))?;
                    let targets = observed.targets.ok_or_else(|| freeze("source-loop/condition-targets"))?;
                    child.bool_condition().ok_or_else(|| freeze("source-loop/condition-packet"))?
                        .corroborate_loop_frame_v2(&graph.product, index, observed.entry, observed.end, observed.value, materialization, targets)?;
                }
                _ => return Err(freeze("source-loop/frame-purpose")),
            }
            child.require_sealed_condition_v1()?;
            child.check_physical_bindings_v1(function, finishing)?;
            child.validate_recorded_in_context(context, function, finishing)?;
            index += 1;
        }
        if index != graph.frames.len() { return Err(freeze("source-loop/frame-census")); }
        Ok(children)
    }
}
