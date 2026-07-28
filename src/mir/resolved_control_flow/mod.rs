//! Control-only, pre-Builder products for canonical resolved-source lowering.
//!
//! See `README.md` before adding a producer or consumer connection.

#![allow(dead_code)]

mod cleanup;
mod function_control;
pub(crate) mod if_control;
mod source_coverage;

#[cfg(test)]
mod function_control_tests;
#[cfg(test)]
mod if_control_tests;
#[cfg(test)]
mod source_coverage_tests;

pub(crate) use function_control::{
    verify_function_completion_v1, DeclaredFunctionResultContractV1,
    FunctionCompletionVerificationErrorV1, FunctionExitCoverageV1, FunctionUnitOriginV1,
    ReturnExitRelationV1, SealedFunctionExitContractV1, SealedFunctionExitDispositionV1,
    VerifiedFunctionCompletionV1,
};
