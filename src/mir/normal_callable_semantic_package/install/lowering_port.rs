use std::rc::Rc;

use crate::mir::builder::{
    NormalCatalogedBoxMethodDraftAdmissionV1, SelectedNormalCallableKeyV1,
    VerifiedMainStaticChildV1,
};

use super::super::ordinary_new_coseal::OrdinaryNewAdmissionClaimV1;
use super::super::s6c_child::S6CSemanticChildRefV1;
use super::{
    MainStaticChildLoweringInputV1, NormalCallableSemanticPackageInstallIssueV1,
    NormalCallableSemanticPackagePortV1, ResolvedCallablePhysicalSignatureLoanV1,
    S6CCommonV2PreSessionLoanRefV1, S6CInstalledCallableLoanRefV1,
    SelectedCallableLoweringInputRefV1, SelectedCatalogedCallableLoweringInputV1,
};

impl NormalCallableSemanticPackagePortV1<'_> {
    pub(crate) fn ordinary_box_is_covered(&self, class: &str) -> bool {
        self.installed.ordinary_box_is_covered(class)
    }

    pub(crate) fn ordinary_new_claim_ledger(
        &self,
    ) -> Rc<super::super::ordinary_new_coseal::OrdinaryNewClaimLedgerV1> {
        self.installed.ordinary_new_claim_ledger()
    }

    pub(crate) fn take_ordinary_new_claim(
        &mut self,
        site: &crate::mir::resolved_semantics::OwnedExprSiteV1,
        class: &str,
        arity: usize,
    ) -> Result<OrdinaryNewAdmissionClaimV1, NormalCallableSemanticPackageInstallIssueV1> {
        self.installed.take_ordinary_new_claim(site, class, arity)
    }

    pub(crate) fn with_s6c_child<R>(
        &mut self,
        callback: impl for<'loan> FnOnce(S6CInstalledCallableLoanRefV1<'loan>) -> R,
    ) -> Result<R, NormalCallableSemanticPackageInstallIssueV1> {
        self.with_s6c_common_v2_pre_session(|loan| callback(loan.callable))
    }

    /// Canonical installed-cohort handoff for the common V2 pre-session.
    /// Selection, S6C child, physical signature, and the common envelope are
    /// consumed once from one package port loan.
    pub(crate) fn with_s6c_common_v2_pre_session<R>(
        &mut self,
        callback: impl for<'loan, 'source, 'join> FnOnce(
            S6CCommonV2PreSessionLoanRefV1<'loan, 'source, 'join>,
        ) -> R,
    ) -> Result<R, NormalCallableSemanticPackageInstallIssueV1> {
        if self.s6c_child_consumed {
            return Err(NormalCallableSemanticPackageInstallIssueV1::S6CChildAlreadyConsumed);
        }
        let child = self
            .installed
            .s6c_child
            .as_ref()
            .ok_or(NormalCallableSemanticPackageInstallIssueV1::S6CChildUnavailable)?;
        let storage_header = self
            .installed
            .s6c_storage_header
            .as_ref()
            .ok_or(NormalCallableSemanticPackageInstallIssueV1::S6CChildUnavailable)?;
        let key = self
            .installed
            .selected
            .key_for_batch_slot(child.batch_slot())
            .cloned()
            .ok_or(NormalCallableSemanticPackageInstallIssueV1::S6CChildKeyUnavailable)?;
        if !self.installed.selected.is_main_child_key(&key) {
            return Err(NormalCallableSemanticPackageInstallIssueV1::MainChildRoleMismatch);
        }
        if self.consumed.contains(&key) {
            return Err(NormalCallableSemanticPackageInstallIssueV1::DuplicateSelectedKey);
        }
        let signature = self
            .installed
            .physical_signature
            .row(child.batch_slot())
            .ok_or(NormalCallableSemanticPackageInstallIssueV1::PhysicalSignatureUnavailable)?;
        let result = self
            .installed
            .with_selected_lowering_input(&key, |selected| {
                let child_ref = S6CSemanticChildRefV1 { child };
                child_ref
                    .with_common_v2_pre_session(|envelope| {
                        callback(S6CCommonV2PreSessionLoanRefV1 {
                            callable: S6CInstalledCallableLoanRefV1 {
                                selected,
                                child: S6CSemanticChildRefV1 { child },
                                signature,
                                storage_header,
                            },
                            envelope,
                        })
                    })
                    .map_err(NormalCallableSemanticPackageInstallIssueV1::S6CCommonV2)
            })??;
        self.consumed.insert(key);
        self.s6c_child_consumed = true;
        Ok(result)
    }

    pub(crate) fn with_selected_lowering_input<R>(
        &mut self,
        key: &SelectedNormalCallableKeyV1,
        callback: impl for<'loan> FnOnce(SelectedCallableLoweringInputRefV1<'loan>) -> R,
    ) -> Result<R, NormalCallableSemanticPackageInstallIssueV1> {
        if self.consumed.contains(key) {
            return Err(NormalCallableSemanticPackageInstallIssueV1::DuplicateSelectedKey);
        }
        if self.installed.selected.is_main_child_key(key) {
            return Err(NormalCallableSemanticPackageInstallIssueV1::MainChildAdmissionRequired);
        }
        let result = self.installed.with_selected_lowering_input(key, callback)?;
        self.consumed.insert(key.clone());
        Ok(result)
    }

    pub(in crate::mir) fn with_selected_cataloged_lowering_input<R>(
        &mut self,
        admission: NormalCatalogedBoxMethodDraftAdmissionV1,
        callback: impl for<'loan> FnOnce(SelectedCatalogedCallableLoweringInputV1<'loan>) -> R,
    ) -> Result<R, NormalCallableSemanticPackageInstallIssueV1> {
        let key = SelectedNormalCallableKeyV1::Cataloged(admission.source_key().clone());
        if self.installed.selected.is_main_child_key(&key) {
            return Err(NormalCallableSemanticPackageInstallIssueV1::MainChildAdmissionRequired);
        }
        let result = self.with_selected_lowering_input(&key, |selected| {
            if selected.selected_key() != &key {
                return Err(
                    NormalCallableSemanticPackageInstallIssueV1::CatalogedAdmissionMismatch,
                );
            }
            let physical_header = self
                .installed
                .take_dynamic_physical_header(admission.source_key());
            Ok(callback(SelectedCatalogedCallableLoweringInputV1 {
                selected,
                admission,
                physical_header,
            }))
        })??;
        Ok(result)
    }

    /// Lend the selected cataloged input and its same-cohort physical
    /// signature as sibling views for one synchronous resolved handoff.
    /// Neither view may escape the callback, and the admission remains the
    /// identity-only collector owner.
    pub(in crate::mir) fn with_selected_cataloged_lowering_input_and_signature<R>(
        &mut self,
        admission: NormalCatalogedBoxMethodDraftAdmissionV1,
        callback: impl for<'loan> FnOnce(
            SelectedCatalogedCallableLoweringInputV1<'loan>,
            ResolvedCallablePhysicalSignatureLoanV1<'loan>,
        ) -> R,
    ) -> Result<R, NormalCallableSemanticPackageInstallIssueV1> {
        let key = SelectedNormalCallableKeyV1::Cataloged(admission.source_key().clone());
        let batch_slot = self
            .installed
            .selected
            .batch_slot(&key)
            .ok_or(NormalCallableSemanticPackageInstallIssueV1::SelectedKeyUnavailable)?;
        let signature = self
            .installed
            .physical_signature
            .row(batch_slot)
            .ok_or(NormalCallableSemanticPackageInstallIssueV1::PhysicalSignatureUnavailable)?;
        self.with_selected_cataloged_lowering_input(admission, |input| {
            callback(
                input,
                ResolvedCallablePhysicalSignatureLoanV1::new(signature),
            )
        })
    }

    pub(in crate::mir) fn with_main_static_child_lowering_input<R>(
        &mut self,
        child: &VerifiedMainStaticChildV1<'_>,
        callback: impl for<'loan> FnOnce(MainStaticChildLoweringInputV1<'loan>) -> R,
    ) -> Result<R, NormalCallableSemanticPackageInstallIssueV1> {
        let Some((key, identity, role)) = self
            .installed
            .selected
            .main_child_selection(child.statement_index(), child.method_ordinal())
        else {
            return Err(NormalCallableSemanticPackageInstallIssueV1::MainChildUnavailable);
        };
        let key = key.clone();
        let identity = identity.clone();
        let child_identity = child
            .parser_identity()
            .ok_or(NormalCallableSemanticPackageInstallIssueV1::MainChildIdentityMismatch)?;
        if !identity.same_as(child_identity) {
            return Err(NormalCallableSemanticPackageInstallIssueV1::MainChildIdentityMismatch);
        }
        if !role.is_main_static_child() {
            return Err(NormalCallableSemanticPackageInstallIssueV1::MainChildRoleMismatch);
        }
        if self.consumed.contains(&key) {
            return Err(NormalCallableSemanticPackageInstallIssueV1::DuplicateSelectedKey);
        }
        let result = self
            .installed
            .with_selected_lowering_input(&key, |selected| {
                if !selected
                    .source_identity()
                    .identity()
                    .same_as(child_identity)
                {
                    return Err(
                        NormalCallableSemanticPackageInstallIssueV1::MainChildIdentityMismatch,
                    );
                }
                let admission = NormalCatalogedBoxMethodDraftAdmissionV1::seal(match &key {
                    SelectedNormalCallableKeyV1::Cataloged(key) => key.clone(),
                    SelectedNormalCallableKeyV1::TopLevel(_) => {
                        return Err(
                            NormalCallableSemanticPackageInstallIssueV1::MainChildRoleMismatch,
                        )
                    }
                })
                .map_err(|_| NormalCallableSemanticPackageInstallIssueV1::MainChildRoleMismatch)?;
                Ok(callback(MainStaticChildLoweringInputV1 {
                    selected,
                    admission,
                    _role: role,
                    _catalog_brand: self.installed.catalog_brand.clone(),
                }))
            })??;
        self.consumed.insert(key);
        Ok(result)
    }

    pub(crate) fn complete(self) -> Result<(), NormalCallableSemanticPackageInstallIssueV1> {
        if self.consumed.len() != self.installed.selected.keys().len()
            || self
                .installed
                .selected
                .keys()
                .any(|key| !self.consumed.contains(key))
        {
            return Err(NormalCallableSemanticPackageInstallIssueV1::IncompleteSelectedCoverage);
        }
        if !self.installed.ordinary_new_claim_ledger.is_empty() {
            return Err(NormalCallableSemanticPackageInstallIssueV1::IncompleteOrdinaryNewCoverage);
        }
        Ok(())
    }
}
