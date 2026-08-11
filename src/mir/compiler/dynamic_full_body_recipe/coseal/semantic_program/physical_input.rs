//! Bounded Dynamic physical-input view issued from the final exit co-seal.
//!
//! This is still a Builder-free semantic view. It only lends the complete
//! verified placement, operation/source-effect, CallSlot, Fault, and logical
//! control relations. It does not schedule instructions or create physical
//! blocks, values, ABI facts, completion, or publication.

use crate::mir::compiler::dynamic_full_body_recipe::coseal::semantic_program::VerifiedDynamicFullLoopSemanticProgramV2;
use crate::mir::compiler::dynamic_full_body_recipe::coseal::{
    DynamicFullLoopOperationPhysicalRefV2, DynamicFullLoopPhysicalEvidenceRejectV2,
    DynamicFullLoopPhysicalItemPlacementV2, DynamicLoopPhysicalArmV2,
    DynamicLoopPhysicalControlRowV2, DYNAMIC_FULL_LOOP_PHYSICAL_ITEM_COUNT_V2,
    DYNAMIC_FULL_LOOP_PHYSICAL_OPERATION_COUNT_V2,
};
use crate::mir::loop_recipe_contract::{
    LoopJoinBranchArmTransferRefV2, LoopJoinLogicalTransferRejectV2, LoopJoinLogicalTransferViewV2,
    LoopOperationExecutionClassV2, LoopOperationFaultFamilyV2, LoopRecipeProvenanceV1,
};
use crate::mir::resolved_semantics::{
    FunctionOwnerIdV1, LoopExecutionFrameKeyV1, ResolvedScopeRegionPairV1,
};
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum DynamicFullLoopPhysicalInputRejectV2 {
    LogicalTransfer(LoopJoinLogicalTransferRejectV2),
    RecipeRelations(DynamicFullLoopPhysicalEvidenceRejectV2),
    ControlCoverage,
    BranchCoverage,
    FaultCoverage,
}

#[derive(Debug)]
pub(in crate::mir) struct DynamicLoopPhysicalControlViewV2<'program> {
    logical: LoopJoinLogicalTransferViewV2<'program>,
    rows: Box<[DynamicLoopPhysicalControlRowV2]>,
}

impl DynamicLoopPhysicalControlViewV2<'_> {
    pub(in crate::mir) fn logical(&self) -> &LoopJoinLogicalTransferViewV2<'_> {
        &self.logical
    }

    pub(in crate::mir) fn rows(&self) -> &[DynamicLoopPhysicalControlRowV2] {
        &self.rows
    }
}
#[derive(Debug)]
pub(in crate::mir) struct DynamicFullLoopPhysicalInputViewV2<'program> {
    owner: FunctionOwnerIdV1,
    frame: &'program LoopExecutionFrameKeyV1,
    scope_region: ResolvedScopeRegionPairV1,
    provenance: &'program LoopRecipeProvenanceV1,
    placements: &'program [DynamicFullLoopPhysicalItemPlacementV2;
                  DYNAMIC_FULL_LOOP_PHYSICAL_ITEM_COUNT_V2],
    operations: Box<
        [DynamicFullLoopOperationPhysicalRefV2<'program>;
            DYNAMIC_FULL_LOOP_PHYSICAL_OPERATION_COUNT_V2],
    >,
    control: DynamicLoopPhysicalControlViewV2<'program>,
    faults: super::DynamicFullLoopFaultCutPointCatalogRefV2<'program>,
}

impl DynamicFullLoopPhysicalInputViewV2<'_> {
    pub(in crate::mir) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(in crate::mir) fn frame(&self) -> &LoopExecutionFrameKeyV1 {
        self.frame
    }

    pub(in crate::mir) const fn scope_region(&self) -> ResolvedScopeRegionPairV1 {
        self.scope_region
    }

    pub(in crate::mir) fn provenance(&self) -> &LoopRecipeProvenanceV1 {
        self.provenance
    }

    pub(in crate::mir) fn placements(
        &self,
    ) -> &[DynamicFullLoopPhysicalItemPlacementV2; DYNAMIC_FULL_LOOP_PHYSICAL_ITEM_COUNT_V2] {
        self.placements
    }

    pub(in crate::mir) fn operations(
        &self,
    ) -> &[DynamicFullLoopOperationPhysicalRefV2<'_>; DYNAMIC_FULL_LOOP_PHYSICAL_OPERATION_COUNT_V2]
    {
        &self.operations
    }

    pub(in crate::mir) fn control(&self) -> &DynamicLoopPhysicalControlViewV2<'_> {
        &self.control
    }

    pub(in crate::mir) const fn faults(
        &self,
    ) -> super::DynamicFullLoopFaultCutPointCatalogRefV2<'_> {
        self.faults
    }
}

pub(in crate::mir) fn issue<R>(
    semantic: &VerifiedDynamicFullLoopSemanticProgramV2,
    callback: impl for<'program> FnOnce(DynamicFullLoopPhysicalInputViewV2<'program>) -> R,
) -> Result<R, DynamicFullLoopPhysicalInputRejectV2> {
    let view = issue_view(semantic)?;
    Ok(callback(view))
}

pub(in crate::mir) fn issue_view(
    semantic: &VerifiedDynamicFullLoopSemanticProgramV2,
) -> Result<DynamicFullLoopPhysicalInputViewV2<'_>, DynamicFullLoopPhysicalInputRejectV2> {
    let logical = semantic
        .logical_transfer_view()
        .map_err(DynamicFullLoopPhysicalInputRejectV2::LogicalTransfer)?;
    let envelope = &semantic.envelope;
    let relations = envelope
        .physical_recipe_relations()
        .map_err(DynamicFullLoopPhysicalInputRejectV2::RecipeRelations)?;
    let (placements, operations, controls) = relations.into_parts();
    let control = issue_control(controls, logical)?;
    verify_fault_coverage(&operations, semantic.fault_cut_points())?;
    let view = DynamicFullLoopPhysicalInputViewV2 {
        owner: envelope.source.owner,
        frame: &envelope.source.frame,
        scope_region: envelope.source.scope_region,
        provenance: envelope.artifact.provenance(),
        placements,
        operations,
        control,
        faults: semantic.fault_cut_points(),
    };
    Ok(view)
}

fn verify_fault_coverage(
    operations: &[DynamicFullLoopOperationPhysicalRefV2<'_>;
         DYNAMIC_FULL_LOOP_PHYSICAL_OPERATION_COUNT_V2],
    faults: super::DynamicFullLoopFaultCutPointCatalogRefV2<'_>,
) -> Result<(), DynamicFullLoopPhysicalInputRejectV2> {
    let mut expected = Vec::new();
    for operation in operations {
        let family = match operation.execution() {
            LoopOperationExecutionClassV2::FaultBeforeNormalResult { family, .. } => match family {
                LoopOperationFaultFamilyV2::DynamicAdd => return Err(
                    DynamicFullLoopPhysicalInputRejectV2::FaultCoverage,
                ),
                LoopOperationFaultFamilyV2::DynamicLess => {
                    super::DynamicFullLoopFaultFamilyV2::DynamicLess
                }
            },
            LoopOperationExecutionClassV2::ExternallyBoundOutcome { .. }
                if operation.call_role().is_some() =>
            {
                super::DynamicFullLoopFaultFamilyV2::DynamicInvocation
            }
            _ => continue,
        };
        expected.push((operation.item(), family));
    }
    if expected.len() != faults.rows().len()
        || expected.iter().any(|(item, family)| {
            faults
                .rows()
                .iter()
                .filter(|row| row.item() == *item && row.family() == *family)
                .count()
                != 1
        })
        || faults
            .rows()
            .iter()
            .any(|row| !expected.contains(&(row.item(), row.family())))
    {
        return Err(DynamicFullLoopPhysicalInputRejectV2::FaultCoverage);
    }
    Ok(())
}

fn issue_control<'program>(
    rows: Box<[DynamicLoopPhysicalControlRowV2]>,
    logical: LoopJoinLogicalTransferViewV2<'program>,
) -> Result<DynamicLoopPhysicalControlViewV2<'program>, DynamicFullLoopPhysicalInputRejectV2> {
    if rows.iter().flat_map(|row| row.branches()).count() != logical.branches().len() {
        return Err(DynamicFullLoopPhysicalInputRejectV2::ControlCoverage);
    }
    for logical_branch in logical.branches() {
        let Some(control_branch) = rows
            .iter()
            .flat_map(|row| row.branches())
            .find(|branch| branch.if_item() == logical_branch.if_item)
        else {
            return Err(DynamicFullLoopPhysicalInputRejectV2::BranchCoverage);
        };
        if control_branch.condition() != logical_branch.condition {
            return Err(DynamicFullLoopPhysicalInputRejectV2::BranchCoverage);
        }
        let logical_exit_item = match logical_branch.then_arm {
            LoopJoinBranchArmTransferRefV2::Exit(exit) => Some(exit.exit_item),
            LoopJoinBranchArmTransferRefV2::Fallthrough { .. } => None,
        };
        let control_exit_item = match control_branch.then_arm() {
            DynamicLoopPhysicalArmV2::Exit { item, .. } => Some(item),
            DynamicLoopPhysicalArmV2::Fallthrough => None,
        };
        if logical_exit_item != control_exit_item {
            return Err(DynamicFullLoopPhysicalInputRejectV2::BranchCoverage);
        }
    }
    Ok(DynamicLoopPhysicalControlViewV2 { logical, rows })
}
