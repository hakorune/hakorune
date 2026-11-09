//! Source-owned plain scalar Return reads share the existing exit and producer verifier.
use super::root_home::*;
use super::*;
use crate::mir::MirModule;

impl OrdinaryNewClaimLedgerV1 {
    pub(in crate::mir) fn prepare_scalar_return_read_v1(
        &self,
        owner: FunctionOwnerIdV1,
        exit: &SourceStmtSiteV1,
        value: ValueId,
        function: &MirFunction,
    ) -> Result<(), String> {
        let Some(TerminalRelationV1::Scalar(terminal)) =
            self.terminal_relation_for_owner_at(owner, exit)
        else {
            return Ok(());
        };
        let Some(source) = terminal
            .original_read_v1()
            .filter(|_| terminal.evaluation().is_none())
        else {
            return Ok(());
        };
        if source.kind()
            != crate::mir::resolved_semantics::home_new_prefix::SourceScalarKind::Integer
            || source.binding().owner() != owner
            || source.read() != terminal.value_site()
        {
            return Err(freeze("expression/scalar-binding-observation"));
        }
        let proof = {
            let rows = self.root_exits.borrow();
            let Some(RootHomeExitProgress::Prepared(order)) =
                rows.get(&(owner, exit.clone().into()))
            else {
                return Err(freeze("root-exit-record-without-prepare"));
            };
            let read = order
                .return_read
                .as_ref()
                .ok_or_else(|| freeze("expression/scalar-binding-observation"))?;
            let proof = super::call_entry::ScalarBindingReadProofV1::prepare(
                source.clone(),
                read,
                CallValidationContextV1::Lowering(self),
                function,
            )?;
            if proof.value(read)? != value || order.scalar_return_read.is_some() {
                return Err(freeze("expression/scalar-binding-observation"));
            }
            proof
        };
        let mut rows = self.root_exits.borrow_mut();
        let Some(RootHomeExitProgress::Prepared(order)) =
            rows.get_mut(&(owner, exit.clone().into()))
        else {
            return Err(freeze("root-exit-record-without-prepare"));
        };
        order.scalar_return_read = Some(proof);
        Ok(())
    }

    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn validate_plain_scalar_return_read_v1(
        &self,
        context: CallValidationContextV1<'_>,
        owner: FunctionOwnerIdV1,
        exit: &SourceStmtSiteV1,
        order: &RootHomeCleanupOrderV1,
        entry: &RootHomeExitEntry,
        bindings: &[(BasicBlockId, MirInstruction)],
        producer_projection: Option<&physical_boundary::FinishedBindings>,
        return_projection: Option<&physical_boundary::FinishedBindings>,
        function: Option<&MirFunction>,
    ) -> Result<Option<Vec<(BasicBlockId, MirInstruction)>>, String> {
        let source = self
            .terminal_relation_for_owner_at(owner, exit)
            .and_then(|terminal| {
                let TerminalRelationV1::Scalar(terminal) = terminal else {
                    return None;
                };
                terminal
                    .original_read_v1()
                    .filter(|_| terminal.evaluation().is_none())
            });
        let Some(source) = source else {
            if order.scalar_return_read.is_some() {
                return Err(freeze("expression/scalar-binding-observation"));
            }
            return Ok(None);
        };
        let proof = order
            .scalar_return_read
            .as_ref()
            .ok_or_else(|| freeze("expression/scalar-binding-observation"))?;
        let read = order
            .return_read
            .as_ref()
            .ok_or_else(|| freeze("expression/scalar-binding-observation"))?;
        if source != proof.source() || !matches!(entry, RootHomeExitEntry::Plain { .. }) {
            return Err(freeze("expression/scalar-binding-observation"));
        }
        let value = proof.value(read)?;
        if let Some(function) = function {
            proof.validate(read, context, function, producer_projection)?;
        }
        let returned = return_projection
            .map(|p| p.bindings(bindings))
            .transpose()?
            .unwrap_or_else(|| bindings.to_vec());
        if returned
            .iter()
            .filter_map(|(_, row)| match row {
                MirInstruction::Return { value } => Some(*value),
                _ => None,
            })
            .collect::<Vec<_>>()
            != vec![Some(value)]
        {
            return Err(freeze("expression/residence-original-pair"));
        }
        let dependencies = proof
            .dependencies(read)?
            .into_iter()
            .map(|row| match producer_projection {
                Some(projection) => projection
                    .relation_binding(function.ok_or_else(|| freeze("no-function"))?, &row)?
                    .ok_or_else(|| freeze("root-exit-binding-drift")),
                None => Ok(row),
            })
            .collect::<Result<Vec<_>, String>>()?;
        Ok(Some(dependencies))
    }
}

impl FinalizedRootSourceHandoffV1 {
    pub(in crate::mir) fn has_source_scalar_root_v1(&self) -> bool {
        let is_integer_read =
            |terminal: &crate::mir::resolved_semantics::home_new_prefix::TerminalScalarReturnV1| {
                terminal.scalar_kind()
                    == crate::mir::resolved_semantics::home_new_prefix::SourceScalarKind::Integer
                    && terminal.evaluation().map_or_else(
                        || terminal.original_read_v1().is_some(),
                        |evaluation| !self.ledger.expression_requires_fault_frame_v1(evaluation),
                    )
            };
        if self.terminals.is_empty()
            || !self.terminals.values().all(|row| match row {
                TerminalRelationV1::Scalar(terminal) => is_integer_read(terminal),
                TerminalRelationV1::IntegerLiteral(_) => true,
                _ => false,
            })
        {
            return false;
        }
        // Preserve the existing bound-read cohort. Literal-only roots need an
        // original inline evaluation witness, not physical frame presence.
        if self.terminals.values().all(|row| matches!(row,
            TerminalRelationV1::Scalar(terminal) if terminal.evaluation().is_none()
                && terminal.original_read_v1().is_some()))
            || self.terminals.values().any(|row| matches!(row,
                TerminalRelationV1::Scalar(terminal) if terminal.evaluation().is_some_and(|evaluation|
                    !self.ledger.expression_requires_fault_frame_v1(evaluation)))) {
            return true;
        }
        let groups = RootLifecyclePrefixBindingGroupV1::snapshot_tree_v2(
            self.local_calls
                .get(&self.owner)
                .map(Vec::as_slice)
                .unwrap_or(&[]),
        );
        groups
            .iter()
            .filter(|group| group.site().owner() == self.owner)
            .filter_map(|group| {
                group
                    .initializer()
                    .map(|packet| packet.frame().evaluation())
                    .or_else(|| group.rebind().map(|packet| packet.frame().evaluation()))
            })
            .any(|evaluation| !self.ledger.expression_requires_fault_frame_v1(evaluation))
    }
    pub(in crate::mir) fn validate_source_scalar_root_v1(
        &self,
        module: &MirModule,
    ) -> Result<bool, String> {
        if !self.has_source_scalar_root_v1() {
            return Ok(false);
        }
        self.ledger
            .with_finished_projection(self.owner, |symbol, projection| {
                let function = module
                    .functions
                    .get(symbol)
                    .ok_or_else(|| freeze("root-result/function-missing"))?;
                projection.validate_finished_sequences_v1(function)
            })?;
        self.validate_finalized_root_cleanup_v1(module)?;
        Ok(true)
    }
}
