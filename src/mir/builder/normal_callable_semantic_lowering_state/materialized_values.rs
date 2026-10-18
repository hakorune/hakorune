//! Original local materialization evidence travels with the existing value slot.
use super::*;
use crate::mir::builder::stmts::CompletedLocalBindingV1;
use crate::mir::{BasicBlockId, MirInstruction};

#[derive(Debug, Clone)]
enum LocalProducerPurposeV1 {
    Initializer,
    BindingRebind(std::rc::Rc<crate::mir::resolved_semantics::home_new_prefix::CheckedBindingRebindFrameV1>),
}
impl PartialEq for LocalProducerPurposeV1 {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Initializer, Self::Initializer) => true,
            (Self::BindingRebind(a), Self::BindingRebind(b)) => std::rc::Rc::ptr_eq(a, b),
            _ => false,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub(in crate::mir) struct LocalProvenanceV1 {
    purpose: LocalProducerPurposeV1,
    root: BindingRefV1,
    root_value: ValueId,
    copies: Vec<(BasicBlockId, MirInstruction)>,
}

impl LocalProvenanceV1 {
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
