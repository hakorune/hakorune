//! RAW-SOURCE0-LOWER0-ROOT0-OWNER0-PHYSICAL0.
//!
//! This module opens only the empty physical carrier for an eligible Raw
//! package. It deliberately does not lower a child or a root body.

use super::module_declaration_facts::SealedModuleDeclarationFactsV1;
use super::module_invocation_brand0::InvocationPhysicalStateV1;
use super::module_invocation_identity::ModuleInvocationTokenV1;
use super::module_lowering_shell::ModuleLoweringShellErrorV1;
use super::raw_expansion_receipt_ledger::{
    RawCallableMainCompatibilityDispositionV1, RawExpansionReceiptLedgerV1,
};
use super::root_body_completion::{
    ActiveRootBodyCompletionTrackerV1, CompletedRootBodyV1, RootBodyCompletionErrorV1,
    RootBodyCompletionTrackerV1, RootBodyResultV1,
};
use crate::mir::builder::module_invocation_identity::ModuleInvocationBrandV1;

pub(in crate::mir) mod callable_main_terminal;
pub(in crate::mir) mod child_terminal;
pub(in crate::mir) mod drain_manifest;
pub(in crate::mir) mod drain_terminal;
pub(in crate::mir) mod environment_terminal;
pub(in crate::mir) mod finalization_terminal;
pub(in crate::mir) mod postprocess_terminal;
pub(in crate::mir) mod root_batch_terminal;

#[derive(Debug)]
enum RawRootLedgerStateV1 {
    Open(RawExpansionReceiptLedgerV1),
    Aborted(super::raw_expansion_receipt_ledger::AbortedRawExpansionReceiptLedgerV1),
    AbortedPlaceholder,
}

#[derive(Debug)]
pub(in crate::mir) struct RawRootPhysicalStateV1 {
    physical: InvocationPhysicalStateV1,
    ledger: RawRootLedgerStateV1,
    tracker: RootBodyCompletionTrackerV1,
    callable_main: RawCallableMainCompatibilityDispositionV1,
}

/// BODY0-only physical transition.  The collector and ledger remain owned by
/// this product but are never borrowed or mutated by the root-body driver.
#[derive(Debug)]
pub(in crate::mir) struct RawRootBodyPhysicalDriveV1 {
    physical: InvocationPhysicalStateV1,
    ledger: RawRootLedgerStateV1,
    tracker: ActiveRootBodyCompletionTrackerV1,
    callable_main: RawCallableMainCompatibilityDispositionV1,
}

#[derive(Debug)]
pub(in crate::mir) struct RawRootPostBodyPhysicalStateV1 {
    physical: InvocationPhysicalStateV1,
    ledger: RawRootLedgerStateV1,
    callable_main: RawCallableMainCompatibilityDispositionV1,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) enum RawRootBodyPhysicalErrorV1 {
    BeginTracker(RootBodyCompletionErrorV1),
    SealTracker(RootBodyCompletionErrorV1),
}

impl RawRootPhysicalStateV1 {
    pub(in crate::mir::builder) fn begin_root_body(
        self,
    ) -> Result<RawRootBodyPhysicalDriveV1, (Self, RawRootBodyPhysicalErrorV1)> {
        let Self {
            physical,
            ledger,
            tracker,
            callable_main,
        } = self;
        let tracker = match tracker.begin_root_body_preserving() {
            Ok(tracker) => tracker,
            Err((tracker, error)) => {
                return Err((
                    Self {
                        physical,
                        ledger,
                        tracker,
                        callable_main,
                    },
                    RawRootBodyPhysicalErrorV1::BeginTracker(error),
                ));
            }
        };
        Ok(RawRootBodyPhysicalDriveV1 {
            physical,
            ledger,
            tracker,
            callable_main,
        })
    }

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
            ledger: RawRootLedgerStateV1::Open(ledger),
            tracker,
            callable_main,
        })
    }

    pub(in crate::mir) fn brand(&self) -> ModuleInvocationBrandV1 {
        self.physical.brand()
    }

    pub(in crate::mir) fn shell_is_empty(&self) -> bool {
        !self.physical.shell().payload().has_published_functions()
    }

    pub(in crate::mir) fn published_function_count(&self) -> usize {
        self.physical.shell().payload().published_function_count()
    }

    pub(in crate::mir) fn ledger_brand(&self) -> ModuleInvocationBrandV1 {
        match &self.ledger {
            RawRootLedgerStateV1::Open(ledger) => ledger.brand(),
            RawRootLedgerStateV1::Aborted(ledger) => ledger.brand(),
            RawRootLedgerStateV1::AbortedPlaceholder => self.physical.brand(),
        }
    }

    pub(in crate::mir) fn tracker_brand(&self) -> ModuleInvocationBrandV1 {
        self.tracker.brand()
    }

    pub(in crate::mir) fn tracker_completed_children(&self) -> usize {
        self.tracker.completed_children()
    }

    pub(in crate::mir::builder) fn environment_lanes_are_vacant(&self) -> bool {
        matches!(&self.ledger, RawRootLedgerStateV1::Open(ledger) if ledger.is_clean_open())
            && self.tracker.is_fresh()
            && self.physical.environment_lanes_are_vacant()
    }

    pub(in crate::mir::builder) fn install_environment_preflighted(
        self,
        facts: SealedModuleDeclarationFactsV1,
        source_file: Option<Box<str>>,
    ) -> Self {
        let Self {
            physical,
            ledger,
            tracker,
            callable_main,
        } = self;
        Self {
            physical: physical.install_environment_preflighted(facts, source_file),
            ledger,
            tracker,
            callable_main,
        }
    }

    pub(in crate::mir) fn callable_main(&self) -> RawCallableMainCompatibilityDispositionV1 {
        self.callable_main
    }
}

impl RawRootBodyPhysicalDriveV1 {
    pub(in crate::mir::builder) fn seal_root_body(
        self,
        result: RootBodyResultV1,
    ) -> Result<(RawRootPostBodyPhysicalStateV1, CompletedRootBodyV1), RawRootBodyPhysicalErrorV1>
    {
        self.seal_root_body_preserving(result)
            .map_err(|(_, error)| error)
    }

    pub(in crate::mir::builder) fn seal_root_body_preserving(
        self,
        result: RootBodyResultV1,
    ) -> Result<
        (RawRootPostBodyPhysicalStateV1, CompletedRootBodyV1),
        (Self, RawRootBodyPhysicalErrorV1),
    > {
        let Self {
            physical,
            ledger,
            tracker,
            callable_main,
        } = self;
        let completed = match tracker.seal_root_body_preserving(result) {
            Ok(completed) => completed,
            Err((_tracker, error)) => {
                return Err((
                    Self {
                        physical,
                        ledger,
                        tracker: _tracker,
                        callable_main,
                    },
                    RawRootBodyPhysicalErrorV1::SealTracker(error),
                ));
            }
        };
        Ok((
            RawRootPostBodyPhysicalStateV1 {
                physical,
                ledger,
                callable_main,
            },
            completed,
        ))
    }

    pub(in crate::mir::builder) fn brand(&self) -> ModuleInvocationBrandV1 {
        self.physical.brand()
    }
}

impl RawRootPostBodyPhysicalStateV1 {
    pub(in crate::mir::builder) fn brand(&self) -> ModuleInvocationBrandV1 {
        self.physical.brand()
    }

    pub(in crate::mir::builder) fn shell_is_empty(&self) -> bool {
        !self.physical.shell().payload().has_published_functions()
    }

    pub(in crate::mir::builder) fn published_function_count(&self) -> usize {
        self.physical.shell().payload().published_function_count()
    }

    pub(in crate::mir::builder) const fn callable_main(
        &self,
    ) -> RawCallableMainCompatibilityDispositionV1 {
        self.callable_main
    }

    pub(in crate::mir::builder) fn collector_and_ledger_untouched(&self) -> bool {
        matches!(&self.ledger, RawRootLedgerStateV1::Open(ledger) if ledger.is_clean_open())
    }

    pub(in crate::mir::builder) fn open_ledger(&self) -> Option<&RawExpansionReceiptLedgerV1> {
        match &self.ledger {
            RawRootLedgerStateV1::Open(ledger) => Some(ledger),
            RawRootLedgerStateV1::Aborted(_) | RawRootLedgerStateV1::AbortedPlaceholder => None,
        }
    }

    pub(in crate::mir::builder) fn collector(
        &self,
    ) -> &super::module_invocation_owner_chain::BrandedCollectorV1<
        super::module_draft_collector::ModuleDraftCollectorV1,
    > {
        self.physical.collector()
    }

    pub(in crate::mir::builder) fn into_parts(
        self,
    ) -> (
        InvocationPhysicalStateV1,
        RawRootLedgerStateV1,
        RawCallableMainCompatibilityDispositionV1,
    ) {
        (self.physical, self.ledger, self.callable_main)
    }
}
