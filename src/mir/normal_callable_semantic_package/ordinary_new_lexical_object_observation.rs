//! Original indexed caller observations corroborate slot result and route only.
use super::*;
use crate::mir::normal_callable_semantic_package::OrdinaryNewResultClassV1;
use crate::mir::resolved_semantics::home_new_prefix::{
    LocalCallObservationV1, LocalCallResultClassV1,
};

impl OrdinaryNewClaimLedgerV1 {
    /// Retained provider writers or constructor relations distinguish Dynamic reads.
    /// This lends storage meaning only; the original Call still owns admission.
    pub(in crate::mir) fn stored_receiver_uses_dynamic_field_v1(
        &self, target: &LexicalInstanceCallSourceTargetV1,
    ) -> Result<bool, String> {
        let Some((_, _, field, child)) = target.stored_receiver() else { return Ok(false); };
        let mut dynamic = false;
        for ((parent, name), source) in self.field_write_claims.object_sources().iter() {
            if source.dynamic_provider_field_v1(parent, name) == Some(field) {
                if source.class() != target.target().owner() {
                    return Err(freeze("lexical-terminal/stored-read-drift"));
                }
                dynamic = true;
            }
        }
        for relation in self.callable_result_classes.construction_child_relation_loans()
            .filter(|relation| relation.field() == field && relation.is_unqualified_dynamic_v1()) {
            if relation.child() != child { return Err(freeze("lexical-terminal/stored-read-drift")); }
            dynamic = true;
        }
        Ok(dynamic)
    }
    /// The local flow and original receiver observation select full packet
    /// demand. They grant neither callee input nor result/teardown permission.
    pub(super) fn has_original_receiver_nullable_packet_source_v1(
        &self,
        target: &LexicalInstanceCallSourceTargetV1,
    ) -> Result<bool, String> {
        if !target.is_self_receiver() {
            return Ok(false);
        }
        let Some(flow) = self.nullable_call_source(target.call_site()) else {
            return Ok(false);
        };
        let observation = self
            .receiver_call_observation(target.call_site())
            .ok_or_else(|| freeze("receiver-object/observation-missing"))?;
        let Some((_, destination)) = flow.local_binding() else {
            return Err(freeze("receiver-object/local-destination-missing"));
        };
        let class_matches = matches!(observation.class(),
            OrdinaryNewResultClassV1::NullableObject(class)
                if self.nullable_result_class(target.target()) == Some(class.as_ref()));
        if observation.callee() != target.target()
            || observation.destination() != destination
            || !class_matches
            || observation.arguments().len() != target.argument_sites().len()
            || observation
                .arguments()
                .iter()
                .zip(target.argument_sites())
                .enumerate()
                .any(|(ordinal, (argument, site))| {
                    argument.ordinal() as usize != ordinal || argument.site() != site
                })
        {
            return Err(format!(
                "{} callee={} destination={} class={:?}/{:?} arguments={:?}/{:?} flow_arity={}",
                freeze("receiver-object/original-local-source-drift"),
                observation.callee() == target.target(),
                observation.destination() == destination,
                observation.class(),
                self.nullable_result_class(target.target()),
                observation.arguments(),
                target.argument_sites(),
                flow.arguments().len(),
            ));
        }
        Ok(true)
    }

    pub(super) fn checked_object_slot_observation_v1(
        &self,
        source: &LexicalInstanceCallSourceTargetV1,
        result: Option<InvokeCallResultKind>,
    ) -> Result<Option<bool>, String> {
        let Some(completion) = self.completion_index.get(&source.call_site().owner()) else {
            return Ok(None);
        };
        let completion = completion.as_ref().map_err(|issue| {
            format!("{}: {issue:?}", freeze("lexical-object/caller-completion"))
        })?;
        if completion.owner() != source.call_site().owner() {
            return Err(freeze("lexical-object/caller-completion-owner"));
        }
        // An original Append candidate is an expression acquisition, not a
        // LOCAL/Return observation. Its source loan corroborates result class
        // without inventing a binding, Home or local-call route. The packet
        // still owes complete callee input/Normal/End and the real consumer
        // must later bind caller cleanup before physical materialization.
        if let Some(Ok(ingress)) = &self.borrowed_formal_source {
            if let Some(producer) = ingress.original_array_producer_source_v1(source)? {
                self.callable_result_classes
                    .recheck_object_call_roots_v1(source, producer)
                    .map_err(|issue| freeze(&format!("lexical-object/array-source/{issue}")))?;
                use crate::mir::normal_callable_semantic_package::OrdinaryNewResultClassV1;
                let expected = match producer.result_class() {
                    OrdinaryNewResultClassV1::Object(_) => InvokeCallResultKind::Handle,
                    OrdinaryNewResultClassV1::NullableObject(_) => {
                        InvokeCallResultKind::NullableHandle
                    }
                    _ => return Err(freeze("lexical-object/array-source-result")),
                };
                if result.is_some_and(|kind| kind != expected) {
                    return Err(freeze("lexical-object/array-result-observation"));
                }
                return Ok(Some(false));
            }
        }
        let Some(flow) = completion.cleanup().root_flow() else {
            return Ok(None);
        };
        let dynamic_result = {
            let class = self.callable_result_classes.get(source.target());
            let (expected, class_name, nullable) = match class {
                Some(OrdinaryNewResultClassV1::Object(class)) => {
                    (InvokeCallResultKind::Handle, class.as_ref(), false)
                }
                Some(OrdinaryNewResultClassV1::NullableObject(class)) => {
                    (InvokeCallResultKind::NullableHandle, class.as_ref(), true)
                }
                _ => {
                    return corroborate_original_object_observations_v1(
                        source.call_site(), result, flow.local_calls(), None,
                    ).map(Some);
                }
            };
            let roots = self.callable_result_classes
                .checked_original_source_roots_v1(source.target(), source.callee_owner())
                .map_err(|issue| freeze(&format!("lexical-object/dynamic-source/{issue}")))?;
            let rooted_class = !roots.is_empty() && roots.iter().all(|root| match root.origin() {
                crate::mir::normal_callable_semantic_package::ResultValueOriginV1::Fresh(name) => name.as_ref() == class_name,
                crate::mir::normal_callable_semantic_package::ResultValueOriginV1::Null => nullable,
                _ => false,
            });
            rooted_class.then_some(expected)
        };
        corroborate_original_object_observations_v1(
            source.call_site(), result, flow.local_calls(), dynamic_result,
        )
            .map(Some)
    }
}

fn corroborate_original_object_observations_v1(
    site: &OwnedExprSiteV1,
    result: Option<InvokeCallResultKind>,
    observations: &[LocalCallObservationV1],
    dynamic_result: Option<InvokeCallResultKind>,
) -> Result<bool, String> {
    let mut matching = observations.iter().filter(|row| row.site() == site);
    let Some(observation) = matching.next() else {
        return Ok(false);
    };
    if matching.next().is_some() || observation.owner() != site.owner() {
        return Err(freeze("lexical-object/caller-observation-identity"));
    }
    if !matches!(
        observation.result(),
        LocalCallResultClassV1::Handle | LocalCallResultClassV1::Nullable
    ) && !(observation.result() == LocalCallResultClassV1::Dynamic && dynamic_result.is_some())
        || match observation.result() {
            LocalCallResultClassV1::Handle => result.is_some_and(|kind| kind != InvokeCallResultKind::Handle),
            LocalCallResultClassV1::Nullable => result.is_some_and(|kind| kind != InvokeCallResultKind::NullableHandle),
            LocalCallResultClassV1::Dynamic => result.is_some_and(|kind| Some(kind) != dynamic_result),
            _ => true,
        }
    {
        return Err(freeze("lexical-object/result-observation"));
    }
    Ok(true)
}

#[cfg(test)]
#[path = "ordinary_new_lexical_object_observation_tests.rs"]
mod tests;
