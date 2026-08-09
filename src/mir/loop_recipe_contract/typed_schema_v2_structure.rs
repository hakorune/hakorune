//! Topology and exact-use checks for the explicit V2 logical wire.
//!
//! Operation domains stay in `typed_schema_v2.rs`. This module only restores
//! the neutral tree/use/preorder invariants already required by V1.

use super::ids::{LoopBlockKeyV1, LoopExitKeyV1, LoopItemKeyV1, LoopNodeKeyV1};
use super::schema_v2::{LoopConditionV2, LoopExitKindV2, LoopRecipeItemV2, LoopRecipeV2};
use super::typed_schema_v2::LoopRecipeV2RejectReason as Reject;

pub(super) fn check_control_structure(recipe: &LoopRecipeV2) -> Result<(), Reject> {
    check_loop_tree(recipe)?;
    check_exact_uses(recipe)?;
    check_recursive_preorder(recipe)
}

fn check_loop_tree(recipe: &LoopRecipeV2) -> Result<(), Reject> {
    let Some(root) = recipe.loops.first() else {
        return Err(Reject::UnknownLoop {
            key: recipe.root_loop,
        });
    };
    if root.key != recipe.root_loop || root.parent.is_some() {
        return Err(Reject::InvalidRootParent);
    }

    for node in &recipe.loops {
        if node.key != recipe.root_loop {
            let Some(parent) = node.parent else {
                return Err(Reject::InvalidLoopParent { loop_key: node.key });
            };
            if parent.raw() >= node.key.raw() || loop_at(recipe, parent).is_none() {
                return Err(Reject::InvalidLoopParent { loop_key: node.key });
            }
        }

        let body = block_at(recipe, node.body).ok_or(Reject::UnknownBlock { key: node.body })?;
        if body.owner_loop != node.key {
            return Err(Reject::BlockOwnerMismatch { key: body.key });
        }
        if let LoopConditionV2::Predicate { block, .. } = node.condition {
            let condition = block_at(recipe, block).ok_or(Reject::UnknownBlock { key: block })?;
            if condition.owner_loop != node.key {
                return Err(Reject::BlockOwnerMismatch { key: block });
            }
        }
    }
    Ok(())
}

fn check_exact_uses(recipe: &LoopRecipeV2) -> Result<(), Reject> {
    let mut block_uses = vec![0_u8; recipe.blocks.len()];
    let mut loop_uses = vec![0_u8; recipe.loops.len()];
    let mut exit_uses = vec![0_u8; recipe.exits.len()];
    loop_uses[recipe.root_loop.raw() as usize] = 1;

    for node in &recipe.loops {
        if let LoopConditionV2::Predicate { block, .. } = node.condition {
            mark_block(&mut block_uses, block)?;
        }
        mark_block(&mut block_uses, node.body)?;
    }

    for block in &recipe.blocks {
        let mut terminated = false;
        for item_key in &block.items {
            if terminated {
                return Err(Reject::UnreachableItem { item: *item_key });
            }
            let item = item_at(recipe, *item_key).ok_or(Reject::UnknownItem { key: *item_key })?;
            match item {
                LoopRecipeItemV2::If {
                    then_block,
                    else_block,
                    ..
                } => {
                    for child in [Some(*then_block), *else_block].into_iter().flatten() {
                        let child_block =
                            block_at(recipe, child).ok_or(Reject::UnknownBlock { key: child })?;
                        if child.raw() <= block.key.raw() {
                            return Err(Reject::ChildBlockMustFollowParent { key: child });
                        }
                        if child_block.owner_loop != block.owner_loop {
                            return Err(Reject::BlockOwnerMismatch { key: child });
                        }
                        mark_block(&mut block_uses, child)?;
                    }
                }
                LoopRecipeItemV2::Loop { loop_key } => {
                    let child =
                        loop_at(recipe, *loop_key).ok_or(Reject::UnknownLoop { key: *loop_key })?;
                    if child.parent != Some(block.owner_loop) {
                        return Err(Reject::NestedLoopOwnerMismatch { key: *loop_key });
                    }
                    mark_loop(&mut loop_uses, *loop_key)?;
                }
                LoopRecipeItemV2::Exit { exit } => {
                    let exit_row =
                        exit_at(recipe, *exit).ok_or(Reject::UnknownExit { key: *exit })?;
                    if exit_row.owner_loop != block.owner_loop {
                        return Err(Reject::ExitOwnerMismatch { key: *exit });
                    }
                    mark_exit(&mut exit_uses, *exit)?;
                    terminated = true;
                }
                LoopRecipeItemV2::Operation { .. } => {}
            }
        }
    }

    for (index, count) in block_uses.into_iter().enumerate() {
        if count == 0 {
            return Err(Reject::UnusedBlock {
                key: LoopBlockKeyV1::new(index as u32),
            });
        }
    }
    for (index, count) in loop_uses.into_iter().enumerate() {
        if count == 0 {
            return Err(Reject::UnusedLoop {
                key: LoopNodeKeyV1::new(index as u32),
            });
        }
    }
    for (index, count) in exit_uses.iter().copied().enumerate() {
        if count == 0 {
            return Err(Reject::UnusedExit {
                key: LoopExitKeyV1::new(index as u32),
            });
        }
    }

    check_exit_targets(recipe)
}

fn check_exit_targets(recipe: &LoopRecipeV2) -> Result<(), Reject> {
    for exit in &recipe.exits {
        let target = match exit.kind {
            LoopExitKindV2::Break { target_loop } | LoopExitKindV2::Continue { target_loop } => {
                Some(target_loop)
            }
            LoopExitKindV2::Return { .. } => None,
        };
        if target.is_some_and(|target| !is_ancestor_or_self(recipe, exit.owner_loop, target)) {
            return Err(Reject::ExitTargetNotAncestor { key: exit.key });
        }
    }
    Ok(())
}

fn check_recursive_preorder(recipe: &LoopRecipeV2) -> Result<(), Reject> {
    fn visit_loop(
        recipe: &LoopRecipeV2,
        key: LoopNodeKeyV1,
        next_loop: &mut u32,
        next_block: &mut u32,
        next_item: &mut u32,
    ) -> Result<(), Reject> {
        expect_recursive_key("recursive_loops", *next_loop, key.raw())?;
        *next_loop += 1;
        let node = loop_at(recipe, key).ok_or(Reject::UnknownLoop { key })?;
        if let LoopConditionV2::Predicate { block, .. } = node.condition {
            visit_block(recipe, block, next_loop, next_block, next_item)?;
        }
        visit_block(recipe, node.body, next_loop, next_block, next_item)
    }

    fn visit_block(
        recipe: &LoopRecipeV2,
        key: LoopBlockKeyV1,
        next_loop: &mut u32,
        next_block: &mut u32,
        next_item: &mut u32,
    ) -> Result<(), Reject> {
        expect_recursive_key("recursive_blocks", *next_block, key.raw())?;
        *next_block += 1;
        let block = block_at(recipe, key).ok_or(Reject::UnknownBlock { key })?;
        for item_key in &block.items {
            expect_recursive_key("recursive_items", *next_item, item_key.raw())?;
            *next_item += 1;
            match item_at(recipe, *item_key).ok_or(Reject::UnknownItem { key: *item_key })? {
                LoopRecipeItemV2::If {
                    then_block,
                    else_block,
                    ..
                } => {
                    visit_block(recipe, *then_block, next_loop, next_block, next_item)?;
                    if let Some(else_block) = else_block {
                        visit_block(recipe, *else_block, next_loop, next_block, next_item)?;
                    }
                }
                LoopRecipeItemV2::Loop { loop_key } => {
                    visit_loop(recipe, *loop_key, next_loop, next_block, next_item)?;
                }
                LoopRecipeItemV2::Operation { .. } | LoopRecipeItemV2::Exit { .. } => {}
            }
        }
        Ok(())
    }

    let mut next_loop = 0;
    let mut next_block = 0;
    let mut next_item = 0;
    visit_loop(
        recipe,
        recipe.root_loop,
        &mut next_loop,
        &mut next_block,
        &mut next_item,
    )?;
    expect_recursive_key("recursive_loops", recipe.loops.len() as u32, next_loop)?;
    expect_recursive_key("recursive_blocks", recipe.blocks.len() as u32, next_block)?;
    expect_recursive_key("recursive_items", recipe.items.len() as u32, next_item)
}

fn expect_recursive_key(domain: &'static str, expected: u32, found: u32) -> Result<(), Reject> {
    if expected == found {
        Ok(())
    } else {
        Err(Reject::NonCanonicalKeyOrder {
            domain,
            expected,
            found,
        })
    }
}

fn mark_block(uses: &mut [u8], key: LoopBlockKeyV1) -> Result<(), Reject> {
    let slot = uses
        .get_mut(key.raw() as usize)
        .ok_or(Reject::UnknownBlock { key })?;
    if *slot != 0 {
        return Err(Reject::DuplicateBlockUse { key });
    }
    *slot = 1;
    Ok(())
}

fn mark_loop(uses: &mut [u8], key: LoopNodeKeyV1) -> Result<(), Reject> {
    let slot = uses
        .get_mut(key.raw() as usize)
        .ok_or(Reject::UnknownLoop { key })?;
    if *slot != 0 {
        return Err(Reject::NestedLoopOwnerMismatch { key });
    }
    *slot = 1;
    Ok(())
}

fn mark_exit(uses: &mut [u8], key: LoopExitKeyV1) -> Result<(), Reject> {
    let slot = uses
        .get_mut(key.raw() as usize)
        .ok_or(Reject::UnknownExit { key })?;
    if *slot != 0 {
        return Err(Reject::DuplicateExitUse { key });
    }
    *slot = 1;
    Ok(())
}

fn loop_at(recipe: &LoopRecipeV2, key: LoopNodeKeyV1) -> Option<&super::schema_v2::LoopNodeV2> {
    recipe.loops.get(key.raw() as usize)
}

fn block_at(
    recipe: &LoopRecipeV2,
    key: LoopBlockKeyV1,
) -> Option<&super::schema_v2::LoopRecipeBlockV2> {
    recipe.blocks.get(key.raw() as usize)
}

fn item_at(recipe: &LoopRecipeV2, key: LoopItemKeyV1) -> Option<&LoopRecipeItemV2> {
    recipe.items.get(key.raw() as usize).map(|row| &row.item)
}

fn exit_at(
    recipe: &LoopRecipeV2,
    key: LoopExitKeyV1,
) -> Option<&super::schema_v2::LoopRecipeExitV2> {
    recipe.exits.get(key.raw() as usize)
}

fn is_ancestor_or_self(
    recipe: &LoopRecipeV2,
    mut owner: LoopNodeKeyV1,
    target: LoopNodeKeyV1,
) -> bool {
    loop {
        if owner == target {
            return true;
        }
        let Some(parent) = loop_at(recipe, owner).and_then(|node| node.parent) else {
            return false;
        };
        owner = parent;
    }
}
