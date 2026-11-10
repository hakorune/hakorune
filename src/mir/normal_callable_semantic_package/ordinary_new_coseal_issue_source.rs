//! Source-claim preparation and assembly for the ordinary-New cohort.
//! This private move preserves source loans, claim order and failure boundaries.
use super::super::coseal_helpers::is_direct_local_initializer;
use super::super::BirthAbiHandoffV1;
use super::*;
use crate::ast::ASTNode;
use crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call::BorrowedResultCohortV1;
use crate::mir::resolved_semantics::home_new_prefix::{
    CallerNewHomePrefixV1, ResultNewHomePrefixV1, SelectedNewArgumentObservationV1,
};
use crate::mir::resolved_semantics::BindingKindV1;

#[path = "ordinary_new_argument_child_relations.rs"]
mod argument_relations;
pub(super) use argument_relations::install_result_classes_with_argument_loans_v1;

#[path = "ordinary_new_named_array_source_selection.rs"]
mod named_array_selection;
pub(super) use named_array_selection::{
    named_array_initializer_source_v1, select_failed_array_initializer_v1,
};

#[path = "ordinary_new_source_claim_preparation.rs"]
mod preparation;
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal) use preparation::{prepare_source_claims, require_declared_borrowed_results_v1};

/// Discover direct local construction candidates from this exact source loan.
/// This produces passive class/constructor identity only; prefix availability,
/// borrowing and lifecycle permission remain the subsequent walk's responsibility.
pub(super) fn collect_local_candidates_v1(
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    instance_constructors: &VerifiedInstanceConstructorSemanticBatchV1,
    input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
) -> Result<Vec<OrdinaryNewCandidate>, OrdinaryNewCoSealIssueV1> {
    let function = input.function();
    let owner = input.owner();
    let mut candidates = Vec::new();
    for initializer in function.expression_source().initializers() {
        let Some(initializer_site) = initializer.initializer_site() else {
            continue;
        };
        if !is_direct_local_initializer(initializer_site.node().segments()) {
            continue;
        }
        let site = OwnedExprSiteV1::new(owner, initializer_site.clone());
        let located = input
            .source()
            .expr_at(&site)
            .map_err(|_| OrdinaryNewCoSealIssueV1::SourceNavigation { site: site.clone() })?;
        let ASTNode::New {
            class,
            arguments,
            field_initializers,
            ..
        } = located.node()
        else {
            continue;
        };
        if initializer.binding().owner() != owner
            || function.declaration_binding(initializer.declaration_site())
                != Some(initializer.binding())
            || !matches!(
                function
                    .binding(initializer.binding())
                    .map(|row| row.kind()),
                Some(BindingKindV1::Local { .. })
            )
        {
            return Err(OrdinaryNewCoSealIssueV1::InitializerBindingMismatch { site });
        }
        if let Some(candidate) = OrdinaryNewCandidate::resolve(
            batch,
            instance_constructors,
            site,
            class.clone().into_boxed_str(),
            arguments.len(),
            initializer.binding(),
            initializer.declaration_site().clone(),
            !field_initializers.is_empty(),
        )? {
            candidates.push(candidate);
        }
    }
    Ok(candidates)
}

/// Calculate once, but keep every slot's success/error at its original walk boundary.
/// No partial candidate vector from a failed slot is exposed to class preparation.
pub(super) fn prepare_local_candidates_by_slot_v1(
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    selected: &VerifiedSelectedCallableBatchMapV1,
    omitted_static_callers: &std::collections::BTreeMap<crate::mir::resolved_semantics::FunctionOwnerIdV1, crate::mir::builder::CanonicalSameModuleCallableKeyV1>,
    instance_constructors: &VerifiedInstanceConstructorSemanticBatchV1,
    app_main_batch_slot: Option<u32>,
    dynamic_slot: Option<u32>,
) -> BTreeMap<u32, Result<Vec<OrdinaryNewCandidate>, OrdinaryNewCoSealIssueV1>> {
    batch
        .declarations()
        .filter_map(|declaration| {
            let slot = declaration.batch_slot();
            if dynamic_slot == Some(slot) { return None; }
            match super::walk_triggers::needs_declaration_walk_v1(batch, selected, omitted_static_callers, app_main_batch_slot, slot) {
                Ok(false) => return None,
                Err(issue) => return Some((slot, Err(issue))),
                Ok(true) => {}
            }
            let candidates = batch
                .with_lowering_input(slot, |input| {
                    collect_local_candidates_v1(batch, instance_constructors, input)
                })
                .map_err(|_| OrdinaryNewCoSealIssueV1::BatchLoan)
                .and_then(|rows| rows);
            Some((slot, candidates))
        })
        .collect()
}

pub(super) fn append_source_claims(
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    instance_constructors: &VerifiedInstanceConstructorSemanticBatchV1,
    field_residences: &field_write_claim::OwnedFieldResidencesV1,
    candidates: Vec<OrdinaryNewCandidate>,
    result_resolutions: Vec<OrdinaryNewSiteResolutionV1>,
    mut home_prefixes: BTreeMap<
        OwnedExprSiteV1,
        Result<CallerNewHomePrefixV1, HomePrefixUnavailableV1>,
    >,
    mut argument_observations: BTreeMap<OwnedExprSiteV1, SelectedNewArgumentObservationV1>,
    mut result_prefixes: BTreeMap<
        OwnedExprSiteV1,
        Result<ResultNewHomePrefixV1, HomePrefixUnavailableV1>,
    >,
    claims: &mut Vec<OrdinaryNewAdmissionClaimV1>,
    result_claims: &mut Vec<OrdinaryNewResultClaimV1>,
    birth_abi_handoffs: &mut BTreeMap<OwnedExprSiteV1, BirthAbiHandoffV1>,
    owned_field_children: &mut BTreeMap<
        hakorune_mir_defs::CanonicalObjectIdV1,
        Option<Box<[OwnedFieldChildV1]>>,
    >,
) -> Result<(), OrdinaryNewCoSealIssueV1> {
    for candidate in candidates {
        let OrdinaryNewCandidate {
            site,
            box_source,
            class,
            arity,
            destination,
            declaration,
            construction,
            construction_source,
            allocation_execution,
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
        if let super::super::OrdinaryNewConstructorDispositionV1::Birth(_) = &constructor {
            if let Some(birth) = instance_constructors.birth_for(&box_source, arity).ok().flatten() {
                use crate::mir::normal_callable_semantic_package::ordinary_new_coseal::candidate as take;
                take::validate_take_declarations_v1(&site, &class, birth)?;
                if let Ok(rows) = &argument_rows {
                    take::validate_take_actuals_v1(&site, &class, birth, rows)?;
                }
            }
        }
        if claims
            .iter()
            .any(|claim: &OrdinaryNewAdmissionClaimV1| claim.core.site == site)
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
        let children = {
            let mut visiting = std::collections::BTreeSet::new();
            owned_field_children_of(
                &site,
                batch,
                instance_constructors,
                &box_source,
                destruction,
                &field_residences,
                owned_field_children,
                &mut visiting,
            )?
        };
        if (destruction).requires_field_end() {
            match owned_field_children.entry(object) {
                std::collections::btree_map::Entry::Vacant(entry) => {
                    entry.insert(children.clone());
                }
                std::collections::btree_map::Entry::Occupied(entry) if *entry.get() == children => {
                }
                std::collections::btree_map::Entry::Occupied(_) => {
                    return Err(OrdinaryNewCoSealIssueV1::DuplicateSite { site });
                }
            }
        }
        claims.push(OrdinaryNewAdmissionClaimV1 {
            core: OrdinaryNewClaimCoreV1 {
                site: site.clone(),
                box_source,
                class,
                arity,
                constructor,
                construction,
                construction_source,
                allocation_execution,
                object,
                destruction,
                argument_rows,
                children,
            },
            destination,
            declaration,
            home_prefix,
        });
    }
    for resolution in result_resolutions {
        let OrdinaryNewSiteResolutionV1 {
            site,
            box_source,
            class,
            arity,
            construction,
            construction_source,
            object,
            destruction,
            constructor,
            birth_handoff,
        } = resolution;
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
            .any(|claim: &OrdinaryNewAdmissionClaimV1| claim.core.site == site)
            || result_claims
                .iter()
                .any(|claim: &OrdinaryNewResultClaimV1| claim.core.site == site)
        {
            return Err(OrdinaryNewCoSealIssueV1::DuplicateSite { site });
        }
        // An unwalked return site stays truthfully unavailable — never
        // fabricate a covered prefix.
        let home_prefix = result_prefixes
            .remove(&site)
            .unwrap_or(Err(HomePrefixUnavailableV1::SourceMismatch));
        if let Some(handoff) = birth_handoff {
            if birth_abi_handoffs.insert(site.clone(), handoff).is_some() {
                return Err(OrdinaryNewCoSealIssueV1::DuplicateSite { site });
            }
        }
        let children = {
            let mut visiting = std::collections::BTreeSet::new();
            owned_field_children_of(
                &site,
                batch,
                instance_constructors,
                &box_source,
                destruction,
                &field_residences,
                owned_field_children,
                &mut visiting,
            )?
        };
        if (destruction).requires_field_end() {
            match owned_field_children.entry(object) {
                std::collections::btree_map::Entry::Vacant(entry) => {
                    entry.insert(children.clone());
                }
                std::collections::btree_map::Entry::Occupied(entry) if *entry.get() == children => {
                }
                std::collections::btree_map::Entry::Occupied(_) => {
                    return Err(OrdinaryNewCoSealIssueV1::DuplicateSite { site });
                }
            }
        }
        result_claims.push(OrdinaryNewResultClaimV1 {
            core: OrdinaryNewClaimCoreV1 {
                site: site.clone(),
                box_source,
                class,
                arity,
                constructor,
                construction,
                construction_source,
                allocation_execution: None,
                object,
                destruction,
                argument_rows,
                children,
            },
            home_prefix,
        });
    }
    Ok(())
}

/// Constructor rows borrow their original program source rather than a batch
/// declaration. Keep this traversal with the existing source index owner.
pub(super) fn collect_constructor_birth_sites_v1(
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    instance_constructors: &VerifiedInstanceConstructorSemanticBatchV1,
    claimed_sites: &BTreeSet<OwnedExprSiteV1>,
    index: &mut BTreeMap<OwnedExprSiteV1, (VerifiedOrdinaryNewBirthRecipeV1, BirthAbiHandoffV1)>,
) -> Result<(), OrdinaryNewCoSealIssueV1> {
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
                    claimed_sites,
                    index,
                )?;
            }
            Ok(())
        })
        .map_err(|_| OrdinaryNewCoSealIssueV1::BatchLoan)?
}

/// Admit one `new` construction site into the destination-less birth index
/// when — and only when — a verified `Birth` recipe exists for it.  Missing
/// coverage, a missing `birth` row, lookup errors, and verification failures
/// all produce no entry: the index is additive-only and never replaces the
/// existing downstream terminal.
pub(super) fn collect_birth_site_index_v1(
    function: &VerifiedResolvedFunctionV1,
    owner: FunctionOwnerIdV1,
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    instance_constructors: &VerifiedInstanceConstructorSemanticBatchV1,
    claimed_sites: &BTreeSet<OwnedExprSiteV1>,
    index: &mut BTreeMap<OwnedExprSiteV1, (VerifiedOrdinaryNewBirthRecipeV1, BirthAbiHandoffV1)>,
) -> Result<(), OrdinaryNewCoSealIssueV1> {
    for construction in function.expression_source().constructions() {
        if is_direct_local_initializer(construction.site().node().segments()) {
            continue;
        }
        let site = OwnedExprSiteV1::new(owner, construction.site().clone());
        if claimed_sites.contains(&site) {
            continue;
        }
        let Ok(coverage_row) = batch.ordinary_box_coverage().row_for(construction.class()) else {
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
        let Ok(pair) = verified_birth_recipe_for_site_v1(
            &site,
            construction.class(),
            construction.arguments().len(),
            birth_row,
        ) else {
            continue;
        };
        if index.insert(site.clone(), pair).is_some() {
            return Err(OrdinaryNewCoSealIssueV1::DuplicateSite { site });
        }
    }
    Ok(())
}

#[path = "ordinary_new_coseal_issue_source_field_batch.rs"]
mod field_batch;
pub(super) use field_batch::prove_local_field_read_batch;
pub(in crate::mir::normal_callable_semantic_package) use field_batch::source_formal_class_v1;
pub(in crate::mir::normal_callable_semantic_package) use field_batch::stage_local_field_read_batch;

#[path = "ordinary_new_coseal_issue_source_home_probe.rs"]
mod home_probe;
pub(super) use home_probe::probe_source_home_prefixes_v1;

#[path = "ordinary_new_coseal_issue_source_owned_children.rs"]
mod owned_children;
use owned_children::{owned_field_children_of, seal_provider_owned_children_v1};
pub(super) use owned_children::{
    stored_child_receiver_v1, stored_child_source_v1, stored_constructor_child_source_v1,
};
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal) use owned_children::stored_child_nonnull_guard_v1;

#[cfg(test)]
#[path = "ordinary_new_field_batch_tests.rs"]
mod field_batch_tests;

/// `Handle`-leaf consult shared by the verified walk and the source
/// probe: true only when the sealed borrowed-formal draft admits a
/// operand use at this exact leaf, or an IntegerReturn with its exact
/// exit/binding request. These are distinct typed source consults:
/// the lent view is read-only here, the draft stays the sole
/// admission authority, and the physical `borrowed_call_uses` whitelist
/// still proves each routed operand.
pub(super) fn dominated_view_use_consult_v1<'a>(
    borrowed_formal_source: &'a Result<
        super::super::lexical_instance_call::PreparedBorrowedFormalIngressV1,
        String,
    >,
    input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'a>,
    results: &'a BorrowedResultCohortV1,
    census: &'a BTreeMap<Box<str>, BTreeSet<hakorune_mir_defs::CanonicalFieldRefV1>>,
    constructors: &'a VerifiedInstanceConstructorSemanticBatchV1,
    coverage: &'a crate::parser::ParserOrdinaryBoxSourceCoverageV1,
) -> impl FnMut(
    &OwnedExprSiteV1,
    crate::mir::resolved_semantics::home_new_prefix::BorrowedViewUseRequestV1<'_>,
) -> Result<bool, OrdinaryNewCoSealIssueV1>
       + 'a {
    move |site: &OwnedExprSiteV1, request| {
        if let crate::mir::resolved_semantics::home_new_prefix::BorrowedViewUseRequestV1::FormalFieldWriteReceiver { target, root, .. } = request {
            let ingress = borrowed_formal_source.as_ref().map_err(|issue|
                OrdinaryNewCoSealIssueV1::BorrowedFormalIngress { site: site.clone(), issue: issue.clone() })?;
            if !ingress.consult_view_use_v1(input, site, request).map_err(|issue|
                OrdinaryNewCoSealIssueV1::BorrowedFormalIngress { site: site.clone(), issue })? {
                return Ok(false);
            }
            let class = ingress.source_object_view_v1(root).map_err(|issue|
                OrdinaryNewCoSealIssueV1::BorrowedFormalIngress { site: site.clone(), issue })?;
            let Some(class) = class else { return Ok(false); };
            let Some(box_source) = coverage.rows().iter().find(|row| row.name() == class.class()) else {
                return Ok(false);
            };
            let Some(crate::mir::resolved_semantics::BodyExpressionShapeV1::FieldAccess { field, .. }) =
                input.body_shape().and_then(|shape| shape.expression_shape(target)) else {
                    return Ok(false);
                };
            return terminal_home::source_declared_field(constructors, box_source,
                &OwnedExprSiteV1::new(input.owner(), target.clone()), field)
                .map(|field| field.is_some_and(|(_, kind)| kind.as_deref()
                    .is_some_and(crate::mir::numeric_substrate::is_numeric_integer_type_name)));
        }
        if let crate::mir::resolved_semantics::home_new_prefix::BorrowedViewUseRequestV1::IntegerNestedFieldReturn { exit, receiver } = request {
            return match borrowed_formal_source {
                Ok(ingress) => results.checked_nested_field_return_v1(ingress, input, site, exit, receiver),
                Err(_) => Ok(false),
            }.map_err(|issue| OrdinaryNewCoSealIssueV1::BorrowedFormalIngress { site: site.clone(), issue });
        }
        if let crate::mir::resolved_semantics::home_new_prefix::BorrowedViewUseRequestV1::IntegerFieldArrayReturn { exit } = request {
            return match results.get(&input.owner()) {
                Some(Ok(proof)) => proof.checked_field_array_return_v1(input, site, exit, census),
                // No retained read product selects this conditional source consult.
                // Other result families keep their original Return classifier.
                Some(Err(_)) | None => Ok(false),
            }.map_err(|issue| OrdinaryNewCoSealIssueV1::BorrowedFormalIngress { site: site.clone(), issue });
        }
        let source = match borrowed_formal_source {
            Ok(source) => source,
            Err(issue) if matches!(request,
                crate::mir::resolved_semantics::home_new_prefix::BorrowedViewUseRequestV1::IntegerMulReturn { .. }) => {
                return Err(OrdinaryNewCoSealIssueV1::BorrowedFormalIngress {
                    site: site.clone(), issue: issue.clone(),
                });
            }
            Err(_) => return Ok(false),
        };
        source
            .consult_view_use_v1(input, site, request)
            .map_err(|issue| OrdinaryNewCoSealIssueV1::BorrowedFormalIngress {
                site: site.clone(),
                issue,
            })
    }
}

/// Share the source-port dispatch between the probe and the selected walk.
/// The walk alone may consult an array-append carrier after ingress failure.
pub(super) fn source_call_arguments_v1(
    input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
    brands: Option<&crate::analysis::brand_program_declaration_catalog::VerifiedBrandProgramDeclarationCatalogV1>,
    batch: &VerifiedResolvedCallableSemanticBatchV1, selected: &VerifiedSelectedCallableBatchMapV1,
    constructors: &VerifiedInstanceConstructorSemanticBatchV1,
    receiver: Option<(BindingRefV1, &crate::parser::ParserOrdinaryBoxSourceRowV1)>,
    entry: Option<&crate::mir::resolved_semantics::VerifiedInstanceEntryHomeLoanV1>,
    classes: &result_class_claim::OrdinaryNewResultClassClaimsV1,
    fields: &field_write_claim::OrdinaryNewFieldWriteClaimsV1,
    targets: &super::super::lexical_instance_call::PreparedLexicalInstanceCallSourceTargetsV1,
    dynamic_calls: &[crate::mir::source_call_target::VerifiedSourceBoundDynamicMemberCallV1],
    contracts: &[super::super::super::model::OwnedCallableParameterContractDeclarationV1],
    candidates: &[OrdinaryNewCandidate],
    source: &Result<super::super::lexical_instance_call::PreparedBorrowedFormalIngressV1, String>,
    static_sites: &Result<BTreeSet<OwnedExprSiteV1>, String>,
    pending: &mut super::super::lexical_instance_call::PendingBorrowedFormalActualsV1,
    results: &BorrowedResultCohortV1,
    receiver_rows: &BTreeMap<OwnedExprSiteV1, super::super::ReceiverCallClassObservationV1>,
    completed: &crate::mir::normal_callable_semantic_package::result_contract::VerifiedCallableResultContractBuilderV1,
    site: &OwnedExprSiteV1, request: BorrowedCallActualRequestV1<'_>, allow_append: bool,
) -> Result<Option<BorrowedCallArgumentsV1>, OrdinaryNewCoSealIssueV1> {
    if matches!(request, BorrowedCallActualRequestV1::StoredMapGetSource) {
        return super::super::StoredMapGetSourceLoanV1::issue_for_site_v1(
            input, site, batch, selected, constructors, receiver, entry, classes, fields,
        ).map(|loan| loan.map(BorrowedCallArgumentsV1::StoredMapGetSource))
            .map_err(|issue| OrdinaryNewCoSealIssueV1::BorrowedFormalIngress { site: site.clone(), issue });
    }
    if let (BorrowedCallActualRequestV1::NamedArrayHolderInitializer(binding), Ok(source)) = (request, source) {
        return named_array_initializer_source_v1(input, batch, brands, source, site, binding)
            .map(|row| row.map(BorrowedCallArgumentsV1::NamedArrayHolderSource));
    }
    if source.is_err() && !matches!(request, BorrowedCallActualRequestV1::DynamicInvocationSource { .. })
        && !(allow_append && matches!(request, BorrowedCallActualRequestV1::ArgumentExpressionCarrierArrayAppendSource { .. }))
        && !select_failed_array_initializer_v1(input, batch, brands, targets, site, request)? { return Ok(None); }
    borrowed_call_arguments_callback_v1(batch, selected, targets, dynamic_calls, contracts, candidates,
        receiver, source, static_sites, pending, results,
        &mut |binding| super::lexical::nullable_received_result_class(selected, batch, classes, candidates, input, receiver, binding),
        site, request, classes, receiver_rows, fields, Some(completed))
}

/// Query the existing sealed observations; issue no new class or completion.
pub(super) fn has_receiver_home_call_v1(
    owner: FunctionOwnerIdV1,
    observations: &BTreeMap<
        OwnedExprSiteV1,
        receiver_call_observation::ReceiverCallClassObservationV1,
    >,
    source: &Result<super::super::lexical_instance_call::PreparedBorrowedFormalIngressV1, String>,
    contracts: &[super::super::super::model::OwnedCallableParameterContractDeclarationV1],
) -> Result<bool, OrdinaryNewCoSealIssueV1> {
    if has_nullable_receiver_call_v1(owner, observations) {
        return Ok(true);
    }
    let Some((site, _)) = observations.iter().find(|(site, row)| {
        site.owner() == owner
            && matches!(
                row.class(),
                result_class_claim::OrdinaryNewResultClassV1::Object(_)
            )
    }) else {
        return Ok(false);
    };
    let Ok(source) = source else {
        return Ok(false);
    };
    super::super::lexical_instance_call::has_original_forwarding_receiver_input_source_v1(
        source,
        Some(contracts),
        owner,
        &mut |target| {
            observations.get(target.call_site()).is_some_and(|row| {
                row.callee() == target.target()
                    && matches!(
                        row.class(),
                        result_class_claim::OrdinaryNewResultClassV1::Object(_)
                    )
            })
        },
    )
    .map_err(|issue| OrdinaryNewCoSealIssueV1::BorrowedFormalIngress {
        site: site.clone(),
        issue,
    })
}

/// Nullable observations retain their existing homes-aware source walk.
pub(super) fn has_nullable_receiver_call_v1(
    owner: FunctionOwnerIdV1,
    receiver_call_observations: &BTreeMap<
        OwnedExprSiteV1,
        receiver_call_observation::ReceiverCallClassObservationV1,
    >,
) -> bool {
    receiver_call_observations.iter().any(|(site, row)| {
        site.owner() == owner
            && matches!(
                row.class(),
                result_class_claim::OrdinaryNewResultClassV1::NullableObject(_)
            )
    })
}

/// Exact ordinary candidate lookup; preserve the verified walk's existing gate.
pub(super) fn ordinary_candidate_compatible_v1(
    candidates: &[OrdinaryNewCandidate],
    site: &OwnedExprSiteV1,
    binding: BindingRefV1,
) -> Result<bool, OrdinaryNewCoSealIssueV1> {
    let mut exact = candidates.iter().filter(|row| &row.site == site);
    let candidate =
        exact
            .next()
            .ok_or_else(|| OrdinaryNewCoSealIssueV1::InitializerBindingMismatch {
                site: site.clone(),
            })?;
    if exact.next().is_some() || candidate.destination != binding {
        return Err(OrdinaryNewCoSealIssueV1::InitializerBindingMismatch { site: site.clone() });
    }
    Ok(candidate.construction.is_ok()
        && candidate.destruction == ObjectDestructionDispositionV1::NUMERIC_NO_HOOK)
}

#[path = "ordinary_new_coseal_field_source.rs"]
mod field_source;
pub(in crate::mir) use field_source::StoredChildFieldSourceV1;
pub(in crate::mir::normal_callable_semantic_package) use field_source::declared_stored_child_field_v1;
pub(super) use field_source::{requires_field_scalar_walk_v1, field_sources_with_original_writes};
