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
use super::coseal_helpers::{convert_selected_new_arguments, retain_child_terminal_relation};
use super::{
    field_reads, field_write_claim, receiver_call_observation, result_class_claim, terminal_home,
};
use super::{
    OrdinaryNewAdmissionClaimV1, OrdinaryNewClaimCoreV1, OrdinaryNewClaimLedgerV1,
    OrdinaryNewCoSealIssueV1, OrdinaryNewResultClaimV1, OwnedFieldChildKindV1, OwnedFieldChildV1,
    VerifiedOrdinaryNewBirthRecipeV1,
};
use crate::ast::ASTNode;
use crate::mir::builder::{CanonicalSameModuleCallableKeyV1, SelectedNormalCallableKeyV1};
use crate::mir::callable_semantic_batch::VerifiedResolvedCallableSemanticBatchV1;
use crate::mir::function::ObjectDestructionDispositionV1;
use crate::mir::resolved_semantics::home_new_prefix::{
    issue_new_home_prefixes_with_arguments_v1, HomePrefixUnavailableV1,
    SelectedNewArgumentUnavailableV1, TerminalRelationV1,
};
use crate::mir::resolved_semantics::{
    BindingRefV1, FunctionOwnerIdV1, OwnedExprSiteV1, SourceBindingSiteV1, SourceExprSiteV1,
    SourceNodeSiteV1, SourcePathSegmentV1, SourceStmtSiteV1, VerifiedResolvedFunctionV1,
};
use hakorune_mir_defs::SameModuleCallableNamespaceV1;

#[path = "ordinary_new_coseal_issue_lexical.rs"]
mod lexical;
use lexical::{
    borrowed_call_arguments_callback_v1, lexical_handle_result_call, lexical_i64_result_call,
    lexical_nullable_result_call,
};
// The borrowed-formal object view resolves received-nullable classes with
// the same sealed `NullableObject` claim lookup as the field issuer.
pub(in crate::mir::normal_callable_semantic_package) use lexical::nullable_received_result_class;

#[path = "ordinary_new_coseal_issue_source.rs"]
mod source_claims;
use source_claims::collect_birth_site_index_v1;

pub(in crate::mir::normal_callable_semantic_package) fn issue_ordinary_source_cohort_v1(
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    selected: &VerifiedSelectedCallableBatchMapV1,
    app_main_identity: Option<&crate::parser::CallableDeclarationIdentityV1>,
    direct_call_loans: Option<&super::super::direct_call_loan::DirectCallDispositionLoansV1>,
    parameter_contracts: &[super::super::model::OwnedCallableParameterContractDeclarationV1],
    entry_home_loans: &crate::mir::resolved_semantics::VerifiedInstanceEntryHomeCatalogV1,
    dynamic: &mut super::super::model::NormalCallableDynamicProjectionV1,
    instance_constructors: &VerifiedInstanceConstructorSemanticBatchV1,
    static_call_claims: &super::super::qualified_static_call_claim::QualifiedStaticCallClaimIndexV1,
    // App Main's canonical catalog key — App Main is never a selected
    // row, so its claim-lookup key comes from the catalog co-seal.
    app_main_claim_key: Option<&CanonicalSameModuleCallableKeyV1>,
) -> Result<
    (
        OrdinaryNewClaimLedgerV1,
        super::super::result_contract::VerifiedCallableResultContractBuilderV1,
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
    // Owned field children per canonical object: `Some` means every
    // residence-capable declared field has a sealed birth-side residence,
    // `None` means the disposition needs children the package never proved.
    let (
        field_write_claims,
        field_residences,
        callable_result_classes,
        mut owned_field_children,
    ) = source_claims::prepare_source_claims(
        batch, selected, instance_constructors, parameter_contracts,
    )?;
    let dynamic_slot = match dynamic {
        super::super::model::NormalCallableDynamicProjectionV1::Selected { batch_slot, .. } => {
            Some(*batch_slot)
        }
        _ => None,
    };
    let names: Box<[Box<str>]> = batch
        .ordinary_box_coverage()
        .rows()
        .iter()
        .map(|row| row.name().to_owned().into_boxed_str())
        .collect();
    let mut local_candidates = source_claims::prepare_local_candidates_by_slot_v1(
        batch,
        selected,
        instance_constructors,
        app_main_batch_slot,
        dynamic_slot,
    );
    let new_classes = local_candidates
        .values()
        .filter_map(|rows| rows.as_ref().ok())
        .flatten()
        .map(|candidate| (candidate.site.clone(), candidate.class.clone()))
        .collect();
    let lexical_source_targets = super::lexical_instance_call::prepare_lexical_source_targets_v1(
        batch,
        selected,
        &new_classes,
        &names,
        &field_write_claims,
        &callable_result_classes,
    );
    let borrowed_formal_source = super::lexical_instance_call::prepare_borrowed_formal_ingress_v1(
        batch,
        selected,
        parameter_contracts,
        &lexical_source_targets,
        app_main_batch_slot,
        dynamic_slot,
        entry_home_loans,
        instance_constructors,
        &local_candidates,
        &callable_result_classes,
    );
    let borrowed_i64_results = super::lexical_instance_call::prepare_borrowed_i64_results_v1(
        &borrowed_formal_source,
        batch,
        parameter_contracts,
        instance_constructors,
    );
    for declaration in batch.declarations() {
        let owner = declaration.owner();
        let batch_slot = declaration.batch_slot();
        let is_app_main = app_main_batch_slot == Some(batch_slot);
        if selected.role_for_batch_slot(batch_slot).is_none() && !is_app_main {
            continue;
        }
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
                let function = input.function();
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
                // Return-position `new` membership: the construction is the
                // exact `ReturnValue` child of an inventoried `Return`
                // statement. Position is checked against the source
                // statement, never inferred from lowered MIR. Builtin or
                // uncovered classes stay outside this family and keep their
                // existing terminal; argument/field positions stay rejected.
                let mut result_resolutions = Vec::new();
                let mut result_sites = BTreeSet::new();
                for construction in function.expression_source().constructions() {
                    let segments = construction.site().node().segments();
                    let Some((SourcePathSegmentV1::Value, parent)) = segments.split_last() else {
                        continue;
                    };
                    let site = OwnedExprSiteV1::new(owner, construction.site().clone());
                    if candidates.iter().any(|row| row.site == site) || !result_sites.insert(site.clone()) {
                        if candidates.iter().any(|row| row.site == site) {
                            continue;
                        }
                        return Err(OrdinaryNewCoSealIssueV1::DuplicateSite { site });
                    }
                    let parent_site = SourceStmtSiteV1::from_node(
                        SourceNodeSiteV1::from_segments(parent.to_vec()),
                    );
                    // A `Value` child of a non-statement parent (for example a
                    // nested construction inside a field initializer) is not
                    // return-position membership — leave the site out.
                    let Ok(statement) = input.source().exact_stmt(&parent_site) else {
                        result_sites.remove(&site);
                        continue;
                    };
                    if !matches!(statement.node(), ASTNode::Return { value: Some(_), .. }) {
                        result_sites.remove(&site);
                        continue;
                    }
                    if let Some(resolution) = OrdinaryNewCandidate::resolve_site(
                        batch,
                        instance_constructors,
                        site.clone(),
                        construction.class().into(),
                        construction.arguments().len(),
                        !construction.field_initializers().is_empty(),
                    )? {
                        result_resolutions.push(resolution);
                    } else {
                        // Uncovered (builtin) classes keep their existing
                        // raw-lane terminal outside this family.
                        result_sites.remove(&site);
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
                }) || batch_params().any(|row| {
                    row.kind
                        == crate::mir::callable_parameter_contract::CallableParameterContractKindV1::Map
                });
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
                );
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
                let mut local_static_call =
                    super::super::qualified_static_call_claim::local_static_call_predicate(
                        static_call_claims,
                        super::super::qualified_static_call_claim::caller_key_for_function(
                            selected, batch_slot, is_app_main, app_main_claim_key,
                        ),
                    );
                let has_borrowed_source_calls = borrowed_formal_source.as_ref().is_ok_and(|rows| {
                    rows.incoming_calls_for_owner(input.owner())
                });
                let has_formal_field_read = borrowed_formal_source.as_ref().is_ok_and(|rows| {
                    rows.formal_field_read_target(input.owner())
                });
                let mut probe = |explicit_sites: &[crate::mir::resolved_semantics::SourceStmtSiteV1],
                                 pending_actuals: &mut super::lexical_instance_call::PendingBorrowedFormalActualsV1| {
                    source_claims::probe_source_home_prefixes_v1(
                        input, &new_sites, entry_home, explicit_sites, pending_actuals,
                        batch_slot, selected, batch, parameter_contracts,
                        &callable_result_classes, &candidates, instance_constructors,
                        receiver_proof, &borrowed_formal_source, &lexical_source_targets, &borrowed_i64_results, &mut local_static_call,
                    )
                };
                let readiness = if seed_eligible && !new_sites.is_empty() {
                    // The probe must predict the verified-completion lane:
                    // without its explicit exit sites a `return` inside a
                    // guard arm reads as `PrefixNotCovered` and poisons the
                    // joined prefix before the selected `new` is walked.
                    match crate::mir::resolved_control_flow::verify_function_completion_v1(input) {
                        Ok(control) => probe(control.explicit_sites(), &mut BTreeMap::new())?
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
                // observations keep today's generic call floor.
                let has_nullable_receiver_call = receiver_call_observations
                    .iter()
                    .any(|(site, row)| {
                        site.owner() == owner
                            && matches!(
                                row.class(),
                                result_class_claim::OrdinaryNewResultClassV1::NullableObject(_)
                            )
                    });
                let seed_completion = seed_eligible
                    && !has_map
                    && !has_nullable_receiver_call
                    && !has_formal_field_read
                    && !child_new_ready
                    && !child_result_ready
                    && (is_app_main || owner_loan.is_none());
                let app_main_integer_result = is_app_main
                    && candidates.is_empty()
                    && result_sites.is_empty()
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
                            root_terminal_relation.insert(
                                relation.return_site().clone(),
                                TerminalRelationV1::IntegerLiteral(relation),
                            );
                        }
                    }
                    if seed_completion {
                        seeds.push_completion(
                            declaration,
                            selected,
                            Rc::clone(&completion),
                            BTreeMap::new(),
                        )
                        .map_err(OrdinaryNewCoSealIssueV1::CompletionSeed)?;
                    }
                }
                // A callee whose sealed use draft admits a dominated
                // `formal.field` read needs the verified walk too: the read
                // is issued only by this lane's `local_field_read` authority
                // — the bounded sibling scan keeps it truthfully unavailable.
                let verified_walk = owner_loan.is_some() || has_nullable_receiver_call || has_formal_field_read || (is_app_main && (!new_sites.is_empty() || has_map || !result_sites.is_empty())) || (seed_eligible && (has_map || child_new_ready || child_result_ready));
                if !verified_walk && has_borrowed_source_calls {
                    // Borrow existing control authority without publishing Completion.
                    let observed = crate::mir::resolved_control_flow::verify_function_completion_v1(input)
                        .map_err(|issue| format!("borrowed-actual/control-unavailable: {issue:?}"))
                        .and_then(|control| probe(control.explicit_sites(), &mut borrowed_formal_actuals)
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
                    let mut field_is_integer = |site: &OwnedExprSiteV1, receiver_site: &SourceExprSiteV1, receiver, home, name: &str| {
                        let field = terminal_home::initialized_integer_field(
                            instance_constructors, &candidates, home, name)?;
                        // A non-null-narrowed received nullable names its
                        // class through the sealed call's `NullableObject`
                        // claim; a guarded borrowed formal names it through
                        // the co-sealed object view — the declared `i64`
                        // contract and `field_home`'s non-null proof are
                        // unchanged.
                        let field = match field {
                            Some(field) => Some(field),
                            None => match lexical::nullable_received_result_class(
                                selected, batch, &callable_result_classes, &candidates, input, home,
                            )
                            .or_else(|| {
                                borrowed_formal_source
                                    .as_ref()
                                    .ok()
                                    .and_then(|source| source.formal_object_view(home))
                                    .map(|view| view.class().into())
                            }) {
                                Some(class) => terminal_home::nullable_result_integer_field(
                                    instance_constructors,
                                    batch.ordinary_box_coverage(),
                                    &class,
                                    site,
                                    name,
                                )?,
                                None => None,
                            },
                        };
                        let Some(field) = field else { return Ok(false); };
                        if staged_reads.insert(site.clone(), field_reads::FieldRead {
                            receiver_site: receiver_site.clone(), receiver, home, field,
                            progress: field_reads::Progress::Pending,
                        }).is_some() { return Err(OrdinaryNewCoSealIssueV1::DuplicateSite { site: site.clone() }); }
                        Ok(true)
                    };
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
                    let mut local_field_read = |requests: &[crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadRequestV1], scalar_only| {
                        let Some(rows) = source_claims::prove_local_field_read_batch(
                            instance_constructors, &candidates, batch.ordinary_box_coverage(),
                            receiver_proof, requests, scalar_only,
                            &mut |home| lexical::nullable_received_result_class(
                                selected, batch, &callable_result_classes, &candidates, input, home,
                            ),
                            &mut |home| {
                                borrowed_formal_source
                                    .as_ref()
                                    .ok()
                                    .and_then(|source| source.formal_object_view(home))
                                    .map(|view| view.class().into())
                            },
                        )? else { return Ok(None); };
                        source_claims::stage_local_field_read_batch(&mut local_staged_reads, rows)
                            .map(Some)
                    };
                    match crate::mir::resolved_control_flow::verify_function_completion_with_new_homes_and_argument_observations_v1(
                        input, &new_sites,
                        batch_params()
                            .map(|row| (row.ordinal, row.binding, row.kind.clone())),
                        entry_home,
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
                            ))
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
                                    .map(|row| row.class()),
                                Some(result_class_claim::OrdinaryNewResultClassV1::NullableObject(_))
                            ))
                        }, &mut local_static_call, &result_sites, &mut argument_field_is_integer, &mut |site: &OwnedExprSiteV1, _: &SourceExprSiteV1, _: BindingRefV1, home, name| {
                            // RHS `me.<field>` reads inside a field write
                            // share the entry-receiver proof; the contract
                            // is the numeric integer-scalar set, not `i64`.
                            terminal_home::receiver_scalar_field(
                                instance_constructors, receiver_proof, site, home, name,
                            ).map(|field| field.is_some())
                        }, &mut |site: &OwnedExprSiteV1, _: &SourceExprSiteV1, _: BindingRefV1, home, name| {
                            // `me.<ArrayBox field>.m(..)` receiver proof
                            // shares the entry receiver root; the declared
                            // `ArrayBox` contract is the only admitted type.
                            terminal_home::receiver_container_field(
                                instance_constructors, receiver_proof, site, home, name,
                            ).map(|field| field.is_some())
                        }, &mut local_field_read, &mut |site, actuals| {
                            borrowed_call_arguments_callback_v1(
                                &lexical_source_targets, parameter_contracts, &candidates, receiver_proof,
                                &borrowed_formal_source, &mut borrowed_formal_actuals, &borrowed_i64_results,
                                &mut |binding| lexical::nullable_received_result_class(
                                    selected, batch, &callable_result_classes, &candidates, input, binding,
                                ),
                                site, actuals,
                            )
                        }, &mut source_claims::dominated_view_use_consult_v1(
                            &borrowed_formal_source,
                        ))? {
                        Ok((
                            completion,
                            prefixes,
                            terminal_relation,
                            observations,
                            result_prefixes,
                        )) => {
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
                                let relation = terminal_relation
                                    .into_iter()
                                    .filter(|(_, row)| retain_child_terminal_relation(row, has_map))
                                    .collect();
                                seeds.push_completion(declaration, selected, Rc::new(completion), relation)
                                    .map_err(OrdinaryNewCoSealIssueV1::CompletionSeed)?;
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
                            )
                        }
                    }
                } else {
                    issue_new_home_prefixes_with_arguments_v1(
                        input, &new_sites, &result_sites,
                        batch_params()
                            .map(|row| (row.ordinal, row.binding, row.kind.clone())),
                        entry_home,
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
    // Constructor (birth) rows are not batch declarations — they join
    // through the program-source loan, so the index collects them here.
    // Their owners never collide with declaration-owned claim sites;
    // the claimed set is still passed defensively.
    let claimed_sites: BTreeSet<OwnedExprSiteV1> = claims
        .iter()
        .map(|claim| claim.site().clone())
        .chain(result_claims.iter().map(|claim| claim.core.site.clone()))
        .collect();
    batch
        .with_normal_program_source_loan(|loan| -> Result<(), OrdinaryNewCoSealIssueV1> {
            for row in instance_constructors.rows() {
                let input = row
                    .lowering_input(loan.program())
                    .map_err(|_| OrdinaryNewCoSealIssueV1::BatchLoan)?;
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

    let mut ledger = OrdinaryNewClaimLedgerV1::issue(
        claims.into_boxed_slice(),
        result_claims.into_boxed_slice(),
        names,
    );
    ledger.lexical_source_targets = Some(lexical_source_targets);
    ledger.install_borrowed_formal_preparation_v1(
        borrowed_formal_source,
        borrowed_formal_actuals,
        borrowed_i64_results,
    );
    ledger.receiver_call_observations = receiver_call_observations;
    ledger.field_write_claims = field_write_claims;
    ledger.callable_result_classes = callable_result_classes;
    ledger.birth_site_index = std::cell::RefCell::new(birth_site_index);
    ledger.root_completion = root_completion;
    ledger.field_reads = std::cell::RefCell::new(field_reads);
    ledger.argument_field_reads = std::cell::RefCell::new(argument_field_reads);
    ledger.local_field_reads = std::cell::RefCell::new(local_field_reads);
    ledger.birth_abi_handoffs = std::cell::RefCell::new(birth_abi_handoffs);
    ledger.owned_field_children = owned_field_children;
    ledger.terminal_relation = root_terminal_relation;
    ledger.app_main_identity = app_main_identity.cloned();
    Ok((ledger, seeds.finish()))
}
