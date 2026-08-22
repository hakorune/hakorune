//! Same-cohort Generic G0 Completion transport.
//!
//! `verify_function_completion_v1` remains the only semantic Completion
//! issuer.  This module only checks that its canonical product matches the
//! already-issued Generic result/tail facts before the source parent retains
//! it once; it does not create a second Completion receipt.

use crate::mir::resolved_control_flow::{
    verify_function_completion_v1, DeclaredFunctionResultContractV1,
    FunctionCompletionVerificationErrorV1, VerifiedFunctionCompletionV1,
};
use crate::mir::resolved_semantics::ResolvedExitSiteV1;

use super::function_input::ResolvedFunctionLoweringInputV1;
use super::generic_g0_result_abi::VerifiedGenericG0ResultAbiV1;

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum GenericG0CompletionRejectV1 {
    Verification(FunctionCompletionVerificationErrorV1),
    OwnerMismatch,
    TargetMismatch,
    NotValue,
    ExplicitSiteCountMismatch,
    TailSiteMismatch,
    ResultContractMismatch,
    CleanupNotEmpty,
}

pub(crate) fn issue_generic_g0_completion_transport_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    result_abi: &VerifiedGenericG0ResultAbiV1,
) -> Result<VerifiedFunctionCompletionV1, GenericG0CompletionRejectV1> {
    let completion = verify_function_completion_v1(input)
        .map_err(GenericG0CompletionRejectV1::Verification)?;
    if completion.owner() != input.owner() {
        return Err(GenericG0CompletionRejectV1::OwnerMismatch);
    }
    if completion.target_function() != input.function().function_region() {
        return Err(GenericG0CompletionRejectV1::TargetMismatch);
    }
    if !completion.returns_value() || completion.is_implicit_void() {
        return Err(GenericG0CompletionRejectV1::NotValue);
    }
    let exits = input.function().resolved_exits().collect::<Vec<_>>();
    let [(ResolvedExitSiteV1::Statement(expected_tail), _)] = exits.as_slice() else {
        return Err(GenericG0CompletionRejectV1::TailSiteMismatch);
    };
    let [actual_site] = completion.explicit_sites() else {
        return Err(GenericG0CompletionRejectV1::ExplicitSiteCountMismatch);
    };
    if *actual_site != *expected_tail {
        return Err(GenericG0CompletionRejectV1::TailSiteMismatch);
    }
    match completion.function_exit_contract().declared_result() {
        DeclaredFunctionResultContractV1::Annotated(name)
            if name.as_ref() == result_abi.abi().source_type_name() => {}
        _ => return Err(GenericG0CompletionRejectV1::ResultContractMismatch),
    }
    if !completion.cleanup().crossed_scopes().is_empty() {
        return Err(GenericG0CompletionRejectV1::CleanupNotEmpty);
    }
    Ok(completion)
}
