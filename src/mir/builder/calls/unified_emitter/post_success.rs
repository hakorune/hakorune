//! Prepared post-success payload for the canonical generic unified Call.
//!
//! This is intentionally Builder-free. It captures only the final existing
//! invocation descriptors that I0 may consume after a physical Call receipt;
//! it neither publishes facts nor selects annotation policy.

use crate::mir::builder::function_signature_lookup::FunctionSignatureLookupV1;
use crate::mir::builder::{MirBuilder, ValueId};
use crate::mir::definitions::call_unified::{Callee, CalleeBoxKind, TypeCertainty};

use super::super::annotation::callee_sig_name;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum UnifiedCallSignaturePublicationV1 {
    Existing,
    ExternalSourceBound,
}

/// Immutable payload prepared after unified-call normalization and before its
/// physical instruction. It is deliberately non-Clone: the later receipt
/// owner must consume one prepared payload at most once.
pub(super) struct PreparedUnifiedCallPostSuccessV1<'lookup> {
    signature: Option<PreparedSignatureAnnotationV1>,
    collection_result: Option<PreparedCollectionResultAnnotationV1>,
    map_write: Option<Box<[crate::mir::builder::types::map_value::post_success::MapWriteObservationDescriptorV1]>>,
    lookup: Option<&'lookup dyn FunctionSignatureLookupV1>,
}

#[derive(Debug, PartialEq, Eq)]
struct PreparedSignatureAnnotationV1 {
    destination: ValueId,
    function_name: String,
}

#[derive(Debug)]
struct PreparedCollectionResultAnnotationV1 {
    destination: ValueId,
    callee: Callee,
    arguments: Box<[ValueId]>,
}

impl<'lookup> PreparedUnifiedCallPostSuccessV1<'lookup> {
    /// Build descriptors only from the already-finalized call shape.
    ///
    /// There is intentionally no `MirBuilder` argument, fact write, lookup,
    /// instruction emission, or commit capability in S0.
    pub(super) fn prepare(
        destination: Option<ValueId>,
        callee: &Callee,
        arguments: &[ValueId],
        map_write: Option<
            crate::mir::builder::types::map_value::post_success::PreparedMapWriteReplayV1,
        >,
        lookup: Option<&'lookup dyn FunctionSignatureLookupV1>,
        signature_publication: UnifiedCallSignaturePublicationV1,
    ) -> Self {
        let signature = match signature_publication {
            UnifiedCallSignaturePublicationV1::Existing => destination.and_then(|destination| {
                let arity = signature_arity(callee, arguments);
                callee_sig_name(callee, arity).map(|function_name| PreparedSignatureAnnotationV1 {
                    destination,
                    function_name,
                })
            }),
            UnifiedCallSignaturePublicationV1::ExternalSourceBound => None,
        };
        let collection_result =
            destination.map(|destination| PreparedCollectionResultAnnotationV1 {
                destination,
                callee: callee.clone(),
                arguments: arguments.into(),
            });
        Self {
            signature,
            collection_result,
            map_write: map_write.map(|replay| replay.into_observations()),
            lookup,
        }
    }

    /// Consume the payload only after its physical Call instruction succeeds.
    ///
    /// The delegated annotation modules retain their existing policies. This
    /// owner changes only when their already-finalized invocation is allowed.
    pub(super) fn commit_after_success(self, builder: &mut MirBuilder) {
        if let Some(observations) = self.map_write {
            for observation in observations.iter() {
                super::super::super::types::map_value::observe_map_write_call(
                    builder,
                    observation.callee(),
                    observation.args(),
                );
            }
        }
        if let Some(signature) = self.signature {
            if crate::config::env::builder_debug_annotation() {
                let ring0 = crate::runtime::get_global_ring0();
                ring0.log.debug(&format!(
                    "[annotation] dst=%{} func_name={}",
                    signature.destination.0, signature.function_name
                ));
            }
            if let Some(lookup) = self.lookup {
                super::super::annotation::annotate_call_result_from_func_name_with_lookup(
                    builder,
                    signature.destination,
                    &signature.function_name,
                    Some(lookup),
                );
            } else {
                super::super::annotation::annotate_call_result_from_func_name(
                    builder,
                    signature.destination,
                    &signature.function_name,
                );
            }
        }
        if let Some(collection) = self.collection_result {
            super::super::super::types::array_element::annotate_array_element_result(
                builder,
                collection.destination,
                &collection.callee,
                &collection.arguments,
            );
            super::super::super::types::map_value::annotate_map_get_result(
                builder,
                collection.destination,
                &collection.callee,
                &collection.arguments,
            );
        }
        crate::mir::builder::emit_guard::verify_after_call(builder);
    }
}

fn signature_arity(callee: &Callee, arguments: &[ValueId]) -> usize {
    match callee {
        Callee::Method {
            receiver: Some(receiver),
            ..
        } if arguments.first() == Some(receiver) => arguments.len().saturating_sub(1),
        _ => arguments.len(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn global_payload_keeps_the_final_signature_and_collection_shape() {
        let destination = ValueId::new(7);
        let arguments = [ValueId::new(1), ValueId::new(2)];
        let prepared = PreparedUnifiedCallPostSuccessV1::prepare(
            Some(destination),
            &Callee::Global("answer".to_string()),
            &arguments,
            None,
            None,
            UnifiedCallSignaturePublicationV1::Existing,
        );

        assert_eq!(
            prepared.signature,
            Some(PreparedSignatureAnnotationV1 {
                destination,
                function_name: "answer/2".to_string(),
            })
        );
        let collection = prepared.collection_result.expect("destination descriptor");
        assert_eq!(collection.destination, destination);
        assert_eq!(collection.arguments.as_ref(), arguments);
    }

    #[test]
    fn method_signature_excludes_the_explicit_receiver_argument() {
        let receiver = ValueId::new(3);
        let prepared = PreparedUnifiedCallPostSuccessV1::prepare(
            Some(ValueId::new(8)),
            &Callee::Method {
                box_name: "ArrayBox".to_string(),
                method: "get".to_string(),
                receiver: Some(receiver),
                certainty: TypeCertainty::Known,
                box_kind: CalleeBoxKind::RuntimeData,
            },
            &[receiver, ValueId::new(4)],
            None,
            None,
            UnifiedCallSignaturePublicationV1::Existing,
        );

        assert_eq!(
            prepared.signature,
            Some(PreparedSignatureAnnotationV1 {
                destination: ValueId::new(8),
                function_name: "ArrayBox.get/1".to_string(),
            })
        );
    }

    #[test]
    fn no_destination_prepares_no_post_success_fact_descriptors() {
        let prepared = PreparedUnifiedCallPostSuccessV1::prepare(
            None,
            &Callee::Value(ValueId::new(9)),
            &[ValueId::new(9)],
            None,
            None,
            UnifiedCallSignaturePublicationV1::Existing,
        );

        assert!(prepared.signature.is_none());
        assert!(prepared.collection_result.is_none());
    }

    #[test]
    fn external_source_bound_publication_suppresses_signature_annotation() {
        let prepared = PreparedUnifiedCallPostSuccessV1::prepare(
            Some(ValueId::new(10)),
            &Callee::Global("answer/0".to_string()),
            &[],
            None,
            None,
            UnifiedCallSignaturePublicationV1::ExternalSourceBound,
        );

        assert!(prepared.signature.is_none());
        assert!(prepared.collection_result.is_some());
    }
}
