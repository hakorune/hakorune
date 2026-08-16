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
        let (key, symbol, arity) = admission.collector_parts();
        pending.complete_before_restore(|draft| {
            let _loan = loan;
            let _target_capability = target_capability;
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
