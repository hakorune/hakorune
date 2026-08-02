//! Structural verifier for one recursive portable Loop recipe.

use std::collections::{BTreeMap, BTreeSet};

use super::error::LoopRecipeRejectReasonV1 as Reject;
use super::ids::{
    LoopBindingKeyV1, LoopBlockKeyV1, LoopExitKeyV1, LoopItemKeyV1, LoopNodeKeyV1, LoopValueKeyV1,
};
use super::schema::{
    LoopConditionV1, LoopExitKindV1, LoopOperationV1, LoopRecipeArtifactV1, LoopRecipeItemV1,
    LoopRecipeProvenanceV1, LoopRecipeV1, LoopValueClassV1, LOOP_RECIPE_SCHEMA_VERSION_V1,
};
use super::source_binding::{
    LoopRecipeSourceClaimVerifierV1, StructurallyVerifiedLoopRecipeSourceClaimV1,
};

#[derive(Debug)]
pub(crate) struct VerifiedLoopRecipeV1(LoopRecipeV1);

impl VerifiedLoopRecipeV1 {
    pub(crate) fn as_recipe(&self) -> &LoopRecipeV1 {
        &self.0
    }

    pub(crate) fn root_loop(&self) -> LoopNodeKeyV1 {
        self.0.root_loop
    }

    pub(crate) fn into_recipe(self) -> LoopRecipeV1 {
        self.0
    }
}

/// Artifact whose recipe and source wire claim are structurally valid.
///
/// This type does not prove source existence, declared-function identity, or
/// correspondence with an AST. Only the semantic recipe is safe to expose to
/// consumers outside this contract module.
#[derive(Debug)]
pub(super) struct VerifiedLoopRecipeArtifactV1 {
    provenance: LoopRecipeProvenanceV1,
    source_binding: StructurallyVerifiedLoopRecipeSourceClaimV1,
    recipe: VerifiedLoopRecipeV1,
}

impl VerifiedLoopRecipeArtifactV1 {
    pub(super) fn provenance(&self) -> &LoopRecipeProvenanceV1 {
        &self.provenance
    }

    pub(super) fn recipe(&self) -> &VerifiedLoopRecipeV1 {
        &self.recipe
    }

    pub(super) fn source_binding(&self) -> &StructurallyVerifiedLoopRecipeSourceClaimV1 {
        &self.source_binding
    }
}

pub(crate) struct LoopRecipeVerifierV1;

impl LoopRecipeVerifierV1 {
    /// Verifies the semantic recipe and the internal shape of its source wire
    /// claim. This performs no lookup against a source owner or AST.
    pub(super) fn verify_artifact(
        artifact: LoopRecipeArtifactV1,
    ) -> Result<VerifiedLoopRecipeArtifactV1, Reject> {
        let LoopRecipeArtifactV1 {
            schema_version,
            provenance,
            source_binding,
            recipe,
        } = artifact;
        if schema_version != LOOP_RECIPE_SCHEMA_VERSION_V1 {
            return Err(Reject::UnsupportedVersion {
                found: schema_version,
            });
        }
        let recipe = Self::verify(recipe)?;
        let source_binding =
            LoopRecipeSourceClaimVerifierV1::verify(recipe.as_recipe(), source_binding)?;
        Ok(VerifiedLoopRecipeArtifactV1 {
            provenance,
            source_binding,
            recipe,
        })
    }

    /// Semantic verification intentionally has no route/family input.
    pub(crate) fn verify(recipe: LoopRecipeV1) -> Result<VerifiedLoopRecipeV1, Reject> {
        check_canonical_keys(&recipe)?;
        check_bindings_and_values(&recipe)?;
        check_loop_tree(&recipe)?;
        let definitions = check_block_tree_and_items(&recipe)?;
        check_recursive_preorder(&recipe)?;
        check_all_values_defined(&recipe, &definitions)?;
        check_carriers(&recipe, &definitions)?;
        Ok(VerifiedLoopRecipeV1(recipe))
    }
}

fn check_canonical_keys(recipe: &LoopRecipeV1) -> Result<(), Reject> {
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

fn check_bindings_and_values(recipe: &LoopRecipeV1) -> Result<(), Reject> {
    for binding in &recipe.bindings {
        if binding.label.is_empty() {
            return Err(Reject::EmptyBindingLabel { key: binding.key });
        }
    }
    let mut previous = None;
    for input in &recipe.inputs {
        if previous.is_some_and(|key: LoopValueKeyV1| key.raw() >= input.raw()) {
            return Err(Reject::NonCanonicalKeyOrder { domain: "inputs" });
        }
        value_class(recipe, *input)?;
        previous = Some(*input);
    }
    Ok(())
}

fn check_loop_tree(recipe: &LoopRecipeV1) -> Result<(), Reject> {
    if recipe.root_loop != LoopNodeKeyV1::new(0) {
        return Err(Reject::RootLoopMustBeZero);
    }
    for loop_node in &recipe.loops {
        if loop_node.key == recipe.root_loop {
            if loop_node.parent.is_some() {
                return Err(Reject::InvalidRootParent);
            }
        } else {
            let Some(parent) = loop_node.parent else {
                return Err(Reject::InvalidLoopParent {
                    loop_key: loop_node.key,
                });
            };
            if loop_at(recipe, parent).is_none() || parent.raw() >= loop_node.key.raw() {
                return Err(Reject::InvalidLoopParent {
                    loop_key: loop_node.key,
                });
            }
        }
        let Some(body) = block_at(recipe, loop_node.body) else {
            return Err(Reject::DanglingBlock {
                key: loop_node.body,
            });
        };
        if body.owner_loop != loop_node.key {
            return Err(Reject::BlockOwnerMismatch { key: body.key });
        }
        if let LoopConditionV1::Predicate { block, value } = loop_node.condition {
            let Some(condition_block) = block_at(recipe, block) else {
                return Err(Reject::DanglingBlock { key: block });
            };
            if condition_block.owner_loop != loop_node.key {
                return Err(Reject::BlockOwnerMismatch { key: block });
            }
            expect_value_class(recipe, value, LoopValueClassV1::Bool)?;
        }
    }
    Ok(())
}

fn check_block_tree_and_items(
    recipe: &LoopRecipeV1,
) -> Result<BTreeMap<LoopValueKeyV1, Option<LoopItemKeyV1>>, Reject> {
    let mut block_uses = vec![0_u8; recipe.blocks.len()];
    let mut item_uses = vec![0_u8; recipe.items.len()];
    let mut loop_uses = vec![0_u8; recipe.loops.len()];
    let mut exit_uses = vec![0_u8; recipe.exits.len()];
    let mut value_definitions = BTreeMap::new();

    for input in &recipe.inputs {
        if value_definitions.insert(*input, None).is_some() {
            return Err(Reject::DuplicateValueDefinition { key: *input });
        }
    }

    if let Some(root) = loop_uses.get_mut(recipe.root_loop.raw() as usize) {
        *root = 1;
    }
    for loop_node in &recipe.loops {
        if let LoopConditionV1::Predicate { block, .. } = loop_node.condition {
            mark_block_use(&mut block_uses, block)?;
        }
        mark_block_use(&mut block_uses, loop_node.body)?;
    }
    for block in &recipe.blocks {
        if loop_at(recipe, block.owner_loop).is_none() {
            return Err(Reject::DanglingLoop {
                key: block.owner_loop,
            });
        }
        let mut previous = None;
        for item_key in &block.items {
            if previous.is_some_and(|key: LoopItemKeyV1| key.raw() >= item_key.raw()) {
                return Err(Reject::NonCanonicalKeyOrder {
                    domain: "block_items",
                });
            }
            previous = Some(*item_key);
            let Some(row) = item_at(recipe, *item_key) else {
                return Err(Reject::DanglingItem { key: *item_key });
            };
            mark_item_use(&mut item_uses, *item_key)?;
            check_item(
                recipe,
                block,
                row,
                &mut block_uses,
                &mut loop_uses,
                &mut exit_uses,
                &mut value_definitions,
            )?;
        }
    }
    for (index, uses) in block_uses.into_iter().enumerate() {
        if uses == 0 {
            return Err(Reject::UnusedBlock {
                key: LoopBlockKeyV1::new(index as u32),
            });
        }
    }
    for (index, uses) in item_uses.into_iter().enumerate() {
        if uses == 0 {
            return Err(Reject::UnusedItem {
                key: LoopItemKeyV1::new(index as u32),
            });
        }
    }
    for (index, uses) in loop_uses.into_iter().enumerate() {
        if uses == 0 {
            return Err(Reject::DanglingLoop {
                key: LoopNodeKeyV1::new(index as u32),
            });
        }
        if uses > 1 {
            return Err(Reject::NestedLoopOwnerMismatch {
                key: LoopNodeKeyV1::new(index as u32),
            });
        }
    }
    check_exits(recipe, &exit_uses)?;
    Ok(value_definitions)
}

fn check_item(
    recipe: &LoopRecipeV1,
    block: &super::schema::LoopRecipeBlockV1,
    row: &super::schema::LoopRecipeItemRowV1,
    block_uses: &mut [u8],
    loop_uses: &mut [u8],
    exit_uses: &mut [u8],
    value_definitions: &mut BTreeMap<LoopValueKeyV1, Option<LoopItemKeyV1>>,
) -> Result<(), Reject> {
    match &row.item {
        LoopRecipeItemV1::Operation { operation } => {
            check_operation(recipe, row.key, *operation, value_definitions)
        }
        LoopRecipeItemV1::If {
            condition,
            then_block,
            else_block,
        } => {
            expect_value_class(recipe, *condition, LoopValueClassV1::Bool)?;
            for child in [Some(*then_block), *else_block].into_iter().flatten() {
                let Some(child_block) = block_at(recipe, child) else {
                    return Err(Reject::DanglingBlock { key: child });
                };
                if child.raw() <= block.key.raw() {
                    return Err(Reject::ChildBlockMustFollowParent { key: child });
                }
                if child_block.owner_loop != block.owner_loop {
                    return Err(Reject::BlockOwnerMismatch { key: child });
                }
                mark_block_use(block_uses, child)?;
            }
            Ok(())
        }
        LoopRecipeItemV1::Loop { loop_key } => {
            let Some(child) = loop_at(recipe, *loop_key) else {
                return Err(Reject::DanglingLoop { key: *loop_key });
            };
            if child.parent != Some(block.owner_loop) {
                return Err(Reject::NestedLoopOwnerMismatch { key: *loop_key });
            }
            let Some(slot) = loop_uses.get_mut(loop_key.raw() as usize) else {
                return Err(Reject::DanglingLoop { key: *loop_key });
            };
            *slot += 1;
            Ok(())
        }
        LoopRecipeItemV1::Exit { exit } => {
            let Some(exit_row) = exit_at(recipe, *exit) else {
                return Err(Reject::DanglingExit { key: *exit });
            };
            if exit_row.owner_loop != block.owner_loop {
                return Err(Reject::ExitOwnerMismatch { key: *exit });
            }
            mark_exit_use(exit_uses, *exit)
        }
    }
}

fn check_operation(
    recipe: &LoopRecipeV1,
    item_key: LoopItemKeyV1,
    operation: LoopOperationV1,
    definitions: &mut BTreeMap<LoopValueKeyV1, Option<LoopItemKeyV1>>,
) -> Result<(), Reject> {
    let mut define = |key: LoopValueKeyV1, class: LoopValueClassV1| {
        expect_value_class(recipe, key, class)?;
        if definitions.insert(key, Some(item_key)).is_some() {
            return Err(Reject::DuplicateValueDefinition { key });
        }
        Ok(())
    };
    match operation {
        LoopOperationV1::ReadBinding { binding, result } => {
            let class = binding_class(recipe, binding)?;
            define(result, class)
        }
        LoopOperationV1::ConstI64 { result, .. } => define(result, LoopValueClassV1::I64),
        LoopOperationV1::BinaryI64 {
            left,
            right,
            result,
            ..
        } => {
            expect_value_class(recipe, left, LoopValueClassV1::I64)?;
            expect_value_class(recipe, right, LoopValueClassV1::I64)?;
            define(result, LoopValueClassV1::I64)
        }
        LoopOperationV1::CompareI64 {
            left,
            right,
            result,
            ..
        } => {
            expect_value_class(recipe, left, LoopValueClassV1::I64)?;
            expect_value_class(recipe, right, LoopValueClassV1::I64)?;
            define(result, LoopValueClassV1::Bool)
        }
        LoopOperationV1::WriteBinding { binding, value } => {
            let class = binding_class(recipe, binding)?;
            expect_value_class(recipe, value, class)
        }
    }
}

fn check_all_values_defined(
    recipe: &LoopRecipeV1,
    definitions: &BTreeMap<LoopValueKeyV1, Option<LoopItemKeyV1>>,
) -> Result<(), Reject> {
    for value in &recipe.values {
        if !definitions.contains_key(&value.key) {
            return Err(Reject::UndefinedValue { key: value.key });
        }
    }
    Ok(())
}

fn check_carriers(
    recipe: &LoopRecipeV1,
    definitions: &BTreeMap<LoopValueKeyV1, Option<LoopItemKeyV1>>,
) -> Result<(), Reject> {
    let mut bindings = BTreeSet::new();
    for carrier in &recipe.carriers {
        if loop_at(recipe, carrier.owner_loop).is_none() {
            return Err(Reject::DanglingLoop {
                key: carrier.owner_loop,
            });
        }
        if !bindings.insert((carrier.owner_loop, carrier.binding)) {
            return Err(Reject::DuplicateCarrierBinding {
                loop_key: carrier.owner_loop,
                binding: carrier.binding,
            });
        }
        if binding_class(recipe, carrier.binding)? != carrier.class {
            return Err(Reject::ValueClassMismatch {
                key: carrier.entry_value,
            });
        }
        expect_value_class(recipe, carrier.entry_value, carrier.class)?;
        let definition = definitions
            .get(&carrier.entry_value)
            .ok_or(Reject::UndefinedValue {
                key: carrier.entry_value,
            })?;
        let entry_item = loop_entry_item(recipe, carrier.owner_loop);
        let available = match (*definition, entry_item) {
            (None, _) => true,
            (Some(_), None) => false,
            (Some(definition), Some(entry)) => definition.raw() < entry.raw(),
        };
        if !available {
            return Err(Reject::CarrierEntryNotAvailable { key: carrier.key });
        }
    }
    Ok(())
}

fn check_recursive_preorder(recipe: &LoopRecipeV1) -> Result<(), Reject> {
    fn visit_loop(
        recipe: &LoopRecipeV1,
        key: LoopNodeKeyV1,
        next_loop: &mut u32,
        next_block: &mut u32,
        next_item: &mut u32,
    ) -> Result<(), Reject> {
        if key.raw() != *next_loop {
            return Err(Reject::NonCanonicalKeyOrder {
                domain: "recursive_loops",
            });
        }
        *next_loop += 1;
        let loop_node = loop_at(recipe, key).ok_or(Reject::DanglingLoop { key })?;
        if let LoopConditionV1::Predicate { block, .. } = loop_node.condition {
            visit_block(recipe, block, next_loop, next_block, next_item)?;
        }
        visit_block(recipe, loop_node.body, next_loop, next_block, next_item)
    }

    fn visit_block(
        recipe: &LoopRecipeV1,
        key: LoopBlockKeyV1,
        next_loop: &mut u32,
        next_block: &mut u32,
        next_item: &mut u32,
    ) -> Result<(), Reject> {
        if key.raw() != *next_block {
            return Err(Reject::NonCanonicalKeyOrder {
                domain: "recursive_blocks",
            });
        }
        *next_block += 1;
        let block = block_at(recipe, key).ok_or(Reject::DanglingBlock { key })?;
        for item_key in &block.items {
            if item_key.raw() != *next_item {
                return Err(Reject::NonCanonicalKeyOrder {
                    domain: "recursive_items",
                });
            }
            *next_item += 1;
            let row = item_at(recipe, *item_key).ok_or(Reject::DanglingItem { key: *item_key })?;
            match row.item {
                LoopRecipeItemV1::If {
                    then_block,
                    else_block,
                    ..
                } => {
                    visit_block(recipe, then_block, next_loop, next_block, next_item)?;
                    if let Some(else_block) = else_block {
                        visit_block(recipe, else_block, next_loop, next_block, next_item)?;
                    }
                }
                LoopRecipeItemV1::Loop { loop_key } => {
                    visit_loop(recipe, loop_key, next_loop, next_block, next_item)?;
                }
                LoopRecipeItemV1::Operation { .. } | LoopRecipeItemV1::Exit { .. } => {}
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
    if next_loop != recipe.loops.len() as u32 {
        return Err(Reject::NonCanonicalKeyOrder {
            domain: "recursive_loops",
        });
    }
    if next_block != recipe.blocks.len() as u32 {
        return Err(Reject::NonCanonicalKeyOrder {
            domain: "recursive_blocks",
        });
    }
    if next_item != recipe.items.len() as u32 {
        return Err(Reject::NonCanonicalKeyOrder {
            domain: "recursive_items",
        });
    }
    Ok(())
}

fn check_exits(recipe: &LoopRecipeV1, uses: &[u8]) -> Result<(), Reject> {
    for exit in &recipe.exits {
        if loop_at(recipe, exit.owner_loop).is_none() {
            return Err(Reject::DanglingLoop {
                key: exit.owner_loop,
            });
        }
        match exit.kind {
            LoopExitKindV1::Break { target_loop } | LoopExitKindV1::Continue { target_loop } => {
                if !is_ancestor_or_self(recipe, exit.owner_loop, target_loop) {
                    return Err(Reject::ExitTargetNotAncestor { key: exit.key });
                }
            }
            LoopExitKindV1::Return { value } => {
                if let Some(value) = value {
                    value_class(recipe, value)?;
                }
            }
        }
    }
    for (index, count) in uses.iter().copied().enumerate() {
        if count == 0 {
            return Err(Reject::UnusedExit {
                key: LoopExitKeyV1::new(index as u32),
            });
        }
    }
    Ok(())
}

fn mark_block_use(uses: &mut [u8], key: LoopBlockKeyV1) -> Result<(), Reject> {
    let Some(slot) = uses.get_mut(key.raw() as usize) else {
        return Err(Reject::DanglingBlock { key });
    };
    *slot += 1;
    if *slot > 1 {
        return Err(Reject::DuplicateBlockUse { key });
    }
    Ok(())
}

fn mark_item_use(uses: &mut [u8], key: LoopItemKeyV1) -> Result<(), Reject> {
    let Some(slot) = uses.get_mut(key.raw() as usize) else {
        return Err(Reject::DanglingItem { key });
    };
    *slot += 1;
    if *slot > 1 {
        return Err(Reject::DuplicateItemUse { key });
    }
    Ok(())
}

fn mark_exit_use(uses: &mut [u8], key: LoopExitKeyV1) -> Result<(), Reject> {
    let Some(slot) = uses.get_mut(key.raw() as usize) else {
        return Err(Reject::DanglingExit { key });
    };
    *slot += 1;
    if *slot > 1 {
        return Err(Reject::DuplicateExitUse { key });
    }
    Ok(())
}

fn loop_at(recipe: &LoopRecipeV1, key: LoopNodeKeyV1) -> Option<&super::schema::LoopNodeV1> {
    recipe
        .loops
        .get(key.raw() as usize)
        .filter(|row| row.key == key)
}

fn block_at(
    recipe: &LoopRecipeV1,
    key: LoopBlockKeyV1,
) -> Option<&super::schema::LoopRecipeBlockV1> {
    recipe
        .blocks
        .get(key.raw() as usize)
        .filter(|row| row.key == key)
}

fn item_at(
    recipe: &LoopRecipeV1,
    key: LoopItemKeyV1,
) -> Option<&super::schema::LoopRecipeItemRowV1> {
    recipe
        .items
        .get(key.raw() as usize)
        .filter(|row| row.key == key)
}

fn exit_at(recipe: &LoopRecipeV1, key: LoopExitKeyV1) -> Option<&super::schema::LoopRecipeExitV1> {
    recipe
        .exits
        .get(key.raw() as usize)
        .filter(|row| row.key == key)
}

fn loop_entry_item(recipe: &LoopRecipeV1, key: LoopNodeKeyV1) -> Option<LoopItemKeyV1> {
    if key == recipe.root_loop {
        return None;
    }
    recipe.items.iter().find_map(|row| match row.item {
        LoopRecipeItemV1::Loop { loop_key } if loop_key == key => Some(row.key),
        _ => None,
    })
}

fn binding_class(recipe: &LoopRecipeV1, key: LoopBindingKeyV1) -> Result<LoopValueClassV1, Reject> {
    recipe
        .bindings
        .get(key.raw() as usize)
        .filter(|row| row.key == key)
        .map(|row| row.class)
        .ok_or(Reject::DanglingBinding { key })
}

fn value_class(recipe: &LoopRecipeV1, key: LoopValueKeyV1) -> Result<LoopValueClassV1, Reject> {
    recipe
        .values
        .get(key.raw() as usize)
        .filter(|row| row.key == key)
        .map(|row| row.class)
        .ok_or(Reject::DanglingValue { key })
}

fn expect_value_class(
    recipe: &LoopRecipeV1,
    key: LoopValueKeyV1,
    expected: LoopValueClassV1,
) -> Result<(), Reject> {
    if value_class(recipe, key)? != expected {
        return Err(Reject::ValueClassMismatch { key });
    }
    Ok(())
}

fn is_ancestor_or_self(recipe: &LoopRecipeV1, owner: LoopNodeKeyV1, target: LoopNodeKeyV1) -> bool {
    let mut cursor = Some(owner);
    while let Some(key) = cursor {
        if key == target {
            return true;
        }
        cursor = loop_at(recipe, key).and_then(|row| row.parent);
    }
    false
}
