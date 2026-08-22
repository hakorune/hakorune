//! Pre-Builder executable representation proofs for canonical resolved source.
//!
//! See `README.md` before adding a producer or consumer connection.

mod analyzer;
mod analyzer_mode;
mod analyzer_policy;
mod consumption;
mod coverage;
mod direct_call;
pub(crate) mod error;
mod function_return;
mod nested_recipe_facts;
mod nested_recipe_mapper;
mod operator;
mod parameter_entry;
pub(crate) mod product;
mod recipe_facts;
mod recipe_mapper;
mod recipe_source_paths;

#[cfg(test)]
mod direct_call_tests;
#[cfg(test)]
mod nested_recipe_tests;
#[cfg(test)]
mod parameter_tests;
#[cfg(test)]
mod recipe_call_tests;
#[cfg(test)]
mod return_tests;
#[cfg(test)]
mod tests;

#[derive(Debug)]
pub(crate) enum TrivialCanonicalOwnerAnalysisV1 {
    Admitted(product::VerifiedTrivialCanonicalOwnerV1),
    NotAdmitted(error::TrivialProfileStopV1),
}

pub(crate) fn analyze_trivial_canonical_with_mode_v1(
    input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
    completion: &crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1,
    if_control: &crate::mir::resolved_control_flow::if_control::VerifiedResolvedFunctionIfControlV1,
    mode: TrivialCanonicalAnalysisModeV1,
) -> Result<TrivialCanonicalOwnerAnalysisV1, error::TrivialProfileContractErrorV1> {
    analyzer::analyze_trivial_canonical_with_mode_impl_v1(input, completion, if_control, mode)
}

pub(crate) use analyzer_mode::TrivialCanonicalAnalysisModeV1;
pub(crate) use consumption::TrivialProfileConsumptionV1;
pub(crate) use direct_call::VerifiedTrivialDirectCallV1;
pub(crate) use nested_recipe_facts::VerifiedNestedTrivialIfRecipeFactsV1;
pub(crate) use nested_recipe_mapper::{map_nested_trivial_if_recipe_v1, NestedIfRecipeMapRejectV1};
pub(crate) use recipe_facts::VerifiedTrivialIfRecipeFactsV1;
pub(crate) use recipe_mapper::{map_trivial_if_recipe_v1, IfRecipeMapRejectV1};

#[cfg(test)]
fn analyze_closed_result_for_test(
    input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
    completion: &crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1,
    if_control: &crate::mir::resolved_control_flow::if_control::VerifiedResolvedFunctionIfControlV1,
) -> Result<TrivialCanonicalOwnerAnalysisV1, error::TrivialProfileContractErrorV1> {
    analyze_trivial_canonical_with_mode_v1(
        input,
        completion,
        if_control,
        TrivialCanonicalAnalysisModeV1::OrdinaryClosed,
    )
}

#[cfg(test)]
fn analyze_closed_for_test(
    input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
    completion: &crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1,
    if_control: &crate::mir::resolved_control_flow::if_control::VerifiedResolvedFunctionIfControlV1,
) -> TrivialCanonicalOwnerAnalysisV1 {
    analyze_closed_result_for_test(input, completion, if_control).unwrap()
}
