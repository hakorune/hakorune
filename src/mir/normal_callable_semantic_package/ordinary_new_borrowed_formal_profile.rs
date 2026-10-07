//! One source/result preparation: Pending eligibility precedes stored receiver issuance.
use super::super::OrdinaryNewCoSealIssueV1;
use super::borrowed_formal_result::{
    grounded_source_results_v1, prepare_pending_results_v1, seal_pending_results_v1,
    stored_eligible_v1, BorrowedI64ResultSourceV1,
};
use super::borrowed_formal_source::{
    collect_borrowed_source_drafts_v1, finish_ingress_from_drafts_v1,
};
use super::source::{PreparedSourceCallNeedV1, StoredReceiverSourceV1};
use super::*;
use crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractDeclarationV1;
use crate::mir::normal_callable_semantic_package::VerifiedInstanceConstructorSemanticBatchV1;
use std::collections::{BTreeMap, BTreeSet};

#[allow(clippy::too_many_arguments)]
pub(in crate::mir::normal_callable_semantic_package) fn prepare_borrowed_profile_v1(
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    selected: &VerifiedSelectedCallableBatchMapV1,
    constructors: &VerifiedInstanceConstructorSemanticBatchV1,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    static_call_claims: &crate::mir::normal_callable_semantic_package::qualified_static_call_claim::QualifiedStaticCallClaimIndexV1,
    app_main_slot: Option<u32>,
    app_main: Option<&BorrowedAppMainSourceLoanV1<'_>>,
    dynamic_slot: Option<u32>,
    entry_home_loans: &crate::mir::resolved_semantics::VerifiedInstanceEntryHomeCatalogV1,
    local_candidates: &BTreeMap<
        u32,
        Result<
            Vec<super::super::candidate::OrdinaryNewCandidate>,
            super::super::OrdinaryNewCoSealIssueV1,
        >,
    >,
    new_classes: &BTreeMap<OwnedExprSiteV1, Box<str>>,
    names: &[Box<str>],
    field_write_claims: &super::super::field_write_claim::OrdinaryNewFieldWriteClaimsV1,
    callable_result_classes: &super::super::result_class_claim::OrdinaryNewResultClassClaimsV1,
    array_i64_fields: &BTreeMap<Box<str>, BTreeSet<hakorune_mir_defs::CanonicalFieldRefV1>>,
    probe: &mut impl FnMut(
        u32,
        crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
        &crate::mir::resolved_semantics::VerifiedResolvedMethodCallSourceV1,
    ) -> Result<Option<StoredReceiverSourceV1>, String>,
    issue: &mut impl FnMut(
        &OwnedExprSiteV1,
        &StoredReceiverSourceV1,
    ) -> Result<Option<LexicalInstanceCallReceiverV1>, String>,
) -> Result<
    (
        PreparedLexicalInstanceCallSourceTargetsV1,
        Result<PreparedBorrowedFormalIngressV1, String>,
        Result<BTreeSet<OwnedExprSiteV1>, String>,
        BTreeMap<FunctionOwnerIdV1, Result<BorrowedI64ResultSourceV1, String>>,
    ),
    OrdinaryNewCoSealIssueV1,
> {
    let needs = source::prepare_lexical_source_targets_v1(
        batch,
        selected,
        new_classes,
        names,
        field_write_claims,
        callable_result_classes,
        probe,
    );
    let drafts = collect_borrowed_source_drafts_v1(
        batch,
        selected,
        contracts,
        Some(static_call_claims),
        app_main_slot,
        dynamic_slot,
        entry_home_loans,
        constructors,
    );
    let (ordinary_callers, definitions, dominated_view_sites) = match drafts {
        Ok(rows) => rows,
        Err(error) => {
            return Ok((
                Err(error.clone()),
                Err(error.clone()),
                Err(error),
                BTreeMap::new(),
            ))
        }
    };
    // Retain exact dispatch membership before later ingress failures. This is
    // a projection of the original facts, never a second classifier or grant.
    let static_arguments = match super::borrowed_static_argument::collect_static_argument_sources_v1(
        batch,
        selected,
        contracts,
        &definitions,
        Some(static_call_claims),
        app_main,
    ) {
        Ok(rows) => rows,
        Err(error) => {
            return Ok((
                Err(error.clone()),
                Err(error.clone()),
                Err(error),
                BTreeMap::new(),
            ))
        }
    };
    let static_source_sites = Ok(static_arguments
        .keys()
        .map(|(site, _)| site.clone())
        .collect());
    let pending = prepare_pending_results_v1(
        batch,
        selected,
        contracts,
        &definitions,
        &needs,
        constructors,
        callable_result_classes,
        array_i64_fields,
    );
    let grounded = grounded_source_results_v1(&pending);
    let targets: PreparedLexicalInstanceCallSourceTargetsV1 = match needs {
        Err(issue) => Err(issue),
        Ok(needs) => {
            let mut targets = Vec::new();
            for row in needs {
                let target = match row {
                    Err(issue) => Err(issue),
                    Ok(None) => Ok(None),
                    Ok(Some(need)) => {
                        if need.stored().is_some()
                            && !stored_eligible_v1(
                                &need,
                                &pending,
                                &grounded,
                                &definitions,
                                contracts,
                            )
                        {
                            targets.push(Ok(None));
                            continue;
                        }
                        match need {
                            PreparedSourceCallNeedV1::Lexical(target) => Ok(Some(target)),
                            PreparedSourceCallNeedV1::Stored {
                                reference,
                                receiver,
                            } => {
                                let receiver = issue_eligible_receiver_v1(
                                    &reference.call_site,
                                    &receiver,
                                    issue,
                                )?;
                                Ok(Some(LexicalInstanceCallSourceTargetV1 {
                                    call_site: reference.call_site,
                                    receiver_site: reference.receiver_site,
                                    receiver,
                                    target: reference.target,
                                    target_batch_slot: reference.target_batch_slot,
                                    callee_owner: reference.callee_owner,
                                    argument_sites: reference.argument_sites,
                                    result_requirement:
                                        LexicalCallSourceResultRequirementV1::ExistingBorrowedResult,
                                }))
                            }
                        }
                    }
                };
                targets.push(target);
            }
            Ok(targets)
        }
    };
    let ingress = (|| {
        let mut calls = BTreeMap::new();
        for row in targets.as_ref().map_err(Clone::clone)? {
            if let Some(row) = row.as_ref().map_err(Clone::clone)? {
                if calls.insert(row.call_site().clone(), row).is_some() {
                    return Err(freeze("borrowed-formal/duplicate-source-call"));
                }
            }
        }
        finish_ingress_from_drafts_v1(
            batch,
            selected,
            contracts,
            entry_home_loans,
            constructors,
            local_candidates,
            callable_result_classes,
            calls,
            Some(static_call_claims),
            app_main,
            (ordinary_callers, definitions, dominated_view_sites),
            static_arguments,
        )
    })();
    let results = seal_pending_results_v1(pending, &ingress, batch, constructors);
    Ok((targets, ingress, static_source_sites, results))
}

/// After eligibility, retain the exact demand instead of erasing it into a row error.
fn issue_eligible_receiver_v1(
    site: &OwnedExprSiteV1,
    receiver: &StoredReceiverSourceV1,
    issue: &mut impl FnMut(
        &OwnedExprSiteV1,
        &StoredReceiverSourceV1,
    ) -> Result<Option<LexicalInstanceCallReceiverV1>, String>,
) -> Result<LexicalInstanceCallReceiverV1, OrdinaryNewCoSealIssueV1> {
    issue(site, receiver)
        .and_then(|row| row.ok_or_else(|| freeze("stored-child/receiver-unavailable")))
        .map_err(|issue| OrdinaryNewCoSealIssueV1::BorrowedFormalIngress {
            site: site.clone(),
            issue,
        })
}
