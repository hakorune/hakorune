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
            (
                PrefixPhysicalProofV1::SharedInitializer(packet),
                LifecyclePrefixSourceV1::SharedInitializer { source, .. },
            ) => Rc::ptr_eq(&packet.source, source),
            (
                PrefixPhysicalProofV1::FieldMapInstall(packet),
                LifecyclePrefixSourceV1::FieldMapInstall { frame, .. },
            ) => Rc::ptr_eq(packet.frame(), frame),
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

impl OrdinaryNewClaimLedgerV1 {
    /// Source-validated ordinary initializer rows use the same physical omission
    /// certificate. Mandatory rows in any consumer remain protected.
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn initializer_optional_bindings_v1(
        &self,
        owner: FunctionOwnerIdV1,
        function: &MirFunction,
    ) -> Result<Vec<(BasicBlockId, MirInstruction)>, String> {
        let roots = self
            .root_local_call_bindings
            .borrow()
            .get(&owner)
            .cloned()
            .unwrap_or_default();
        let groups = RootLifecyclePrefixBindingGroupV1::snapshot_tree_v2(&roots);
        // Per-local FieldGet elision retains its existing owner. This source
        // certificate lends only context-independent read-elision laws.
        let reads = crate::mir::passes::dce::local_fields::LocalReadInfo::default();
        let eligible = |row: &MirInstruction| {
            matches!(row,MirInstruction::Copy {..} | MirInstruction::Const {value:crate::mir::ConstValue::Null,..})
            || crate::mir::passes::dce::local_fields::is_removable_effect_sensitive_read_instruction(row,&reads)
        };
        let mut protected = std::collections::BTreeSet::new();
        let mut candidates = Vec::new();
        for group in &groups {
            let packet=group.initializer().filter(|_| group.bindings().iter().any(|(_,row)|
                crate::mir::passes::dce::local_fields::is_removable_effect_sensitive_read_instruction(row,&reads)));
            let mut source = group.bindings().to_vec();
            if let Some(packet) = packet {
                group.validate_recorded_in_context(
                    CallValidationContextV1::Lowering(self),
                    function,
                    None,
                )?;
                source.extend(
                    packet.source_dependencies_v1(CallValidationContextV1::Lowering(self))?,
                );
                source.extend(packet.receipt_copy_binding_v1().cloned());
            }
            for binding in source {
                if packet.is_some() && eligible(&binding.1) {
                    if !candidates.contains(&binding) {
                        candidates.push(binding);
                    }
                } else if let Some(dst) = binding.1.dst_value() {
                    protected.insert(dst);
                }
            }
        }
        candidates.retain(|(_, row)| row.dst_value().is_some_and(|dst| !protected.contains(&dst)));
        Ok(candidates)
    }
}
