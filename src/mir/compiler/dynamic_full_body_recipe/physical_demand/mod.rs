//! Builder-free whole-program physical demand for the bounded Dynamic cohort.
//!
//! This module consumes only the complete HRTB physical-input view issued by
//! the final Dynamic exit transaction. It does not adapt the V1 demand path,
//! reread Recipe/JoinSig/source, select one operation, or create physical IDs.

mod issuer;
mod model;

pub(in crate::mir) use issuer::issue_dynamic_full_loop_operation_physical_demand_v2;
pub(in crate::mir) use model::{
    DynamicFullLoopPhysicalDemandCoverageV2, DynamicFullLoopPhysicalDemandRejectV2,
    PreparedDynamicLoopOperationProgramV2, VerifiedDynamicLoopOperationPhysicalDemandV2,
};
