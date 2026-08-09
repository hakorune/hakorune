//! Borrowed logical Recipe view used by the JoinSig engine.
//!
//! The view exposes only the structure and value-flow facts needed to
//! elaborate a JoinSig. It owns no source admission, Recipe verification,
//! physical layout, or control-flow meaning of its own.

use super::super::ids::{
    LoopBindingKeyV1, LoopBlockKeyV1, LoopExitKeyV1, LoopItemKeyV1, LoopNodeKeyV1, LoopValueKeyV1,
};
use super::super::schema::{
    LoopConditionV1, LoopExitKindV1, LoopOperationV1, LoopRecipeItemV1, LoopRecipeV1,
    LoopValueClassV1,
};
use super::super::verify::VerifiedLoopRecipeV1;

pub(in crate::mir::loop_recipe_contract) trait LoopJoinRecipeView {
    type Class: Copy + Eq;

    fn root_loop(&self) -> LoopNodeKeyV1;
    fn inputs(&self) -> &[LoopValueKeyV1];
    fn loop_count(&self) -> usize;
    fn loop_at(&self, key: LoopNodeKeyV1) -> Option<LoopJoinLoopView>;
    fn block_at(&self, key: LoopBlockKeyV1) -> Option<LoopJoinBlockView<'_>>;
    fn item_at(&self, key: LoopItemKeyV1) -> Option<LoopJoinItemView<'_>>;
    fn carrier_count(&self) -> usize;
    fn carrier_at(&self, index: usize) -> Option<LoopJoinCarrierView<Self::Class>>;
    fn exit_at(&self, key: LoopExitKeyV1) -> Option<LoopJoinExitView>;
}

#[derive(Clone, Copy)]
pub(in crate::mir::loop_recipe_contract) struct LoopJoinLoopView {
    pub(super) parent: Option<LoopNodeKeyV1>,
    pub(super) condition: LoopJoinConditionView,
    pub(super) body: LoopBlockKeyV1,
}

#[derive(Clone, Copy)]
pub(in crate::mir::loop_recipe_contract) enum LoopJoinConditionView {
    Always,
    Predicate {
        block: LoopBlockKeyV1,
        value: LoopValueKeyV1,
    },
}

pub(in crate::mir::loop_recipe_contract) struct LoopJoinBlockView<'a> {
    pub(in crate::mir::loop_recipe_contract) items: &'a [LoopItemKeyV1],
}

pub(in crate::mir::loop_recipe_contract) enum LoopJoinItemView<'a> {
    Operation(LoopJoinOperationView<'a>),
    If {
        condition: LoopValueKeyV1,
        then_block: LoopBlockKeyV1,
        else_block: Option<LoopBlockKeyV1>,
    },
    Loop {
        loop_key: LoopNodeKeyV1,
    },
    Exit {
        exit: LoopExitKeyV1,
    },
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::loop_recipe_contract) enum LoopJoinOperationFamily {
    ReadBinding,
    ConstI64,
    BinaryI64,
    CompareI64,
    WriteBinding,
}

pub(in crate::mir::loop_recipe_contract) enum LoopJoinOperationView<'a> {
    ReadBinding {
        binding: LoopBindingKeyV1,
        result: LoopValueKeyV1,
    },
    Define {
        family: LoopJoinOperationFamily,
        uses: LoopJoinValueUses<'a>,
        result: Option<LoopValueKeyV1>,
    },
    WriteBinding {
        binding: LoopBindingKeyV1,
        value: LoopValueKeyV1,
    },
}

pub(in crate::mir::loop_recipe_contract) enum LoopJoinValueUses<'a> {
    None,
    Two(LoopValueKeyV1, LoopValueKeyV1),
    Call {
        receiver: Option<LoopValueKeyV1>,
        args: &'a [LoopValueKeyV1],
    },
}

#[derive(Clone, Copy)]
pub(in crate::mir::loop_recipe_contract) struct LoopJoinCarrierView<C> {
    pub(super) owner_loop: LoopNodeKeyV1,
    pub(super) binding: LoopBindingKeyV1,
    pub(super) class: C,
    pub(super) entry_value: LoopValueKeyV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::loop_recipe_contract) enum LoopJoinExitView {
    Break { target_loop: LoopNodeKeyV1 },
    Continue { target_loop: LoopNodeKeyV1 },
    Return { value: Option<LoopValueKeyV1> },
}

pub(super) struct LoopRecipeV1JoinView<'a> {
    recipe: &'a LoopRecipeV1,
}

impl<'a> LoopRecipeV1JoinView<'a> {
    pub(super) fn verified(recipe: &'a VerifiedLoopRecipeV1) -> Self {
        Self {
            recipe: recipe.as_recipe(),
        }
    }

    pub(super) fn raw(recipe: &'a LoopRecipeV1) -> Self {
        Self { recipe }
    }
}

impl LoopJoinRecipeView for LoopRecipeV1JoinView<'_> {
    type Class = LoopValueClassV1;

    fn root_loop(&self) -> LoopNodeKeyV1 {
        self.recipe.root_loop
    }

    fn inputs(&self) -> &[LoopValueKeyV1] {
        &self.recipe.inputs
    }

    fn loop_count(&self) -> usize {
        self.recipe.loops.len()
    }

    fn loop_at(&self, key: LoopNodeKeyV1) -> Option<LoopJoinLoopView> {
        let row = self.recipe.loops.get(key.raw() as usize)?;
        let condition = match row.condition {
            LoopConditionV1::Always => LoopJoinConditionView::Always,
            LoopConditionV1::Predicate { block, value } => {
                LoopJoinConditionView::Predicate { block, value }
            }
        };
        Some(LoopJoinLoopView {
            parent: row.parent,
            condition,
            body: row.body,
        })
    }

    fn block_at(&self, key: LoopBlockKeyV1) -> Option<LoopJoinBlockView<'_>> {
        let row = self.recipe.blocks.get(key.raw() as usize)?;
        Some(LoopJoinBlockView { items: &row.items })
    }

    fn item_at(&self, key: LoopItemKeyV1) -> Option<LoopJoinItemView<'_>> {
        let row = self.recipe.items.get(key.raw() as usize)?;
        Some(match &row.item {
            LoopRecipeItemV1::Operation { operation } => {
                LoopJoinItemView::Operation(operation_view(*operation))
            }
            LoopRecipeItemV1::If {
                condition,
                then_block,
                else_block,
            } => LoopJoinItemView::If {
                condition: *condition,
                then_block: *then_block,
                else_block: *else_block,
            },
            LoopRecipeItemV1::Loop { loop_key } => LoopJoinItemView::Loop {
                loop_key: *loop_key,
            },
            LoopRecipeItemV1::Exit { exit } => LoopJoinItemView::Exit { exit: *exit },
        })
    }

    fn carrier_count(&self) -> usize {
        self.recipe.carriers.len()
    }

    fn carrier_at(&self, index: usize) -> Option<LoopJoinCarrierView<Self::Class>> {
        let row = self.recipe.carriers.get(index)?;
        Some(LoopJoinCarrierView {
            owner_loop: row.owner_loop,
            binding: row.binding,
            class: row.class,
            entry_value: row.entry_value,
        })
    }

    fn exit_at(&self, key: LoopExitKeyV1) -> Option<LoopJoinExitView> {
        let row = self.recipe.exits.get(key.raw() as usize)?;
        Some(match row.kind {
            LoopExitKindV1::Break { target_loop } => LoopJoinExitView::Break { target_loop },
            LoopExitKindV1::Continue { target_loop } => LoopJoinExitView::Continue { target_loop },
            LoopExitKindV1::Return { value } => LoopJoinExitView::Return { value },
        })
    }
}

fn operation_view(operation: LoopOperationV1) -> LoopJoinOperationView<'static> {
    match operation {
        LoopOperationV1::ReadBinding { binding, result } => {
            LoopJoinOperationView::ReadBinding { binding, result }
        }
        LoopOperationV1::ConstI64 { result, .. } => LoopJoinOperationView::Define {
            family: LoopJoinOperationFamily::ConstI64,
            uses: LoopJoinValueUses::None,
            result: Some(result),
        },
        LoopOperationV1::BinaryI64 {
            left,
            right,
            result,
            ..
        } => LoopJoinOperationView::Define {
            family: LoopJoinOperationFamily::BinaryI64,
            uses: LoopJoinValueUses::Two(left, right),
            result: Some(result),
        },
        LoopOperationV1::CompareI64 {
            left,
            right,
            result,
            ..
        } => LoopJoinOperationView::Define {
            family: LoopJoinOperationFamily::CompareI64,
            uses: LoopJoinValueUses::Two(left, right),
            result: Some(result),
        },
        LoopOperationV1::WriteBinding { binding, value } => {
            LoopJoinOperationView::WriteBinding { binding, value }
        }
    }
}
