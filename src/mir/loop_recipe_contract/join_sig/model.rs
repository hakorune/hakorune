use super::super::ids::{LoopBindingKeyV1, LoopItemKeyV1, LoopNodeKeyV1, LoopValueKeyV1};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum LoopJoinPortV1 {
    Preheader,
    Header,
    Body,
    After,
    FunctionExit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum LoopJoinEdgeRoleV1 {
    Enter,
    PredicateTrue,
    PredicateFalse,
    BodyEntry,
    Backedge,
    Break,
    Continue,
    Return,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LoopJoinPayloadV1 {
    pub(crate) binding: LoopBindingKeyV1,
    pub(crate) value: LoopValueKeyV1,
    pub(crate) class: super::super::schema::LoopValueClassV1,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LoopJoinEdgeV1 {
    pub(crate) from: LoopJoinPortV1,
    pub(crate) to: LoopJoinPortV1,
    pub(crate) role: LoopJoinEdgeRoleV1,
    pub(crate) payload: Vec<LoopJoinPayloadV1>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LoopJoinLoopV1 {
    pub(crate) key: LoopNodeKeyV1,
    pub(crate) parent: Option<LoopNodeKeyV1>,
    pub(crate) carriers: Vec<LoopJoinPayloadV1>,
    pub(crate) edges: Vec<LoopJoinEdgeV1>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LoopJoinSigV1 {
    pub(crate) loops: Vec<LoopJoinLoopV1>,
    pub(crate) branches: Vec<LoopJoinBranchV1>,
}

/// Caller-zero logical evidence for the bounded LoopTrue branch shape.
///
/// This is deliberately not a CFG edge or a PHI plan. It records the source
/// If item and its two direct exits so a later physical consumer can decide
/// how to materialize the already-verified choice.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LoopJoinBranchV1 {
    pub(crate) owner_loop: LoopNodeKeyV1,
    pub(crate) if_item: LoopItemKeyV1,
    pub(crate) condition: LoopValueKeyV1,
    pub(crate) then_exit: LoopJoinBranchExitV1,
    pub(crate) else_exit: LoopJoinBranchExitV1,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LoopJoinBranchExitV1 {
    pub(crate) exit_item: LoopItemKeyV1,
    pub(crate) role: LoopJoinEdgeRoleV1,
    pub(crate) target_loop: LoopNodeKeyV1,
    pub(crate) payload: Vec<LoopJoinPayloadV1>,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedLoopJoinSigV1(LoopJoinSigV1);

impl VerifiedLoopJoinSigV1 {
    pub(super) fn from_sig(sig: LoopJoinSigV1) -> Self {
        Self(sig)
    }

    pub(crate) fn as_sig(&self) -> &LoopJoinSigV1 {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum LoopJoinSigRejectReasonV1 {
    MissingCarrierClosure {
        loop_key: LoopNodeKeyV1,
        binding: LoopBindingKeyV1,
    },
    BindingNotAvailable {
        binding: LoopBindingKeyV1,
    },
    ValueNotAvailable {
        value: LoopValueKeyV1,
    },
    UnreachableItem {
        item: LoopItemKeyV1,
    },
    BranchMergeMismatch {
        item: LoopItemKeyV1,
    },
    UnsupportedExit {
        item: LoopItemKeyV1,
    },
    UnsupportedNestedPredicate {
        loop_key: LoopNodeKeyV1,
    },
}
