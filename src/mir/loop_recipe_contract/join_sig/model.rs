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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct LoopJoinPortBindingV1 {
    pub(crate) loop_key: LoopNodeKeyV1,
    pub(crate) port: LoopJoinPortV1,
    pub(crate) binding: LoopBindingKeyV1,
    pub(crate) class: super::super::schema::LoopValueClassV1,
}

/// A verified logical identity at a loop's After port.
///
/// This is deliberately not Clone: later consumers must request and consume
/// the exact logical port capability rather than reconstructing it from a
/// source name, payload value, or physical ID.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedLoopAfterBindingV1(LoopJoinPortBindingV1);

impl VerifiedLoopAfterBindingV1 {
    pub(crate) fn loop_key(&self) -> LoopNodeKeyV1 {
        self.0.loop_key
    }

    pub(crate) fn binding(&self) -> LoopBindingKeyV1 {
        self.0.binding
    }

    pub(crate) fn class(&self) -> super::super::schema::LoopValueClassV1 {
        self.0.class
    }
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
    pub(crate) port_bindings: Vec<LoopJoinPortBindingV1>,
}

/// Caller-zero logical evidence for a verified conditional branch.
///
/// This is deliberately not a CFG edge or a PHI plan. It records the source
/// If item and each arm's logical disposition so a later physical consumer can
/// materialize the already-verified choice without rediscovering fallthrough.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LoopJoinBranchV1 {
    pub(crate) owner_loop: LoopNodeKeyV1,
    pub(crate) if_item: LoopItemKeyV1,
    pub(crate) condition: LoopValueKeyV1,
    pub(crate) then_arm: LoopJoinBranchArmV1,
    pub(crate) else_arm: LoopJoinBranchArmV1,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum LoopJoinBranchArmV1 {
    Exit(LoopJoinBranchExitV1),
    Fallthrough { payload: Vec<LoopJoinPayloadV1> },
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

    pub(crate) fn require_after_binding(
        &self,
        loop_key: LoopNodeKeyV1,
        binding: LoopBindingKeyV1,
        class: super::super::schema::LoopValueClassV1,
    ) -> Result<VerifiedLoopAfterBindingV1, LoopJoinSigRejectReasonV1> {
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
        Ok(VerifiedLoopAfterBindingV1(LoopJoinPortBindingV1 {
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
