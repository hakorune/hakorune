use std::collections::{btree_map::Entry, BTreeMap, BTreeSet};

use super::super::ids::{LoopBlockKeyV1, LoopNodeKeyV1, LoopValueKeyV1};
use super::super::schema::{LoopRecipeItemV1, LoopRecipeV1};
use super::model::{LoopJoinPayloadV1, LoopJoinSigRejectReasonV1};

pub(super) fn seed_carriers(
    recipe: &LoopRecipeV1,
    key: LoopNodeKeyV1,
    bindings: &mut BTreeMap<super::super::ids::LoopBindingKeyV1, LoopValueKeyV1>,
    available: &mut BTreeSet<LoopValueKeyV1>,
) {
    for carrier in recipe
        .carriers
        .iter()
        .filter(|carrier| carrier.owner_loop == key)
    {
        bindings.insert(carrier.binding, carrier.entry_value);
        available.insert(carrier.entry_value);
    }
}

pub(super) fn payloads(
    recipe: &LoopRecipeV1,
    key: LoopNodeKeyV1,
    bindings: &BTreeMap<super::super::ids::LoopBindingKeyV1, LoopValueKeyV1>,
) -> Result<Vec<LoopJoinPayloadV1>, LoopJoinSigRejectReasonV1> {
    recipe
        .carriers
        .iter()
        .filter(|carrier| carrier.owner_loop == key)
        .map(|carrier| {
            let value = bindings.get(&carrier.binding).copied().ok_or(
                LoopJoinSigRejectReasonV1::MissingCarrierClosure {
                    loop_key: key,
                    binding: carrier.binding,
                },
            )?;
            Ok(LoopJoinPayloadV1 {
                binding: carrier.binding,
                value,
                class: carrier.class,
            })
        })
        .collect()
}

pub(in crate::mir::loop_recipe_contract) fn visible_payloads(
    recipe: &LoopRecipeV1,
    key: LoopNodeKeyV1,
    bindings: &BTreeMap<super::super::ids::LoopBindingKeyV1, LoopValueKeyV1>,
) -> Result<Vec<LoopJoinPayloadV1>, LoopJoinSigRejectReasonV1> {
    let mut visible = BTreeMap::new();
    let mut cursor = Some(key);
    while let Some(owner) = cursor {
        for carrier in recipe
            .carriers
            .iter()
            .filter(|carrier| carrier.owner_loop == owner)
        {
            // Walk from the target toward the root: the first carrier is the
            // innermost one, so an ancestor with the same binding is hidden.
            let Entry::Vacant(slot) = visible.entry(carrier.binding) else {
                continue;
            };
            let value = bindings.get(&carrier.binding).copied().ok_or(
                LoopJoinSigRejectReasonV1::MissingCarrierClosure {
                    loop_key: carrier.owner_loop,
                    binding: carrier.binding,
                },
            )?;
            slot.insert(LoopJoinPayloadV1 {
                binding: carrier.binding,
                value,
                class: carrier.class,
            });
        }
        cursor = recipe
            .loops
            .get(owner.raw() as usize)
            .and_then(|node| node.parent);
    }
    Ok(visible.into_values().collect())
}

pub(super) fn block_item(
    recipe: &LoopRecipeV1,
    key: LoopBlockKeyV1,
) -> super::super::ids::LoopItemKeyV1 {
    recipe
        .blocks
        .get(key.raw() as usize)
        .and_then(|block| block.items.first().copied())
        .unwrap_or(super::super::ids::LoopItemKeyV1::new(0))
}

pub(super) fn has_only_operations(
    recipe: &LoopRecipeV1,
    block: LoopBlockKeyV1,
    allowed: fn(super::super::schema::LoopOperationV1) -> bool,
) -> bool {
    let Some(block) = recipe.blocks.get(block.raw() as usize) else {
        return false;
    };
    block.items.iter().all(|item_key| {
        let Some(row) = recipe.items.get(item_key.raw() as usize) else {
            return false;
        };
        matches!(&row.item, LoopRecipeItemV1::Operation { operation } if allowed(*operation))
    })
}
