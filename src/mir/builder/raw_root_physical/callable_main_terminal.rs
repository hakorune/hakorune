//! CALLMAIN0 Builder-side terminal.
//!
//! The Raw root physical state remains the only shell/collector/ledger owner.
//! This role-specific terminal consumes that owner, performs one callable-Main
//! admission, and returns a named success or rejection product.

use crate::mir::MirBuilder;

use super::super::calls::CanonicalFunctionSessionErrorV1;
use super::super::module_draft_collector::CollectedDraftAdmissionReceiptV1;
use super::super::module_invocation_owner_chain::InvocationBranded;
use super::super::module_lowering_invocation::ModuleLoweringPortChildErrorV1;
use super::super::raw_expansion_receipt_ledger::{
    RawExpansionAbortReasonV1, RawExpansionDraftRequestV1, RawExpansionReceiptLedgerErrorV1,
};
use super::super::RawCallableMainWorkV1;
use super::{RawRootLedgerStateV1, RawRootPhysicalStateV1};

#[derive(Debug)]
pub(in crate::mir) enum RawRootPhysicalCallableMainErrorV1 {
    Request(RawExpansionReceiptLedgerErrorV1),
    Reservation(RawExpansionReceiptLedgerErrorV1),
    Child(ModuleLoweringPortChildErrorV1),
    Ledger(RawExpansionReceiptLedgerErrorV1),
    Abort(RawExpansionReceiptLedgerErrorV1),
}

#[derive(Debug)]
pub(in crate::mir) struct CompletedRawCallableMainPhysicalV1 {
    physical: RawRootPhysicalStateV1,
    receipt: InvocationBranded<CollectedDraftAdmissionReceiptV1>,
}

impl CompletedRawCallableMainPhysicalV1 {
    pub(in crate::mir) fn into_parts(
        self,
    ) -> (
        RawRootPhysicalStateV1,
        InvocationBranded<CollectedDraftAdmissionReceiptV1>,
    ) {
        (self.physical, self.receipt)
    }
}

#[derive(Debug)]
pub(in crate::mir) struct RejectedRawCallableMainPhysicalV1 {
    physical: RawRootPhysicalStateV1,
    issued_receipt: Option<InvocationBranded<CollectedDraftAdmissionReceiptV1>>,
    error: RawRootPhysicalCallableMainErrorV1,
}

impl RejectedRawCallableMainPhysicalV1 {
    pub(in crate::mir) fn into_parts(
        self,
    ) -> (
        RawRootPhysicalStateV1,
        Option<InvocationBranded<CollectedDraftAdmissionReceiptV1>>,
        RawRootPhysicalCallableMainErrorV1,
    ) {
        (self.physical, self.issued_receipt, self.error)
    }
}

impl RawRootPhysicalStateV1 {
    pub(in crate::mir) fn complete_callable_main(
        mut self,
        builder: &mut MirBuilder,
        work: RawCallableMainWorkV1,
    ) -> Result<CompletedRawCallableMainPhysicalV1, RejectedRawCallableMainPhysicalV1> {
        let work = work.into_callable_main_draft();
        let request = match RawExpansionDraftRequestV1::callable_main_compatibility(
            work.symbol().to_owned(),
            work.arity(),
        ) {
            Ok(request) => request,
            Err(error) => {
                return Err(rejected(
                    self,
                    None,
                    RawRootPhysicalCallableMainErrorV1::Request(error),
                ))
            }
        };
        let reservation = match &mut self.ledger {
            RawRootLedgerStateV1::Open(ledger) => match ledger.reserve(request) {
                Ok(reservation) => reservation,
                Err(error) => {
                    return Err(rejected(
                        self,
                        None,
                        RawRootPhysicalCallableMainErrorV1::Reservation(error),
                    ))
                }
            },
            RawRootLedgerStateV1::Aborted(_) | RawRootLedgerStateV1::AbortedPlaceholder => {
                return Err(rejected(
                    self,
                    None,
                    RawRootPhysicalCallableMainErrorV1::Request(
                        RawExpansionReceiptLedgerErrorV1::LedgerPoisoned,
                    ),
                ))
            }
        };
        let receipt = match self.physical.complete_raw_static_child(builder, work) {
            Ok(receipt) => receipt,
            Err(error) => {
                let aborted = match std::mem::replace(
                    &mut self.ledger,
                    RawRootLedgerStateV1::AbortedPlaceholder,
                ) {
                    RawRootLedgerStateV1::Open(ledger) => {
                        match ledger.abort(reservation, map_abort_reason(&error)) {
                            Ok(aborted) => aborted,
                            Err(abort_error) => {
                                return Err(rejected(
                                    self,
                                    None,
                                    RawRootPhysicalCallableMainErrorV1::Abort(abort_error),
                                ))
                            }
                        }
                    }
                    RawRootLedgerStateV1::Aborted(_) | RawRootLedgerStateV1::AbortedPlaceholder => {
                        return Err(rejected(
                            self,
                            None,
                            RawRootPhysicalCallableMainErrorV1::Request(
                                RawExpansionReceiptLedgerErrorV1::LedgerPoisoned,
                            ),
                        ))
                    }
                };
                self.ledger = RawRootLedgerStateV1::Aborted(aborted);
                return Err(rejected(
                    self,
                    None,
                    RawRootPhysicalCallableMainErrorV1::Child(error),
                ));
            }
        };
        if let Err(error) = match &mut self.ledger {
            RawRootLedgerStateV1::Open(ledger) => ledger.complete_branded(reservation, &receipt),
            RawRootLedgerStateV1::Aborted(_) | RawRootLedgerStateV1::AbortedPlaceholder => {
                return Err(rejected(
                    self,
                    Some(receipt),
                    RawRootPhysicalCallableMainErrorV1::Ledger(
                        RawExpansionReceiptLedgerErrorV1::LedgerPoisoned,
                    ),
                ))
            }
        } {
            return Err(rejected(
                self,
                Some(receipt),
                RawRootPhysicalCallableMainErrorV1::Ledger(error),
            ));
        }
        Ok(CompletedRawCallableMainPhysicalV1 {
            physical: self,
            receipt,
        })
    }
}

fn rejected(
    physical: RawRootPhysicalStateV1,
    issued_receipt: Option<InvocationBranded<CollectedDraftAdmissionReceiptV1>>,
    error: RawRootPhysicalCallableMainErrorV1,
) -> RejectedRawCallableMainPhysicalV1 {
    RejectedRawCallableMainPhysicalV1 {
        physical,
        issued_receipt,
        error,
    }
}

fn map_abort_reason(error: &ModuleLoweringPortChildErrorV1) -> RawExpansionAbortReasonV1 {
    match error {
        ModuleLoweringPortChildErrorV1::Session(CanonicalFunctionSessionErrorV1::Primary(_)) => {
            RawExpansionAbortReasonV1::Primary
        }
        ModuleLoweringPortChildErrorV1::Session(CanonicalFunctionSessionErrorV1::Cleanup(_))
        | ModuleLoweringPortChildErrorV1::Session(
            CanonicalFunctionSessionErrorV1::DuringCleanup { .. },
        ) => RawExpansionAbortReasonV1::Cleanup,
        ModuleLoweringPortChildErrorV1::Session(CanonicalFunctionSessionErrorV1::Publication(
            _,
        ))
        | ModuleLoweringPortChildErrorV1::Admission(_)
        | ModuleLoweringPortChildErrorV1::ReceiptBrand(_)
        | ModuleLoweringPortChildErrorV1::PhysicalSignatureMismatch
        | ModuleLoweringPortChildErrorV1::PinnedTextBackendFrameContractMismatch => {
            RawExpansionAbortReasonV1::Admission
        }
    }
}
