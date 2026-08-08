//! Logical conditional-branch helpers for the caller-zero JoinSig product.

use super::ids::{LoopBlockKeyV1, LoopItemKeyV1, LoopNodeKeyV1, LoopValueKeyV1};
use super::join_sig::{
    visible_payloads, Flow, LoopJoinBranchArmV1, LoopJoinBranchExitV1, LoopJoinBranchV1,
    LoopJoinEdgeRoleV1, LoopJoinPayloadV1, LoopJoinSigRejectReasonV1,
};
use super::schema::{LoopExitKindV1, LoopRecipeItemV1, LoopRecipeV1};

pub(super) fn branch_row(
    recipe: &LoopRecipeV1,
    owner_loop: LoopNodeKeyV1,
    if_item: LoopItemKeyV1,
    condition: LoopValueKeyV1,
    then_block: LoopBlockKeyV1,
    else_block: Option<LoopBlockKeyV1>,
    then_flow: &Flow,
    else_flow: &Flow,
) -> Result<LoopJoinBranchV1, LoopJoinSigRejectReasonV1> {
    let then_arm = branch_arm(recipe, owner_loop, then_block, then_flow)?;
    let else_arm = match else_block {
        Some(block) => branch_arm(recipe, owner_loop, block, else_flow)?,
        None => fallthrough_arm(recipe, owner_loop, else_flow)?,
    };
    if !supported_arm_pair(owner_loop, &then_arm, &else_arm) {
        return Err(LoopJoinSigRejectReasonV1::BranchMergeMismatch { item: if_item });
    }
    Ok(LoopJoinBranchV1 {
        owner_loop,
        if_item,
        condition,
        then_arm,
        else_arm,
    })
}

fn branch_arm(
    recipe: &LoopRecipeV1,
    owner_loop: LoopNodeKeyV1,
    block: LoopBlockKeyV1,
    flow: &Flow,
) -> Result<LoopJoinBranchArmV1, LoopJoinSigRejectReasonV1> {
    if let Some((item, kind)) = flow.exit {
        if !is_direct_exit_at_block_end(recipe, block, item)
            || !is_supported_loop_exit(owner_loop, kind)
        {
            return Err(LoopJoinSigRejectReasonV1::BranchMergeMismatch { item });
        }
        return Ok(LoopJoinBranchArmV1::Exit(branch_exit(
            item,
            kind,
            visible_payloads(recipe, owner_loop, &flow.bindings)?,
        )));
    }
    fallthrough_arm(recipe, owner_loop, flow)
}

fn fallthrough_arm(
    recipe: &LoopRecipeV1,
    owner_loop: LoopNodeKeyV1,
    flow: &Flow,
) -> Result<LoopJoinBranchArmV1, LoopJoinSigRejectReasonV1> {
    Ok(LoopJoinBranchArmV1::Fallthrough {
        payload: visible_payloads(recipe, owner_loop, &flow.bindings)?,
    })
}

fn supported_arm_pair(
    _owner_loop: LoopNodeKeyV1,
    then_arm: &LoopJoinBranchArmV1,
    else_arm: &LoopJoinBranchArmV1,
) -> bool {
    match (then_arm, else_arm) {
        (LoopJoinBranchArmV1::Exit(then_exit), LoopJoinBranchArmV1::Exit(else_exit)) => {
            then_exit.role == LoopJoinEdgeRoleV1::Break
                && else_exit.role == LoopJoinEdgeRoleV1::Continue
        }
        _ => true,
    }
}

fn branch_exit(
    exit_item: LoopItemKeyV1,
    exit: LoopExitKindV1,
    payload: Vec<LoopJoinPayloadV1>,
) -> LoopJoinBranchExitV1 {
    LoopJoinBranchExitV1 {
        exit_item,
        role: exit_role(exit),
        target_loop: match exit {
            LoopExitKindV1::Break { target_loop } | LoopExitKindV1::Continue { target_loop } => {
                target_loop
            }
            LoopExitKindV1::Return { .. } => unreachable!("branch rows reject Return"),
        },
        payload,
    }
}

fn is_direct_exit_at_block_end(
    recipe: &LoopRecipeV1,
    block: LoopBlockKeyV1,
    exit_item: LoopItemKeyV1,
) -> bool {
    let block = match recipe.blocks.get(block.raw() as usize) {
        Some(block) => block,
        None => return false,
    };
    let Some(item) = block.items.last().copied() else {
        return false;
    };
    if item != exit_item {
        return false;
    }
    let Some(row) = recipe.items.get(item.raw() as usize) else {
        return false;
    };
    matches!(row.item, LoopRecipeItemV1::Exit { .. })
}

pub(super) fn is_supported_loop_exit(owner_loop: LoopNodeKeyV1, exit: LoopExitKindV1) -> bool {
    matches!(exit, LoopExitKindV1::Break { target_loop } | LoopExitKindV1::Continue { target_loop } if target_loop == owner_loop)
}

pub(super) fn is_supported_loop_branch_pair(
    owner_loop: LoopNodeKeyV1,
    then_exit: LoopExitKindV1,
    else_exit: LoopExitKindV1,
) -> bool {
    matches!(then_exit, LoopExitKindV1::Break { target_loop } if target_loop == owner_loop)
        && matches!(else_exit, LoopExitKindV1::Continue { target_loop } if target_loop == owner_loop)
}

fn exit_role(exit: LoopExitKindV1) -> LoopJoinEdgeRoleV1 {
    match exit {
        LoopExitKindV1::Break { .. } => LoopJoinEdgeRoleV1::Break,
        LoopExitKindV1::Continue { .. } => LoopJoinEdgeRoleV1::Continue,
        LoopExitKindV1::Return { .. } => unreachable!("direct branch rows reject Return"),
    }
}
