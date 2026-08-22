//! Resolved selected-normal completion with one package signature sibling loan.

use super::super::calls::PendingFunctionSessionCloseV1;
use super::super::module_draft_collector::DraftPublicationPolicyV1;
use super::{ModuleLoweringPortChildErrorV1, ModuleLoweringPortV1, ResolvedChildDraftAdmissionV1};
use crate::mir::normal_callable_semantic_package::ResolvedCallablePhysicalSignatureLoanV1;

impl ModuleLoweringPortV1<'_> {
    /// Consume a package-owned physical-signature sibling loan while the
    /// resolved draft closes. The module/collector never retains the loan.
    pub(in crate::mir::builder) fn complete_resolved_child_with_physical_loan<'loan>(
        &mut self,
        pending: PendingFunctionSessionCloseV1<'_>,
        admission: ResolvedChildDraftAdmissionV1,
        loan: ResolvedCallablePhysicalSignatureLoanV1<'loan>,
        target_capability: Option<
            &'loan crate::mir::compiler::target_capability::PinnedTextCompileTargetCapabilityV1,
        >,
    ) -> Result<(), ModuleLoweringPortChildErrorV1> {
        if loan.owner() != admission.owner() {
            return Err(ModuleLoweringPortChildErrorV1::PhysicalSignatureMismatch);
        }
        let backend_target =
            if loan.has_exact_text_formal() {
                Some(target_capability.ok_or(
                    ModuleLoweringPortChildErrorV1::PinnedTextBackendFrameContractMismatch,
                )?)
            } else {
                None
            };
        let (key, symbol, arity) = admission.collector_parts();
        pending.complete_before_restore(|mut draft| {
            let backend_contract = if let Some(target) = backend_target {
                Some(crate::mir::compiler::pinned_text_backend_frame::issue_pinned_text_backend_frame_contract_v1(
                    &loan,
                    &draft.metadata.pinned_text_access_plans,
                    crate::runtime::text_formal_residence::residence_abi_layout_v1(),
                    target,
                )
                .map_err(|_| {
                    ModuleLoweringPortChildErrorV1::PinnedTextBackendFrameContractMismatch
                })?)
            } else {
                None
            };
            draft.metadata.pinned_text_backend_frame_contract = backend_contract;
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
                .collect();
            Ok(())
        })
    }
}
