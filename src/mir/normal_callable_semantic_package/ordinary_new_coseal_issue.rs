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
use super::candidate::{
    verified_birth_recipe_for_site_v1, OrdinaryNewCandidate, OrdinaryNewSiteResolutionV1,
};
use super::coseal_helpers::{convert_selected_new_arguments, retain_child_terminal_relations_v1};
use super::{
    array_i64_fields, field_reads, field_write_claim, receiver_call_observation,
    result_class_claim, terminal_home,
};
use super::{
    OrdinaryNewAdmissionClaimV1, OrdinaryNewClaimCoreV1, OrdinaryNewClaimLedgerV1,
    OrdinaryNewCoSealIssueV1, OrdinaryNewResultClaimV1, OwnedFieldChildKindV1, OwnedFieldChildV1,
    VerifiedOrdinaryNewBirthRecipeV1,
};
use crate::mir::builder::{CanonicalSameModuleCallableKeyV1, SelectedNormalCallableKeyV1};
use crate::mir::callable_semantic_batch::VerifiedResolvedCallableSemanticBatchV1;
use crate::mir::function::ObjectDestructionDispositionV1;
use crate::mir::resolved_semantics::home_new_prefix::{
    issue_new_home_prefixes_with_arguments_v1, BorrowedCallActualRequestV1,
    BorrowedCallArgumentsV1, HomePrefixUnavailableV1, SelectedNewArgumentUnavailableV1,
    TerminalRelationV1,
};
use crate::mir::resolved_semantics::{
    BindingRefV1, FunctionOwnerIdV1, OwnedExprSiteV1, SourceBindingSiteV1, SourceExprSiteV1,
    SourceStmtSiteV1, VerifiedResolvedFunctionV1,
};
use hakorune_mir_defs::SameModuleCallableNamespaceV1;

#[path = "ordinary_new_coseal_issue_completion_seed.rs"]
mod completion_seed;

#[path = "ordinary_new_coseal_issue_lexical.rs"]
mod lexical;
use lexical::{
    borrowed_call_arguments_callback_v1, lexical_handle_result_call, lexical_i64_result_call,
    lexical_nullable_result_call,
};
// The borrowed-formal object view resolves received-nullable classes with
// the same sealed `NullableObject` claim lookup as the field issuer.
pub(in crate::mir::normal_callable_semantic_package) use lexical::nullable_received_result_class;

#[path = "ordinary_new_coseal_issue_walk_triggers.rs"]
mod walk_triggers;

#[path = "ordinary_new_coseal_issue_integer_field.rs"]
mod integer_fields;

#[path = "ordinary_new_coseal_issue_source.rs"]
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal) mod source_claims;
use source_claims::collect_birth_site_index_v1;

#[path = "ordinary_new_coseal_issue_result_membership.rs"]
mod result_membership;

#[path = "ordinary_new_coseal_issue_loop.rs"]
mod loop_issue;
pub(crate) use loop_issue::RetainedSourceLoopProductV1;
pub(super) use loop_issue::{
    LoopProductUnavailableV1, SelectedLoopProductRowV1, SelectedLoopProductV1,
};

pub(in crate::mir::normal_callable_semantic_package) fn issue_ordinary_source_cohort_v1(
    brands: Option<&crate::analysis::brand_program_declaration_catalog::VerifiedBrandProgramDeclarationCatalogV1>,
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    selected: &VerifiedSelectedCallableBatchMapV1,
    omitted_static_callers: &BTreeMap<
        FunctionOwnerIdV1,
        crate::mir::builder::CanonicalSameModuleCallableKeyV1,
    >,
    app_main_source: Option<&crate::mir::builder::AppMainCatalogCoSealV1>,
    direct_call_loans: Option<&super::super::direct_call_loan::DirectCallDispositionLoansV1>,
    parameter_contracts: &[super::super::model::OwnedCallableParameterContractDeclarationV1],
    entry_home_loans: &crate::mir::resolved_semantics::VerifiedInstanceEntryHomeCatalogV1,
    dynamic: &mut super::super::model::NormalCallableDynamicProjectionV1,
    dynamic_source_calls: &[crate::mir::source_call_target::VerifiedSourceBoundDynamicMemberCallV1],
    instance_constructors: &VerifiedInstanceConstructorSemanticBatchV1,
    static_call_claims: &super::super::qualified_static_call_claim::QualifiedStaticCallClaimIndexV1,
    source_core_method_calls: &BTreeMap<
        SelectedNormalCallableKeyV1,
        super::super::core_method_source::SourceCoreMethodCallRowsV1,
    >,
    loop_policy: crate::mir::builder::LoopFactsPolicyFrameV1,
    field_writers: Rc<super::OriginalFieldWriterInventoryV1>,
) -> Result<
    (
        OrdinaryNewClaimLedgerV1,
        super::super::result_contract::VerifiedCallableResultContractBuilderV1,
    ),
    OrdinaryNewCoSealIssueV1,
> {
    let app_main = super::lexical_instance_call::borrow_app_main_source_v1(batch, app_main_source)?;
    let app_main_batch_slot = app_main.as_ref().map(|main| main.batch_slot());
    let mut claims = Vec::new();
    let mut result_claims = Vec::new();
    let mut seeds = super::super::result_contract::VerifiedCallableResultContractBuilderV1::new();
    let mut root_completion = None;
    let mut field_reads = BTreeMap::new();
    let mut argument_field_reads = BTreeMap::new();
    let mut local_field_reads = BTreeMap::new();
    let mut root_terminal_relation = BTreeMap::new();
    let mut birth_abi_handoffs = BTreeMap::new();
    let mut receiver_call_observations = BTreeMap::new();
    let mut birth_site_index = BTreeMap::new();
    let mut borrowed_formal_actuals = BTreeMap::new();
    let mut selected_loop_products = BTreeMap::new();
    // Owned field children per canonical object: `Some` means every
    // residence-capable declared field has a sealed birth-side residence,
    // `None` means the disposition needs children the package never proved.
    let (field_write_claims, field_residences, callable_result_classes, mut owned_field_children) =
        source_claims::prepare_source_claims(
            batch,
            selected,
            app_main.as_ref(),
            instance_constructors,
            parameter_contracts,
            field_writers,
            Some(entry_home_loans),
        )?;
    let dynamic_slot = match dynamic {
        super::super::model::NormalCallableDynamicProjectionV1::Selected { batch_slot, .. } => {
            Some(*batch_slot)
        }
        _ => None,
    };
    // The whole-Box `ArrayBox` element-integer census — one sealed
    // canonical-field set per ordinary Box, issued once before any walk so
    // the probe, the verified lane, and the borrowed-result return proof
    // consult the identical authority. A field outside the set keeps the
    // manifest `Dynamic` get result everywhere.
    let mut array_i64_field_sets: BTreeMap<
        Box<str>,
        BTreeSet<hakorune_mir_defs::CanonicalFieldRefV1>,
    > = BTreeMap::new();
    let mut array_field_write_candidates = Vec::new();
    let mut array_field_get_candidates = Vec::new();
    for box_source in batch.ordinary_box_coverage().rows() {
        let census = array_i64_fields::issue_array_i64_fields_v1(
            selected,
            batch,
            instance_constructors,
            box_source,
        )?;
        if !census.integer_fields.is_empty() {
            array_i64_field_sets.insert(box_source.name().into(), census.integer_fields);
        }
        array_field_write_candidates.extend(census.write_fields.into_vec());
        array_field_get_candidates.extend(census.field_reads.into_vec());
    }
    let (
        names,
        mut local_candidates,
        lexical_source_targets,
        borrowed_formal_source,
        static_source_sites,
        borrowed_i64_results,
    ) = lexical::prepare_source_preflight_v1(
        brands,
        batch,
        selected,
        omitted_static_callers,
        instance_constructors,
        parameter_contracts,
        static_call_claims,
        app_main_batch_slot,
        app_main.as_ref(),
        dynamic_slot,
        entry_home_loans,
        &field_write_claims,
        &callable_result_classes,
        &field_residences,
        &array_i64_field_sets,
        &array_field_write_candidates,
        &array_field_get_candidates,
        &mut owned_field_children,
    )?;
    let mut loop_static_source_loans =
        super::loop_static_source_loan::issue_loop_static_source_loans_v1(
            batch,
            selected,
            parameter_contracts,
            static_call_claims,
            borrowed_formal_source.as_ref().ok(),
        )?;
    let mut preissued_static_i64_products = BTreeMap::new();
    let typed_static_walk_owners = walk_triggers::typed_static_walk_owners_v1(
        &borrowed_formal_source,
        selected,
        app_main.as_ref(),
        batch,
        parameter_contracts,
    )?;
    for declaration in walk_triggers::text_completion_order_v1(
        &borrowed_formal_source,
        &borrowed_i64_results,
        &lexical_source_targets,
        selected,
        app_main.as_ref(),
        batch,
        parameter_contracts,
    )? {
        let owner = declaration.owner();
        let batch_slot = declaration.batch_slot();
        let is_app_main = app_main_batch_slot == Some(batch_slot);
        let source_core_calls = selected
            .keys()
            .find(|key| selected.batch_slot(key) == Some(batch_slot))
            .and_then(|key| source_core_method_calls.get(key));
        let has_typed_static_walk = typed_static_walk_owners.contains(&owner);
        if !walk_triggers::needs_declaration_walk_v1(batch, selected, omitted_static_callers, app_main_batch_slot, batch_slot)? { continue; }
        let seed_eligible = super::super::result_contract::preflight_declaration(
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
                            super::super::result_contract::validate_result(
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
        let (
            candidates,
            home_prefixes,
            argument_observations,
            result_resolutions,
            result_prefixes,
        ) = batch
            .with_lowering_input(batch_slot, |input| -> Result<_, OrdinaryNewCoSealIssueV1> {
                let mut top_level_input =
                    super::super::result_contract::issue_top_level_scalar_input_v1(
                        declaration,
                        selected,
                        input,
                    )
                    .map_err(OrdinaryNewCoSealIssueV1::CompletionSeed)?;
                let owner_loan = direct_call_loans.and_then(|loans| loans.get(owner));
                let entry_home = entry_home_loans.for_batch_slot(batch_slot);
                let batch_contract_rows: Vec<_> = parameter_contracts
                    .iter()
                    .filter(|row| row.batch_slot == batch_slot)
                    .collect();
                let batch_params =
                    || batch_contract_rows.iter().flat_map(|row| row.parameters.iter());
                let candidates = local_candidates.remove(&batch_slot)
                    .ok_or(OrdinaryNewCoSealIssueV1::BatchLoan)??;
                let (result_resolutions, result_sites) =
                    result_membership::issue_return_new_membership_v1(
                        input,
                        &candidates,
                        batch,
                        instance_constructors,
                    )?;
                let has_map = walk_triggers::has_map_v1(input, || batch_params().any(|row| {
                    row.kind
                        == crate::mir::callable_parameter_contract::CallableParameterContractKindV1::Map
                }));
                let new_sites: BTreeMap<_, _> = candidates.iter().map(|candidate| (candidate.site.clone(), candidate.destination)).collect();
                // The entry loan proves `me`-receiver reads: the sole Home
                // ABI issuer bound `me` to this box — shared by the probe,
                // the walk, and the `me.m(..)` call-result observation.
                let receiver_proof = terminal_home::entry_receiver_box_proof(
                    selected, batch, entry_home, batch_slot,
                );
                receiver_call_observation::observe_receiver_call_sites(
                    input, receiver_proof, selected, &callable_result_classes,
                    &batch_contract_rows,
                    &mut receiver_call_observations,
                ).map_err(|(site, issue)| OrdinaryNewCoSealIssueV1::BorrowedFormalIngress { site, issue })?;
                // Destination-less birth index collects in the same
                // sweep: this declaration's claimed sites are exactly
                // `candidates` ∪ `result_resolutions` — both push
                // unconditionally after the closure returns.
                let declared_claimed: BTreeSet<OwnedExprSiteV1> = candidates
                    .iter()
                    .map(|row| row.site.clone())
                    .chain(result_resolutions.iter().map(|row| row.site.clone()))
                    .collect();
                collect_birth_site_index_v1(
                    input.function(), owner, batch, instance_constructors,
                    &declared_claimed, &mut birth_site_index,
                )?;
                // The caller's canonical key drives the qualified static-call
                // claim index — the probe and the verified walk share this
                // predicate so readiness never diverges from the real lane.
                let static_caller_key =
                    super::super::qualified_static_call_claim::caller_key_for_function(
                        selected, batch_slot, is_app_main, app_main_source.map(|main| main.catalog_key()),
                    );
                let (mut local_static_call, has_current_owner_i64_source) =
                    super::super::qualified_static_call_claim::local_static_call_predicate(
                        static_call_claims,
                        static_caller_key.clone(),
                        input, selected, parameter_contracts, app_main.as_ref(),
                    )?;
                let has_borrowed_source_calls = borrowed_formal_source.as_ref().is_ok_and(|rows| {
                    rows.incoming_calls_for_owner(input.owner())
                });
                let has_integer_return = borrowed_formal_source.as_ref().is_ok_and(|rows|
                    rows.integer_return_target(owner) || rows.integer_mul_return_target(input));
                let has_formal_field_read = borrowed_formal_source.as_ref().is_ok_and(|rows| {
                    rows.formal_field_read_target(input.owner())
                });
                let has_dynamic_local_initializer = walk_triggers::has_dynamic_local_initializer_v1(
                    input,
                    dynamic_source_calls,
                );
                let has_borrowed_walk = walk_triggers::has_original_borrowed_walk_v1(
                    &borrowed_formal_source, parameter_contracts, &lexical_source_targets, owner,
                )?;
                let mut probe = |explicit_sites: &[crate::mir::resolved_semantics::SourceStmtSiteV1],
                                 pending_actuals: &mut super::lexical_instance_call::PendingBorrowedFormalActualsV1,
                                 completed: &super::super::result_contract::VerifiedCallableResultContractBuilderV1| {
                    source_claims::probe_source_home_prefixes_v1(
                        brands, source_core_calls, input, &new_sites, entry_home, explicit_sites, pending_actuals,
                        dynamic_source_calls,
                        batch_slot, selected, batch, parameter_contracts,
                        &callable_result_classes, &candidates, instance_constructors,
                        receiver_proof, &array_i64_field_sets,
                        &borrowed_formal_source, &static_source_sites, &lexical_source_targets, &borrowed_i64_results, &mut local_static_call, &receiver_call_observations, &field_write_claims, completed,
                    )
                };
                let readiness = if seed_eligible && !new_sites.is_empty() {
                    // The probe must predict the verified-completion lane:
                    // without its explicit exit sites a `return` inside a
                    // guard arm reads as `PrefixNotCovered` and poisons the
                    // joined prefix before the selected `new` is walked.
                    match crate::mir::resolved_control_flow::verify_function_completion_v1(input) {
                        Ok(control) => probe(control.explicit_sites(), &mut BTreeMap::new(), &seeds)?
                            .values()
                            .all(Result::is_ok),
                        // When completion itself is unverifiable the verified
                        // lane cannot run; predict `false` so the bounded
                        // sibling keeps its existing graceful fallback.
                        Err(_) => false,
                    }
                } else { false };
                let child_new_ready = seed_eligible && !new_sites.is_empty() && readiness;
                // An owner whose `return` statement carries a `new`
                // construction needs the homes-aware completion: the
                // returned `Invoke{NewBox}` is a lifecycle instruction that
                // consumes the attached homes flow, and the
                // `Value(Construction)` terminal relation only comes from
                // the verified walk — the plain seed path cannot issue it.
                let child_result_ready = seed_eligible && !result_sites.is_empty();
                // A sealed `Nullable` `me.m(..)` observation needs the
                // homes-aware verify: the caller's local-call flow row,
                // the conditional release at exit, and the prior-home
                // unwind all come from the scanned root flow — the plain
                // seed path cannot issue any of them. `Object` receiver
                // observations outside the guarded profile keep the generic floor.
                let has_receiver_home_call = source_claims::has_receiver_home_call_v1(
                    owner, &receiver_call_observations,
                    &borrowed_formal_source, parameter_contracts,
                )?;
                let has_me_object_field_read = walk_triggers::has_me_object_field_read_v1(
                    input, instance_constructors, &candidates,
                    batch.ordinary_box_coverage(), receiver_proof,
                )?;
                let has_stored_terminal = lexical::has_stored_terminal_v1(&lexical_source_targets, &borrowed_i64_results, input)?;
                let has_field_scalar_walk = source_claims::requires_field_scalar_walk_v1(&field_write_claims, owner)?;
                let has_required_scalar_source_walk = super::lexical_instance_call::required_scalar_source_walk_v1(
                    &borrowed_formal_source, &borrowed_i64_results, parameter_contracts, owner, &callable_result_classes,
                ).map_err(|issue| OrdinaryNewCoSealIssueV1::BorrowedFormalIngress { site: issue.0, issue: issue.1 })?;
                let has_static_source_local = walk_triggers::has_static_source_local_v1(input, &static_source_sites) || has_current_owner_i64_source;
                // Print expressions need the same original Home scan even
                // without constructions, borrowed calls or owned temporaries.
                let has_print_expression = walk_triggers::has_print_expression_v1(input);
                let has_condition_expression = walk_triggers::has_condition_expression_v1(input);
                let has_text_initializer = borrowed_formal_source.as_ref().is_ok_and(|source| source.requires_initializer_expression_walk_v1(input));
                let has_text_return = borrowed_formal_source.as_ref().is_ok_and(|source| source.requires_text_return_walk_v1(input, &borrowed_i64_results));
                let has_array_residence = source_core_calls.is_some_and(|rows| rows.values()
                    .any(|row| row.contract().named_array_requirement().and_then(|row| row.residence()).is_some()));
                let seed_completion = (seed_eligible
                    || (top_level_input.is_some() && new_sites.is_empty() && result_sites.is_empty()))
                    && !has_map
                    && !callable_result_classes.has_object_call_return(owner)
                    && !has_receiver_home_call
                    && !has_me_object_field_read
                    && !has_stored_terminal
                    && !has_required_scalar_source_walk
                    && !has_field_scalar_walk
                    && !has_static_source_local && !has_typed_static_walk
                    && !has_print_expression
                    && !has_condition_expression
                    && !has_text_initializer && !has_text_return
                    && !has_array_residence
                    && !has_integer_return
                    && !has_formal_field_read
                    && !has_dynamic_local_initializer
                    && !has_borrowed_walk
                    && !child_new_ready
                    && !child_result_ready
                    && (is_app_main || owner_loan.is_none());
                let app_main_integer_result = walk_triggers::plain_main_integer_result_v1(
                    input,
                    is_app_main,
                    candidates.is_empty() && result_sites.is_empty() && !has_print_expression && !has_condition_expression && !has_text_initializer && !has_text_return && !has_typed_static_walk,
                    !has_map && owner_loan.is_none(),
                    &static_source_sites,
                );
                completion_seed::issue_plain_completion(
                    input,
                    declaration,
                    selected,
                    seed_completion,
                    app_main_integer_result,
                    &mut seeds,
                    &mut root_completion,
                    &mut root_terminal_relation,
                    &mut top_level_input,
                )?;
                let verified_walk = has_print_expression || has_condition_expression || has_typed_static_walk
                    || has_text_initializer || has_text_return
                    || has_array_residence
                    || owner_loan.is_some()
                    || callable_result_classes.has_object_call_return(owner)
                    || has_receiver_home_call
                    || has_me_object_field_read
                    || has_static_source_local
                    || has_integer_return
                    || has_formal_field_read
                    || has_borrowed_walk
                    || has_stored_terminal
                    || has_required_scalar_source_walk
                    || has_field_scalar_walk
                    || has_dynamic_local_initializer
                    || (is_app_main && (!new_sites.is_empty() || has_map || !result_sites.is_empty()))
                    || (seed_eligible && (has_map || child_new_ready || child_result_ready));
                if !verified_walk && has_borrowed_source_calls {
                    // Borrow existing control authority without publishing Completion.
                    let observed = crate::mir::resolved_control_flow::verify_function_completion_v1(input)
                        .map_err(|issue| format!("borrowed-actual/control-unavailable: {issue:?}"))
                        .and_then(|control| probe(control.explicit_sites(), &mut borrowed_formal_actuals, &seeds)
                            .map(|_| ()).map_err(|issue| format!("borrowed-actual/source-walk: {issue:?}")));
                    if let Err(issue) = observed {
                        super::lexical_instance_call::reject_borrowed_actuals_for_owner_v1(
                            &borrowed_formal_source, input.owner(), &mut borrowed_formal_actuals, issue,
                        );
                    }
                }
                drop(probe);
                let (home_prefixes, argument_observations, result_prefixes) = if verified_walk {
                    let mut staged_reads = BTreeMap::new();
                    let mut field_is_integer = integer_fields::terminal_field_consult_v1(instance_constructors,
                        &candidates, batch.ordinary_box_coverage(), &mut staged_reads,
                        |site, receiver_site, receiver, home, name| integer_fields::consult_v1(input, selected, batch,
                            instance_constructors, &candidates, &callable_result_classes, &borrowed_formal_source,
                            &borrowed_i64_results, receiver_proof, site, receiver_site, receiver, home, name));
                    // Argument-position `receiver.field` reads: a claim-local
                    // `new` Home receiver reuses the terminal provenance; the
                    // entry loan's receiver root proves against this
                    // declaration's own box source. The same site is asked
                    // twice (observation + accounting pass) — the staged row
                    // makes the answer idempotent.
                    let mut argument_staged_reads = BTreeMap::new();
                    let mut argument_field_is_integer = |site: &OwnedExprSiteV1, receiver_site: &SourceExprSiteV1, receiver, home, name: &str| {
                        if argument_staged_reads.contains_key(site) {
                            return Ok(true);
                        }
                        let Some(field) = terminal_home::argument_integer_field(
                            instance_constructors, &candidates, site, receiver_proof, home, name,
                        )? else { return Ok(false); };
                        argument_staged_reads.insert(site.clone(), field_reads::ArgumentFieldRead {
                            receiver_site: receiver_site.clone(), object: receiver, field,
                            progress: field_reads::Progress::Pending,
                        });
                        Ok(true)
                    };
                    // Local-initializer `receiver.field` reads: the same
                    // site may be re-asked across paths, so the staged row
                    // is the idempotent answer — the first proof stages
                    // the ledger row, later asks return its result class.
                    let mut local_staged_reads: BTreeMap<OwnedExprSiteV1, field_reads::LocalFieldRead> = BTreeMap::new();
                    let mut local_field_read = |requests: &[crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadRequestV1], demand| {
                        let probe = matches!(&demand, crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadDemandV1::SourceExpression { probe: true, .. });
                        let Some(rows) = source_claims::prove_local_field_read_batch(
                            instance_constructors, &candidates, batch.ordinary_box_coverage(),
                            receiver_proof, entry_home, source_core_calls, requests, demand, &borrowed_i64_results, &borrowed_formal_source,
                            &mut |home| lexical::nullable_received_result_class(
                                selected, batch, &callable_result_classes, &candidates, input, receiver_proof, home,
                            ),
                            &mut |home| source_claims::source_formal_class_v1(&borrowed_formal_source, home), Some((batch, selected, &callable_result_classes, &lexical_source_targets, &field_write_claims)),
                        )? else { return Ok(None); };
                        if probe { return Ok(Some(rows.into_iter().map(|(_, row)| row.source.result().clone()).collect())); }
                        source_claims::stage_local_field_read_batch(&mut local_staged_reads, rows)
                            .map(Some)
                    };
                    let base_completion = crate::mir::resolved_control_flow::verify_function_completion_v1(input);
                    if let Ok(ref completion) = base_completion {
                        if loop_static_source_loans.selected_sites.iter().any(|(owner, _)| *owner == input.owner()) {
                            let product = super::loop_static_preissue::issue_selected_product_v2(
                                input, completion, &mut loop_static_source_loans,
                            ).and_then(|row| row.ok_or_else(|| "[freeze:contract][callable-loop/static-i64-v2/source-unavailable]".to_owned()));
                            if preissued_static_i64_products.insert(input.owner(), product).is_some() {
                                return Err(OrdinaryNewCoSealIssueV1::BatchLoan);
                            }
                        }
                    }
                    match crate::mir::resolved_control_flow::verify_function_completion_with_new_homes_and_argument_observations_v1(
                        input, Some(base_completion), &new_sites,
                        batch_params()
                            .map(|row| (row.ordinal, row.binding, row.kind.clone())),
                        entry_home,
                        borrowed_formal_source.as_ref().ok().and_then(|source| {
                            source.lend_source_value_class_facts_v1(input.owner())
                        }),
                        &mut field_is_integer, &mut |site, binding| {
                            source_claims::ordinary_candidate_compatible_v1(&candidates, site, binding)
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
                        }, &mut |site| {
                            Ok(owner_loan.is_some_and(|loan| {
                                loan.is_handle_result_call(batch, parameter_contracts, input, site)
                            }) || lexical_handle_result_call(
                                selected,
                                batch,
                                parameter_contracts,
                                &candidates,
                                input,
                                site,
                            ) || lexical::has_object_receiver_call_at_v1(&receiver_call_observations, site))
                        }, &mut |site| {
                            // Lexical `recv.m(..)` i64-result membership —
                            // the claim-local receiver, exact-i64 formals,
                            // and the literal-only callee exits are all
                            // proved from sealed facts before the claim.
                            Ok(lexical_i64_result_call(
                                selected,
                                batch,
                                parameter_contracts,
                                &callable_result_classes,
                                &candidates,
                                input,
                                site,
                            ))
                        }, &mut |site| {
                            // Lexical `recv.m(..)` nullable-result
                            // membership — claim-local receiver plus the
                            // callee's sealed `NullableObject` claim and
                            // its null/construction return sites. The
                            // `me.m(..)` receiver observation stays a
                            // separate lane below.
                            Ok(lexical_nullable_result_call(
                                selected,
                                batch,
                                &callable_result_classes,
                                &candidates,
                                input,
                                site,
                            ))
                        }, &mut |site| {
                            // Nullable `me.m(..)` membership is the sealed
                            // package observation map restricted to the
                            // `NullableObject` class — an `Object` claim
                            // stays on the Handle lane, and the flow row
                            // never re-reads the expression or a callee claim.
                            Ok(matches!(
                                receiver_call_observations
                                    .get(site)
                                    .filter(|row| row.map_stored_source().is_some())
                                    .map(|row| row.class()),
                                Some(result_class_claim::OrdinaryNewResultClassV1::NullableObject(_))
                            ))
                        }, &mut local_static_call, &result_sites, &mut argument_field_is_integer, &mut source_claims::scalar_fields_with_original_writes(static_caller_key.as_ref(), parameter_contracts, instance_constructors, receiver_proof, entry_home, &borrowed_i64_results, &borrowed_formal_source, &candidates, &field_write_claims, |site: &OwnedExprSiteV1, _: &SourceExprSiteV1, _: BindingRefV1, home, name| {
                            // RHS `me.<field>` reads inside a field write
                            // share the entry-receiver proof; the contract
                            // is the numeric integer-scalar set, not `i64`.
                            terminal_home::receiver_scalar_field(
                                instance_constructors, receiver_proof, site, home, name,
                            ).map(|field| field.is_some())
                        }), &mut |site: &OwnedExprSiteV1, _: &SourceExprSiteV1, _: BindingRefV1, home, name| {
                            // `me.<ArrayBox field>.m(..)` receiver proof
                            // shares the entry receiver root; the declared
                            // `ArrayBox` contract is the only admitted type.
                            terminal_home::receiver_container_field(
                                instance_constructors, receiver_proof, site, home, name,
                            ).map(|field| field.is_some())
                        }, &mut |site: &OwnedExprSiteV1, _: &SourceExprSiteV1, _: BindingRefV1, home, name| {
                            // `me.<ArrayBox field>.m(..)` receiver proof
                            // adds the sealed whole-Box element-integer
                            // census: the declared type alone never carries
                            // an i64 element contract.
                            terminal_home::receiver_array_i64_field_from_census_v1(
                                instance_constructors, receiver_proof, &array_i64_field_sets,
                                site, home, name,
                            )
                        }, &mut source_claims::formal_i64_index_consult_v1(
                            input, instance_constructors, batch.ordinary_box_coverage(),
                        ), &mut local_field_read, &mut |site, actuals| {
                            source_claims::source_call_arguments_v1(
                                input, brands, batch, selected, instance_constructors, receiver_proof, entry_home,
                                &callable_result_classes, &field_write_claims, &lexical_source_targets, dynamic_source_calls,
                                parameter_contracts, &candidates, &borrowed_formal_source, &static_source_sites,
                                &mut borrowed_formal_actuals, &borrowed_i64_results, &receiver_call_observations,
                                &seeds, site, actuals, true,
                            )
                        }, &mut source_claims::dominated_view_use_consult_v1(
                            &borrowed_formal_source, input, &borrowed_i64_results, &array_i64_field_sets,
                            instance_constructors, batch.ordinary_box_coverage(),
                        ), &mut |prestate| {
                            let loan = loop_issue::issue_selected_home_loop_loan_v1(
                                input, prestate, &loop_static_source_loans.selected_sites,
                                &preissued_static_i64_products, &mut selected_loop_products, source_core_calls, loop_policy,
                            );
                            Ok(loan)
                        }, &mut |site| Ok(callable_result_classes.object_return_qualification(site)))? {
                        Ok((
                            completion,
                            prefixes,
                            terminal_relation,
                            observations,
                            result_prefixes,
                        )) => {
                            if let Some(issue) = completion.cleanup().root_flow()
                                .and_then(|flow| flow.explicit_move_failure()) {
                                return Err(OrdinaryNewCoSealIssueV1::RootTerminalSource(issue));
                            }
                            field_reads::merge_staged_argument_field_reads(
                                &mut argument_field_reads,
                                input.owner(),
                                &observations,
                                argument_staged_reads,
                            )?;
                            field_reads::merge_staged_local_field_reads(
                                &mut local_field_reads,
                                input.owner(),
                                local_staged_reads,
                            )?;
                            if is_app_main {
                                if completion
                                    .cleanup()
                                    .root_flow()
                                    .is_some_and(|flow| flow.all_exits_ready())
                                {
                                    for terminal in terminal_relation.values() {
                                        match terminal {
                                            TerminalRelationV1::I64Add(result) => {
                                                if result.owner() != input.owner()
                                                    || result.field_reads().iter().any(|site|
                                                        !staged_reads.contains_key(site))
                                                {
                                                    return Err(OrdinaryNewCoSealIssueV1::TerminalResultFieldReadMissing {
                                                        site: result.add_site().clone(),
                                                    });
                                                }
                                            }
                                            TerminalRelationV1::I64Field(result) => {
                                                if result.owner() != input.owner()
                                                    || !staged_reads.contains_key(result.field_read_site())
                                                {
                                                    return Err(OrdinaryNewCoSealIssueV1::TerminalResultFieldReadMissing {
                                                        site: result.field_read_site().clone(),
                                                    });
                                                }
                                            }
                                            _ => {}
                                        }
                                    }
                                    field_reads::merge_staged_field_reads(
                                        &mut field_reads,
                                        input.owner(),
                                        staged_reads,
                                    )?;
                                    root_terminal_relation.extend(terminal_relation);
                                }
                                root_completion = Some(Ok(Rc::new(completion)));
                            } else {
                                field_reads::merge_terminal_relation_field_reads(
                                    &mut field_reads,
                                    input.owner(),
                                    &terminal_relation,
                                    staged_reads,
                                )?;
                                let relation = retain_child_terminal_relations_v1(
                                    terminal_relation, owner, has_map, |row| {
                                    let qualified = match row {
                                        TerminalRelationV1::Call(call) => local_static_call(&OwnedExprSiteV1::new(owner, call.call_site().clone()))?
                                            .is_some_and(|claim| claim.is_qualified_terminal()),
                                        _ => false,
                                    };
                                    Ok(qualified || borrowed_formal_source.as_ref().is_ok_and(|source| source.retains_borrowed_formal_terminal_v1(row))
                                        || matches!(row, TerminalRelationV1::Call(call)
                                            if call.owner() == owner && static_caller_key.as_ref().is_some_and(|key|
                                                static_call_claims.current_owner_i64_source(key, call.call_site()).is_some()))
                                        // An instance call whose scalar result was lent by the package keeps
                                        // its relation when the callee's scalar proof verified; the root-home
                                        // exit later requires the same proof (`scalar-result-source-missing`).
                                        || matches!(row, TerminalRelationV1::Call(call)
                                            if call.owner() == owner && call.source_scalar().is_some_and(|loan|
                                                borrowed_i64_results.get(&loan.source().callee_owner()).is_some_and(|proof| proof.is_ok())))
                                        || loop_issue::retain_static_i64_search_early_return_v1(
                                            input, static_caller_key.as_ref(), static_call_claims,
                                            &preissued_static_i64_products, row))
                                })?;
                                // An original nested-call walk may lend actual
                                // source facts without selecting an executable
                                // callable or issuing its physical header.
                                if selected.role_for_batch_slot(batch_slot).is_some() {
                                    seeds.push_completion_with_top_level_input(
                                        declaration, selected, Rc::new(completion), relation, top_level_input.take(),
                                    ).map_err(OrdinaryNewCoSealIssueV1::CompletionSeed)?;
                                }
                            }
                            (prefixes, observations, result_prefixes)
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
                                input, &new_sites, &result_sites,
                                batch_params()
                                    .map(|row| (row.ordinal, row.binding, row.kind.clone())),
                                entry_home,
                                borrowed_formal_source.as_ref().ok().and_then(|source| {
                                    source.lend_source_value_class_facts_v1(input.owner())
                                }),
                            )
                        }
                    }
                } else {
                    issue_new_home_prefixes_with_arguments_v1(
                        input, &new_sites, &result_sites,
                        batch_params()
                            .map(|row| (row.ordinal, row.binding, row.kind.clone())),
                        entry_home,
                        borrowed_formal_source.as_ref().ok().and_then(|source| {
                            source.lend_source_value_class_facts_v1(input.owner())
                        }),
                    )
                };
                Ok((
                    candidates,
                    home_prefixes,
                    argument_observations,
                    result_resolutions,
                    result_prefixes,
                ))
            })
            .map_err(|_| OrdinaryNewCoSealIssueV1::BatchLoan)??;
        source_claims::append_source_claims(
            batch,
            instance_constructors,
            &field_residences,
            candidates,
            result_resolutions,
            home_prefixes,
            argument_observations,
            result_prefixes,
            &mut claims,
            &mut result_claims,
            &mut birth_abi_handoffs,
            &mut owned_field_children,
        )?;
    }
    super::field_array_read_source::retain_argument_reads(
        batch,
        selected,
        instance_constructors,
        &borrowed_i64_results,
        &array_i64_field_sets,
        &mut claims,
        &mut result_claims,
    )?;
    // Constructor (birth) rows are not batch declarations — they join
    // through the program-source loan, so the index collects them here.
    // Their owners never collide with declaration-owned claim sites;
    // the claimed set is still passed defensively.
    let claimed_sites: BTreeSet<OwnedExprSiteV1> = claims
        .iter()
        .map(|claim| claim.site().clone())
        .chain(result_claims.iter().map(|claim| claim.core.site.clone()))
        .collect();
    source_claims::collect_constructor_birth_sites_v1(
        batch,
        instance_constructors,
        &claimed_sites,
        &mut birth_site_index,
    )?;

    let mut ledger = OrdinaryNewClaimLedgerV1::issue(
        claims.into_boxed_slice(),
        result_claims.into_boxed_slice(),
        names,
    );
    ledger
        .retain_source_dynamic_member_calls_v1(dynamic_source_calls)
        .map_err(|_| OrdinaryNewCoSealIssueV1::BatchLoan)?;
    ledger.lexical_source_targets = Some(lexical_source_targets);
    ledger.borrowed_static_source_sites = Some(static_source_sites);
    ledger.selected_loop_products = selected_loop_products;
    ledger.install_borrowed_formal_preparation_v1(
        borrowed_formal_source,
        borrowed_formal_actuals,
        borrowed_i64_results,
    );
    ledger.receiver_call_observations = receiver_call_observations;
    ledger.field_write_claims = field_write_claims;
    source_claims::install_result_classes_with_argument_loans_v1(
        &mut ledger,
        callable_result_classes,
    )?;
    ledger.array_i64_fields = array_i64_field_sets;
    ledger.birth_site_index = std::cell::RefCell::new(birth_site_index);
    ledger.root_completion = root_completion;
    ledger.field_reads = std::cell::RefCell::new(field_reads);
    ledger.argument_field_reads = std::cell::RefCell::new(argument_field_reads);
    ledger.local_field_reads = std::cell::RefCell::new(local_field_reads);
    ledger.birth_abi_handoffs = std::cell::RefCell::new(birth_abi_handoffs);
    ledger.owned_field_children = owned_field_children;
    ledger.terminal_relation = std::rc::Rc::new(root_terminal_relation);
    ledger.retain_app_main_source_v1(app_main_source, batch, parameter_contracts)?;
    ledger.install_loop_static_source_loans_v1(loop_static_source_loans);
    *ledger.preissued_static_i64_products.get_mut() = preissued_static_i64_products;
    Ok((ledger, seeds.finish()))
}
