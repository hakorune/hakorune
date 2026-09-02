use std::collections::BTreeSet;

use crate::analysis::brand_program_declaration_catalog::VerifiedBrandProgramDeclarationCatalogV1;
use crate::mir::builder::{
    issue_source_backed_same_module_callable_catalog_v1,
    CatalogedBoxMethodPhysicalHeaderProjectionV1, ConsumedNormalRootCallableSourceV1,
    SameModuleCallableNamespaceV1, SelectedNormalCallableKeyV1, SourceBackedCallableCatalogIssueV1,
    VerifiedSourceBackedSameModuleCallableCatalogV1,
};
#[cfg(test)]
use crate::mir::builder::{NormalRootExecutionConsumerRejectV1, NormalRootExecutionConsumerV1};
use crate::mir::callable_parameter_contract::{
    issue_callable_parameter_contract_v1, CallableParameterContractIssueV1,
};
use crate::mir::callable_semantic_batch::{
    issue_resolved_callable_semantic_batch_with_main_freestatic_targets_v1,
    issue_resolved_callable_semantic_batch_with_policy_v1, DirectCallObservationBatchPolicyV1,
    ResolvedCallableDeclarationModeV1, ResolvedCallableSemanticBatchIssueV1,
    ResolvedCallableSemanticBatchLoanErrorV1, VerifiedResolvedCallableSemanticBatchV1,
};
use crate::mir::compiler::dynamic_full_body_recipe::{
    issue_dynamic_exit_transaction_coseal_i0, issue_dynamic_full_loop_semantic_program_v2,
    issue_dynamic_full_loop_source_recipe_envelope_v2,
    issue_dynamic_invocation_carrier_lifecycle_program_v1,
    issue_dynamic_invocation_cleanup_projection_i0,
    produce_dynamic_full_loop_recipe_v2_with_contract, DynamicExitTransactionCoSealRejectV1,
    DynamicFullLoopRecipeProducerRejectV2, DynamicFullLoopSemanticProgramRejectV2,
    DynamicFullLoopSourceRecipeEnvelopeRejectV2, DynamicInvocationCarrierLifecycleProgramRejectV1,
    DynamicInvocationCleanupProjectionRejectV1,
};
use crate::mir::resolved_semantics::{CallableLookupErrorV1, FunctionSemanticResolverSessionV1};
#[cfg(test)]
use crate::parser::VerifiedFinalCallableProgramSourceV1;
use std::rc::Rc;

#[path = "app_main_relation.rs"]
mod app_main_relation;

use super::completion_seed::issue_callable_completion_seed_cohort_v1;
use super::declared_instance_locator::{
    issue_declared_instance_call_package_locator_v1, DeclaredInstanceCallPackageLocatorIssueV1,
};
use super::direct_call_loan::{
    AppMainDirectCallDispositionLoanV1, AppMainDirectCallDispositionRowV1,
};
use super::dynamic_admission::{
    admit_dynamic_callable_v1, issue_dynamic_parameter_contract_v2,
    DynamicCallableAdmissionIssueV1, DynamicCallableAdmissionV1,
};
use super::instance_constructor_semantic::{
    issue_instance_constructor_semantic_batch_v1, InstanceConstructorSemanticBatchIssueV1,
};
use super::model::{
    NormalCallableDynamicProjectionV1, OwnedCallableParameterContractDeclarationV1,
    OwnedCallableParameterContractV1, VerifiedNormalCallableSemanticPackageV1,
};
use super::ordinary_new_coseal::{issue_ordinary_new_claims_v1, OrdinaryNewCoSealIssueV1};
use super::physical_header::{
    issue_callable_physical_header_from_result_contract_v1, CallablePhysicalHeaderIssueV1,
};
use super::physical_signature::{
    issue_callable_physical_signature_v1, CallablePhysicalSignatureIssueV1,
};
use super::result_contract::{
    issue_callable_result_contract_cohort_v1, CallableResultContractIssueV1,
};
use super::s6c_child::{issue_s6c_semantic_child_v1, S6CSemanticChildIssueV1};
use super::s6c_storage_header::VerifiedS6CStorageHeaderProjectionV1;
use super::selected_mapping::{
    issue_selected_callable_batch_map_v1, SelectedCallableBatchMapIssueV1,
    VerifiedSelectedCallableBatchMapV1,
};

/// Typed failure boundary for the selected App Main direct-call handoff.
/// Every cause remains observable; none is collapsed into `()` or a generic
/// package batch rejection before the affine loan is issued.
#[derive(Debug)]
pub(crate) enum AppMainDirectCallDispositionIssueV1 {
    SourceCoverage,
    NestedOwnerObservation,
    TargetMissing,
    HeaderLookup(CallableLookupErrorV1),
    TargetOwnerMismatch,
    PublishedTargetMissing,
    PublishedTargetDuplicate,
    PublishedTargetNotCataloged,
    PublishedTargetNamespaceMismatch,
    PublishedTargetNameMismatch,
    PublishedTargetArityMismatch,
    CompilationBrandMismatch,
    TargetNameMismatch,
    ArityMismatch,
    ArgumentSiteMismatch,
    BatchLoan(ResolvedCallableSemanticBatchLoanErrorV1),
    Loan(super::direct_call_loan::AppMainDirectCallLoanErrorV1),
}

/// Validate the expected Cataloged owner/site/provenance relation without
/// issuing a target.  Actual raw lineage remains a later Builder boundary.
/// The boolean is a private transient only: callers still fail closed with
/// the existing unissued-observation issue when any row is present.
fn validate_cataloged_source_co_seal_v1(
    catalog: &VerifiedSourceBackedSameModuleCallableCatalogV1,
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    selected: &VerifiedSelectedCallableBatchMapV1,
) -> Result<bool, ResolvedCallableSemanticBatchIssueV1> {
    let declaration_catalog = catalog.catalog();
    if !declaration_catalog
        .brand()
        .is_same(declaration_catalog.selected_source_inventory().brand())
    {
        return Err(ResolvedCallableSemanticBatchIssueV1::UnissuedDirectCallObservation);
    }

    let mut observed = false;
    let mut owned_sites = BTreeSet::new();
    let app_main_identity = declaration_catalog
        .source_backed_app_main()
        .map(|main| main.parser_identity());
    for declaration in batch.declarations() {
        if app_main_identity.is_some_and(|identity| declaration.identity().same_as(identity)) {
            // App Main has no selected catalog row.  Its exact owner/forest
            // relation is validated by the dedicated pre-install gate below.
            continue;
        }
        let slot = declaration.batch_slot();
        let Some(key) = selected.key_for_batch_slot(slot) else {
            return Err(ResolvedCallableSemanticBatchIssueV1::UnissuedDirectCallObservation);
        };
        let Some(identity) = selected.identity_for_batch_slot(slot) else {
            return Err(ResolvedCallableSemanticBatchIssueV1::UnissuedDirectCallObservation);
        };
        let Some((catalog_key, catalog_identity, _role)) = catalog
            .selected_identities()
            .find(|(candidate, _, _)| *candidate == key)
        else {
            return Err(ResolvedCallableSemanticBatchIssueV1::UnissuedDirectCallObservation);
        };
        if !identity.same_as(catalog_identity)
            || !declaration.same_declaration_identity(catalog_identity)
        {
            return Err(ResolvedCallableSemanticBatchIssueV1::UnissuedDirectCallObservation);
        }

        let Some(_source_site) = declaration_catalog
            .selected_source_inventory()
            .site(catalog_key)
        else {
            return Err(ResolvedCallableSemanticBatchIssueV1::UnissuedDirectCallObservation);
        };
        let Some(catalog_key) = (match key {
            SelectedNormalCallableKeyV1::Cataloged(key)
                if key.namespace() == SameModuleCallableNamespaceV1::StaticBoxMethod =>
            {
                Some(key)
            }
            _ => None,
        }) else {
            // A non-Cataloged row has no expected source-backed provenance.
            // It is accepted only when its forest contains no observation.
            let has_observation = batch
                .with_lowering_input(slot, |input| {
                    input
                        .forest()
                        .owners()
                        .any(|(_, function)| function.direct_call_observations().next().is_some())
                })
                .map_err(|_| ResolvedCallableSemanticBatchIssueV1::UnissuedDirectCallObservation)?;
            if has_observation {
                return Err(ResolvedCallableSemanticBatchIssueV1::UnissuedDirectCallObservation);
            }
            continue;
        };
        let Some(declaration_row) = declaration_catalog.declaration(catalog_key) else {
            return Err(ResolvedCallableSemanticBatchIssueV1::UnissuedDirectCallObservation);
        };
        let expected_parameter_count =
            u32::try_from(declaration_row.params().len()).unwrap_or(u32::MAX);
        if catalog_key.arity() != expected_parameter_count
            || declaration.parameter_count() != expected_parameter_count
        {
            return Err(ResolvedCallableSemanticBatchIssueV1::UnissuedDirectCallObservation);
        }

        let has_observation = batch
            .with_lowering_input(slot, |input| {
                let forest = input.forest();
                if forest.semantic_owners().any(|(owner, product)| {
                    product.as_function().is_none() || product.owner() != owner
                }) {
                    return None;
                }
                let compilation = input.owner().compilation_brand();
                let mut has_observation = false;
                for (owner, function) in forest.owners() {
                    if owner.compilation_brand() != compilation
                        || function.owner() != owner
                        || function.source_site_inventory().owner() != owner
                        || function.source_site_inventory().function_origin()
                            != function.function_origin()
                    {
                        return None;
                    }
                    for (site, _observation) in function.direct_call_observations() {
                        has_observation = true;
                        if !function.source_site_inventory().contains_expression(site)
                            || !owned_sites.insert(
                                crate::mir::resolved_semantics::OwnedExprSiteV1::new(
                                    owner,
                                    site.clone(),
                                ),
                            )
                        {
                            return None;
                        }
                    }
                }
                Some(has_observation)
            })
            .map_err(|_| ResolvedCallableSemanticBatchIssueV1::UnissuedDirectCallObservation)?
            .ok_or(ResolvedCallableSemanticBatchIssueV1::UnissuedDirectCallObservation)?;
        observed |= has_observation;
    }
    Ok(observed)
}

/// Move the exact App Main direct-call products into a private package loan.
///
/// The resolver has already co-issued the source observations and the
/// callable index.  This helper only joins those existing products by their
/// owner/site relation; it never resolves a name or emits a new target.
fn issue_app_main_direct_call_loan_v1(
    catalog: &VerifiedSourceBackedSameModuleCallableCatalogV1,
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    selected: &VerifiedSelectedCallableBatchMapV1,
    app_main_identity: &crate::parser::CallableDeclarationIdentityV1,
) -> Result<Option<AppMainDirectCallDispositionLoanV1>, AppMainDirectCallDispositionIssueV1> {
    let Some((main_slot, callable_index)) = batch.main_callable_index() else {
        return Ok(None);
    };
    let mut declarations = batch
        .declarations()
        .filter(|declaration| declaration.identity().same_as(app_main_identity));
    let Some(main) = declarations.next() else {
        return Err(AppMainDirectCallDispositionIssueV1::SourceCoverage);
    };
    if declarations.next().is_some()
        || main.batch_slot() != main_slot
        || main.mode() != ResolvedCallableDeclarationModeV1::StaticBoxMethod
    {
        return Err(AppMainDirectCallDispositionIssueV1::SourceCoverage);
    }
    let owner = main.owner();
    let mut rows = Vec::new();
    batch
        .with_lowering_input(main_slot, |input| {
            let [root] = input.forest().roots() else {
                return Err(AppMainDirectCallDispositionIssueV1::SourceCoverage);
            };
            if *root != owner {
                return Err(AppMainDirectCallDispositionIssueV1::SourceCoverage);
            }
            for (candidate, function) in input.forest().owners() {
                if candidate != owner {
                    if function.direct_call_observations().next().is_some() {
                        return Err(
                            AppMainDirectCallDispositionIssueV1::NestedOwnerObservation,
                        );
                    }
                    continue;
                }
                for (site, observation) in function.direct_call_observations() {
                    let target = function
                        .direct_call_target(site)
                        .ok_or(AppMainDirectCallDispositionIssueV1::TargetMissing)?;
                    let header = callable_index
                        .header_for_callable(target.callable())
                        .map_err(AppMainDirectCallDispositionIssueV1::HeaderLookup)?;
                    if header.callable().owner() != target.callable().owner() {
                        return Err(AppMainDirectCallDispositionIssueV1::TargetOwnerMismatch);
                    }
                    let published_key = {
                        let mut matches = batch
                            .declarations()
                            .filter(|declaration| declaration.owner() == target.callable().owner());
                        let declaration = matches
                            .next()
                            .ok_or(AppMainDirectCallDispositionIssueV1::PublishedTargetMissing)?;
                        if matches.next().is_some() {
                            return Err(
                                AppMainDirectCallDispositionIssueV1::PublishedTargetDuplicate,
                            );
                        }
                        let selected_key = selected
                            .key_for_batch_slot(declaration.batch_slot())
                            .ok_or(
                                AppMainDirectCallDispositionIssueV1::PublishedTargetMissing,
                            )?;
                        let SelectedNormalCallableKeyV1::Cataloged(key) = selected_key else {
                            return Err(
                                AppMainDirectCallDispositionIssueV1::PublishedTargetNotCataloged,
                            );
                        };
                        if key.namespace() != SameModuleCallableNamespaceV1::StaticBoxMethod {
                            return Err(
                                AppMainDirectCallDispositionIssueV1::PublishedTargetNamespaceMismatch,
                            );
                        }
                        if key.name() != header.source_key().name() {
                            return Err(
                                AppMainDirectCallDispositionIssueV1::PublishedTargetNameMismatch,
                            );
                        }
                        if key.arity() != header.source_key().arity() {
                            return Err(
                                AppMainDirectCallDispositionIssueV1::PublishedTargetArityMismatch,
                            );
                        }
                        if catalog.catalog().declaration(key).is_none() {
                            return Err(
                                AppMainDirectCallDispositionIssueV1::PublishedTargetMissing,
                            );
                        }
                        key.clone()
                    };
                    if header.callable().owner().compilation_brand()
                        != owner.compilation_brand()
                    {
                        return Err(
                            AppMainDirectCallDispositionIssueV1::CompilationBrandMismatch,
                        );
                    }
                    if header.source_key().name() != observation.name() {
                        return Err(AppMainDirectCallDispositionIssueV1::TargetNameMismatch);
                    }
                    if header.signature().arity() != observation.arity() as usize {
                        return Err(AppMainDirectCallDispositionIssueV1::ArityMismatch);
                    }
                    if header.signature().arity() != observation.argument_sites().len() {
                        return Err(AppMainDirectCallDispositionIssueV1::ArgumentSiteMismatch);
                    }
                    let emission = crate::mir::canonical_direct_call::VerifiedCanonicalDirectCallEmissionV1::from_header_with_published_key(
                        header,
                        published_key,
                    );
                    rows.push((
                        site.clone(),
                        AppMainDirectCallDispositionRowV1::new(
                            observation.argument_sites().to_vec().into_boxed_slice(),
                            emission,
                        ),
                    ));
                }
            }
            Ok(())
        })
        .map_err(AppMainDirectCallDispositionIssueV1::BatchLoan)??;
    if rows.is_empty() {
        return Ok(None);
    }
    AppMainDirectCallDispositionLoanV1::from_rows(owner, rows)
        .map(Some)
        .map_err(AppMainDirectCallDispositionIssueV1::Loan)
}

#[derive(Debug)]
pub(in crate::mir) enum NormalCallableSemanticPackageIssueV1 {
    #[cfg(test)]
    RootExecution(NormalRootExecutionConsumerRejectV1),
    SourceBackedCatalog {
        _error: SourceBackedCallableCatalogIssueV1,
    },
    Batch {
        _error: ResolvedCallableSemanticBatchIssueV1,
    },
    OrdinaryNew {
        _error: OrdinaryNewCoSealIssueV1,
    },
    InstanceConstructors {
        _error: InstanceConstructorSemanticBatchIssueV1,
    },
    SelectedMapping {
        _error: SelectedCallableBatchMapIssueV1,
    },
    ParameterContract {
        _error: CallableParameterContractIssueV1,
    },
    PhysicalHeader {
        _error: CallablePhysicalHeaderIssueV1,
    },
    ResultContract {
        _error: CallableResultContractIssueV1,
    },
    PhysicalSignature {
        _error: CallablePhysicalSignatureIssueV1,
    },
    DeclaredInstanceLocator {
        _error: DeclaredInstanceCallPackageLocatorIssueV1,
    },
    S6CChild {
        _error: S6CSemanticChildIssueV1,
    },
    BatchLoan {
        _error: ResolvedCallableSemanticBatchLoanErrorV1,
    },
    AppMainDirectCall {
        _error: AppMainDirectCallDispositionIssueV1,
    },
    AppMainRoot {
        _error: app_main_relation::AppMainRootRelationIssueV1,
    },
    Dynamic {
        _batch_slot: u32,
        _issue: DynamicCallableAdmissionIssueV1,
    },
    DynamicRecipe {
        _error: DynamicFullLoopRecipeProducerRejectV2,
    },
    DuplicateDynamicCandidate,
    MissingDynamicParameterContract,
    DynamicParameterContractIdentity,
    DynamicRecipeEnvelope {
        _error: DynamicFullLoopSourceRecipeEnvelopeRejectV2,
    },
    DynamicSemanticProgram {
        _error: DynamicFullLoopSemanticProgramRejectV2,
    },
    DynamicInvocationLifecycle {
        _error: DynamicInvocationCarrierLifecycleProgramRejectV1,
    },
    DynamicCleanup {
        _error: DynamicInvocationCleanupProjectionRejectV1,
    },
    DynamicExitTransaction {
        _error: DynamicExitTransactionCoSealRejectV1,
    },
    MissingDynamicPhysicalHeader,
    MissingS6CStorageHeader,
}

#[cfg(test)]
pub(in crate::mir) fn issue_normal_callable_semantic_package_v1(
    resolver: &mut FunctionSemanticResolverSessionV1,
    source: VerifiedFinalCallableProgramSourceV1,
) -> Result<VerifiedNormalCallableSemanticPackageV1, NormalCallableSemanticPackageIssueV1> {
    let source = NormalRootExecutionConsumerV1::consume_once(source)
        .map_err(|rejected| {
            let error = rejected.into_error_after_discard();
            NormalCallableSemanticPackageIssueV1::RootExecution(error)
        })?
        .into_consumed_source();
    issue_normal_callable_semantic_package_with_brand_catalog_v1(resolver, source, None)
}

pub(in crate::mir) fn issue_normal_callable_semantic_package_with_brand_catalog_v1(
    resolver: &mut FunctionSemanticResolverSessionV1,
    source: ConsumedNormalRootCallableSourceV1,
    brand_catalog: Option<&VerifiedBrandProgramDeclarationCatalogV1>,
) -> Result<VerifiedNormalCallableSemanticPackageV1, NormalCallableSemanticPackageIssueV1> {
    let instance_constructors =
        issue_instance_constructor_semantic_batch_v1(resolver, source.source(), brand_catalog)
            .map_err(
                |error| NormalCallableSemanticPackageIssueV1::InstanceConstructors {
                    _error: error,
                },
            )?;
    let catalog =
        issue_source_backed_same_module_callable_catalog_v1(&source).map_err(|error| {
            NormalCallableSemanticPackageIssueV1::SourceBackedCatalog { _error: error }
        })?;
    let app_main_identity = catalog
        .catalog()
        .source_backed_app_main()
        .map(|main| main.parser_identity().clone());
    let (batch, root_execution) = source
        .consume_into_semantic_package(|source, root_execution| {
            let batch = match app_main_identity.as_ref() {
                Some(identity) => {
                    issue_resolved_callable_semantic_batch_with_main_freestatic_targets_v1(
                        resolver,
                        source,
                        brand_catalog,
                        identity,
                    )
                }
                None => issue_resolved_callable_semantic_batch_with_policy_v1(
                    resolver,
                    source,
                    brand_catalog,
                    DirectCallObservationBatchPolicyV1::ObserveForCatalogedValidation,
                ),
            }?;
            Ok((batch, root_execution))
        })
        .map_err(|error| NormalCallableSemanticPackageIssueV1::Batch { _error: error })?;
    let selected = issue_selected_callable_batch_map_v1(&catalog, &batch)
        .map_err(|error| NormalCallableSemanticPackageIssueV1::SelectedMapping { _error: error })?;
    if validate_cataloged_source_co_seal_v1(&catalog, &batch, &selected)
        .map_err(|error| NormalCallableSemanticPackageIssueV1::Batch { _error: error })?
    {
        return Err(NormalCallableSemanticPackageIssueV1::Batch {
            _error: ResolvedCallableSemanticBatchIssueV1::UnissuedDirectCallObservation,
        });
    }
    app_main_relation::validate_app_main_root_owner_relation_v1(&catalog, &batch)
        .map_err(|error| NormalCallableSemanticPackageIssueV1::AppMainRoot { _error: error })?;
    let app_main_direct_call_loan = match app_main_identity.as_ref() {
        Some(identity) => issue_app_main_direct_call_loan_v1(&catalog, &batch, &selected, identity)
            .map_err(
                |error| NormalCallableSemanticPackageIssueV1::AppMainDirectCall { _error: error },
            )?,
        None => None,
    };
    let parameter_contracts = {
        let catalog = issue_callable_parameter_contract_v1(&batch).map_err(|error| {
            NormalCallableSemanticPackageIssueV1::ParameterContract { _error: error }
        })?;
        catalog
            .declarations()
            .map(|declaration| OwnedCallableParameterContractDeclarationV1 {
                batch_slot: declaration.batch_slot(),
                owner: declaration.owner(),
                mode: declaration.mode(),
                parameters: declaration
                    .parameters()
                    .iter()
                    .map(|parameter| OwnedCallableParameterContractV1 {
                        ordinal: parameter.ordinal(),
                        binding: parameter.binding(),
                        kind: parameter.kind(),
                    })
                    .collect::<Vec<_>>()
                    .into_boxed_slice(),
            })
            .collect::<Vec<_>>()
            .into_boxed_slice()
    };
    let completion_seeds =
        issue_callable_completion_seed_cohort_v1(&batch, &selected, &parameter_contracts).map_err(
            |error| NormalCallableSemanticPackageIssueV1::PhysicalHeader { _error: error },
        )?;
    let mut completion_seeds = completion_seeds;
    let s6c_child = issue_s6c_semantic_child_v1(&batch, &selected, &mut completion_seeds)
        .map_err(|error| NormalCallableSemanticPackageIssueV1::S6CChild { _error: error })?;
    let s6c_storage_header = match s6c_child.as_ref() {
        None => None,
        Some(child) => {
            let Some(crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(key)) =
                selected.key_for_batch_slot(child.batch_slot())
            else {
                return Err(NormalCallableSemanticPackageIssueV1::MissingS6CStorageHeader);
            };
            let declaration = catalog
                .catalog()
                .declaration(key)
                .ok_or(NormalCallableSemanticPackageIssueV1::MissingS6CStorageHeader)?;
            Some(VerifiedS6CStorageHeaderProjectionV1::from_catalog_declaration(declaration))
        }
    };
    let result_contracts = issue_callable_result_contract_cohort_v1(completion_seeds.into_rows())
        .map_err(|error| {
        NormalCallableSemanticPackageIssueV1::ResultContract { _error: error }
    })?;
    let physical_header = issue_callable_physical_header_from_result_contract_v1(&result_contracts);
    let mut candidate = None;
    for declaration in batch.declarations() {
        // The resolved batch row is the sole declaration-mode authority.  The
        // bounded Dynamic lowerer accepts only static Box methods; instance
        // and top-level rows remain ordinary-owned and must not be probed by
        // Dynamic source/parameter admission.
        if declaration.mode() != ResolvedCallableDeclarationModeV1::StaticBoxMethod {
            continue;
        }
        let batch_slot = declaration.batch_slot();
        if !selected.dynamic_eligible_batch_slot(batch_slot) {
            continue;
        }
        let admission = batch
            .with_lowering_input(batch_slot, admit_dynamic_callable_v1)
            .map_err(|error| NormalCallableSemanticPackageIssueV1::BatchLoan { _error: error })?
            .map_err(|issue| NormalCallableSemanticPackageIssueV1::Dynamic {
                _batch_slot: batch_slot,
                _issue: issue,
            })?;
        if let DynamicCallableAdmissionV1::Candidate {
            owner,
            source,
            inventory,
            calls,
        } = admission
        {
            if candidate
                .replace((batch_slot, owner, source, inventory, calls))
                .is_some()
            {
                return Err(NormalCallableSemanticPackageIssueV1::DuplicateDynamicCandidate);
            }
        }
    }
    let (dynamic, dynamic_physical_header) = match candidate {
        None => (NormalCallableDynamicProjectionV1::ValidUnselected, None),
        Some((
            dynamic_batch_slot,
            dynamic_owner,
            dynamic_source,
            dynamic_inventory,
            dynamic_calls,
        )) => {
            let mut dynamic_contracts = parameter_contracts
                .iter()
                .filter(|declaration| declaration.batch_slot == dynamic_batch_slot);
            let Some(dynamic_contract) = dynamic_contracts.next() else {
                return Err(NormalCallableSemanticPackageIssueV1::MissingDynamicParameterContract);
            };
            if dynamic_contracts.next().is_some() || dynamic_contract.owner != dynamic_owner {
                return Err(NormalCallableSemanticPackageIssueV1::DynamicParameterContractIdentity);
            }
            let typed_contract = issue_dynamic_parameter_contract_v2(&dynamic_contract.parameters)
                .map_err(|issue| match issue {
                    DynamicCallableAdmissionIssueV1::Recipe(reject) => {
                        NormalCallableSemanticPackageIssueV1::DynamicRecipe { _error: reject }
                    }
                    other => NormalCallableSemanticPackageIssueV1::Dynamic {
                        _batch_slot: dynamic_batch_slot,
                        _issue: other,
                    },
                })?;
            let dynamic_recipe = produce_dynamic_full_loop_recipe_v2_with_contract(
                dynamic_inventory,
                typed_contract,
            )
            .map_err(
                |error| NormalCallableSemanticPackageIssueV1::DynamicRecipe { _error: error },
            )?;
            let dynamic_envelope =
                issue_dynamic_full_loop_source_recipe_envelope_v2(dynamic_recipe, dynamic_calls)
                    .map_err(|error| {
                        NormalCallableSemanticPackageIssueV1::DynamicRecipeEnvelope {
                            _error: error,
                        }
                    })?;
            let dynamic_semantic =
                issue_dynamic_full_loop_semantic_program_v2(dynamic_envelope).map_err(|error| {
                    NormalCallableSemanticPackageIssueV1::DynamicSemanticProgram { _error: error }
                })?;
            let dynamic_invocation = issue_dynamic_invocation_carrier_lifecycle_program_v1(
                dynamic_semantic,
            )
            .map_err(|error| {
                NormalCallableSemanticPackageIssueV1::DynamicInvocationLifecycle { _error: error }
            })?;
            let program = issue_dynamic_invocation_cleanup_projection_i0(dynamic_invocation)
                .map_err(
                    |error| NormalCallableSemanticPackageIssueV1::DynamicCleanup { _error: error },
                )?;
            let program = issue_dynamic_exit_transaction_coseal_i0(program).map_err(|error| {
                NormalCallableSemanticPackageIssueV1::DynamicExitTransaction { _error: error }
            })?;
            let Some(crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(dynamic_key)) =
                selected.key_for_batch_slot(dynamic_batch_slot)
            else {
                return Err(NormalCallableSemanticPackageIssueV1::MissingDynamicPhysicalHeader);
            };
            let declaration = catalog
                .catalog()
                .declaration(dynamic_key)
                .ok_or(NormalCallableSemanticPackageIssueV1::MissingDynamicPhysicalHeader)?;
            let physical_header =
                CatalogedBoxMethodPhysicalHeaderProjectionV1::from_catalog_declaration(declaration);
            (
                NormalCallableDynamicProjectionV1::Selected {
                    batch_slot: dynamic_batch_slot,
                    _owner: dynamic_owner,
                    source: Rc::new(dynamic_source),
                    program,
                },
                Some(physical_header),
            )
        }
    };
    let dynamic_batch_slot = match &dynamic {
        NormalCallableDynamicProjectionV1::Selected { batch_slot, .. } => Some(*batch_slot),
        NormalCallableDynamicProjectionV1::ValidUnselected => None,
    };
    let ordinary_new_claims = issue_ordinary_new_claims_v1(
        &batch,
        &selected,
        dynamic_batch_slot,
        &instance_constructors,
    )
    .map_err(|error| NormalCallableSemanticPackageIssueV1::OrdinaryNew { _error: error })?;
    let physical_signature = issue_callable_physical_signature_v1(
        catalog.catalog().brand().clone(),
        &batch,
        &selected,
        &parameter_contracts,
    )
    .map_err(|error| NormalCallableSemanticPackageIssueV1::PhysicalSignature { _error: error })?;
    let declared_instance_call_locators = issue_declared_instance_call_package_locator_v1(
        &batch,
        &selected,
        &result_contracts,
        &physical_signature,
    )
    .map_err(
        |error| NormalCallableSemanticPackageIssueV1::DeclaredInstanceLocator { _error: error },
    )?;

    Ok(VerifiedNormalCallableSemanticPackageV1 {
        root_execution: super::model::NormalRootExecutionPackageStateV1::Prepared(root_execution),
        catalog,
        batch,
        app_main_direct_call_loan,
        ordinary_new_claims,
        instance_constructors,
        selected,
        parameter_contracts,
        result_contracts,
        physical_signature,
        declared_instance_call_locators,
        s6c_child,
        s6c_storage_header,
        physical_header,
        dynamic,
        dynamic_physical_header,
    })
}
