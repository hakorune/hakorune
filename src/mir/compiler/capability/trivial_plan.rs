use super::{
    CanonicalFirstFamilyPlanBrandV1, ResolvedOwnerHeaderFamilyV1, ResolvedOwnerHeaderSealErrorV1,
    VerifiedResolvedOwnerHeaderV1,
};
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::resolved_control_flow::if_control::VerifiedResolvedFunctionIfControlV1;
use crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1;
use crate::mir::resolved_value_profile::product::VerifiedTrivialCanonicalOwnerV1;

#[derive(Debug)]
pub(crate) struct CanonicalTrivialBindingSsaPlanV1<'a> {
    pub(super) function: ResolvedFunctionLoweringInputV1<'a>,
    pub(super) if_control: VerifiedResolvedFunctionIfControlV1,
    pub(super) completion: VerifiedFunctionCompletionV1,
    pub(super) profile: VerifiedTrivialCanonicalOwnerV1,
    pub(super) block_expr_count: usize,
}

impl<'a> CanonicalTrivialBindingSsaPlanV1<'a> {
    /// Borrow the already sealed canonical function input without exposing
    /// plan parts.  The callback is intentionally the only handoff surface
    /// for a selected-package co-seal; callers cannot re-pair a raw input
    /// after consuming the plan or synthesize a second source authority.
    pub(crate) fn with_function_input<R>(
        &self,
        callback: impl FnOnce(ResolvedFunctionLoweringInputV1<'a>) -> R,
    ) -> R {
        callback(self.function)
    }

    pub(crate) fn seal_resolved_owner_header_v1(
        &self,
    ) -> Result<VerifiedResolvedOwnerHeaderV1, ResolvedOwnerHeaderSealErrorV1> {
        VerifiedResolvedOwnerHeaderV1::seal_input(
            CanonicalFirstFamilyPlanBrandV1::from_family(
                ResolvedOwnerHeaderFamilyV1::TrivialBindingSsa,
            ),
            self.function,
        )
    }

    pub(crate) fn direct_call_count(&self) -> usize {
        self.profile.direct_calls().len()
    }

    pub(crate) fn completion(&self) -> &VerifiedFunctionCompletionV1 {
        &self.completion
    }

    pub(crate) fn terminal_profile(
        &self,
    ) -> &crate::mir::resolved_value_profile::product::TrivialTerminalProfileV1 {
        self.profile.terminal()
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        ResolvedFunctionLoweringInputV1<'a>,
        VerifiedResolvedFunctionIfControlV1,
        VerifiedFunctionCompletionV1,
        VerifiedTrivialCanonicalOwnerV1,
        usize,
    ) {
        (
            self.function,
            self.if_control,
            self.completion,
            self.profile,
            self.block_expr_count,
        )
    }
}
