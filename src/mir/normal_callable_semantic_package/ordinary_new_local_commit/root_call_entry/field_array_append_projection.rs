//! One original field-Array Push, its exact materialization and outward Fault.
use super::*;
use crate::mir::normal_callable_semantic_package::{CallValidationContextV1, FieldArrayAppendSourceLoanV1};
use crate::mir::builder::{ExactLexicalReadV1, ObservedSourceValueV1};
use crate::mir::builder::LocalSsaPhysicalRelationV1;
use super::super::physical_boundary::FinishedBindings;
use std::rc::Rc;
use std::cell::OnceCell;
type Binding = (BasicBlockId, MirInstruction);

#[derive(Debug)]
pub(crate) struct EmittedFieldArrayAppendV1 {
    source: FieldArrayAppendSourceLoanV1,
    receiver: Rc<ExactLexicalReadV1>,
    argument: ObservedSourceValueV1,
    pins: [Rc<LocalSsaPhysicalRelationV1>; 2],
    invoke: Binding,
    continuation: Binding,
    original: Vec<Binding>,
    completed: OnceCell<Vec<Binding>>,
    fault_bindings: Vec<Binding>,
}
impl EmittedFieldArrayAppendV1 {
    pub(crate) fn source(&self) -> &FieldArrayAppendSourceLoanV1 { &self.source }
    pub(crate) fn invoke_binding(&self) -> &Binding { &self.invoke }
    pub(crate) fn bindings(&self) -> &[Binding] { self.completed.get().unwrap_or(&self.original) }
    pub(super) fn require_sealed_v1(&self) -> Result<(), String> {
        if self.completed.get().is_none() { return Err(freeze("field-array-append/unsealed")); }
        Ok(())
    }
    /// Freeze the inventory from the same writer loans after function lowering,
    /// before PhysicalBoundary capture; never reconstruct it from final MIR.
    pub(super) fn seal_bindings_v1(&self) -> Result<(), String> {
        if self.completed.get().is_some() { return Ok(()); }
        self.argument.require_completed_v1()?;
        for pin in &self.pins { pin.require_completed_v1()?; }
        let mut bindings = Vec::new();
        for original in &self.original {
            let row = self.resolve_observed_binding_v1(original, true)?;
            if !bindings.contains(&row) { bindings.push(row); }
        }
        self.completed.set(bindings).map_err(|_| freeze("field-array-append/duplicate-seal"))
    }
    pub(super) fn resolve_observed_binding_v1(&self, original: &Binding, require_completed: bool) -> Result<Binding, String> {
        let mut resolved = self.argument.resolve_binding_v1(original, require_completed)?;
        for pin in &self.pins {
            if resolved.is_none() { resolved = pin.resolve_observed_binding_v1(original, require_completed)?; }
        }
        Ok(resolved.unwrap_or_else(|| original.clone()))
    }
    pub(crate) fn borrows_receiver_read_v1(&self, original: &Rc<ExactLexicalReadV1>) -> bool { Rc::ptr_eq(&self.receiver, original) }
    pub(crate) fn new(source: FieldArrayAppendSourceLoanV1, receiver: Rc<ExactLexicalReadV1>,
        argument: ObservedSourceValueV1, pins: [Rc<LocalSsaPhysicalRelationV1>; 2],
        invoke: Binding, continuation: Binding, original: Vec<Binding>, fault_bindings: Vec<Binding>,
        ledger: &OrdinaryNewClaimLedgerV1, function: &MirFunction,
    ) -> Result<Self, String> {
        let packet = Self { source, receiver, argument, pins, invoke, continuation, original, completed: OnceCell::new(), fault_bindings };
        packet.validate_paths(CallValidationContextV1::Lowering(ledger), function, None)?;
        Ok(packet)
    }
    pub(super) fn validate_recorded(&self, bindings: &[Binding], finishing: Option<&FinishedBindings>) -> Result<(), String> {
        for original in self.bindings() {
            let mapped = match finishing { Some(p) => p.binding(original.0, &original.1)?, None => Some(original.clone()) };
            if let Some(mapped) = mapped {
                if bindings.iter().filter(|row| **row == mapped).count() != 1 { return Err(freeze("field-array-append/recorded-binding")); }
            }
        }
        Ok(())
    }
    pub(super) fn validate_paths(&self, context: CallValidationContextV1<'_>, function: &MirFunction,
        finishing: Option<&FinishedBindings>,
    ) -> Result<(), String> {
        let ledger = context.ledger();
        self.source.validate_source(ledger)?;
        self.argument.validate_source(self.source.site().owner(), self.source.integer_argument()?)?;
        let receiver = self.receiver.value_for(self.source.site().owner(), self.source.receiver_site().node(), self.source.integer_requirement()?.binding())
            .map_err(|issue| issue.to_string())?;
        if self.pins[0].values().0 != receiver || self.pins[1].values().0 != self.argument.root_value() {
            return Err(freeze("field-array-append/pin-origin"));
        }
        let mut project_relation = |original: &Binding| -> Result<Option<Binding>, String> {
            match finishing { Some(p) => p.relation_binding(function, original), None => Ok(Some(original.clone())) }
        };
        self.argument.validate(function, &mut project_relation).map_err(|issue| format!("{issue} field-array-append=argument finishing={}", finishing.is_some()))?;
        for pin in &self.pins {
            pin.verify_projected(function, &mut project_relation).map_err(|issue| format!("{issue} field-array-append=pin {:?} finishing={}", pin.values(), finishing.is_some()))?;
        }
        let map = |original: &Binding| -> Result<Binding, String> {
            match finishing {
                Some(p) => p.binding(original.0, &original.1)?.ok_or_else(|| freeze("field-array-append/mandatory-binding")),
                None => Ok(original.clone()),
            }
        };
        for original in self.bindings() {
            // Lowering may revisit the same packet after a Phi patch but before
            // the function-wide seal. Borrow only its own writer observation.
            let observed = self.resolve_observed_binding_v1(original, false)?;
            if !super::super::physical_boundary::check_binding(function, finishing, observed.0, &observed.1)? {
                return Err(format!("{} function={} finishing={} original={original:?} actual={:?}",
                    freeze("field-array-append/physical-binding"), function.signature.name, finishing.is_some(), function.blocks.get(&original.0)));
            }
        }
        let invoke = map(&self.invoke)?;
        let MirInstruction::Invoke { operation: InvokeOperation::ArrayElementWrite {
            kind: crate::mir::ArrayElementWriteKind::Push, producer: crate::mir::ArrayWriteProducerKind::MethodCall,
            receiver, index: None, value, .. }, fault_frame, normal_landing, fault_landing } = &invoke.1
            else { return Err(freeze("field-array-append/invoke-shape")); };
        let continuation = match finishing {
            Some(p) => p.binding(self.continuation.0, &self.continuation.1)?,
            None => Some(self.continuation.clone()),
        };
        let normal_block = finishing.map_or(Some(self.continuation.0), |p| p.destination(self.continuation.0));
        if *receiver != self.pins[0].values().1 || value.scalar() != Some(self.pins[1].values().1)
            || Some(*normal_landing) != normal_block || normal_landing == fault_landing
            || continuation.as_ref().is_some_and(|row| !matches!(row.1, MirInstruction::Jump { edge_args: None, .. })) {
            return Err(freeze("field-array-append/invoke-operands"));
        }
        let operations = ledger.field_array_append_unwind_v1(&self.source)?;
        super::super::root_cleanup_graph::ordered_paths::pending_operations(function, *fault_landing, &operations, Some(*fault_frame))?;
        // Every instruction on the Fault graph belongs to the actual writer
        // packet, not just its terminators. No extra effect may hide in a block.
        let expected: Vec<_> = self.fault_bindings.iter().map(map).collect::<Result<_, _>>()?;
        super::borrowed_field_write_projection::validate_fault_inventory_v1(
            function, *fault_landing, *fault_frame, &expected, "field-array-append")?;
        Ok(())
    }
    /// Marker linkage is separate from the mandatory source/finishing checks
    /// above. A bare marker never authorizes this checked operation.
    pub(crate) fn validate_marker(&self, function: &MirFunction, marker: &crate::mir::named_array_obligation::NamedArrayWriteMarkerV1) -> Result<(), String> {
        let MirInstruction::Invoke { operation: original @ InvokeOperation::ArrayElementWrite { site_id, receiver, value, .. }, .. } = &self.invoke.1
            else { return Err(freeze("field-array-append/marker-source")); };
        if marker.owner != self.source.site().owner() || marker.write != *site_id || marker.receiver != *receiver || &marker.argument != value {
            return Err(freeze("field-array-append/marker-drift"));
        }
        let mut found = 0;
        for row in function.blocks.values().flat_map(|body| body.all_instructions()) {
            match row {
                MirInstruction::Invoke { operation: actual @ InvokeOperation::ArrayElementWrite { site_id: current, .. }, .. } if current == site_id => {
                    if actual != original { return Err(freeze("field-array-append/marker-operation")); } found += 1;
                }
                MirInstruction::ArrayElementWrite { site_id: current, .. } if current == site_id => return Err(freeze("field-array-append/bare-write")),
                _ => {}
            }
        }
        if found != 1 { return Err(freeze("field-array-append/marker-cardinality")); }
        Ok(())
    }
}
impl OrdinaryNewClaimLedgerV1 {
    /// Borrow the same sealed source inventory for Phi repair and required
    /// liveness. Physical instructions cannot select a source or grant a Push.
    pub(in crate::mir) fn original_field_array_append_bindings_v1(
        &self, owner: FunctionOwnerIdV1,
    ) -> Result<Vec<Binding>, String> {
        let groups = self.root_local_call_bindings.borrow();
        let mut bindings = Vec::new();
        for group in groups.get(&owner).into_iter().flatten() {
            let Some(packet) = group.field_array_append() else { continue };
            packet.require_sealed_v1()?;
            packet.source.validate_source(self)?;
            if packet.source.site().owner() != owner || group.site() != packet.source.site() {
                return Err(freeze("field-array-append/retention-source-drift"));
            }
            for row in packet.bindings() {
                if !bindings.contains(row) { bindings.push(row.clone()); }
            }
        }
        Ok(bindings)
    }

    pub(crate) fn field_array_append_unwind_v1(&self, source: &FieldArrayAppendSourceLoanV1) -> Result<Vec<InvokeOperation>, String> {
        source.validate_source(self)?;
        let rows = self.local_commits.borrow();
        let plans = super::source_home_end_plans_v1(&rows, source.prior_homes())?
            .ok_or_else(|| freeze("field-array-append/end-unavailable"))?;
        Ok(plans.into_iter().map(|(_, _, operation)| operation).collect())
    }
    pub(crate) fn record_field_array_append_bindings_v1(&self, packet: Rc<EmittedFieldArrayAppendV1>) -> Result<(), String> {
        self.prepare_local_call_binding_group(packet.source.site().owner(),
            RootLifecyclePrefixBindingGroupV1::new_field_append(Rc::clone(&packet), packet.bindings().to_vec())?)?.commit()
    }
}
