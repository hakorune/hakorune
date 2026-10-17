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
    graph: CheckedFieldWriteGraphV1,
}
#[path = "checked_field_write_graph.rs"]
mod checked_graph;
use checked_graph::CheckedFieldWriteGraphV1;

impl EmittedBorrowedFieldWriteV1 {
    pub(crate) fn source(&self) -> &BorrowedFieldWriteSourceLoanV1 { &self.source }
    pub(crate) fn invoke_binding(&self) -> &Binding { self.graph.invoke_binding() }
    pub(in crate::mir) fn pins(&self) -> &[Rc<LocalSsaPhysicalRelationV1>; 2] { self.graph.pins() }
    pub(crate) fn bindings(&self) -> &[Binding] { self.graph.bindings() }
    pub(super) fn require_sealed_v1(&self) -> Result<(), String> { self.graph.require_sealed_v1() }
    pub(super) fn resolve_observed_binding_v1(&self, original: &Binding, completed: bool) -> Result<Binding, String> {
        self.graph.resolve_observed_binding_v1(original, completed)
    }
    pub(super) fn seal_bindings_v1(&self) -> Result<(), String> { self.graph.seal_bindings_v1() }
    pub(crate) fn new(source: BorrowedFieldWriteSourceLoanV1, reads: [Rc<ExactLexicalReadV1>; 2],
        pins: [Rc<LocalSsaPhysicalRelationV1>; 2], invoke: Binding, continuation: Binding,
        original: Vec<Binding>, fault_bindings: Vec<Binding>, ledger: &OrdinaryNewClaimLedgerV1, function: &MirFunction,
    ) -> Result<Self, String> {
        let graph = CheckedFieldWriteGraphV1::new(pins, invoke, continuation, original, fault_bindings);
        let packet = Self { source, reads, graph };
        packet.validate_paths(CallValidationContextV1::Lowering(ledger), function, None)?;
        Ok(packet)
    }
    pub(super) fn validate_recorded(&self, rows: &[Binding], finishing: Option<&FinishedBindings>) -> Result<(), String> {
        self.graph.validate_recorded(rows, finishing)
    }
    pub(super) fn validate_paths(&self, context: CallValidationContextV1<'_>, function: &MirFunction,
        finishing: Option<&FinishedBindings>,
    ) -> Result<(), String> {
        let ledger = context.ledger();
        self.source.validate_source(ledger)?;
        let owner = self.source.frame().source().owner();
        for (index, (binding, _, site)) in [self.source.receiver(), self.source.value()].into_iter().enumerate() {
            let value = self.reads[index].value_for(owner, site.site().node(), binding).map_err(|issue| issue.to_string())?;
            self.graph.validate_operand(index, value, function, finishing)?;
        }
        self.graph.validate_paths(ledger, function, finishing, self.source.field_source().field(),
            self.source.frame().prior_homes())
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
