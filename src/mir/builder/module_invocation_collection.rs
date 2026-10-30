//! CUT0-I0-COLLECT0-S0: raw/canonical-single source and collector co-seal.
//!
//! The source wrappers below own real sealed ledger/header authority. The
//! physical collector and receipts remain the only row owners; this terminal
//! checks brand, family, cardinality, key, symbol, arity, policy, and raw
//! replacement history before issuing a collected set. It has no production
//! caller until the later all-route cutover.

use super::module_draft_collector::{
    CollectedDraftAdmissionReceiptV1, CompletedDraftSignatureViewV1, DraftPublicationPolicyV1,
    FunctionDraftKeyV1, ModuleDraftCollectorV1,
};
use super::module_invocation_identity::{
    ModuleInvocationBrandV1, ModuleInvocationFamilyV1, ModuleInvocationTokenV1,
};
use super::module_invocation_owner_chain::{BrandedCollectorV1, InvocationBranded};
use super::raw_expansion_receipt_ledger::{
    RawExpansionReplacementEventV1, SealedRawExpansionReceiptLedgerV1,
};
#[cfg(test)]
use crate::mir::compiler::capability::ResolvedOwnerHeaderFamilyV1;
use crate::mir::compiler::capability::VerifiedResolvedOwnerHeaderV1;

#[derive(Debug)]
pub(in crate::mir::builder) struct RawCollectionSourcePayloadV1 {
    token: ModuleInvocationTokenV1,
    ledger: SealedRawExpansionReceiptLedgerV1,
}

#[derive(Debug)]
pub(in crate::mir::builder) struct CanonicalSingleCollectionSourcePayloadV1 {
    token: ModuleInvocationTokenV1,
    header: VerifiedResolvedOwnerHeaderV1,
}

pub(in crate::mir::builder) type RawInvocationSourceProofV1 =
    InvocationBranded<RawCollectionSourcePayloadV1>;
pub(in crate::mir::builder) type CanonicalSingleInvocationSourceProofV1 =
    InvocationBranded<CanonicalSingleCollectionSourcePayloadV1>;
pub(in crate::mir::builder) type InvocationPhysicalReceiptV1 =
    InvocationBranded<CollectedDraftAdmissionReceiptV1>;

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) enum InvocationCollectionSealErrorV1 {
    SourceFamilyMismatch {
        expected: ModuleInvocationFamilyV1,
        actual: ModuleInvocationFamilyV1,
    },
    ForeignOwner {
        expected: u64,
        actual: u64,
    },
    CardinalityMismatch {
        expected: usize,
        actual: usize,
    },
    MissingRow {
        symbol: String,
    },
    SurplusRow {
        symbol: String,
    },
    KeyMismatch {
        symbol: String,
    },
    SymbolMismatch {
        expected: String,
        actual: String,
    },
    ArityMismatch {
        symbol: String,
        expected: usize,
        actual: usize,
    },
    PolicyMismatch {
        symbol: String,
    },
    ReplacementHistoryMismatch {
        symbol: String,
    },
}

impl std::fmt::Display for InvocationCollectionSealErrorV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "[freeze:contract][invocation_collection] {self:?}"
        )
    }
}

impl std::error::Error for InvocationCollectionSealErrorV1 {}

#[derive(Debug)]
pub(in crate::mir::builder) struct RawCollectedInvocationDraftSetV1 {
    source: RawInvocationSourceProofV1,
    collector: BrandedCollectorV1<ModuleDraftCollectorV1>,
    receipts: Box<[InvocationPhysicalReceiptV1]>,
    _seal: RawCollectedInvocationDraftSetSealV1,
}

impl RawCollectedInvocationDraftSetV1 {
    pub(in crate::mir::builder) fn receipt_count(&self) -> usize {
        self.receipts.len()
    }
}

#[derive(Debug)]
struct RawCollectedInvocationDraftSetSealV1;

fn check_brand(
    expected: ModuleInvocationBrandV1,
    actual: ModuleInvocationBrandV1,
) -> Result<(), InvocationCollectionSealErrorV1> {
    if expected.same(actual) {
        Ok(())
    } else {
        Err(InvocationCollectionSealErrorV1::ForeignOwner {
            expected: expected.ordinal(),
            actual: actual.ordinal(),
        })
    }
}

#[cfg(test)]
pub(in crate::mir::builder) fn raw_source_from_parts(
    token: ModuleInvocationTokenV1,
    ledger: SealedRawExpansionReceiptLedgerV1,
) -> Result<RawInvocationSourceProofV1, InvocationCollectionSealErrorV1> {
    if token.family() != ModuleInvocationFamilyV1::Raw {
        return Err(InvocationCollectionSealErrorV1::SourceFamilyMismatch {
            expected: ModuleInvocationFamilyV1::Raw,
            actual: token.family(),
        });
    }
    let brand = token.brand();
    Ok(InvocationBranded::from_test(
        brand,
        RawCollectionSourcePayloadV1 { token, ledger },
    ))
}

#[cfg(test)]
pub(in crate::mir::builder) fn canonical_source_from_parts(
    token: ModuleInvocationTokenV1,
    header: VerifiedResolvedOwnerHeaderV1,
) -> Result<CanonicalSingleInvocationSourceProofV1, InvocationCollectionSealErrorV1> {
    let expected = match token.family() {
        ModuleInvocationFamilyV1::CanonicalAPlus => {
            ResolvedOwnerHeaderFamilyV1::CurrentCanonicalAPlus
        }
        ModuleInvocationFamilyV1::BindingSsaTrivial => {
            ResolvedOwnerHeaderFamilyV1::TrivialBindingSsa
        }
        actual => {
            return Err(InvocationCollectionSealErrorV1::SourceFamilyMismatch {
                expected: ModuleInvocationFamilyV1::CanonicalAPlus,
                actual,
            })
        }
    };
    if header.family() != expected {
        return Err(InvocationCollectionSealErrorV1::SourceFamilyMismatch {
            expected: token.family(),
            actual: match header.family() {
                ResolvedOwnerHeaderFamilyV1::CurrentCanonicalAPlus => {
                    ModuleInvocationFamilyV1::CanonicalAPlus
                }
                ResolvedOwnerHeaderFamilyV1::TrivialBindingSsa => {
                    ModuleInvocationFamilyV1::BindingSsaTrivial
                }
            },
        });
    }
    let brand = token.brand();
    Ok(InvocationBranded::from_test(
        brand,
        CanonicalSingleCollectionSourcePayloadV1 { token, header },
    ))
}

#[cfg(test)]
pub(in crate::mir::builder) fn physical_receipt_from_test(
    brand: ModuleInvocationBrandV1,
    receipt: CollectedDraftAdmissionReceiptV1,
) -> InvocationPhysicalReceiptV1 {
    InvocationBranded::from_test(brand, receipt)
}

fn physical_receipt(receipt: &InvocationPhysicalReceiptV1) -> &CollectedDraftAdmissionReceiptV1 {
    receipt.payload()
}

pub(in crate::mir::builder) fn seal_raw(
    source: RawInvocationSourceProofV1,
    collector: BrandedCollectorV1<ModuleDraftCollectorV1>,
    receipts: Vec<InvocationPhysicalReceiptV1>,
) -> Result<RawCollectedInvocationDraftSetV1, InvocationCollectionSealErrorV1> {
    let brand = source.brand();
    check_brand(brand, collector.brand())?;
    for receipt in &receipts {
        check_brand(brand, receipt.brand())?;
    }
    let expected_count = source.payload().ledger.final_count();
    let actual_count = collector.payload().symbol_count();
    if expected_count != actual_count || expected_count != receipts.len() {
        return Err(InvocationCollectionSealErrorV1::CardinalityMismatch {
            expected: expected_count,
            actual: receipts.len(),
        });
    }
    for receipt in &receipts {
        let physical = physical_receipt(receipt);
        let symbol = physical.symbol().to_owned();
        let event = source
            .payload()
            .ledger
            .final_event_for_symbol(&symbol)
            .ok_or_else(|| InvocationCollectionSealErrorV1::MissingRow {
                symbol: symbol.clone(),
            })?;
        if collector.payload().key_for_symbol(&symbol) != Some(physical.key())
            || event.key() != physical.key()
        {
            return Err(InvocationCollectionSealErrorV1::KeyMismatch { symbol });
        }
        if event.symbol() != physical.symbol() {
            return Err(InvocationCollectionSealErrorV1::SymbolMismatch {
                expected: event.symbol().to_owned(),
                actual: physical.symbol().to_owned(),
            });
        }
        if event.arity() != physical.arity() {
            return Err(InvocationCollectionSealErrorV1::ArityMismatch {
                symbol,
                expected: event.arity(),
                actual: physical.arity(),
            });
        }
        if event.policy() != physical.policy() {
            return Err(InvocationCollectionSealErrorV1::PolicyMismatch { symbol });
        }
        if !replacement_matches(event.replacement(), physical.replacement()) {
            return Err(InvocationCollectionSealErrorV1::ReplacementHistoryMismatch { symbol });
        }
    }
    Ok(RawCollectedInvocationDraftSetV1 {
        source,
        collector,
        receipts: receipts.into_boxed_slice(),
        _seal: RawCollectedInvocationDraftSetSealV1,
    })
}

fn replacement_matches(
    event: &RawExpansionReplacementEventV1,
    receipt: &super::module_draft_collector::CollectedDraftReplacementDispositionV1,
) -> bool {
    match (event, receipt) {
        (
            RawExpansionReplacementEventV1::Inserted,
            super::module_draft_collector::CollectedDraftReplacementDispositionV1::Inserted,
        ) => true,
        (
            RawExpansionReplacementEventV1::ReplacedWholePair {
                previous_key,
                previous_symbol,
            },
            super::module_draft_collector::CollectedDraftReplacementDispositionV1::ReplacedWholePair {
                previous_key: receipt_key,
                previous_symbol: receipt_symbol,
            },
        ) => previous_key == receipt_key && previous_symbol.as_ref() == receipt_symbol.as_ref(),
        _ => false,
    }
}
