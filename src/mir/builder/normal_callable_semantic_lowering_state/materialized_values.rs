//! Original local materialization evidence travels with the existing value slot.
use super::*;
use crate::mir::builder::stmts::CompletedLocalBindingV1;
use crate::mir::{BasicBlockId, MirInstruction};

#[derive(Debug, Clone, PartialEq)]
pub(super) struct LocalProvenanceV1 {
    root: BindingRefV1,
    root_value: ValueId,
    copies: Vec<(BasicBlockId, MirInstruction)>,
}

impl LocalProvenanceV1 {
    pub(super) fn matches(&self, binding: BindingRefV1, value: ValueId) -> bool {
        self.root == binding && self.root_value == value
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
        let mut proof = entry.provenance.clone().unwrap_or(LocalProvenanceV1 {
            root: source,
            root_value: entry.value,
            copies: Vec::new(),
        });
        match row.copy() {
            Some(copy) => proof.copies.push(copy.clone()),
            None if row.local() == row.initializer() => {}
            None => return None,
        }
        Some(proof)
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
