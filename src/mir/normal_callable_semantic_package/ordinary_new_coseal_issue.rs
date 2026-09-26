//! Cohort issuer for the Raw ordinary-`New` co-seal.
//!
//! The claim/ledger/issue vocabulary lives in `ordinary_new_coseal`; this
//! module owns only the per-declaration issuance walk that produces the
//! ledger and completion-seed cohort.  No admission shape is decided here
//! beyond what the sealed inputs already carry.

use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;

use super::super::instance_constructor_semantic::VerifiedInstanceConstructorSemanticBatchV1;
use super::super::selected_mapping::VerifiedSelectedCallableBatchMapV1;
use super::candidate::{verified_birth_recipe_for_site_v1, OrdinaryNewCandidate};
use super::coseal_helpers::{
    convert_selected_new_arguments, is_direct_local_initializer, retain_child_terminal_relation,
};
use super::{field_reads, terminal_home};
use super::{
    OrdinaryNewAdmissionClaimV1, OrdinaryNewClaimLedgerV1, OrdinaryNewCoSealIssueV1,
    VerifiedOrdinaryNewBirthRecipeV1,
};
use crate::ast::ASTNode;
use crate::mir::callable_semantic_batch::VerifiedResolvedCallableSemanticBatchV1;
use crate::mir::function::ObjectDestructionDispositionV1;
use crate::mir::resolved_semantics::home_new_prefix::{
    issue_new_home_prefixes_v1, issue_new_home_prefixes_with_arguments_v1,
    SelectedNewArgumentUnavailableV1, TerminalRelationV1,
};
use crate::mir::resolved_semantics::{
    BindingKindV1, FunctionOwnerIdV1, OwnedExprSiteV1, SourceBindingSiteV1, SourceExprSiteV1,
    VerifiedResolvedFunctionV1,
};

pub(in crate::mir::normal_callable_semantic_package) fn issue_ordinary_source_cohort_v1(
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    selected: &VerifiedSelectedCallableBatchMapV1,
    app_main_identity: Option<&crate::parser::CallableDeclarationIdentityV1>,
    direct_call_loans: Option<&super::super::direct_call_loan::DirectCallDispositionLoansV1>,
    parameter_contracts: &[super::super::model::OwnedCallableParameterContractDeclarationV1],
    dynamic: &mut super::super::model::NormalCallableDynamicProjectionV1,
    instance_constructors: &VerifiedInstanceConstructorSemanticBatchV1,
) -> Result<
    (
        OrdinaryNewClaimLedgerV1,
        super::super::completion_seed::VerifiedCallableCompletionSeedCohortV1,
    ),
    OrdinaryNewCoSealIssueV1,
> {
    let app_main_batch_slot = app_main_identity
        .map(|identity| {
            let mut matches = batch
                .declarations()
                .filter(|declaration| declaration.identity().same_as(identity));
            let declaration = matches
                .next()
                .ok_or(OrdinaryNewCoSealIssueV1::AppMainIdentityMissing)?;
            if matches.next().is_some() {
                return Err(OrdinaryNewCoSealIssueV1::AppMainIdentityDuplicate);
            }
            Ok(declaration.batch_slot())
        })
        .transpose()?;
    let mut claims = Vec::new();
    let mut seeds = super::super::completion_seed::VerifiedCallableCompletionSeedCohortV1::new();
    let mut root_completion = None;
    let mut field_reads = BTreeMap::new();
    let mut root_terminal_relation = None;
    let mut birth_abi_handoffs = BTreeMap::new();
    for declaration in batch.declarations() {
        let owner = declaration.owner();
        let batch_slot = declaration.batch_slot();
        let is_app_main = app_main_batch_slot == Some(batch_slot);
        if selected.role_for_batch_slot(batch_slot).is_none() && !is_app_main {
            continue;
        }
        let seed_eligible = super::super::completion_seed::preflight_declaration(
            declaration,
            selected,
            parameter_contracts,
        )
        .map_err(OrdinaryNewCoSealIssueV1::CompletionSeed)?;
        if let super::super::model::NormalCallableDynamicProjectionV1::Selected {
            batch_slot: dynamic_slot,
            program,
            result,
            ..
        } = dynamic
        {
            if *dynamic_slot == batch_slot {
                if seed_eligible {
                    *result = program
                        .with_canonical_session_authority(|authority| {
                            super::super::completion_seed::validate_result(
                                authority.completion(),
                                owner,
                                batch_slot,
                            )
                        })
                        .map_err(OrdinaryNewCoSealIssueV1::CompletionSeed)?;
                }
                continue;
            }
        }
        let (candidates, mut home_prefixes, mut argument_observations) = batch
            .with_lowering_input(batch_slot, |input| -> Result<_, OrdinaryNewCoSealIssueV1> {
                let function = input.function();
                let owner_loan = direct_call_loans.and_then(|loans| loans.get(owner));
                let mut candidates = Vec::new();
                for initializer in function.expression_source().initializers() {
                    let Some(initializer_site) = initializer.initializer_site() else {
                        continue;
                    };
                    if !is_direct_local_initializer(initializer_site.node().segments()) {
                        continue;
                    }
                    let site = OwnedExprSiteV1::new(owner, initializer_site.clone());
                    let located = input.source().expr_at(&site).map_err(|_| {
                        OrdinaryNewCoSealIssueV1::SourceNavigation { site: site.clone() }
                    })?;
                    let ASTNode::New { class, arguments, field_initializers, .. } = located.node() else {
                        continue;
                    };
                    if initializer.binding().owner() != owner
                        || function.declaration_binding(initializer.declaration_site())
                            != Some(initializer.binding())
                        || !matches!(function.binding(initializer.binding()).map(|row| row.kind()),
                            Some(BindingKindV1::Local { .. }))
                    {
                        return Err(OrdinaryNewCoSealIssueV1::InitializerBindingMismatch { site });
                    }
                    if let Some(candidate) = OrdinaryNewCandidate::resolve(
                        batch, instance_constructors, site, class.clone().into_boxed_str(),
                        arguments.len(), initializer.binding(), initializer.declaration_site().clone(),
                        !field_initializers.is_empty(),
                    )? {
                        candidates.push(candidate);
                    }
                }
                // A `%{...}` literal makes the owner a map owner; a `: MapBox`
                // declared formal does too — the borrowed-map read terminal
                // needs the homes-aware completion that classifies it.
                let has_map = input.body_shape().is_some_and(|shape| {
                    shape.expressions().iter().any(|row| matches!(
                        row,
                        crate::mir::resolved_semantics::BodyExpressionShapeV1::MapLiteral { .. }
                    ))
                }) || parameter_contracts
                    .iter()
                    .filter(|row| row.batch_slot == batch_slot)
                    .flat_map(|row| row.parameters.iter())
                    .any(|row| {
                        row.kind
                            == crate::mir::callable_parameter_contract::CallableParameterContractKindV1::Map
                    });
                let new_sites: BTreeMap<_, _> = candidates.iter().map(|candidate| (candidate.site.clone(), candidate.destination)).collect();
                let child_new_ready = seed_eligible && !new_sites.is_empty()
                    && issue_new_home_prefixes_v1(input, &new_sites).values().all(Result::is_ok);
                let seed_completion = seed_eligible
                    && !has_map
                    && !child_new_ready
                    && (is_app_main || owner_loan.is_none());
                let app_main_integer_result = is_app_main
                    && candidates.is_empty()
                    && !has_map
                    && owner_loan.is_none()
                    && !function
                        .declaration_sites()
                        .any(|site| matches!(site, SourceBindingSiteV1::Receiver))
                    && input
                        .forest()
                        .ordered_capture_demands(input.owner())
                        .is_empty();
                if seed_completion || app_main_integer_result {
                    let completion = Rc::new(
                        crate::mir::resolved_control_flow::verify_function_completion_v1(input)
                            .map_err(|issue| OrdinaryNewCoSealIssueV1::CompletionSeed(
                            super::super::physical_header::CallablePhysicalHeaderIssueV1::Completion {
                                _batch_slot: batch_slot, _issue: issue,
                            }))?,
                    );
                    if app_main_integer_result {
                        if let Some(relation) = crate::mir::resolved_semantics::home_new_prefix::issue_terminal_integer_literal_return_from_completion_v1(
                            input,
                            completion.as_ref(),
                        )
                        .map_err(OrdinaryNewCoSealIssueV1::RootTerminalSource)?
                        {
                            root_completion = Some(Ok(Rc::clone(&completion)));
                            root_terminal_relation = Some(TerminalRelationV1::IntegerLiteral(relation));
                        }
                    }
                    if seed_completion {
                        seeds.push_completion(
                            declaration,
                            selected,
                            Rc::clone(&completion),
                            None,
                        )
                        .map_err(OrdinaryNewCoSealIssueV1::CompletionSeed)?;
                    }
                }
                let (home_prefixes, argument_observations) = if owner_loan.is_some() || (is_app_main && (!new_sites.is_empty() || has_map)) || (seed_eligible && (has_map || child_new_ready)) {
                    let mut staged_reads = BTreeMap::new();
                    let mut field_is_integer = |site: &OwnedExprSiteV1, receiver_site: &SourceExprSiteV1, receiver, home, name: &str| {
                        let field = terminal_home::initialized_integer_field(
                            instance_constructors, &candidates, home, name)?;
                        let Some(field) = field else { return Ok(false); };
                        if staged_reads.insert(site.clone(), field_reads::FieldRead {
                            receiver_site: receiver_site.clone(), receiver, home, field,
                            progress: field_reads::Progress::Pending,
                        }).is_some() { return Err(OrdinaryNewCoSealIssueV1::DuplicateSite { site: site.clone() }); }
                        Ok(true)
                    };
                    match crate::mir::resolved_control_flow::verify_function_completion_with_new_homes_and_argument_observations_v1(
                        input, &new_sites,
                        parameter_contracts.iter().filter(|row| row.batch_slot == batch_slot)
                            .flat_map(|row| row.parameters.iter())
                            .map(|row| (row.ordinal, row.binding, row.kind)),
                        &mut field_is_integer, &mut |site, binding| {
                            let mut exact = candidates.iter().filter(|row| &row.site == site);
                            let candidate = exact.next().ok_or_else(|| OrdinaryNewCoSealIssueV1::InitializerBindingMismatch { site: site.clone() })?;
                            if exact.next().is_some() || candidate.destination != binding {
                                return Err(OrdinaryNewCoSealIssueV1::InitializerBindingMismatch { site: site.clone() });
                            }
                            Ok(candidate.construction.is_ok() && candidate.destruction == ObjectDestructionDispositionV1::PlainI64NoHook)
                        }, &mut |site| {
                            let direct = owner_loan
                                .is_some_and(|loan| loan.is_i64_call(input, site));
                            let instance = is_app_main
                                && input.function().method_calls().any(|(call_site, call)| {
                                    call_site == site.site()
                                        && call.arguments().is_empty()
                                        && matches!(
                                            call.receiver(),
                                            crate::mir::resolved_semantics::ResolvedMethodCallReceiverSourceV1::Lexical(
                                                crate::mir::resolved_semantics::ResolvedLexicalRefV1::Local(_)
                                            )
                                        )
                                });
                            Ok(direct || instance)
                        }, &mut |site| {
                            Ok(owner_loan.is_some_and(|loan| {
                                loan.is_map_result_call(batch, parameter_contracts, input, site)
                            }))
                        })? {
                        Ok((completion, prefixes, mut terminal_relation, observations)) => {
                            if is_app_main {
                                if matches!(completion.cleanup().terminal_homes(), Some(Ok(_))) {
                                    if let Some(TerminalRelationV1::I64Add(result)) = &terminal_relation {
                                        if result.owner() != input.owner()
                                            || result.field_reads().iter().any(|site|
                                                !staged_reads.contains_key(site))
                                        {
                                            return Err(OrdinaryNewCoSealIssueV1::TerminalResultFieldReadMissing {
                                                site: result.add_site().clone(),
                                            });
                                        }
                                    }
                                    if let Some(TerminalRelationV1::I64Field(result)) = &terminal_relation {
                                        if result.owner() != input.owner()
                                            || !staged_reads.contains_key(result.field_read_site())
                                        {
                                            return Err(OrdinaryNewCoSealIssueV1::TerminalResultFieldReadMissing {
                                                site: result.field_read_site().clone(),
                                            });
                                        }
                                    }
                                    field_reads::merge_staged_field_reads(
                                        &mut field_reads,
                                        input.owner(),
                                        staged_reads,
                                    )?;
                                    root_terminal_relation = terminal_relation.take();
                                }
                                root_completion = Some(Ok(Rc::new(completion)));
                            } else {
                                field_reads::merge_terminal_relation_field_reads(
                                    &mut field_reads,
                                    input.owner(),
                                    terminal_relation.as_ref(),
                                    staged_reads,
                                )?;
                                let relation = terminal_relation
                                    .filter(|row| retain_child_terminal_relation(row, has_map));
                                seeds.push_completion(declaration, selected, Rc::new(completion), relation)
                                    .map_err(OrdinaryNewCoSealIssueV1::CompletionSeed)?;
                            }
                            (prefixes, observations)
                        }
                        Err(error) => {
                            if !is_app_main {
                                return Err(OrdinaryNewCoSealIssueV1::CompletionSeed(
                                    super::super::physical_header::CallablePhysicalHeaderIssueV1::Completion {
                                        _batch_slot: batch_slot, _issue: error,
                                    }));
                            }
                            root_completion = Some(Err(error));
                            issue_new_home_prefixes_with_arguments_v1(
                                input, &new_sites,
                                parameter_contracts.iter().filter(|row| row.batch_slot == batch_slot)
                                    .flat_map(|row| row.parameters.iter())
                                    .map(|row| (row.ordinal, row.binding, row.kind)),
                            )
                        }
                    }
                } else {
                    issue_new_home_prefixes_with_arguments_v1(
                        input, &new_sites,
                        parameter_contracts.iter().filter(|row| row.batch_slot == batch_slot)
                            .flat_map(|row| row.parameters.iter())
                            .map(|row| (row.ordinal, row.binding, row.kind)),
                    )
                };
                Ok((candidates, home_prefixes, argument_observations))
            })
            .map_err(|_| OrdinaryNewCoSealIssueV1::BatchLoan)??;
        for candidate in candidates {
            let OrdinaryNewCandidate {
                site,
                box_source,
                class,
                arity,
                destination,
                declaration,
                construction,
                object,
                destruction,
                constructor,
                birth_handoff,
            } = candidate;
            let argument_rows = argument_observations
                .remove(&site)
                .map(convert_selected_new_arguments)
                .unwrap_or_else(|| {
                    Err(SelectedNewArgumentUnavailableV1::SourceMismatch {
                        new_site: site.clone(),
                    })
                });
            if claims
                .iter()
                .any(|claim: &OrdinaryNewAdmissionClaimV1| claim.site == site)
            {
                return Err(OrdinaryNewCoSealIssueV1::DuplicateSite { site });
            }
            let home_prefix = home_prefixes.remove(&site).ok_or_else(|| {
                OrdinaryNewCoSealIssueV1::InitializerBindingMismatch { site: site.clone() }
            })?;
            if let Some(handoff) = birth_handoff {
                if birth_abi_handoffs.insert(site.clone(), handoff).is_some() {
                    return Err(OrdinaryNewCoSealIssueV1::DuplicateSite { site });
                }
            }
            claims.push(OrdinaryNewAdmissionClaimV1 {
                site: site.clone(),
                box_source,
                class,
                arity,
                constructor,
                destination,
                declaration,
                home_prefix,
                construction,
                object,
                destruction,
                argument_rows,
            });
        }
    }
    // Destination-less birth-recipe index for `new` sites outside the
    // local-commit claim lane.  `constructions` records every `new`
    // regardless of position; this walk only admits a site whose class is
    // covered and whose `birth_for` row passes the same verification as a
    // claim candidate.  A site that admits nothing keeps its existing
    // downstream terminal — the index never issues a new error.
    let mut birth_site_index = BTreeMap::new();
    let claimed_sites: BTreeSet<OwnedExprSiteV1> =
        claims.iter().map(|claim| claim.site().clone()).collect();
    for declaration in batch.declarations() {
        let owner = declaration.owner();
        batch
            .with_lowering_input(declaration.batch_slot(), |input| {
                collect_birth_site_index_v1(
                    input.function(),
                    owner,
                    batch,
                    instance_constructors,
                    &claimed_sites,
                    &mut birth_site_index,
                )
            })
            .map_err(|_| OrdinaryNewCoSealIssueV1::BatchLoan)??;
    }
    batch
        .with_normal_program_source_loan(|loan| -> Result<(), OrdinaryNewCoSealIssueV1> {
            for row in instance_constructors.rows() {
                let input = row.lowering_input(loan.program()).map_err(|_| {
                    OrdinaryNewCoSealIssueV1::BatchLoan
                })?;
                collect_birth_site_index_v1(
                    input.function(),
                    input.owner(),
                    batch,
                    instance_constructors,
                    &claimed_sites,
                    &mut birth_site_index,
                )?;
            }
            Ok(())
        })
        .map_err(|_| OrdinaryNewCoSealIssueV1::BatchLoan)??;
    let names = batch
        .ordinary_box_coverage()
        .rows()
        .iter()
        .map(|row| row.name().to_owned().into_boxed_str())
        .collect();
    let mut ledger = OrdinaryNewClaimLedgerV1::issue(claims.into_boxed_slice(), names);
    ledger.birth_site_index = std::cell::RefCell::new(birth_site_index);
    ledger.root_completion = root_completion;
    ledger.field_reads = std::cell::RefCell::new(field_reads);
    ledger.birth_abi_handoffs = std::cell::RefCell::new(birth_abi_handoffs);
    ledger.terminal_relation = root_terminal_relation;
    ledger.app_main_identity = app_main_identity.cloned();
    let seeds = seeds.finish();
    Ok((ledger, seeds))
}

/// Admit one `new` construction site into the destination-less birth index
/// when — and only when — a verified `Birth` recipe exists for it.  Missing
/// coverage, a missing `birth` row, lookup errors, and verification failures
/// all produce no entry: the index is additive-only and never replaces the
/// existing downstream terminal.
fn collect_birth_site_index_v1(
    function: &VerifiedResolvedFunctionV1,
    owner: FunctionOwnerIdV1,
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    instance_constructors: &VerifiedInstanceConstructorSemanticBatchV1,
    claimed_sites: &BTreeSet<OwnedExprSiteV1>,
    index: &mut BTreeMap<OwnedExprSiteV1, VerifiedOrdinaryNewBirthRecipeV1>,
) -> Result<(), OrdinaryNewCoSealIssueV1> {
    for construction in function.expression_source().constructions() {
        if is_direct_local_initializer(construction.site().node().segments()) {
            continue;
        }
        let site = OwnedExprSiteV1::new(owner, construction.site().clone());
        if claimed_sites.contains(&site) {
            continue;
        }
        let Ok(coverage_row) = batch
            .ordinary_box_coverage()
            .row_for(construction.class())
        else {
            continue;
        };
        let Some(box_source) = coverage_row else {
            continue;
        };
        let Ok(Some(birth_row)) =
            instance_constructors.birth_for(box_source, construction.arguments().len())
        else {
            continue;
        };
        let Ok((recipe, _)) = verified_birth_recipe_for_site_v1(
            &site,
            construction.class(),
            construction.arguments().len(),
            birth_row,
        ) else {
            continue;
        };
        if index.insert(site.clone(), recipe).is_some() {
            return Err(OrdinaryNewCoSealIssueV1::DuplicateSite { site });
        }
    }
    Ok(())
}
