//! Finite direct-call capability proof for Main.main/0.
//!
//! The source owner remains whole and non-self-referential. Borrowed lowering
//! input is consumed only while the existing capability owners issue durable
//! completion, control, and representation proofs.

use crate::mir::compiler::capability::CanonicalLoweringPreflightV1;
use crate::mir::compiler::lowering_input::CanonicalLoweringErrorV1;
use crate::mir::resolved_control_flow::if_control::VerifiedResolvedFunctionIfControlV1;
use crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1;
use crate::mir::resolved_value_profile::product::VerifiedTrivialCanonicalOwnerV1;

use super::main_direct_call_source::VerifiedNormalMainDirectCallSourceUnitV1;

#[derive(Debug)]
pub(crate) enum NormalMainDirectCallPlanErrorV1 {
    CanonicalPreflight(CanonicalLoweringErrorV1),
}

#[derive(Debug)]
pub(crate) struct VerifiedNormalMainDirectCallPlanV1 {
    source: VerifiedNormalMainDirectCallSourceUnitV1,
    if_control: VerifiedResolvedFunctionIfControlV1,
    completion: VerifiedFunctionCompletionV1,
    profile: VerifiedTrivialCanonicalOwnerV1,
    block_expr_count: usize,
    _seal: VerifiedNormalMainDirectCallPlanSealV1,
}

#[derive(Debug)]
struct VerifiedNormalMainDirectCallPlanSealV1;

impl VerifiedNormalMainDirectCallPlanV1 {
    pub(crate) fn source_identity(&self) -> &str {
        self.source.source_identity()
    }

    pub(crate) fn direct_call_count(&self) -> usize {
        self.profile.direct_calls().len()
    }

    pub(crate) fn direct_calls(
        &self,
    ) -> &[crate::mir::resolved_value_profile::VerifiedTrivialDirectCallV1] {
        self.profile.direct_calls()
    }

    pub(crate) const fn completion(&self) -> &VerifiedFunctionCompletionV1 {
        &self.completion
    }

    pub(crate) const fn if_control(&self) -> &VerifiedResolvedFunctionIfControlV1 {
        &self.if_control
    }

    pub(crate) const fn block_expr_count(&self) -> usize {
        self.block_expr_count
    }

    pub(super) fn into_parts(
        self,
    ) -> (
        VerifiedNormalMainDirectCallSourceUnitV1,
        VerifiedResolvedFunctionIfControlV1,
        VerifiedFunctionCompletionV1,
        VerifiedTrivialCanonicalOwnerV1,
        usize,
    ) {
        (
            self.source,
            self.if_control,
            self.completion,
            self.profile,
            self.block_expr_count,
        )
    }
}

#[derive(Debug)]
pub(crate) struct RejectedNormalMainDirectCallPlanV1 {
    owner: VerifiedNormalMainDirectCallSourceUnitV1,
    error: NormalMainDirectCallPlanErrorV1,
}

impl RejectedNormalMainDirectCallPlanV1 {
    pub(crate) const fn error(&self) -> &NormalMainDirectCallPlanErrorV1 {
        &self.error
    }

    pub(crate) fn discard(self) {
        drop(self);
    }
}

pub(crate) struct NormalMainDirectCallPreflightV1;

impl NormalMainDirectCallPreflightV1 {
    pub(crate) fn seal(
        source: VerifiedNormalMainDirectCallSourceUnitV1,
    ) -> Result<VerifiedNormalMainDirectCallPlanV1, RejectedNormalMainDirectCallPlanV1> {
        let plan = match source.borrow_function_input().and_then(|input| {
            if input.function().direct_call_targets().next().is_some() {
                CanonicalLoweringPreflightV1::verify_normal_main0_function_with_finite_direct_calls_v1(
                    input,
                    source.role(),
                )
            } else {
                CanonicalLoweringPreflightV1::verify_normal_main0_function_v1(
                    input,
                    source.role(),
                )
            }
        }) {
            Ok(plan) => plan,
            Err(error) => {
                return Err(RejectedNormalMainDirectCallPlanV1 {
                    owner: source,
                    error: NormalMainDirectCallPlanErrorV1::CanonicalPreflight(error),
                })
            }
        };
        let (_, if_control, completion, profile, block_expr_count) = plan.into_parts();
        Ok(VerifiedNormalMainDirectCallPlanV1 {
            source,
            if_control,
            completion,
            profile,
            block_expr_count,
            _seal: VerifiedNormalMainDirectCallPlanSealV1,
        })
    }
}

#[cfg(test)]
#[path = "main_direct_call_plan_tests.rs"]
mod tests;
