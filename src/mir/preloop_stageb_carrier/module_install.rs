//! Named consuming projection from one source activation plan into the
//! selected candidate-context transaction.
//!
//! The projection keeps the exact catalog and armed row paired. It exposes no
//! generic tuple, catalog reseal, root lowering, retry, or fallback terminal.

use crate::mir::builder::preloop_stageb_context_install::{
    InstalledPreloopStageBContextV1, PreparedPreloopStageBAliasInstallV1,
    PreparedPreloopStageBContextInstallV1, RejectedPreloopStageBContextInstallV1,
};
use crate::mir::builder::{MirBuilder, VerifiedSameModuleCallableDeclarationCatalogV1};

use super::activation::OwnedPreloopStageBCarrierRowV1;

#[derive(Debug)]
pub(in crate::mir) struct PreparedPreloopStageBActivationInstallPartsV1 {
    catalog: VerifiedSameModuleCallableDeclarationCatalogV1,
    row: OwnedPreloopStageBCarrierRowV1,
}

impl PreparedPreloopStageBActivationInstallPartsV1 {
    pub(super) const fn new(
        catalog: VerifiedSameModuleCallableDeclarationCatalogV1,
        row: OwnedPreloopStageBCarrierRowV1,
    ) -> Self {
        Self { catalog, row }
    }

    pub(in crate::mir) fn attach_aliases(
        self,
        aliases: PreparedPreloopStageBAliasInstallV1,
    ) -> PreparedPreloopStageBActivationContextInstallV1 {
        PreparedPreloopStageBActivationContextInstallV1 {
            context: PreparedPreloopStageBContextInstallV1::new(self.catalog, aliases),
            row: self.row,
        }
    }
}

#[derive(Debug)]
pub(in crate::mir) struct PreparedPreloopStageBActivationContextInstallV1 {
    context: PreparedPreloopStageBContextInstallV1,
    row: OwnedPreloopStageBCarrierRowV1,
}

impl PreparedPreloopStageBActivationContextInstallV1 {
    pub(in crate::mir) fn commit(
        self,
        builder: &mut MirBuilder,
    ) -> Result<
        InstalledPreloopStageBActivationContextPartsV1,
        RejectedPreloopStageBActivationContextInstallV1,
    > {
        match self.context.commit(builder) {
            Ok(context) => Ok(InstalledPreloopStageBActivationContextPartsV1 {
                context,
                row: self.row,
            }),
            Err(context) => Err(RejectedPreloopStageBActivationContextInstallV1 {
                context,
                row: self.row,
            }),
        }
    }
}

#[derive(Debug)]
pub(in crate::mir) struct InstalledPreloopStageBActivationContextPartsV1 {
    context: InstalledPreloopStageBContextV1,
    row: OwnedPreloopStageBCarrierRowV1,
}

impl InstalledPreloopStageBActivationContextPartsV1 {
    pub(in crate::mir) const fn context(&self) -> &InstalledPreloopStageBContextV1 {
        &self.context
    }

    pub(in crate::mir) const fn row(&self) -> &OwnedPreloopStageBCarrierRowV1 {
        &self.row
    }

    pub(in crate::mir) fn into_ledger_parts(self) -> PreparedPreloopStageBActivationLedgerPartsV1 {
        PreparedPreloopStageBActivationLedgerPartsV1 {
            context: self.context,
            row: self.row,
        }
    }
}

#[derive(Debug)]
pub(in crate::mir) struct PreparedPreloopStageBActivationLedgerPartsV1 {
    context: InstalledPreloopStageBContextV1,
    row: OwnedPreloopStageBCarrierRowV1,
}

impl PreparedPreloopStageBActivationLedgerPartsV1 {
    pub(in crate::mir) fn context(&self) -> &InstalledPreloopStageBContextV1 {
        &self.context
    }

    pub(in crate::mir) fn row(&self) -> &OwnedPreloopStageBCarrierRowV1 {
        &self.row
    }
}

#[derive(Debug)]
pub(in crate::mir) struct RejectedPreloopStageBActivationContextInstallV1 {
    context: RejectedPreloopStageBContextInstallV1,
    row: OwnedPreloopStageBCarrierRowV1,
}

impl RejectedPreloopStageBActivationContextInstallV1 {
    pub(in crate::mir) fn bounded_report(&self) -> Box<str> {
        format!(
            "[mir/preloop-stageb/activation-context/{:?}] caller={:?}",
            self.context.cause(),
            self.row.caller()
        )
        .into_boxed_str()
    }

    pub(in crate::mir) fn discard(self) {
        self.context.discard();
        let _ = self.row;
    }
}
