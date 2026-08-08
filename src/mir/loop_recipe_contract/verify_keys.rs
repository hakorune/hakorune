//! Canonical-key, lookup, and use-marking helpers for Recipe verification.

use super::super::error::LoopRecipeRejectReasonV1 as Reject;
use super::super::ids::{
    LoopBindingKeyV1, LoopBlockKeyV1, LoopExitKeyV1, LoopItemKeyV1, LoopNodeKeyV1, LoopValueKeyV1,
};
use super::super::schema::{LoopRecipeItemV1, LoopRecipeV1, LoopValueClassV1};

pub(super) fn check_canonical_keys(recipe: &LoopRecipeV1) -> Result<(), Reject> {
    for (domain, canonical) in [
        (
            "loops",
            recipe
                .loops
                .iter()
                .enumerate()
                .all(|(i, row)| row.key.raw() == i as u32),
        ),
        (
            "blocks",
            recipe
                .blocks
                .iter()
                .enumerate()
                .all(|(i, row)| row.key.raw() == i as u32),
        ),
        (
            "items",
            recipe
                .items
                .iter()
                .enumerate()
                .all(|(i, row)| row.key.raw() == i as u32),
        ),
        (
            "bindings",
            recipe
                .bindings
                .iter()
                .enumerate()
                .all(|(i, row)| row.key.raw() == i as u32),
        ),
        (
            "values",
            recipe
                .values
                .iter()
                .enumerate()
                .all(|(i, row)| row.key.raw() == i as u32),
        ),
        (
            "carriers",
            recipe
                .carriers
                .iter()
                .enumerate()
                .all(|(i, row)| row.key.raw() == i as u32),
        ),
        (
            "exits",
            recipe
                .exits
                .iter()
                .enumerate()
                .all(|(i, row)| row.key.raw() == i as u32),
        ),
    ] {
        if !canonical {
            return Err(Reject::NonCanonicalKeyOrder { domain });
        }
    }
    Ok(())
}

pub(super) fn mark_block_use(uses: &mut [u8], key: LoopBlockKeyV1) -> Result<(), Reject> {
    let Some(slot) = uses.get_mut(key.raw() as usize) else {
        return Err(Reject::DanglingBlock { key });
    };
    *slot += 1;
    if *slot > 1 {
        return Err(Reject::DuplicateBlockUse { key });
    }
    Ok(())
}

pub(super) fn mark_item_use(uses: &mut [u8], key: LoopItemKeyV1) -> Result<(), Reject> {
    let Some(slot) = uses.get_mut(key.raw() as usize) else {
        return Err(Reject::DanglingItem { key });
    };
    *slot += 1;
    if *slot > 1 {
        return Err(Reject::DuplicateItemUse { key });
    }
    Ok(())
}

pub(super) fn mark_exit_use(uses: &mut [u8], key: LoopExitKeyV1) -> Result<(), Reject> {
    let Some(slot) = uses.get_mut(key.raw() as usize) else {
        return Err(Reject::DanglingExit { key });
    };
    *slot += 1;
    if *slot > 1 {
        return Err(Reject::DuplicateExitUse { key });
    }
    Ok(())
}

pub(super) fn loop_at(
    recipe: &LoopRecipeV1,
    key: LoopNodeKeyV1,
) -> Option<&super::super::schema::LoopNodeV1> {
    recipe
        .loops
        .get(key.raw() as usize)
        .filter(|row| row.key == key)
}

pub(super) fn block_at(
    recipe: &LoopRecipeV1,
    key: LoopBlockKeyV1,
) -> Option<&super::super::schema::LoopRecipeBlockV1> {
    recipe
        .blocks
        .get(key.raw() as usize)
        .filter(|row| row.key == key)
}

pub(super) fn item_at(
    recipe: &LoopRecipeV1,
    key: LoopItemKeyV1,
) -> Option<&super::super::schema::LoopRecipeItemRowV1> {
    recipe
        .items
        .get(key.raw() as usize)
        .filter(|row| row.key == key)
}

pub(super) fn exit_at(
    recipe: &LoopRecipeV1,
    key: LoopExitKeyV1,
) -> Option<&super::super::schema::LoopRecipeExitV1> {
    recipe
        .exits
        .get(key.raw() as usize)
        .filter(|row| row.key == key)
}

pub(super) fn loop_entry_item(recipe: &LoopRecipeV1, key: LoopNodeKeyV1) -> Option<LoopItemKeyV1> {
    if key == recipe.root_loop {
        return None;
    }
    recipe.items.iter().find_map(|row| match row.item {
        LoopRecipeItemV1::Loop { loop_key } if loop_key == key => Some(row.key),
        _ => None,
    })
}

pub(super) fn binding_class(
    recipe: &LoopRecipeV1,
    key: LoopBindingKeyV1,
) -> Result<LoopValueClassV1, Reject> {
    recipe
        .bindings
        .get(key.raw() as usize)
        .filter(|row| row.key == key)
        .map(|row| row.class)
        .ok_or(Reject::DanglingBinding { key })
}

pub(super) fn value_class(
    recipe: &LoopRecipeV1,
    key: LoopValueKeyV1,
) -> Result<LoopValueClassV1, Reject> {
    recipe
        .values
        .get(key.raw() as usize)
        .filter(|row| row.key == key)
        .map(|row| row.class)
        .ok_or(Reject::DanglingValue { key })
}

pub(super) fn expect_value_class(
    recipe: &LoopRecipeV1,
    key: LoopValueKeyV1,
    expected: LoopValueClassV1,
) -> Result<(), Reject> {
    if value_class(recipe, key)? != expected {
        return Err(Reject::ValueClassMismatch { key });
    }
    Ok(())
}

pub(super) fn is_ancestor_or_self(
    recipe: &LoopRecipeV1,
    owner: LoopNodeKeyV1,
    target: LoopNodeKeyV1,
) -> bool {
    let mut cursor = Some(owner);
    while let Some(key) = cursor {
        if key == target {
            return true;
        }
        cursor = loop_at(recipe, key).and_then(|row| row.parent);
    }
    false
}
