//! Final-root validation is separate from local New emission state changes.

use super::*;
use std::collections::BTreeSet;

#[cfg(test)]
#[path = "terminal_field_projection_tests.rs"]
mod terminal_field_projection_tests;

#[cfg(test)]
#[path = "terminal_literal_storage_tests.rs"]
mod terminal_literal_storage_tests;

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
            RootNewValidation::Checked(..)
            | RootNewValidation::FinishingChecked { .. }
            | RootNewValidation::ArtifactFinalized { .. } => {
                return Err(freeze("duplicate-root-validation"));
            }
        };
        self.validate_root_body(owner, function, None)?;
        let observation = self.finalized_root_observation(owner);
        self.validate_root_cleanup_shape(owner, function)?;
        let bindings = self.lifecycle_bindings(owner)?;
        let copies = self.source_local_copies(owner)?;
        let aliases = self.borrowed_ordinary_alias_bindings_v1(owner)?;
        let boundary = super::physical_boundary::PhysicalBoundary::capture_with_source_copies(
            function, &bindings, &copies, &aliases,
        )?;
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
            self.validate_terminal_i64_field_return_projected(owner, function, Some(&projection))?;
            self.validate_root_home_exit(owner, function, Some(&projection))?;
            boundary.validate_complete(function, &mut projection, &bindings)?;
            self.verify_finished_borrowed_compares_v1(owner, function, |original| {
                let mapped = projection.binding(original.0, &original.1)?
                    .ok_or_else(|| freeze("borrowed-compare/mandatory-binding-removed"))?;
                if !projection.recorded().contains(&mapped) {
                    return Err(freeze("borrowed-compare/mandatory-binding-unrecorded"));
                }
                Ok(mapped)
            })?;
                self.verify_finished_borrowed_compares_v1(owner, function, |original| {
                let mapped = projection.binding(original.0, &original.1)?
                    .ok_or_else(|| freeze("borrowed-compare/mandatory-binding-removed"))?;
                if !projection.recorded().contains(&mapped) {
                    return Err(freeze("borrowed-compare/mandatory-binding-unrecorded"));
                }
                Ok(mapped)
            })?;
            self.validate_forwarded_copies(owner, function, &projection)?;
            if artifact {
                // Children share the root's inadmissibility contract: a
                // RetainedUnavailable commit row is source-unavailable, not a
                // stray coverage site. Mirror the root's observation ordering
                // so the same failure class reports the same terminal.
                let retained: Vec<_> = self
                    .local_commits
                    .borrow()
                    .values()
                    .filter(|row| row.owner() == owner)
                    .filter_map(|row| {
                        let row = row.ordinary()?;
                        matches!(row.emission, NewEmissionProgress::RetainedUnavailable { .. })
                            .then(|| format!(
                                "{:?} decl={:?} construction={:?} home_prefix_err={} children={}",
                                row.emission,
                                row.declaration,
                                row.construction,
                                row.home_prefix.is_err(),
                                row.children.is_some()
                            ))
                    })
                    .collect();
                if !retained.is_empty() {
                    return Err(format!(
                        "{} owner={owner:?} symbol={symbol} retained={retained:?}",
                        freeze("artifact-source-unavailable"),
                    ));
                }
                self.validate_artifact_lifecycle_coverage(owner, function, projection.recorded())?;
            }
            *state = ChildPhysicalValidation::FinishingChecked {
                symbol: symbol.clone(),
                projection,
            };
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
        self.validate_terminal_i64_field_return_projected(owner, function, projection)?;
        Ok(())
    }

    /// The same canonical boundary checker owns draft and finished read positions.
    /// The terminal source validator receives no independent block remapper.
    pub(super) fn validate_terminal_i64_field_return_projected(
        &self,
        owner: FunctionOwnerIdV1,
        function: &MirFunction,
        projection: Option<&super::physical_boundary::FinishedBindings>,
    ) -> Result<(), String> {
        self.validate_terminal_i64_field_return(owner, function, |block, instruction| {
            super::physical_boundary::check_binding(function, projection, block, instruction)
        })
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
            RootNewValidation::FinishingChecked { .. }
            | RootNewValidation::ArtifactFinalized { .. } => {
                return Err(freeze("duplicate-finishing-validation"));
            }
        };
        let bindings = self.lifecycle_bindings(owner)?;
        let mut projection = boundary.project(function)?;
        self.validate_root_body(owner, function, Some(&projection))?;
        boundary.validate_complete(function, &mut projection, &bindings)?;
        self.validate_forwarded_copies(owner, function, &projection)?;
        // The finishing projection may rewrite block identities. Rebind each
        // already-issued Call payload at its own exit before the handoff
        // moves it affinely.
        for site in self.terminal_relation.keys() {
            if self.verified_terminal_call_source_v1(owner, site)?.is_some() {
                self.rebind_root_call_entry(owner, site, &projection)?;
            }
        }
        if artifact
            && !matches!(
                self.finalized_root_observation(owner),
                RootOrdinaryNewObservation::SourceCompleteAtFinalization
                    | RootOrdinaryNewObservation::NoSelectedLocalNew
            )
        {
            return Err(format!(
                "{} debug={:?}",
                freeze("artifact-source-unavailable"),
                self.finalized_root_observation(owner)
            ));
        }
        if self.finalized_root_observation(owner) != function.root_ordinary_new_observation() {
            return Err(freeze("root-observation-drift"));
        }
        if artifact {
            self.validate_artifact_lifecycle_coverage(owner, function, projection.recorded())?;
        }
        *state = RootNewValidation::FinishingChecked {
            owner,
            symbol: function.signature.name.clone(),
            projection: Rc::new(projection),
        };
        Ok(())
    }
}

impl OrdinaryNewClaimLedgerV1 {
    pub(super) fn validate_terminal_unit_return(
        &self,
        owner: FunctionOwnerIdV1,
        function: &MirFunction,
    ) -> Result<(), String> {
        let units: Vec<&TerminalUnitReturnV1> = self
            .terminal_relation
            .values()
            .filter_map(|relation| match relation {
                TerminalRelationV1::Unit(row) => Some(row),
                _ => None,
            })
            .collect();
        if units.is_empty() {
            return Ok(());
        }
        if units.iter().any(|relation| relation.owner() != owner) {
            return Err(freeze("unit-return-owner-drift"));
        }
        let count = function
            .blocks
            .values()
            .flat_map(|block| block.all_instructions())
            .filter(|instruction| matches!(instruction, MirInstruction::Return { value: None }))
            .count();
        if count != units.len() {
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
        let relations: Vec<&TerminalIntegerLiteralReturnV1> = self
            .terminal_relations_for_owner(owner)
            .into_iter()
            .filter_map(|relation| match relation {
                TerminalRelationV1::IntegerLiteral(row) => Some(row),
                _ => None,
            })
            .collect();
        if relations.is_empty() {
            return Ok(());
        }
        for relation in relations {
            let site = relation.return_site();
            let value = if self.terminal_scalar_uses_child_storage(owner, site) {
                self.terminal_integer_literal_values
                    .borrow()
                    .get(&(owner, site.clone()))
                    .copied()
            } else {
                self.terminal_integer_literal_value
                    .borrow()
                    .get(site)
                    .copied()
            };
            let Some(value) = value else {
                return Err(freeze("literal-unconsumed"));
            };
            let exact = function.blocks.values().flat_map(|block| block.all_instructions()).any(|instruction| matches!(instruction, MirInstruction::Const { dst, value: crate::mir::ConstValue::Integer(actual) } if *dst == value && *actual == relation.value()));
            let returned = function.blocks.values().flat_map(|block| block.all_instructions()).any(|instruction| matches!(instruction, MirInstruction::Return { value: Some(actual) } if *actual == value));
            if !(exact && returned) {
                return Err(freeze("literal-physical-drift"));
            }
        }
        Ok(())
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
            .filter(|row| row.owner() == owner)
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
            for (instruction_index, actual) in block
                .all_instructions()
                .enumerate()
                .filter(|(_, i)| i.requires_lifecycle_validation())
            {
                let index = expected
                    .iter()
                    .position(|(id, instruction)| *id == block.id && *instruction == actual)
                    .ok_or_else(|| {
                        format!(
                            "{} function={} block={:?} instruction_index={} instruction={:?}",
                            freeze("artifact-unowned-lifecycle-site"),
                            function.signature.name,
                            block.id,
                            instruction_index,
                            actual
                        )
                    })?;
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
    pub(super) fn source_local_copies(
        &self,
        owner: FunctionOwnerIdV1,
    ) -> Result<Vec<(ValueId, ValueId)>, String> {
        let mut result = Vec::new();
        for row in self
            .local_commits
            .borrow()
            .values()
            .filter(|row| row.owner() == owner)
        {
            let LocalCommitV1::Ordinary(row) = row else {
                continue;
            };
            let NewEmissionProgress::Emitted {
                result: emitted, ..
            } = &row.emission
            else {
                continue;
            };
            let local = row
                .emission
                .local()
                .ok_or_else(|| freeze("source-copy-local-unavailable"))?;
            result.push((local, *emitted));
        }
        Ok(result)
    }

    pub(super) fn validate_forwarded_copies(
        &self,
        owner: FunctionOwnerIdV1,
        function: &MirFunction,
        projection: &super::physical_boundary::FinishedBindings,
    ) -> Result<(), String> {
        for ((row_owner, _), progress) in self.root_exits.borrow().iter() {
            if *row_owner != owner {
                continue;
            }
            let RootHomeExitProgress::Emitted { entry, .. } = progress else {
                continue;
            };
            for original in entry.copy_dependencies(self)? {
                let finished = projection
                    .binding(original.0, &original.1)?
                    .ok_or_else(|| freeze("forwarded-copy/removed"))?;
                let MirInstruction::Copy { dst, .. } = &finished.1 else {
                    return Err(freeze("forwarded-copy/finished-instruction"));
                };
                let count = function
                    .blocks
                    .get(&finished.0)
                    .ok_or_else(|| freeze("forwarded-copy/finished-block"))?
                    .all_instructions()
                    .filter(|i| *i == &finished.1)
                    .count();
                let definitions = function
                    .blocks
                    .values()
                    .flat_map(|b| b.all_instructions())
                    .filter(|i| i.dst_value() == Some(*dst))
                    .count();
                if count != 1 || definitions != 1 {
                    return Err(freeze("forwarded-copy/finished-unique"));
                }
            }
        }
        Ok(())
    }

    pub(super) fn lifecycle_bindings(
        &self,
        owner: FunctionOwnerIdV1,
    ) -> Result<Vec<(BasicBlockId, MirInstruction)>, String> {
        let mut result = Vec::new();
        for row in self
            .local_commits
            .borrow()
            .values()
            .filter(|r| r.owner() == owner)
        {
            let bindings = match row {
                LocalCommitV1::Map(map) => map.checked_bindings()?,
                LocalCommitV1::CallReceived(row) => row.checked_bindings()?,
                LocalCommitV1::Ordinary(_) | LocalCommitV1::Result(_) => {
                    match row.new_emission().expect("new row carries emission") {
                        NewEmissionProgress::Emitted { bindings, .. } => bindings.as_slice(),
                        NewEmissionProgress::RetainedUnavailable { .. } => continue,
                        _ => return Err(freeze("emission-residual")),
                    }
                }
            };
            result.extend_from_slice(bindings);
        }
        for ((row_owner, _), progress) in self.root_exits.borrow().iter() {
            if *row_owner != owner {
                continue;
            }
            if let RootHomeExitProgress::Emitted {
                bindings, entry, order, ..
            } = progress
            {
                result.extend_from_slice(bindings);
                entry.append_bindings(&mut result);
                if let Some(original) = order.null_return_binding() {
                    result.push(original.clone());
                }
                for dependency in entry.copy_dependencies(self)? {
                    if !result.contains(&dependency) {
                        result.push(dependency);
                    }
                }
            }
        }
        if let Some(groups) = self.map_read_bindings.borrow().get(&owner) {
            for (_, bindings) in groups {
                result.extend_from_slice(bindings);
            }
        }
        result.extend(self.borrowed_compare_bindings_v1(owner)?);
        Ok(result)
    }
}
