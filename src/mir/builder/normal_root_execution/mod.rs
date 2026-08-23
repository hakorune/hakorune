//! One-shot normal-root admission before selected-normal Builder effects.
//!
//! Parser preservation owns the source truth. This module consumes that
//! truth once and carries only an admitted execution mode beside the intact
//! final callable source. It does not inspect names, ordinals, or raw AST.

mod consumer;
mod model;

pub(in crate::mir) use consumer::{
    NormalRootExecutionConsumerRejectV1, NormalRootExecutionConsumerV1,
    RejectedNormalRootExecutionConsumptionV1,
};
pub(in crate::mir) use model::{
    AdmittedNormalRootExecutionModeV1, ConsumedNormalRootCallableSourceV1,
    PreparedNormalRootExecutionConsumptionV1,
};

pub(in crate::mir::builder) struct NormalRootExecutionProjectionPermitV1 {
    _seal: NormalRootExecutionProjectionPermitSealV1,
}

struct NormalRootExecutionProjectionPermitSealV1;

impl NormalRootExecutionProjectionPermitV1 {
    const fn issue_for_consumer() -> Self {
        Self {
            _seal: NormalRootExecutionProjectionPermitSealV1,
        }
    }
}

#[cfg(test)]
mod tests;
