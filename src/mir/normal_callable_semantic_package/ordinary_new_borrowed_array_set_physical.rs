//! Source-occurrence to sole Set writer correspondence in the existing entry.
use super::*;
use crate::mir::builder::{CompletedArrayElementWriteV1, LocalSsaPhysicalRelationV1};
use crate::mir::normal_callable_semantic_package::BorrowedArraySetSourceLoanV1;
use crate::mir::{ArrayElementWriteKind, ArrayWriteProducerKind, BasicBlockId, MirFunction, MirInstruction};
type Binding = (BasicBlockId, MirInstruction);

#[derive(Debug)]
pub(super) struct ArraySetObservationV1 {
    source: BorrowedArraySetSourceLoanV1,
    write: CompletedArrayElementWriteV1,
    reads: [Binding; 2],
    operands: [Rc<LocalSsaPhysicalRelationV1>; 3],
}
impl OrdinaryNewClaimLedgerV1 {
    pub(in crate::mir) fn record_borrowed_array_set_v1(
        &self, source: BorrowedArraySetSourceLoanV1, write: CompletedArrayElementWriteV1,
        operands: [Rc<LocalSsaPhysicalRelationV1>; 3], function: &MirFunction,
    ) -> Result<(), String> {
        source.validate_source(self)?;
        let reads = source.original_reads(self)?;
        let MirInstruction::ArrayElementWrite { site_id, kind: ArrayElementWriteKind::Set,
            producer: ArrayWriteProducerKind::MethodCall, receiver, index: Some(index), value, .. } = &write.original().1
            else { return Err(freeze("borrowed-set/write-shape")) };
        if *site_id != write.site() || [*receiver, *index, *value] != operands.each_ref().map(|row| row.values().1)
            || reads[0].1.dst_value() != Some(operands[0].values().0)
            || reads[1].1.dst_value() != Some(operands[1].values().0)
            || !matches!(reads[1].1, MirInstruction::ObjectFieldGet { field, .. } if field == source.index_field())
        { return Err(freeze("borrowed-set/operand-correspondence")); }
        for operand in &operands { operand.verify(function)?; }
        let mut entries = self.borrowed_entry_values.borrow_mut();
        let entry = entries.get_mut(&source.call().owner()).ok_or_else(|| freeze("borrowed-set/entry-missing"))?;
        let expected = if source.value_binding() == source.value_formal() {
            entry.values.as_ref().map_err(Clone::clone)?.iter()
                .find(|(_, binding, _)| *binding == source.value_formal()).map(|(_, _, value)| *value)
        } else {
            entry.aliases.get(&source.value_binding()).filter(|alias| alias.formal() == source.value_formal())
                .map(|alias| alias.value())
        }.ok_or_else(|| freeze("borrowed-set/value-binding"))?;
        if expected != operands[2].values().0 { return Err(freeze("borrowed-set/value-child")); }
        if entry.array_sets.contains_key(source.call()) { return Err(freeze("borrowed-set/duplicate-write")); }
        entry.array_sets.insert(source.call().clone(), ArraySetObservationV1 { source, write, reads, operands });
        Ok(())
    }

    pub(in crate::mir) fn original_borrowed_array_set_bindings_v1(
        &self, owner: FunctionOwnerIdV1,
    ) -> Result<Vec<Binding>, String> {
        let entries = self.borrowed_entry_values.borrow();
        let Some(entry) = entries.get(&owner) else { return Ok(Vec::new()) };
        let source = self.finalized_borrowed_ordinary_entry_source_v1(owner)?;
        use super::super::super::borrowed_formal_uses::BorrowedFormalUseDraftKindV1 as Use;
        let expected: BTreeSet<_> = source.definition.uses.iter().filter_map(|row| match &row.kind {
            Use::ArrayElementFieldIndexValue { call, .. } => Some(call.clone()), _ => None,
        }).collect();
        if entry.array_sets.keys().cloned().collect::<BTreeSet<_>>() != expected {
            return Err(freeze("borrowed-set/source-coverage"));
        }
        let mut bindings = Vec::new();
        for record in entry.array_sets.values() {
            record.source.validate_source(self)?;
            let mut original = vec![record.write.original().clone()];
            original.extend(record.reads.iter().cloned());
            for operand in &record.operands {
                operand.require_completed_v1()?;
                operand.append_observed_bindings_v1(&mut original);
            }
            for row in original {
                let mut resolved = None;
                for operand in &record.operands {
                    if let Some(binding) = operand.resolve_observed_binding_v1(&row, true)? {
                        if resolved.as_ref().is_some_and(|prior| prior != &binding) {
                            return Err(freeze("borrowed-set/ambiguous-completion"));
                        }
                        resolved = Some(binding);
                    }
                }
                let row = resolved.unwrap_or(row);
                if !bindings.contains(&row) { bindings.push(row); }
            }
        }
        Ok(bindings)
    }

    pub(in crate::mir::normal_callable_semantic_package) fn with_borrowed_array_set_originals_v1(
        &self, owner: FunctionOwnerIdV1,
        mut visit: impl FnMut(&BorrowedArraySetSourceLoanV1, &Binding, &[Binding; 2],
            &[Rc<LocalSsaPhysicalRelationV1>; 3]) -> Result<(), String>,
    ) -> Result<(), String> {
        self.original_borrowed_array_set_bindings_v1(owner)?;
        let entries = self.borrowed_entry_values.borrow();
        if let Some(entry) = entries.get(&owner) {
            for record in entry.array_sets.values() {
                visit(&record.source, record.write.original(), &record.reads, &record.operands)?;
            }
        }
        Ok(())
    }
}
