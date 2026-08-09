//! Structural verification for the explicit V2 typed Loop wire.
//!
//! This verifier owns no source lookup and no physical lowering.  It checks
//! only the self-contained logical wire before a resolver/source-bound row is
//! allowed to consume it.

use std::collections::{BTreeMap, BTreeSet};

use super::ids::{
    LoopBindingKeyV1, LoopBlockKeyV1, LoopCarrierKeyV1, LoopExitKeyV1, LoopItemKeyV1,
    LoopNodeKeyV1, LoopValueKeyV1,
};
use super::schema::LoopRecipeSourceBindingV1;
use super::schema_v2::{
    LoopConditionV2, LoopExitKindV2, LoopOperationV2, LoopRecipeArtifactV2, LoopRecipeItemV2,
    LoopRecipeV2, LoopValueClassV2, LOOP_RECIPE_SCHEMA_VERSION_V2,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum LoopRecipeV2RejectReason {
    UnsupportedVersion {
        found: u16,
    },
    NonCanonicalKeyOrder {
        domain: &'static str,
        expected: u32,
        found: u32,
    },
    EmptyBindingLabel {
        key: LoopBindingKeyV1,
    },
    DuplicateInput {
        key: LoopValueKeyV1,
    },
    UnknownBinding {
        key: LoopBindingKeyV1,
    },
    UnknownValue {
        key: LoopValueKeyV1,
    },
    UnknownBlock {
        key: LoopBlockKeyV1,
    },
    UnknownLoop {
        key: LoopNodeKeyV1,
    },
    UnknownItem {
        key: LoopItemKeyV1,
    },
    UnknownCarrier {
        key: LoopCarrierKeyV1,
    },
    UnknownExit {
        key: LoopExitKeyV1,
    },
    DuplicateValueDefinition {
        key: LoopValueKeyV1,
    },
    UndefinedValue {
        key: LoopValueKeyV1,
    },
    ValueUsedBeforeDefinition {
        item: LoopItemKeyV1,
        key: LoopValueKeyV1,
    },
    ValueClassMismatch {
        key: LoopValueKeyV1,
    },
    TextEqOperandClassMismatch {
        item: LoopItemKeyV1,
    },
    TextEqResultClassMismatch {
        item: LoopItemKeyV1,
    },
    InvalidOperationDomain {
        item: LoopItemKeyV1,
    },
    InvalidLoopCondition {
        loop_key: LoopNodeKeyV1,
    },
    InvalidCarrierClass {
        key: LoopCarrierKeyV1,
    },
    InvalidCarrierBinding {
        key: LoopCarrierKeyV1,
    },
    DuplicateItemUse {
        key: LoopItemKeyV1,
    },
    DuplicateCarrierBinding {
        loop_key: LoopNodeKeyV1,
        binding: LoopBindingKeyV1,
    },
    DuplicateExitUse {
        key: LoopExitKeyV1,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct VerifiedLoopRecipeV2(LoopRecipeV2);

impl VerifiedLoopRecipeV2 {
    pub(crate) fn as_recipe(&self) -> &LoopRecipeV2 {
        &self.0
    }

    pub(crate) fn into_recipe(self) -> LoopRecipeV2 {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct VerifiedLoopRecipeArtifactV2 {
    provenance: super::schema::LoopRecipeProvenanceV1,
    source_binding: LoopRecipeSourceBindingV1,
    recipe: VerifiedLoopRecipeV2,
}

impl VerifiedLoopRecipeArtifactV2 {
    pub(crate) fn recipe(&self) -> &VerifiedLoopRecipeV2 {
        &self.recipe
    }

    pub(crate) fn provenance(&self) -> &super::schema::LoopRecipeProvenanceV1 {
        &self.provenance
    }

    pub(crate) fn source_binding(&self) -> &LoopRecipeSourceBindingV1 {
        &self.source_binding
    }
}

pub(crate) struct LoopRecipeVerifierV2;

impl LoopRecipeVerifierV2 {
    pub(crate) fn verify_artifact(
        artifact: LoopRecipeArtifactV2,
    ) -> Result<VerifiedLoopRecipeArtifactV2, LoopRecipeV2RejectReason> {
        if artifact.schema_version != LOOP_RECIPE_SCHEMA_VERSION_V2 {
            return Err(LoopRecipeV2RejectReason::UnsupportedVersion {
                found: artifact.schema_version,
            });
        }
        let recipe = Self::verify(artifact.recipe)?;
        Ok(VerifiedLoopRecipeArtifactV2 {
            provenance: artifact.provenance,
            source_binding: artifact.source_binding,
            recipe,
        })
    }

    pub(crate) fn verify(
        recipe: LoopRecipeV2,
    ) -> Result<VerifiedLoopRecipeV2, LoopRecipeV2RejectReason> {
        check_canonical_keys(&recipe)?;
        let bindings = binding_classes(&recipe)?;
        let values = value_classes(&recipe)?;
        check_inputs(&recipe, &values)?;
        check_loops(&recipe, &values)?;
        check_carriers(&recipe, &bindings, &values)?;
        check_exits(&recipe)?;
        check_blocks_and_items(&recipe, &bindings, &values)?;
        Ok(VerifiedLoopRecipeV2(recipe))
    }
}

fn check_canonical_keys(recipe: &LoopRecipeV2) -> Result<(), LoopRecipeV2RejectReason> {
    if recipe.root_loop != LoopNodeKeyV1::new(0) {
        return Err(LoopRecipeV2RejectReason::NonCanonicalKeyOrder {
            domain: "root_loop",
            expected: 0,
            found: recipe.root_loop.raw(),
        });
    }
    for (index, node) in recipe.loops.iter().enumerate() {
        expect_key("loops", index as u32, node.key.raw())?;
    }
    for (index, block) in recipe.blocks.iter().enumerate() {
        expect_key("blocks", index as u32, block.key.raw())?;
    }
    for (index, item) in recipe.items.iter().enumerate() {
        expect_key("items", index as u32, item.key.raw())?;
    }
    for (index, binding) in recipe.bindings.iter().enumerate() {
        expect_key("bindings", index as u32, binding.key.raw())?;
    }
    for (index, value) in recipe.values.iter().enumerate() {
        expect_key("values", index as u32, value.key.raw())?;
    }
    for (index, carrier) in recipe.carriers.iter().enumerate() {
        expect_key("carriers", index as u32, carrier.key.raw())?;
    }
    for (index, exit) in recipe.exits.iter().enumerate() {
        expect_key("exits", index as u32, exit.key.raw())?;
    }
    Ok(())
}

fn expect_key(
    domain: &'static str,
    expected: u32,
    found: u32,
) -> Result<(), LoopRecipeV2RejectReason> {
    if expected != found {
        return Err(LoopRecipeV2RejectReason::NonCanonicalKeyOrder {
            domain,
            expected,
            found,
        });
    }
    Ok(())
}

fn binding_classes(
    recipe: &LoopRecipeV2,
) -> Result<BTreeMap<LoopBindingKeyV1, LoopValueClassV2>, LoopRecipeV2RejectReason> {
    let mut classes = BTreeMap::new();
    for binding in &recipe.bindings {
        if binding.label.is_empty() {
            return Err(LoopRecipeV2RejectReason::EmptyBindingLabel { key: binding.key });
        }
        classes.insert(binding.key, binding.class);
    }
    Ok(classes)
}

fn value_classes(
    recipe: &LoopRecipeV2,
) -> Result<BTreeMap<LoopValueKeyV1, LoopValueClassV2>, LoopRecipeV2RejectReason> {
    Ok(recipe
        .values
        .iter()
        .map(|value| (value.key, value.class))
        .collect())
}

fn check_inputs(
    recipe: &LoopRecipeV2,
    values: &BTreeMap<LoopValueKeyV1, LoopValueClassV2>,
) -> Result<(), LoopRecipeV2RejectReason> {
    let mut previous = None;
    for input in &recipe.inputs {
        if previous.is_some_and(|key: LoopValueKeyV1| key.raw() >= input.raw()) {
            return Err(LoopRecipeV2RejectReason::DuplicateInput { key: *input });
        }
        if !values.contains_key(input) {
            return Err(LoopRecipeV2RejectReason::UnknownValue { key: *input });
        }
        previous = Some(*input);
    }
    Ok(())
}

fn check_loops(
    recipe: &LoopRecipeV2,
    values: &BTreeMap<LoopValueKeyV1, LoopValueClassV2>,
) -> Result<(), LoopRecipeV2RejectReason> {
    let loops: BTreeSet<_> = recipe.loops.iter().map(|node| node.key).collect();
    let blocks: BTreeSet<_> = recipe.blocks.iter().map(|block| block.key).collect();
    for node in &recipe.loops {
        if node.parent.is_some_and(|parent| !loops.contains(&parent))
            || !blocks.contains(&node.body)
        {
            return Err(LoopRecipeV2RejectReason::UnknownLoop { key: node.key });
        }
        if let LoopConditionV2::Predicate { block, value } = node.condition {
            if !blocks.contains(&block) || values.get(&value) != Some(&LoopValueClassV2::Bool) {
                return Err(LoopRecipeV2RejectReason::InvalidLoopCondition { loop_key: node.key });
            }
        }
    }
    Ok(())
}

fn check_carriers(
    recipe: &LoopRecipeV2,
    bindings: &BTreeMap<LoopBindingKeyV1, LoopValueClassV2>,
    values: &BTreeMap<LoopValueKeyV1, LoopValueClassV2>,
) -> Result<(), LoopRecipeV2RejectReason> {
    let loops: BTreeSet<_> = recipe.loops.iter().map(|node| node.key).collect();
    let mut seen = BTreeSet::new();
    for carrier in &recipe.carriers {
        if !loops.contains(&carrier.owner_loop) {
            return Err(LoopRecipeV2RejectReason::UnknownLoop {
                key: carrier.owner_loop,
            });
        }
        if !seen.insert((carrier.owner_loop, carrier.binding)) {
            return Err(LoopRecipeV2RejectReason::DuplicateCarrierBinding {
                loop_key: carrier.owner_loop,
                binding: carrier.binding,
            });
        }
        if bindings.get(&carrier.binding) != Some(&carrier.class) {
            return Err(LoopRecipeV2RejectReason::InvalidCarrierBinding { key: carrier.key });
        }
        if values.get(&carrier.entry_value) != Some(&carrier.class) {
            return Err(LoopRecipeV2RejectReason::InvalidCarrierClass { key: carrier.key });
        }
    }
    Ok(())
}

fn check_exits(recipe: &LoopRecipeV2) -> Result<(), LoopRecipeV2RejectReason> {
    let loops: BTreeSet<_> = recipe.loops.iter().map(|node| node.key).collect();
    let mut seen = BTreeSet::new();
    for exit in &recipe.exits {
        if !loops.contains(&exit.owner_loop) {
            return Err(LoopRecipeV2RejectReason::UnknownLoop {
                key: exit.owner_loop,
            });
        }
        if !seen.insert(exit.key) {
            return Err(LoopRecipeV2RejectReason::DuplicateExitUse { key: exit.key });
        }
        let target = match exit.kind {
            LoopExitKindV2::Break { target_loop } | LoopExitKindV2::Continue { target_loop } => {
                Some(target_loop)
            }
            LoopExitKindV2::Return { .. } => None,
        };
        if target.is_some_and(|target| !loops.contains(&target)) {
            return Err(LoopRecipeV2RejectReason::UnknownLoop {
                key: exit.owner_loop,
            });
        }
    }
    Ok(())
}

fn check_blocks_and_items(
    recipe: &LoopRecipeV2,
    bindings: &BTreeMap<LoopBindingKeyV1, LoopValueClassV2>,
    values: &BTreeMap<LoopValueKeyV1, LoopValueClassV2>,
) -> Result<(), LoopRecipeV2RejectReason> {
    let loops: BTreeSet<_> = recipe.loops.iter().map(|node| node.key).collect();
    let blocks: BTreeSet<_> = recipe.blocks.iter().map(|block| block.key).collect();
    let items: BTreeMap<_, _> = recipe
        .items
        .iter()
        .map(|row| (row.key, &row.item))
        .collect();
    let exits: BTreeMap<_, _> = recipe
        .exits
        .iter()
        .map(|exit| (exit.key, exit.kind))
        .collect();
    let mut uses = BTreeSet::new();
    let mut definitions = recipe
        .inputs
        .iter()
        .copied()
        .map(|key| (key, None))
        .collect::<BTreeMap<_, Option<LoopItemKeyV1>>>();

    for block in &recipe.blocks {
        if !loops.contains(&block.owner_loop) {
            return Err(LoopRecipeV2RejectReason::UnknownLoop {
                key: block.owner_loop,
            });
        }
        let mut previous = None;
        for item_key in &block.items {
            if previous.is_some_and(|key: LoopItemKeyV1| key.raw() >= item_key.raw()) {
                return Err(LoopRecipeV2RejectReason::NonCanonicalKeyOrder {
                    domain: "block_items",
                    expected: previous.map_or(0, |key| key.raw() + 1),
                    found: item_key.raw(),
                });
            }
            previous = Some(*item_key);
            if !uses.insert(*item_key) {
                return Err(LoopRecipeV2RejectReason::DuplicateItemUse { key: *item_key });
            }
            let Some(item) = items.get(item_key) else {
                return Err(LoopRecipeV2RejectReason::UnknownItem { key: *item_key });
            };
            check_item(
                item_key,
                item,
                &blocks,
                &loops,
                &exits,
                bindings,
                values,
                &mut definitions,
            )?;
        }
    }

    for row in &recipe.items {
        if !uses.contains(&row.key) {
            return Err(LoopRecipeV2RejectReason::UnknownItem { key: row.key });
        }
    }
    for value in &recipe.values {
        if !definitions.contains_key(&value.key) {
            return Err(LoopRecipeV2RejectReason::UndefinedValue { key: value.key });
        }
    }
    Ok(())
}

fn check_item(
    item_key: &LoopItemKeyV1,
    item: &LoopRecipeItemV2,
    blocks: &BTreeSet<LoopBlockKeyV1>,
    loops: &BTreeSet<LoopNodeKeyV1>,
    exits: &BTreeMap<LoopExitKeyV1, LoopExitKindV2>,
    bindings: &BTreeMap<LoopBindingKeyV1, LoopValueClassV2>,
    values: &BTreeMap<LoopValueKeyV1, LoopValueClassV2>,
    definitions: &mut BTreeMap<LoopValueKeyV1, Option<LoopItemKeyV1>>,
) -> Result<(), LoopRecipeV2RejectReason> {
    match item {
        LoopRecipeItemV2::Operation { operation } => {
            check_operation(item_key, operation, bindings, values, definitions)
        }
        LoopRecipeItemV2::If {
            condition,
            then_block,
            else_block,
        } => {
            if expect_defined_value(*item_key, *condition, values, definitions)?
                != LoopValueClassV2::Bool
                || !blocks.contains(then_block)
                || else_block.is_some_and(|block| !blocks.contains(&block))
            {
                return Err(LoopRecipeV2RejectReason::InvalidOperationDomain { item: *item_key });
            }
            Ok(())
        }
        LoopRecipeItemV2::Loop { loop_key } => {
            if !loops.contains(loop_key) {
                return Err(LoopRecipeV2RejectReason::UnknownLoop { key: *loop_key });
            }
            Ok(())
        }
        LoopRecipeItemV2::Exit { exit } => {
            let Some(kind) = exits.get(exit) else {
                return Err(LoopRecipeV2RejectReason::UnknownExit { key: *exit });
            };
            if let LoopExitKindV2::Return { value: Some(value) } = kind {
                expect_defined_value(*item_key, *value, values, definitions)?;
            }
            Ok(())
        }
    }
}

fn check_operation(
    item_key: &LoopItemKeyV1,
    operation: &LoopOperationV2,
    bindings: &BTreeMap<LoopBindingKeyV1, LoopValueClassV2>,
    values: &BTreeMap<LoopValueKeyV1, LoopValueClassV2>,
    definitions: &mut BTreeMap<LoopValueKeyV1, Option<LoopItemKeyV1>>,
) -> Result<(), LoopRecipeV2RejectReason> {
    match operation {
        LoopOperationV2::ReadBinding { binding, result } => {
            let Some(class) = bindings.get(binding) else {
                return Err(LoopRecipeV2RejectReason::UnknownBinding { key: *binding });
            };
            define_value(*item_key, *result, *class, values, definitions)
        }
        LoopOperationV2::ConstI64 { result, .. } => define_value(
            *item_key,
            *result,
            LoopValueClassV2::I64,
            values,
            definitions,
        ),
        LoopOperationV2::BinaryI64 {
            left,
            right,
            result,
            ..
        } => {
            expect_defined_class(*item_key, *left, LoopValueClassV2::I64, values, definitions)?;
            expect_defined_class(
                *item_key,
                *right,
                LoopValueClassV2::I64,
                values,
                definitions,
            )?;
            define_value(
                *item_key,
                *result,
                LoopValueClassV2::I64,
                values,
                definitions,
            )
        }
        LoopOperationV2::CompareI64 {
            left,
            right,
            result,
            ..
        } => {
            expect_defined_class(*item_key, *left, LoopValueClassV2::I64, values, definitions)?;
            expect_defined_class(
                *item_key,
                *right,
                LoopValueClassV2::I64,
                values,
                definitions,
            )?;
            define_value(
                *item_key,
                *result,
                LoopValueClassV2::Bool,
                values,
                definitions,
            )
        }
        LoopOperationV2::WriteBinding { binding, value } => {
            let Some(class) = bindings.get(binding) else {
                return Err(LoopRecipeV2RejectReason::UnknownBinding { key: *binding });
            };
            if expect_defined_value(*item_key, *value, values, definitions)? != *class {
                return Err(LoopRecipeV2RejectReason::ValueClassMismatch { key: *value });
            }
            Ok(())
        }
        LoopOperationV2::CallSlot {
            receiver,
            args,
            result,
        } => {
            if let Some(key) = receiver {
                expect_defined_value(*item_key, *key, values, definitions)?;
            }
            for key in args {
                expect_defined_value(*item_key, *key, values, definitions)?;
            }
            if let Some(result) = result {
                let class = *values
                    .get(result)
                    .ok_or(LoopRecipeV2RejectReason::UnknownValue { key: *result })?;
                define_value(*item_key, *result, class, values, definitions)
            } else {
                Ok(())
            }
        }
        LoopOperationV2::TextEq {
            left,
            right,
            result,
        } => {
            if expect_defined_value(*item_key, *left, values, definitions)?
                != LoopValueClassV2::Text
                || expect_defined_value(*item_key, *right, values, definitions)?
                    != LoopValueClassV2::Text
            {
                return Err(LoopRecipeV2RejectReason::TextEqOperandClassMismatch {
                    item: *item_key,
                });
            }
            if values.get(result) != Some(&LoopValueClassV2::Bool) {
                return Err(LoopRecipeV2RejectReason::TextEqResultClassMismatch {
                    item: *item_key,
                });
            }
            define_value(
                *item_key,
                *result,
                LoopValueClassV2::Bool,
                values,
                definitions,
            )
        }
    }
}

fn expect_defined_value(
    item: LoopItemKeyV1,
    key: LoopValueKeyV1,
    values: &BTreeMap<LoopValueKeyV1, LoopValueClassV2>,
    definitions: &BTreeMap<LoopValueKeyV1, Option<LoopItemKeyV1>>,
) -> Result<LoopValueClassV2, LoopRecipeV2RejectReason> {
    let class = values
        .get(&key)
        .copied()
        .ok_or(LoopRecipeV2RejectReason::UnknownValue { key })?;
    if !definitions.contains_key(&key) {
        return Err(LoopRecipeV2RejectReason::ValueUsedBeforeDefinition { item, key });
    }
    Ok(class)
}

fn expect_defined_class(
    item: LoopItemKeyV1,
    key: LoopValueKeyV1,
    expected: LoopValueClassV2,
    values: &BTreeMap<LoopValueKeyV1, LoopValueClassV2>,
    definitions: &BTreeMap<LoopValueKeyV1, Option<LoopItemKeyV1>>,
) -> Result<(), LoopRecipeV2RejectReason> {
    if expect_defined_value(item, key, values, definitions)? == expected {
        Ok(())
    } else {
        Err(LoopRecipeV2RejectReason::InvalidOperationDomain { item })
    }
}

fn define_value(
    item: LoopItemKeyV1,
    key: LoopValueKeyV1,
    class: LoopValueClassV2,
    values: &BTreeMap<LoopValueKeyV1, LoopValueClassV2>,
    definitions: &mut BTreeMap<LoopValueKeyV1, Option<LoopItemKeyV1>>,
) -> Result<(), LoopRecipeV2RejectReason> {
    if values.get(&key) != Some(&class) {
        return Err(LoopRecipeV2RejectReason::ValueClassMismatch { key });
    }
    if definitions.insert(key, Some(item)).is_some() {
        return Err(LoopRecipeV2RejectReason::DuplicateValueDefinition { key });
    }
    Ok(())
}
