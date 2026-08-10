//! Selector-independent semantic contract for source-bound Dynamic calls.
//!
//! This module sits between the route-neutral source target catalog and any
//! Recipe or physical consumer. See `README.md` for its authority boundary.

mod catalog;
mod model;

pub(crate) use catalog::{
    DynamicInvocationEnvelopeIssueV1, DynamicInvocationEnvelopeLookupV1,
    VerifiedDynamicInvocationEnvelopeCatalogV1,
};
#[allow(unused_imports)]
pub(crate) use model::{
    dynamic_invocation_execution_envelope_v1, DynamicInvocationControlV1,
    DynamicInvocationEffectV1, DynamicInvocationInputHomeV1, DynamicInvocationOrderingV1,
    DynamicInvocationOutcomeV1, DynamicInvocationSuspensionV1,
    VerifiedDynamicInvocationEnvelopeRefV1, VerifiedDynamicInvocationExecutionEnvelopeV1,
};

#[cfg(test)]
mod tests;
