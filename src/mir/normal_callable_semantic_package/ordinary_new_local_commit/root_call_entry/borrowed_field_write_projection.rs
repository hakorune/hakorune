//! Original borrowed field write, its exact pins and checked Normal/Fault graph.
use super::*;
use crate::mir::normal_callable_semantic_package::{BorrowedFieldWriteSourceLoanV1, CallValidationContextV1};
use crate::mir::builder::{ExactLexicalReadV1, LocalSsaPhysicalRelationV1};
use super::super::physical_boundary::FinishedBindings;
use std::{rc::Rc, cell::OnceCell};
type Binding = (BasicBlockId, MirInstruction);

#[derive(Debug)]
pub(crate) struct EmittedBorrowedFieldWriteV1 {
    source: BorrowedFieldWriteSourceLoanV1,
    reads: [Rc<ExactLexicalReadV1>; 2],
    pins: [Rc<LocalSsaPhysicalRelationV1>; 2],
    invoke: Binding,
    continuation: Binding,
    original: Vec<Binding>,
    completed: OnceCell<Vec<Binding>>,
    fault_bindings: Vec<Binding>,
}
impl EmittedBorrowedFieldWriteV1 {
    pub(crate) fn source(&self) -> &BorrowedFieldWriteSourceLoanV1 { &self.source }
    pub(crate) fn invoke_binding(&self) -> &Binding { &self.invoke }
    pub(in crate::mir) fn pins(&self) -> &[Rc<LocalSsaPhysicalRelationV1>; 2] { &self.pins }
    pub(crate) fn bindings(&self) -> &[Binding] { self.completed.get().unwrap_or(&self.original) }
    pub(super) fn require_sealed_v1(&self) -> Result<(), String> {
        if self.completed.get().is_none() { return Err(freeze("borrowed-field-write/unsealed")); }
        Ok(())
    }
    pub(super) fn resolve_observed_binding_v1(&self, original: &Binding, completed: bool) -> Result<Binding, String> {
        for pin in &self.pins {
            if let Some(row) = pin.resolve_observed_binding_v1(original, completed)? { return Ok(row); }
        }
        Ok(original.clone())
    }
    pub(super) fn seal_bindings_v1(&self) -> Result<(), String> {
        if self.completed.get().is_some() { return Ok(()); }
        for pin in &self.pins { pin.require_completed_v1()?; }
        let mut rows = Vec::new();
        for original in &self.original {
            let row = self.resolve_observed_binding_v1(original, true)?;
            if !rows.contains(&row) { rows.push(row); }
        }
        self.completed.set(rows).map_err(|_| freeze("borrowed-field-write/duplicate-seal"))
    }
    pub(crate) fn new(source: BorrowedFieldWriteSourceLoanV1, reads: [Rc<ExactLexicalReadV1>; 2],
        pins: [Rc<LocalSsaPhysicalRelationV1>; 2], invoke: Binding, continuation: Binding,
        original: Vec<Binding>, fault_bindings: Vec<Binding>, ledger: &OrdinaryNewClaimLedgerV1, function: &MirFunction,
    ) -> Result<Self, String> {
        let packet = Self { source, reads, pins, invoke, continuation, original, fault_bindings, completed: OnceCell::new() };
        packet.validate_paths(CallValidationContextV1::Lowering(ledger), function, None)?;
        Ok(packet)
    }
    pub(super) fn validate_recorded(&self, rows: &[Binding], finishing: Option<&FinishedBindings>) -> Result<(), String> {
        for original in self.bindings() {
            let mapped = match finishing { Some(p) => p.binding(original.0, &original.1)?, None => Some(original.clone()) };
            if let Some(mapped) = mapped {
                if rows.iter().filter(|row| **row == mapped).count() != 1 { return Err(freeze("borrowed-field-write/recorded-binding")); }
            }
        }
        Ok(())
    }
    pub(super) fn validate_paths(&self, context: CallValidationContextV1<'_>, function: &MirFunction,
        finishing: Option<&FinishedBindings>,
    ) -> Result<(), String> {
        let ledger = context.ledger();
        self.source.validate_source(ledger)?;
        let owner = self.source.frame().source().owner();
        for (index, (binding, _, site)) in [self.source.receiver(), self.source.value()].into_iter().enumerate() {
            let value = self.reads[index].value_for(owner, site.site().node(), binding).map_err(|issue| issue.to_string())?;
            if self.pins[index].values().0 != value { return Err(freeze("borrowed-field-write/pin-origin")); }
            self.pins[index].verify_projected(function, &mut |row|
                match finishing { Some(p) => p.relation_binding(function, row), None => Ok(Some(row.clone())) })?;
        }
        for original in self.bindings() {
            let observed = self.resolve_observed_binding_v1(original, false)?;
            if !super::super::physical_boundary::check_binding(function, finishing, observed.0, &observed.1)? {
                return Err(freeze("borrowed-field-write/physical-binding"));
            }
        }
        let map = |original: &Binding| -> Result<Binding, String> {
            match finishing { Some(p) => p.binding(original.0, &original.1)?.ok_or_else(|| freeze("borrowed-field-write/mandatory-binding")),
                None => Ok(original.clone()) }
        };
        let invoke = map(&self.invoke)?;
        let MirInstruction::Invoke { operation: InvokeOperation::FieldSet { field, base, value },
            fault_frame, normal_landing, fault_landing } = &invoke.1 else { return Err(freeze("borrowed-field-write/invoke-shape")); };
        let normal_block = finishing.map_or(Some(self.continuation.0), |p| p.destination(self.continuation.0));
        let continuation = match finishing { Some(p) => p.binding(self.continuation.0, &self.continuation.1)?, None => Some(self.continuation.clone()) };
        if *field != self.source.field_source().field() || *base != self.pins[0].values().1 || *value != self.pins[1].values().1
            || Some(*normal_landing) != normal_block || normal_landing == fault_landing
            || continuation.as_ref().is_some_and(|row| !matches!(row.1, MirInstruction::Jump { edge_args: None, .. }))
        { return Err(freeze("borrowed-field-write/invoke-operands")); }
        let operations = ledger.prior_home_unwind_for(self.source.frame().prior_homes())?;
        super::super::root_cleanup_graph::ordered_paths::pending_operations(function, *fault_landing, &operations, Some(*fault_frame))?;
        let expected = self.fault_bindings.iter().map(map).collect::<Result<Vec<_>, _>>()?;
        validate_fault_inventory_v1(function, *fault_landing, *fault_frame, &expected, "borrowed-field-write")
    }
}
pub(super) fn validate_fault_inventory_v1(function: &MirFunction, landing: BasicBlockId,
    frame: crate::mir::ValueId, expected: &[Binding], family: &str,
) -> Result<(), String> {
        let mut pending = vec![landing];
        let mut seen = std::collections::BTreeSet::new();
        while let Some(block) = pending.pop() {
            if !seen.insert(block) { continue; }
            let body = function.blocks.get(&block).ok_or_else(|| freeze(&format!("{family}/fault-block")))?;
            for row in body.all_instructions() {
                if !matches!(row, MirInstruction::Invoke { .. } | MirInstruction::Jump { .. } | MirInstruction::ReturnFault { .. }) {
                    return Err(freeze(&format!("{family}/fault-unexpected-effect")));
                }
                if expected.iter().filter(|binding| binding.0 == block && &binding.1 == row).count() != 1 {
                    return Err(freeze(&format!("{family}/fault-extra-instruction")));
                }
            }
            let terminal = body.terminator.as_ref().ok_or_else(|| freeze(&format!("{family}/fault-terminal")))?;
            match terminal {
                MirInstruction::Invoke { normal_landing, fault_landing, .. } => { pending.push(*normal_landing); pending.push(*fault_landing); }
                MirInstruction::Jump { target, .. } => pending.push(*target),
                MirInstruction::ReturnFault { fault_frame: actual } if actual == &frame => {}
                _ => return Err(freeze(&format!("{family}/fault-escape"))),
            }
        }
    Ok(())
}
impl OrdinaryNewClaimLedgerV1 {
    pub(crate) fn record_borrowed_field_write_bindings_v1(&self, packet: Rc<EmittedBorrowedFieldWriteV1>) -> Result<(), String> {
        let source = packet.source.frame().source();
        self.prepare_local_call_binding_group(source.owner(),
            RootLifecyclePrefixBindingGroupV1::new_borrowed_field_write(packet)?)?.commit()
    }
    pub(in crate::mir) fn original_borrowed_field_write_bindings_v1(&self, owner: FunctionOwnerIdV1) -> Result<Vec<Binding>, String> {
        let groups = self.root_local_call_bindings.borrow();
        let mut rows = Vec::new();
        for group in groups.get(&owner).into_iter().flatten() {
            let Some(packet) = group.borrowed_field_write() else { continue };
            packet.require_sealed_v1()?;
            packet.source.validate_source(self)?;
            if packet.source.frame().source().owner() != owner || group.site() != packet.source.value().2 {
                return Err(freeze("borrowed-field-write/retention-source"));
            }
            for row in packet.bindings() { if !rows.contains(row) { rows.push(row.clone()); } }
        }
        Ok(rows)
    }
}
