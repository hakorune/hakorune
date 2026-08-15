use crate::mir::builder::{
    issue_source_backed_dynamic_callable_v1, VerifiedSourceBackedDynamicCallableV1,
};
use crate::mir::compiler::dynamic_full_body_recipe::{
    DynamicFullLoopParameterContractV2, DynamicFullLoopRecipeProducerRejectV2,
};
use crate::mir::compiler::dynamic_full_body_source::{
    DynamicFullBodySourceIssueV1, DynamicFullBodySourceIssuerV1,
    VerifiedDynamicLoopFullBodySourceInventoryV1,
};
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::resolved_control_flow::{
    verify_function_completion_v1, FunctionCompletionVerificationErrorV1,
};
use crate::mir::resolved_semantics::{FunctionOwnerIdV1, ResolvedLoopRegionLookupErrorV1};
use crate::mir::source_call_target::{
    issue_source_bound_dynamic_member_calls_v1, DynamicMemberSourceIssueV1,
    VerifiedSourceBoundDynamicMemberCallV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum DynamicCallableDeclineV1 {
    NoExactSingleLoop,
    RootShape,
    LoopShape,
    BodyShape,
    ExpressionShape,
}

#[derive(Debug)]
pub(super) enum DynamicCallableAdmissionV1 {
    Candidate {
        owner: FunctionOwnerIdV1,
        source: VerifiedSourceBackedDynamicCallableV1,
        inventory: VerifiedDynamicLoopFullBodySourceInventoryV1,
        calls: Box<[VerifiedSourceBoundDynamicMemberCallV1]>,
    },
    Declined(DynamicCallableDeclineV1),
}

#[derive(Debug)]
pub(super) enum DynamicCallableAdmissionIssueV1 {
    Unresolved(DynamicFullBodySourceIssueV1),
    Rejected(DynamicFullBodySourceIssueV1),
    Completion(FunctionCompletionVerificationErrorV1),
    Recipe(DynamicFullLoopRecipeProducerRejectV2),
    Calls(DynamicMemberSourceIssueV1),
    SourceBacked(String),
}

pub(super) fn admit_dynamic_callable_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
) -> Result<DynamicCallableAdmissionV1, DynamicCallableAdmissionIssueV1> {
    let ledger = input
        .forest()
        .callable_source_ledger(input.owner())
        .map_err(|_| {
            DynamicCallableAdmissionIssueV1::Unresolved(
                DynamicFullBodySourceIssueV1::SourceNavigation,
            )
        })?;
    let membership = match ledger.only_loop_site() {
        Ok(membership) => membership,
        Err(ResolvedLoopRegionLookupErrorV1::NoUniqueLoopSite { .. }) => {
            return Ok(DynamicCallableAdmissionV1::Declined(
                DynamicCallableDeclineV1::NoExactSingleLoop,
            ))
        }
        Err(
            ResolvedLoopRegionLookupErrorV1::MissingExactBundle(_)
            | ResolvedLoopRegionLookupErrorV1::PairContractMismatch,
        ) => {
            return Err(DynamicCallableAdmissionIssueV1::Unresolved(
                DynamicFullBodySourceIssueV1::SourceNavigation,
            ))
        }
    };
    let completion = verify_function_completion_v1(input)
        .map_err(DynamicCallableAdmissionIssueV1::Completion)?;
    let source = match DynamicFullBodySourceIssuerV1::issue(input, membership, completion) {
        Ok(source) => source,
        Err(error) => {
            return match classify_source_issue(error) {
                ClassifiedDynamicSourceIssueV1::Declined(reason) => {
                    Ok(DynamicCallableAdmissionV1::Declined(reason))
                }
                ClassifiedDynamicSourceIssueV1::Unresolved(error) => {
                    Err(DynamicCallableAdmissionIssueV1::Unresolved(error))
                }
                ClassifiedDynamicSourceIssueV1::Rejected(error) => {
                    Err(DynamicCallableAdmissionIssueV1::Rejected(error))
                }
            }
        }
    };
    let owner = source.owner();
    // The package is the sole production owner of this source-backed Dynamic
    // classification. Lowering later borrows the retained product instead of
    // reissuing it from the resolved AST a second time.
    let source_backed = issue_source_backed_dynamic_callable_v1(input)
        .map_err(DynamicCallableAdmissionIssueV1::SourceBacked)?;
    let calls = issue_source_bound_dynamic_member_calls_v1(input, &source_backed)
        .map_err(DynamicCallableAdmissionIssueV1::Calls)?;
    Ok(DynamicCallableAdmissionV1::Candidate {
        owner,
        source: source_backed,
        inventory: source,
        calls,
    })
}

pub(super) fn issue_dynamic_parameter_contract_v2(
    rows: &[super::model::OwnedCallableParameterContractV1],
) -> Result<DynamicFullLoopParameterContractV2, DynamicCallableAdmissionIssueV1> {
    let rows = rows
        .iter()
        .map(|row| {
            let class = match row.kind {
                crate::mir::callable_parameter_contract::CallableParameterContractKindV1::OpaqueHandle => {
                    crate::mir::compiler::dynamic_full_body_recipe::DynamicFullLoopParameterClassV2::Dynamic
                }
                crate::mir::callable_parameter_contract::CallableParameterContractKindV1::ExactTrivial(abi) => {
                    if abi != crate::mir::exact_trivial_parameter_abi::ExactTrivialParameterAbiV1::I64 {
                        return Err(DynamicCallableAdmissionIssueV1::Recipe(
                            DynamicFullLoopRecipeProducerRejectV2::ParameterContractMismatch,
                        ));
                    }
                    crate::mir::compiler::dynamic_full_body_recipe::DynamicFullLoopParameterClassV2::I64
                }
                crate::mir::callable_parameter_contract::CallableParameterContractKindV1::ExactText(_) => {
                    return Err(DynamicCallableAdmissionIssueV1::Recipe(
                        DynamicFullLoopRecipeProducerRejectV2::ParameterContractMismatch,
                    ));
                }
            };
            Ok(crate::mir::compiler::dynamic_full_body_recipe::DynamicFullLoopParameterContractRowV2 {
                ordinal: row.ordinal,
                binding: row.binding,
                class,
            })
        })
        .collect::<Result<Vec<_>, _>>()?
        .into_boxed_slice();
    Ok(DynamicFullLoopParameterContractV2::new(rows))
}

enum ClassifiedDynamicSourceIssueV1 {
    Declined(DynamicCallableDeclineV1),
    Unresolved(DynamicFullBodySourceIssueV1),
    Rejected(DynamicFullBodySourceIssueV1),
}

fn classify_source_issue(error: DynamicFullBodySourceIssueV1) -> ClassifiedDynamicSourceIssueV1 {
    match error {
        DynamicFullBodySourceIssueV1::RootShape => {
            ClassifiedDynamicSourceIssueV1::Declined(DynamicCallableDeclineV1::RootShape)
        }
        DynamicFullBodySourceIssueV1::LoopShape => {
            ClassifiedDynamicSourceIssueV1::Declined(DynamicCallableDeclineV1::LoopShape)
        }
        DynamicFullBodySourceIssueV1::BodyShape => {
            ClassifiedDynamicSourceIssueV1::Declined(DynamicCallableDeclineV1::BodyShape)
        }
        DynamicFullBodySourceIssueV1::ExpressionShape => {
            ClassifiedDynamicSourceIssueV1::Declined(DynamicCallableDeclineV1::ExpressionShape)
        }
        DynamicFullBodySourceIssueV1::SourceNavigation
        | DynamicFullBodySourceIssueV1::MissingResolverEvidence => {
            ClassifiedDynamicSourceIssueV1::Unresolved(error)
        }
        DynamicFullBodySourceIssueV1::ForeignOwner
        | DynamicFullBodySourceIssueV1::BindingMismatch
        | DynamicFullBodySourceIssueV1::IterationLocalScopeMismatch
        | DynamicFullBodySourceIssueV1::IterationLocalUseClosureMismatch
        | DynamicFullBodySourceIssueV1::DuplicateSourceRole
        | DynamicFullBodySourceIssueV1::DuplicateSourceSite
        | DynamicFullBodySourceIssueV1::CompletionMismatch
        | DynamicFullBodySourceIssueV1::CoverageMismatch => {
            ClassifiedDynamicSourceIssueV1::Rejected(error)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::model::OwnedCallableParameterContractV1;
    use super::{issue_dynamic_parameter_contract_v2, DynamicCallableAdmissionIssueV1};
    use crate::mir::callable_parameter_contract::CallableParameterContractKindV1;
    use crate::mir::exact_text_parameter_abi::ExactTextFormalAbiV1;
    use crate::mir::resolved_semantics::FunctionOwnerIssuerV1;
    use hakorune_mir_core::BindingId;

    #[test]
    fn exact_text_is_rejected_before_dynamic_recipe_reclassification() {
        let mut issuer = FunctionOwnerIssuerV1::new_for_compilation().unwrap();
        let owner = issuer.issue().unwrap();
        let rows = [OwnedCallableParameterContractV1 {
            ordinal: 0,
            binding: crate::mir::resolved_semantics::BindingRefV1::new(owner, BindingId::new(0)),
            kind: CallableParameterContractKindV1::ExactText(ExactTextFormalAbiV1::STRING_BOX),
        }];

        assert!(matches!(
            issue_dynamic_parameter_contract_v2(&rows),
            Err(DynamicCallableAdmissionIssueV1::Recipe(
                crate::mir::compiler::dynamic_full_body_recipe::DynamicFullLoopRecipeProducerRejectV2::ParameterContractMismatch
            ))
        ));
    }
}
