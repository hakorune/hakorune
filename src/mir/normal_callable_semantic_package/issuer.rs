use crate::mir::builder::{
    issue_source_backed_same_module_callable_catalog_v1,
    CatalogedBoxMethodPhysicalHeaderProjectionV1, SourceBackedCallableCatalogIssueV1,
};
use crate::mir::callable_parameter_contract::{
    issue_callable_parameter_contract_v1, CallableParameterContractIssueV1,
};
use crate::mir::callable_semantic_batch::ResolvedCallableDeclarationModeV1;
use crate::mir::callable_semantic_batch::{
    issue_resolved_callable_semantic_batch_v1, ResolvedCallableSemanticBatchIssueV1,
    ResolvedCallableSemanticBatchLoanErrorV1,
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
use crate::parser::VerifiedFinalCallableProgramSourceV1;
use std::rc::Rc;

use super::dynamic_admission::{
    admit_dynamic_callable_v1, issue_dynamic_parameter_contract_v2,
    DynamicCallableAdmissionIssueV1, DynamicCallableAdmissionV1,
};
use super::model::{
    NormalCallableDynamicProjectionV1, OwnedCallableParameterContractDeclarationV1,
    OwnedCallableParameterContractV1, VerifiedNormalCallableSemanticPackageV1,
};
use super::selected_mapping::{
    issue_selected_callable_batch_map_v1, SelectedCallableBatchMapIssueV1,
};

#[derive(Debug)]
pub(crate) enum NormalCallableSemanticPackageIssueV1 {
    SourceBackedCatalog(SourceBackedCallableCatalogIssueV1),
    Batch(ResolvedCallableSemanticBatchIssueV1),
    SelectedMapping(SelectedCallableBatchMapIssueV1),
    ParameterContract(CallableParameterContractIssueV1),
    BatchLoan(ResolvedCallableSemanticBatchLoanErrorV1),
    Dynamic {
        batch_slot: u32,
        issue: DynamicCallableAdmissionIssueV1,
    },
    DynamicRecipe(DynamicFullLoopRecipeProducerRejectV2),
    DuplicateDynamicCandidate,
    MissingDynamicParameterContract,
    DynamicParameterContractIdentity,
    DynamicRecipeEnvelope(DynamicFullLoopSourceRecipeEnvelopeRejectV2),
    DynamicSemanticProgram(DynamicFullLoopSemanticProgramRejectV2),
    DynamicInvocationLifecycle(DynamicInvocationCarrierLifecycleProgramRejectV1),
    DynamicCleanup(DynamicInvocationCleanupProjectionRejectV1),
    DynamicExitTransaction(DynamicExitTransactionCoSealRejectV1),
    MissingDynamicPhysicalHeader,
}

pub(crate) fn issue_normal_callable_semantic_package_v1(
    resolver: &mut FunctionSemanticResolverSessionV1,
    source: VerifiedFinalCallableProgramSourceV1,
) -> Result<VerifiedNormalCallableSemanticPackageV1, NormalCallableSemanticPackageIssueV1> {
    let catalog = issue_source_backed_same_module_callable_catalog_v1(&source)
        .map_err(NormalCallableSemanticPackageIssueV1::SourceBackedCatalog)?;
    let batch = issue_resolved_callable_semantic_batch_v1(resolver, source)
        .map_err(NormalCallableSemanticPackageIssueV1::Batch)?;
    let selected = issue_selected_callable_batch_map_v1(&catalog, &batch)
        .map_err(NormalCallableSemanticPackageIssueV1::SelectedMapping)?;
    let parameter_contracts = {
        let catalog = issue_callable_parameter_contract_v1(&batch)
            .map_err(NormalCallableSemanticPackageIssueV1::ParameterContract)?;
        catalog
            .declarations()
            .map(|declaration| OwnedCallableParameterContractDeclarationV1 {
                batch_slot: declaration.batch_slot(),
                owner: declaration.owner(),
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
        let admission = batch
            .with_lowering_input(batch_slot, admit_dynamic_callable_v1)
            .map_err(NormalCallableSemanticPackageIssueV1::BatchLoan)?
            .map_err(|issue| NormalCallableSemanticPackageIssueV1::Dynamic { batch_slot, issue })?;
        if let DynamicCallableAdmissionV1::Candidate {
            owner,
            source,
            inventory,
            calls,
        } = admission
        {
            if !selected.contains_batch_slot(batch_slot) {
                continue;
            }
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
                        NormalCallableSemanticPackageIssueV1::DynamicRecipe(reject)
                    }
                    other => NormalCallableSemanticPackageIssueV1::Dynamic {
                        batch_slot: dynamic_batch_slot,
                        issue: other,
                    },
                })?;
            let dynamic_recipe = produce_dynamic_full_loop_recipe_v2_with_contract(
                dynamic_inventory,
                typed_contract,
            )
            .map_err(NormalCallableSemanticPackageIssueV1::DynamicRecipe)?;
            let dynamic_envelope =
                issue_dynamic_full_loop_source_recipe_envelope_v2(dynamic_recipe, dynamic_calls)
                    .map_err(NormalCallableSemanticPackageIssueV1::DynamicRecipeEnvelope)?;
            let dynamic_semantic = issue_dynamic_full_loop_semantic_program_v2(dynamic_envelope)
                .map_err(NormalCallableSemanticPackageIssueV1::DynamicSemanticProgram)?;
            let dynamic_invocation =
                issue_dynamic_invocation_carrier_lifecycle_program_v1(dynamic_semantic)
                    .map_err(NormalCallableSemanticPackageIssueV1::DynamicInvocationLifecycle)?;
            let program = issue_dynamic_invocation_cleanup_projection_i0(dynamic_invocation)
                .map_err(NormalCallableSemanticPackageIssueV1::DynamicCleanup)?;
            let program = issue_dynamic_exit_transaction_coseal_i0(program)
                .map_err(NormalCallableSemanticPackageIssueV1::DynamicExitTransaction)?;
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
                    owner: dynamic_owner,
                    source: Rc::new(dynamic_source),
                    program,
                },
                Some(physical_header),
            )
        }
    };

    Ok(VerifiedNormalCallableSemanticPackageV1 {
        catalog,
        batch,
        selected,
        parameter_contracts,
        dynamic,
        dynamic_physical_header,
    })
}
