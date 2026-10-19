//! Original local materialization evidence travels with the existing value slot.
use super::*;
use crate::mir::builder::stmts::CompletedLocalBindingV1;
use crate::mir::{BasicBlockId, MirInstruction};

#[derive(Debug, Clone)]
enum LocalProducerPurposeV1 {
    ScalarJoin(Rc<ScalarJoinPointValueV1>),
    Initializer,
    BindingRebind(std::rc::Rc<crate::mir::resolved_semantics::home_new_prefix::CheckedBindingRebindFrameV1>),
}
impl PartialEq for LocalProducerPurposeV1 {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Initializer, Self::Initializer) => true,
            (Self::ScalarJoin(a), Self::ScalarJoin(b)) => Rc::ptr_eq(a, b),
            (Self::BindingRebind(a), Self::BindingRebind(b)) => std::rc::Rc::ptr_eq(a, b),
            _ => false,
        }
    }
}
/// An immutable source coordinate borrows the SAME indexed physical Phi.
/// This is a value purpose, not a second loop registry or a completion grant.
#[derive(Debug, Clone)]
pub(in crate::mir) struct ScalarJoinPointValueV1 {
    source: crate::mir::resolved_semantics::home_new_prefix::CurrentScalarProducerV1,
    relation: Rc<crate::mir::builder::ssa::local::LocalSsaPhysicalRelationV1>,
    branch: Option<Rc<crate::mir::normal_callable_semantic_package::BranchScalarPhiObservationV1>>,
}
impl PartialEq for ScalarJoinPointValueV1 {
    fn eq(&self, other: &Self) -> bool {
        self.source == other.source && Rc::ptr_eq(&self.relation, &other.relation)
    }
}
impl ScalarJoinPointValueV1 {
    pub(in crate::mir::builder) fn new(
        source: crate::mir::resolved_semantics::home_new_prefix::CurrentScalarProducerV1,
        relation: Rc<crate::mir::builder::ssa::local::LocalSsaPhysicalRelationV1>,
        function: &crate::mir::MirFunction,
    ) -> Result<Rc<Self>, String> {
        use crate::mir::resolved_semantics::home_new_prefix::CurrentScalarProducerV1 as Producer;
        if !matches!(source, Producer::LoopHeader(_) | Producer::LoopAfter { .. }) {
            return Err("[freeze:contract][local-read/loop-point-source-kind]".into());
        }
        relation.identity_phi_binding_v1()?;
        if matches!(source, Producer::LoopAfter { .. }) { relation.require_completed_v1()?; }
        relation.verify(function)?;
        Ok(Rc::new(Self { source, relation, branch: None }))
    }
    pub(in crate::mir) fn from_branch_v1(
        branch: Rc<crate::mir::normal_callable_semantic_package::BranchScalarPhiObservationV1>,
    ) -> Rc<Self> {
        Rc::new(Self { source: crate::mir::resolved_semantics::home_new_prefix::CurrentScalarProducerV1::BranchNormal(Rc::clone(branch.source())),
            relation: Rc::clone(branch.relation()), branch: Some(branch) })
    }
    pub(in crate::mir) fn branch(&self) -> Option<&Rc<crate::mir::normal_callable_semantic_package::BranchScalarPhiObservationV1>> { self.branch.as_ref() }
    pub(in crate::mir) fn binding(&self) -> BindingRefV1 {
        use crate::mir::resolved_semantics::home_new_prefix::CurrentScalarProducerV1 as Producer;
        match &self.source { Producer::LoopHeader(basis) => basis.binding(),
            Producer::LoopAfter { binding, .. } => *binding, Producer::BranchNormal(basis) => basis.binding(), _ => unreachable!("constructor checked Loop purpose") }
    }
    pub(in crate::mir) fn value(&self) -> ValueId { self.relation.values().0 }
    pub(in crate::mir) fn source(&self) -> &crate::mir::resolved_semantics::home_new_prefix::CurrentScalarProducerV1 { &self.source }
    pub(in crate::mir) fn relation(&self) -> &Rc<crate::mir::builder::ssa::local::LocalSsaPhysicalRelationV1> { &self.relation }
}

#[derive(Debug, Clone, PartialEq)]
pub(in crate::mir) struct LocalProvenanceV1 {
    purpose: LocalProducerPurposeV1,
    root: BindingRefV1,
    root_value: ValueId,
    copies: Vec<(BasicBlockId, MirInstruction)>,
}

impl LocalProvenanceV1 {
    pub(in crate::mir) fn loan_scalar_join_v1(&self,
        source: &crate::mir::resolved_semantics::home_new_prefix::CurrentScalarProducerV1,
        result: ValueId,
    ) -> Result<&Rc<ScalarJoinPointValueV1>, String> {
        let LocalProducerPurposeV1::ScalarJoin(point) = &self.purpose else {
            return Err("[freeze:contract][local-read/loop-point-purpose]".into());
        };
        if point.source() != source { return Err("[freeze:contract][local-read/loop-point-source]".into()); }
        self.loan_copies(point.binding(), point.value(), result)?;
        Ok(point)
    }
    pub(in crate::mir) fn original_copy_dependencies_v1(&self, result: ValueId)
        -> Result<&[(BasicBlockId, MirInstruction)], String> {
        self.loan_copies(self.root, self.root_value, result)
    }
    /// Borrow the existing local initializer and its complete alias chain.
    /// The caller still owes the original source initializer and actual producer.
    pub(in crate::mir) fn loan_initializer_v1(&self, root: BindingRefV1, result: ValueId)
        -> Result<(ValueId, &[(BasicBlockId, MirInstruction)]), String> {
        if !matches!(self.purpose, LocalProducerPurposeV1::Initializer) {
            return Err("[freeze:contract][local-read/rebind-not-initializer]".into());
        }
        let copies = self.loan_copies(root, self.root_value, result)?;
        Ok((self.root_value, copies))
    }
    pub(in crate::mir) fn loan_checked_rebind_v1(&self,
        frame: &std::rc::Rc<crate::mir::resolved_semantics::home_new_prefix::CheckedBindingRebindFrameV1>, result: ValueId,
    ) -> Result<(ValueId, &[(BasicBlockId, MirInstruction)]), String> {
        if !matches!(&self.purpose, LocalProducerPurposeV1::BindingRebind(original) if std::rc::Rc::ptr_eq(original, frame)) {
            return Err("[freeze:contract][local-read/rebind-purpose]".into());
        }
        Ok((self.root_value, self.loan_copies(frame.target(), self.root_value, result)?))
    }
    pub(super) fn matches(&self, binding: BindingRefV1, value: ValueId) -> bool {
        self.root == binding && self.root_value == value
    }

    pub(in crate::mir) fn loan_copies(
        &self,
        binding: BindingRefV1,
        value: ValueId,
        result: ValueId,
    ) -> Result<&[(BasicBlockId, MirInstruction)], String> {
        if !self.matches(binding, value) {
            return Err("[freeze:contract][forwarded-copy/root]".into());
        }
        let mut current = value;
        for (index, copy) in self.copies.iter().enumerate() {
            let MirInstruction::Copy { dst, src } = &copy.1 else {
                return Err("[freeze:contract][forwarded-copy/instruction]".into());
            };
            if *src != current || self.copies[..index].contains(copy) {
                return Err("[freeze:contract][forwarded-copy/chain]".into());
            }
            current = *dst;
        }
        if current != result {
            return Err("[freeze:contract][forwarded-copy/result]".into());
        }
        Ok(&self.copies)
    }
}

#[derive(Debug, Clone)]
struct MaterializedValueV1 {
    value: ValueId,
    provenance: Option<LocalProvenanceV1>,
}

#[derive(Debug, Clone, Default)]
pub(in crate::mir::builder) struct MaterializedValuesV1 {
    entries: BTreeMap<BindingRefV1, MaterializedValueV1>,
}

impl MaterializedValuesV1 {
    pub(super) fn get(&self, binding: &BindingRefV1) -> Option<&ValueId> {
        self.entries.get(binding).map(|entry| &entry.value)
    }

    /// Every ordinary publication/rebind invalidates the old local proof.
    pub(super) fn insert(&mut self, binding: BindingRefV1, value: ValueId) -> Option<ValueId> {
        self.entries
            .insert(
                binding,
                MaterializedValueV1 {
                    value,
                    provenance: None,
                },
            )
            .map(|entry| entry.value)
    }

    pub(super) fn contains_key(&self, binding: &BindingRefV1) -> bool {
        self.entries.contains_key(binding)
    }

    pub(super) fn provenance(&self, binding: &BindingRefV1) -> Option<LocalProvenanceV1> {
        self.entries
            .get(binding)
            .and_then(|entry| entry.provenance.clone())
    }

    pub(super) fn local_provenance(
        &self,
        source: BindingRefV1,
        row: &CompletedLocalBindingV1,
    ) -> Option<LocalProvenanceV1> {
        let entry = self.entries.get(&source)?;
        if entry.value != row.initializer() {
            return None;
        }
        let proof = entry.provenance.clone().unwrap_or(LocalProvenanceV1 {
            purpose: LocalProducerPurposeV1::Initializer,
            root: source,
            root_value: entry.value,
            copies: Vec::new(),
        });
        Self::append_local_copy(proof, row)
    }

    pub(super) fn initializer_provenance(
        binding: BindingRefV1, row: &CompletedLocalBindingV1,
    ) -> Option<LocalProvenanceV1> {
        Self::append_local_copy(LocalProvenanceV1 { purpose: LocalProducerPurposeV1::Initializer, root: binding,
            root_value: row.initializer(), copies: Vec::new() }, row)
    }

    fn append_local_copy(mut proof: LocalProvenanceV1, row: &CompletedLocalBindingV1) -> Option<LocalProvenanceV1> {
        match row.copy() {
            Some(copy) => proof.copies.push(copy.clone()),
            None if row.local() == row.initializer() => {}
            None => return None,
        }
        Some(proof)
    }

    pub(super) fn attach_scalar_join_v1(&mut self, point: Rc<ScalarJoinPointValueV1>) -> Result<(), String> {
        let binding = point.binding();
        if self.get(&binding) != Some(&point.value()) {
            return Err("[freeze:contract][local-read/loop-point-publication]".into());
        }
        self.attach_provenance(binding, Some(LocalProvenanceV1 {
            root: binding, root_value: point.value(), copies: Vec::new(),
            purpose: LocalProducerPurposeV1::ScalarJoin(point),
        }));
        Ok(())
    }

    pub(super) fn attach_checked_rebind(&mut self,
        frame: std::rc::Rc<crate::mir::resolved_semantics::home_new_prefix::CheckedBindingRebindFrameV1>, value: ValueId,
    ) {
        let binding = frame.target();
        self.attach_provenance(binding, Some(LocalProvenanceV1 {
            purpose: LocalProducerPurposeV1::BindingRebind(frame), root: binding, root_value: value, copies: Vec::new(),
        }));
    }

    pub(super) fn attach_provenance(
        &mut self,
        binding: BindingRefV1,
        proof: Option<LocalProvenanceV1>,
    ) {
        if let Some(entry) = self.entries.get_mut(&binding) {
            entry.provenance = proof;
        }
    }

    #[cfg(test)]
    pub(super) fn clear(&mut self) {
        self.entries.clear();
    }
    #[cfg(test)]
    pub(super) fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    #[cfg(test)]
    pub(super) fn remove(&mut self, binding: &BindingRefV1) -> Option<ValueId> {
        self.entries.remove(binding).map(|entry| entry.value)
    }
}

#[cfg(test)]
mod copy_loan_tests {
    use super::*;
    use crate::mir::resolved_semantics::FunctionOwnerIssuerV1;
    use hakorune_mir_core::BindingId;

    fn proof() -> LocalProvenanceV1 {
        let owner = FunctionOwnerIssuerV1::new_for_compilation()
            .unwrap()
            .issue()
            .unwrap();
        LocalProvenanceV1 {
            purpose: LocalProducerPurposeV1::Initializer,
            root: BindingRefV1::new(owner, BindingId::new(1)),
            root_value: ValueId(77),
            copies: vec![
                (
                    BasicBlockId(1),
                    MirInstruction::Copy {
                        dst: ValueId(78),
                        src: ValueId(77),
                    },
                ),
                (
                    BasicBlockId(2),
                    MirInstruction::Copy {
                        dst: ValueId(79),
                        src: ValueId(78),
                    },
                ),
            ],
        }
    }

    #[test]
    fn original_copy_loan_rejects_disconnected_changed_and_duplicate_dependencies() {
        for mutation in 0..6 {
            let mut proof = proof();
            match mutation {
                0 => {
                    proof.copies.remove(0);
                }
                1 => {
                    proof.copies.remove(1);
                }
                2 => {
                    proof.copies[1].1 = MirInstruction::Copy {
                        dst: ValueId(79),
                        src: ValueId(77),
                    };
                }
                3 => {
                    proof.copies[1].1 = MirInstruction::Copy {
                        dst: ValueId(80),
                        src: ValueId(78),
                    };
                }
                4 => {
                    proof.copies.push(proof.copies[1].clone());
                }
                _ => {
                    proof.copies[0].1 = MirInstruction::Return { value: None };
                }
            };
            assert!(proof
                .loan_copies(proof.root, ValueId(77), ValueId(79))
                .is_err());
        }
    }

    #[test]
    fn original_copy_loan_retains_both_blocks_and_refuses_wrong_entry_result() {
        let original = proof();
        assert_eq!(
            original
                .loan_copies(original.root, ValueId(77), ValueId(79))
                .unwrap(),
            original.copies
        );
        assert!(original
            .loan_copies(original.root, ValueId(78), ValueId(79))
            .is_err());
        assert!(original
            .loan_copies(original.root, ValueId(77), ValueId(80))
            .is_err());
        let foreign = proof();
        assert!(original
            .loan_copies(foreign.root, ValueId(77), ValueId(79))
            .is_err());
    }
}

impl CallableSemanticLoweringState {
    pub(in crate::mir::builder) fn record_checked_rebind_v1(&mut self,
        target: &SourceNodeSiteV1,
        frame: std::rc::Rc<crate::mir::resolved_semantics::home_new_prefix::CheckedBindingRebindFrameV1>,
        value: ValueId,
    ) -> Result<(), String> {
        if self.assignments.get(target) != Some(&frame.target()) || frame.source().owner() != self.owner {
            return Err(freeze("checked-rebind/source-target"));
        }
        self.rebind(target, value)?;
        self.values.attach_checked_rebind(frame, value);
        Ok(())
    }
}

#[cfg(test)]
impl LocalProvenanceV1 {
    pub(in crate::mir) fn assert_indexed_phi_loan_for_test(
        source: crate::mir::resolved_semantics::home_new_prefix::CurrentScalarProducerV1,
    ) {
        use crate::mir::builder::emission::phi_lifecycle::PhiTxn;
        use crate::mir::builder::MirBuilder;
        use crate::mir::{MirType, ConstValue};
        crate::runtime::ring0::ensure_global_ring0_initialized();
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("loop_point_loan/0".into());
        let entry = builder.function_state.current_block.unwrap();
        let initial = builder.alloc_typed(MirType::Integer);
        builder.emit_instruction(MirInstruction::Const { dst: initial, value: ConstValue::Integer(0) }).unwrap();
        let header = builder.next_block_id();
        builder.ensure_block_exists(header).unwrap();
        crate::mir::builder::emission::branch::emit_jump(&mut builder, header).unwrap();
        let dst = builder.alloc_typed(MirType::Integer);
        let mut txn = PhiTxn::begin("loop-point-original-phi");
        let token = txn.define_provisional_phi(&mut builder, header, dst, "header").unwrap();
        let function = builder.function_state.current_function.as_ref().unwrap();
        let relation = builder.function_state.local_ssa_correspondence.loan(function, dst, dst).unwrap();
        let point = ScalarJoinPointValueV1::new(source.clone(), Rc::clone(&relation), function).unwrap();
        let ordinary = builder.function_state.local_ssa_correspondence.loan(function, initial, initial).unwrap();
        assert!(ScalarJoinPointValueV1::new(source.clone(), ordinary, function).is_err(), "ordinary definitions cannot become Phi purposes");
        let mut values = MaterializedValuesV1::default();
        values.insert(point.binding(), dst);
        values.attach_scalar_join_v1(Rc::clone(&point)).unwrap();
        let proof = values.provenance(&point.binding()).unwrap();
        assert!(Rc::ptr_eq(proof.loan_scalar_join_v1(&source, dst).unwrap(), &point));
        assert!(proof.loan_initializer_v1(point.binding(), dst).is_err(), "a Header is not its pre-loop initializer");
        assert!(proof.loan_scalar_join_v1(&crate::mir::resolved_semantics::home_new_prefix::CurrentScalarProducerV1::SignedEntry(point.binding()), dst).is_err());
        assert!(relation.require_completed_v1().is_err(), "defined is not completed");
        builder.start_new_block(header).unwrap();
        let copied = builder.alloc_typed(MirType::Integer);
        let copy = MirInstruction::Copy { dst: copied, src: dst };
        builder.emit_instruction(copy.clone()).unwrap();
        let receipt = CompletedLocalBindingV1::new(0, dst, copied).with_copy(Some((header, copy))).unwrap();
        let alias = values.local_provenance(point.binding(), &receipt).unwrap();
        assert!(Rc::ptr_eq(alias.loan_scalar_join_v1(&source, copied).unwrap(), &point));
        assert!(alias.loan_scalar_join_v1(&source, dst).is_err(), "complete Copy chain is mandatory");
        txn.patch_phi_inputs(&mut builder, token, vec![(entry, initial)], "complete-original").unwrap();
        txn.commit(&mut builder).unwrap();
        relation.require_completed_v1().unwrap();
        relation.verify(builder.function_state.current_function.as_ref().unwrap()).unwrap();
        assert!(matches!(relation.identity_phi_binding_v1().unwrap().1, MirInstruction::Phi { ref inputs, .. } if inputs == &vec![(entry, initial)]));
        values.insert(point.binding(), initial);
        assert!(values.provenance(&point.binding()).is_none(), "ordinary rebind invalidates source point purpose");
        let foreign = builder.next_block_id();
        builder.ensure_block_exists(foreign).unwrap();
        let doomed = builder.alloc_typed(MirType::Integer);
        let mut txn = PhiTxn::begin("loop-point-abort");
        txn.define_provisional_phi(&mut builder, foreign, doomed, "pending").unwrap();
        let function = builder.function_state.current_function.as_ref().unwrap();
        let aborted = builder.function_state.local_ssa_correspondence.loan(function, doomed, doomed).unwrap();
        let _point = ScalarJoinPointValueV1::new(source, Rc::clone(&aborted), function).unwrap();
        assert!(txn.abort_on_err(&mut builder, "expected abort".into()).cleanup_failures().is_empty());
        assert!(aborted.identity_phi_binding_v1().is_err(), "a rolled-back observation cannot be used");
    }
}
