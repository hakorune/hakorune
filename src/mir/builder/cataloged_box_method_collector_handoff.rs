//! F-COLLECTOR-PORT-R1: one selected Box-method draft-to-collector terminal.
//!
//! The completed draft already carries the catalog admission projection. This
//! port only performs the existing invocation-owned prepare/seal/collect
//! transition; it does not publish a module or reinterpret the callable key.

use super::module_draft_collector::{CollectedDraftAdmissionReceiptV1, DraftPublicationPolicyV1};
use super::module_invocation_owner_chain::InvocationBranded;
use super::module_lowering_invocation::{ModuleLoweringPortChildErrorV1, ModuleLoweringPortV1};
use super::resolved_lowering::CompletedCatalogedBoxCallableDraftV1;

impl ModuleLoweringPortV1<'_> {
    /// Consume one completed selected Box-method draft through this
    /// invocation's collector. The canonical policy is fixed and the receipt
    /// is branded by the collector owner; no publication or drain occurs here.
    pub(in crate::mir::builder) fn commit_cataloged_box_method_completed(
        &mut self,
        completed: CompletedCatalogedBoxCallableDraftV1,
    ) -> Result<InvocationBranded<CollectedDraftAdmissionReceiptV1>, ModuleLoweringPortChildErrorV1>
    {
        let (key, symbol, arity, draft) = completed.into_collector_parts();
        let prepared = self
            .prepare_draft_admission(
                key,
                symbol,
                arity,
                DraftPublicationPolicyV1::CanonicalRejectDuplicate,
            )
            .map_err(ModuleLoweringPortChildErrorV1::Admission)?;
        prepared
            .seal(draft)
            .map_err(ModuleLoweringPortChildErrorV1::Admission)?
            .collect_branded()
            .map_err(ModuleLoweringPortChildErrorV1::ReceiptBrand)
    }
}
