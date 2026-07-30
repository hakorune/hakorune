//! CHILDREN0's sole Builder-side static-helper terminal.

use crate::mir::MirBuilder;

use super::super::calls::CanonicalFunctionSessionErrorV1;
use super::super::module_draft_collector::CollectedDraftAdmissionReceiptV1;
use super::super::module_invocation_owner_chain::InvocationBranded;
use super::super::module_lowering_invocation::ModuleLoweringPortChildErrorV1;
use super::super::raw_expansion_receipt_ledger::{
    RawExpansionAbortReasonV1, RawExpansionDraftRequestV1, RawExpansionDraftRoleV1,
    RawExpansionReceiptLedgerErrorV1,
};
use super::super::PreparedRawRootStaticChildDraftV1;
use super::{RawRootLedgerStateV1, RawRootPhysicalStateV1};

#[derive(Debug)]
pub(in crate::mir) enum RawRootPhysicalChildErrorV1 {
    Request(RawExpansionReceiptLedgerErrorV1),
    Reservation(RawExpansionReceiptLedgerErrorV1),
    Child(ModuleLoweringPortChildErrorV1),
    Ledger(RawExpansionReceiptLedgerErrorV1),
    Abort(RawExpansionReceiptLedgerErrorV1),
}

impl RawRootPhysicalStateV1 {
    pub(in crate::mir) fn complete_static_child(
        &mut self,
        builder: &mut MirBuilder,
        work: PreparedRawRootStaticChildDraftV1,
    ) -> Result<InvocationBranded<CollectedDraftAdmissionReceiptV1>, RawRootPhysicalChildErrorV1>
    {
        let request = match RawExpansionDraftRequestV1::legacy_discovered(
            RawExpansionDraftRoleV1::StaticMethod,
            work.symbol().to_owned(),
            work.arity(),
        ) {
            Ok(request) => request,
            Err(error) => {
                return Err(RawRootPhysicalChildErrorV1::Request(error));
            }
        };
        let reservation = match &mut self.ledger {
            RawRootLedgerStateV1::Open(ledger) => match ledger.reserve(request) {
                Ok(reservation) => reservation,
                Err(error) => {
                    return Err(RawRootPhysicalChildErrorV1::Reservation(error));
                }
            },
            RawRootLedgerStateV1::Aborted(_) | RawRootLedgerStateV1::AbortedPlaceholder => {
                return Err(RawRootPhysicalChildErrorV1::Request(
                    RawExpansionReceiptLedgerErrorV1::LedgerPoisoned,
                ));
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
                                return Err(RawRootPhysicalChildErrorV1::Abort(abort_error));
                            }
                        }
                    }
                    RawRootLedgerStateV1::Aborted(_) | RawRootLedgerStateV1::AbortedPlaceholder => {
                        return Err(RawRootPhysicalChildErrorV1::Request(
                            RawExpansionReceiptLedgerErrorV1::LedgerPoisoned,
                        ));
                    }
                };
                self.ledger = RawRootLedgerStateV1::Aborted(aborted);
                return Err(RawRootPhysicalChildErrorV1::Child(error));
            }
        };
        let ledger = match &mut self.ledger {
            RawRootLedgerStateV1::Open(ledger) => ledger,
            RawRootLedgerStateV1::Aborted(_) | RawRootLedgerStateV1::AbortedPlaceholder => {
                return Err(RawRootPhysicalChildErrorV1::Ledger(
                    RawExpansionReceiptLedgerErrorV1::LedgerPoisoned,
                ));
            }
        };
        if let Err(error) = ledger.complete_branded(reservation, &receipt) {
            return Err(RawRootPhysicalChildErrorV1::Ledger(error));
        }
        Ok(receipt)
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
        | ModuleLoweringPortChildErrorV1::ReceiptBrand(_) => RawExpansionAbortReasonV1::Admission,
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::calls::CanonicalFunctionSessionErrorV1;
    use super::super::super::module_draft_collector::{
        CollectorReceiptBrandErrorV1, FunctionDraftKeyV1, ModuleDraftAdmissionErrorV1,
    };
    use super::*;

    #[test]
    fn typed_child_causes_map_to_existing_coarse_abort_reasons() {
        assert_eq!(
            map_abort_reason(&ModuleLoweringPortChildErrorV1::Session(
                CanonicalFunctionSessionErrorV1::Primary("primary".into()),
            )),
            RawExpansionAbortReasonV1::Primary
        );
        assert_eq!(
            map_abort_reason(&ModuleLoweringPortChildErrorV1::Session(
                CanonicalFunctionSessionErrorV1::Cleanup("cleanup".into()),
            )),
            RawExpansionAbortReasonV1::Cleanup
        );
        assert_eq!(
            map_abort_reason(&ModuleLoweringPortChildErrorV1::Session(
                CanonicalFunctionSessionErrorV1::DuringCleanup {
                    primary: "primary".into(),
                    cleanup: "cleanup".into(),
                },
            )),
            RawExpansionAbortReasonV1::Cleanup
        );
        assert_eq!(
            map_abort_reason(&ModuleLoweringPortChildErrorV1::Admission(
                ModuleDraftAdmissionErrorV1::DuplicateKey(FunctionDraftKeyV1::LegacySymbol(
                    "Main.alpha/0".into(),
                )),
            )),
            RawExpansionAbortReasonV1::Admission
        );
        assert_eq!(
            map_abort_reason(&ModuleLoweringPortChildErrorV1::ReceiptBrand(
                CollectorReceiptBrandErrorV1::CollectorUnbranded,
            )),
            RawExpansionAbortReasonV1::Admission
        );
    }
}
