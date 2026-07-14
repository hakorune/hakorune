use super::model::MirOwnershipKindV1;
use crate::mir::{BasicBlockId, ValueId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum OwnershipSsaErrorV1 {
    ParameterArity {
        expected: usize,
        actual: usize,
    },
    UnreachableBlock {
        block: BasicBlockId,
    },
    EdgeArgumentsForbidden {
        source: BasicBlockId,
        target: BasicBlockId,
    },
    UnknownValueKind {
        value: ValueId,
    },
    PhiKindMismatch {
        block: BasicBlockId,
        dst: ValueId,
    },
    BorrowedPhiForbidden {
        block: BasicBlockId,
        dst: ValueId,
    },
    BorrowedReturnForbidden {
        block: BasicBlockId,
        value: ValueId,
    },
    ResultOwnershipMismatch {
        block: BasicBlockId,
    },
    CopyOwnedSourceNotStrong {
        block: BasicBlockId,
        value: ValueId,
    },
    DestroyRequiresOwned {
        block: BasicBlockId,
        value: ValueId,
    },
    CopyOnOwned {
        block: BasicBlockId,
        value: ValueId,
    },
    OwnedUseAfterConsume {
        block: BasicBlockId,
        value: ValueId,
    },
    DuplicateConsumeOnEdge {
        source: BasicBlockId,
        target: BasicBlockId,
        value: ValueId,
    },
    PhiInputMissing {
        source: BasicBlockId,
        target: BasicBlockId,
        dst: ValueId,
    },
    PhiPredecessorMismatch {
        block: BasicBlockId,
        dst: ValueId,
    },
    ManagedCallOwnershipUnsupported {
        block: BasicBlockId,
    },
    LiveSetMismatch {
        block: BasicBlockId,
    },
    MissingDispositionAtExit {
        block: BasicBlockId,
        values: Box<[ValueId]>,
    },
    OwnedDestinationAlreadyLive {
        block: BasicBlockId,
        value: ValueId,
    },
    KindConflict {
        value: ValueId,
        first: MirOwnershipKindV1,
        second: MirOwnershipKindV1,
    },
}
