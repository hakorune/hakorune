use crate::mir::callable_parameter_demand::{
    issue_callable_parameter_demands_v1, CallableParameterDemandIssueV1,
};
use crate::mir::callable_semantic_batch::{
    issue_resolved_callable_semantic_batch_v1, ResolvedCallableSemanticBatchIssueV1,
    ResolvedCallableSemanticBatchLoanErrorV1,
};
use crate::mir::resolved_semantics::FunctionSemanticResolverSessionV1;
use crate::parser::VerifiedFinalCallableProgramSourceV1;

use super::dynamic_admission::{
    admit_dynamic_callable_v1, DynamicCallableAdmissionIssueV1, DynamicCallableAdmissionV1,
};
use super::model::{
    OwnedCallableParameterDemandDeclarationV1, OwnedCallableParameterDemandV1,
    VerifiedNormalCallableSemanticDynamicPackageV1,
};

#[derive(Debug)]
pub(crate) enum NormalCallableSemanticDynamicPackageIssueV1 {
    Batch(ResolvedCallableSemanticBatchIssueV1),
    ParameterDemand(CallableParameterDemandIssueV1),
    BatchLoan(ResolvedCallableSemanticBatchLoanErrorV1),
    Dynamic {
        source_row_index: u32,
        issue: DynamicCallableAdmissionIssueV1,
    },
    MissingDynamicCandidate,
    DuplicateDynamicCandidate,
    ParameterCoverage,
}

pub(crate) fn issue_normal_callable_semantic_dynamic_package_v1(
    resolver: &mut FunctionSemanticResolverSessionV1,
    source: VerifiedFinalCallableProgramSourceV1,
) -> Result<
    VerifiedNormalCallableSemanticDynamicPackageV1,
    NormalCallableSemanticDynamicPackageIssueV1,
> {
    let batch = issue_resolved_callable_semantic_batch_v1(resolver, source)
        .map_err(NormalCallableSemanticDynamicPackageIssueV1::Batch)?;
    let parameter_demands = {
        let catalog = issue_callable_parameter_demands_v1(&batch)
            .map_err(NormalCallableSemanticDynamicPackageIssueV1::ParameterDemand)?;
        catalog
            .declarations()
            .map(|declaration| OwnedCallableParameterDemandDeclarationV1 {
                source_row_index: declaration.source_row_index(),
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
    if parameter_demands.len() != batch.declarations().len() {
        return Err(NormalCallableSemanticDynamicPackageIssueV1::ParameterCoverage);
    }

    let mut candidate = None;
    for declaration in batch.declarations() {
        let source_row_index = declaration.source_row_index();
        let admission = batch
            .with_lowering_input(source_row_index, admit_dynamic_callable_v1)
            .map_err(NormalCallableSemanticDynamicPackageIssueV1::BatchLoan)?
            .map_err(|issue| NormalCallableSemanticDynamicPackageIssueV1::Dynamic {
                source_row_index,
                issue,
            })?;
        if let DynamicCallableAdmissionV1::Candidate { owner, recipe } = admission {
            if candidate.replace((source_row_index, owner, recipe)).is_some() {
                return Err(
                    NormalCallableSemanticDynamicPackageIssueV1::DuplicateDynamicCandidate,
                );
            }
        }
    }
    let Some((dynamic_source_row_index, dynamic_owner, dynamic_recipe)) = candidate else {
        return Err(NormalCallableSemanticDynamicPackageIssueV1::MissingDynamicCandidate);
    };

    Ok(VerifiedNormalCallableSemanticDynamicPackageV1 {
        batch,
        parameter_demands,
        dynamic_source_row_index,
        dynamic_owner,
        dynamic_recipe,
    })
}
