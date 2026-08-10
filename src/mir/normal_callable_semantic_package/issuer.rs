use crate::mir::builder::{
    issue_source_backed_same_module_callable_catalog_v1, SourceBackedCallableCatalogIssueV1,
};
use crate::mir::callable_parameter_demand::{
    issue_callable_parameter_demands_v1, CallableParameterDemandIssueV1,
};
use crate::mir::callable_semantic_batch::{
    issue_resolved_callable_semantic_batch_v1, ResolvedCallableSemanticBatchIssueV1,
    ResolvedCallableSemanticBatchLoanErrorV1,
};
use crate::mir::compiler::dynamic_full_body_recipe::{
    issue_dynamic_carrier_ingress_lifecycle_program_v1,
    issue_dynamic_full_loop_semantic_program_v2, issue_dynamic_full_loop_source_recipe_envelope_v2,
    issue_dynamic_invocation_carrier_lifecycle_program_v1,
    issue_dynamic_operator_carrier_lifecycle_program_v1,
    DynamicCarrierIngressLifecycleProgramRejectV1, DynamicFullLoopSemanticProgramRejectV2,
    DynamicFullLoopSourceRecipeEnvelopeRejectV2, DynamicInvocationCarrierLifecycleProgramRejectV1,
    DynamicOperatorCarrierLifecycleProgramRejectV1,
};
use crate::mir::resolved_semantics::FunctionSemanticResolverSessionV1;
use crate::parser::VerifiedFinalCallableProgramSourceV1;

use super::dynamic_admission::{
    admit_dynamic_callable_v1, DynamicCallableAdmissionIssueV1, DynamicCallableAdmissionV1,
};
use super::model::{
    NormalCallableDynamicProjectionV1, OwnedCallableParameterDemandDeclarationV1,
    OwnedCallableParameterDemandV1, VerifiedNormalCallableSemanticPackageV1,
};
use super::selected_mapping::{
    issue_selected_callable_batch_map_v1, SelectedCallableBatchMapIssueV1,
};

#[derive(Debug)]
pub(crate) enum NormalCallableSemanticPackageIssueV1 {
    SourceBackedCatalog(SourceBackedCallableCatalogIssueV1),
    Batch(ResolvedCallableSemanticBatchIssueV1),
    SelectedMapping(SelectedCallableBatchMapIssueV1),
    ParameterDemand(CallableParameterDemandIssueV1),
    BatchLoan(ResolvedCallableSemanticBatchLoanErrorV1),
    Dynamic {
        batch_slot: u32,
        issue: DynamicCallableAdmissionIssueV1,
    },
    DuplicateDynamicCandidate,
    MissingDynamicParameterDemand,
    DynamicParameterDemandIdentity,
    DynamicRecipeEnvelope(DynamicFullLoopSourceRecipeEnvelopeRejectV2),
    DynamicSemanticProgram(DynamicFullLoopSemanticProgramRejectV2),
    DynamicInvocationLifecycle(DynamicInvocationCarrierLifecycleProgramRejectV1),
    DynamicOperatorLifecycle(DynamicOperatorCarrierLifecycleProgramRejectV1),
    DynamicIngress(DynamicCarrierIngressLifecycleProgramRejectV1),
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
    let parameter_demands = {
        let catalog = issue_callable_parameter_demands_v1(&batch)
            .map_err(NormalCallableSemanticPackageIssueV1::ParameterDemand)?;
        catalog
            .declarations()
            .map(|declaration| OwnedCallableParameterDemandDeclarationV1 {
                batch_slot: declaration.batch_slot(),
                owner: declaration.owner(),
                parameters: declaration
                    .parameters()
                    .iter()
                    .map(|parameter| OwnedCallableParameterDemandV1 {
                        ordinal: parameter.ordinal(),
                        binding: parameter.binding(),
                        demand: parameter.demand(),
                    })
                    .collect::<Vec<_>>()
                    .into_boxed_slice(),
            })
            .collect::<Vec<_>>()
            .into_boxed_slice()
    };
    let mut candidate = None;
    for declaration in batch.declarations() {
        let batch_slot = declaration.batch_slot();
        let admission = batch
            .with_lowering_input(batch_slot, admit_dynamic_callable_v1)
            .map_err(NormalCallableSemanticPackageIssueV1::BatchLoan)?
            .map_err(|issue| NormalCallableSemanticPackageIssueV1::Dynamic { batch_slot, issue })?;
        if let DynamicCallableAdmissionV1::Candidate {
            owner,
            recipe,
            calls,
        } = admission
        {
            if !selected.contains_batch_slot(batch_slot) {
                continue;
            }
            if candidate
                .replace((batch_slot, owner, recipe, calls))
                .is_some()
            {
                return Err(NormalCallableSemanticPackageIssueV1::DuplicateDynamicCandidate);
            }
        }
    }
    let dynamic = match candidate {
        None => NormalCallableDynamicProjectionV1::ValidUnselected,
        Some((dynamic_batch_slot, dynamic_owner, dynamic_recipe, dynamic_calls)) => {
            let mut dynamic_demands = parameter_demands
                .iter()
                .filter(|declaration| declaration.batch_slot == dynamic_batch_slot);
            let Some(dynamic_demand) = dynamic_demands.next() else {
                return Err(NormalCallableSemanticPackageIssueV1::MissingDynamicParameterDemand);
            };
            if dynamic_demands.next().is_some() || dynamic_demand.owner != dynamic_owner {
                return Err(NormalCallableSemanticPackageIssueV1::DynamicParameterDemandIdentity);
            }
            let dynamic_envelope =
                issue_dynamic_full_loop_source_recipe_envelope_v2(dynamic_recipe, &dynamic_calls)
                    .map_err(NormalCallableSemanticPackageIssueV1::DynamicRecipeEnvelope)?;
            let dynamic_semantic = issue_dynamic_full_loop_semantic_program_v2(dynamic_envelope)
                .map_err(NormalCallableSemanticPackageIssueV1::DynamicSemanticProgram)?;
            let dynamic_invocation =
                issue_dynamic_invocation_carrier_lifecycle_program_v1(dynamic_semantic)
                    .map_err(NormalCallableSemanticPackageIssueV1::DynamicInvocationLifecycle)?;
            let program =
                issue_dynamic_operator_carrier_lifecycle_program_v1(dynamic_invocation)
                    .map_err(NormalCallableSemanticPackageIssueV1::DynamicOperatorLifecycle)?;
            let mut parameter_one_rows = dynamic_demand
                .parameters
                .iter()
                .filter(|parameter| parameter.ordinal == 1);
            let Some(parameter_one) = parameter_one_rows.next() else {
                return Err(NormalCallableSemanticPackageIssueV1::MissingDynamicParameterDemand);
            };
            if parameter_one_rows.next().is_some() {
                return Err(NormalCallableSemanticPackageIssueV1::DynamicParameterDemandIdentity);
            }
            let program = issue_dynamic_carrier_ingress_lifecycle_program_v1(
                program,
                parameter_one.ordinal,
                parameter_one.binding,
                parameter_one.demand,
            )
            .map_err(NormalCallableSemanticPackageIssueV1::DynamicIngress)?;
            NormalCallableDynamicProjectionV1::Selected {
                batch_slot: dynamic_batch_slot,
                owner: dynamic_owner,
                program,
            }
        }
    };

    Ok(VerifiedNormalCallableSemanticPackageV1 {
        catalog,
        batch,
        selected,
        parameter_demands,
        dynamic,
    })
}
