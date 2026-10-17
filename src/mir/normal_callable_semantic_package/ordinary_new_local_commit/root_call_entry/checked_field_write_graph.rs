//! Physical checked FieldSet graph, independent of source operand provenance.
//! This is correspondence only; source packets retain their own admission proof.
use super::*;

#[derive(Debug)]
pub(crate) struct CheckedFieldWriteGraphV1 {
    pins: [Rc<LocalSsaPhysicalRelationV1>; 2],
    invoke: Binding,
    continuation: Binding,
    original: Vec<Binding>,
    completed: OnceCell<Vec<Binding>>,
    fault_bindings: Vec<Binding>,
}
impl CheckedFieldWriteGraphV1 {
    pub(in crate::mir) fn new(pins: [Rc<LocalSsaPhysicalRelationV1>; 2], invoke: Binding,
        continuation: Binding, original: Vec<Binding>, fault_bindings: Vec<Binding>,
    ) -> Self {
        Self { pins, invoke, continuation, original, fault_bindings, completed: OnceCell::new() }
    }
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
    pub(super) fn validate_recorded(&self, rows: &[Binding], finishing: Option<&FinishedBindings>) -> Result<(), String> {
        for original in self.bindings() {
            let mapped = match finishing { Some(p) => p.binding(original.0, &original.1)?, None => Some(original.clone()) };
            if let Some(mapped) = mapped {
                if rows.iter().filter(|row| **row == mapped).count() != 1 { return Err(freeze("borrowed-field-write/recorded-binding")); }
            }
        }
        Ok(())
    }
    pub(super) fn validate_operand(&self, index: usize, value: crate::mir::ValueId,
        function: &MirFunction, finishing: Option<&FinishedBindings>,
    ) -> Result<(), String> {
        if self.pins[index].values().0 != value { return Err(freeze("borrowed-field-write/pin-origin")); }
        self.pins[index].verify_projected(function, &mut |row|
            match finishing { Some(p) => p.relation_binding(function, row), None => Ok(Some(row.clone())) })
    }
    pub(super) fn validate_paths(&self, ledger: &OrdinaryNewClaimLedgerV1,
        function: &MirFunction, finishing: Option<&FinishedBindings>,
        expected_field: hakorune_mir_defs::CanonicalFieldRefV1,
        prior_homes: &[BindingRefV1],
    ) -> Result<(), String> {
        for original in self.bindings() {
            let observed = self.resolve_observed_binding_v1(original, false)?;
            if !super::super::super::physical_boundary::check_binding(function, finishing, observed.0, &observed.1)? {
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
        if *field != expected_field || *base != self.pins[0].values().1 || *value != self.pins[1].values().1
            || Some(*normal_landing) != normal_block || normal_landing == fault_landing
            || continuation.as_ref().is_some_and(|row| !matches!(row.1, MirInstruction::Jump { edge_args: None, .. }))
        { return Err(freeze("borrowed-field-write/invoke-operands")); }
        let operations = ledger.prior_home_unwind_for(prior_homes)?;
        super::super::super::root_cleanup_graph::ordered_paths::pending_operations(function, *fault_landing, &operations, Some(*fault_frame))?;
        let expected = self.fault_bindings.iter().map(map).collect::<Result<Vec<_>, _>>()?;
        super::validate_fault_inventory_v1(function, *fault_landing, *fault_frame, &expected, "borrowed-field-write")
    }
}
