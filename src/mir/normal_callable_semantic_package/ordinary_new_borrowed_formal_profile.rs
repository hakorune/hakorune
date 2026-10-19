//! One source/result preparation: Pending eligibility precedes stored receiver issuance.
use crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call::BorrowedResultCohortV1;
use super::super::OrdinaryNewCoSealIssueV1;
use super::borrowed_formal_result::{
    grounded_source_results_v1, prepare_pending_results_v1, seal_pending_results_v1,
    stored_eligible_v1,
};
use super::borrowed_formal_source::{
    collect_borrowed_source_drafts_v1, finish_ingress_from_drafts_v1, prepare_original_incoming_v1,
};
use super::source::{PreparedSourceCallNeedV1, StoredReceiverSourceV1};
use super::*;
use crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractDeclarationV1;
use crate::mir::normal_callable_semantic_package::VerifiedInstanceConstructorSemanticBatchV1;
use std::collections::{BTreeMap, BTreeSet};

#[path = "ordinary_new_borrowed_stored_nullable_source.rs"]
mod stored_nullable;

fn attach_owning_object_local_input_v1(
    need: &mut PreparedSourceCallNeedV1,
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    constructors: &VerifiedInstanceConstructorSemanticBatchV1,
    selected: &VerifiedSelectedCallableBatchMapV1,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    facts: &super::super::result_class_claim::OrdinaryNewResultClassClaimsV1,
    app_main: Option<&BorrowedAppMainSourceLoanV1<'_>>,
) -> Result<(), String> {
    let reference = need.reference();
    if !matches!(facts.get(&reference.target), Some(super::super::result_class_claim::OrdinaryNewResultClassV1::Object(_)
        | super::super::result_class_claim::OrdinaryNewResultClassV1::NullableObject(_))) {
        return Ok(());
    }
    let destination = crate::mir::normal_callable_semantic_package::original_call_source::with_original_source_input_v1(
        batch, constructors, reference.call_site.owner(), |_, input| {
            let mut rows = input.function().expression_source().initializers()
                .filter(|row| row.initializer_site() == Some(reference.call_site.site()));
            let destination = rows.next().map(|row| row.binding());
            if rows.next().is_some() { return Err(freeze("nullable-input/initializer-not-unique")); }
            Ok(destination)
        })??;
    let Some(destination) = destination else { return Ok(()); };
    let loan = facts.owning_object_local_input_source_v1(batch, selected, contracts,
        &reference, destination, app_main.map(|main| main.caller_source_ref_v1()))
        .map_err(freeze)?.ok_or_else(|| freeze("nullable-input/source-missing"))?;
    if loan.key() != &reference.target || loan.callee_owner() != reference.callee_owner
        || loan.receiver() != Some(&reference.receiver_site)
        || !loan.matches_arguments_v1(&reference.argument_sites) {
        return Err(freeze("nullable-input/original-target"));
    }
    need.set_owning_object_input_v1(std::rc::Rc::new(loan));
    Ok(())
}

fn original_mixed_local_loan_v1(
    need: &PreparedSourceCallNeedV1,
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    constructors: &VerifiedInstanceConstructorSemanticBatchV1,
    selected: &VerifiedSelectedCallableBatchMapV1,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    fields: &super::super::field_write_claim::OrdinaryNewFieldWriteClaimsV1,
    facts: &super::super::result_class_claim::OrdinaryNewResultClassClaimsV1,
 app_main: Option<&BorrowedAppMainSourceLoanV1<'_>>) -> Result<Option<super::super::result_class_claim::MixedLocalCallSourceLoanV1>, String> {
    use super::super::result_class_claim::ResultValueOriginV1;
    let reference = need.reference();
    let Some(exits) = facts.outcomes(&reference.target) else { return Ok(None); };
    if facts.get(&reference.target).is_some()
        || !exits.iter().flat_map(|row| row.witnesses()).any(|root| matches!(root.origin(), ResultValueOriginV1::Fresh(_)))
        || !exits.iter().flat_map(|row| row.witnesses()).any(|root| matches!(root.origin(), ResultValueOriginV1::ForwardFormal { .. }))
    { return Ok(None); }
    let destination = crate::mir::normal_callable_semantic_package::original_call_source::with_original_source_input_v1(
        batch, constructors, reference.call_site.owner(), |context, input| {
            let original = input.function().method_call(reference.call_site.site())
                .ok_or_else(|| freeze("mixed-local/method-source"))?;
            context.constructor_key_for_call(&reference.call_site, original)?;
            let mut rows = input.function().expression_source().initializers()
                .filter(|row| row.initializer_site() == Some(reference.call_site.site()));
            let destination = rows.next().map(|row| row.binding());
            if rows.next().is_some() { return Err(freeze("mixed-local/initializer-not-unique")); }
            Ok(destination)
        })??;
    let Some(destination) = destination else { return Ok(None); };
    let loan = facts.mixed_local_call_source_v1(batch, selected, constructors, fields, contracts,
        &reference.call_site, destination, app_main.map(|main| main.caller_source_ref_v1())).map_err(freeze)?
        .ok_or_else(|| freeze("mixed-local/source-missing"))?;
    if loan.key() != &reference.target || loan.callee_owner() != reference.callee_owner
        || loan.receiver() != &reference.receiver_site
    { return Err(freeze("mixed-local/target-identity")); }
    Ok(Some(loan))
}

#[allow(clippy::too_many_arguments)]
pub(in crate::mir::normal_callable_semantic_package) fn prepare_borrowed_profile_v1(
    brands: Option<&crate::analysis::brand_program_declaration_catalog::VerifiedBrandProgramDeclarationCatalogV1>,
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    selected: &VerifiedSelectedCallableBatchMapV1,
    omitted_static_callers: &BTreeMap<FunctionOwnerIdV1, crate::mir::builder::CanonicalSameModuleCallableKeyV1>,
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
        crate::mir::normal_callable_semantic_package::original_call_source::OriginalCallSourceCallerV1<'_>,
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
        BorrowedResultCohortV1,
    ),
    OrdinaryNewCoSealIssueV1,
> {
    let mut needs = prepare_lexical_source_targets_v1(
        batch,
        selected,
        new_classes,
        names,
        field_write_claims,
        callable_result_classes,
        probe,
        Some(constructors),
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
        Some((callable_result_classes, field_write_claims)),
    );
    let (ordinary_callers, definitions, dominated_view_sites, guarded_actuals, originals) = match drafts {
        Ok(rows) => rows,
        Err(error) => {
            return Ok((
                Err(error.clone()),
                Err(error.clone()),
                Err(error.clone()),
                BorrowedResultCohortV1::unavailable(error),
            ))
        }
    };
    let original_returns = super::borrowed_formal_result::original::prepare_original_returns_v1(batch, selected, contracts);
    // Attach original terminal qualifications before the one incoming inventory.
    // Missing forward drafts remain explicit; they never become an empty proof.
    if let Ok(rows) = &mut needs {
        for row in rows {
            let Ok(Some(need)) = row else {
                continue;
            };
            let reference = need.reference();
            if let Some(loan) = original_mixed_local_loan_v1(need, batch, constructors, selected,
                contracts, field_write_claims, callable_result_classes, app_main)
                .map_err(|issue| OrdinaryNewCoSealIssueV1::BorrowedFormalIngress {
                    site: reference.call_site.clone(), issue,
                })? {
                need.set_result_requirement(LexicalCallSourceResultRequirementV1::MixedLocal(Box::new(loan)));
                continue;
            }
            if let Some(loan) = stored_nullable::original_local_loan_v1(
                need,
                batch,
                constructors,
                selected,
                contracts,
                field_write_claims,
                callable_result_classes,
             app_main.map(|main| main.caller_source_ref_v1()))
            .map_err(|issue| OrdinaryNewCoSealIssueV1::BorrowedFormalIngress {
                site: reference.call_site.clone(),
                issue,
            })? {
                need.set_result_requirement(
                    LexicalCallSourceResultRequirementV1::BorrowedNullableLocal(Box::new(loan)),
                );
                continue;
            }
            attach_owning_object_local_input_v1(need, batch, constructors, selected, contracts,
                callable_result_classes, app_main)
                .map_err(|issue| OrdinaryNewCoSealIssueV1::BorrowedFormalIngress {
                    site: reference.call_site.clone(), issue,
                })?;
            let qualifications = callable_result_classes
                .qualifications_for_call(&reference.call_site, &reference.target);
            if qualifications.is_empty() {
                if let Some(dependencies) = callable_result_classes
                    .object_return_dependencies(&reference.target, reference.callee_owner)
                {
                    need.set_result_requirement(
                        LexicalCallSourceResultRequirementV1::ObjectProducerDependency(
                            dependencies,
                        ),
                    );
                } else if let Some(original) = original_returns.get(&reference.callee_owner) {
                    if let Some(original) = original.as_ref().map_err(|issue| OrdinaryNewCoSealIssueV1::BorrowedFormalIngress {
                        site: reference.call_site.clone(), issue: issue.clone(),
                    })? {
                        if !original.matches(&reference) { return Err(OrdinaryNewCoSealIssueV1::BorrowedFormalIngress {
                            site: reference.call_site.clone(), issue: "original-return/target-identity".into(),
                        }); }
                        if original.is_unit_candidate_v1() {
                            let discard = crate::mir::normal_callable_semantic_package::original_call_source::with_original_source_input_v1(
                                batch, constructors, reference.call_site.owner(), |_, input| {
                                    input.body_shape().is_some_and(|shape| shape.statements().iter().any(|statement|
                                        matches!(statement, crate::mir::resolved_semantics::BodyStatementShapeV1::SequenceItem { site }
                                            if site.node() == reference.call_site.site().node())))
                                }).map_err(|issue| OrdinaryNewCoSealIssueV1::BorrowedFormalIngress {
                                    site: reference.call_site.clone(), issue,
                                })?;
                            if discard { need.set_result_requirement(LexicalCallSourceResultRequirementV1::UnitDiscardSource(original.clone())); }
                        } else if !original.is_literal_scalar_v1() || !definitions.contains_key(&reference.callee_owner) {
                            // A retained literal census does not replace the
                            // existing original borrowed-use family or grant ABI.
                            need.set_result_requirement(LexicalCallSourceResultRequirementV1::ScalarReturnSource(original.clone()));
                        }
                    }
                }
                continue;
            }
            let forwards = super::borrowed_formal_result::collect_observed_forward_identities_v1(
                need,
                contracts,
                &definitions,
            )
            .map(Vec::into_boxed_slice);
            need.set_result_requirement(LexicalCallSourceResultRequirementV1::ObjectReturnSource {
                qualifications,
                forwards,
            });
        }
    }
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
                Err(error.clone()),
                BorrowedResultCohortV1::unavailable_with_original_v1(error, original_returns),
            ))
        }
    };
    let mut static_source_sites: Result<BTreeSet<OwnedExprSiteV1>, String> = Ok(static_arguments
        .iter()
        .filter(|(_, fact)| fact.call_source().is_qualified_i64_source_v1())
        .map(|((site, _), _)| site.clone())
        .collect());
    // The source scan sees exact original Stored dispatch without pretending
    // its receiver is executable. The same inventory is completed below.
    let original_inventory = (|| {
        let mut lexical_calls = BTreeMap::new();
        for need in needs.as_ref().map_err(Clone::clone)? {
            if let Some(PreparedSourceCallNeedV1::Lexical(target)) = need.as_ref().map_err(Clone::clone)? {
                if lexical_calls.insert(target.call_site().clone(), target).is_some() {
                    return Err(freeze("borrowed-formal/duplicate-source-call"));
                }
            }
        }
        prepare_original_incoming_v1(
            brands, batch, selected, omitted_static_callers, contracts, &lexical_calls,
            &ordinary_callers, &definitions, Some(static_call_claims), &static_arguments,
            app_main, Some(&needs), constructors,
            entry_home_loans, local_candidates, callable_result_classes, &guarded_actuals, array_i64_fields,
        )
    })();
    let pending = prepare_pending_results_v1(
        batch,
        selected,
        contracts,
        &definitions,
        &needs,
        constructors,
        callable_result_classes,
        array_i64_fields,
        &original_inventory, field_write_claims, original_returns,
    );
    let grounded = grounded_source_results_v1(&pending);
    let targets: PreparedLexicalInstanceCallSourceTargetsV1 = match &needs {
        Err(issue) => Err(issue.clone()),
        Ok(needs) => {
            let mut targets = Vec::new();
            for row in needs {
                let target = match row {
                    Err(issue) => Err(issue.clone()),
                    Ok(None) => Ok(None),
                    Ok(Some(need)) => {
                        if need.stored().is_some() {
                            let eligible = if let Some(loan) = need.owning_object_input_v1() {
                                callable_result_classes.recheck_owning_object_local_input_roots_v1(loan)
                                    .map_err(|issue| OrdinaryNewCoSealIssueV1::BorrowedFormalIngress {
                                        site: need.reference().call_site, issue: freeze(issue),
                                    })?;
                                true
                            } else { match need.result_requirement() {
                                LexicalCallSourceResultRequirementV1::MixedLocal(loan) => {
                                    callable_result_classes.recheck_mixed_local_source_v1(batch, selected, constructors,
                                        field_write_claims, contracts, loan, app_main.map(|main| main.caller_source_ref_v1())).map_err(|issue|
                                        OrdinaryNewCoSealIssueV1::BorrowedFormalIngress {
                                            site: need.reference().call_site, issue: issue.into(),
                                        })?;
                                    true
                                }
                                LexicalCallSourceResultRequirementV1::BorrowedNullableLocal(
                                    loan,
                                ) => {
                                    stored_nullable::recheck_original_local_loan_v1(
                                        need,
                                        loan,
                                        batch,
                                        selected,
                                        contracts,
                                        field_write_claims,
                                        callable_result_classes,
                                     app_main.map(|main| main.caller_source_ref_v1()))
                                    .map_err(|issue| {
                                        OrdinaryNewCoSealIssueV1::BorrowedFormalIngress {
                                            site: need.reference().call_site,
                                            issue,
                                        }
                                    })?;
                                    true
                                }
                                LexicalCallSourceResultRequirementV1::ScalarReturnSource(original) => {
                                    let reference = need.reference();
                                    let proof = pending.get(&reference.callee_owner).ok_or_else(|| OrdinaryNewCoSealIssueV1::BorrowedFormalIngress {
                                        site: reference.call_site.clone(), issue: "original-return/result-missing".into(),
                                    })?.as_ref().map_err(|issue| OrdinaryNewCoSealIssueV1::BorrowedFormalIngress {
                                        site: reference.call_site.clone(), issue: issue.clone(),
                                    })?;
                                    if !original.matches(&reference) || !proof.matches_original_return_v1(original)
                                        || !grounded.contains(&reference.callee_owner) || original.scalar_class_v1() != Some(proof.class)
                                    { return Err(OrdinaryNewCoSealIssueV1::BorrowedFormalIngress {
                                        site: reference.call_site, issue: "original-return/source-unproven".into(),
                                    }); }
                                    true
                                }
                                LexicalCallSourceResultRequirementV1::UnitDiscardSource(original) => {
                                    let reference = need.reference();
                                    let constructor = need.stored().is_some_and(|receiver|
                                        matches!(receiver.origin, super::source::StoredReceiverSourceOriginV1::Constructor(_)));
                                    let declarations: Vec<_> = contracts.iter().filter(|row| row.owner == reference.callee_owner).collect();
                                    constructor && original.is_unit_candidate_v1() && original.matches(&reference)
                                        && reference.target.arity() == 0 && reference.argument_sites.is_empty()
                                        && declarations.len() == 1 && declarations[0].parameters.is_empty()
                                        && declarations[0].batch_slot == reference.target_batch_slot
                                        && declarations[0].mode == crate::mir::callable_parameter_contract::CallableParameterDeclarationModeV1::InstanceBoxMethod
                                },
                                LexicalCallSourceResultRequirementV1::ExistingBorrowedResult => {
                                    stored_eligible_v1(
                                        need,
                                        &pending,
                                        &grounded,
                                        &definitions,
                                        contracts,
                                    )
                                }
                                _ => stored_object_source_eligible_v1(
                                    need,
                                    batch,
                                    selected,
                                    contracts,
                                    &definitions,
                                    callable_result_classes,
                                )
                                .map_err(|issue| {
                                    OrdinaryNewCoSealIssueV1::BorrowedFormalIngress {
                                        site: need.reference().call_site,
                                        issue,
                                    }
                                })?,
                            }};
                            if !eligible {
                                targets.push(Ok(None));
                                continue;
                            }
                        }
                        match need {
                            PreparedSourceCallNeedV1::Lexical(target) => Ok(Some(target.clone())),
                            PreparedSourceCallNeedV1::Stored {
                                reference,
                                receiver,
                                result_requirement,
                                owning_object_input,
                            } => {
                                let receiver = issue_eligible_receiver_v1(
                                    &reference.call_site,
                                    &receiver,
                                    issue,
                                )?;
                                Ok(Some(LexicalInstanceCallSourceTargetV1 {
                                    call_site: reference.call_site.clone(),
                                    receiver_site: reference.receiver_site.clone(),
                                    receiver,
                                    target: reference.target.clone(),
                                    target_batch_slot: reference.target_batch_slot,
                                    callee_owner: reference.callee_owner,
                                    argument_sites: reference.argument_sites.clone(),
                                    owning_object_input: owning_object_input.clone(),
                                    result_requirement: result_requirement.clone(),
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
        brands,
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
            (
                ordinary_callers,
                definitions,
                dominated_view_sites,
                guarded_actuals,
                originals,
            ),
            static_arguments,
            original_inventory,
        )
    })();
    // Successful final closure adds literal-only Main sites. An error retains
    // the original fact selection and its existing failure scope.
    if let (Ok(prepared), Ok(sites)) = (&ingress, &mut static_source_sites) {
        sites.extend(
            prepared
                .incoming
                .iter()
                .filter(|row| {
                    matches!(
                        row.source,
                        super::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(ref original)
                            if original.is_qualified_i64_source_v1() || original.is_current_owner_i64_source_v1()
                    )
                })
                .map(|row| row.call.clone()),
        );
        sites.extend(
            prepared
                .target_static
                .values()
                .flat_map(|cohort| cohort.sites().cloned()),
        );
    }
    let results = seal_pending_results_v1(pending, contracts, &ingress, batch, constructors);
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

/// Source-only eligibility. The same receiver issuer and whole incoming scan
/// still own receiver residence and execution; no old-result retry is permitted.
fn stored_object_source_eligible_v1(
    need: &PreparedSourceCallNeedV1,
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    selected: &VerifiedSelectedCallableBatchMapV1,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    drafts: &BTreeMap<FunctionOwnerIdV1, super::borrowed_formal_uses::BorrowedFormalUsesDraftV1>,
    facts: &super::super::result_class_claim::OrdinaryNewResultClassClaimsV1,
) -> Result<bool, String> {
    use super::super::result_class_claim::{OrdinaryNewResultClassV1, ResultValueOriginV1};
    let reference = need.reference();
    let mut matching = contracts
        .iter()
        .filter(|row| row.owner == reference.callee_owner);
    let Some(contract) = matching.next() else {
        return Ok(false);
    };
    if matching.next().is_some()
        || contract.batch_slot != reference.target_batch_slot
        || selected.batch_slot(&SelectedNormalCallableKeyV1::Cataloged(
            reference.target.clone(),
        )) != Some(reference.target_batch_slot)
        || !batch.declarations().any(|row| {
            row.batch_slot() == reference.target_batch_slot && row.owner() == reference.callee_owner
        })
        || contract.parameters.len() != reference.argument_sites.len()
        || reference.argument_sites.len() != reference.target.arity() as usize
        || contract.parameters.iter().enumerate().any(|(i, formal)| {
            formal.ordinal as usize != i || formal.binding.owner() != reference.callee_owner
        })
    {
        return Err(freeze("stored-child/object-source-target-identity"));
    }
    let Some(caller_slot) = batch
        .declarations()
        .find(|row| row.owner() == reference.call_site.owner())
        .map(|row| row.batch_slot())
    else {
        return Ok(false);
    };
    let exact_call = batch.with_lowering_input(caller_slot, |input| {
        input
            .function()
            .method_call(reference.call_site.site())
            .is_some_and(|call| {
                call.receiver_site() == &reference.receiver_site
                    && call.selector() == reference.target.name()
                    && call.arity() == reference.target.arity()
                    && call
                        .arguments()
                        .iter()
                        .map(|arg| arg.site())
                        .eq(reference.argument_sites.iter())
            })
    });
    if !matches!(exact_call, Ok(true)) {
        return Err(freeze("stored-child/object-source-call-identity"));
    }
    let Some(class) = facts.get(&reference.target) else {
        return Ok(false);
    };
    if !matches!(
        class,
        OrdinaryNewResultClassV1::Object(_) | OrdinaryNewResultClassV1::NullableObject(_)
    ) {
        return Ok(false);
    }
    let roots = facts
        .checked_original_source_roots_v1(&reference.target, reference.callee_owner)
        .map_err(freeze)?;
    if roots.is_empty()
        || roots.iter().any(|row| match row.origin() {
            ResultValueOriginV1::Null => false,
            ResultValueOriginV1::Fresh(name) => Some(name.as_ref()) != class.class(),
            ResultValueOriginV1::ForwardFormal { .. } => true,
        })
    {
        return Ok(false);
    }
    let Some(forwards) = super::borrowed_formal_result::collect_observed_forward_identities_v1(
        need, contracts, drafts,
    ) else {
        return Ok(false);
    };
    match need.result_requirement() {
        LexicalCallSourceResultRequirementV1::ObjectReturnSource {
            qualifications,
            forwards: retained,
        } => {
            if qualifications.is_empty()
                || facts
                    .qualifications_for_call(&reference.call_site, &reference.target)
                    .as_ref()
                    != qualifications.as_ref()
                || retained.as_deref() != Some(forwards.as_slice())
            {
                return Err(freeze("stored-child/object-source-qualification-identity"));
            }
            for loan in qualifications {
                facts
                    .check_original_call_root_coverage_v1(loan, &roots)
                    .map_err(freeze)?;
            }
        }
        LexicalCallSourceResultRequirementV1::ObjectProducerDependency(dependencies) => {
            if dependencies.is_empty()
                || facts
                    .object_return_dependencies(&reference.target, reference.callee_owner)
                    .as_deref()
                    != Some(dependencies.as_ref())
            {
                return Err(freeze("stored-child/object-source-dependency-identity"));
            }
        }
        LexicalCallSourceResultRequirementV1::ExistingBorrowedResult
        | LexicalCallSourceResultRequirementV1::ScalarReturnSource(_)
        | LexicalCallSourceResultRequirementV1::UnitDiscardSource(_)
        | LexicalCallSourceResultRequirementV1::BorrowedNullableLocal(_)
        | LexicalCallSourceResultRequirementV1::MixedLocal(_) => {
            return Err(freeze("stored-child/object-source-requirement"))
        }
    }
    Ok(true)
}
