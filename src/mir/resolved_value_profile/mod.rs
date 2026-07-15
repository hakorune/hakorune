//! Pre-Builder executable representation proofs for canonical resolved source.
//!
//! See `README.md` before adding a producer or consumer connection.

mod analyzer;
mod consumption;
mod coverage;
mod direct_call;
pub(crate) mod error;
mod function_return;
mod operator;
mod parameter_entry;
pub(crate) mod product;

#[cfg(test)]
mod direct_call_tests;
#[cfg(test)]
mod parameter_tests;
#[cfg(test)]
mod return_tests;
#[cfg(test)]
mod tests;

#[derive(Debug)]
pub(crate) enum TrivialCanonicalOwnerAnalysisV1 {
    Admitted(product::VerifiedTrivialCanonicalOwnerV1),
    NotAdmitted(error::TrivialProfileStopV1),
}

pub(crate) fn analyze_trivial_canonical_owner_v1(
    input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
    completion: &crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1,
    if_control: &crate::mir::resolved_control_flow::if_control::VerifiedResolvedFunctionIfControlV1,
) -> Result<TrivialCanonicalOwnerAnalysisV1, error::TrivialProfileContractErrorV1> {
    analyzer::analyze_trivial_canonical_owner_impl_v1(input, completion, if_control)
}

/// Disconnected P0c-S0b analyzer. Production routing continues to call the
/// call-disabled entry above until the atomic P0c-I1 activation.
pub(crate) fn analyze_trivial_canonical_owner_with_direct_call_v1(
    input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
    completion: &crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1,
    if_control: &crate::mir::resolved_control_flow::if_control::VerifiedResolvedFunctionIfControlV1,
) -> Result<TrivialCanonicalOwnerAnalysisV1, error::TrivialProfileContractErrorV1> {
    analyzer::analyze_trivial_canonical_owner_with_direct_call_impl_v1(
        input, completion, if_control,
    )
}

/// Disconnected P0c-F-DX0a analyzer for finite one-or-more exact calls.
/// Production routes retain their existing call-disabled/exact-one facades.
pub(crate) fn analyze_trivial_canonical_owner_with_finite_direct_calls_v1(
    input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
    completion: &crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1,
    if_control: &crate::mir::resolved_control_flow::if_control::VerifiedResolvedFunctionIfControlV1,
) -> Result<TrivialCanonicalOwnerAnalysisV1, error::TrivialProfileContractErrorV1> {
    analyzer::analyze_trivial_canonical_owner_with_finite_direct_calls_impl_v1(
        input, completion, if_control,
    )
}

pub(crate) use consumption::TrivialProfileConsumptionV1;
pub(crate) use direct_call::VerifiedTrivialDirectCallV1;
