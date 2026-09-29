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
    pub(super) progress: CallReceivedProgress,
}
#[derive(Debug)]
pub(super) enum CallReceivedProgress {
    Emitting,
    Emitted {
        result: ValueId,
        bindings: Vec<(BasicBlockId, MirInstruction)>,
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
            CallReceivedProgress::Emitted {
                result,
                phase,
                ..
            } if *result == local && *phase == CallReceivedPhase::ExpressionCompleted => {
                *phase = CallReceivedPhase::Installed
            }
            _ => unreachable!("call-received local batch preflight"),
        }
    }
    pub(super) fn mark_checked(&mut self) {
        match &mut self.progress {
            CallReceivedProgress::Emitted { phase, .. } => {
                *phase = CallReceivedPhase::Checked
            }
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
    pub(super) fn end_operation(&self) -> InvokeOperation {
        InvokeOperation::HomeRelease {
            object: self.object,
            value: self.local().expect("installed received handle"),
        }
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
