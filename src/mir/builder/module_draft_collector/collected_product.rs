//! RECEIPT0: collector-issued single-draft receipt product.
//!
//! The collector and its exact physical receipt are moved together.  This
//! prevents a completion caller from pairing a receipt with a different
//! collector or from branding a receipt after collection.

use super::{
    CollectedDraftAdmissionReceiptV1, DraftPublicationPolicyV1, FunctionDraftKeyV1,
    ModuleDraftAdmissionErrorV1, ModuleDraftCollectorV1,
};
use crate::mir::builder::module_invocation_owner_chain::{
    BrandedCollectorV1, InvocationBranded,
};
use crate::mir::MirFunction;

#[derive(Debug)]
pub(in crate::mir::builder) struct CollectedDraftAdmissionProductV1 {
    collector: BrandedCollectorV1<ModuleDraftCollectorV1>,
    receipt: InvocationBranded<CollectedDraftAdmissionReceiptV1>,
}

#[derive(Debug)]
pub(in crate::mir::builder) struct RejectedCollectedDraftAdmissionV1 {
    collector: BrandedCollectorV1<ModuleDraftCollectorV1>,
    error: CollectedDraftAdmissionProductErrorV1,
}

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) enum CollectedDraftAdmissionProductErrorV1 {
    Admission(ModuleDraftAdmissionErrorV1),
    CollectorUnbranded,
}

impl CollectedDraftAdmissionProductV1 {
    pub(in crate::mir::builder) fn into_parts(
        self,
    ) -> (
        BrandedCollectorV1<ModuleDraftCollectorV1>,
        InvocationBranded<CollectedDraftAdmissionReceiptV1>,
    ) {
        (self.collector, self.receipt)
    }
}

impl RejectedCollectedDraftAdmissionV1 {
    pub(in crate::mir::builder) fn collector(&self) -> &BrandedCollectorV1<ModuleDraftCollectorV1> {
        &self.collector
    }

    pub(in crate::mir::builder) fn error(&self) -> &CollectedDraftAdmissionProductErrorV1 {
        &self.error
    }

    pub(in crate::mir::builder) fn into_parts(
        self,
    ) -> (
        BrandedCollectorV1<ModuleDraftCollectorV1>,
        CollectedDraftAdmissionProductErrorV1,
    ) {
        (self.collector, self.error)
    }
}

impl InvocationBranded<ModuleDraftCollectorV1> {
    /// Canonical single-draft terminal.  All admission checks and receipt
    /// provenance checks complete before the collector/receipt product exists.
    pub(in crate::mir::builder) fn collect_canonical_single(
        self,
        key: FunctionDraftKeyV1,
        symbol: String,
        arity: usize,
        draft: MirFunction,
    ) -> Result<CollectedDraftAdmissionProductV1, RejectedCollectedDraftAdmissionV1> {
        let brand = self.brand();
        let mut collector = self.into_payload();
        if collector.receipt_brand() != Some(brand) {
            return Err(RejectedCollectedDraftAdmissionV1 {
                collector: InvocationBranded::from_source(brand, collector),
                error: CollectedDraftAdmissionProductErrorV1::CollectorUnbranded,
            });
        }
        let admission = match collector.prepare_admission(
            key,
            symbol,
            arity,
            DraftPublicationPolicyV1::CanonicalRejectDuplicate,
        ) {
            Ok(admission) => admission,
            Err(error) => {
                return Err(RejectedCollectedDraftAdmissionV1 {
                    collector: InvocationBranded::from_source(brand, collector),
                    error: CollectedDraftAdmissionProductErrorV1::Admission(error),
                })
            }
        };
        let unpublished = match admission.seal(draft) {
            Ok(unpublished) => unpublished,
            Err(error) => {
                return Err(RejectedCollectedDraftAdmissionV1 {
                    collector: InvocationBranded::from_source(brand, collector),
                    error: CollectedDraftAdmissionProductErrorV1::Admission(error),
                })
            }
        };
        let receipt = unpublished
            .collect_branded()
            .expect("collector brand was preflighted before single collection");
        Ok(CollectedDraftAdmissionProductV1 {
            collector: InvocationBranded::from_source(brand, collector),
            receipt,
        })
    }
}
