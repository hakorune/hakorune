use std::collections::{btree_map::Entry, BTreeMap, BTreeSet};

use super::super::ids::{LoopBindingKeyV1, LoopNodeKeyV1, LoopValueKeyV1};
use super::super::schema::LoopRecipeV1;
use super::model::{LoopJoinPayload, LoopJoinPayloadV1, LoopJoinSigRejectReasonV1};
use super::recipe_view::{
    LoopJoinItemView, LoopJoinOperationFamily, LoopJoinOperationView, LoopJoinRecipeView,
    LoopRecipeV1JoinView,
};

pub(super) fn seed_carriers<V: LoopJoinRecipeView>(
    recipe: &V,
    key: LoopNodeKeyV1,
    bindings: &mut BTreeMap<LoopBindingKeyV1, LoopValueKeyV1>,
    available: &mut BTreeSet<LoopValueKeyV1>,
) {
    for index in 0..recipe.carrier_count() {
        let carrier = recipe
            .carrier_at(index)
            .expect("verified Recipe view has dense carrier rows");
        if carrier.owner_loop == key {
            bindings.insert(carrier.binding, carrier.entry_value);
            available.insert(carrier.entry_value);
        }
    }
}

pub(super) fn payloads<V: LoopJoinRecipeView>(
    recipe: &V,
    key: LoopNodeKeyV1,
    bindings: &BTreeMap<LoopBindingKeyV1, LoopValueKeyV1>,
) -> Result<Vec<LoopJoinPayload<V::Class>>, LoopJoinSigRejectReasonV1> {
    let mut payloads = Vec::new();
    for index in 0..recipe.carrier_count() {
        let carrier = recipe
            .carrier_at(index)
            .expect("verified Recipe view has dense carrier rows");
        if carrier.owner_loop != key {
            continue;
        }
        let value = bindings.get(&carrier.binding).copied().ok_or(
            LoopJoinSigRejectReasonV1::MissingCarrierClosure {
                loop_key: key,
                binding: carrier.binding,
            },
        )?;
        payloads.push(LoopJoinPayload {
            binding: carrier.binding,
            value,
            class: carrier.class,
        });
    }
    Ok(payloads)
}

pub(in crate::mir::loop_recipe_contract) fn visible_payloads_from_view<V: LoopJoinRecipeView>(
    recipe: &V,
    key: LoopNodeKeyV1,
    bindings: &BTreeMap<LoopBindingKeyV1, LoopValueKeyV1>,
) -> Result<Vec<LoopJoinPayload<V::Class>>, LoopJoinSigRejectReasonV1> {
    let mut visible = BTreeMap::new();
    let mut cursor = Some(key);
    while let Some(owner) = cursor {
        for index in 0..recipe.carrier_count() {
            let carrier = recipe
                .carrier_at(index)
                .expect("verified Recipe view has dense carrier rows");
            if carrier.owner_loop != owner {
                continue;
            }
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
            slot.insert(LoopJoinPayload {
                binding: carrier.binding,
                value,
                class: carrier.class,
            });
        }
        cursor = recipe.loop_at(owner).and_then(|node| node.parent);
    }
    Ok(visible.into_values().collect())
}

/// Historical V1 test seam. Production elaboration uses the borrowed verified
/// Recipe view directly; this wrapper preserves existing projection tests.
pub(in crate::mir::loop_recipe_contract) fn visible_payloads(
    recipe: &LoopRecipeV1,
    key: LoopNodeKeyV1,
    bindings: &BTreeMap<LoopBindingKeyV1, LoopValueKeyV1>,
) -> Result<Vec<LoopJoinPayloadV1>, LoopJoinSigRejectReasonV1> {
    visible_payloads_from_view(&LoopRecipeV1JoinView::raw(recipe), key, bindings)
}

pub(super) fn block_item<V: LoopJoinRecipeView>(
    recipe: &V,
    key: super::super::ids::LoopBlockKeyV1,
) -> super::super::ids::LoopItemKeyV1 {
    recipe
        .block_at(key)
        .and_then(|block| block.items.first().copied())
        .unwrap_or(super::super::ids::LoopItemKeyV1::new(0))
}

pub(super) fn has_only_operations<V: LoopJoinRecipeView>(
    recipe: &V,
    block: super::super::ids::LoopBlockKeyV1,
    allowed: fn(LoopJoinOperationFamily) -> bool,
) -> bool {
    let Some(block) = recipe.block_at(block) else {
        return false;
    };
    block.items.iter().all(|item_key| {
        let family = match recipe.item_at(*item_key) {
            Some(LoopJoinItemView::Operation(LoopJoinOperationView::ReadBinding { .. })) => {
                LoopJoinOperationFamily::ReadBinding
            }
            Some(LoopJoinItemView::Operation(LoopJoinOperationView::Define { family, .. })) => {
                family
            }
            Some(LoopJoinItemView::Operation(LoopJoinOperationView::WriteBinding { .. })) => {
                LoopJoinOperationFamily::WriteBinding
            }
            _ => return false,
        };
        allowed(family)
    })
}
