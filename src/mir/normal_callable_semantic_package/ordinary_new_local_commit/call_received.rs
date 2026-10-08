//! Call-received commit row: one handle installed at the receiving local.
use super::*;

/// Caller-side commit row for `local h = <call>` where the callee's sealed
/// terminal is `return new <class>`: the callee's canonical object changed
/// ownership at the Return edge, and the caller installs the receiving
/// local as an owned Home owing exactly one `HomeRelease`. The row keeps
/// the callee's `object` identity — one object, one owner at a time — but
/// this is a call-site acquisition, never a local `new` home.
#[derive(Debug)]
pub(super) struct CallReceivedCommitV1 {
    pub(super) owner: FunctionOwnerIdV1,
    pub(super) binding: BindingRefV1,
    pub(super) declaration: SourceBindingSiteV1,
    pub(super) object: hakorune_mir_defs::CanonicalObjectIdV1,
    /// How the caller's exit discharges the received value: a definite
    /// `Handle` result is unconditionally live, while a `Nullable` result
    /// may carry the `Void` sentinel and owes a checked release instead.
    pub(super) release: CallReceivedReleaseV1,
    /// Owned-field children copied from the ledger's sealed residence map
    /// at begin time: `Some(None)` marks an owned object the package
    /// never proved — `end_available` stays false rather than dropping
    /// the children.
    pub(super) end_children: Option<Option<Box<[super::super::OwnedFieldChildV1]>>>,
    pub(super) progress: CallReceivedProgress,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum CallReceivedReleaseV1 {
    Handle,
    Nullable,
}
#[derive(Debug)]
pub(super) enum CallReceivedProgress {
    Emitting,
    Emitted {
        result: ValueId,
        bindings: Vec<(BasicBlockId, MirInstruction)>,
        /// The original physical packet shared with its binding group.
        packet: Option<std::rc::Rc<EmittedLexicalCallProjectionV1>>,
        phase: CallReceivedPhase,
    },
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum CallReceivedPhase {
    ExpressionCompleted,
    Installed,
    Checked,
}
impl CallReceivedCommitV1 {
    /// The call's normal-result value before the local install — the
    /// initializer the receiving statement binds.
    pub(super) fn initializer(&self) -> Option<ValueId> {
        match self.progress {
            CallReceivedProgress::Emitted {
                result,
                phase: CallReceivedPhase::ExpressionCompleted,
                ..
            } => Some(result),
            _ => None,
        }
    }
    /// The received handle value once the local is installed — the same
    /// value the sole `HomeRelease` reads at the caller's terminal exit.
    pub(super) fn local(&self) -> Option<ValueId> {
        match self.progress {
            CallReceivedProgress::Emitted {
                result,
                phase: CallReceivedPhase::Installed | CallReceivedPhase::Checked,
                ..
            } => Some(result),
            _ => None,
        }
    }
    pub(super) fn install(&mut self, local: ValueId) {
        match &mut self.progress {
            CallReceivedProgress::Emitted { result, phase, .. }
                if *result == local && *phase == CallReceivedPhase::ExpressionCompleted =>
            {
                *phase = CallReceivedPhase::Installed
            }
            _ => unreachable!("call-received local batch preflight"),
        }
    }
    pub(super) fn mark_checked(&mut self) {
        match &mut self.progress {
            CallReceivedProgress::Emitted { phase, .. } => *phase = CallReceivedPhase::Checked,
            _ => unreachable!("call-received emission batch validation"),
        }
    }
    pub(super) fn is_complete(&self) -> bool {
        matches!(
            self.progress,
            CallReceivedProgress::Emitted {
                phase: CallReceivedPhase::Checked,
                ..
            }
        )
    }
    /// Teardown plan for the received handle: proven owned field
    /// children in reverse declaration order, then the release the sealed
    /// result kind owes. `end_available` guarantees `end_children` is not
    /// `Some(None)`, so a missing proof never reaches this method.
    pub(super) fn end_plan(
        &self,
    ) -> Box<[(super::root_home::RootHomeReleaseSubjectV1, InvokeOperation)]> {
        let base = self.local().expect("installed received handle");
        let children = match &self.end_children {
            Some(Some(children)) => children.as_ref(),
            // An owned object without residence proof has no releasable
            // plan — `end_available` rejects it before any emission.
            Some(None) => unreachable!("owned teardown without sealed residences"),
            None => &[],
        };
        let release = match self.release {
            CallReceivedReleaseV1::Handle => InvokeOperation::HomeRelease {
                object: self.object,
                value: base,
            },
            CallReceivedReleaseV1::Nullable => InvokeOperation::HomeReleaseIfLive {
                object: self.object,
                value: base,
            },
        };
        children
            .iter()
            .rev()
            .map(|child| {
                (
                    super::root_home::RootHomeReleaseSubjectV1::FieldResidence {
                        binding: self.binding,
                        field: child.field,
                    },
                    match child.kind {
                        super::super::OwnedFieldChildKindV1::Array => {
                            InvokeOperation::OwnedFieldResidenceRelease {
                                field: child.field,
                                base,
                            }
                        }
                        super::super::OwnedFieldChildKindV1::Object(child_object) => {
                            InvokeOperation::OwnedObjectFieldRelease {
                                field: child.field,
                                base,
                                child: child_object,
                            }
                        }
                    },
                )
            })
            .chain(std::iter::once((
                super::root_home::RootHomeReleaseSubjectV1::Binding(self.binding),
                release,
            )))
            .collect()
    }
    pub(super) fn checked_bindings(&self) -> Result<&[(BasicBlockId, MirInstruction)], String> {
        match &self.progress {
            CallReceivedProgress::Emitted {
                bindings,
                phase: CallReceivedPhase::Checked,
                ..
            } => Ok(bindings),
            _ => Err(freeze("artifact-handle-unchecked")),
        }
    }
}
