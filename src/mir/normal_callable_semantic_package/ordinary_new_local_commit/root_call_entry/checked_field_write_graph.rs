//! Physical checked FieldSet graph, independent of source operand provenance.
//! This is correspondence only; source packets retain their own admission proof.
use super::*;
use crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1;

#[derive(Debug)]
pub(crate) struct CheckedFieldWriteGraphV1 {
    pins: [Rc<LocalSsaPhysicalRelationV1>; 2],
    operand_relations: Vec<Rc<LocalSsaPhysicalRelationV1>>,
    invoke: Binding,
    continuation: Binding,
    original: Vec<Binding>,
    completed: OnceCell<Vec<Binding>>,
    fault_bindings: Vec<Binding>,
    post_rhs_prefix: Vec<Binding>,
    normal_cleanup: Option<(Binding, Vec<Binding>)>,
}
impl CheckedFieldWriteGraphV1 {
    pub(in crate::mir) fn new(
        pins: [Rc<LocalSsaPhysicalRelationV1>; 2],
        invoke: Binding,
        continuation: Binding,
        original: Vec<Binding>,
        fault_bindings: Vec<Binding>,
        post_rhs_prefix: Vec<Binding>,
        normal_cleanup: Option<(Binding, Vec<Binding>)>,
    ) -> Self {
        Self {
            pins,
            operand_relations: Vec::new(),
            invoke,
            continuation,
            original,
            fault_bindings,
            post_rhs_prefix,
            normal_cleanup,
            completed: OnceCell::new(),
        }
    }
    pub(in crate::mir) fn with_operand_projection(
        mut self,
        prefix: Vec<Binding>,
        relations: Vec<Rc<LocalSsaPhysicalRelationV1>>,
    ) -> Self {
        let mut original = prefix;
        for relation in &relations {
            relation.append_observed_bindings_v1(&mut original);
        }
        for row in self.original {
            if !original.contains(&row) {
                original.push(row);
            }
        }
        let mut unique = Vec::new();
        for row in original {
            if !unique.contains(&row) {
                unique.push(row);
            }
        }
        self.original = unique;
        self.operand_relations = relations;
        self
    }
    pub(super) fn post_rhs_prefix(&self) -> &[Binding] {
        &self.post_rhs_prefix
    }
    pub(crate) fn invoke_binding(&self) -> &Binding {
        &self.invoke
    }
    pub(in crate::mir) fn pins(&self) -> &[Rc<LocalSsaPhysicalRelationV1>; 2] {
        &self.pins
    }
    pub(crate) fn bindings(&self) -> &[Binding] {
        self.completed.get().unwrap_or(&self.original)
    }
    pub(super) fn require_sealed_v1(&self) -> Result<(), String> {
        if self.completed.get().is_none() {
            return Err(freeze("borrowed-field-write/unsealed"));
        }
        Ok(())
    }
    pub(super) fn resolve_observed_binding_v1(
        &self,
        original: &Binding,
        completed: bool,
    ) -> Result<Binding, String> {
        for pin in self.pins.iter().chain(&self.operand_relations) {
            if let Some(row) = pin.resolve_observed_binding_v1(original, completed)? {
                return Ok(row);
            }
        }
        Ok(original.clone())
    }
    pub(super) fn seal_bindings_v1(&self) -> Result<(), String> {
        if self.completed.get().is_some() {
            return Ok(());
        }
        for pin in self.pins.iter().chain(&self.operand_relations) {
            pin.require_completed_v1()?;
        }
        let mut rows = Vec::new();
        for original in &self.original {
            let row = self.resolve_observed_binding_v1(original, true)?;
            if !rows.contains(&row) {
                rows.push(row);
            }
        }
        self.completed
            .set(rows)
            .map_err(|_| freeze("borrowed-field-write/duplicate-seal"))
    }
    pub(super) fn validate_recorded(
        &self,
        rows: &[Binding],
        finishing: Option<&FinishedBindings>,
    ) -> Result<(), String> {
        for original in self.bindings() {
            let mapped = match finishing {
                Some(p) => p.binding(original.0, &original.1)?,
                None => Some(original.clone()),
            };
            if let Some(mapped) = mapped {
                if rows.iter().filter(|row| **row == mapped).count() != 1 {
                    return Err(freeze("borrowed-field-write/recorded-binding"));
                }
            }
        }
        Ok(())
    }
    pub(super) fn validate_operand(
        &self,
        index: usize,
        value: crate::mir::ValueId,
        function: &MirFunction,
        finishing: Option<&FinishedBindings>,
    ) -> Result<(), String> {
        if self.pins[index].values().0 != value {
            return Err(freeze("borrowed-field-write/pin-origin"));
        }
        self.pins[index].verify_projected(function, &mut |row| match finishing {
            Some(p) => p.relation_binding(function, row),
            None => Ok(Some(row.clone())),
        })
    }
    pub(super) fn validate_paths(
        &self,
        ledger: &OrdinaryNewClaimLedgerV1,
        function: &MirFunction,
        finishing: Option<&FinishedBindings>,
        expected_field: hakorune_mir_defs::CanonicalFieldRefV1,
        expected_mode: crate::mir::instruction::FieldSetModeV1,
        prior_homes: &[LiveHomeV1],
        deferred: &[InvokeOperation], finalized: bool,
    ) -> Result<(), String> {
        for original in self.bindings() {
            let observed = self.resolve_observed_binding_v1(original, false)?;
            if !super::super::super::physical_boundary::check_binding(
                function,
                finishing,
                observed.0,
                &observed.1,
            )? {
                return Err(freeze("borrowed-field-write/physical-binding"));
            }
        }
        let map = |original: &Binding| -> Result<Binding, String> {
            match finishing {
                Some(p) => p
                    .binding(original.0, &original.1)?
                    .ok_or_else(|| freeze("borrowed-field-write/mandatory-binding")),
                None => Ok(original.clone()),
            }
        };
        let invoke = map(&self.invoke)?;
        let MirInstruction::Invoke {
            operation:
                InvokeOperation::FieldSet {
                    mode,
                    field,
                    base,
                    value,
                },
            fault_frame,
            normal_landing,
            fault_landing,
        } = &invoke.1
        else {
            return Err(freeze("borrowed-field-write/invoke-shape"));
        };
        let normal_block = finishing.map_or(Some(self.continuation.0), |p| {
            p.destination(self.continuation.0)
        });
        let continuation = match finishing {
            Some(p) => p.binding(self.continuation.0, &self.continuation.1)?,
            None => Some(self.continuation.clone()),
        };
        if *mode != expected_mode
            || *field != expected_field
            || *base != self.pins[0].values().1
            || *value != self.pins[1].values().1
            || Some(*normal_landing) != normal_block
            || normal_landing == fault_landing
            || continuation.as_ref().is_some_and(|row| {
                !matches!(
                    row.1,
                    MirInstruction::Jump {
                        edge_args: None,
                        ..
                    }
                )
            })
        {
            return Err(freeze("borrowed-field-write/invoke-operands"));
        }
        let prior = ledger.prior_home_unwind_for(prior_homes)?;
        if deferred.is_empty() != self.normal_cleanup.is_none()
            || !deferred.is_empty() && !expected_mode.consumes_text_candidate_on_fault_v1() {
            return Err(freeze("borrowed-field-write/invoke-operands"));
        }
        if let Some((end, origins)) = &self.normal_cleanup {
            let stop = match finishing {
                Some(boundary) => {
                    boundary.structural_jump_binding(function, end, finalized)?;
                    boundary.destination(end.0).ok_or_else(|| freeze("borrowed-field-write/mandatory-binding"))?
                }
                None => {
                    if !super::super::super::physical_boundary::check_binding(function, None, end.0, &end.1)? {
                        return Err(freeze("borrowed-field-write/mandatory-binding"));
                    }
                    end.0
                }
            };
            if origins.len() != deferred.len() { return Err(freeze("borrowed-field-write/invoke-operands")); }
            let mut normal = *normal_landing;
            for (index, operation) in deferred.iter().enumerate() {
                use super::super::super::root_cleanup_graph::ordered_paths as paths;
                normal = paths::skip_until(function, normal, Some(stop))?;
                let observed = map(&origins[index])?;
                if observed.0 != normal || !super::super::super::physical_boundary::check_binding(
                    function, None, observed.0, &observed.1)? {
                    return Err(freeze("borrowed-field-write/invoke-operands"));
                }
                let (next, fault) = paths::release(function, normal, operation, Some(*fault_frame))?;
                let residual = deferred[index + 1..].iter().chain(&prior).cloned().collect::<Vec<_>>();
                paths::pending_operations(function, fault, &residual, Some(*fault_frame))?;
                normal = next;
            }
            if super::super::super::root_cleanup_graph::ordered_paths::skip_until(function, normal, Some(stop))? != stop {
                return Err(freeze("borrowed-field-write/invoke-operands"));
            }
        }
        let operations = deferred.iter().chain(&prior).cloned().collect::<Vec<_>>();
        super::super::super::root_cleanup_graph::ordered_paths::pending_operations(
            function,
            *fault_landing,
            &operations,
            Some(*fault_frame),
        )?;
        let mut expected = Vec::new();
        for original in &self.fault_bindings {
            let row = match finishing {
                Some(boundary) if matches!(original.1, MirInstruction::Jump { edge_args: None, .. }) =>
                    boundary.structural_jump_binding(function, original, finalized)?,
                _ => Some(map(original)?),
            };
            if let Some(row) = row { expected.push(row); }
        }
        super::validate_fault_inventory_v1(
            function,
            *fault_landing,
            *fault_frame,
            &expected,
            "borrowed-field-write",
        )
    }
}
