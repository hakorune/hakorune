//! Read-only traversal of the existing group/Loop/argument tree.
use super::*;
impl RootLifecyclePrefixBindingGroupV1 {
    pub(in crate::mir) fn visit_i64_origins_v1(
        &self,
        visit: &mut dyn FnMut(
            &crate::mir::normal_callable_semantic_package::ExpressionReferenceReadV1,
        ) -> Result<(), String>,
    ) -> Result<(), String> {
        match &self.proof {
            PrefixPhysicalProofV1::SharedInitializer(_) | PrefixPhysicalProofV1::Call(None) => {
                Ok(())
            }
            PrefixPhysicalProofV1::Loop(packet) => packet.visit_i64_origins_v1(visit),
            PrefixPhysicalProofV1::Call(Some(packet)) => packet.visit_i64_origins_v1(visit),
            PrefixPhysicalProofV1::DirectScalar(packet) => packet.visit_i64_origins_v1(visit),
            PrefixPhysicalProofV1::EvaluatedBinding(EvaluatedBindingProjectionV1::Initializer(
                packet,
            )) => packet.expression().visit_i64_origins_v1(visit),
            PrefixPhysicalProofV1::EvaluatedBinding(EvaluatedBindingProjectionV1::Rebind(
                packet,
            )) => packet.expression().visit_i64_origins_v1(visit),
            PrefixPhysicalProofV1::PrintExpression(packet) => {
                packet.expression().visit_i64_origins_v1(visit)
            }
            PrefixPhysicalProofV1::BoolCondition(packet) => packet.visit_i64_origins_v1(visit),
            PrefixPhysicalProofV1::StoredMapGet(packet) => packet.visit_i64_origins_v1(visit),
            PrefixPhysicalProofV1::FieldMapInstall(packet) => packet.visit_i64_origins_v1(visit),
            PrefixPhysicalProofV1::FieldArrayAppend(packet) => packet.visit_i64_origins_v1(visit),
            PrefixPhysicalProofV1::OwnedArrayAppend(packet) => packet.visit_i64_origins_v1(visit),
            PrefixPhysicalProofV1::BorrowedFieldWrite(packet) => packet.visit_i64_origins_v1(visit),
        }
    }
}
