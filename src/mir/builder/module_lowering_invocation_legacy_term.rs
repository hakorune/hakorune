//! Raw/legacy child completion terminals split from the main invocation port.
//!
//! Keeping these terminals in a sibling keeps the shared port vocabulary below
//! the source-file size cap while preserving one collector admission owner.

use super::calls::{LegacyFunctionPendingSessionV1, PendingFunctionSessionCloseV1};
use super::module_draft_collector::CollectedDraftAdmissionReceiptV1;
use super::module_draft_collector::DraftPublicationPolicyV1;
use super::module_invocation_owner_chain::InvocationBranded;
use super::module_lowering_invocation::{
    LegacyChildDraftAdmissionV1, ModuleLoweringPortChildErrorV1, ModuleLoweringPortV1,
};

impl ModuleLoweringPortV1<'_> {
    #[allow(dead_code)]
    pub(in crate::mir::builder) fn commit_legacy_pending(
        &mut self,
        pending: LegacyFunctionPendingSessionV1<'_>,
        admission: LegacyChildDraftAdmissionV1,
    ) -> Result<(), ModuleLoweringPortChildErrorV1> {
        self.commit_legacy_symbol_pending(pending, admission.collector_parts())
    }

    pub(in crate::mir::builder) fn commit_legacy_symbol_pending(
        &mut self,
        pending: LegacyFunctionPendingSessionV1<'_>,
        (key, symbol, arity): (
            super::module_draft_collector::FunctionDraftKeyV1,
            String,
            usize,
        ),
    ) -> Result<(), ModuleLoweringPortChildErrorV1> {
        pending.complete_before_restore(|draft| {
            let prepared = self
                .prepare_draft_admission(
                    key,
                    symbol,
                    arity,
                    DraftPublicationPolicyV1::LegacyReplaceWholePair,
                )
                .map_err(ModuleLoweringPortChildErrorV1::Admission)?;
            prepared
                .seal(draft)
                .map_err(ModuleLoweringPortChildErrorV1::Admission)?
                .collect();
            Ok(())
        })
    }

    pub(in crate::mir::builder) fn commit_legacy_symbol_pending_branded(
        &mut self,
        pending: LegacyFunctionPendingSessionV1<'_>,
        (key, symbol, arity): (
            super::module_draft_collector::FunctionDraftKeyV1,
            String,
            usize,
        ),
    ) -> Result<InvocationBranded<CollectedDraftAdmissionReceiptV1>, ModuleLoweringPortChildErrorV1>
    {
        pending.complete_before_restore(|draft| {
            let prepared = self
                .prepare_draft_admission(
                    key,
                    symbol,
                    arity,
                    DraftPublicationPolicyV1::LegacyReplaceWholePair,
                )
                .map_err(ModuleLoweringPortChildErrorV1::Admission)?;
            prepared
                .seal(draft)
                .map_err(ModuleLoweringPortChildErrorV1::Admission)?
                .collect_branded()
                .map_err(ModuleLoweringPortChildErrorV1::ReceiptBrand)
        })
    }
}
