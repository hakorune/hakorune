//! Typed inputs close over the original inventory, Completion and signature.
//! This regenerates actuals, never a borrowed entry or a Normal return seal.
use super::*;
use crate::mir::callable_parameter_contract::CallableParameterDeclarationModeV1;
use crate::mir::normal_callable_semantic_package::{
    physical_signature::{PhysicalCallableLaneRoleV1, VerifiedCallablePhysicalSignatureCohortV1},
    selected_mapping::VerifiedSelectedCallableBatchMapV1,
    OrdinaryNewClaimLedgerV1,
};
use std::collections::BTreeSet;

#[cfg(test)]
#[path = "ordinary_new_borrowed_object_input_finish_tests.rs"]
mod tests;

impl OrdinaryNewClaimLedgerV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn finish_typed_object_input_actuals_v1(
        &mut self,
        selected: &VerifiedSelectedCallableBatchMapV1,
        contracts: &[OwnedCallableParameterContractDeclarationV1],
        signatures: &VerifiedCallablePhysicalSignatureCohortV1,
    ) -> Result<(), String> {
        let Some(Ok(source)) = &self.borrowed_formal_source else {
            // No successful inventory to select. Preserve the original failure.
            return Ok(());
        };
        let owners: BTreeSet<_> = source
            .source_incoming
            .exact_rows()
            .filter(|row| {
                row.source
                    .instance()
                    .is_some_and(|target| target.has_object_source_requirement())
            })
            .map(|row| row.callee)
            .collect();
        for owner in owners {
            let mut matching = contracts.iter().filter(|row| row.owner == owner);
            let contract = matching
                .next()
                .ok_or_else(|| freeze("object-input/contract-missing"))?;
            if matching.next().is_some() {
                return Err(freeze("object-input/contract-duplicate"));
            }
            if !contract.parameters.iter().all(|formal| matches!(formal.kind,
                CallableParameterContractKindV1::ExactTrivial(abi)
                    if abi == crate::mir::exact_trivial_parameter_abi::ExactTrivialParameterAbiV1::I64)) {
                continue;
            }
            match self
                .prepare_typed_object_input_owner_v1(source, selected, contract, signatures)?
            {
                TypedInputClosureV1::Pending => {}
                TypedInputClosureV1::Refused(issue) => {
                    for call in source
                        .source_incoming
                        .exact_rows()
                        .filter(|row| row.callee == owner)
                    {
                        if let Some(row) = self.borrowed_formal_actuals.get_mut(&call.call) {
                            if row.is_ok() {
                                *row = Err(issue.clone());
                            }
                        }
                    }
                }
                TypedInputClosureV1::Ready(rows) => {
                    for (site, actuals) in rows {
                        self.borrowed_formal_actuals.insert(site, Ok(actuals));
                    }
                }
            }
        }
        Ok(())
    }

    /// Demand-time recheck of the original completed input cohort.
    /// Returns original argument storage only; no result or Normal authority.
    pub(in crate::mir::normal_callable_semantic_package) fn checked_completed_typed_object_arguments_v1(
        &self,
        target: &LexicalInstanceCallSourceTargetV1,
        loan: &crate::mir::normal_callable_semantic_package::ObjectReturnCallQualificationV1,
        selected: &VerifiedSelectedCallableBatchMapV1,
        contracts: &[OwnedCallableParameterContractDeclarationV1],
        signatures: &VerifiedCallablePhysicalSignatureCohortV1,
    ) -> Result<Option<&[LocalCallArgumentV1]>, String> {
        if loan.call() != target.call_site()
            || loan.key() != target.target()
            || self
                .callable_result_classes
                .object_return_qualification(loan.value())
                .as_ref()
                != Some(loan)
            || !target
                .object_return_sources()
                .is_some_and(|rows| rows.contains(loan))
        {
            return Err(freeze("object-input/return-qualification"));
        }
        self.checked_completed_typed_object_target_arguments_v1(
            target, selected, contracts, signatures,
        )
    }

    /// Borrows input evidence for the exact original target without granting result authority.
    pub(in crate::mir::normal_callable_semantic_package) fn checked_completed_typed_object_target_arguments_v1(
        &self,
        target: &LexicalInstanceCallSourceTargetV1,
        selected: &VerifiedSelectedCallableBatchMapV1,
        contracts: &[OwnedCallableParameterContractDeclarationV1],
        signatures: &VerifiedCallablePhysicalSignatureCohortV1,
    ) -> Result<Option<&[LocalCallArgumentV1]>, String> {
        let source = self
            .borrowed_formal_source
            .as_ref()
            .ok_or_else(|| freeze("object-input/source-missing"))?
            .as_ref()
            .map_err(Clone::clone)?;
        let mut matching = contracts
            .iter()
            .filter(|row| row.owner == target.callee_owner());
        let contract = matching
            .next()
            .ok_or_else(|| freeze("object-input/contract-missing"))?;
        if matching.next().is_some() {
            return Err(freeze("object-input/contract-duplicate"));
        }
        if !contract.parameters.iter().all(|formal| {
            matches!(formal.kind,
            CallableParameterContractKindV1::ExactTrivial(abi)
                if abi == crate::mir::exact_trivial_parameter_abi::ExactTrivialParameterAbiV1::I64)
        }) {
            return Ok(None);
        }
        match self.prepare_typed_object_input_owner_v1(source, selected, contract, signatures)? {
            TypedInputClosureV1::Pending => return Ok(None),
            TypedInputClosureV1::Refused(issue) => return Err(issue),
            TypedInputClosureV1::Ready(rows) if !rows.is_empty() => return Ok(None),
            TypedInputClosureV1::Ready(_) => {}
        }
        let mut matching = source
            .source_incoming
            .exact_rows()
            .filter(|row| &row.call == target.call_site());
        let call = matching
            .next()
            .ok_or_else(|| freeze("object-input/original-row-missing"))?;
        if matching.next().is_some()
            || call.source.require_instance()? != target
            || call.callee != target.callee_owner()
        {
            return Err(freeze("object-input/original-target"));
        }
        let Some(actuals) = self.borrowed_formal_actuals.get(target.call_site()) else {
            return Ok(None);
        };
        let actuals = actuals.as_ref().map_err(Clone::clone)?;
        actuals.ordered_arguments_for_v1(call).map(Some)
    }

    fn prepare_typed_object_input_owner_v1(
        &self,
        source: &PreparedBorrowedFormalIngressV1,
        selected: &VerifiedSelectedCallableBatchMapV1,
        contract: &OwnedCallableParameterContractDeclarationV1,
        signatures: &VerifiedCallablePhysicalSignatureCohortV1,
    ) -> Result<TypedInputClosureV1, String> {
        let owner = contract.owner;
        let Some(crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(key)) =
            selected.key_for_batch_slot(contract.batch_slot)
        else {
            return Err(freeze("object-input/selected-key"));
        };
        if contract.mode != CallableParameterDeclarationModeV1::InstanceBoxMethod
            || key.namespace()
                != hakorune_mir_defs::SameModuleCallableNamespaceV1::InstanceBoxMethod
            || key.arity() as usize != contract.parameters.len()
            || contract
                .parameters
                .iter()
                .enumerate()
                .any(|(ordinal, formal)| {
                    formal.ordinal as usize != ordinal || formal.binding.owner() != owner
                })
        {
            return Err(freeze("object-input/contract-identity"));
        }
        let calls = match source
            .source_incoming
            .project(&[owner].into_iter().collect())
        {
            Ok(rows) => rows,
            Err(issue) => {
                return Ok(TypedInputClosureV1::Refused(format!(
                    "{}: {issue:?}",
                    freeze("borrowed-formal/incoming-coverage")
                )))
            }
        };
        let Some(completion) = self.completion_index.get(&owner) else {
            return Ok(TypedInputClosureV1::Pending);
        };
        let completion = match completion {
            Ok(row) => row,
            Err(issue) => {
                return Ok(TypedInputClosureV1::Refused(format!(
                    "{}: {issue:?}",
                    freeze("object-input/callee-completion")
                )))
            }
        };
        if completion.owner() != owner {
            return Err(freeze("object-input/completion-owner"));
        }
        let Some(flow) = completion.cleanup().root_flow() else {
            return Ok(TypedInputClosureV1::Pending);
        };
        if completion.explicit_sites().is_empty() {
            return Ok(TypedInputClosureV1::Pending);
        }
        for exit in completion.explicit_sites() {
            match flow.exit_row(exit) {
                Some(Err(issue)) => {
                    return Ok(TypedInputClosureV1::Refused(format!(
                        "{}: {issue:?}",
                        freeze("object-input/callee-exit")
                    )))
                }
                None => return Ok(TypedInputClosureV1::Pending),
                Some(Ok(_)) => {}
            }
            if self.terminal_relation_for_owner_at(owner, exit).is_none() {
                return Ok(TypedInputClosureV1::Pending);
            }
        }
        let Some(signature) = signatures.row(contract.batch_slot) else {
            return Ok(TypedInputClosureV1::Pending);
        };
        let identity = selected
            .identity_for_batch_slot(contract.batch_slot)
            .ok_or_else(|| freeze("object-input/signature-identity"))?;
        let lanes = signature.lanes();
        if !signature.identity().same_as(identity)
            || signature.owner() != owner
            || signature.mode() != contract.mode
            || signature.source_logical_arity() as usize != contract.parameters.len()
            || signature.receiver_lane_count() != 1
            || signature.physical_formal_lane_count() as usize != contract.parameters.len()
            || signature.physical_callable_lane_count() as usize != contract.parameters.len() + 1
            || lanes.len() != contract.parameters.len() + 1
            || lanes[0].role() != PhysicalCallableLaneRoleV1::InstanceReceiver
            || lanes[0].index() != 0
            || lanes[0].logical_ordinal().is_some()
            || lanes[0].binding().owner() != owner
            || lanes[1..]
                .iter()
                .zip(&contract.parameters)
                .any(|(lane, formal)| {
                    lane.role() != PhysicalCallableLaneRoleV1::OrdinaryScalar
                        || lane.index() != formal.ordinal + 1
                        || lane.logical_ordinal() != Some(formal.ordinal)
                        || lane.binding() != formal.binding
                })
        {
            return Err(freeze("object-input/signature-identity"));
        }
        let mut missing = false;
        let mut replacements = Vec::new();
        for call in &calls {
            let target = call.source.require_instance()?;
            if call.callee != owner
                || target.callee_owner() != owner
                || target.target() != key
                || target.target_batch_slot() != contract.batch_slot
                || target.call_site() != &call.call
                || !call.arguments.is_empty()
                || !source.source_incoming.has_object_input_callee_v1(target)
            {
                return Err(freeze("object-input/original-target"));
            }
            let Some(actuals) = self.borrowed_formal_actuals.get(&call.call) else {
                missing = true;
                continue;
            };
            let actuals = match actuals {
                Ok(rows) => rows,
                Err(issue) => return Ok(TypedInputClosureV1::Refused(issue.clone())),
            };
            match &actuals.phase {
                BorrowedCallActualEvidencePhaseV1::SourceObject(original) => {
                    if original.source != *target
                        || original.incoming_arguments != call.arguments
                        || !actuals.opaque_actuals.is_empty()
                    {
                        return Err(freeze("object-input/original-actuals"));
                    }
                    let regenerated = construct_borrowed_call_actuals_v1(
                        source,
                        call,
                        std::slice::from_ref(contract),
                        &call.call,
                        &original.candidates,
                        &[],
                        None,
                        &mut |_| None,
                        None,
                    )?
                    .ok_or_else(|| freeze("object-input/constructor-unavailable"))?;
                    if regenerated.ordered_arguments != actuals.ordered_arguments {
                        return Err(freeze("object-input/argument-snapshot-drift"));
                    }
                    replacements.push((call.call.clone(), regenerated));
                }
                BorrowedCallActualEvidencePhaseV1::Executable => {
                    actuals.ordered_arguments_for_v1(call)?;
                }
                _ => return Err(freeze("object-input/actual-phase")),
            }
        }
        Ok(if missing {
            TypedInputClosureV1::Pending
        } else {
            TypedInputClosureV1::Ready(replacements)
        })
    }
}
