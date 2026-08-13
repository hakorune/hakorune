//! The first physical failure terminals for the selected Dynamic corridor.
//!
//! This module consumes only the already co-sealed lifecycle plan and opaque
//! callout landings.  I6 Fault stops without End; I7 Fault consumes the single
//! I6/V10 lease slot before stopping.  No Fault edge rejoins `After`, and no
//! runtime/provider decision is made here.

use super::callout_corridor::DynamicV2CallOutCorridorV1;
use super::lifecycle_terminal::{
    DynamicV2PhysicalCleanupCursorV1, DynamicV2PhysicalCleanupCutPointV1,
    DynamicV2PhysicalLifecycleTerminalPlanV1,
};
use super::targets::DynamicV2OpaquePhysicalTargetV1;
use super::{DynamicV2I8EmitterRejectV1, DynamicV2PhysicalSessionBrandV1};
use crate::mir::builder::calls::CanonicalFunctionLoweringSessionV1;
use crate::mir::builder::resolved_lowering::canonical_ssa::CanonicalSsaFunctionSessionV2;

fn reject(message: impl Into<String>) -> DynamicV2I8EmitterRejectV1 {
    DynamicV2I8EmitterRejectV1::PhysicalCorridor(message.into())
}

fn emit_fault_terminal(
    canonical: &mut CanonicalSsaFunctionSessionV2<'_>,
    outer: &mut CanonicalFunctionLoweringSessionV1<'_>,
    target: &DynamicV2OpaquePhysicalTargetV1,
    brand: &DynamicV2PhysicalSessionBrandV1,
    site: crate::mir::checked_callout::CheckedCallOutSiteIdV1,
) -> Result<(), DynamicV2I8EmitterRejectV1> {
    if !target.matches(brand) {
        return Err(reject("Fault terminal has a foreign session brand"));
    }
    let block = target.block();
    canonical
        .cfg
        .select_block(outer.builder_view_mut_for_lowering(), block)
        .map_err(|error| reject(error.to_string()))?;
    let function = outer
        .builder_view_mut_for_lowering()
        .function_state
        .current_function
        .as_mut()
        .ok_or_else(|| reject("missing function while emitting Fault terminal"))?;
    canonical
        .cfg
        .emit_checked_callout_fault(function, block, site)
        .map_err(|error| reject(error.to_string()))
}

pub(super) fn emit(
    canonical: &mut CanonicalSsaFunctionSessionV2<'_>,
    outer: &mut CanonicalFunctionLoweringSessionV1<'_>,
    corridor: &DynamicV2CallOutCorridorV1,
    lifecycle: &DynamicV2PhysicalLifecycleTerminalPlanV1,
    cleanup: &mut DynamicV2PhysicalCleanupCursorV1,
    brand: &DynamicV2PhysicalSessionBrandV1,
) -> Result<(), DynamicV2I8EmitterRejectV1> {
    if !corridor.matches(brand) {
        return Err(reject("Fault corridor has a foreign session brand"));
    }
    if !corridor.site_pair_matches(lifecycle.i6_site(), lifecycle.i7_site()) {
        return Err(reject(
            "Fault corridor site pair diverges from lifecycle plan",
        ));
    }

    corridor.with_i6_fault(|i6_fault| {
        emit_fault_terminal(canonical, outer, i6_fault, brand, lifecycle.i6_site())
    })?;
    cleanup
        .claim(DynamicV2PhysicalCleanupCutPointV1::I6Fault)
        .map_err(|error| reject(format!("I6 Fault cleanup claim: {error:?}")))?;

    corridor.with_i7_fault(|i7_fault| {
        if !i7_fault.matches(brand) {
            return Err(reject("I7 Fault landing has a foreign session brand"));
        }
        let block = i7_fault.block();
        canonical
            .cfg
            .select_block(outer.builder_view_mut_for_lowering(), block)
            .map_err(|error| reject(error.to_string()))?;
        canonical
            .emit_checked_callout_end(
                outer.builder_view_mut_for_lowering(),
                block,
                lifecycle.i6_site(),
                lifecycle.lease_slot(),
            )
            .map_err(reject)?;
        let function = outer
            .builder_view_mut_for_lowering()
            .function_state
            .current_function
            .as_mut()
            .ok_or_else(|| reject("missing function while emitting I7 Fault"))?;
        canonical
            .cfg
            .emit_checked_callout_fault(function, block, lifecycle.i7_site())
            .map_err(|error| reject(error.to_string()))
    })?;
    cleanup
        .claim(DynamicV2PhysicalCleanupCutPointV1::I7Fault)
        .map(|_| ())
        .map_err(|error| reject(format!("I7 Fault cleanup claim: {error:?}")))
}
