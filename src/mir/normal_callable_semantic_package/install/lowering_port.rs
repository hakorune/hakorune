use std::{collections::BTreeMap, rc::Rc};

use crate::mir::builder::{
    CanonicalSameModuleCallableKeyV1, NormalCatalogedBoxMethodDraftAdmissionV1,
    SameModuleCallableNamespaceV1, SelectedNormalCallableKeyV1, VerifiedMainStaticChildV1,
};
use crate::mir::callable_semantic_batch::ResolvedCallableDeclarationModeV1;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::normal_callable_semantic_package::SelectedSourceCoreMethodCallV1;
use crate::mir::resolved_semantics::ReceiverPolicyV1;
use crate::parser::CallableDeclarationIdentityV1;

use super::super::declared_instance_locator::DeclaredInstanceCallLocatorScopeV1;
use super::super::ordinary_new_coseal::OrdinaryNewAdmissionClaimV1;
use super::super::s6c_child::S6CSemanticChildRefV1;
use super::{
    MainStaticChildLoweringInputV1, NormalCallableSemanticPackageInstallIssueV1,
    NormalCallableSemanticPackagePortV1, ResolvedCallablePhysicalSignatureLoanV1,
    S6CCommonV2PreSessionLoanRefV1, S6CInstalledCallableLoanRefV1,
    SelectedCallableLoweringInputRefV1, SelectedCatalogedCallableLoweringInputV1,
};

impl NormalCallableSemanticPackagePortV1<'_> {
    pub(crate) fn named_array_emission_collector(
        &self,
    ) -> Rc<super::super::NamedArrayEmissionCollectorV1> {
        Rc::clone(&self.installed.named_array_emissions)
    }

    pub(crate) fn take_source_core_method_calls(
        &self,
        key: &SelectedNormalCallableKeyV1,
    ) -> BTreeMap<crate::mir::resolved_semantics::SourceExprSiteV1, SelectedSourceCoreMethodCallV1>
    {
        self.installed.take_source_core_method_calls(key)
    }

    /// Lend the installed package's source-backed App Main root input once.
    ///
    /// The catalog relation is copied only as an opaque comparison witness;
    /// the batch remains the sole issuer of the lowering input.  This row is
    /// intentionally target-free and does not consume a direct-call loan.
    pub(crate) fn with_app_main_root_lowering_input<R>(
        &mut self,
        expected_key: &CanonicalSameModuleCallableKeyV1,
        expected_identity: &CallableDeclarationIdentityV1,
        callback: impl for<'source> FnOnce(
            ResolvedFunctionLoweringInputV1<'source>,
            crate::mir::callable_semantic_batch::VerifiedResolvedCallableSourceIdentityV1,
        ) -> R,
    ) -> Result<R, NormalCallableSemanticPackageInstallIssueV1> {
        if self.main_root_consumed {
            return Err(NormalCallableSemanticPackageInstallIssueV1::MainRootAlreadyConsumed);
        }
        let batch_slot = self.app_main_root_batch_slot(expected_key, expected_identity)?;
        self.main_root_consumed = true;
        self.with_app_main_root_source_view(batch_slot, expected_identity, callback)
    }

    /// Observe the same App Main root input without consuming the one-shot
    /// loan.
    ///
    /// Pre-wrapper route selection uses this read-only borrow to classify the
    /// source; the selected branch still performs the single consumption via
    /// `with_app_main_root_lowering_input`, and a non-selected source leaves
    /// the legacy hook's consumption intact.  The identical slot resolution
    /// and source validation run here so the observed input is exactly the
    /// one a later consumption would lend.
    pub(crate) fn observe_app_main_root_source_v1<R>(
        &self,
        expected_key: &CanonicalSameModuleCallableKeyV1,
        expected_identity: &CallableDeclarationIdentityV1,
        callback: impl for<'source> FnOnce(
            ResolvedFunctionLoweringInputV1<'source>,
            crate::mir::callable_semantic_batch::VerifiedResolvedCallableSourceIdentityV1,
        ) -> R,
    ) -> Result<R, NormalCallableSemanticPackageInstallIssueV1> {
        if self.main_root_consumed {
            return Err(NormalCallableSemanticPackageInstallIssueV1::MainRootAlreadyConsumed);
        }
        let batch_slot = self.app_main_root_batch_slot(expected_key, expected_identity)?;
        self.with_app_main_root_source_view(batch_slot, expected_identity, callback)
    }

    fn app_main_root_batch_slot(
        &self,
        expected_key: &CanonicalSameModuleCallableKeyV1,
        expected_identity: &CallableDeclarationIdentityV1,
    ) -> Result<u32, NormalCallableSemanticPackageInstallIssueV1> {
        if expected_key.namespace() != SameModuleCallableNamespaceV1::StaticBoxMethod {
            return Err(NormalCallableSemanticPackageInstallIssueV1::MainRootRelationMismatch);
        }
        self.installed
            .batch
            .with_declaration_semantics(|batch| {
                let mut declarations = batch
                    .declarations()
                    .iter()
                    .filter(|declaration| declaration.identity().same_as(expected_identity));
                let declaration = declarations
                    .next()
                    .ok_or(NormalCallableSemanticPackageInstallIssueV1::MainRootRelationMismatch)?;
                if declarations.next().is_some()
                    || declaration.mode() != ResolvedCallableDeclarationModeV1::StaticBoxMethod
                    || u32::try_from(
                        declaration
                            .parameters()
                            .map_or(0, |parameters| parameters.len()),
                    )
                    .ok()
                        != Some(expected_key.arity())
                {
                    return Err(
                        NormalCallableSemanticPackageInstallIssueV1::MainRootRelationMismatch,
                    );
                }
                Ok(declaration.batch_slot())
            })
            .map_err(|_| NormalCallableSemanticPackageInstallIssueV1::BatchLoan)?
    }

    fn with_app_main_root_source_view<R>(
        &self,
        batch_slot: u32,
        expected_identity: &CallableDeclarationIdentityV1,
        callback: impl for<'source> FnOnce(
            ResolvedFunctionLoweringInputV1<'source>,
            crate::mir::callable_semantic_batch::VerifiedResolvedCallableSourceIdentityV1,
        ) -> R,
    ) -> Result<R, NormalCallableSemanticPackageInstallIssueV1> {
        self.installed
            .batch
            .with_lowering_input_and_source_identity(batch_slot, |input, source_identity| {
                let owner = input.owner();
                let Some(root) = input.forest().owner(owner) else {
                    return Err(
                        NormalCallableSemanticPackageInstallIssueV1::MainRootRelationMismatch,
                    );
                };
                if !source_identity.identity().same_as(expected_identity)
                    || source_identity.mode() != ResolvedCallableDeclarationModeV1::StaticBoxMethod
                    || source_identity.owner() != owner
                    || input.forest().roots() != std::slice::from_ref(&owner)
                    || root.owner() != owner
                    || root.source_site_inventory().owner() != owner
                    || root.root_profile().receiver_policy() != ReceiverPolicyV1::StaticCurrentOwner
                {
                    return Err(
                        NormalCallableSemanticPackageInstallIssueV1::MainRootRelationMismatch,
                    );
                }
                Ok(callback(input, source_identity))
            })
            .map_err(|_| NormalCallableSemanticPackageInstallIssueV1::BatchLoan)?
    }

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

    /// Lend one selected input together with the route-neutral CoreMethod arm
    /// issued from that same resolver ledger.  The catalog is consumed into
    /// the callback's scoped lowering state; no second name map is retained.
    pub(in crate::mir) fn with_selected_lowering_input_and_core_methods<R>(
        &mut self,
        key: &SelectedNormalCallableKeyV1,
        callback: impl for<'loan> FnOnce(
            SelectedCallableLoweringInputRefV1<'loan>,
            BTreeMap<
                crate::mir::resolved_semantics::SourceExprSiteV1,
                SelectedSourceCoreMethodCallV1,
            >,
            super::LoopBreakSourcePackageTakeHandle<'loan>,
        ) -> Result<R, String>,
    ) -> Result<R, NormalCallableSemanticPackageInstallIssueV1> {
        if self.consumed.contains(key) {
            return Err(NormalCallableSemanticPackageInstallIssueV1::DuplicateSelectedKey);
        }
        if self.installed.selected.is_main_child_key(key) {
            return Err(NormalCallableSemanticPackageInstallIssueV1::MainChildAdmissionRequired);
        }
        let core_method_calls = self.installed.take_source_core_method_calls(key);
        let result = self
            .installed
            .with_selected_lowering_input(key, |selected| {
                let loop_break_take = super::LoopBreakSourcePackageTakeHandle {
                    installed: self.installed,
                };
                callback(selected, core_method_calls, loop_break_take)
            })?
            .map_err(|error| {
                NormalCallableSemanticPackageInstallIssueV1::CoreMethodSource(
                    error.into_boxed_str(),
                )
            })?;
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

    /// Variant for the source-backed lowering path that also lends the
    /// package-owned LoopBreak transport handle. Existing callers keep the
    /// narrower signature above.
    pub(in crate::mir) fn with_selected_cataloged_lowering_input_signature_and_loop_break<R>(
        &mut self,
        admission: NormalCatalogedBoxMethodDraftAdmissionV1,
        callback: impl for<'loan> FnOnce(
            SelectedCatalogedCallableLoweringInputV1<'loan>,
            ResolvedCallablePhysicalSignatureLoanV1<'loan>,
            super::LoopBreakSourcePackageTakeHandle<'loan>,
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
            let loop_break_take = super::LoopBreakSourcePackageTakeHandle {
                installed: self.installed,
            };
            callback(
                input,
                ResolvedCallablePhysicalSignatureLoanV1::new(signature),
                loop_break_take,
            )
        })
    }

    /// Lend the selected cataloged input, physical signature, and the same
    /// installed package's DeclaredInstance receiver locator in one callback.
    /// The locator is transport-only and is consumed by exact source site;
    /// no target or receiver value is issued here.
    pub(in crate::mir) fn with_selected_cataloged_lowering_input_signature_and_declared_instance_locator<
        R,
    >(
        &mut self,
        admission: NormalCatalogedBoxMethodDraftAdmissionV1,
        callback: impl for<'loan> FnOnce(
            SelectedCatalogedCallableLoweringInputV1<'loan>,
            ResolvedCallablePhysicalSignatureLoanV1<'loan>,
            DeclaredInstanceCallLocatorScopeV1<'loan>,
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
        let installed = self.installed;
        let locator_consumed = &self.declared_instance_consumed;
        let result = installed.with_selected_lowering_input(&key, |selected| {
            if selected.selected_key() != &key {
                return Err(
                    NormalCallableSemanticPackageInstallIssueV1::CatalogedAdmissionMismatch,
                );
            }
            let physical_header = installed.take_dynamic_physical_header(admission.source_key());
            installed.with_declared_instance_call_locators(|view| {
                Ok(callback(
                    SelectedCatalogedCallableLoweringInputV1 {
                        selected,
                        admission,
                        physical_header,
                    },
                    ResolvedCallablePhysicalSignatureLoanV1::new(signature),
                    DeclaredInstanceCallLocatorScopeV1::new(view, locator_consumed),
                ))
            })
        })??;
        self.consumed.insert(key);
        Ok(result)
    }

    pub(in crate::mir) fn with_main_static_child_lowering_input<R>(
        &mut self,
        child: &VerifiedMainStaticChildV1<'_>,
        callback: impl for<'loan> FnOnce(
            MainStaticChildLoweringInputV1<'loan>,
            BTreeMap<
                crate::mir::resolved_semantics::SourceExprSiteV1,
                SelectedSourceCoreMethodCallV1,
            >,
            super::LoopBreakSourcePackageTakeHandle<'loan>,
        ) -> R,
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
        let core_method_calls = self.installed.take_source_core_method_calls(&key);
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
                let loop_break_take = super::LoopBreakSourcePackageTakeHandle {
                    installed: self.installed,
                };
                Ok(callback(
                    MainStaticChildLoweringInputV1 {
                        selected,
                        admission,
                        signature: ResolvedCallablePhysicalSignatureLoanV1::new(signature),
                        _role: role,
                        _catalog_brand: self.installed.catalog_brand.clone(),
                    },
                    core_method_calls,
                    loop_break_take,
                ))
            })??;
        self.consumed.insert(key);
        Ok(result)
    }

    pub(crate) fn complete(self) -> Result<(), NormalCallableSemanticPackageInstallIssueV1> {
        if self.direct_call_loans.is_some() {
            return Err(NormalCallableSemanticPackageInstallIssueV1::DirectCallLoanNotConsumed);
        }
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
        let locator_count = self
            .installed
            .with_declared_instance_call_locators(|view| view.row_count());
        if self.declared_instance_consumed.borrow().len() != locator_count {
            return Err(
                NormalCallableSemanticPackageInstallIssueV1::DeclaredInstanceLocatorNotConsumed,
            );
        }
        if self
            .installed
            .instance_constructors
            .has_pending_object_definitions()
        {
            return Err(NormalCallableSemanticPackageInstallIssueV1::ObjectDefinitionsNotConsumed);
        }
        self.installed.lowering_completed.set(true);
        Ok(())
    }
}
