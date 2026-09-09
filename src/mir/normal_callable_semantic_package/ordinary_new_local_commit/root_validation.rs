//! Final-root validation is separate from local New emission state changes.

use super::*;
use std::collections::BTreeSet;

impl OrdinaryNewClaimLedgerV1 {
    /// Called on the exact physical root after all module finalization passes.
    /// Script-only packages never register a callable root; an empty New set
    /// does not erase a registered root's validation obligation.
    pub(crate) fn validate_finalized_new_root(
        &self,
        function: &MirFunction,
    ) -> Result<RootOrdinaryNewObservation, String> {
        let mut state = self.root_validation.borrow_mut();
        let owner = match *state {
            RootNewValidation::Unregistered => return Ok(RootOrdinaryNewObservation::NotIssued),
            RootNewValidation::Pending(owner) => owner,
            RootNewValidation::Checked(..) | RootNewValidation::FinishingChecked => {
                return Err(freeze("duplicate-root-validation"));
            }
        };
        self.validate_root_body(owner, function, None)?;
        let observation = self.finalized_root_observation(owner);
        self.validate_root_cleanup_shape(owner, function)?;
        let bindings = self.lifecycle_bindings(owner)?;
        let boundary = super::physical_boundary::PhysicalBoundary::capture(function, &bindings)?;
        *state = RootNewValidation::Checked(owner, boundary);
        Ok(observation)
    }

    /// Recheck every selected ordinary child against the same physical
    /// boundary captured at draft finalization. The function symbol is only a
    /// physical draft locator; source ownership remains the owner-indexed
    /// ledger row.
    pub(crate) fn validate_finalized_child_functions(
        &self,
        module: &crate::mir::MirModule,
        artifact: bool,
    ) -> Result<BTreeSet<String>, String> {
        let owners: Vec<_> = self
            .child_physical_validation
            .borrow()
            .keys()
            .copied()
            .collect();
        let mut covered = BTreeSet::new();
        for owner in owners {
            let mut states = self.child_physical_validation.borrow_mut();
            let state = states
                .get_mut(&owner)
                .ok_or_else(|| freeze("child-physical-state-missing"))?;
            let ChildPhysicalValidation::Checked { symbol, boundary } = state else {
                return Err(freeze("duplicate-child-finishing-validation"));
            };
            let function = module
                .functions
                .get(symbol)
                .ok_or_else(|| freeze("child-definition-missing"))?;
            if function.signature.name != *symbol || !covered.insert(symbol.clone()) {
                return Err(freeze("child-definition-symbol-drift"));
            }
            let bindings = self.lifecycle_bindings(owner)?;
            let mut projection = boundary.project(function)?;
            self.validate_new_emissions_projected(owner, function, Some(&projection))?;
            self.validate_field_reads(owner, function)?;
            self.validate_terminal_integer_literal_return(owner, function)?;
            self.validate_terminal_i64_field_return(owner, function)?;
            self.validate_root_home_exit(owner, function, Some(&projection))?;
            boundary.validate_complete(function, &mut projection, &bindings)?;
            if artifact {
                self.validate_artifact_lifecycle_coverage(
                    owner,
                    function,
                    projection.recorded(),
                )?;
            }
            *state = ChildPhysicalValidation::FinishingChecked;
        }
        Ok(covered)
    }

    fn validate_root_body(
        &self,
        owner: FunctionOwnerIdV1,
        function: &MirFunction,
        projection: Option<&super::physical_boundary::FinishedBindings>,
    ) -> Result<(), String> {
        self.validate_new_emissions_projected(owner, function, projection)?;
        self.validate_terminal_unit_return(owner, function)?;
        self.validate_root_home_exit(owner, function, projection)?;
        self.validate_field_reads(owner, function)?;
        self.validate_terminal_i64_add_return(owner, function)?;
        self.validate_terminal_integer_literal_return(owner, function)?;
        self.validate_terminal_i64_field_return(owner, function)?;
        Ok(())
    }

    /// Recheck the same retained source obligations after compiler finishing.
    /// The early observation cannot authorize a modified function. No source
    /// products are reconstructed from the final CFG or its metadata.
    pub(crate) fn validate_after_compiler_finishing(
        &self,
        function: &MirFunction,
    ) -> Result<(), String> {
        self.validate_finished_root(function, false)
    }

    pub(crate) fn validate_artifact_after_compiler_finishing(
        &self,
        function: &MirFunction,
    ) -> Result<(), String> {
        self.validate_finished_root(function, true)
    }

    fn validate_finished_root(&self, function: &MirFunction, artifact: bool) -> Result<(), String> {
        let mut state = self.root_validation.borrow_mut();
        if artifact && !matches!(*state, RootNewValidation::Checked(..)) {
            return Err(freeze("artifact-root-not-checked"));
        }
        let (owner, boundary) = match &*state {
            RootNewValidation::Unregistered => return Ok(()),
            RootNewValidation::Checked(owner, boundary) => (*owner, boundary),
            RootNewValidation::Pending(_) => return Err(freeze("root-before-draft-validation")),
            RootNewValidation::FinishingChecked => {
                return Err(freeze("duplicate-finishing-validation"));
            }
        };
        let bindings = self.lifecycle_bindings(owner)?;
        let mut projection = boundary.project(function)?;
        self.validate_root_body(owner, function, Some(&projection))?;
        boundary.validate_complete(function, &mut projection, &bindings)?;
        // The finishing projection may rewrite block identities. Rebind the
        // already-issued Call payload before the handoff moves it affinely.
        if matches!(self.terminal_relation.as_ref(), Some(TerminalRelationV1::Call(_))) {
            self.rebind_root_call_entry(owner, &projection)?;
        }
        if artifact
            && self.finalized_root_observation(owner)
                != RootOrdinaryNewObservation::SourceCompleteAtFinalization
        {
            return Err(freeze("artifact-source-unavailable"));
        }
        if self.finalized_root_observation(owner) != function.root_ordinary_new_observation() {
            return Err(freeze("root-observation-drift"));
        }
        if artifact {
            self.validate_artifact_lifecycle_coverage(owner, function, projection.recorded())?;
        }
        *state = RootNewValidation::FinishingChecked;
        Ok(())
    }
}

impl OrdinaryNewClaimLedgerV1 {
    pub(super) fn validate_terminal_unit_return(
        &self,
        owner: FunctionOwnerIdV1,
        function: &MirFunction,
    ) -> Result<(), String> {
        let Some(relation) = self.terminal_unit_return() else {
            return Ok(());
        };
        if relation.owner() != owner {
            return Err(freeze("unit-return-owner-drift"));
        }
        let count = function
            .blocks
            .values()
            .flat_map(|block| block.all_instructions())
            .filter(|instruction| matches!(instruction, MirInstruction::Return { value: None }))
            .count();
        if count != 1 {
            return Err(freeze("unit-return-control-drift"));
        }
        Ok(())
    }
}

impl OrdinaryNewClaimLedgerV1 {
    pub(super) fn validate_terminal_integer_literal_return(
        &self,
        owner: FunctionOwnerIdV1,
        function: &MirFunction,
    ) -> Result<(), String> {
        let Some(relation) = self.terminal_integer_literal_return_for_owner(owner) else {
            return Ok(());
        };
        let value = if self
            .terminal_relation
            .as_ref()
            .is_some_and(|terminal| terminal.owner() == owner)
        {
            *self.terminal_integer_literal_value.borrow()
        } else {
            self.terminal_integer_literal_values
                .borrow()
                .get(&owner)
                .copied()
        };
        let Some(value) = value else {
            return Err(freeze("literal-unconsumed"));
        };
        let exact = function.blocks.values().flat_map(|block| block.all_instructions()).any(|instruction| matches!(instruction, MirInstruction::Const { dst, value: crate::mir::ConstValue::Integer(actual) } if *dst == value && *actual == relation.value()));
        let returned = function.blocks.values().flat_map(|block| block.all_instructions()).any(|instruction| matches!(instruction, MirInstruction::Return { value: Some(actual) } if *actual == value));
        (exact && returned)
            .then_some(())
            .ok_or_else(|| freeze("literal-physical-drift"))
    }
}

impl OrdinaryNewClaimLedgerV1 {
    /// Use the same checked cleanup projection as diagnostic finishing. No stale
    /// block observer or independently reconstructed release list may enter here.
    fn validate_artifact_lifecycle_coverage(
        &self,
        owner: FunctionOwnerIdV1,
        function: &MirFunction,
        cleanup: &[(BasicBlockId, MirInstruction)],
    ) -> Result<(), String> {
        if self
            .local_commits
            .borrow()
            .values()
            .filter(|row| row.binding().owner() == owner)
            .any(|row| !row.is_complete())
        {
            return Err(freeze("artifact-emission-unchecked"));
        }
        let mut expected = Vec::new();
        for (block, instruction) in cleanup {
            if instruction.requires_lifecycle_validation()
                && !expected.contains(&(*block, instruction))
            {
                expected.push((*block, instruction));
            }
        }
        for block in function.blocks.values() {
            for actual in block
                .all_instructions()
                .filter(|i| i.requires_lifecycle_validation())
            {
                let index = expected
                    .iter()
                    .position(|(id, instruction)| *id == block.id && *instruction == actual)
                    .ok_or_else(|| freeze("artifact-unowned-lifecycle-site"))?;
                expected.swap_remove(index);
            }
        }
        if !expected.is_empty() {
            return Err(freeze("artifact-lifecycle-residual"));
        }
        Ok(())
    }
}

impl OrdinaryNewClaimLedgerV1 {
    pub(super) fn lifecycle_bindings(
        &self,
        owner: FunctionOwnerIdV1,
    ) -> Result<Vec<(BasicBlockId, MirInstruction)>, String> {
        let mut result = Vec::new();
        for row in self
            .local_commits
            .borrow()
            .values()
            .filter(|r| r.binding().owner() == owner)
        {
            let bindings = match row {
                LocalCommitV1::Map(map) => map.checked_bindings()?,
                LocalCommitV1::Ordinary(row) => match &row.emission {
                    NewEmissionProgress::Emitted { bindings, .. } => bindings.as_slice(),
                    NewEmissionProgress::RetainedUnavailable { .. } => continue,
                    _ => return Err(freeze("emission-residual")),
                },
            };
            result.extend_from_slice(bindings);
        }
        if let Some(RootHomeExitProgress::Emitted { bindings, entry, .. }) =
            self.root_exits.borrow().get(&owner)
        {
            result.extend_from_slice(bindings);
            entry.append_bindings(&mut result);
        }
        Ok(result)
    }
}
