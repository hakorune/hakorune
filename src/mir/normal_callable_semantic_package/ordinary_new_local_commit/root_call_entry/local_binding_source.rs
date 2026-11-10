//! Original source membership of existing lifecycle prefix binding groups.
use super::*;

impl RootLifecyclePrefixBindingGroupV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn matches_source(
        &self,
        source: &LifecyclePrefixSourceV1,
    ) -> bool {
        if self.site() != source.site() {
            return false;
        }
        match (&self.proof, source) {
            (PrefixPhysicalProofV1::EvaluatedBinding(EvaluatedBindingProjectionV1::ComparisonInitializer(packet)),
                LifecyclePrefixSourceV1::ComparisonInitializer { operation, .. }) => Rc::ptr_eq(packet.source(), operation),
            (PrefixPhysicalProofV1::SharedInitializer(packet), LifecyclePrefixSourceV1::SharedInitializer { source, .. }) => Rc::ptr_eq(&packet.source, source),
            (PrefixPhysicalProofV1::FieldMapInstall(packet), LifecyclePrefixSourceV1::FieldMapInstall { frame, .. }) => Rc::ptr_eq(packet.frame(), frame),
            (
                PrefixPhysicalProofV1::StoredMapGet(packet),
                LifecyclePrefixSourceV1::StoredMapGet { frame, .. },
            ) => Rc::ptr_eq(packet.frame(), frame),
            (
                PrefixPhysicalProofV1::Loop(packet),
                LifecyclePrefixSourceV1::Loop { product, .. },
            ) => Rc::ptr_eq(packet.product(), product),
            (
                PrefixPhysicalProofV1::EvaluatedBinding(EvaluatedBindingProjectionV1::Initializer(
                    packet,
                )),
                LifecyclePrefixSourceV1::InitializerExpression { frame, .. },
            ) => Rc::ptr_eq(packet.frame(), frame),
            (
                PrefixPhysicalProofV1::EvaluatedBinding(EvaluatedBindingProjectionV1::Rebind(
                    packet,
                )),
                LifecyclePrefixSourceV1::BindingRebind { frame, .. },
            ) => Rc::ptr_eq(packet.frame(), frame),
            (
                PrefixPhysicalProofV1::BorrowedFieldWrite(packet),
                LifecyclePrefixSourceV1::BorrowedFieldWrite { frame, .. },
            ) => Rc::ptr_eq(packet.source().frame(), frame),
            (
                PrefixPhysicalProofV1::Call(_) | PrefixPhysicalProofV1::DirectScalar(_),
                LifecyclePrefixSourceV1::Call { .. },
            ) => true,
            (
                PrefixPhysicalProofV1::FieldArrayAppend(packet),
                LifecyclePrefixSourceV1::FieldArrayAppend { proof, index, .. },
            ) => packet.source().matches_original(proof, *index),
            (
                PrefixPhysicalProofV1::FieldArrayAppend(packet),
                LifecyclePrefixSourceV1::FieldTextAppend { frame, .. },
            ) => packet.source().matches_text_frame(frame),
            (
                PrefixPhysicalProofV1::BoolCondition(packet),
                LifecyclePrefixSourceV1::BoolCondition { frame, .. },
            ) => Rc::ptr_eq(packet.frame(), frame),
            (
                PrefixPhysicalProofV1::PrintExpression(packet),
                LifecyclePrefixSourceV1::PrintExpression { frame, .. },
            ) => Rc::ptr_eq(packet.frame(), frame),
            (
                PrefixPhysicalProofV1::OwnedArrayAppend(packet),
                LifecyclePrefixSourceV1::OwnedArrayAppend {
                    holder, candidate, ..
                },
            ) => packet.source().matches_original_v1(holder, candidate),
            _ => false,
        }
    }
}
