//! Effect-free admission for one common Loop V2 session boundary.
//!
//! This module only co-seals already-issued source products.  It does not
//! construct a session, consume Completion, allocate CFG/SSA state, or
//! reinterpret Recipe/MIR data.

use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::normal_callable_semantic_package::S6CCommonV2PreSessionLoanRefV1;
use crate::mir::resolved_control_flow::if_control::VerifiedResolvedFunctionIfControlV1;
use crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1;
use crate::mir::resolved_semantics::{
    SourceStmtSiteV1, VerifiedResolvedBlockExpressionExpectationV1,
};
use crate::mir::loop_recipe_contract::PreparedLoopV2PreSessionEnvelopeV1;

#[derive(Debug)]
pub(crate) enum LoopV2CanonicalSessionAdmissionIssueV1 {
    CallableOwnerMismatch,
    EnvelopeOwnerMismatch,
    BlockExprOwnerMismatch,
    BlockExprFunctionOriginMismatch,
    BlockExprBodyRootMismatch,
    NoUniqueLoopSite(String),
    OuterIf(String),
    CompletionOwnerMismatch,
    CompletionTargetMismatch,
}

/// Callback-scoped fan-in of the exact source products needed by the future
/// canonical session.  The aggregate is non-Clone and contains no physical
/// IDs; its Completion borrow cannot escape the nested HRTB callback.
#[derive(Debug)]
pub(crate) struct LoopV2CanonicalSessionAdmissionRefV1<
    'source,
    'envelope,
    'completion,
> {
    input: ResolvedFunctionLoweringInputV1<'source>,
    loop_site: SourceStmtSiteV1,
    outer_if: VerifiedResolvedFunctionIfControlV1,
    block_expr_expectation: &'source VerifiedResolvedBlockExpressionExpectationV1,
    envelope: &'envelope PreparedLoopV2PreSessionEnvelopeV1<'envelope, 'envelope>,
    completion: &'completion VerifiedFunctionCompletionV1,
}

impl<'source, 'envelope, 'completion>
    LoopV2CanonicalSessionAdmissionRefV1<'source, 'envelope, 'completion>
{
    pub(crate) const fn input(&self) -> ResolvedFunctionLoweringInputV1<'source> {
        self.input
    }

    pub(crate) fn loop_site(&self) -> &SourceStmtSiteV1 {
        &self.loop_site
    }

    pub(crate) fn outer_if(&self) -> &VerifiedResolvedFunctionIfControlV1 {
        &self.outer_if
    }

    pub(crate) const fn block_expr_expectation(
        &self,
    ) -> &'source VerifiedResolvedBlockExpressionExpectationV1 {
        self.block_expr_expectation
    }

    pub(crate) fn envelope(
        &self,
    ) -> &'envelope PreparedLoopV2PreSessionEnvelopeV1<'envelope, 'envelope> {
        self.envelope
    }

    pub(crate) const fn completion(&self) -> &'completion VerifiedFunctionCompletionV1 {
        self.completion
    }
}

/// Borrow one installed S6C cohort and issue the session admission only for
/// the duration of the nested Completion callback.
pub(crate) fn with_loop_v2_canonical_session_admission<R>(
    loan: &S6CCommonV2PreSessionLoanRefV1<'_, '_, '_>,
    callback: impl for<'completion> FnOnce(
        LoopV2CanonicalSessionAdmissionRefV1<'_, '_, 'completion>,
    ) -> R,
) -> Result<R, LoopV2CanonicalSessionAdmissionIssueV1> {
    let selected = loan.callable().selected();
    let input = selected.source();
    let owner = input.owner();
    if owner != loan.callable().owner() {
        return Err(LoopV2CanonicalSessionAdmissionIssueV1::CallableOwnerMismatch);
    }

    let envelope = loan.envelope();
    if envelope.owner() != owner {
        return Err(LoopV2CanonicalSessionAdmissionIssueV1::EnvelopeOwnerMismatch);
    }

    let expectation = selected.block_expr_expectation();
    if expectation.owner() != owner {
        return Err(LoopV2CanonicalSessionAdmissionIssueV1::BlockExprOwnerMismatch);
    }
    if expectation.function_origin() != input.function().function_origin() {
        return Err(
            LoopV2CanonicalSessionAdmissionIssueV1::BlockExprFunctionOriginMismatch,
        );
    }
    if expectation.body_root() != input.function().root_profile().body_root() {
        return Err(LoopV2CanonicalSessionAdmissionIssueV1::BlockExprBodyRootMismatch);
    }

    let loop_site = input
        .function()
        .only_loop_site()
        .map_err(|error| LoopV2CanonicalSessionAdmissionIssueV1::NoUniqueLoopSite(format!("{error:?}")))?;
    let outer_if = VerifiedResolvedFunctionIfControlV1::empty_for_owned_loop_profile(
        input,
        loop_site.node(),
    )
    .map_err(LoopV2CanonicalSessionAdmissionIssueV1::OuterIf)?;
    if outer_if.owner() != owner {
        return Err(LoopV2CanonicalSessionAdmissionIssueV1::CallableOwnerMismatch);
    }

    loan.callable().with_completion(|completion_ref| {
        let completion = completion_ref.completion();
        if completion.owner() != owner {
            return Err(LoopV2CanonicalSessionAdmissionIssueV1::CompletionOwnerMismatch);
        }
        if completion.target_function() != input.function().function_region() {
            return Err(LoopV2CanonicalSessionAdmissionIssueV1::CompletionTargetMismatch);
        }
        Ok(callback(LoopV2CanonicalSessionAdmissionRefV1 {
            input,
            loop_site,
            outer_if,
            block_expr_expectation: expectation,
            envelope,
            completion,
        }))
    })
}
