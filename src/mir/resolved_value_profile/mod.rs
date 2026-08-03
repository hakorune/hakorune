//! Pre-Builder executable representation proofs for canonical resolved source.
//!
//! See `README.md` before adding a producer or consumer connection.

mod analyzer;
mod analyzer_policy;
mod consumption;
mod coverage;
mod direct_call;
pub(crate) mod error;
mod function_return;
mod operator;
mod parameter_entry;
mod recipe_mapper;
pub(crate) mod product;
mod recipe_facts;

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

pub(crate) fn analyze_trivial_canonical_main_owner_v1(
    input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
    completion: &crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1,
    if_control: &crate::mir::resolved_control_flow::if_control::VerifiedResolvedFunctionIfControlV1,
    role: crate::mir::compiler::normal_source_plan::VerifiedNormalMainRoleV1,
) -> Result<TrivialCanonicalOwnerAnalysisV1, error::TrivialProfileContractErrorV1> {
    analyzer::analyze_trivial_canonical_main_owner_impl_v1(input, completion, if_control, role)
}

pub(crate) fn analyze_trivial_canonical_main_owner_with_finite_direct_calls_v1(
    input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
    completion: &crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1,
    if_control: &crate::mir::resolved_control_flow::if_control::VerifiedResolvedFunctionIfControlV1,
    role: crate::mir::compiler::normal_source_plan::VerifiedNormalMainRoleV1,
) -> Result<TrivialCanonicalOwnerAnalysisV1, error::TrivialProfileContractErrorV1> {
    analyzer::analyze_trivial_canonical_main_owner_with_finite_direct_calls_impl_v1(
        input, completion, if_control, role,
    )
}

/// Disconnected P0c-F-DX0a analyzer for finite one-or-more exact calls.
/// Callable Program routes use this finite policy; body-only compilation uses
/// the call-disabled entry above.
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
pub(crate) use recipe_mapper::{map_trivial_if_recipe_v1, IfRecipeMapRejectV1};
pub(crate) use recipe_facts::VerifiedTrivialIfRecipeFactsV1;
