//! RAW-SOURCE0-LOWER0-ROOT0-OWNER0-PHYSICAL0.
//!
//! This module opens only the empty physical carrier for an eligible Raw
//! package. It deliberately does not lower a child or a root body.

use super::module_invocation_brand0::InvocationPhysicalStateV1;
use super::module_invocation_identity::ModuleInvocationTokenV1;
use super::module_lowering_shell::ModuleLoweringShellErrorV1;
use super::raw_expansion_receipt_ledger::{
    RawCallableMainCompatibilityDispositionV1, RawExpansionReceiptLedgerV1,
};
use super::root_body_completion::RootBodyCompletionTrackerV1;
use crate::mir::builder::module_invocation_identity::ModuleInvocationBrandV1;

#[derive(Debug)]
pub(in crate::mir) struct RawRootPhysicalStateV1 {
    physical: InvocationPhysicalStateV1,
    ledger: RawExpansionReceiptLedgerV1,
    tracker: RootBodyCompletionTrackerV1,
}

impl RawRootPhysicalStateV1 {
    pub(in crate::mir) fn open(
        token: &ModuleInvocationTokenV1,
        module_name: String,
        callable_main: RawCallableMainCompatibilityDispositionV1,
    ) -> Result<Self, ModuleLoweringShellErrorV1> {
        let physical = InvocationPhysicalStateV1::from_token(token, module_name)?;
        let ledger = RawExpansionReceiptLedgerV1::new_for_token(token, callable_main);
        let tracker = RootBodyCompletionTrackerV1::new_for_brand(token.brand());
        debug_assert_eq!(physical.brand(), token.brand());
        debug_assert_eq!(ledger.brand(), token.brand());
        Ok(Self {
            physical,
            ledger,
            tracker,
        })
    }

    pub(in crate::mir) fn brand(&self) -> ModuleInvocationBrandV1 {
        self.physical.brand()
    }

    pub(in crate::mir) fn shell_is_empty(&self) -> bool {
        !self.physical.shell().payload().has_published_functions()
    }

    pub(in crate::mir) fn ledger_brand(&self) -> ModuleInvocationBrandV1 {
        self.ledger.brand()
    }

    pub(in crate::mir) fn tracker_brand(&self) -> ModuleInvocationBrandV1 {
        self.tracker.brand()
    }

    pub(in crate::mir) fn callable_main(&self) -> RawCallableMainCompatibilityDispositionV1 {
        self.ledger.callable_main()
    }

    pub(in crate::mir) fn into_parts(
        self,
    ) -> (
        InvocationPhysicalStateV1,
        RawExpansionReceiptLedgerV1,
        RootBodyCompletionTrackerV1,
    ) {
        (self.physical, self.ledger, self.tracker)
    }
}
