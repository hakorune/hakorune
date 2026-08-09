//! Profile-neutral semantic contract for Dynamic operators.
//!
//! This module owns the complete language-wide Add/Less execution envelope.
//! It owns no source/Recipe correspondence, runtime/provider implementation,
//! Home classification, destination flow, or physical projection.

mod issuer;
mod model;

pub(crate) use issuer::{
    issue_dynamic_operator_execution_envelope_v1, DynamicOperatorEnvelopeIssueV1,
};
pub(crate) use model::{
    DynamicOperatorControlV1, DynamicOperatorDomainV1, DynamicOperatorEffectV1,
    DynamicOperatorFamilyV1, DynamicOperatorFaultV1, DynamicOperatorInputAccessV1,
    DynamicOperatorNormalResultV1, DynamicOperatorOrderingV1, DynamicOperatorSuspensionV1,
    DynamicOperatorValueClassV1, VerifiedDynamicOperatorExecutionEnvelopeV1,
};

#[cfg(test)]
mod tests;
