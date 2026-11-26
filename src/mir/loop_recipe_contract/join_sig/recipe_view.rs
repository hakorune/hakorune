//! Borrowed logical Recipe view used by the JoinSig engine.
//!
//! The view exposes only the structure and value-flow facts needed to
//! elaborate a JoinSig. It owns no source admission, Recipe verification,
//! physical layout, or control-flow meaning of its own.

use super::super::ids::{
    LoopBindingKeyV1, LoopBlockKeyV1, LoopExitKeyV1, LoopItemKeyV1, LoopNodeKeyV1, LoopValueKeyV1,
};
use super::model::LoopJoinBranchTarget;

pub(in crate::mir::loop_recipe_contract) trait LoopJoinRecipeView {
    type Class: Copy + Eq;
    type BranchTarget: LoopJoinBranchTarget;

    fn root_loop(&self) -> LoopNodeKeyV1;
    fn inputs(&self) -> &[LoopValueKeyV1];
    fn loop_count(&self) -> usize;
    fn loop_at(&self, key: LoopNodeKeyV1) -> Option<LoopJoinLoopView>;
    fn block_at(&self, key: LoopBlockKeyV1) -> Option<LoopJoinBlockView<'_>>;
    fn item_at(&self, key: LoopItemKeyV1) -> Option<LoopJoinItemView<'_>>;
    fn carrier_count(&self) -> usize;
    fn carrier_at(&self, index: usize) -> Option<LoopJoinCarrierView<Self::Class>>;
    fn read_only_input_count(&self) -> usize {
        0
    }
    fn read_only_input_at(&self, _index: usize) -> Option<LoopJoinReadOnlyInputView> {
        None
    }
    fn exit_at(&self, key: LoopExitKeyV1) -> Option<LoopJoinExitView>;
    fn branch_exit_target(
        &self,
        owner_loop: LoopNodeKeyV1,
        exit: LoopJoinExitView,
    ) -> Option<Self::BranchTarget>;
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
    BranchPhi,
    ReadBinding,
    ConstI64,
    ConstText,
    ConstNull,
    BinaryI64,
    TextAdd,
    CompareI64,
    WriteBinding,
    DynamicAdd,
    DynamicSub,
    DynamicLess,
    CallSlot,
    TextEq,
    BoolExpression,
    ChildLoopAfter,
}

pub(in crate::mir::loop_recipe_contract) enum LoopJoinOperationView<'a> {
    BranchPhi {
        source_if: LoopItemKeyV1,
        then_value: LoopValueKeyV1,
        else_value: LoopValueKeyV1,
        result: LoopValueKeyV1,
    },
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
    Many(&'a [LoopValueKeyV1]),
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

#[derive(Clone, Copy)]
pub(in crate::mir::loop_recipe_contract) struct LoopJoinReadOnlyInputView {
    pub(super) owner_loop: LoopNodeKeyV1,
    pub(super) binding: LoopBindingKeyV1,
    pub(super) entry_value: LoopValueKeyV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::loop_recipe_contract) enum LoopJoinExitView {
    Break { target_loop: LoopNodeKeyV1 },
    Continue { target_loop: LoopNodeKeyV1 },
    Return { value: Option<LoopValueKeyV1> },
}
