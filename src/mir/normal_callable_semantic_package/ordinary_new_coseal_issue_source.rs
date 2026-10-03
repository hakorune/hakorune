//! Source-claim preparation and assembly for the ordinary-New cohort.
//! This private move preserves source loans, claim order and failure boundaries.
use super::super::coseal_helpers::is_direct_local_initializer;
use super::super::BirthAbiHandoffV1;
use super::*;
use crate::mir::resolved_semantics::home_new_prefix::{
    CallerNewHomePrefixV1, ResultNewHomePrefixV1, SelectedNewArgumentObservationV1,
};
use crate::mir::resolved_semantics::BindingKindV1;

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

pub(super) fn prepare_source_claims(
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    selected: &VerifiedSelectedCallableBatchMapV1,
    instance_constructors: &VerifiedInstanceConstructorSemanticBatchV1,
) -> Result<
    (
        field_write_claim::OrdinaryNewFieldWriteClaimsV1,
        field_write_claim::OwnedFieldResidencesV1,
        result_class_claim::OrdinaryNewResultClassClaimsV1,
    ),
    OrdinaryNewCoSealIssueV1,
> {
    // Field-write claims and callable result-class claims compose
    // without any Home evidence — seal both before the verified walk so
    // that walk can mint claim-faithful `me.m(..)` observations in the
    // same sweep. Constructor (birth) rows join through the
    // program-source loan, where `lowering_input` needs the program.
    let mut field_write_draft = field_write_claim::OrdinaryNewFieldWriteClaimDraftV1::new();
    let mut result_class_draft = result_class_claim::OrdinaryNewResultClassClaimDraftV1::new();
    for declaration in batch.declarations() {
        let selected_key = selected
            .keys()
            .filter_map(|selected_key| {
                let SelectedNormalCallableKeyV1::Cataloged(key) = selected_key else {
                    return None;
                };
                (selected.batch_slot(selected_key) == Some(declaration.batch_slot()))
                    .then(|| key.clone())
            })
            .next();
        // Field-write claims belong to instance boxes; the result-class
        // claim admits any cataloged key — a static-box sibling returning
        // `return new <class>` names the class for the direct-call
        // handle-result edge too.
        let owner_box = selected_key
            .as_ref()
            .filter(|key| key.namespace() == SameModuleCallableNamespaceV1::InstanceBoxMethod)
            .map(|key| key.owner());
        batch
            .with_lowering_input(declaration.batch_slot(), |input| {
                field_write_draft.observe_function(
                    input.function(),
                    input.body_shape(),
                    owner_box,
                    false,
                );
                if let Some(key) = &selected_key {
                    result_class_draft.observe_function(input, key, declaration.batch_slot());
                }
            })
            .map_err(|_| OrdinaryNewCoSealIssueV1::BatchLoan)?;
    }
    batch
        .with_normal_program_source_loan(|loan| -> Result<(), OrdinaryNewCoSealIssueV1> {
            for row in instance_constructors.rows() {
                let input = row
                    .lowering_input(loan.program())
                    .map_err(|_| OrdinaryNewCoSealIssueV1::BatchLoan)?;
                field_write_draft.observe_function(
                    input.function(),
                    input.body_shape(),
                    Some(row.box_name()),
                    true,
                );
            }
            Ok(())
        })
        .map_err(|_| OrdinaryNewCoSealIssueV1::BatchLoan)??;
    let (field_write_claims, field_residences) =
        field_write_draft.finish(batch.ordinary_box_coverage());
    let callable_result_classes = result_class_draft.finish(
        batch.ordinary_box_coverage(),
        batch,
        selected,
        &field_write_claims,
    );
    Ok((
        field_write_claims,
        field_residences,
        callable_result_classes,
    ))
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
        let children = owned_field_children_of(
            &site,
            batch,
            instance_constructors,
            &box_source,
            destruction,
            &field_residences,
        )?;
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
        let children = owned_field_children_of(
            &site,
            batch,
            instance_constructors,
            &box_source,
            destruction,
            &field_residences,
        )?;
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

/// Prove the owned field children of one canonical object.
/// `OwnedArrayFieldsNoHook`/`OwnedObjectFieldsNoHook` objects carry their
/// residence-capable declared fields in declaration order only when each
/// field has a sealed birth-side residence whose written class equals the
/// declared type. A user-object child must additionally resolve to a
/// `PlainI64NoHook` object — deeper teardown stays unadmitted for S0.
/// Any unproven field — or a missing object definition — yields `None`,
/// which every consumer treats as unadmitted, never silently plain.
fn owned_field_children_of(
    site: &OwnedExprSiteV1,
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    instance_constructors: &crate::mir::normal_callable_semantic_package::instance_constructor_semantic::VerifiedInstanceConstructorSemanticBatchV1,
    box_source: &crate::parser::ParserOrdinaryBoxSourceRowV1,
    destruction: crate::mir::function::ObjectDestructionDispositionV1,
    residences: &field_write_claim::OwnedFieldResidencesV1,
) -> Result<Option<Box<[OwnedFieldChildV1]>>, OrdinaryNewCoSealIssueV1> {
    use crate::mir::function::ObjectDestructionDispositionV1;
    if !matches!(
        destruction,
        ObjectDestructionDispositionV1::OwnedArrayFieldsNoHook
            | ObjectDestructionDispositionV1::OwnedObjectFieldsNoHook
    ) {
        return Ok(None);
    }
    instance_constructors
        .with_source_object_definition(box_source, |object, definition| {
            let mut children = Vec::new();
            for (ordinal, field) in definition.fields().iter().enumerate() {
                let declared = field.declared_type_name.as_deref();
                if declared
                    .and_then(
                        crate::mir::declared_type_storage::exact_numeric_storage_for_declared_type,
                    )
                    .is_some()
                {
                    continue;
                }
                let key = (
                    box_source.name().into(),
                    field.name.clone().into_boxed_str(),
                );
                let Some(class) = residences.get(&key) else {
                    return None;
                };
                // The sole birth write must store the declared class
                // exactly — a proven residence of a different class does
                // not satisfy the typed field.
                if declared != Some(class.as_ref()) {
                    return None;
                }
                let field_ref = hakorune_mir_defs::CanonicalFieldRefV1::from_declaration_ordinal(
                    object, ordinal,
                )
                .expect("declared field ordinal resolves canonically");
                let kind = if class.as_ref() == "ArrayBox" {
                    OwnedFieldChildKindV1::Array
                } else {
                    let child_source = match batch.ordinary_box_coverage().row_for(class.as_ref()) {
                        Ok(Some(row)) => row,
                        _ => return None,
                    };
                    let child = match instance_constructors.destruction_for(child_source) {
                        Ok((child, ObjectDestructionDispositionV1::PlainI64NoHook))
                            if child != object =>
                        {
                            child
                        }
                        _ => return None,
                    };
                    OwnedFieldChildKindV1::Object(child)
                };
                children.push(OwnedFieldChildV1 {
                    field: field_ref,
                    kind,
                });
            }
            Some(children.into_boxed_slice())
        })
        .map_err(|error| OrdinaryNewCoSealIssueV1::ConstructorLookup {
            site: site.clone(),
            class: box_source.name().into(),
            error,
        })
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

#[cfg(test)]
#[path = "ordinary_new_field_batch_tests.rs"]
mod field_batch_tests;

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
            .map(|row| (row.ordinal, row.binding, row.kind)),
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
                    None => Ok(false),
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
    )
}
