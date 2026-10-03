//! Profile-neutral semantic contract for Dynamic operators.
//!
//! This module owns the complete language-wide Add/Less execution envelope.
//! It owns no source/Recipe correspondence, runtime/provider implementation,
//! Home classification, destination flow, or physical projection.

mod issuer;
mod model;

pub(crate) use issuer::issue_dynamic_operator_execution_envelope_v1;
pub(crate) use model::{
    DynamicOperatorDomainV1, DynamicOperatorFamilyV1, DynamicOperatorNormalResultV1,
    DynamicOperatorValueClassV1, VerifiedDynamicOperatorExecutionEnvelopeV1,
};

#[cfg(test)]
mod tests;
