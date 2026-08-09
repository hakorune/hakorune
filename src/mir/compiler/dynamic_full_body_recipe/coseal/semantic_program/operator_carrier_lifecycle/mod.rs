//! Exact V9/V17 destination lifecycle relations.
//!
//! This module consumes the complete invocation-lifecycle semantic program as
//! one authority. It issues no Home, rebind transaction, cleanup operation, or
//! physical control-flow state.

mod issuer;
mod model;

#[cfg(test)]
mod tests;

pub(super) use issuer::issue_operator_carrier_lifecycle_v1;
pub(super) use model::VerifiedDynamicOperatorCarrierLifecycleCatalogV1;
pub(in crate::mir) use model::{
    DynamicOperatorCarrierDestinationRefV1, DynamicOperatorCarrierLifecycleCatalogRefV1,
    DynamicOperatorCarrierLifecycleProgramRejectV1, DynamicOperatorCarrierLifecycleRowRefV1,
    DynamicOperatorCarrierPublicationV1,
};
