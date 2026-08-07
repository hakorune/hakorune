//! Neutral logical Loop After continuation capability.

use crate::mir::resolved_semantics::FunctionOwnerIdV1;

use super::ids::LoopNodeKeyV1;
use super::join_sig::VerifiedLoopAfterBindingV1;

/// Move-only wrapper for the already verified logical Loop After port.
///
/// The JoinSig issuer remains the authority for `VerifiedLoopAfterBindingV1`.
/// This wrapper only transports that capability across profile boundaries.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedLoopContinuationContractV1 {
    owner: FunctionOwnerIdV1,
    loop_key: LoopNodeKeyV1,
    after: VerifiedLoopAfterBindingV1,
}

impl VerifiedLoopContinuationContractV1 {
    pub(crate) fn from_after(owner: FunctionOwnerIdV1, after: VerifiedLoopAfterBindingV1) -> Self {
        Self {
            owner,
            loop_key: after.loop_key(),
            after,
        }
    }

    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(crate) const fn loop_key(&self) -> LoopNodeKeyV1 {
        self.loop_key
    }

    pub(crate) fn after(&self) -> &VerifiedLoopAfterBindingV1 {
        &self.after
    }

    pub(crate) fn into_after(self) -> VerifiedLoopAfterBindingV1 {
        self.after
    }
}
