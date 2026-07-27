//! Stack-owned, payload-retaining function activation ledger.
//!
//! C4 has no exact-function production ingress. Only `Armed` and its
//! unobserved terminal are real in this row; D1 adds the later transitions
//! when their producers exist.

use crate::mir::builder::preloop_stageb_context_install::InstalledPreloopStageBContextV1;
use crate::mir::preloop_stageb_carrier::{
    OwnedPreloopStageBCarrierRowV1, PreparedPreloopStageBActivationLedgerPartsV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum PreloopStageBFunctionActivationLedgerErrorV1 {
    SelectedCallerNotObserved,
}

#[derive(Debug)]
pub(super) struct PreloopStageBFunctionActivationLedgerV1 {
    armed: PreparedPreloopStageBActivationLedgerPartsV1,
}

impl PreloopStageBFunctionActivationLedgerV1 {
    pub(super) const fn armed(armed: PreparedPreloopStageBActivationLedgerPartsV1) -> Self {
        Self { armed }
    }

    pub(super) fn context(&self) -> &InstalledPreloopStageBContextV1 {
        self.armed.context()
    }

    pub(super) fn row(&self) -> &OwnedPreloopStageBCarrierRowV1 {
        self.armed.row()
    }

    pub(super) fn finish(self) -> RejectedPreloopStageBFunctionActivationV1 {
        RejectedPreloopStageBFunctionActivationV1 {
            armed: self.armed,
            cause: PreloopStageBFunctionActivationLedgerErrorV1::SelectedCallerNotObserved,
        }
    }
}

#[derive(Debug)]
pub(super) struct RejectedPreloopStageBFunctionActivationV1 {
    armed: PreparedPreloopStageBActivationLedgerPartsV1,
    cause: PreloopStageBFunctionActivationLedgerErrorV1,
}

impl RejectedPreloopStageBFunctionActivationV1 {
    pub(super) const fn cause(&self) -> PreloopStageBFunctionActivationLedgerErrorV1 {
        self.cause
    }

    pub(super) fn row(&self) -> &OwnedPreloopStageBCarrierRowV1 {
        self.armed.row()
    }

    pub(super) fn discard(self) {
        let _ = self.armed;
    }
}
