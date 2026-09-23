//! Draft Recipe assembler with canonical key allocation and source-anchor
//! co-recording.
//!
//! A producer emits the admitted source program in canonical preorder and the
//! draft assigns every loop/block/item/binding/value/carrier/exit key itself.
//! Each pushed item carries its source anchor once, so binding, effect,
//! initialized-input, and operation-evidence relations fall out of the same
//! construction instead of a second hand-written table.
//!
//! The draft is an assembler, not a verifier: `LoopRecipeVerifierV1` remains
//! the sole semantic authority over the assembled `LoopRecipeV1`, and this
//! module owns no AST, resolver ledger, route, or physical policy.

use std::collections::BTreeMap;

use crate::mir::resolved_semantics::{
    BindingOriginV1, BindingRefV1, FunctionOwnerIdV1, OwnedExprSiteV1, SourceBindingSiteV1,
    SourceExprSiteV1, SourceStmtSiteV1,
};

use super::ids::{
    LoopBindingKeyV1, LoopBlockKeyV1, LoopCarrierKeyV1, LoopExitKeyV1, LoopItemKeyV1,
    LoopNodeKeyV1, LoopValueKeyV1,
};
use super::input_source::LoopInitializedLocalInputSourceRelationV1;
use super::operation_effect::LoopOperationSourceEvidenceV1;
use super::schema::{
    LoopBinaryI64OpV1, LoopCompareI64OpV1, LoopConditionV1, LoopExitKindV1, LoopNodeV1,
    LoopOperationV1, LoopRecipeBindingV1, LoopRecipeBlockV1, LoopRecipeCarrierV1,
    LoopRecipeExitV1, LoopRecipeItemRowV1, LoopRecipeItemV1, LoopRecipeV1, LoopRecipeValueV1,
    LoopValueClassV1,
};
use super::source_bound_core::{
    LoopBindingEffectAnchorV1, LoopBindingEffectRelationV1, LoopBindingEffectRoleV1,
    LoopRecipeBindingRelationV1,
};

#[derive(Debug)]
struct DraftLoopV1 {
    parent: Option<LoopNodeKeyV1>,
    source: SourceStmtSiteV1,
    condition: Option<LoopConditionV1>,
    body: Option<LoopBlockKeyV1>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LoopRecipeDraftRejectV1 {
    RootLoopReopened,
    MissingLoopBody { loop_key: LoopNodeKeyV1 },
    MissingLoopCondition { loop_key: LoopNodeKeyV1 },
    ConditionReopened { loop_key: LoopNodeKeyV1 },
    UnknownLoop { loop_key: LoopNodeKeyV1 },
    UnknownBlock { block: LoopBlockKeyV1 },
    UnknownItem { item: LoopItemKeyV1 },
    NotAnIfItem { item: LoopItemKeyV1 },
    ElseBlockReopened { item: LoopItemKeyV1 },
    UnknownBinding { binding: LoopBindingKeyV1 },
    UnknownValue { value: LoopValueKeyV1 },
    UnknownExit { exit: LoopExitKeyV1 },
    ExitOwnerMismatch { exit: LoopExitKeyV1 },
}

/// One assembled recipe plus the relations recorded while it was built.
///
/// The recipe remains unverified; only `LoopRecipeVerifierV1` may seal it.
/// The relation rows are unsealed DTOs for the existing Core/input/evidence
/// issuers, which re-verify them against the sealed Core.
#[derive(Debug)]
pub(crate) struct LoopRecipeDraftProductV1 {
    recipe: LoopRecipeV1,
    bindings: Vec<LoopRecipeBindingRelationV1>,
    effects: Vec<LoopBindingEffectRelationV1>,
    inputs: Vec<LoopInitializedLocalInputSourceRelationV1>,
    evidence: Vec<LoopOperationSourceEvidenceV1>,
}

impl LoopRecipeDraftProductV1 {
    pub(crate) fn recipe(&self) -> &LoopRecipeV1 {
        &self.recipe
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        LoopRecipeV1,
        Vec<LoopRecipeBindingRelationV1>,
        Vec<LoopBindingEffectRelationV1>,
        Vec<LoopInitializedLocalInputSourceRelationV1>,
        Vec<LoopOperationSourceEvidenceV1>,
    ) {
        (
            self.recipe,
            self.bindings,
            self.effects,
            self.inputs,
            self.evidence,
        )
    }
}

pub(crate) struct LoopRecipeDraftV1 {
    owner: FunctionOwnerIdV1,
    loops: Vec<DraftLoopV1>,
    blocks: Vec<LoopRecipeBlockV1>,
    items: Vec<LoopRecipeItemRowV1>,
    item_blocks: Vec<LoopBlockKeyV1>,
    bindings: Vec<LoopRecipeBindingV1>,
    binding_sources: Vec<(BindingRefV1, SourceBindingSiteV1)>,
    binding_relations: Vec<LoopRecipeBindingRelationV1>,
    values: Vec<LoopRecipeValueV1>,
    inputs: Vec<LoopValueKeyV1>,
    carriers: Vec<LoopRecipeCarrierV1>,
    exits: Vec<LoopRecipeExitV1>,
    effect_relations: Vec<LoopBindingEffectRelationV1>,
    input_relations: Vec<LoopInitializedLocalInputSourceRelationV1>,
    evidence: Vec<LoopOperationSourceEvidenceV1>,
    read_ordinals: BTreeMap<LoopBindingKeyV1, u32>,
    write_ordinals: BTreeMap<LoopBindingKeyV1, u32>,
}

impl LoopRecipeDraftV1 {
    pub(crate) fn new(owner: FunctionOwnerIdV1) -> Self {
        Self {
            owner,
            loops: Vec::new(),
            blocks: Vec::new(),
            items: Vec::new(),
            item_blocks: Vec::new(),
            bindings: Vec::new(),
            binding_sources: Vec::new(),
            binding_relations: Vec::new(),
            values: Vec::new(),
            inputs: Vec::new(),
            carriers: Vec::new(),
            exits: Vec::new(),
            effect_relations: Vec::new(),
            input_relations: Vec::new(),
            evidence: Vec::new(),
            read_ordinals: BTreeMap::new(),
            write_ordinals: BTreeMap::new(),
        }
    }

    /// Declare one source-local binding.  The binding relation and its source
    /// declaration are recorded at the same time so no later table rebuilds
    /// the correspondence.
    pub(crate) fn declare_local_binding(
        &mut self,
        label: impl Into<String>,
        class: LoopValueClassV1,
        source_binding: BindingRefV1,
        declaration: SourceBindingSiteV1,
    ) -> LoopBindingKeyV1 {
        let key = LoopBindingKeyV1::new(self.bindings.len() as u32);
        self.bindings.push(LoopRecipeBindingV1 {
            key,
            label: label.into(),
            class,
        });
        self.binding_sources.push((source_binding, declaration.clone()));
        self.binding_relations.push(LoopRecipeBindingRelationV1::new(
            key,
            source_binding,
            class,
            BindingOriginV1::Source(declaration),
        ));
        key
    }

    /// Open the single root loop.  Canonical order requires it to be the
    /// first loop emitted; the verifier re-checks that ordering.
    pub(crate) fn open_root_loop(
        &mut self,
        source: SourceStmtSiteV1,
    ) -> Result<LoopNodeKeyV1, LoopRecipeDraftRejectV1> {
        if !self.loops.is_empty() {
            return Err(LoopRecipeDraftRejectV1::RootLoopReopened);
        }
        self.open_loop(None, source)
    }

    /// Open a nested loop.  The loop key is allocated at the position where
    /// the parent emits it, so callers must open a child only through
    /// `push_loop_item` to preserve canonical preorder.
    fn open_loop(
        &mut self,
        parent: Option<LoopNodeKeyV1>,
        source: SourceStmtSiteV1,
    ) -> Result<LoopNodeKeyV1, LoopRecipeDraftRejectV1> {
        let key = LoopNodeKeyV1::new(self.loops.len() as u32);
        self.loops.push(DraftLoopV1 {
            parent,
            source,
            condition: None,
            body: None,
        });
        Ok(key)
    }

    /// Emit one nested-loop item and open its child loop atomically.  Fusing
    /// the two allocations keeps the loop key equal to its canonical
    /// preorder visit position.
    pub(crate) fn push_loop_item(
        &mut self,
        block: LoopBlockKeyV1,
        source: SourceStmtSiteV1,
    ) -> Result<LoopNodeKeyV1, LoopRecipeDraftRejectV1> {
        let owner_loop = self.block_owner(block)?;
        let loop_key = self.open_loop(Some(owner_loop), source)?;
        let item = self.push_item(block, LoopRecipeItemV1::Loop { loop_key })?;
        debug_assert_eq!(item.raw() as usize + 1, self.items.len());
        Ok(loop_key)
    }

    /// Open the predicate block of a loop condition.  `seal_condition` fixes
    /// the resulting predicate value once it is emitted.
    pub(crate) fn open_condition_block(
        &mut self,
        loop_key: LoopNodeKeyV1,
    ) -> Result<LoopBlockKeyV1, LoopRecipeDraftRejectV1> {
        self.loop_row(loop_key)?;
        Ok(self.open_block(loop_key))
    }

    pub(crate) fn seal_condition(
        &mut self,
        loop_key: LoopNodeKeyV1,
        block: LoopBlockKeyV1,
        value: LoopValueKeyV1,
    ) -> Result<(), LoopRecipeDraftRejectV1> {
        self.value_row(value)?;
        let row = self.loop_row(loop_key)?;
        if row.condition.is_some() {
            return Err(LoopRecipeDraftRejectV1::ConditionReopened { loop_key });
        }
        self.loops[loop_key.raw() as usize].condition =
            Some(LoopConditionV1::Predicate { block, value });
        Ok(())
    }

    pub(crate) fn seal_condition_always(
        &mut self,
        loop_key: LoopNodeKeyV1,
    ) -> Result<(), LoopRecipeDraftRejectV1> {
        let row = self.loop_row(loop_key)?;
        if row.condition.is_some() {
            return Err(LoopRecipeDraftRejectV1::ConditionReopened { loop_key });
        }
        self.loops[loop_key.raw() as usize].condition = Some(LoopConditionV1::Always);
        Ok(())
    }

    pub(crate) fn open_body_block(
        &mut self,
        loop_key: LoopNodeKeyV1,
    ) -> Result<LoopBlockKeyV1, LoopRecipeDraftRejectV1> {
        self.loop_row(loop_key)?;
        let block = self.open_block(loop_key);
        self.loops[loop_key.raw() as usize].body = Some(block);
        Ok(block)
    }

    /// Declare one loop carrier whose entry value is an initialized input.
    /// The carrier, its DerivedCarrierEntry effect, and the initialized-local
    /// input relation are issued together from the same call.
    pub(crate) fn carrier_input(
        &mut self,
        owner_loop: LoopNodeKeyV1,
        binding: LoopBindingKeyV1,
        initializer: SourceExprSiteV1,
    ) -> Result<LoopValueKeyV1, LoopRecipeDraftRejectV1> {
        let class = self.binding_class(binding)?;
        let loop_source = self.loop_row(owner_loop)?.source.clone();
        let value = self.fresh_value(class);
        self.inputs.push(value);
        let carrier = LoopCarrierKeyV1::new(self.carriers.len() as u32);
        self.carriers.push(LoopRecipeCarrierV1 {
            key: carrier,
            owner_loop,
            binding,
            class,
            entry_value: value,
        });
        let (source_binding, declaration) = self.binding_sources[binding.raw() as usize].clone();
        self.effect_relations.push(LoopBindingEffectRelationV1::new(
            LoopBindingEffectRoleV1::DerivedCarrierEntry,
            binding,
            source_binding,
            class,
            LoopBindingEffectAnchorV1::DerivedCarrierEntry {
                owner: self.owner,
                source_loop: loop_source,
                carrier,
            },
        ));
        self.input_relations
            .push(LoopInitializedLocalInputSourceRelationV1::new(
                declaration,
                initializer,
                source_binding,
                value,
                class,
            ));
        Ok(value)
    }

    pub(crate) fn declare_exit(
        &mut self,
        owner_loop: LoopNodeKeyV1,
        kind: LoopExitKindV1,
    ) -> Result<LoopExitKeyV1, LoopRecipeDraftRejectV1> {
        self.loop_row(owner_loop)?;
        let key = LoopExitKeyV1::new(self.exits.len() as u32);
        self.exits.push(LoopRecipeExitV1 {
            key,
            owner_loop,
            kind,
        });
        Ok(key)
    }

    /// Push an `if` item and open its then block.  The then block key is
    /// allocated now so its items receive the canonical keys that follow the
    /// `if` item in preorder.  An else block is attached later through
    /// `open_else_block` so its key always follows the complete then subtree.
    pub(crate) fn push_if(
        &mut self,
        block: LoopBlockKeyV1,
        condition: LoopValueKeyV1,
    ) -> Result<(LoopItemKeyV1, LoopBlockKeyV1), LoopRecipeDraftRejectV1> {
        self.value_row(condition)?;
        let owner_loop = self.block_owner(block)?;
        let then_block = self.open_block(owner_loop);
        let item = self.push_item(
            block,
            LoopRecipeItemV1::If {
                condition,
                then_block,
                else_block: None,
            },
        )?;
        Ok((item, then_block))
    }

    /// Open the else block of an already pushed `if` item.  Call this only
    /// after the then subtree is fully emitted; canonical preorder visits the
    /// else block after every nested block of the then arm.
    pub(crate) fn open_else_block(
        &mut self,
        if_item: LoopItemKeyV1,
    ) -> Result<LoopBlockKeyV1, LoopRecipeDraftRejectV1> {
        let parent_block = self
            .item_blocks
            .get(if_item.raw() as usize)
            .copied()
            .ok_or(LoopRecipeDraftRejectV1::UnknownItem { item: if_item })?;
        let owner_loop = self.block_owner(parent_block)?;
        match self
            .items
            .get(if_item.raw() as usize)
            .filter(|row| row.key == if_item)
            .map(|row| &row.item)
        {
            Some(LoopRecipeItemV1::If {
                else_block: None, ..
            }) => {}
            Some(LoopRecipeItemV1::If { .. }) => {
                return Err(LoopRecipeDraftRejectV1::ElseBlockReopened { item: if_item })
            }
            Some(_) => return Err(LoopRecipeDraftRejectV1::NotAnIfItem { item: if_item }),
            None => return Err(LoopRecipeDraftRejectV1::UnknownItem { item: if_item }),
        }
        let block = self.open_block(owner_loop);
        let LoopRecipeItemV1::If { else_block, .. } =
            &mut self.items[if_item.raw() as usize].item
        else {
            unreachable!("if item shape checked above");
        };
        *else_block = Some(block);
        Ok(block)
    }

    /// Push an exit item (`break`/`continue`/`return`) into a block.
    pub(crate) fn push_exit(
        &mut self,
        block: LoopBlockKeyV1,
        exit: LoopExitKeyV1,
    ) -> Result<LoopItemKeyV1, LoopRecipeDraftRejectV1> {
        let owner = self.block_owner(block)?;
        let row = self
            .exits
            .get(exit.raw() as usize)
            .filter(|row| row.key == exit)
            .ok_or(LoopRecipeDraftRejectV1::UnknownExit { exit })?;
        if row.owner_loop != owner {
            return Err(LoopRecipeDraftRejectV1::ExitOwnerMismatch { exit });
        }
        self.push_item(block, LoopRecipeItemV1::Exit { exit })
    }

    /// Push a `ReadBinding` operation and record the read's source anchor.
    pub(crate) fn push_read_binding(
        &mut self,
        block: LoopBlockKeyV1,
        binding: LoopBindingKeyV1,
        site: &SourceExprSiteV1,
    ) -> Result<(LoopItemKeyV1, LoopValueKeyV1), LoopRecipeDraftRejectV1> {
        let class = self.binding_class(binding)?;
        let result = self.fresh_value(class);
        let item = self.push_item(
            block,
            LoopRecipeItemV1::Operation {
                operation: LoopOperationV1::ReadBinding { binding, result },
            },
        )?;
        self.record_effect(block, item, binding, site, true)?;
        Ok((item, result))
    }

    /// Push a `ConstI64` operation; pure items carry their expression site as
    /// evidence anchor but no binding effect.
    pub(crate) fn push_const_i64(
        &mut self,
        block: LoopBlockKeyV1,
        value: i64,
        site: &SourceExprSiteV1,
    ) -> Result<(LoopItemKeyV1, LoopValueKeyV1), LoopRecipeDraftRejectV1> {
        let result = self.fresh_value(LoopValueClassV1::I64);
        let item = self.push_item(
            block,
            LoopRecipeItemV1::Operation {
                operation: LoopOperationV1::ConstI64 { result, value },
            },
        )?;
        self.record_pure_evidence(block, item, site)?;
        Ok((item, result))
    }

    pub(crate) fn push_binary_i64(
        &mut self,
        block: LoopBlockKeyV1,
        op: LoopBinaryI64OpV1,
        left: LoopValueKeyV1,
        right: LoopValueKeyV1,
        site: &SourceExprSiteV1,
    ) -> Result<(LoopItemKeyV1, LoopValueKeyV1), LoopRecipeDraftRejectV1> {
        self.value_row(left)?;
        self.value_row(right)?;
        let result = self.fresh_value(LoopValueClassV1::I64);
        let item = self.push_item(
            block,
            LoopRecipeItemV1::Operation {
                operation: LoopOperationV1::BinaryI64 {
                    op,
                    left,
                    right,
                    result,
                },
            },
        )?;
        self.record_pure_evidence(block, item, site)?;
        Ok((item, result))
    }

    pub(crate) fn push_compare_i64(
        &mut self,
        block: LoopBlockKeyV1,
        op: LoopCompareI64OpV1,
        left: LoopValueKeyV1,
        right: LoopValueKeyV1,
        site: &SourceExprSiteV1,
    ) -> Result<(LoopItemKeyV1, LoopValueKeyV1), LoopRecipeDraftRejectV1> {
        self.value_row(left)?;
        self.value_row(right)?;
        let result = self.fresh_value(LoopValueClassV1::Bool);
        let item = self.push_item(
            block,
            LoopRecipeItemV1::Operation {
                operation: LoopOperationV1::CompareI64 {
                    op,
                    left,
                    right,
                    result,
                },
            },
        )?;
        self.record_pure_evidence(block, item, site)?;
        Ok((item, result))
    }

    /// Push a `WriteBinding` operation and record the write's source anchor.
    pub(crate) fn push_write_binding(
        &mut self,
        block: LoopBlockKeyV1,
        binding: LoopBindingKeyV1,
        value: LoopValueKeyV1,
        site: &SourceExprSiteV1,
    ) -> Result<LoopItemKeyV1, LoopRecipeDraftRejectV1> {
        self.value_row(value)?;
        self.binding_class(binding)?;
        let item = self.push_item(
            block,
            LoopRecipeItemV1::Operation {
                operation: LoopOperationV1::WriteBinding { binding, value },
            },
        )?;
        self.record_effect(block, item, binding, site, false)?;
        Ok(item)
    }

    /// Assemble the recipe and the co-recorded relations.  Rejects an
    /// incomplete draft before the semantic verifier runs.
    pub(crate) fn finish(self) -> Result<LoopRecipeDraftProductV1, LoopRecipeDraftRejectV1> {
        let mut loops = Vec::with_capacity(self.loops.len());
        for (index, row) in self.loops.iter().enumerate() {
            let loop_key = LoopNodeKeyV1::new(index as u32);
            let Some(condition) = row.condition else {
                return Err(LoopRecipeDraftRejectV1::MissingLoopCondition { loop_key });
            };
            let Some(body) = row.body else {
                return Err(LoopRecipeDraftRejectV1::MissingLoopBody { loop_key });
            };
            loops.push(LoopNodeV1 {
                key: loop_key,
                parent: row.parent,
                condition,
                body,
            });
        }
        Ok(LoopRecipeDraftProductV1 {
            recipe: LoopRecipeV1 {
                root_loop: LoopNodeKeyV1::new(0),
                loops,
                blocks: self.blocks,
                items: self.items,
                bindings: self.bindings,
                values: self.values,
                inputs: self.inputs,
                carriers: self.carriers,
                exits: self.exits,
            },
            bindings: self.binding_relations,
            effects: self.effect_relations,
            inputs: self.input_relations,
            evidence: self.evidence,
        })
    }

    fn record_effect(
        &mut self,
        block: LoopBlockKeyV1,
        item: LoopItemKeyV1,
        binding: LoopBindingKeyV1,
        site: &SourceExprSiteV1,
        is_read: bool,
    ) -> Result<(), LoopRecipeDraftRejectV1> {
        let owner_loop = self.block_owner(block)?;
        let (source_binding, _) = self.binding_sources[binding.raw() as usize].clone();
        let class = self.bindings[binding.raw() as usize].class;
        let anchor = LoopBindingEffectAnchorV1::Expr(OwnedExprSiteV1::new(self.owner, site.clone()));
        let role = if is_read {
            let ordinal = self.read_ordinals.entry(binding).or_insert(0);
            let role = LoopBindingEffectRoleV1::SourceRead { ordinal: *ordinal };
            *ordinal += 1;
            role
        } else {
            let ordinal = self.write_ordinals.entry(binding).or_insert(0);
            let role = LoopBindingEffectRoleV1::SourceWrite { ordinal: *ordinal };
            *ordinal += 1;
            role
        };
        self.effect_relations.push(LoopBindingEffectRelationV1::new(
            role,
            binding,
            source_binding,
            class,
            anchor.clone(),
        ));
        self.evidence.push(LoopOperationSourceEvidenceV1::new(
            item,
            anchor,
            self.loops[owner_loop.raw() as usize].source.clone(),
            owner_loop,
            block,
            Some(source_binding),
        ));
        Ok(())
    }

    fn record_pure_evidence(
        &mut self,
        block: LoopBlockKeyV1,
        item: LoopItemKeyV1,
        site: &SourceExprSiteV1,
    ) -> Result<(), LoopRecipeDraftRejectV1> {
        let owner_loop = self.block_owner(block)?;
        self.evidence.push(LoopOperationSourceEvidenceV1::new(
            item,
            LoopBindingEffectAnchorV1::Expr(OwnedExprSiteV1::new(self.owner, site.clone())),
            self.loops[owner_loop.raw() as usize].source.clone(),
            owner_loop,
            block,
            None,
        ));
        Ok(())
    }

    fn open_block(&mut self, owner_loop: LoopNodeKeyV1) -> LoopBlockKeyV1 {
        let key = LoopBlockKeyV1::new(self.blocks.len() as u32);
        self.blocks.push(LoopRecipeBlockV1 {
            key,
            owner_loop,
            items: Vec::new(),
        });
        key
    }

    fn push_item(
        &mut self,
        block: LoopBlockKeyV1,
        item: LoopRecipeItemV1,
    ) -> Result<LoopItemKeyV1, LoopRecipeDraftRejectV1> {
        let key = LoopItemKeyV1::new(self.items.len() as u32);
        let block_row = self
            .blocks
            .get_mut(block.raw() as usize)
            .filter(|row| row.key == block)
            .ok_or(LoopRecipeDraftRejectV1::UnknownBlock { block })?;
        block_row.items.push(key);
        self.items.push(LoopRecipeItemRowV1 { key, item });
        self.item_blocks.push(block);
        Ok(key)
    }

    fn fresh_value(&mut self, class: LoopValueClassV1) -> LoopValueKeyV1 {
        let key = LoopValueKeyV1::new(self.values.len() as u32);
        self.values.push(LoopRecipeValueV1 { key, class });
        key
    }

    fn loop_row(&self, loop_key: LoopNodeKeyV1) -> Result<&DraftLoopV1, LoopRecipeDraftRejectV1> {
        self.loops
            .get(loop_key.raw() as usize)
            .ok_or(LoopRecipeDraftRejectV1::UnknownLoop { loop_key })
    }

    fn block_owner(&self, block: LoopBlockKeyV1) -> Result<LoopNodeKeyV1, LoopRecipeDraftRejectV1> {
        self.blocks
            .get(block.raw() as usize)
            .filter(|row| row.key == block)
            .map(|row| row.owner_loop)
            .ok_or(LoopRecipeDraftRejectV1::UnknownBlock { block })
    }

    fn binding_class(
        &self,
        binding: LoopBindingKeyV1,
    ) -> Result<LoopValueClassV1, LoopRecipeDraftRejectV1> {
        self.bindings
            .get(binding.raw() as usize)
            .filter(|row| row.key == binding)
            .map(|row| row.class)
            .ok_or(LoopRecipeDraftRejectV1::UnknownBinding { binding })
    }

    fn value_row(&self, value: LoopValueKeyV1) -> Result<(), LoopRecipeDraftRejectV1> {
        self.values
            .get(value.raw() as usize)
            .filter(|row| row.key == value)
            .map(|_| ())
            .ok_or(LoopRecipeDraftRejectV1::UnknownValue { value })
    }
}
