//! Typed structural rejection vocabulary for portable Loop recipes.

use super::ids::{
    LoopBindingKeyV1, LoopNodeKeyV1, LoopValueKeyV1,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum LoopRecipeRejectReasonV1 {
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
    DanglingBinding {
        key: LoopBindingKeyV1,
    },
    DanglingValue {
        key: LoopValueKeyV1,
    },
    ReadOnlyInputNotRoot {
        binding: LoopBindingKeyV1,
    },
    DuplicateReadOnlyInput {
        binding: LoopBindingKeyV1,
    },
    DuplicateReadOnlyValue {
        value: LoopValueKeyV1,
    },
    ReadOnlyInputNotDeclared {
        value: LoopValueKeyV1,
    },
    ReadOnlyInputCarrierOverlap {
        binding: LoopBindingKeyV1,
    },
    ReadOnlyInputWritten {
        binding: LoopBindingKeyV1,
    },
    ReadOnlyInputUnused {
        binding: LoopBindingKeyV1,
    },
    ValueClassMismatch {
        key: LoopValueKeyV1,
    },
}
