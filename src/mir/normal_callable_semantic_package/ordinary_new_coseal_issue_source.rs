//! Source-claim preparation and assembly for the ordinary-New cohort.
//! This private move preserves source loans, claim order and failure boundaries.
use super::super::coseal_helpers::is_direct_local_initializer;
use super::super::BirthAbiHandoffV1;
use super::*;
use crate::mir::resolved_semantics::home_new_prefix::{
    CallerNewHomePrefixV1, ResultNewHomePrefixV1, SelectedNewArgumentObservationV1,
};
use crate::mir::resolved_semantics::BindingKindV1;

#[path = "ordinary_new_source_claim_preparation.rs"]
mod preparation;
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal) use preparation::prepare_source_claims;

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
    instance_constructors: &VerifiedInstanceConstructorSemanticBatchV1,
    app_main_batch_slot: Option<u32>,
    dynamic_slot: Option<u32>,
) -> BTreeMap<u32, Result<Vec<OrdinaryNewCandidate>, OrdinaryNewCoSealIssueV1>> {
    batch
        .declarations()
        .filter_map(|declaration| {
            let slot = declaration.batch_slot();
            if (selected.role_for_batch_slot(slot).is_none() && app_main_batch_slot != Some(slot))
                || dynamic_slot == Some(slot)
            {
                return None;
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
        if matches!(
            destruction,
            ObjectDestructionDispositionV1::OwnedArrayFieldsNoHook
                | ObjectDestructionDispositionV1::OwnedObjectFieldsNoHook
        ) {
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
        if matches!(
            destruction,
            ObjectDestructionDispositionV1::OwnedArrayFieldsNoHook
                | ObjectDestructionDispositionV1::OwnedObjectFieldsNoHook
        ) {
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
pub(super) use field_batch::{prove_local_field_read_batch, stage_local_field_read_batch};

#[path = "ordinary_new_coseal_issue_source_owned_children.rs"]
mod owned_children;
use owned_children::{owned_field_children_of, seal_provider_owned_children_v1};
pub(super) use owned_children::{stored_child_receiver_v1, stored_child_source_v1};

#[cfg(test)]
#[path = "ordinary_new_field_batch_tests.rs"]
mod field_batch_tests;

/// `Handle`-leaf consult shared by the verified walk and the source
/// probe: true only when the sealed borrowed-formal draft admits a
/// dominated-view value use — `ArrayElementValue`, `AddOperand`, or
/// `NewArgument` — at this exact leaf site. This is a coverage consult
/// only: the lent view is read-only here, the draft stays the sole
/// admission authority, and the physical `borrowed_call_uses` whitelist
/// still proves each routed operand.
pub(super) fn dominated_view_use_consult_v1<'a>(
    borrowed_formal_source: &'a Result<
        super::super::lexical_instance_call::PreparedBorrowedFormalIngressV1,
        String,
    >,
) -> impl FnMut(&OwnedExprSiteV1) -> Result<bool, OrdinaryNewCoSealIssueV1> + 'a {
    move |site: &OwnedExprSiteV1| {
        Ok(borrowed_formal_source
            .as_ref()
            .ok()
            .is_some_and(|source| source.dominated_view_use_at(site.owner(), site)))
    }
}

/// `formal.<field>` index consult shared by the verified walk and the
/// source probe: `true` only when the receiver's resolved binding is an
/// exact `Parameter` of this callable and the field name satisfies the
/// package's unique non-weak integer declaration census — the opaque
/// formal's class is never consulted.
pub(super) fn formal_i64_index_consult_v1<'a>(
    input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'a>,
    instance_constructors: &'a VerifiedInstanceConstructorSemanticBatchV1,
    coverage: &'a crate::parser::ParserOrdinaryBoxSourceCoverageV1,
) -> impl FnMut(
    &OwnedExprSiteV1,
    &SourceExprSiteV1,
    BindingRefV1,
    BindingRefV1,
    &str,
) -> Result<bool, OrdinaryNewCoSealIssueV1>
       + 'a {
    move |site, _, _, binding, name| {
        let Ok(ledger) = input.forest().callable_source_ledger(site.owner()) else {
            return Ok(false);
        };
        Ok(array_i64_fields::formal_i64_index_field(
            &ledger,
            instance_constructors,
            coverage,
            binding,
            name,
        ))
    }
}

/// Run the source-only readiness/actual probe with the verified walk's same
/// source predicates. Readiness and Completion selection remain in the issuer.
pub(super) fn probe_source_home_prefixes_v1(
    input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
    new_sites: &BTreeMap<OwnedExprSiteV1, BindingRefV1>,
    entry_home: Option<&crate::mir::resolved_semantics::VerifiedInstanceEntryHomeLoanV1>,
    explicit_sites: &[SourceStmtSiteV1],
    pending_actuals: &mut super::super::lexical_instance_call::PendingBorrowedFormalActualsV1,
    batch_slot: u32,
    selected: &VerifiedSelectedCallableBatchMapV1,
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    parameter_contracts: &[super::super::super::model::OwnedCallableParameterContractDeclarationV1],
    callable_result_classes: &result_class_claim::OrdinaryNewResultClassClaimsV1,
    candidates: &[OrdinaryNewCandidate],
    instance_constructors: &VerifiedInstanceConstructorSemanticBatchV1,
    receiver_proof: Option<(BindingRefV1, &crate::parser::ParserOrdinaryBoxSourceRowV1)>,
    array_i64_field_sets: &BTreeMap<
        Box<str>,
        BTreeSet<hakorune_mir_defs::CanonicalFieldRefV1>,
    >,
    borrowed_formal_source: &Result<
        super::super::lexical_instance_call::PreparedBorrowedFormalIngressV1,
        String,
    >,
    lexical_source_targets: &super::super::lexical_instance_call::PreparedLexicalInstanceCallSourceTargetsV1,
    borrowed_i64_results: &BTreeMap<
        FunctionOwnerIdV1,
        Result<super::super::lexical_instance_call::BorrowedI64ResultSourceV1, String>,
    >,
    local_static_call: &mut impl FnMut(
        &OwnedExprSiteV1,
    ) -> Result<
        Option<crate::mir::resolved_semantics::home_new_prefix::QualifiedStaticCallClaimV1>,
        OrdinaryNewCoSealIssueV1,
    >,
) -> Result<
    BTreeMap<OwnedExprSiteV1, Result<CallerNewHomePrefixV1, HomePrefixUnavailableV1>>,
    OrdinaryNewCoSealIssueV1,
> {
    crate::mir::resolved_semantics::home_new_prefix::issue_new_home_prefixes_probing_fields_v1(
        input,
        &new_sites,
        entry_home,
        explicit_sites,
        parameter_contracts
            .iter()
            .filter(|row| row.batch_slot == batch_slot)
            .flat_map(|row| row.parameters.iter())
            .map(|row| (row.ordinal, row.binding, row.kind.clone())),
        &mut |site| {
            Ok::<_, OrdinaryNewCoSealIssueV1>(lexical_i64_result_call(
                selected,
                batch,
                parameter_contracts,
                &callable_result_classes,
                &candidates,
                input,
                site,
            ))
        },
        &mut |site| {
            Ok::<_, OrdinaryNewCoSealIssueV1>(lexical_nullable_result_call(
                selected,
                batch,
                &callable_result_classes,
                &candidates,
                input,
                site,
            ))
        },
        local_static_call,
        &mut |site: &OwnedExprSiteV1, _: &SourceExprSiteV1, _: BindingRefV1, home, name| {
            let field = terminal_home::initialized_integer_field(
                instance_constructors,
                &candidates,
                home,
                name,
            )?;
            match field {
                Some(_) => Ok(true),
                None => match super::lexical::nullable_received_result_class(
                    selected,
                    batch,
                    callable_result_classes,
                    &candidates,
                    input,
                    home,
                ) {
                    Some(class) => terminal_home::nullable_result_integer_field(
                        instance_constructors,
                        batch.ordinary_box_coverage(),
                        &class,
                        site,
                        name,
                    )
                    .map(|field| field.is_some()),
                    // A guarded borrowed formal names its class through
                    // the co-sealed object view; the declared `i64`
                    // contract is unchanged.
                    None => match borrowed_formal_source
                        .as_ref()
                        .ok()
                        .and_then(|source| source.formal_object_view(home))
                    {
                        Some(view) => terminal_home::nullable_result_integer_field(
                            instance_constructors,
                            batch.ordinary_box_coverage(),
                            view.class(),
                            site,
                            name,
                        )
                        .map(|field| field.is_some()),
                        None => Ok(false),
                    },
                },
            }
        },
        &mut |site: &OwnedExprSiteV1, _: &SourceExprSiteV1, _: BindingRefV1, home, name| {
            terminal_home::argument_integer_field(
                instance_constructors,
                &candidates,
                site,
                receiver_proof,
                home,
                name,
            )
            .map(|field| field.is_some())
        },
        &mut |site: &OwnedExprSiteV1, _: &SourceExprSiteV1, _: BindingRefV1, home, name| {
            terminal_home::receiver_scalar_field(
                instance_constructors,
                receiver_proof,
                site,
                home,
                name,
            )
            .map(|field| field.is_some())
        },
        &mut |site: &OwnedExprSiteV1, _: &SourceExprSiteV1, _: BindingRefV1, home, name| {
            terminal_home::receiver_container_field(
                instance_constructors,
                receiver_proof,
                site,
                home,
                name,
            )
            .map(|field| field.is_some())
        },
        &mut |site: &OwnedExprSiteV1, _: &SourceExprSiteV1, _: BindingRefV1, home, name| {
            // The probe shares the verified lane's sealed census — the
            // declared `ArrayBox` type alone admits nothing.
            let Some((_, box_source)) = receiver_proof else {
                return Ok(false);
            };
            let Some(proven) = array_i64_field_sets.get(box_source.name()) else {
                return Ok(false);
            };
            terminal_home::receiver_array_i64_field(
                instance_constructors,
                receiver_proof,
                site,
                home,
                name,
                proven,
            )
            .map(|field| field.is_some())
        },
        // The probe shares the verified lane's `formal.<field>` index
        // proof — same consult factory, same sealed authority.
        &mut formal_i64_index_consult_v1(
            input,
            instance_constructors,
            batch.ordinary_box_coverage(),
        ),
        // The probe shares the verified lane's local
        // field-read membership — no staging here; the
        // verified walk owns the ledger rows.
        &mut |requests, scalar_only| {
            prove_local_field_read_batch(
                instance_constructors,
                &candidates,
                batch.ordinary_box_coverage(),
                receiver_proof,
                requests,
                scalar_only,
                &mut |home| {
                    super::lexical::nullable_received_result_class(
                        selected,
                        batch,
                        callable_result_classes,
                        &candidates,
                        input,
                        home,
                    )
                },
                &mut |home| {
                    borrowed_formal_source
                        .as_ref()
                        .ok()
                        .and_then(|source| source.formal_object_view(home))
                        .map(|view| view.class().into())
                },
            )
            .map(|rows| rows.map(|rows| rows.into_iter().map(|(_, row)| row.result).collect()))
        },
        &mut |site, actuals| {
            borrowed_call_arguments_callback_v1(
                lexical_source_targets,
                parameter_contracts,
                candidates,
                receiver_proof,
                borrowed_formal_source,
                pending_actuals,
                borrowed_i64_results,
                &mut |binding| {
                    super::lexical::nullable_received_result_class(
                        selected,
                        batch,
                        callable_result_classes,
                        &candidates,
                        input,
                        binding,
                    )
                },
                site,
                actuals,
            )
        },
        &mut dominated_view_use_consult_v1(borrowed_formal_source),
    )
}

/// Same exact declaration identity query, before any source cohort effects.
pub(super) fn app_main_batch_slot_v1(
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    app_main_identity: Option<&crate::parser::CallableDeclarationIdentityV1>,
) -> Result<Option<u32>, OrdinaryNewCoSealIssueV1> {
    app_main_identity
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
        .transpose()
}

/// Query the existing sealed observations; issue no new class or completion.
pub(super) fn has_nullable_receiver_call_v1(
    owner: FunctionOwnerIdV1,
    receiver_call_observations: &BTreeMap<
        OwnedExprSiteV1,
        receiver_call_observation::ReceiverCallClassObservationV1,
    >,
) -> bool {
    receiver_call_observations
        .iter()
        .any(|(site, row)| {
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
    let candidate = exact.next().ok_or_else(|| {
        OrdinaryNewCoSealIssueV1::InitializerBindingMismatch { site: site.clone() }
    })?;
    if exact.next().is_some() || candidate.destination != binding {
        return Err(OrdinaryNewCoSealIssueV1::InitializerBindingMismatch { site: site.clone() });
    }
    Ok(candidate.construction.is_ok()
        && candidate.destruction == ObjectDestructionDispositionV1::PlainI64NoHook)
}
