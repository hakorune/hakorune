//! Exact construction commit joins its original producer to the same exit.
use super::super::physical_boundary::FinishedBindings;
use super::*;

impl OrdinaryNewClaimLedgerV1 {
    /// Lend the original completed Construction's End; do not install a Home.
    fn completed_fresh_return_cleanup(
        &self,
        owner: FunctionOwnerIdV1,
        exit: &SourceStmtSiteV1,
    ) -> Result<Option<(ValueId, Vec<RootHomeReleaseOriginV1>)>, String> {
        let Some(TerminalRelationV1::Value(terminal)) = self.terminal_relation_for_owner_at(owner, exit)
        else { return Ok(None) };
        let TerminalReturnedSourceV1::Construction(site) = terminal.returned()
        else { return Ok(None) };
        if terminal.owner() != owner || terminal.return_site() != exit
            || site != &OwnedExprSiteV1::new(owner, terminal.value_site().clone())
        { return Err(freeze("fresh-result-cleanup/source")); }
        let rows = self.local_commits.borrow();
        let Some(LocalCommitV1::Result(row)) = rows.get(site)
        else { return Err(freeze("fresh-result-cleanup/commit")) };
        // Preserve diagnostic-only construction unavailability. It issues no
        // End plan or artifact permission, as in the producer checker below.
        if matches!(row.emission, NewEmissionProgress::RetainedUnavailable { .. }) {
            return Ok(None);
        }
        if &row.site != site || !matches!(&row.construction, Ok(plan) if plan.object() == row.object)
            || row.home_prefix.is_err() || row.argument_rows.is_err()
        { return Err(freeze("fresh-result-cleanup/construction")); }
        let NewEmissionProgress::Emitted { result, progress, .. } = &row.emission
        else { return Err(freeze("fresh-result-cleanup/not-emitted")) };
        if !matches!(progress, EmittedLocalProgress::ExpressionCompleted)
            && !matches!(progress, EmittedLocalProgress::Checked { local } if local == result)
        { return Err(freeze("fresh-result-cleanup/not-completed")); }
        let children = match row.destruction {
            super::super::super::ObjectDestructionDispositionV1::PlainI64NoHook => &[][..],
            super::super::super::ObjectDestructionDispositionV1::OwnedArrayFieldsNoHook
            | super::super::super::ObjectDestructionDispositionV1::OwnedObjectFieldsNoHook =>
                row.children.as_deref().ok_or_else(|| freeze("fresh-result-cleanup/children"))?,
            super::super::super::ObjectDestructionDispositionV1::Unavailable(_) =>
                return Err(freeze("fresh-result-cleanup/destruction")),
        };
        let origins = super::super::super::object_end::operations_v1(row.object, false, children, *result)
            .map(|(field, operation)| RootHomeReleaseOriginV1 {
                subject: match field {
                    Some(field) => RootHomeReleaseSubjectV1::DirectResultField { site: site.clone(), field },
                    None => RootHomeReleaseSubjectV1::DirectResult { site: site.clone() },
                },
                operation,
                exit: exit.clone().into(),
            }).collect();
        Ok(Some((*result, origins)))
    }

    pub(crate) fn bind_root_fresh_result_v1(
        &self,
        owner: FunctionOwnerIdV1,
        exit: &SourceStmtSiteV1,
        value: ValueId,
    ) -> Result<(), String> {
        let mut exits = self.root_exits.borrow_mut();
        let Some(RootHomeExitProgress::Emitting(order)) = exits.get_mut(&(owner, exit.clone().into()))
        else { return Err(freeze("fresh-result-cleanup/not-emitting")) };
        // Without a Normal cleanup step there is no later cleanup Fault.
        if order.normal().is_empty() { return Ok(()) }
        let Some((original, origins)) = self.completed_fresh_return_cleanup(owner, exit)?
        else { return Ok(()) };
        if original != value { return Err(freeze("fresh-result-cleanup/value")); }
        order.prepend_fault_only_result(origins)
    }
}

impl RootHomeCleanupOrderV1 {
    pub(super) fn validate_fresh_result_cleanup(
        &self,
        ledger: &OrdinaryNewClaimLedgerV1,
        owner: FunctionOwnerIdV1,
        exit: &SourceStmtSiteV1,
        entry: &RootHomeExitEntry,
    ) -> Result<(), String> {
        if self.normal().is_empty() { return Ok(()) }
        let Some((_, origins)) = ledger.completed_fresh_return_cleanup(owner, exit)?
        else { return Ok(()) };
        if !matches!(entry, RootHomeExitEntry::Plain { .. }) {
            return Err(freeze("fresh-result-cleanup/entry"));
        }
        self.validate_fault_only_result(&origins)
    }
}

impl OrdinaryNewClaimLedgerV1 {
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn validate_fresh_return_producer_v1(
        &self,
        owner: FunctionOwnerIdV1,
        exit: &SourceStmtSiteV1,
        entry: &RootHomeExitEntry,
        bindings: &[(BasicBlockId, MirInstruction)],
        producer_projection: Option<&FinishedBindings>,
        return_projection: Option<&FinishedBindings>,
        function: Option<&MirFunction>,
    ) -> Result<Option<[(BasicBlockId, MirInstruction); 2]>, String> {
        let Some(TerminalRelationV1::Value(value)) =
            self.terminal_relation_for_owner_at(owner, exit)
        else {
            return Ok(None);
        };
        let TerminalReturnedSourceV1::Construction(site) = value.returned() else {
            return Ok(None);
        };
        if value.owner() != owner
            || value.return_site() != exit
            || site != &OwnedExprSiteV1::new(owner, value.value_site().clone())
            || !matches!(entry, RootHomeExitEntry::Plain { .. })
        {
            return Err(freeze("fresh-return/source-or-entry"));
        }
        let rows = self.local_commits.borrow();
        let Some(LocalCommitV1::Result(row)) = rows.get(site) else {
            return Err(freeze("fresh-return/commit-missing"));
        };
        if &row.site != site {
            return Err(freeze("fresh-return/commit-identity"));
        }
        // Retained diagnostics remain unavailable, never a normal producer.
        if matches!(
            row.emission,
            NewEmissionProgress::RetainedUnavailable { .. }
        ) {
            return Ok(None);
        }
        let NewEmissionProgress::Emitted {
            result,
            bindings: produced,
            progress,
            ..
        } = &row.emission
        else {
            return Err(freeze("fresh-return/producer-missing"));
        };
        match (producer_projection, progress) {
            (None, EmittedLocalProgress::ExpressionCompleted) => {}
            (_, EmittedLocalProgress::Checked { local }) if local == result => {}
            _ => return Err(freeze("fresh-return/producer-phase")),
        }
        let mut producers = produced.iter().filter(|(_, instruction)| {
            matches!(instruction, MirInstruction::InvokeNormalResult { dst, .. } if dst == result)
        });
        let original = producers
            .next()
            .ok_or_else(|| freeze("fresh-return/producer-missing"))?;
        if producers.next().is_some() {
            return Err(freeze("fresh-return/producer-duplicate"));
        }
        let MirInstruction::InvokeNormalResult { invoke_block, .. } = &original.1 else {
            unreachable!()
        };
        let mut allocations = produced.iter().filter(|(block, instruction)| {
            block == invoke_block
                && matches!(instruction, MirInstruction::Invoke {
                operation: InvokeOperation::NewBox { object }, normal_landing, ..
            } if *object == row.object && *normal_landing == original.0)
        });
        let allocation = allocations
            .next()
            .ok_or_else(|| freeze("fresh-return/allocation-identity"))?;
        if allocations.next().is_some() {
            return Err(freeze("fresh-return/allocation-identity"));
        }
        let allocation = match producer_projection {
            Some(projection) => projection
                .binding(allocation.0, &allocation.1)?
                .ok_or_else(|| freeze("fresh-return/allocation-removed"))?,
            None => allocation.clone(),
        };
        let producer = match producer_projection {
            Some(projection) => projection
                .binding(original.0, &original.1)?
                .ok_or_else(|| freeze("fresh-return/producer-removed"))?,
            None => original.clone(),
        };
        let MirInstruction::InvokeNormalResult { dst, .. } = &producer.1 else {
            return Err(freeze("fresh-return/producer-kind"));
        };
        let returned = match return_projection {
            Some(projection) => projection.bindings(bindings)?,
            None => bindings.to_vec(),
        };
        let mut returns = returned
            .iter()
            .filter_map(|(_, instruction)| match instruction {
                MirInstruction::Return { value } => Some(value),
                _ => None,
            });
        if returns.next() != Some(&Some(*dst)) || returns.next().is_some() {
            return Err(freeze("fresh-return/return-value"));
        }
        if let Some(function) = function {
            for binding in [&producer, &allocation] {
                if !super::super::physical_boundary::check_binding(
                    function, None, binding.0, &binding.1,
                )? {
                    return Err(freeze("fresh-return/producer-actual-drift"));
                }
            }
        }
        Ok(Some([producer, allocation]))
    }
}
