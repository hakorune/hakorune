use crate::mir::ownership_ssa::FunctionResultOwnershipV1;
use crate::mir::resolved_semantics::{BindingRefV1, FunctionOwnerIdV1};
use crate::mir::ValueId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum OwnershipTransitionErrorV1 {
    ForeignOwner {
        expected: FunctionOwnerIdV1,
        actual: FunctionOwnerIdV1,
        binding: BindingRefV1,
    },
    DuplicateClosingBinding {
        binding: BindingRefV1,
    },
    DuplicateOwnedToken {
        value: ValueId,
    },
    OwnedNextAliasesPrevious {
        value: ValueId,
    },
    ScopeLocalTailMissing {
        binding: BindingRefV1,
    },
    ScopeLocalTailValueMismatch {
        binding: BindingRefV1,
        expected: ValueId,
        actual: ValueId,
    },
    OuterBorrowedTailIsScopeLocal {
        binding: BindingRefV1,
    },
    ForwardedOwnedStillOwnedByScope {
        value: ValueId,
    },
    ResultOwnershipMismatch {
        expected: FunctionResultOwnershipV1,
        actual: FunctionResultOwnershipV1,
    },
}
