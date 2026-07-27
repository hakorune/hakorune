//! Thin compiler handoff to the builder-owned exact-function ledger.
//!
//! The transition authority lives in `PreparedPreloopStageBFunctionActivationV1`.
//! This wrapper only preserves the selected module-install boundary.

use crate::mir::builder::{
    preloop_stageb_context_install::InstalledPreloopStageBContextV1,
    PreparedPreloopStageBFunctionActivationV1,
};
use crate::mir::preloop_stageb_carrier::{
    OwnedPreloopStageBCarrierRowV1, PreparedPreloopStageBActivationLedgerPartsV1,
};

#[derive(Debug)]
pub(super) struct PreloopStageBFunctionActivationLedgerV1 {
    prepared: PreparedPreloopStageBFunctionActivationV1,
}

impl PreloopStageBFunctionActivationLedgerV1 {
    pub(super) fn armed(armed: PreparedPreloopStageBActivationLedgerPartsV1) -> Self {
        Self {
            prepared: PreparedPreloopStageBFunctionActivationV1::armed(armed),
        }
    }

    pub(super) fn context(&self) -> &InstalledPreloopStageBContextV1 {
        self.prepared.context()
    }

    pub(super) fn row(&self) -> &OwnedPreloopStageBCarrierRowV1 {
        self.prepared.row()
    }

    pub(super) fn into_prepared(self) -> PreparedPreloopStageBFunctionActivationV1 {
        self.prepared
    }
}
