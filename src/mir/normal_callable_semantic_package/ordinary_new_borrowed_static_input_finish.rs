//! Zero-input Static closure uses the original inventory, signature and Completion.
//! Finished evidence stays in the existing actual row; no borrowed entry is issued.
use super::*;
use crate::mir::callable_parameter_contract::CallableParameterDeclarationModeV1;
use crate::mir::normal_callable_semantic_package::{
    physical_signature::VerifiedCallablePhysicalSignatureCohortV1,
    qualified_static_call_claim::incoming_source::StaticIncomingSourceV1,
    result_contract::VerifiedCallableResultContractCohortV1,
    selected_mapping::VerifiedSelectedCallableBatchMapV1, OrdinaryNewClaimLedgerV1,
};
use std::rc::Rc;

#[derive(Debug, Clone)]
pub(super) struct StaticZeroInputFinishV1 {
    source: Rc<StaticIncomingSourceV1>,
    completion: Rc<crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1>,
}
impl PartialEq for StaticZeroInputFinishV1 {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.source, &other.source) && Rc::ptr_eq(&self.completion, &other.completion)
    }
}
impl Eq for StaticZeroInputFinishV1 {}
impl StaticZeroInputFinishV1 {
    pub(super) fn corroborates_source_v1(&self, source: &Rc<StaticIncomingSourceV1>) -> bool {
        Rc::ptr_eq(&self.source, source)
            && source.is_zeroarg_i64_v1()
            && self.completion.owner() == source.callee_owner()
    }
}

impl OrdinaryNewClaimLedgerV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn finish_static_zero_input_actuals_v1(
        &mut self,
        selected: &VerifiedSelectedCallableBatchMapV1,
        contracts: &[OwnedCallableParameterContractDeclarationV1],
        signatures: &VerifiedCallablePhysicalSignatureCohortV1,
        results: &VerifiedCallableResultContractCohortV1,
    ) -> Result<(), String> {
        let Some(Ok(source)) = self.borrowed_formal_source.as_ref() else {
            return Ok(());
        };
        let owners: std::collections::BTreeSet<_> = source
            .source_incoming
            .exact_rows()
            .filter_map(|call| match &call.source {
                super::super::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(original)
                    if original.current_owner_source().is_some()
                        && original.is_zeroarg_i64_v1() =>
                {
                    Some(call.callee)
                }
                _ => None,
            })
            .collect();
        for owner in owners {
            let mut matching = contracts.iter().filter(|row| row.owner == owner);
            let contract = matching
                .next()
                .ok_or_else(|| freeze("static-zero/contract-missing"))?;
            if matching.next().is_some() {
                return Err(freeze("static-zero/contract-duplicate"));
            }
            match self.prepare_static_zero_input_owner_v1(
                source, selected, contract, signatures, results,
            )? {
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

    /// Borrow only the SAME completed whole input cohort. No caller result,
    /// transport or affine publication permission is issued by this lender.
    pub(in crate::mir::normal_callable_semantic_package) fn checked_completed_static_zero_arguments_v1(
        &self,
        original: &Rc<StaticIncomingSourceV1>,
    ) -> Result<Option<&[LocalCallArgumentV1]>, String> {
        if !original.is_zeroarg_i64_v1() {
            return Err(freeze("static-zero/source-contract"));
        }
        let source = self
            .borrowed_formal_source
            .as_ref()
            .ok_or_else(|| freeze("static-zero/source-missing"))?
            .as_ref()
            .map_err(Clone::clone)?;
        let retained = source
            .source_incoming
            .static_observations()
            .get(original.call_site())
            .ok_or_else(|| freeze("static-zero/original-source-missing"))?
            .as_ref()
            .map_err(Clone::clone)?;
        if !Rc::ptr_eq(retained, original) {
            return Err(freeze("static-zero/original-source-drift"));
        }
        let owner = original.callee_owner();
        let calls = source
            .source_incoming
            .project(&[owner].into_iter().collect())
            .map_err(|issue| {
                format!("{}: {issue:?}", freeze("borrowed-formal/incoming-coverage"))
            })?;
        let Some(completion) = self.completion_index.get(&owner) else {
            return Ok(None);
        };
        let completion = completion
            .as_ref()
            .map_err(|issue| format!("{}: {issue:?}", freeze("static-zero/callee-completion")))?;
        if completion.owner() != owner {
            return Err(freeze("static-zero/completion-owner"));
        }
        let mut missing = false;
        let mut requested = None;
        let mut seen = std::collections::BTreeSet::new();
        for call in &calls {
            if !seen.insert(call.call.clone()) {
                return Err(freeze("static-zero/original-row-duplicate"));
            }
            let super::super::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(source_call) =
                &call.source
            else {
                return Err(freeze("static-zero/original-source-kind"));
            };
            let retained = source
                .source_incoming
                .static_observations()
                .get(&call.call)
                .ok_or_else(|| freeze("static-zero/original-source-missing"))?
                .as_ref()
                .map_err(Clone::clone)?;
            if !Rc::ptr_eq(source_call, retained)
                || !source_call.is_zeroarg_i64_v1()
                || source_call.target() != original.target()
                || source_call.callee_owner() != owner
                || source_call.target_batch_slot() != original.target_batch_slot()
                || source_call.call_site() != &call.call
                || !call.arguments.is_empty()
            {
                return Err(freeze("static-zero/original-target"));
            }
            let Some(actuals) = self.borrowed_formal_actuals.get(&call.call) else {
                missing = true;
                continue;
            };
            let actuals = actuals.as_ref().map_err(Clone::clone)?;
            let BorrowedCallActualEvidencePhaseV1::ExecutableStaticZero(finish) = &actuals.phase
            else {
                missing = true;
                continue;
            };
            if !finish.corroborates_source_v1(source_call)
                || !Rc::ptr_eq(&finish.completion, completion)
            {
                return Err(freeze("static-zero/completion-drift"));
            }
            let arguments = actuals.ordered_arguments_for_v1(call)?;
            if !arguments.is_empty() {
                return Err(freeze("static-zero/argument-snapshot-drift"));
            }
            if &call.call == original.call_site() {
                if requested.is_some() {
                    return Err(freeze("static-zero/original-row-duplicate"));
                }
                requested = Some(arguments);
            }
        }
        Ok(if missing { None } else { requested })
    }

    fn prepare_static_zero_input_owner_v1(
        &self,
        source: &PreparedBorrowedFormalIngressV1,
        selected: &VerifiedSelectedCallableBatchMapV1,
        contract: &OwnedCallableParameterContractDeclarationV1,
        signatures: &VerifiedCallablePhysicalSignatureCohortV1,
        results: &VerifiedCallableResultContractCohortV1,
    ) -> Result<TypedInputClosureV1, String> {
        let owner = contract.owner;
        let Some(SelectedNormalCallableKeyV1::Cataloged(key)) =
            selected.key_for_batch_slot(contract.batch_slot)
        else {
            return Err(freeze("static-zero/selected-key"));
        };
        if contract.mode != CallableParameterDeclarationModeV1::StaticBoxMethod
            || key.namespace() != hakorune_mir_defs::SameModuleCallableNamespaceV1::StaticBoxMethod
            || key.arity() != 0
            || !contract.parameters.is_empty()
        {
            return Err(freeze("static-zero/contract-identity"));
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
                    freeze("static-zero/callee-completion")
                )))
            }
        };
        let Some(signature) = signatures.row(contract.batch_slot) else {
            return Ok(TypedInputClosureV1::Pending);
        };
        let identity = selected
            .identity_for_batch_slot(contract.batch_slot)
            .ok_or_else(|| freeze("static-zero/signature-identity"))?;
        if !signature.identity().same_as(identity)
            || signature.owner() != owner
            || signature.mode() != contract.mode
            || signature.source_logical_arity() != 0
            || signature.receiver_lane_count() != 0
            || signature.physical_formal_lane_count() != 0
            || signature.physical_callable_lane_count() != 0
            || !signature.lanes().is_empty()
        {
            return Err(freeze("static-zero/signature-identity"));
        }
        let Some(result) = results.row(contract.batch_slot) else {
            return Ok(TypedInputClosureV1::Pending);
        };
        let result = result.borrow();
        if result.owner() != owner
            || !result.identity().same_as(identity)
            || !result.declared_result_agrees_with_i64_source()
            || !completion.returns_value()
            || completion.owner() != owner
            || !std::ptr::eq(result.completion(), completion.as_ref())
        {
            return Err(freeze("static-zero/result-completion-identity"));
        }
        let mut missing = false;
        let mut replacements = Vec::new();
        let mut seen = std::collections::BTreeSet::new();
        for call in &calls {
            if !seen.insert(call.call.clone()) {
                return Err(freeze("static-zero/original-row-duplicate"));
            }
            let super::super::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(original) =
                &call.source
            else {
                return Err(freeze("static-zero/original-source-kind"));
            };
            let retained = source
                .source_incoming
                .static_observations()
                .get(&call.call)
                .ok_or_else(|| freeze("static-zero/original-source-missing"))?
                .as_ref()
                .map_err(Clone::clone)?;
            if call.callee != owner
                || !Rc::ptr_eq(original, retained)
                || !original.is_zeroarg_i64_v1()
                || original.callee_owner() != owner
                || original.target() != key
                || original.target_batch_slot() != contract.batch_slot
                || original.call_site() != &call.call
                || !call.arguments.is_empty()
            {
                return Err(freeze("static-zero/original-target"));
            }
            let Some(actuals) = self.borrowed_formal_actuals.get(&call.call) else {
                missing = true;
                continue;
            };
            let actuals = match actuals {
                Ok(rows) => rows,
                Err(issue) => return Ok(TypedInputClosureV1::Refused(issue.clone())),
            };
            if !actuals.opaque_actuals.is_empty() || !actuals.ordered_arguments.is_empty() {
                return Err(freeze("static-zero/original-actuals"));
            }
            match &actuals.phase {
                BorrowedCallActualEvidencePhaseV1::SourceStatic(staged) => {
                    if !Rc::ptr_eq(&staged.source, original)
                        || !staged.candidates.is_empty()
                        || !staged.integer_evidence.is_empty()
                    {
                        return Err(freeze("static-zero/original-actuals"));
                    }
                    let finish = StaticZeroInputFinishV1 {
                        source: Rc::clone(original),
                        completion: Rc::clone(completion),
                    };
                    let regenerated = construct_borrowed_call_actuals_v1(
                        source,
                        call,
                        std::slice::from_ref(contract),
                        &call.call,
                        &staged.candidates,
                        &[],
                        None,
                        &mut |_| None,
                        Some(finish),
                    )?
                    .ok_or_else(|| freeze("static-zero/constructor-unavailable"))?;
                    if regenerated.ordered_arguments != actuals.ordered_arguments {
                        return Err(freeze("static-zero/argument-snapshot-drift"));
                    }
                    replacements.push((call.call.clone(), regenerated));
                }
                BorrowedCallActualEvidencePhaseV1::ExecutableStaticZero(finish) => {
                    if !finish.corroborates_source_v1(original)
                        || !Rc::ptr_eq(&finish.completion, completion)
                    {
                        return Err(freeze("static-zero/completion-drift"));
                    }
                    actuals.ordered_arguments_for_v1(call)?;
                }
                _ => return Err(freeze("static-zero/actual-phase")),
            }
        }
        Ok(if missing {
            TypedInputClosureV1::Pending
        } else {
            TypedInputClosureV1::Ready(replacements)
        })
    }
}

#[cfg(test)]
#[path = "ordinary_new_borrowed_static_input_finish_tests.rs"]
mod tests;
