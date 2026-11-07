//! Whole Static input closure uses the original inventory, signature and Completion.
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
pub(super) struct StaticInputFinishV1 {
    pub(super) source: Rc<StaticIncomingSourceV1>,
    pub(super) completion: Rc<crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1>,
    pub(super) evidence: StaticInputEvidenceV1,
    pub(super) text_result: Option<
        crate::mir::normal_callable_semantic_package::result_contract::CompletedTextResultSourceV1,
    >,
}
#[derive(Debug, Clone)]
pub(super) enum StaticInputEvidenceV1 {
    Zero,
    Qualified {
        snapshot: Box<PreparedBorrowedCallActualsV1>,
        /// `None` when the callee has no borrowed formal: every formal is an i64 actual (K21).
        uses: Option<Rc<super::super::borrowed_formal_uses::OriginalFormalUseSourceV1>>,
        domains: Box<[super::super::borrowed_formal_entry::BorrowedFormalDomainV1]>,
    },
}
impl PartialEq for StaticInputEvidenceV1 {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Zero, Self::Zero) => true,
            (
                Self::Qualified {
                    snapshot: a,
                    uses: au,
                    domains: ad,
                },
                Self::Qualified {
                    snapshot: b,
                    uses: bu,
                    domains: bd,
                },
            ) => a == b && au.as_ref().map(Rc::as_ptr) == bu.as_ref().map(Rc::as_ptr) && ad == bd,
            _ => false,
        }
    }
}
impl Eq for StaticInputEvidenceV1 {}
impl PartialEq for StaticInputFinishV1 {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.source, &other.source)
            && Rc::ptr_eq(&self.completion, &other.completion)
            && self.evidence == other.evidence
            && self.text_result == other.text_result
    }
}
impl Eq for StaticInputFinishV1 {}
impl StaticInputFinishV1 {
    pub(super) fn corroborates_source_v1(&self, source: &Rc<StaticIncomingSourceV1>) -> bool {
        Rc::ptr_eq(&self.source, source)
            && matches!(self.evidence, StaticInputEvidenceV1::Zero)
            && source.is_zero_input_source_v1()
            && self.completion.owner() == source.callee_owner()
    }
    pub(super) fn source_snapshot_v1(&self) -> Option<&PreparedBorrowedCallActualsV1> {
        match &self.evidence {
            StaticInputEvidenceV1::Qualified { snapshot, .. } => Some(snapshot),
            _ => None,
        }
    }
}

/// Text borrows the full original census; the existing I64 lane still requires
/// its original executable observation. Neither branch reconstructs a source.
pub(super) fn original_zero_input_source_v1<'a>(
    prepared: &'a PreparedBorrowedFormalIngressV1,
    original: &StaticIncomingSourceV1,
) -> Result<Option<&'a Rc<StaticIncomingSourceV1>>, String> {
    if original.is_zeroarg_text_v1() {
        super::static_source::original_static_source_v1(prepared, original.call_site())
    } else {
        prepared
            .source_incoming
            .static_observations()
            .get(original.call_site())
            .map(|row| row.as_ref().map_err(Clone::clone))
            .transpose()
    }
}

impl OrdinaryNewClaimLedgerV1 {


    pub(in crate::mir::normal_callable_semantic_package) fn finish_static_input_actuals_v1(
        &mut self,
        selected: &VerifiedSelectedCallableBatchMapV1,
        contracts: &[OwnedCallableParameterContractDeclarationV1],
        signatures: &VerifiedCallablePhysicalSignatureCohortV1,
        results: &VerifiedCallableResultContractCohortV1,
    ) -> Result<(), String> {
        let Some(Ok(source)) = self.borrowed_formal_source.as_ref() else {
            return Ok(());
        };
        // Preserve the original I64 executable census; Text uses the complete
        // original source census, including a still-passive sibling.
        let zero_i64 = source
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
            });
        let zero = zero_i64.chain(source.source_incoming.source_rows_v1().filter_map(|call| {
            match &call.source {
                super::super::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(original)
                    if original.is_zeroarg_text_v1()
                        && selected
                            .key_for_batch_slot(original.target_batch_slot())
                            .is_some() =>
                {
                    Some(call.callee)
                }
                _ => None,
            }
        }));
        let qualified = source
            .source_incoming
            .source_rows_v1()
            .filter(|call| self.is_static_expression_input_owner_v1(source, call.callee))
            .map(|call| call.callee);
        let owners: std::collections::BTreeSet<_> = zero.chain(qualified).collect();
        for owner in owners {
            let mut matching = contracts.iter().filter(|row| row.owner == owner);
            let contract = matching
                .next()
                .ok_or_else(|| freeze("static-zero/contract-missing"))?;
            if matching.next().is_some() {
                return Err(freeze("static-zero/contract-duplicate"));
            }
            let prepared = if self.is_static_expression_input_owner_v1(source, owner) {
                // Preserve original route membership even while Completion is
                // pending. Demand must not retry a generic Static writer.
                let incoming = source
                    .source_incoming
                    .project_source_v1(&[owner].into_iter().collect())
                    .map_err(|issue| {
                        format!("{}: {issue:?}", freeze("static-input/source-cohort-veto"))
                    })?;
                let sites: Vec<_> = incoming
                    .iter()
                    .filter(|call| {
                        self.local_call_for_owner(call.call.owner(), call.call.site())
                            .is_some()
                    })
                    .map(|call| call.call.clone())
                    .collect();
                if let Some(Ok(selected)) = self.borrowed_static_source_sites.as_mut() {
                    selected.extend(sites);
                }
                self.prepare_static_expression_input_owner_v1(
                    source, selected, contract, signatures, results,
                )?
            } else {
                self.prepare_static_zero_input_owner_v1(
                    source, selected, contract, signatures, results,
                )?
            };
            match prepared {
                TypedInputClosureV1::Pending => {}
                TypedInputClosureV1::Refused(issue) => {
                    for call in source
                        .source_incoming
                        .source_rows_v1()
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



















    fn prepare_static_expression_input_owner_v1(
        &self,
        source: &PreparedBorrowedFormalIngressV1,
        selected: &VerifiedSelectedCallableBatchMapV1,
        contract: &OwnedCallableParameterContractDeclarationV1,
        signatures: &VerifiedCallablePhysicalSignatureCohortV1,
        results: &VerifiedCallableResultContractCohortV1,
    ) -> Result<TypedInputClosureV1, String> {
        let owner = contract.owner;
        let debug_pending = |reason: &str| {
            if crate::config::env::builder_debug_enabled() {
                eprintln!("[static-input/pending] owner={owner:?} reason={reason}");
            }
        };
        if !source
            .source_incoming
            .source_registered_owners_v1()
            .contains(&owner)
            || source
                .source_incoming
                .sum_vetoed_owners_v1()
                .contains(&owner)
        {
            debug_pending("source-owner");
            return Ok(TypedInputClosureV1::Pending);
        }
        let Some(completion) = self.completion_index.get(&owner) else {
            debug_pending("completion-missing");
            return Ok(TypedInputClosureV1::Pending);
        };
        let completion = match completion {
            Ok(completion) => completion,
            Err(error) => {
                return Ok(TypedInputClosureV1::Refused(format!(
                    "{}: {error:?}",
                    freeze("static-input/callee-completion")
                )))
            }
        };
        if !self.verify_static_input_header_v1(
            selected,
            contract,
            signatures,
            results,
            completion,
            "static-input",
        )? {
            debug_pending("header");
            return Ok(TypedInputClosureV1::Pending);
        }
        let Some(uses) = self.checked_static_input_uses_v1(contract)? else {
            debug_pending("uses");
            return Ok(TypedInputClosureV1::Pending);
        };
        let Some(domains) = self.static_input_domains_v1(source, contract) else {
            debug_pending("domains");
            return Ok(TypedInputClosureV1::Pending);
        };
        let Some(SelectedNormalCallableKeyV1::Cataloged(key)) =
            selected.key_for_batch_slot(contract.batch_slot)
        else {
            return Err(freeze("static-input/selected-key"));
        };
        let mut seen = std::collections::BTreeSet::new();
        let mut missing = false;
        let mut replacements = Vec::new();
        let incoming = source
            .source_incoming
            .project_source_v1(&[owner].into_iter().collect())
            .map_err(|issue| format!("{}: {issue:?}", freeze("static-input/source-cohort-veto")))?;
        for call in &incoming {
            if !seen.insert(call.call.clone()) {
                return Err(freeze("static-input/original-row-duplicate"));
            }
            let super::super::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(original) =
                &call.source
            else {
                return Err(freeze("static-input/original-source-kind"));
            };
            if original.call_site() != &call.call
                || original.callee_owner() != owner
                || original.target_batch_slot() != contract.batch_slot
                || original.target() != key
                || original.parameters() != contract.parameters.as_ref()
                || original.argument_sites().len() != contract.parameters.len()
            {
                return Err(freeze("static-input/original-target"));
            }
            let text_result = results
                .row(contract.batch_slot)
                .and_then(|row| row.completed_text_source_v1(selected, original));
            if matches!(original.source_result(), crate::mir::callable_result_representation::VerifiedCallableResultDispositionV1::ExactString)
                && text_result.is_none() { debug_pending("text-result"); return Ok(TypedInputClosureV1::Pending); }
            let Some(rows) = self.borrowed_formal_actuals.get(&call.call) else {
                missing = true;
                continue;
            };
            let rows = match rows {
                Ok(rows) => rows,
                Err(issue) => return Ok(TypedInputClosureV1::Refused(issue.clone())),
            };
            let Some(snapshot) = rows.source_static_snapshot_v1() else {
                missing = true;
                continue;
            };
            let actuals = self.checked_static_expression_actuals_v1(source, call, snapshot)?;
            match &rows.phase {
                BorrowedCallActualEvidencePhaseV1::SourceStatic(_) => {
                    replacements.push((
                        call.call.clone(),
                        PreparedBorrowedCallActualsV1 {
                            phase: BorrowedCallActualEvidencePhaseV1::ExecutableStaticInput(
                                StaticInputFinishV1 {
                                    source: Rc::clone(original),
                                    completion: Rc::clone(completion),
                                    text_result,
                                    evidence: StaticInputEvidenceV1::Qualified {
                                        snapshot: Box::new(snapshot.clone()),
                                        uses: uses.clone(),
                                        domains: domains.clone(),
                                    },
                                },
                            ),
                            opaque_actuals: actuals.into_boxed_slice(),
                            ordered_arguments: snapshot.ordered_arguments.clone(),
                            dynamic_invocation_arguments: None,
                        },
                    ));
                }
                BorrowedCallActualEvidencePhaseV1::ExecutableStaticInput(proof) => {
                    let StaticInputEvidenceV1::Qualified {
                        uses: retained,
                        domains: retained_domains,
                        ..
                    } = &proof.evidence
                    else {
                        return Err(freeze("static-input/evidence-kind"));
                    };
                    if !Rc::ptr_eq(&proof.source, original)
                        || !Rc::ptr_eq(&proof.completion, completion)
                        || retained.as_ref().map(Rc::as_ptr) != uses.as_ref().map(Rc::as_ptr)
                        || retained_domains != &domains
                        || rows.opaque_actuals.as_ref() != actuals.as_slice()
                        || rows.ordered_arguments != snapshot.ordered_arguments
                    {
                        return Err(freeze("static-input/completed-source-drift"));
                    }
                    rows.ordered_arguments_for_v1(call)?;
                }
                _ => return Err(freeze("static-input/actual-phase")),
            }
        }
        if missing || seen.is_empty() {
            debug_pending(if missing {
                "incoming-actuals"
            } else {
                "empty-incoming"
            });
        }
        Ok(if missing || seen.is_empty() {
            TypedInputClosureV1::Pending
        } else {
            TypedInputClosureV1::Ready(replacements)
        })
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
        let owners = [owner].into_iter().collect();
        let text = source.source_incoming.source_rows_v1().any(|row| row.callee == owner
            && matches!(&row.source, super::super::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(original)
                if original.is_zeroarg_text_v1()));
        let calls = match if text {
            source.source_incoming.project_source_v1(&owners)
        } else {
            source.source_incoming.project(&owners)
        } {
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
        if !self.verify_static_input_header_v1(
            selected,
            contract,
            signatures,
            results,
            completion,
            "static-zero",
        )? {
            return Ok(TypedInputClosureV1::Pending);
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
            let retained = original_zero_input_source_v1(source, original)?
                .ok_or_else(|| freeze("static-zero/original-source-missing"))?;
            if call.callee != owner
                || !Rc::ptr_eq(original, retained)
                || !original.is_zero_input_source_v1()
                || original.callee_owner() != owner
                || original.target() != key
                || original.target_batch_slot() != contract.batch_slot
                || original.call_site() != &call.call
                || !call.arguments.is_empty()
            {
                return Err(freeze("static-zero/original-target"));
            }
            let text_result = results
                .row(contract.batch_slot)
                .and_then(|row| row.completed_text_source_v1(selected, original));
            if matches!(original.source_result(), crate::mir::callable_result_representation::VerifiedCallableResultDispositionV1::ExactString)
                && text_result.is_none() { return Ok(TypedInputClosureV1::Pending); }
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
                    let finish = StaticInputFinishV1 {
                        source: Rc::clone(original),
                        completion: Rc::clone(completion),
                        evidence: StaticInputEvidenceV1::Zero,
                        text_result,
                    };
                    let regenerated = construct_borrowed_call_actuals_v1(
                        source,
                        call,
                        std::slice::from_ref(contract),
                        &call.call,
                        &staged.candidates,
                        object_source::ObjectActualHomeSourceV1::Candidates(&[]),
                        None,
                        &mut |_| None,
                        Some(finish),
                        None,
                        &mut |_, _, _| Ok(None),
                    )?
                    .ok_or_else(|| freeze("static-zero/constructor-unavailable"))?;
                    if regenerated.ordered_arguments != actuals.ordered_arguments {
                        return Err(freeze("static-zero/argument-snapshot-drift"));
                    }
                    replacements.push((call.call.clone(), regenerated));
                }
                BorrowedCallActualEvidencePhaseV1::ExecutableStaticInput(finish) => {
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
