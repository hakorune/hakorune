//! Borrowed V2 adapter for the neutral JoinSig engine.
//!
//! This adapter projects the verified typed Recipe without converting its
//! value classes, selecting a source profile, or owning a second traversal.

use super::super::ids::{
    LoopBlockKeyV1, LoopExitKeyV1, LoopItemKeyV1, LoopNodeKeyV1, LoopValueKeyV1,
};
use super::super::schema_v2::{
    LoopConditionV2, LoopExitKindV2, LoopOperationV2, LoopRecipeItemV2, LoopRecipeV2,
    LoopValueClassV2,
};
use super::super::typed_schema_v2::VerifiedLoopRecipeV2;
use super::model::LoopJoinBranchExitTargetV2;
use super::recipe_view::{
    LoopJoinBlockView, LoopJoinCarrierView, LoopJoinConditionView, LoopJoinExitView,
    LoopJoinItemView, LoopJoinLoopView, LoopJoinOperationFamily, LoopJoinOperationView,
    LoopJoinRecipeView, LoopJoinValueUses,
};

pub(super) struct LoopRecipeV2JoinView<'a> {
    recipe: &'a LoopRecipeV2,
}

impl<'a> LoopRecipeV2JoinView<'a> {
    pub(super) fn verified(recipe: &'a VerifiedLoopRecipeV2) -> Self {
        Self {
            recipe: recipe.as_recipe(),
        }
    }
}

impl LoopJoinRecipeView for LoopRecipeV2JoinView<'_> {
    type Class = LoopValueClassV2;
    type BranchTarget = LoopJoinBranchExitTargetV2;

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
            LoopConditionV2::Always => LoopJoinConditionView::Always,
            LoopConditionV2::Predicate { block, value } => {
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
            LoopRecipeItemV2::Operation { operation } => {
                LoopJoinItemView::Operation(operation_view(operation))
            }
            LoopRecipeItemV2::If {
                condition,
                then_block,
                else_block,
            } => LoopJoinItemView::If {
                condition: *condition,
                then_block: *then_block,
                else_block: *else_block,
            },
            LoopRecipeItemV2::Loop { loop_key } => LoopJoinItemView::Loop {
                loop_key: *loop_key,
            },
            LoopRecipeItemV2::Exit { exit } => LoopJoinItemView::Exit { exit: *exit },
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
            LoopExitKindV2::Break { target_loop } => LoopJoinExitView::Break { target_loop },
            LoopExitKindV2::Continue { target_loop } => LoopJoinExitView::Continue { target_loop },
            LoopExitKindV2::Return { value } => LoopJoinExitView::Return { value },
        })
    }

    fn branch_exit_target(
        &self,
        owner_loop: LoopNodeKeyV1,
        exit: LoopJoinExitView,
    ) -> Option<Self::BranchTarget> {
        match exit {
            LoopJoinExitView::Break { target_loop }
            | LoopJoinExitView::Continue { target_loop }
                if target_loop == owner_loop =>
            {
                Some(LoopJoinBranchExitTargetV2::Loop(target_loop))
            }
            LoopJoinExitView::Return { .. } => Some(LoopJoinBranchExitTargetV2::FunctionExit),
            LoopJoinExitView::Break { .. } | LoopJoinExitView::Continue { .. } => None,
        }
    }
}

fn operation_view(operation: &LoopOperationV2) -> LoopJoinOperationView<'_> {
    match operation {
        LoopOperationV2::ReadBinding { binding, result } => LoopJoinOperationView::ReadBinding {
            binding: *binding,
            result: *result,
        },
        LoopOperationV2::ConstI64 { result, .. } => define(
            LoopJoinOperationFamily::ConstI64,
            LoopJoinValueUses::None,
            Some(*result),
        ),
        LoopOperationV2::BinaryI64 {
            left,
            right,
            result,
            ..
        } => define(
            LoopJoinOperationFamily::BinaryI64,
            LoopJoinValueUses::Two(*left, *right),
            Some(*result),
        ),
        LoopOperationV2::CompareI64 {
            left,
            right,
            result,
            ..
        } => define(
            LoopJoinOperationFamily::CompareI64,
            LoopJoinValueUses::Two(*left, *right),
            Some(*result),
        ),
        LoopOperationV2::DynamicAdd {
            left,
            right,
            result,
        } => define(
            LoopJoinOperationFamily::DynamicAdd,
            LoopJoinValueUses::Two(*left, *right),
            Some(*result),
        ),
        LoopOperationV2::DynamicLess {
            left,
            right,
            result,
        } => define(
            LoopJoinOperationFamily::DynamicLess,
            LoopJoinValueUses::Two(*left, *right),
            Some(*result),
        ),
        LoopOperationV2::WriteBinding { binding, value } => LoopJoinOperationView::WriteBinding {
            binding: *binding,
            value: *value,
        },
        LoopOperationV2::CallSlot {
            receiver,
            args,
            result,
        } => define(
            LoopJoinOperationFamily::CallSlot,
            LoopJoinValueUses::Call {
                receiver: *receiver,
                args,
            },
            *result,
        ),
        LoopOperationV2::TextEq {
            left,
            right,
            result,
        } => define(
            LoopJoinOperationFamily::TextEq,
            LoopJoinValueUses::Two(*left, *right),
            Some(*result),
        ),
    }
}

fn define(
    family: LoopJoinOperationFamily,
    uses: LoopJoinValueUses<'_>,
    result: Option<LoopValueKeyV1>,
) -> LoopJoinOperationView<'_> {
    LoopJoinOperationView::Define {
        family,
        uses,
        result,
    }
}
