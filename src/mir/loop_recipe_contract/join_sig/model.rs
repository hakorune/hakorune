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
pub(crate) struct LoopJoinPayload<C> {
    pub(crate) binding: LoopBindingKeyV1,
    pub(crate) value: LoopValueKeyV1,
    pub(crate) class: C,
}

pub(crate) type LoopJoinPayloadV1 = LoopJoinPayload<super::super::schema::LoopValueClassV1>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct LoopJoinPortBinding<C> {
    pub(crate) loop_key: LoopNodeKeyV1,
    pub(crate) port: LoopJoinPortV1,
    pub(crate) binding: LoopBindingKeyV1,
    pub(crate) class: C,
}

pub(crate) type LoopJoinPortBindingV1 = LoopJoinPortBinding<super::super::schema::LoopValueClassV1>;

/// A verified logical identity at a loop's After port.
///
/// This is deliberately not Clone: later consumers must request and consume
/// the exact logical port capability rather than reconstructing it from a
/// source name, payload value, or physical ID.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedLoopAfterBinding<C>(LoopJoinPortBinding<C>);

pub(crate) type VerifiedLoopAfterBindingV1 =
    VerifiedLoopAfterBinding<super::super::schema::LoopValueClassV1>;

impl<C: Copy> VerifiedLoopAfterBinding<C> {
    pub(crate) fn loop_key(&self) -> LoopNodeKeyV1 {
        self.0.loop_key
    }

    pub(crate) fn binding(&self) -> LoopBindingKeyV1 {
        self.0.binding
    }

    pub(crate) fn class(&self) -> C {
        self.0.class
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LoopJoinEdge<C> {
    pub(crate) from: LoopJoinPortV1,
    pub(crate) to: LoopJoinPortV1,
    pub(crate) role: LoopJoinEdgeRoleV1,
    pub(crate) payload: Vec<LoopJoinPayload<C>>,
}

pub(crate) type LoopJoinEdgeV1 = LoopJoinEdge<super::super::schema::LoopValueClassV1>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LoopJoinLoop<C> {
    pub(crate) key: LoopNodeKeyV1,
    pub(crate) parent: Option<LoopNodeKeyV1>,
    pub(crate) carriers: Vec<LoopJoinPayload<C>>,
    pub(crate) edges: Vec<LoopJoinEdge<C>>,
}

pub(crate) type LoopJoinLoopV1 = LoopJoinLoop<super::super::schema::LoopValueClassV1>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LoopJoinSig<C> {
    pub(crate) loops: Vec<LoopJoinLoop<C>>,
    pub(crate) branches: Vec<LoopJoinBranch<C>>,
    pub(crate) port_bindings: Vec<LoopJoinPortBinding<C>>,
}

pub(crate) type LoopJoinSigV1 = LoopJoinSig<super::super::schema::LoopValueClassV1>;

/// Caller-zero logical evidence for a verified conditional branch.
///
/// This is deliberately not a CFG edge or a PHI plan. It records the source
/// If item and each arm's logical disposition so a later physical consumer can
/// materialize the already-verified choice without rediscovering fallthrough.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LoopJoinBranch<C> {
    pub(crate) owner_loop: LoopNodeKeyV1,
    pub(crate) if_item: LoopItemKeyV1,
    pub(crate) condition: LoopValueKeyV1,
    pub(crate) then_arm: LoopJoinBranchArm<C>,
    pub(crate) else_arm: LoopJoinBranchArm<C>,
}

pub(crate) type LoopJoinBranchV1 = LoopJoinBranch<super::super::schema::LoopValueClassV1>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum LoopJoinBranchArm<C> {
    Exit(LoopJoinBranchExit<C>),
    Fallthrough { payload: Vec<LoopJoinPayload<C>> },
}

pub(crate) type LoopJoinBranchArmV1 = LoopJoinBranchArm<super::super::schema::LoopValueClassV1>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LoopJoinBranchExit<C> {
    pub(crate) exit_item: LoopItemKeyV1,
    pub(crate) role: LoopJoinEdgeRoleV1,
    pub(crate) target_loop: LoopNodeKeyV1,
    pub(crate) payload: Vec<LoopJoinPayload<C>>,
}

pub(crate) type LoopJoinBranchExitV1 = LoopJoinBranchExit<super::super::schema::LoopValueClassV1>;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedLoopJoinSig<C>(LoopJoinSig<C>);

pub(crate) type VerifiedLoopJoinSigV1 = VerifiedLoopJoinSig<super::super::schema::LoopValueClassV1>;

impl<C: Copy + PartialEq> VerifiedLoopJoinSig<C> {
    pub(super) fn from_sig(sig: LoopJoinSig<C>) -> Self {
        Self(sig)
    }

    pub(crate) fn as_sig(&self) -> &LoopJoinSig<C> {
        &self.0
    }

    pub(crate) fn require_after_binding(
        &self,
        loop_key: LoopNodeKeyV1,
        binding: LoopBindingKeyV1,
        class: C,
    ) -> Result<VerifiedLoopAfterBinding<C>, LoopJoinSigRejectReasonV1> {
        let Some(row) = self.0.port_bindings.iter().find(|row| {
            row.loop_key == loop_key && row.port == LoopJoinPortV1::After && row.binding == binding
        }) else {
            return Err(LoopJoinSigRejectReasonV1::AfterBindingUnavailable { loop_key, binding });
        };
        if row.class != class {
            return Err(LoopJoinSigRejectReasonV1::AfterBindingClassMismatch {
                loop_key,
                port: LoopJoinPortV1::After,
                binding,
            });
        }
        Ok(VerifiedLoopAfterBinding(LoopJoinPortBinding {
            loop_key,
            port: LoopJoinPortV1::After,
            binding,
            class,
        }))
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
    DuplicatePortBinding {
        loop_key: LoopNodeKeyV1,
        port: LoopJoinPortV1,
        binding: LoopBindingKeyV1,
    },
    PortBindingSetMismatch {
        loop_key: LoopNodeKeyV1,
        port: LoopJoinPortV1,
    },
    PortBindingClassMismatch {
        loop_key: LoopNodeKeyV1,
        port: LoopJoinPortV1,
        binding: LoopBindingKeyV1,
    },
    AfterBindingUnavailable {
        loop_key: LoopNodeKeyV1,
        binding: LoopBindingKeyV1,
    },
    AfterBindingClassMismatch {
        loop_key: LoopNodeKeyV1,
        port: LoopJoinPortV1,
        binding: LoopBindingKeyV1,
    },
}
