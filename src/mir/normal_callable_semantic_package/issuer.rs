use crate::analysis::brand_program_declaration_catalog::VerifiedBrandProgramDeclarationCatalogV1;
use crate::mir::builder::{
    issue_source_backed_same_module_callable_catalog_v1,
    CatalogedBoxMethodPhysicalHeaderProjectionV1, ConsumedNormalRootCallableSourceV1,
    SourceBackedCallableCatalogIssueV1,
};
#[cfg(test)]
use crate::mir::builder::{NormalRootExecutionConsumerRejectV1, NormalRootExecutionConsumerV1};
use crate::mir::callable_parameter_contract::{
    issue_callable_parameter_contract_v1, CallableParameterContractIssueV1,
};
use crate::mir::callable_semantic_batch::ResolvedCallableDeclarationModeV1;
use crate::mir::callable_semantic_batch::{
    issue_resolved_callable_semantic_batch_with_brand_catalog_v1,
    ResolvedCallableSemanticBatchIssueV1, ResolvedCallableSemanticBatchLoanErrorV1,
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
use crate::mir::resolved_semantics::FunctionSemanticResolverSessionV1;
#[cfg(test)]
use crate::parser::VerifiedFinalCallableProgramSourceV1;
use std::rc::Rc;

use super::completion_seed::issue_callable_completion_seed_cohort_v1;
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
    issue_callable_physical_header_from_seeds_v1, CallablePhysicalHeaderIssueV1,
};
use super::physical_signature::{
    issue_callable_physical_signature_v1, CallablePhysicalSignatureIssueV1,
};
use super::s6c_child::{issue_s6c_semantic_child_v1, S6CSemanticChildIssueV1};
use super::s6c_storage_header::VerifiedS6CStorageHeaderProjectionV1;
use super::selected_mapping::{
    issue_selected_callable_batch_map_v1, SelectedCallableBatchMapIssueV1,
};

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
    PhysicalSignature {
        _error: CallablePhysicalSignatureIssueV1,
    },
    S6CChild {
        _error: S6CSemanticChildIssueV1,
    },
    BatchLoan {
        _error: ResolvedCallableSemanticBatchLoanErrorV1,
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
    let (batch, root_execution) = source
        .consume_into_semantic_package(|source, root_execution| {
            issue_resolved_callable_semantic_batch_with_brand_catalog_v1(
                resolver,
                source,
                brand_catalog,
            )
            .map(|batch| (batch, root_execution))
        })
        .map_err(|error| NormalCallableSemanticPackageIssueV1::Batch { _error: error })?;
    let selected = issue_selected_callable_batch_map_v1(&catalog, &batch)
        .map_err(|error| NormalCallableSemanticPackageIssueV1::SelectedMapping { _error: error })?;
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
    let physical_header =
        issue_callable_physical_header_from_seeds_v1(completion_seeds.into_rows());
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
    let ordinary_new_claims =
        issue_ordinary_new_claims_v1(&batch, &selected, dynamic_batch_slot)
            .map_err(|error| NormalCallableSemanticPackageIssueV1::OrdinaryNew { _error: error })?;
    let physical_signature = issue_callable_physical_signature_v1(
        catalog.catalog().brand().clone(),
        &batch,
        &selected,
        &parameter_contracts,
    )
    .map_err(|error| NormalCallableSemanticPackageIssueV1::PhysicalSignature { _error: error })?;

    Ok(VerifiedNormalCallableSemanticPackageV1 {
        root_execution: super::model::NormalRootExecutionPackageStateV1::Prepared(root_execution),
        catalog,
        batch,
        ordinary_new_claims,
        instance_constructors,
        selected,
        parameter_contracts,
        physical_signature,
        s6c_child,
        s6c_storage_header,
        physical_header,
        dynamic,
        dynamic_physical_header,
    })
}
