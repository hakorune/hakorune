//! Whole Static input closure uses the original inventory, signature and Completion.
//! Finished evidence stays in the existing actual row; no borrowed entry is issued.
use super::static_input_validation::checked_terminal_static_literal_v1;
use super::*;
use crate::mir::normal_callable_semantic_package::{
    physical_signature::VerifiedCallablePhysicalSignatureCohortV1,
    qualified_static_call_claim::incoming_source::StaticIncomingSourceV1,
    result_contract::VerifiedCallableResultContractCohortV1,
    selected_mapping::VerifiedSelectedCallableBatchMapV1, OrdinaryNewClaimLedgerV1,
};
use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;

pub(super) type StaticInputRowsV1 = Vec<(OwnedExprSiteV1, PreparedBorrowedCallActualsV1)>;

#[derive(Debug, Clone)]
pub(super) struct StaticInputFinishV1 {
    pub(super) source: Rc<StaticIncomingSourceV1>,
    pub(super) completion: Rc<crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1>,
    pub(super) evidence: StaticInputEvidenceV1,
    pub(super) text_result: Option<
        crate::mir::normal_callable_semantic_package::result_contract::CompletedCallResultSourceV1,
    >,
}
#[derive(Debug, Clone)]
pub(super) enum StaticInputEvidenceV1 {
    Forwarded,
    Numeric(super::static_numeric_evidence::StaticScalarActualV1),
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
            (Self::Forwarded, Self::Forwarded) => true,
            (Self::Numeric(a), Self::Numeric(b)) => a == b,
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
    pub(super) fn finish_arguments_v1(
        source: &Rc<StaticIncomingSourceV1>,
        completion: &Rc<crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1>,
        snapshot: &PreparedBorrowedCallActualsV1,
        evidence: StaticInputEvidenceV1,
        actuals: Box<[PreparedBorrowedFormalActualV1]>,
        text_result: Option<
            crate::mir::normal_callable_semantic_package::result_contract::CompletedCallResultSourceV1,
        >,
    ) -> PreparedBorrowedCallActualsV1 {
        PreparedBorrowedCallActualsV1 {
            phase: BorrowedCallActualEvidencePhaseV1::ExecutableStaticInput(Self {
                source: Rc::clone(source),
                completion: Rc::clone(completion),
                evidence,
                text_result,
            }),
            opaque_actuals: actuals,
            ordered_arguments: snapshot.ordered_arguments.clone(),
            dynamic_invocation_arguments: None,
        }
    }

    pub(super) fn corroborates_snapshot_v1(
        &self,
        original: &Rc<StaticIncomingSourceV1>,
        completion: &Rc<crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1>,
        snapshot: &PreparedBorrowedCallActualsV1,
        rows: &PreparedBorrowedCallActualsV1,
    ) -> bool {
        Rc::ptr_eq(&self.source, original)
            && Rc::ptr_eq(&self.completion, completion)
            && matches!(&snapshot.phase, BorrowedCallActualEvidencePhaseV1::SourceStatic(identity)
                if Rc::ptr_eq(&identity.source, original))
            && rows.ordered_arguments == snapshot.ordered_arguments
    }

    pub(super) fn corroborates_forwarded_v1(&self, original: &Rc<StaticIncomingSourceV1>) -> bool {
        matches!(self.evidence, StaticInputEvidenceV1::Forwarded)
            && Rc::ptr_eq(&self.source, original)
            && self.completion.owner() == original.callee_owner()
    }

    pub(super) fn source_snapshot_v1(&self) -> Option<&PreparedBorrowedCallActualsV1> {
        match &self.evidence {
            StaticInputEvidenceV1::Qualified { snapshot, .. } => Some(snapshot),
            _ => None,
        }
    }
}

fn static_input_contract_v1(
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    owner: FunctionOwnerIdV1,
) -> Result<&OwnedCallableParameterContractDeclarationV1, String> {
    let contract = OwnedCallableParameterContractDeclarationV1::unique_for_owner(
        contracts, owner,
        || freeze("static-input/contract-missing"),
        || freeze("static-input/contract-duplicate"),
    )?;
    Ok(contract)
}

impl OrdinaryNewClaimLedgerV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn finish_static_input_actuals_v1(
        &mut self,
        batch: &VerifiedResolvedCallableSemanticBatchV1,
        selected: &VerifiedSelectedCallableBatchMapV1,
        contracts: &[OwnedCallableParameterContractDeclarationV1],
        signatures: &VerifiedCallablePhysicalSignatureCohortV1,
        results: &VerifiedCallableResultContractCohortV1,
    ) -> Result<(), String> {
        for (site, actuals) in
            self.prepare_static_forwarded_inputs_v1(selected, contracts, signatures, results)?
        {
            self.borrowed_formal_actuals.insert(site, Ok(actuals));
        }
        for (site, actuals) in
            self.prepare_numeric_static_inputs_v1(selected, contracts, signatures, results)?
        {
            self.borrowed_formal_actuals.insert(site, Ok(actuals));
        }
        self.finish_static_bool_entry_actual_v1(batch, selected, contracts, signatures, results)?;
        self.finish_static_input_cohorts_v1(selected, contracts, signatures, results)
    }

    pub(in crate::mir::normal_callable_semantic_package) fn finish_static_input_cohorts_v1(
        &mut self,
        selected: &VerifiedSelectedCallableBatchMapV1,
        contracts: &[OwnedCallableParameterContractDeclarationV1],
        signatures: &VerifiedCallablePhysicalSignatureCohortV1,
        results: &VerifiedCallableResultContractCohortV1,
    ) -> Result<(), String> {
        let Some(Ok(source)) = self.borrowed_formal_source.as_ref() else {
            return Ok(());
        };
        let owners: BTreeSet<_> = source
            .source_incoming
            .source_rows_v1()
            .filter(|call| self.is_static_expression_input_owner_v1(source, call.callee))
            .map(|call| call.callee)
            .collect();
        for owner in owners {
            let contract = static_input_contract_v1(contracts, owner)?;
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
            let prepared = self.prepare_static_expression_input_owner_v1(
                source, selected, contract, signatures, results,
            )?;
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
        let tagged = self.uses_tagged_static_inputs_v1(source, contract);
        for call in incoming.iter().filter(|call| {
            !tagged || self.static_input_caller_owes_actuals_v1(call)
        }) {
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
            // Every caller keeps its original identity check above. Only a caller
            // recorded as omitted from this closed-app artifact may lack its row.
            let Some(rows) = self.borrowed_formal_actuals.get(&call.call) else {
                missing |= self.static_input_caller_owes_actuals_v1(call);
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
                        StaticInputFinishV1::finish_arguments_v1(
                            original,
                            completion,
                            snapshot,
                            StaticInputEvidenceV1::Qualified {
                                snapshot: Box::new(snapshot.clone()),
                                uses: uses.clone(),
                                domains: domains.clone(),
                            },
                            actuals.into_boxed_slice(),
                            text_result,
                        ),
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
                    if !proof.corroborates_snapshot_v1(original, completion, snapshot, rows)
                        || retained.as_ref().map(Rc::as_ptr) != uses.as_ref().map(Rc::as_ptr)
                        || retained_domains != &domains
                        || rows.opaque_actuals.as_ref() != actuals.as_slice()
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
}

#[cfg(test)]
#[path = "ordinary_new_borrowed_static_input_finish_tests.rs"]
mod tests;
