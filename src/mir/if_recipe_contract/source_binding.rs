//! Structural verification of the If-to-source wire claim.

use super::error::IfRecipeRejectReasonV1 as Reject;
use super::schema::{IfRecipeSourceBindingV1, IfSourceClaimRoleV1, IfSourcePathStepV1};

#[derive(Debug)]
pub(crate) struct VerifiedIfRecipeSourceClaimV1(IfRecipeSourceBindingV1);

impl VerifiedIfRecipeSourceClaimV1 {
    pub(crate) fn as_source_binding(&self) -> &IfRecipeSourceBindingV1 {
        &self.0
    }
}

pub(crate) struct IfRecipeSourceClaimVerifierV1;

impl IfRecipeSourceClaimVerifierV1 {
    pub(crate) fn verify(
        binding: IfRecipeSourceBindingV1,
    ) -> Result<VerifiedIfRecipeSourceClaimV1, Reject> {
        if !matches!(binding.claims.len(), 4 | 5 | 6) {
            return Err(Reject::SourceClaimCoverageMismatch {
                expected: 4,
                found: binding.claims.len(),
            });
        }

        let expected_prefix = [
            IfSourceClaimRoleV1::IfNode,
            IfSourceClaimRoleV1::Condition,
            IfSourceClaimRoleV1::ThenAssignment,
        ];
        for (claim, role) in binding.claims.iter().zip(expected_prefix) {
            if claim.role != role {
                return Err(Reject::SourceClaimOrderMismatch);
            }
        }
        if !matches!(
            binding.claims[3].role,
            IfSourceClaimRoleV1::ElseAssignment | IfSourceClaimRoleV1::ImplicitBaseline
        ) {
            return Err(Reject::SourceClaimOrderMismatch);
        }
        if binding.claims.len() >= 5
            && binding.claims[4].role != IfSourceClaimRoleV1::DirectStaticCall
        {
            return Err(Reject::SourceClaimOrderMismatch);
        }
        if binding.claims.len() == 6
            && (binding.claims[5].role != IfSourceClaimRoleV1::DirectStaticCall
                || !matches!(binding.claims[3].role, IfSourceClaimRoleV1::ElseAssignment))
        {
            return Err(Reject::SourceClaimOrderMismatch);
        }

        let Some(IfSourcePathStepV1::BodyItem { index }) = binding.claims[0].path.steps.first()
        else {
            return Err(Reject::InvalidSourcePath);
        };
        if binding.claims[0].path.steps.len() != 1 {
            return Err(Reject::InvalidSourcePath);
        }
        let root_index = *index;
        for claim in binding.claims.iter().skip(1) {
            let Some(IfSourcePathStepV1::BodyItem { index: found }) = claim.path.steps.first()
            else {
                return Err(Reject::InvalidSourcePath);
            };
            let suffix_ok = match claim.role {
                IfSourceClaimRoleV1::Condition => {
                    claim.path.steps[1..] == [IfSourcePathStepV1::IfCondition]
                }
                IfSourceClaimRoleV1::ThenAssignment => {
                    matches!(
                        claim.path.steps.get(1),
                        Some(IfSourcePathStepV1::IfThenItem { .. })
                    ) && claim.path.steps.len() == 2
                }
                IfSourceClaimRoleV1::ElseAssignment => {
                    matches!(
                        claim.path.steps.get(1),
                        Some(IfSourcePathStepV1::IfElseItem { .. })
                    ) && claim.path.steps.len() == 2
                }
                IfSourceClaimRoleV1::ImplicitBaseline => {
                    claim.path.steps[1..] == [IfSourcePathStepV1::IfImplicitBaseline]
                }
                IfSourceClaimRoleV1::DirectStaticCall => matches!(
                    claim.path.steps.as_slice(),
                    [
                        IfSourcePathStepV1::BodyItem { .. },
                        IfSourcePathStepV1::IfThenItem { .. }
                            | IfSourcePathStepV1::IfElseItem { .. },
                        IfSourcePathStepV1::AssignmentValue,
                    ]
                ),
                IfSourceClaimRoleV1::IfNode => false,
            };
            if *found != root_index || !suffix_ok {
                return Err(Reject::InvalidSourcePath);
            }
        }

        let mut then_calls = 0;
        let mut else_calls = 0;
        for (offset, call_claim) in binding.claims.iter().skip(4).enumerate() {
            if call_claim.role != IfSourceClaimRoleV1::DirectStaticCall {
                return Err(Reject::SourceClaimOrderMismatch);
            }
            let Some((then_branch, call_item)) = direct_call_location(call_claim, root_index)
            else {
                return Err(Reject::InvalidSourcePath);
            };
            if !then_branch && binding.claims[3].role == IfSourceClaimRoleV1::ImplicitBaseline {
                return Err(Reject::InvalidSourcePath);
            }
            let assignment_claim = if then_branch {
                &binding.claims[2]
            } else {
                &binding.claims[3]
            };
            if assignment_item(assignment_claim, root_index, then_branch) != Some(call_item) {
                return Err(Reject::InvalidSourcePath);
            }
            if then_branch {
                then_calls += 1;
            } else {
                else_calls += 1;
            }
            if then_calls > 1 || else_calls > 1 {
                return Err(Reject::SourceClaimOrderMismatch);
            }
            if binding.claims.len() == 6 {
                let expected_then = offset == 0;
                if expected_then != then_branch {
                    return Err(Reject::SourceClaimOrderMismatch);
                }
            }
        }
        if binding.claims.len() == 6 && (then_calls, else_calls) != (1, 1) {
            return Err(Reject::SourceClaimOrderMismatch);
        }

        Ok(VerifiedIfRecipeSourceClaimV1(binding))
    }
}

fn direct_call_location(
    claim: &super::schema::IfSourceClaimV1,
    root_index: u32,
) -> Option<(bool, u32)> {
    match claim.path.steps.as_slice() {
        [IfSourcePathStepV1::BodyItem { index: found }, IfSourcePathStepV1::IfThenItem { index }, IfSourcePathStepV1::AssignmentValue]
            if *found == root_index =>
        {
            Some((true, *index))
        }
        [IfSourcePathStepV1::BodyItem { index: found }, IfSourcePathStepV1::IfElseItem { index }, IfSourcePathStepV1::AssignmentValue]
            if *found == root_index =>
        {
            Some((false, *index))
        }
        _ => None,
    }
}

fn assignment_item(
    claim: &super::schema::IfSourceClaimV1,
    root_index: u32,
    then_branch: bool,
) -> Option<u32> {
    match (then_branch, claim.path.steps.as_slice()) {
        (
            true,
            [IfSourcePathStepV1::BodyItem { index: found }, IfSourcePathStepV1::IfThenItem { index }],
        ) if *found == root_index => Some(*index),
        (
            false,
            [IfSourcePathStepV1::BodyItem { index: found }, IfSourcePathStepV1::IfElseItem { index }],
        ) if *found == root_index => Some(*index),
        _ => None,
    }
}
