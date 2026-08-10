//! Canonical issuer for the complete bounded Dynamic physical demand.

use super::super::coseal::DynamicFullLoopPhysicalInputViewV2;
use super::model::{
    coverage_of, DynamicFullLoopPhysicalDemandRejectV2,
    VerifiedDynamicLoopOperationPhysicalDemandV2,
};

pub(in crate::mir) fn issue_dynamic_full_loop_operation_physical_demand_v2<'program>(
    input: DynamicFullLoopPhysicalInputViewV2<'program>,
) -> Result<
    VerifiedDynamicLoopOperationPhysicalDemandV2<'program>,
    DynamicFullLoopPhysicalDemandRejectV2,
> {
    // The input view is already the final exit-transaction HRTB product. This
    // issuer only consumes it and does not reopen any upstream authority.
    let coverage = coverage_of(&input)?;
    Ok(VerifiedDynamicLoopOperationPhysicalDemandV2::from_parts(
        input, coverage,
    ))
}
