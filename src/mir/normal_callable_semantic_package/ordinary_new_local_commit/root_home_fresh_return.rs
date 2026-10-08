//! Exact construction commit joins its original producer to the same exit.
use super::super::physical_boundary::FinishedBindings;
use super::*;

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
