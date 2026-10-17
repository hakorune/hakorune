//! Physical completion of source-owned field readiness, after read take borrows.
use super::*;
use crate::mir::normal_callable_semantic_package::CallValidationContextV1;
use crate::mir::instruction::InvokeOperation;
use crate::mir::verification::utils::compute_dominators;
type Binding = (BasicBlockId, MirInstruction);

impl OrdinaryNewClaimLedgerV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn owned_ready_bindings_for_frame_v1(
        &self, frame: &Rc<CheckedFieldWriteFrameV1>, installed: ValueId, context: CallValidationContextV1<'_>,
    ) -> Result<Vec<(Binding, bool, Option<Binding>)>, String> {
        let mut bindings = Vec::new();
        for row in self.field_reads.borrow().values() {
            let TerminalFieldValueSourceV1::OwnedLocal(source) = &row.value_source else { continue };
            if !Rc::ptr_eq(source.frame(), frame) { continue; }
            let Progress::Emitted(block, instruction) = &row.progress else { return Err(fault("owned-ready/read-unconsumed")); };
            bindings.push(((*block, instruction.clone()), true, None));
        }
        for (site, row) in self.local_field_reads.borrow().iter() {
            if !row.source.owned_ready.as_ref().is_some_and(|source| Rc::ptr_eq(source.frame(), frame)) { continue; }
            match &row.progress {
                LocalReadProgress::Emitted { block, instruction, .. } => bindings.push(((*block, instruction.clone()), true, None)),
                LocalReadProgress::CheckedEmitted { read, .. } => {
                    let [invoke, normal] = self.checked_field_pair_v1(site, context)?
                        .ok_or_else(|| fault("owned-ready/read-unconsumed"))?;
                    let exact = read.as_ref().ok_or_else(|| fault("owned-ready/exact-read-missing"))?;
                    for copy in exact.loan_forwarded_copies(row.receiver, installed)? {
                        bindings.push((copy.clone(), false, Some(invoke.clone())));
                    }
                    for row in self.checked_field_normal_prefix_v1(site)?.into_iter()
                        .chain(self.checked_field_cleanup_bindings_v1(site)?) {
                        bindings.push((row, false, None));
                    }
                    bindings.extend([(invoke, true, None), (normal, false, None)]);
                },
                _ => return Err(fault("owned-ready/read-unconsumed")),
            }
        }
        Ok(bindings)
    }
    pub(in crate::mir::normal_callable_semantic_package) fn validate_owned_ready_reads_v1(
        &self, owner: FunctionOwnerIdV1, function: &MirFunction, context: CallValidationContextV1<'_>,
        mut project: impl FnMut(&Binding) -> Result<Option<Binding>, String>,
    ) -> Result<(), String> {
        context.require_ledger(self)?;
        let mut reads = Vec::new();
        for (site, row) in self.field_reads.borrow().iter().filter(|(site, _)| site.owner() == owner) {
            let TerminalFieldValueSourceV1::OwnedLocal(source) = &row.value_source else { continue };
            let Progress::Emitted(block, instruction) = &row.progress else { return Err(fault("owned-ready/read-unconsumed")); };
            reads.push((site.clone(), source.clone(), (*block, instruction.clone())));
        }
        for (site, row) in self.local_field_reads.borrow().iter().filter(|(site, _)| site.owner() == owner) {
            let Some(source) = &row.source.owned_ready else { continue };
            let read = match &row.progress {
                LocalReadProgress::Emitted { block, instruction, .. } => (*block, instruction.clone()),
                LocalReadProgress::CheckedEmitted { invoke, .. } => invoke.clone(),
                _ => return Err(fault("owned-ready/read-unconsumed")),
            };
            reads.push((site.clone(), source.clone(), read));
        }
        if reads.is_empty() { return Ok(()); }
        for body in function.blocks.values() {
            if body.successors != body.successors_from_terminator().into_iter().collect() {
                return Err(fault("owned-ready/successor-drift"));
            }
        }
        let dom = compute_dominators(function);
        for (site, source, read) in reads {
            let read = project(&read)?.ok_or_else(|| fault("owned-ready/read-removed"))?;
            let body = function.blocks.get(&read.0).ok_or_else(|| fault("owned-ready/read-block"))?;
            if body.all_instructions().filter(|row| **row == read.1).count() != 1 {
                return Err(fault("owned-ready/read-membership"));
            }
            context.with_original_field_write_v1(source.frame(), |packet| {
                let store = project(packet.invoke_binding())?.ok_or_else(|| fault("owned-ready/store-removed"))?;
                let MirInstruction::Invoke { operation: InvokeOperation::FieldSet { field, .. },
                    normal_landing, fault_landing, .. } = store.1 else { return Err(fault("owned-ready/store-shape")); };
                if field != source.field() || normal_landing == fault_landing
                    || !dom.is_reachable(store.0) || !dom.is_reachable(read.0)
                    || !dom.is_reachable(normal_landing) || !dom.dominates(normal_landing, read.0)
                { return Err(fault("owned-ready/store-normal-dominance")); }
                // The source and packet are exact; the finished writer/graph owner
                // validates complete bindings separately with its projection.
                if source.request.site != site { return Err(fault("owned-ready/read-source")); }
                packet.source().validate_source(self)
            })?;
        }
        Ok(())
    }
}
