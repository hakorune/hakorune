use crate::mir::compiler::dynamic_full_body_recipe::{
    produce_dynamic_full_loop_recipe_v2, DynamicFullLoopRecipeCandidateV2,
    DynamicFullLoopRecipeProducerRejectV2,
};
use crate::mir::compiler::dynamic_full_body_source::{
    DynamicFullBodySourceIssueV1, DynamicFullBodySourceIssuerV1,
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
        recipe: DynamicFullLoopRecipeCandidateV2,
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
        Err(ResolvedLoopRegionLookupErrorV1::MissingExactBundle(_)) => {
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
    let recipe = produce_dynamic_full_loop_recipe_v2(source)
        .map_err(DynamicCallableAdmissionIssueV1::Recipe)?;
    let calls = issue_source_bound_dynamic_member_calls_v1(input)
        .map_err(DynamicCallableAdmissionIssueV1::Calls)?;
    Ok(DynamicCallableAdmissionV1::Candidate {
        owner,
        recipe,
        calls,
    })
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
