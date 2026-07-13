//! Lifetime-free, pre-Builder resolved control-flow products.
//!
//! See `README.md` before adding a producer or consumer connection.

#![allow(dead_code, unused_imports)]

mod analyzer;
mod coverage;
mod if_flow;
mod ports;
mod verifier;

#[cfg(test)]
mod if_flow_tests;

pub(crate) use analyzer::{analyze_resolved_function_flow_v1, ResolvedRegionFlowErrorV1};

pub(crate) use coverage::{
    FunctionFlowCoverageDraftV1, IfFlowCoverageDraftV1, VerifiedFunctionFlowCoverageV1,
    VerifiedIfFlowCoverageV1,
};
pub(crate) use if_flow::{VerifiedResolvedFunctionFlowV1, VerifiedResolvedIfFlowV1};
pub(crate) use ports::{
    ResolvedElseFallthroughV1, ResolvedFallthroughPortV1, ResolvedIfConditionEffectsV1,
    ResolvedIfJoinBindingV1, ResolvedIfJoinContractV1, ResolvedIfPortValueSourceV1,
    ResolvedIfWholeEffectsV1,
};
pub(crate) use verifier::ResolvedRegionFlowVerificationErrorV1;
