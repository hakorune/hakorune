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
        if binding.claims.len() != 4 {
            return Err(Reject::SourceClaimCoverageMismatch {
                expected: 4,
                found: binding.claims.len(),
            });
        }

        let expected = [
            IfSourceClaimRoleV1::IfNode,
            IfSourceClaimRoleV1::Condition,
            IfSourceClaimRoleV1::ThenAssignment,
            IfSourceClaimRoleV1::ElseAssignment,
        ];
        for (claim, role) in binding.claims.iter().zip(expected) {
            if claim.role != role {
                return Err(Reject::SourceClaimOrderMismatch);
            }
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
                IfSourceClaimRoleV1::IfNode => false,
            };
            if *found != root_index || !suffix_ok {
                return Err(Reject::InvalidSourcePath);
            }
        }

        Ok(VerifiedIfRecipeSourceClaimV1(binding))
    }
}
