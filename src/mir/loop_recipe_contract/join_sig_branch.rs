//! Shape-specific helpers for the caller-zero LoopTrue branch product.

use super::ids::{LoopBlockKeyV1, LoopItemKeyV1, LoopNodeKeyV1, LoopValueKeyV1};
use super::join_sig::{
    visible_payloads, Flow, LoopJoinBranchExitV1, LoopJoinBranchV1, LoopJoinEdgeRoleV1,
    LoopJoinEdgeV1, LoopJoinPayloadV1, LoopJoinPortV1, LoopJoinSigRejectReasonV1,
};
use super::schema::{LoopExitKindV1, LoopRecipeItemV1, LoopRecipeV1};

pub(super) fn direct_branch_row(
    recipe: &LoopRecipeV1,
    owner_loop: LoopNodeKeyV1,
    if_item: LoopItemKeyV1,
    condition: LoopValueKeyV1,
    then_block: LoopBlockKeyV1,
    else_block: Option<LoopBlockKeyV1>,
    then_flow: &Flow,
    else_flow: &Flow,
) -> Result<LoopJoinBranchV1, LoopJoinSigRejectReasonV1> {
    let Some(else_block) = else_block else {
        return Err(LoopJoinSigRejectReasonV1::BranchMergeMismatch { item: if_item });
    };
    let Some((then_item, then_kind)) = direct_exit(recipe, then_block) else {
        return Err(LoopJoinSigRejectReasonV1::BranchMergeMismatch { item: if_item });
    };
    let Some((else_item, else_kind)) = direct_exit(recipe, else_block) else {
        return Err(LoopJoinSigRejectReasonV1::BranchMergeMismatch { item: if_item });
    };
    if !is_supported_loop_branch_pair(owner_loop, then_kind, else_kind) {
        return Err(LoopJoinSigRejectReasonV1::BranchMergeMismatch { item: if_item });
    }
    Ok(LoopJoinBranchV1 {
        owner_loop,
        if_item,
        condition,
        then_exit: branch_exit(
            owner_loop,
            then_item,
            then_kind,
            visible_payloads(recipe, owner_loop, &then_flow.bindings)?,
        ),
        else_exit: branch_exit(
            owner_loop,
            else_item,
            else_kind,
            visible_payloads(recipe, owner_loop, &else_flow.bindings)?,
        ),
    })
}

fn branch_exit(
    owner_loop: LoopNodeKeyV1,
    exit_item: LoopItemKeyV1,
    exit: LoopExitKindV1,
    payload: Vec<LoopJoinPayloadV1>,
) -> LoopJoinBranchExitV1 {
    LoopJoinBranchExitV1 {
        exit_item,
        role: exit_role(exit),
        target_loop: owner_loop,
        payload,
    }
}

fn direct_exit(
    recipe: &LoopRecipeV1,
    block: LoopBlockKeyV1,
) -> Option<(LoopItemKeyV1, LoopExitKindV1)> {
    let block = recipe.blocks.get(block.raw() as usize)?;
    if block.items.len() != 1 {
        return None;
    }
    let item = block.items[0];
    let row = recipe.items.get(item.raw() as usize)?;
    let LoopRecipeItemV1::Exit { exit } = row.item else {
        return None;
    };
    Some((item, recipe.exits.get(exit.raw() as usize)?.kind))
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

pub(super) fn loop_exit_edge(
    exit: LoopExitKindV1,
    payload: Vec<LoopJoinPayloadV1>,
) -> LoopJoinEdgeV1 {
    match exit {
        LoopExitKindV1::Break { .. } => LoopJoinEdgeV1 {
            from: LoopJoinPortV1::Body,
            to: LoopJoinPortV1::After,
            role: LoopJoinEdgeRoleV1::Break,
            payload,
        },
        LoopExitKindV1::Continue { .. } => LoopJoinEdgeV1 {
            from: LoopJoinPortV1::Body,
            to: LoopJoinPortV1::Header,
            role: LoopJoinEdgeRoleV1::Continue,
            payload,
        },
        LoopExitKindV1::Return { .. } => unreachable!("direct branch rows reject Return"),
    }
}
