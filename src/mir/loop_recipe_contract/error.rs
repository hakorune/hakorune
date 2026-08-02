//! Typed structural rejection vocabulary for portable Loop recipes.

use super::ids::{
    LoopBindingKeyV1, LoopBlockKeyV1, LoopCarrierKeyV1, LoopExitKeyV1, LoopItemKeyV1,
    LoopNodeKeyV1, LoopValueKeyV1,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum LoopRecipeRejectReasonV1 {
    UnsupportedVersion {
        found: u16,
    },
    SourceBindingCoverageMismatch {
        expected: usize,
        found: usize,
    },
    NonCanonicalSourceBindingOrder {
        expected: LoopNodeKeyV1,
        found: LoopNodeKeyV1,
    },
    DuplicateLoopSourcePath {
        first: LoopNodeKeyV1,
        second: LoopNodeKeyV1,
    },
    RootSourcePathMustStartWithBodyItem {
        loop_key: LoopNodeKeyV1,
    },
    SourcePathBodyItemAfterRoot {
        loop_key: LoopNodeKeyV1,
        step_index: usize,
    },
    NestedSourcePathNotDescendant {
        loop_key: LoopNodeKeyV1,
        parent_loop: LoopNodeKeyV1,
    },
    NestedSourcePathMustEnterLoopBody {
        loop_key: LoopNodeKeyV1,
        parent_loop: LoopNodeKeyV1,
    },
    NestedSourcePathSkipsIntermediateLoop {
        loop_key: LoopNodeKeyV1,
        parent_loop: LoopNodeKeyV1,
        step_index: usize,
    },
    EmptyBindingLabel {
        key: LoopBindingKeyV1,
    },
    NonCanonicalKeyOrder {
        domain: &'static str,
    },
    RootLoopMustBeZero,
    InvalidRootParent,
    InvalidLoopParent {
        loop_key: LoopNodeKeyV1,
    },
    DanglingLoop {
        key: LoopNodeKeyV1,
    },
    DanglingBlock {
        key: LoopBlockKeyV1,
    },
    DanglingItem {
        key: LoopItemKeyV1,
    },
    DanglingBinding {
        key: LoopBindingKeyV1,
    },
    DanglingValue {
        key: LoopValueKeyV1,
    },
    DanglingExit {
        key: LoopExitKeyV1,
    },
    DuplicateBlockUse {
        key: LoopBlockKeyV1,
    },
    DuplicateItemUse {
        key: LoopItemKeyV1,
    },
    DuplicateExitUse {
        key: LoopExitKeyV1,
    },
    DuplicateCarrierBinding {
        loop_key: LoopNodeKeyV1,
        binding: LoopBindingKeyV1,
    },
    DuplicateValueDefinition {
        key: LoopValueKeyV1,
    },
    UndefinedValue {
        key: LoopValueKeyV1,
    },
    UnusedBlock {
        key: LoopBlockKeyV1,
    },
    UnusedItem {
        key: LoopItemKeyV1,
    },
    UnusedExit {
        key: LoopExitKeyV1,
    },
    BlockOwnerMismatch {
        key: LoopBlockKeyV1,
    },
    ChildBlockMustFollowParent {
        key: LoopBlockKeyV1,
    },
    NestedLoopOwnerMismatch {
        key: LoopNodeKeyV1,
    },
    ExitOwnerMismatch {
        key: LoopExitKeyV1,
    },
    ExitTargetNotAncestor {
        key: LoopExitKeyV1,
    },
    CarrierEntryNotAvailable {
        key: LoopCarrierKeyV1,
    },
    ValueClassMismatch {
        key: LoopValueKeyV1,
    },
}
