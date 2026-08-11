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
    LoopConditionV2, LoopExitKindV2, LoopJoinBranchArmTransferRefV2, LoopJoinBranchExitRefV2,
    LoopJoinBranchExitTargetV2, LoopJoinEdgeRoleV1, LoopJoinLogicalTransferRejectV2,
    LoopJoinLogicalTransferViewV2, LoopOperationExecutionClassV2, LoopOperationFaultFamilyV2,
    LoopRecipeProvenanceV1,
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
    let control = issue_control(placements, controls, logical)?;
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
                LoopOperationFaultFamilyV2::DynamicAdd => {
                    return Err(DynamicFullLoopPhysicalInputRejectV2::FaultCoverage)
                }
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
    placements: &[DynamicFullLoopPhysicalItemPlacementV2; DYNAMIC_FULL_LOOP_PHYSICAL_ITEM_COUNT_V2],
    rows: Box<[DynamicLoopPhysicalControlRowV2]>,
    logical: LoopJoinLogicalTransferViewV2<'program>,
) -> Result<DynamicLoopPhysicalControlViewV2<'program>, DynamicFullLoopPhysicalInputRejectV2> {
    if rows.len() != 1 || logical.branches().len() != 1 {
        return Err(DynamicFullLoopPhysicalInputRejectV2::ControlCoverage);
    }
    for logical_branch in logical.branches() {
        let matching = rows
            .iter()
            .flat_map(|row| row.branches())
            .filter(|branch| branch.if_item() == logical_branch.if_item)
            .collect::<Vec<_>>();
        if matching.len() != 1 {
            return Err(DynamicFullLoopPhysicalInputRejectV2::BranchCoverage);
        }
        let control_branch = matching[0];
        let Some(control_row) = rows
            .iter()
            .find(|row| row.loop_key() == logical_branch.owner_loop)
        else {
            return Err(DynamicFullLoopPhysicalInputRejectV2::BranchCoverage);
        };
        if control_row.branches().len() != 1
            || control_row.body_block() != control_branch.owner_block()
            || !matches!(control_row.condition(), LoopConditionV2::Predicate { .. })
        {
            return Err(DynamicFullLoopPhysicalInputRejectV2::ControlCoverage);
        }
        let if_placements = placements
            .iter()
            .filter(|placement| placement.item() == logical_branch.if_item)
            .collect::<Vec<_>>();
        if if_placements.len() != 1
            || if_placements[0].owner_loop() != logical_branch.owner_loop
            || if_placements[0].block() != control_branch.owner_block()
            || if_placements[0].kind() != super::super::DynamicFullLoopPhysicalItemKindV2::If
        {
            return Err(DynamicFullLoopPhysicalInputRejectV2::BranchCoverage);
        }
        if control_branch.condition() != logical_branch.condition {
            return Err(DynamicFullLoopPhysicalInputRejectV2::BranchCoverage);
        }
        if !verify_arm(
            logical_branch.then_arm,
            control_branch.then_arm(),
            placements,
            logical_branch.owner_loop,
            Some(control_branch.then_block()),
        ) || !verify_arm(
            logical_branch.else_arm,
            control_branch.else_arm(),
            placements,
            logical_branch.owner_loop,
            control_branch.else_block(),
        ) {
            return Err(DynamicFullLoopPhysicalInputRejectV2::BranchCoverage);
        }
    }
    Ok(DynamicLoopPhysicalControlViewV2 { logical, rows })
}

fn verify_arm(
    logical: LoopJoinBranchArmTransferRefV2<'_>,
    physical: DynamicLoopPhysicalArmV2,
    placements: &[DynamicFullLoopPhysicalItemPlacementV2; DYNAMIC_FULL_LOOP_PHYSICAL_ITEM_COUNT_V2],
    owner_loop: crate::mir::loop_recipe_contract::LoopNodeKeyV1,
    block: Option<crate::mir::loop_recipe_contract::LoopBlockKeyV1>,
) -> bool {
    match (logical, physical) {
        (
            LoopJoinBranchArmTransferRefV2::Fallthrough { .. },
            DynamicLoopPhysicalArmV2::Fallthrough,
        ) => block.is_none(),
        (
            LoopJoinBranchArmTransferRefV2::Exit(exit),
            DynamicLoopPhysicalArmV2::Exit { item, .. },
        ) => {
            let Some(block) = block else {
                return false;
            };
            let exit_placements = placements
                .iter()
                .filter(|placement| placement.item() == exit.exit_item)
                .collect::<Vec<_>>();
            exit_placements.len() == 1
                && exit_placements[0].owner_loop() == owner_loop
                && exit_placements[0].block() == block
                && exit_placements[0].kind()
                    == super::super::DynamicFullLoopPhysicalItemKindV2::Exit
                && item == exit.exit_item
                && expected_exit_kind(exit).is_some_and(|kind| exit_kind_matches(physical, kind))
        }
        _ => false,
    }
}

fn expected_exit_kind(exit: LoopJoinBranchExitRefV2<'_>) -> Option<LoopExitKindV2> {
    match (exit.role, exit.target) {
        (LoopJoinEdgeRoleV1::Break, LoopJoinBranchExitTargetV2::Loop(target_loop)) => {
            Some(LoopExitKindV2::Break { target_loop })
        }
        (LoopJoinEdgeRoleV1::Continue, LoopJoinBranchExitTargetV2::Loop(target_loop)) => {
            Some(LoopExitKindV2::Continue { target_loop })
        }
        (LoopJoinEdgeRoleV1::Return, LoopJoinBranchExitTargetV2::FunctionExit) => {
            // The logical payload is the JoinSig carrier transfer, not the
            // Recipe Return operand. The latter is already sealed by the
            // physical evidence row for this exact exit item.
            Some(LoopExitKindV2::Return { value: None })
        }
        _ => None,
    }
}

fn exit_kind_matches(physical: DynamicLoopPhysicalArmV2, expected: LoopExitKindV2) -> bool {
    match (physical.exit_kind(), expected) {
        (
            Some(LoopExitKindV2::Break {
                target_loop: actual,
            }),
            LoopExitKindV2::Break {
                target_loop: expected,
            },
        )
        | (
            Some(LoopExitKindV2::Continue {
                target_loop: actual,
            }),
            LoopExitKindV2::Continue {
                target_loop: expected,
            },
        ) => actual == expected,
        (Some(LoopExitKindV2::Return { value }), LoopExitKindV2::Return { .. }) => value.is_some(),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::compiler::dynamic_full_body_recipe::coseal::DynamicFullLoopPhysicalItemKindV2;
    use crate::mir::loop_recipe_contract::{LoopBlockKeyV1, LoopJoinPayloadV2, LoopValueClassV2};

    fn placements_with_exit(
        owner_loop: crate::mir::loop_recipe_contract::LoopNodeKeyV1,
        block: crate::mir::loop_recipe_contract::LoopBlockKeyV1,
        item: crate::mir::loop_recipe_contract::LoopItemKeyV1,
    ) -> [DynamicFullLoopPhysicalItemPlacementV2; DYNAMIC_FULL_LOOP_PHYSICAL_ITEM_COUNT_V2] {
        let mut placements = [DynamicFullLoopPhysicalItemPlacementV2::for_test(
            crate::mir::loop_recipe_contract::LoopItemKeyV1::new(0),
            owner_loop,
            LoopBlockKeyV1::new(1),
            DynamicFullLoopPhysicalItemKindV2::Operation,
        ); DYNAMIC_FULL_LOOP_PHYSICAL_ITEM_COUNT_V2];
        placements[12] = DynamicFullLoopPhysicalItemPlacementV2::for_test(
            item,
            owner_loop,
            block,
            DynamicFullLoopPhysicalItemKindV2::Exit,
        );
        placements
    }

    fn return_arm(
        item: crate::mir::loop_recipe_contract::LoopItemKeyV1,
        value: crate::mir::loop_recipe_contract::LoopValueKeyV1,
    ) -> LoopJoinBranchArmTransferRefV2<'static> {
        let payload = Box::leak(
            vec![LoopJoinPayloadV2 {
                binding: crate::mir::loop_recipe_contract::LoopBindingKeyV1::new(0),
                value,
                class: LoopValueClassV2::I64,
            }]
            .into_boxed_slice(),
        );
        LoopJoinBranchArmTransferRefV2::Exit(LoopJoinBranchExitRefV2 {
            exit_item: item,
            role: LoopJoinEdgeRoleV1::Return,
            target: LoopJoinBranchExitTargetV2::FunctionExit,
            payload,
        })
    }

    fn return_arm_to_loop(
        item: crate::mir::loop_recipe_contract::LoopItemKeyV1,
        value: crate::mir::loop_recipe_contract::LoopValueKeyV1,
        target_loop: crate::mir::loop_recipe_contract::LoopNodeKeyV1,
    ) -> LoopJoinBranchArmTransferRefV2<'static> {
        let payload = Box::leak(
            vec![LoopJoinPayloadV2 {
                binding: crate::mir::loop_recipe_contract::LoopBindingKeyV1::new(0),
                value,
                class: LoopValueClassV2::I64,
            }]
            .into_boxed_slice(),
        );
        LoopJoinBranchArmTransferRefV2::Exit(LoopJoinBranchExitRefV2 {
            exit_item: item,
            role: LoopJoinEdgeRoleV1::Return,
            target: LoopJoinBranchExitTargetV2::Loop(target_loop),
            payload,
        })
    }

    #[test]
    fn if_exit_arm_requires_exact_item_block_kind_and_exit_role() {
        let owner = crate::mir::loop_recipe_contract::LoopNodeKeyV1::new(0);
        let exit_item = crate::mir::loop_recipe_contract::LoopItemKeyV1::new(12);
        let value = crate::mir::loop_recipe_contract::LoopValueKeyV1::new(14);
        let placements = placements_with_exit(owner, LoopBlockKeyV1::new(2), exit_item);
        let logical = return_arm(exit_item, value);
        let physical = DynamicLoopPhysicalArmV2::Exit {
            item: exit_item,
            kind: LoopExitKindV2::Return { value: Some(value) },
        };
        assert!(verify_arm(
            logical,
            physical,
            &placements,
            owner,
            Some(LoopBlockKeyV1::new(2)),
        ));

        assert!(!verify_arm(
            return_arm(
                crate::mir::loop_recipe_contract::LoopItemKeyV1::new(11),
                value
            ),
            physical,
            &placements,
            owner,
            Some(LoopBlockKeyV1::new(2)),
        ));
        assert!(!verify_arm(
            logical,
            physical,
            &placements,
            owner,
            Some(LoopBlockKeyV1::new(1)),
        ));
        assert!(!verify_arm(
            return_arm(exit_item, value),
            DynamicLoopPhysicalArmV2::Exit {
                item: exit_item,
                kind: LoopExitKindV2::Break { target_loop: owner },
            },
            &placements,
            owner,
            Some(LoopBlockKeyV1::new(2)),
        ));
        assert!(!verify_arm(
            return_arm(exit_item, value),
            DynamicLoopPhysicalArmV2::Exit {
                item: exit_item,
                kind: LoopExitKindV2::Return { value: None },
            },
            &placements,
            owner,
            Some(LoopBlockKeyV1::new(2)),
        ));
        assert!(!verify_arm(
            return_arm_to_loop(exit_item, value, owner),
            physical,
            &placements,
            owner,
            Some(LoopBlockKeyV1::new(2)),
        ));
    }

    #[test]
    fn if_fallthrough_arm_requires_no_else_block() {
        let owner = crate::mir::loop_recipe_contract::LoopNodeKeyV1::new(0);
        let placements = placements_with_exit(
            owner,
            LoopBlockKeyV1::new(2),
            crate::mir::loop_recipe_contract::LoopItemKeyV1::new(12),
        );
        let payload: &'static [LoopJoinPayloadV2] = &[];
        let logical = LoopJoinBranchArmTransferRefV2::Fallthrough { payload };
        assert!(verify_arm(
            logical,
            DynamicLoopPhysicalArmV2::Fallthrough,
            &placements,
            owner,
            None,
        ));
        assert!(!verify_arm(
            logical,
            DynamicLoopPhysicalArmV2::Fallthrough,
            &placements,
            owner,
            Some(LoopBlockKeyV1::new(2)),
        ));
    }
}
