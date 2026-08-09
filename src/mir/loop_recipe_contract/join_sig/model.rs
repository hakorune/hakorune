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
pub(crate) type LoopJoinPayloadV2 = LoopJoinPayload<super::super::schema_v2::LoopValueClassV2>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct LoopJoinPortBinding<C> {
    pub(crate) loop_key: LoopNodeKeyV1,
    pub(crate) port: LoopJoinPortV1,
    pub(crate) binding: LoopBindingKeyV1,
    pub(crate) class: C,
}

pub(crate) type LoopJoinPortBindingV1 = LoopJoinPortBinding<super::super::schema::LoopValueClassV1>;
pub(crate) type LoopJoinPortBindingV2 =
    LoopJoinPortBinding<super::super::schema_v2::LoopValueClassV2>;

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
pub(crate) type LoopJoinEdgeV2 = LoopJoinEdge<super::super::schema_v2::LoopValueClassV2>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LoopJoinLoop<C> {
    pub(crate) key: LoopNodeKeyV1,
    pub(crate) parent: Option<LoopNodeKeyV1>,
    pub(crate) carriers: Vec<LoopJoinPayload<C>>,
    pub(crate) edges: Vec<LoopJoinEdge<C>>,
}

pub(crate) type LoopJoinLoopV1 = LoopJoinLoop<super::super::schema::LoopValueClassV1>;
pub(crate) type LoopJoinLoopV2 = LoopJoinLoop<super::super::schema_v2::LoopValueClassV2>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LoopJoinSig<C, T = LoopNodeKeyV1> {
    pub(crate) loops: Vec<LoopJoinLoop<C>>,
    pub(crate) branches: Vec<LoopJoinBranch<C, T>>,
    pub(crate) port_bindings: Vec<LoopJoinPortBinding<C>>,
}

pub(crate) type LoopJoinSigV1 = LoopJoinSig<super::super::schema::LoopValueClassV1, LoopNodeKeyV1>;
pub(crate) type LoopJoinSigV2 =
    LoopJoinSig<super::super::schema_v2::LoopValueClassV2, LoopJoinBranchExitTargetV2>;

/// Caller-zero logical evidence for a verified conditional branch.
///
/// This is deliberately not a CFG edge or a PHI plan. It records the source
/// If item and each arm's logical disposition so a later physical consumer can
/// materialize the already-verified choice without rediscovering fallthrough.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LoopJoinBranch<C, T = LoopNodeKeyV1> {
    pub(crate) owner_loop: LoopNodeKeyV1,
    pub(crate) if_item: LoopItemKeyV1,
    pub(crate) condition: LoopValueKeyV1,
    pub(crate) then_arm: LoopJoinBranchArm<C, T>,
    pub(crate) else_arm: LoopJoinBranchArm<C, T>,
}

pub(crate) type LoopJoinBranchV1 =
    LoopJoinBranch<super::super::schema::LoopValueClassV1, LoopNodeKeyV1>;
pub(crate) type LoopJoinBranchV2 =
    LoopJoinBranch<super::super::schema_v2::LoopValueClassV2, LoopJoinBranchExitTargetV2>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum LoopJoinBranchArm<C, T = LoopNodeKeyV1> {
    Exit(LoopJoinBranchExit<C, T>),
    Fallthrough { payload: Vec<LoopJoinPayload<C>> },
}

pub(crate) type LoopJoinBranchArmV1 =
    LoopJoinBranchArm<super::super::schema::LoopValueClassV1, LoopNodeKeyV1>;
pub(crate) type LoopJoinBranchArmV2 =
    LoopJoinBranchArm<super::super::schema_v2::LoopValueClassV2, LoopJoinBranchExitTargetV2>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LoopJoinBranchExitTargetV2 {
    Loop(LoopNodeKeyV1),
    FunctionExit,
}

impl LoopJoinBranchExitTargetV2 {
    pub(crate) fn accepts_role(self, role: LoopJoinEdgeRoleV1) -> bool {
        matches!(
            (self, role),
            (
                Self::Loop(_),
                LoopJoinEdgeRoleV1::Break | LoopJoinEdgeRoleV1::Continue
            ) | (Self::FunctionExit, LoopJoinEdgeRoleV1::Return)
        )
    }
}

pub(in crate::mir::loop_recipe_contract) trait LoopJoinBranchTarget:
    Copy + Eq
{
    fn accepts(self, role: LoopJoinEdgeRoleV1) -> bool;
}

impl LoopJoinBranchTarget for LoopNodeKeyV1 {
    fn accepts(self, role: LoopJoinEdgeRoleV1) -> bool {
        matches!(
            role,
            LoopJoinEdgeRoleV1::Break | LoopJoinEdgeRoleV1::Continue
        )
    }
}

impl LoopJoinBranchTarget for LoopJoinBranchExitTargetV2 {
    fn accepts(self, role: LoopJoinEdgeRoleV1) -> bool {
        self.accepts_role(role)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LoopJoinBranchExit<C, T = LoopNodeKeyV1> {
    pub(crate) exit_item: LoopItemKeyV1,
    pub(crate) role: LoopJoinEdgeRoleV1,
    pub(crate) target: T,
    pub(crate) payload: Vec<LoopJoinPayload<C>>,
}

pub(crate) type LoopJoinBranchExitV1 =
    LoopJoinBranchExit<super::super::schema::LoopValueClassV1, LoopNodeKeyV1>;
pub(crate) type LoopJoinBranchExitV2 =
    LoopJoinBranchExit<super::super::schema_v2::LoopValueClassV2, LoopJoinBranchExitTargetV2>;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedLoopJoinSig<C, T = LoopNodeKeyV1>(LoopJoinSig<C, T>);

pub(crate) type VerifiedLoopJoinSigV1 =
    VerifiedLoopJoinSig<super::super::schema::LoopValueClassV1, LoopNodeKeyV1>;
pub(crate) type VerifiedLoopJoinSigV2 =
    VerifiedLoopJoinSig<super::super::schema_v2::LoopValueClassV2, LoopJoinBranchExitTargetV2>;

impl<C: Copy + PartialEq, T> VerifiedLoopJoinSig<C, T> {
    pub(super) fn from_sig(sig: LoopJoinSig<C, T>) -> Self {
        Self(sig)
    }

    pub(crate) fn as_sig(&self) -> &LoopJoinSig<C, T> {
        &self.0
    }

    /// Issue one exact After capability inside the JoinSig owner.
    ///
    /// V2 keeps this raw-key operation private to the `join_sig` subtree. Its
    /// public-in-MIR boundary issues JoinSig and After as one non-splittable
    /// closure instead of exporting this lookup.
    pub(super) fn require_after_binding_internal(
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

impl VerifiedLoopJoinSig<super::super::schema::LoopValueClassV1, LoopNodeKeyV1> {
    pub(crate) fn require_after_binding(
        &self,
        loop_key: LoopNodeKeyV1,
        binding: LoopBindingKeyV1,
        class: super::super::schema::LoopValueClassV1,
    ) -> Result<VerifiedLoopAfterBindingV1, LoopJoinSigRejectReasonV1> {
        self.require_after_binding_internal(loop_key, binding, class)
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
