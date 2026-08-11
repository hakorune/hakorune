//! Move-only whole-program Dynamic physical-demand products.

use std::collections::BTreeMap;

use super::super::coseal::DynamicFullLoopFaultCutPointCatalogRefV2;
use super::super::coseal::{
    DynamicFullLoopOperationPhysicalRefV2, DynamicFullLoopPhysicalInputViewV2,
    DynamicFullLoopPhysicalItemPlacementV2, DynamicLoopPhysicalControlViewV2,
    DYNAMIC_FULL_LOOP_PHYSICAL_ITEM_COUNT_V2, DYNAMIC_FULL_LOOP_PHYSICAL_OPERATION_COUNT_V2,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum DynamicFullLoopPhysicalDemandRejectV2 {
    PlacementCoverage,
    OperationCoverage,
    OperationOrder,
    ControlCoverage,
    FaultCoverage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) struct DynamicFullLoopPhysicalDemandCoverageV2 {
    operation_count: usize,
    placement_count: usize,
    control_count: usize,
    fault_count: usize,
}

impl DynamicFullLoopPhysicalDemandCoverageV2 {
    pub(in crate::mir) const fn operation_count(self) -> usize {
        self.operation_count
    }

    pub(in crate::mir) const fn placement_count(self) -> usize {
        self.placement_count
    }

    pub(in crate::mir) const fn control_count(self) -> usize {
        self.control_count
    }

    pub(in crate::mir) const fn fault_count(self) -> usize {
        self.fault_count
    }
}

/// Complete Dynamic physical demand. The input view is retained as one
/// borrowed whole; no raw Recipe/JoinSig/source product can be reassembled.
#[derive(Debug)]
pub(in crate::mir) struct VerifiedDynamicLoopOperationPhysicalDemandV2<'program> {
    input: DynamicFullLoopPhysicalInputViewV2<'program>,
    coverage: DynamicFullLoopPhysicalDemandCoverageV2,
}

impl<'program> VerifiedDynamicLoopOperationPhysicalDemandV2<'program> {
    pub(super) fn from_parts(
        input: DynamicFullLoopPhysicalInputViewV2<'program>,
        coverage: DynamicFullLoopPhysicalDemandCoverageV2,
    ) -> Self {
        Self { input, coverage }
    }
}

/// Whole-program operation demand prepared without Builder effects.
#[derive(Debug)]
pub(in crate::mir) struct PreparedDynamicLoopOperationProgramV2<'program> {
    demand: VerifiedDynamicLoopOperationPhysicalDemandV2<'program>,
}

impl<'program> VerifiedDynamicLoopOperationPhysicalDemandV2<'program> {
    pub(in crate::mir) fn prepare_all(
        self,
    ) -> Result<
        PreparedDynamicLoopOperationProgramV2<'program>,
        DynamicFullLoopPhysicalDemandRejectV2,
    > {
        Ok(PreparedDynamicLoopOperationProgramV2 { demand: self })
    }
}

impl PreparedDynamicLoopOperationProgramV2<'_> {
    pub(in crate::mir) const fn coverage(&self) -> DynamicFullLoopPhysicalDemandCoverageV2 {
        self.demand.coverage
    }

    /// The only operation surface is the complete Recipe-order array.
    pub(in crate::mir) fn operation_rows(
        &self,
    ) -> &[DynamicFullLoopOperationPhysicalRefV2<'_>; DYNAMIC_FULL_LOOP_PHYSICAL_OPERATION_COUNT_V2]
    {
        self.demand.input.operations()
    }

    pub(in crate::mir) fn placement_rows(
        &self,
    ) -> &[DynamicFullLoopPhysicalItemPlacementV2; DYNAMIC_FULL_LOOP_PHYSICAL_ITEM_COUNT_V2] {
        self.demand.input.placements()
    }

    pub(in crate::mir) fn control(&self) -> &DynamicLoopPhysicalControlViewV2<'_> {
        self.demand.input.control()
    }

    pub(in crate::mir) const fn faults(&self) -> DynamicFullLoopFaultCutPointCatalogRefV2<'_> {
        self.demand.input.faults()
    }
}

pub(super) fn coverage_of(
    input: &DynamicFullLoopPhysicalInputViewV2<'_>,
) -> Result<DynamicFullLoopPhysicalDemandCoverageV2, DynamicFullLoopPhysicalDemandRejectV2> {
    let placements = input.placements();
    let operations = input.operations();
    let operation_placements = placements
        .iter()
        .filter(|row| {
            row.kind() == super::super::coseal::DynamicFullLoopPhysicalItemKindV2::Operation
        })
        .collect::<Vec<_>>();
    if placements.len() != DYNAMIC_FULL_LOOP_PHYSICAL_ITEM_COUNT_V2 {
        return Err(DynamicFullLoopPhysicalDemandRejectV2::PlacementCoverage);
    }
    if operation_placements.len() != DYNAMIC_FULL_LOOP_PHYSICAL_OPERATION_COUNT_V2
        || operations.len() != DYNAMIC_FULL_LOOP_PHYSICAL_OPERATION_COUNT_V2
    {
        return Err(DynamicFullLoopPhysicalDemandRejectV2::OperationCoverage);
    }
    let mut placement_by_item = BTreeMap::new();
    for placement in operation_placements {
        if placement_by_item
            .insert(placement.item(), placement)
            .is_some()
        {
            return Err(DynamicFullLoopPhysicalDemandRejectV2::OperationOrder);
        }
    }
    for operation in operations {
        let Some(placement) = placement_by_item.remove(&operation.item()) else {
            return Err(DynamicFullLoopPhysicalDemandRejectV2::OperationOrder);
        };
        if placement.item() != operation.item()
            || placement.block() != operation.block()
            || placement.owner_loop() != operation.owner_loop()
        {
            return Err(DynamicFullLoopPhysicalDemandRejectV2::OperationOrder);
        }
    }
    if !placement_by_item.is_empty() {
        return Err(DynamicFullLoopPhysicalDemandRejectV2::OperationOrder);
    }
    let control_count = input.control().rows().len();
    if control_count != 1 || input.control().logical().branches().len() != 1 {
        return Err(DynamicFullLoopPhysicalDemandRejectV2::ControlCoverage);
    }
    let fault_count = input.faults().rows().len();
    if fault_count != 2 {
        return Err(DynamicFullLoopPhysicalDemandRejectV2::FaultCoverage);
    }
    Ok(DynamicFullLoopPhysicalDemandCoverageV2 {
        operation_count: operations.len(),
        placement_count: placements.len(),
        control_count,
        fault_count,
    })
}
