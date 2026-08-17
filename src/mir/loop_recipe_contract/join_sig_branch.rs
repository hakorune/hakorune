//! Logical conditional-branch helpers for the caller-zero JoinSig product.

use super::ids::{LoopBlockKeyV1, LoopItemKeyV1, LoopNodeKeyV1, LoopValueKeyV1};
use super::join_sig::recipe_view::{LoopJoinExitView, LoopJoinItemView, LoopJoinRecipeView};
use super::join_sig::{
    visible_payloads_from_view, Flow, LoopJoinBranch, LoopJoinBranchArm, LoopJoinBranchExit,
    LoopJoinBranchTarget, LoopJoinEdgeRoleV1, LoopJoinNextItemV1, LoopJoinPayload,
    LoopJoinSigRejectReasonV1,
};

pub(super) fn branch_row<V: LoopJoinRecipeView>(
    recipe: &V,
    owner_loop: LoopNodeKeyV1,
    if_item: LoopItemKeyV1,
    condition: LoopValueKeyV1,
    then_block: LoopBlockKeyV1,
    else_block: Option<LoopBlockKeyV1>,
    continuation: Option<LoopJoinNextItemV1>,
    then_flow: &Flow<V::Class>,
    else_flow: &Flow<V::Class>,
) -> Result<LoopJoinBranch<V::Class, V::BranchTarget>, LoopJoinSigRejectReasonV1> {
    let then_arm = branch_arm(
        recipe,
        owner_loop,
        if_item,
        then_block,
        continuation,
        then_flow,
    )?;
    let else_arm = match else_block {
        Some(block) => branch_arm(recipe, owner_loop, if_item, block, continuation, else_flow)?,
        None => fallthrough_arm(recipe, owner_loop, if_item, continuation, else_flow)?,
    };
    if !supported_arm_pair(&then_arm, &else_arm) {
        return Err(LoopJoinSigRejectReasonV1::BranchMergeMismatch { item: if_item });
    }
    Ok(LoopJoinBranch {
        owner_loop,
        if_item,
        condition,
        then_arm,
        else_arm,
    })
}

fn branch_arm<V: LoopJoinRecipeView>(
    recipe: &V,
    owner_loop: LoopNodeKeyV1,
    if_item: LoopItemKeyV1,
    block: LoopBlockKeyV1,
    continuation: Option<LoopJoinNextItemV1>,
    flow: &Flow<V::Class>,
) -> Result<LoopJoinBranchArm<V::Class, V::BranchTarget>, LoopJoinSigRejectReasonV1> {
    if let Some((item, kind)) = flow.exit {
        let Some(target) = recipe.branch_exit_target(owner_loop, kind) else {
            return Err(LoopJoinSigRejectReasonV1::BranchMergeMismatch { item });
        };
        if !is_direct_exit_at_block_end(recipe, block, item) {
            return Err(LoopJoinSigRejectReasonV1::BranchMergeMismatch { item });
        }
        return Ok(LoopJoinBranchArm::Exit(branch_exit(
            item,
            kind,
            target,
            visible_payloads_from_view(recipe, owner_loop, &flow.bindings)?,
        )?));
    }
    fallthrough_arm(recipe, owner_loop, if_item, continuation, flow)
}

fn fallthrough_arm<V: LoopJoinRecipeView>(
    recipe: &V,
    owner_loop: LoopNodeKeyV1,
    if_item: LoopItemKeyV1,
    continuation: Option<LoopJoinNextItemV1>,
    flow: &Flow<V::Class>,
) -> Result<LoopJoinBranchArm<V::Class, V::BranchTarget>, LoopJoinSigRejectReasonV1> {
    let continuation = continuation
        .ok_or(LoopJoinSigRejectReasonV1::MissingFallthroughContinuation { item: if_item })?;
    Ok(LoopJoinBranchArm::Fallthrough {
        continuation,
        payload: visible_payloads_from_view(recipe, owner_loop, &flow.bindings)?,
    })
}

fn supported_arm_pair<C, T>(
    then_arm: &LoopJoinBranchArm<C, T>,
    else_arm: &LoopJoinBranchArm<C, T>,
) -> bool {
    match (then_arm, else_arm) {
        (LoopJoinBranchArm::Exit(then_exit), LoopJoinBranchArm::Exit(else_exit)) => {
            then_exit.role == LoopJoinEdgeRoleV1::Break
                && else_exit.role == LoopJoinEdgeRoleV1::Continue
        }
        _ => true,
    }
}

fn branch_exit<C, T: LoopJoinBranchTarget>(
    exit_item: LoopItemKeyV1,
    exit: LoopJoinExitView,
    target: T,
    payload: Vec<LoopJoinPayload<C>>,
) -> Result<LoopJoinBranchExit<C, T>, LoopJoinSigRejectReasonV1> {
    let role = exit_role(exit);
    if !target.accepts(role) {
        return Err(LoopJoinSigRejectReasonV1::BranchMergeMismatch { item: exit_item });
    }
    Ok(LoopJoinBranchExit {
        exit_item,
        role,
        target,
        payload,
    })
}

fn is_direct_exit_at_block_end<V: LoopJoinRecipeView>(
    recipe: &V,
    block: LoopBlockKeyV1,
    exit_item: LoopItemKeyV1,
) -> bool {
    let block = match recipe.block_at(block) {
        Some(block) => block,
        None => return false,
    };
    let Some(item) = block.items.last().copied() else {
        return false;
    };
    if item != exit_item {
        return false;
    }
    matches!(recipe.item_at(item), Some(LoopJoinItemView::Exit { .. }))
}

pub(super) fn is_supported_loop_branch_pair<V: LoopJoinRecipeView>(
    recipe: &V,
    owner_loop: LoopNodeKeyV1,
    then_exit: LoopJoinExitView,
    else_exit: LoopJoinExitView,
) -> bool {
    matches!(then_exit, LoopJoinExitView::Break { target_loop } if target_loop == owner_loop)
        && matches!(else_exit, LoopJoinExitView::Continue { target_loop } if target_loop == owner_loop)
        && recipe.branch_exit_target(owner_loop, then_exit).is_some()
        && recipe.branch_exit_target(owner_loop, else_exit).is_some()
}

fn exit_role(exit: LoopJoinExitView) -> LoopJoinEdgeRoleV1 {
    match exit {
        LoopJoinExitView::Break { .. } => LoopJoinEdgeRoleV1::Break,
        LoopJoinExitView::Continue { .. } => LoopJoinEdgeRoleV1::Continue,
        LoopJoinExitView::Return { .. } => LoopJoinEdgeRoleV1::Return,
    }
}
