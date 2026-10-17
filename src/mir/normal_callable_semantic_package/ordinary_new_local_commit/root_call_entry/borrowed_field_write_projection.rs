//! Original borrowed field write, its exact pins and checked Normal/Fault graph.
use super::*;
use crate::mir::normal_callable_semantic_package::{FieldWriteSourceLoanV1, CallValidationContextV1};
use crate::mir::builder::{ExactLexicalReadV1, LocalSsaPhysicalRelationV1};
use super::super::physical_boundary::FinishedBindings;
use std::{rc::Rc, cell::OnceCell};
type Binding = (BasicBlockId, MirInstruction);

#[derive(Debug)]
pub(crate) struct EmittedBorrowedFieldWriteV1 {
    source: FieldWriteSourceLoanV1,
    receiver: Rc<ExactLexicalReadV1>,
    value: FieldWriteValueV1,
    graph: CheckedFieldWriteGraphV1,
}
#[path = "checked_field_write_graph.rs"]
mod checked_graph;
#[path = "field_write_rhs_projection.rs"]
mod rhs;
pub(in crate::mir) use rhs::{EmittedFieldWriteRhsV1, FieldWriteValueV1, FieldWriteRhsNodeV1, FieldWriteRhsProofV1};
pub(crate) use checked_graph::CheckedFieldWriteGraphV1;

impl EmittedBorrowedFieldWriteV1 {
    pub(crate) fn source(&self) -> &FieldWriteSourceLoanV1 { &self.source }
    pub(crate) fn invoke_binding(&self) -> &Binding { self.graph.invoke_binding() }
    pub(in crate::mir) fn pins(&self) -> &[Rc<LocalSsaPhysicalRelationV1>; 2] { self.graph.pins() }
    pub(crate) fn bindings(&self) -> &[Binding] { self.graph.bindings() }
    pub(in crate::mir::normal_callable_semantic_package) fn owned_operand_bindings_v1(
        &self, root: BindingRefV1, installed: crate::mir::ValueId,
    ) -> Result<Vec<Binding>, String> {
        let mut rows = self.receiver.loan_forwarded_copies(root, installed)?.to_vec();
        if let FieldWriteValueV1::Expression(rhs) = &self.value { rows.extend(rhs.bindings()); }
        for pin in self.graph.pins() { pin.append_observed_bindings_v1(&mut rows); }
        rows.into_iter().map(|row| self.graph.resolve_observed_binding_v1(&row, true)).collect()
    }
    pub(in crate::mir::normal_callable_semantic_package) fn owned_handle_values_v1(
        &self, root: BindingRefV1, installed: crate::mir::ValueId, result: crate::mir::ValueId,
    ) -> Result<std::collections::BTreeSet<crate::mir::ValueId>, String> {
        let pin = self.graph.pins()[0].values();
        let mut values = std::collections::BTreeSet::from([installed, result, pin.0, pin.1]);
        for (_, row) in self.receiver.loan_forwarded_copies(root, installed)? {
            if let MirInstruction::Copy { dst, src } = row { values.extend([*dst, *src]); }
        }
        Ok(values)
    }
    pub(super) fn require_sealed_v1(&self) -> Result<(), String> { self.graph.require_sealed_v1() }
    pub(super) fn resolve_observed_binding_v1(&self, original: &Binding, completed: bool) -> Result<Binding, String> {
        self.graph.resolve_observed_binding_v1(original, completed)
    }
    pub(super) fn seal_bindings_v1(&self) -> Result<(), String> { self.graph.seal_bindings_v1() }
    pub(in crate::mir) fn new(source: FieldWriteSourceLoanV1, receiver: Rc<ExactLexicalReadV1>, value: FieldWriteValueV1,
        graph: CheckedFieldWriteGraphV1, ledger: &OrdinaryNewClaimLedgerV1, function: &MirFunction,
    ) -> Result<Self, String> {
        let packet = Self { source, receiver, value, graph };
        packet.validate_paths(CallValidationContextV1::Lowering(ledger), function, None)?;
        Ok(packet)
    }
    pub(super) fn validate_recorded(&self, rows: &[Binding], finishing: Option<&FinishedBindings>) -> Result<(), String> {
        self.graph.validate_recorded(rows, finishing)
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn validate_paths(&self, context: CallValidationContextV1<'_>, function: &MirFunction,
        finishing: Option<&FinishedBindings>,
    ) -> Result<(), String> {
        let ledger = context.ledger();
        self.source.validate_source(ledger)?;
        let (receiver, site) = self.source.receiver()?;
        let base = self.receiver.value_for(site.owner(), site.site().node(), receiver).map_err(|issue| issue.to_string())?;
        self.graph.validate_operand(0, base, function, finishing)?;
        let (value, field) = match (&self.value, self.source.borrowed()) {
            (FieldWriteValueV1::Lexical(read), Some(formal)) => {
                let (binding, _, site) = formal.value();
                (read.value_for(site.owner(), site.site().node(), binding).map_err(|issue| issue.to_string())?, formal.field_source().field())
            },
            (FieldWriteValueV1::Expression(rhs), None) => {
                if self.source.owned_rhs().is_none_or(|original| !Rc::ptr_eq(original, rhs.recipe())) {
                    return Err(freeze("field-write-rhs/foreign-recipe"));
                }
                rhs.validate(context, function, finishing)?;
                (rhs.value(), self.source.frame().owned_local().ok_or_else(|| freeze("field-write-rhs/local-origin"))?.field())
            },
            _ => return Err(freeze("field-write-rhs/operand-origin")),
        };
        self.graph.validate_operand(1, value, function, finishing)?;
        self.graph.validate_paths(ledger, function, finishing, field, self.source.frame().prior_homes())
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
    pub(in crate::mir::normal_callable_semantic_package) fn validate_source_field_fault_v1(
        &self, function: &MirFunction, invoke: &Binding, rows: &[Binding], homes: &[BindingRefV1],
    ) -> Result<(), String> {
        let MirInstruction::Invoke { operation: crate::mir::instruction::InvokeOperation::FieldGet { .. },
            fault_landing, fault_frame, .. } = &invoke.1 else { return Err(freeze("field-read/fault-invoke")); };
        let operations = self.prior_home_unwind_for(homes)?;
        super::super::super::root_cleanup_graph::ordered_paths::pending_operations(
            function, *fault_landing, &operations, Some(*fault_frame))?;
        validate_fault_inventory_v1(function, *fault_landing, *fault_frame, rows, "field-read")
    }
    pub(crate) fn record_borrowed_field_write_bindings_v1(&self, packet: Rc<EmittedBorrowedFieldWriteV1>) -> Result<(), String> {
        let source = packet.source.frame().source();
        self.prepare_local_call_binding_group(source.owner(),
            RootLifecyclePrefixBindingGroupV1::new_borrowed_field_write(packet)?)?.commit()
    }
    pub(in crate::mir) fn original_borrowed_field_write_bindings_v1(&self, owner: FunctionOwnerIdV1, completed: bool) -> Result<Vec<Binding>, String> {
        let groups = self.root_local_call_bindings.borrow();
        let mut rows = Vec::new();
        for group in groups.get(&owner).into_iter().flatten() {
            let Some(packet) = group.borrowed_field_write() else { continue };
            if completed { packet.require_sealed_v1()?; }
            packet.source.validate_source(self)?;
            if packet.source.frame().source().owner() != owner || group.site() != &packet.source.value_site() {
                return Err(freeze("borrowed-field-write/retention-source"));
            }
            for original in packet.bindings() {
                let row = packet.resolve_observed_binding_v1(original, completed)?;
                if !rows.contains(&row) { rows.push(row); }
            }
        }
        Ok(rows)
    }
}
