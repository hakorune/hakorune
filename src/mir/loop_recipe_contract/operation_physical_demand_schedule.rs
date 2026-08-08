//! Recipe-order schedule construction for the full Loop operation demand.

use std::collections::{BTreeMap, BTreeSet};

use super::super::ids::{LoopBlockKeyV1, LoopItemKeyV1, LoopNodeKeyV1};
use super::super::operation_effect::VerifiedLoopOperationEffectProductV1;
use super::super::schema::{LoopConditionV1, LoopRecipeItemV1, LoopRecipeV1};
use super::operation_physical_demand_rows::PreparedLoopOperationScheduleRowV1;
use super::{LoopOperationPhysicalDemandRejectV1, LoopOperationPhysicalIndexV1};

pub(super) fn append_operation_schedule(
    recipe: &LoopRecipeV1,
    loop_key: LoopNodeKeyV1,
    item_rows: &BTreeMap<LoopItemKeyV1, LoopRecipeItemV1>,
    operation_effect: &VerifiedLoopOperationEffectProductV1,
    index: &LoopOperationPhysicalIndexV1,
    seen: &mut BTreeSet<LoopItemKeyV1>,
    schedule: &mut Vec<PreparedLoopOperationScheduleRowV1>,
) -> Result<(), LoopOperationPhysicalDemandRejectV1> {
    let expected = operation_effect.evidence().len();
    let Some(loop_node) = recipe.loops.iter().find(|row| row.key == loop_key) else {
        return Err(LoopOperationPhysicalDemandRejectV1::IncompleteSchedule {
            expected,
            found: schedule.len(),
        });
    };
    if let LoopConditionV1::Predicate { block, .. } = loop_node.condition {
        append_block_operation_schedule(
            recipe,
            block,
            item_rows,
            operation_effect,
            index,
            seen,
            schedule,
        )?;
    }
    append_block_operation_schedule(
        recipe,
        loop_node.body,
        item_rows,
        operation_effect,
        index,
        seen,
        schedule,
    )
}

fn append_block_operation_schedule(
    recipe: &LoopRecipeV1,
    block_key: LoopBlockKeyV1,
    item_rows: &BTreeMap<LoopItemKeyV1, LoopRecipeItemV1>,
    operation_effect: &VerifiedLoopOperationEffectProductV1,
    index: &LoopOperationPhysicalIndexV1,
    seen: &mut BTreeSet<LoopItemKeyV1>,
    schedule: &mut Vec<PreparedLoopOperationScheduleRowV1>,
) -> Result<(), LoopOperationPhysicalDemandRejectV1> {
    let expected = operation_effect.evidence().len();
    let Some(block) = recipe.blocks.iter().find(|row| row.key == block_key) else {
        return Err(LoopOperationPhysicalDemandRejectV1::IncompleteSchedule {
            expected,
            found: schedule.len(),
        });
    };
    for item in &block.items {
        match item_rows.get(item).cloned() {
            Some(LoopRecipeItemV1::Operation { .. }) => {
                append_one_operation_schedule(
                    *item,
                    block,
                    operation_effect,
                    index,
                    seen,
                    schedule,
                )?;
            }
            Some(LoopRecipeItemV1::If {
                then_block,
                else_block,
                ..
            }) => {
                append_block_operation_schedule(
                    recipe,
                    then_block,
                    item_rows,
                    operation_effect,
                    index,
                    seen,
                    schedule,
                )?;
                if let Some(else_block) = else_block {
                    append_block_operation_schedule(
                        recipe,
                        else_block,
                        item_rows,
                        operation_effect,
                        index,
                        seen,
                        schedule,
                    )?;
                }
            }
            Some(LoopRecipeItemV1::Loop { loop_key }) => {
                append_operation_schedule(
                    recipe,
                    loop_key,
                    item_rows,
                    operation_effect,
                    index,
                    seen,
                    schedule,
                )?;
            }
            Some(LoopRecipeItemV1::Exit { .. }) => {}
            None => {
                return Err(LoopOperationPhysicalDemandRejectV1::IncompleteSchedule {
                    expected,
                    found: schedule.len(),
                });
            }
        }
    }
    Ok(())
}

fn append_one_operation_schedule(
    item: LoopItemKeyV1,
    block: &super::super::schema::LoopRecipeBlockV1,
    operation_effect: &VerifiedLoopOperationEffectProductV1,
    index: &LoopOperationPhysicalIndexV1,
    seen: &mut BTreeSet<LoopItemKeyV1>,
    schedule: &mut Vec<PreparedLoopOperationScheduleRowV1>,
) -> Result<(), LoopOperationPhysicalDemandRejectV1> {
    if !seen.insert(item) {
        return Err(LoopOperationPhysicalDemandRejectV1::DuplicateSchedule { item });
    }
    let Some(evidence_index) = index.evidence_by_item.get(&item).copied() else {
        return Err(LoopOperationPhysicalDemandRejectV1::MissingEvidence { item });
    };
    let evidence = &operation_effect.evidence()[evidence_index];
    if evidence.block() != block.key || evidence.owner_loop() != block.owner_loop {
        return Err(LoopOperationPhysicalDemandRejectV1::EvidencePlacementMismatch { item });
    }
    schedule.push(PreparedLoopOperationScheduleRowV1 {
        item,
        block: block.key,
        owner_loop: block.owner_loop,
    });
    Ok(())
}
