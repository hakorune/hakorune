//! Pre-Builder executable representation proofs for canonical resolved source.
//!
//! See `README.md` before adding a producer or consumer connection.

mod analyzer;
mod consumption;
mod coverage;
pub(crate) mod error;
mod function_return;
mod operator;
mod parameter_entry;
pub(crate) mod product;

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

pub(crate) use consumption::TrivialProfileConsumptionV1;
