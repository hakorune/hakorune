//! Consuming source-backed catalog installation and scoped selected loans.

use std::ptr;
use std::{cell::RefCell, collections::BTreeSet};

use crate::mir::builder::{
    CatalogedBoxMethodPhysicalHeaderProjectionV1, CompilationContext,
    NormalCatalogedBoxMethodDraftAdmissionV1, SameModuleCallableCatalogBrandV1,
    SelectedNormalCallableKeyV1, VerifiedMainStaticChildV1, VerifiedSourceBackedDynamicCallableV1,
};
use crate::mir::callable_parameter_contract::CallableParameterContractKindV1;
use crate::mir::callable_semantic_batch::VerifiedResolvedCallableSourceIdentityV1;
use crate::mir::compiler::dynamic_full_body_recipe::VerifiedDynamicExitTransactionCoSealV1;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::resolved_semantics::{BindingRefV1, VerifiedResolvedBlockExpressionExpectationV1};
use crate::parser::CallableMethodSourceObservationV1;

use super::model::{
    NormalCallableDynamicProjectionV1, OwnedCallableParameterContractDeclarationV1,
    VerifiedNormalCallableSemanticPackageV1,
};
use super::physical_header::{CallablePhysicalHeaderRefV1, VerifiedCallablePhysicalHeaderCohortV1};
use super::physical_signature::{
    PhysicalCallableSignatureRowRefV1, VerifiedCallablePhysicalSignatureCohortV1,
};
use super::s6c_storage_header::VerifiedS6CStorageHeaderProjectionV1;
use super::selected_mapping::VerifiedSelectedCallableBatchMapV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum NormalCallableSemanticPackageInstallIssueV1 {
    ForeignCatalog,
    SelectedKeyUnavailable,
    DuplicateSelectedKey,
    IncompleteSelectedCoverage,
    BatchLoan,
    CatalogedAdmissionMismatch,
    MissingParameterContract,
    DuplicateParameterContract,
    ParameterContractOwnerMismatch,
    PhysicalSignatureMismatch,
    MainChildUnavailable,
    MainChildSourceMismatch,
    MainChildIdentityMismatch,
    MainChildRoleMismatch,
    MainChildAdmissionRequired,
    S6CChildUnavailable,
    S6CChildAlreadyConsumed,
    S6CChildKeyUnavailable,
    PhysicalSignatureUnavailable,
    S6CCommonV2(crate::mir::loop_recipe_contract::CommonV2IssuerRejectV1),
}

#[derive(Debug)]
pub(crate) struct InstalledNormalCallableSemanticPackageV1 {
    catalog_brand: SameModuleCallableCatalogBrandV1,
    batch: crate::mir::callable_semantic_batch::VerifiedResolvedCallableSemanticBatchV1,
    selected: VerifiedSelectedCallableBatchMapV1,
    parameter_contracts: Box<[OwnedCallableParameterContractDeclarationV1]>,
    physical_signature: VerifiedCallablePhysicalSignatureCohortV1,
    s6c_child: Option<super::s6c_child::VerifiedS6CSemanticChildV1>,
    s6c_storage_header: Option<VerifiedS6CStorageHeaderProjectionV1>,
    physical_header: Option<VerifiedCallablePhysicalHeaderCohortV1>,
    dynamic: NormalCallableDynamicProjectionV1,
    dynamic_physical_header: RefCell<Option<CatalogedBoxMethodPhysicalHeaderProjectionV1>>,
}

pub(crate) struct SelectedCallableLoweringInputRefV1<'loan> {
    source: ResolvedFunctionLoweringInputV1<'loan>,
    parameter_contracts: &'loan [super::model::OwnedCallableParameterContractV1],
    block_expr_expectation: &'loan VerifiedResolvedBlockExpressionExpectationV1,
    physical_header: Option<CallablePhysicalHeaderRefV1<'loan>>,
    semantic: SelectedCallableSemanticRefV1<'loan>,
    source_identity: VerifiedResolvedCallableSourceIdentityV1,
    selected_key: SelectedNormalCallableKeyV1,
}

/// Non-owning, non-Clone signature sibling for one resolved lowering loan.
/// The package issuer is the only constructor; the module handoff consumes
/// this view before the surrounding HRTB callback returns.
pub(crate) struct ResolvedCallablePhysicalSignatureLoanV1<'loan> {
    row: PhysicalCallableSignatureRowRefV1<'loan>,
    _seal: ResolvedCallablePhysicalSignatureLoanSealV1,
}

struct ResolvedCallablePhysicalSignatureLoanSealV1;

impl<'loan> ResolvedCallablePhysicalSignatureLoanV1<'loan> {
    fn new(row: PhysicalCallableSignatureRowRefV1<'loan>) -> Self {
        Self {
            row,
            _seal: ResolvedCallablePhysicalSignatureLoanSealV1,
        }
    }

    pub(crate) const fn owner(&self) -> crate::mir::resolved_semantics::FunctionOwnerIdV1 {
        self.row.owner()
    }

    pub(crate) fn identity(&self) -> &crate::parser::CallableDeclarationIdentityV1 {
        self.row.identity()
    }

    pub(crate) const fn physical_callable_lane_count(&self) -> u32 {
        self.row.physical_callable_lane_count()
    }

    pub(crate) const fn receiver_lane_count(&self) -> u32 {
        self.row.receiver_lane_count()
    }

    pub(crate) const fn source_logical_arity(&self) -> u32 {
        self.row.source_logical_arity()
    }

    pub(crate) const fn physical_formal_lane_count(&self) -> u32 {
        self.row.physical_formal_lane_count()
    }

    pub(crate) fn has_exact_text_formal(&self) -> bool {
        self.row.lanes().iter().any(|lane| {
            matches!(
                lane.role(),
                crate::mir::normal_callable_semantic_package::PhysicalCallableLaneRoleV1::
                    ExactTextSlot
            )
        })
    }

    /// Borrow the complete lane rows from the package-owned cohort.  This is
    /// a scoped sibling view; it never copies or reissues the signature rows.
    pub(crate) fn lanes(
        &self,
    ) -> &[crate::mir::normal_callable_semantic_package::physical_signature::PhysicalCallableLaneV1]
    {
        self.row.lanes()
    }
}

/// Exactly-once selected input paired with the catalog admission that already
/// crossed the catalog boundary. Downstream users consume this wrapper instead
/// of reconstructing an admission from a source key.
pub(crate) struct SelectedCatalogedCallableLoweringInputV1<'loan> {
    selected: SelectedCallableLoweringInputRefV1<'loan>,
    admission: NormalCatalogedBoxMethodDraftAdmissionV1,
    physical_header: Option<CatalogedBoxMethodPhysicalHeaderProjectionV1>,
}

/// One exactly-once S6C child loan.  The selected input, its parameter
/// contracts, the package-owned child, and the physical signature row are
/// sibling views of the same installed cohort; callers cannot open them by
/// separate key/slot lookups.
pub(crate) struct S6CInstalledCallableLoanRefV1<'loan> {
    selected: SelectedCallableLoweringInputRefV1<'loan>,
    child: super::s6c_child::S6CSemanticChildRefV1<'loan>,
    signature: PhysicalCallableSignatureRowRefV1<'loan>,
    storage_header: &'loan VerifiedS6CStorageHeaderProjectionV1,
}

/// One installed S6C callable loan plus the generic common-V2 products
/// issued from that same retained source cohort.  The envelope is scoped to
/// the callback and cannot be paired with another callable or Completion.
pub(crate) struct S6CCommonV2PreSessionLoanRefV1<'loan, 'source, 'join> {
    callable: S6CInstalledCallableLoanRefV1<'loan>,
    envelope: crate::mir::loop_recipe_contract::PreparedLoopV2PreSessionEnvelopeV1<'source, 'join>,
}

impl S6CCommonV2PreSessionLoanRefV1<'_, '_, '_> {
    pub(crate) fn callable(&self) -> &S6CInstalledCallableLoanRefV1<'_> {
        &self.callable
    }

    pub(crate) fn envelope(
        &self,
    ) -> &crate::mir::loop_recipe_contract::PreparedLoopV2PreSessionEnvelopeV1<'_, '_> {
        &self.envelope
    }
}

impl S6CInstalledCallableLoanRefV1<'_> {
    pub(crate) fn selected(&self) -> &SelectedCallableLoweringInputRefV1<'_> {
        &self.selected
    }

    pub(crate) const fn signature(&self) -> PhysicalCallableSignatureRowRefV1<'_> {
        self.signature
    }

    pub(crate) fn storage_header(&self) -> &VerifiedS6CStorageHeaderProjectionV1 {
        self.storage_header
    }

    pub(crate) fn physical_effects(
        &self,
    ) -> &super::s6c_effects::VerifiedS6CPhysicalFunctionEffectsV1 {
        self.child.physical_effects()
    }

    pub(crate) const fn owner(&self) -> crate::mir::resolved_semantics::FunctionOwnerIdV1 {
        self.child.owner()
    }

    pub(crate) const fn result(
        &self,
    ) -> crate::mir::exact_trivial_scalar_abi::ExactTrivialScalarAbiV1 {
        self.child.result()
    }

    pub(crate) fn with_completion<R>(
        &self,
        callback: impl for<'facts> FnOnce(
            crate::mir::loop_recipe_contract::S6CPrephysicalCompletionRefV2<'facts>,
        ) -> R,
    ) -> R {
        self.child.with_completion(callback)
    }

    pub(crate) fn with_completion_parity<R>(
        &self,
        callback: impl FnOnce(
            crate::mir::loop_recipe_contract::S6CPrephysicalCompletionParityRefV2,
        ) -> R,
    ) -> R {
        self.child.with_completion_parity(callback)
    }

    /// Forward the installed child’s callback-scoped source relation.  The
    /// package loan remains the only source owner; no source view is
    /// recoverable after the callback returns.
    pub(crate) fn with_scalar_scan_source<R>(
        &self,
        callback: impl for<'a, 'rows, 'facts> FnOnce(
            crate::mir::loop_recipe_contract::S6CScalarScanSourceRefV1<'a, 'rows, 'facts>,
        ) -> Result<
            R,
            crate::mir::loop_recipe_contract::S6CScalarScanSourceRejectV1,
        >,
    ) -> Result<R, crate::mir::loop_recipe_contract::S6CScalarScanSourceRejectV1> {
        self.child.with_scalar_scan_source(callback)
    }
}

/// Exactly-once Main static-child input.  The generic key-only admission is
/// deliberately not exposed here; this wrapper is issued only after the
/// installed batch source, parser identity, and Main-child role co-seal.
pub(crate) struct MainStaticChildLoweringInputV1<'loan> {
    selected: SelectedCallableLoweringInputRefV1<'loan>,
    admission: NormalCatalogedBoxMethodDraftAdmissionV1,
    _role: crate::mir::builder::SelectedCallableConsumptionRoleV1,
    _catalog_brand: SameModuleCallableCatalogBrandV1,
}

impl<'loan> MainStaticChildLoweringInputV1<'loan> {
    pub(crate) fn into_lowering_and_admission(
        self,
    ) -> (
        SelectedCallableLoweringInputRefV1<'loan>,
        NormalCatalogedBoxMethodDraftAdmissionV1,
    ) {
        (self.selected, self.admission)
    }
}

impl<'loan> SelectedCatalogedCallableLoweringInputV1<'loan> {
    pub(crate) fn selected(&self) -> &SelectedCallableLoweringInputRefV1<'loan> {
        &self.selected
    }

    /// Lend the selected semantic input and its already-sealed catalog
    /// admission together for one bounded cross-check. Neither borrowed view
    /// can escape this callback, and the wrapper remains the only consuming
    /// path for the admission.
    pub(crate) fn with_selected_and_admission<R>(
        &self,
        callback: impl for<'view> FnOnce(
            &'view SelectedCallableLoweringInputRefV1<'loan>,
            &'view NormalCatalogedBoxMethodDraftAdmissionV1,
        ) -> R,
    ) -> R {
        callback(&self.selected, &self.admission)
    }

    pub(crate) fn into_lowering_and_admission(
        self,
    ) -> (
        SelectedCallableLoweringInputRefV1<'loan>,
        NormalCatalogedBoxMethodDraftAdmissionV1,
        Option<CatalogedBoxMethodPhysicalHeaderProjectionV1>,
    ) {
        (self.selected, self.admission, self.physical_header)
    }
}

#[derive(Clone, Copy)]
pub(crate) enum SelectedCallableSemanticRefV1<'loan> {
    Ordinary,
    Dynamic {
        program: &'loan VerifiedDynamicExitTransactionCoSealV1,
        source: &'loan std::rc::Rc<VerifiedSourceBackedDynamicCallableV1>,
    },
}

pub(crate) struct PreparedNormalCallableSemanticPackageInstallV1<'context> {
    context: &'context mut CompilationContext,
    package: VerifiedNormalCallableSemanticPackageV1,
}

/// Exactly-once lowering surface for one installed package.
///
/// The port borrows the whole installed package and never reveals a batch
/// slot.  It records selected-key consumption and must be completed before
/// the outer lowering transaction can close.
pub(crate) struct NormalCallableSemanticPackagePortV1<'package> {
    installed: &'package InstalledNormalCallableSemanticPackageV1,
    consumed: BTreeSet<SelectedNormalCallableKeyV1>,
    s6c_child_consumed: bool,
}

impl VerifiedNormalCallableSemanticPackageV1 {
    pub(crate) fn prepare_install<'context>(
        self,
        context: &'context mut CompilationContext,
    ) -> Result<PreparedNormalCallableSemanticPackageInstallV1<'context>, Self> {
        if !context.callable_declaration_catalog_vacant() {
            return Err(self);
        }
        Ok(PreparedNormalCallableSemanticPackageInstallV1 {
            context,
            package: self,
        })
    }
}

impl PreparedNormalCallableSemanticPackageInstallV1<'_> {
    pub(crate) fn commit(self) -> InstalledNormalCallableSemanticPackageV1 {
        let VerifiedNormalCallableSemanticPackageV1 {
            catalog,
            batch,
            selected,
            parameter_contracts,
            physical_signature,
            s6c_child,
            s6c_storage_header,
            physical_header,
            dynamic,
            dynamic_physical_header,
        } = self.package;
        let catalog_brand = catalog.catalog().brand().clone();
        self.context
            .install_callable_declaration_catalog_preflighted(catalog.into_catalog());
        InstalledNormalCallableSemanticPackageV1 {
            catalog_brand,
            batch,
            selected,
            parameter_contracts,
            physical_signature,
            s6c_child,
            s6c_storage_header,
            physical_header,
            dynamic,
            dynamic_physical_header: RefCell::new(dynamic_physical_header),
        }
    }
}

impl InstalledNormalCallableSemanticPackageV1 {
    pub(crate) fn source_ast(&self) -> &crate::ast::ASTNode {
        self.batch.source_ast()
    }

    pub(crate) fn installed_in(&self, context: &CompilationContext) -> bool {
        context
            .callable_declaration_catalog()
            .is_ok_and(|catalog| catalog.brand().is_same(&self.catalog_brand))
    }

    pub(crate) fn begin_lowering(
        &self,
        context: &CompilationContext,
    ) -> Result<NormalCallableSemanticPackagePortV1<'_>, NormalCallableSemanticPackageInstallIssueV1>
    {
        if !self.installed_in(context) {
            return Err(NormalCallableSemanticPackageInstallIssueV1::ForeignCatalog);
        }
        Ok(NormalCallableSemanticPackagePortV1 {
            installed: self,
            consumed: BTreeSet::new(),
            s6c_child_consumed: false,
        })
    }

    fn take_dynamic_physical_header(
        &self,
        key: &crate::mir::builder::CanonicalSameModuleCallableKeyV1,
    ) -> Option<CatalogedBoxMethodPhysicalHeaderProjectionV1> {
        let mut slot = self.dynamic_physical_header.borrow_mut();
        let header = slot.take()?;
        if header.key() == key {
            Some(header)
        } else {
            *slot = Some(header);
            None
        }
    }

    fn with_selected_lowering_input<R>(
        &self,
        key: &SelectedNormalCallableKeyV1,
        callback: impl for<'loan> FnOnce(SelectedCallableLoweringInputRefV1<'loan>) -> R,
    ) -> Result<R, NormalCallableSemanticPackageInstallIssueV1> {
        let batch_slot = self
            .selected
            .batch_slot(key)
            .ok_or(NormalCallableSemanticPackageInstallIssueV1::SelectedKeyUnavailable)?;
        // Reborrow the package's canonical key instead of trusting a
        // caller-owned spelling. The batch-slot check above proves
        // membership; this clone preserves that exact catalog identity for
        // the later physical-header projection.
        let selected_key = self
            .selected
            .keys()
            .find(|candidate| *candidate == key)
            .cloned()
            .ok_or(NormalCallableSemanticPackageInstallIssueV1::SelectedKeyUnavailable)?;
        let parameter_declaration = match key {
            SelectedNormalCallableKeyV1::Cataloged(_) => {
                let mut declarations = self
                    .parameter_contracts
                    .iter()
                    .filter(|row| row.batch_slot == batch_slot);
                let declaration = declarations
                    .next()
                    .ok_or(NormalCallableSemanticPackageInstallIssueV1::MissingParameterContract)?;
                if declarations.next().is_some() {
                    return Err(
                        NormalCallableSemanticPackageInstallIssueV1::DuplicateParameterContract,
                    );
                }
                Some(declaration)
            }
            SelectedNormalCallableKeyV1::TopLevel(_) => None,
        };
        let physical_header = self
            .physical_header
            .as_ref()
            .and_then(|cohort| cohort.row(batch_slot))
            .map(|row| row.borrow());
        let block_expr_expectation = self
            .batch
            .block_expr_expectation(batch_slot)
            .map_err(|_| NormalCallableSemanticPackageInstallIssueV1::BatchLoan)?;
        let semantic = match &self.dynamic {
            NormalCallableDynamicProjectionV1::Selected {
                batch_slot: dynamic_slot,
                program,
                source,
                ..
            } if *dynamic_slot == batch_slot => {
                SelectedCallableSemanticRefV1::Dynamic { program, source }
            }
            _ => SelectedCallableSemanticRefV1::Ordinary,
        };
        self.batch
            .with_lowering_input_and_source_identity(batch_slot, |source, source_identity| {
                let parameters = match parameter_declaration {
                    Some(declaration) => {
                        if declaration.owner != source.owner() {
                            return Err(
                                NormalCallableSemanticPackageInstallIssueV1::
                                    ParameterContractOwnerMismatch,
                            );
                        }
                        declaration.parameters.as_ref()
                    }
                    None => &[],
                };
                Ok(callback(SelectedCallableLoweringInputRefV1 {
                    source,
                    parameter_contracts: parameters,
                    block_expr_expectation,
                    physical_header,
                    semantic,
                    source_identity,
                    selected_key,
                }))
            })
            .map_err(|_| NormalCallableSemanticPackageInstallIssueV1::BatchLoan)?
    }
}

impl NormalCallableSemanticPackagePortV1<'_> {
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
                let child_ref = super::s6c_child::S6CSemanticChildRefV1 { child };
                child_ref
                    .with_common_v2_pre_session(|envelope| {
                        callback(S6CCommonV2PreSessionLoanRefV1 {
                            callable: S6CInstalledCallableLoanRefV1 {
                                selected,
                                child: super::s6c_child::S6CSemanticChildRefV1 { child },
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

    pub(crate) fn with_selected_cataloged_lowering_input<R>(
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
    pub(crate) fn with_selected_cataloged_lowering_input_and_signature<R>(
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

    pub(crate) fn with_main_static_child_lowering_input<R>(
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
        if !role.is_main_static_child() {
            return Err(NormalCallableSemanticPackageInstallIssueV1::MainChildRoleMismatch);
        }
        if self.consumed.contains(&key) {
            return Err(NormalCallableSemanticPackageInstallIssueV1::DuplicateSelectedKey);
        }
        let result = self
            .installed
            .with_selected_lowering_input(&key, |selected| {
                if !selected.source_identity().identity().same_as(&identity) {
                    return Err(
                        NormalCallableSemanticPackageInstallIssueV1::MainChildIdentityMismatch,
                    );
                }
                if !ptr::eq(selected.source().source().root(), child.source()) {
                    return Err(
                        NormalCallableSemanticPackageInstallIssueV1::MainChildSourceMismatch,
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
        Ok(())
    }
}

impl<'loan> SelectedCallableLoweringInputRefV1<'loan> {
    pub(crate) fn source(&self) -> ResolvedFunctionLoweringInputV1<'loan> {
        self.source
    }

    pub(crate) fn parameter_contracts(
        &self,
    ) -> impl ExactSizeIterator<Item = (u32, BindingRefV1, CallableParameterContractKindV1)> + '_
    {
        self.parameter_contracts
            .iter()
            .map(|row| (row.ordinal, row.binding, row.kind))
    }

    /// Borrow the resolver-owned BlockExpr expectation from the same batch
    /// row. This is transport only: no count is recomputed and the receipt is
    /// neither cloned nor reissued by the installed package.
    pub(crate) fn block_expr_expectation(&self) -> &VerifiedResolvedBlockExpressionExpectationV1 {
        self.block_expr_expectation
    }

    pub(crate) fn physical_header(&self) -> Option<CallablePhysicalHeaderRefV1<'_>> {
        self.physical_header
    }

    pub(crate) fn semantic(&self) -> SelectedCallableSemanticRefV1<'loan> {
        self.semantic
    }

    pub(crate) fn method_source_observation(&self) -> Option<&CallableMethodSourceObservationV1> {
        self.source_identity.method_source_observation()
    }

    pub(crate) fn source_identity(&self) -> &VerifiedResolvedCallableSourceIdentityV1 {
        &self.source_identity
    }

    pub(crate) fn selected_key(&self) -> &SelectedNormalCallableKeyV1 {
        &self.selected_key
    }
}
