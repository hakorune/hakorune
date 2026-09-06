//! Consuming source-backed catalog installation and scoped selected loans.

mod lowering_port;
#[path = "selected_input.rs"]
mod selected_input;
mod signature_loan;

use std::{cell::RefCell, collections::BTreeSet, rc::Rc};

use crate::mir::builder::{
    BuilderInstallConsumerV1, BuilderPrivateInstalledCallablePackageBundleV1,
    CatalogedBoxMethodPhysicalHeaderProjectionV1, CompilationContext,
    NormalCatalogedBoxMethodDraftAdmissionV1, SameModuleCallableCatalogBrandV1,
    SelectedNormalCallableKeyV1, VerifiedSourceBackedDynamicCallableV1,
};
use crate::mir::callable_semantic_batch::VerifiedResolvedCallableSourceIdentityV1;
use crate::mir::compiler::dynamic_full_body_recipe::VerifiedDynamicExitTransactionCoSealV1;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::resolved_semantics::VerifiedResolvedBlockExpressionExpectationV1;
use crate::parser::{ParserNormalProgramSourceLoanRejectV1, ParserNormalProgramSourceLoanV1};

use super::declared_instance_locator::{
    DeclaredInstanceCallLocatorScopeV1, DeclaredInstanceCallLocatorViewV1,
};
use super::ordinary_new_coseal::{
    OrdinaryNewAdmissionClaimV1, OrdinaryNewClaimLedgerV1, OrdinaryNewClaimTakeErrorV1,
};
use super::physical_header::{CallablePhysicalHeaderRefV1, VerifiedCallablePhysicalHeaderCohortV1};
use super::physical_signature::{
    PhysicalCallableSignatureRowRefV1, VerifiedCallablePhysicalSignatureCohortV1,
};
use super::result_contract::{CallableResultContractRefV1, VerifiedCallableResultContractCohortV1};
use super::s6c_storage_header::VerifiedS6CStorageHeaderProjectionV1;
use super::selected_mapping::VerifiedSelectedCallableBatchMapV1;
use super::{
    model::{
        NormalCallableDynamicProjectionV1, NormalRootExecutionPackageStateV1,
        OwnedCallableParameterContractDeclarationV1, VerifiedNormalCallableSemanticPackageV1,
    },
    BuilderInstallTokenV1,
};

pub(crate) use signature_loan::ResolvedCallablePhysicalSignatureLoanV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum NormalCallableSemanticPackageInstallIssueV1 {
    ForeignCatalog,
    SelectedKeyUnavailable,
    DuplicateSelectedKey,
    IncompleteSelectedCoverage,
    IncompleteOrdinaryNewCoverage,
    OrdinaryNewClaimUnavailable,
    OrdinaryNewClaimMismatch,
    BatchLoan,
    CatalogedAdmissionMismatch,
    MissingParameterContract,
    DuplicateParameterContract,
    ParameterContractOwnerMismatch,
    PhysicalSignatureMismatch,
    ResultContractUnavailable,
    ResultContractMismatch,
    MainChildUnavailable,
    MainChildIdentityMismatch,
    MainChildRoleMismatch,
    MainChildAdmissionRequired,
    S6CChildUnavailable,
    S6CChildAlreadyConsumed,
    S6CChildKeyUnavailable,
    PhysicalSignatureUnavailable,
    CatalogSlotOccupied,
    LoweringAlreadyStarted,
    DirectCallLoanNotConsumed,
    MainRootUnavailable,
    MainRootRelationMismatch,
    MainRootAlreadyConsumed,
    DeclaredInstanceLocatorNotConsumed,
    ObjectDefinitionsNotConsumed,
    S6CCommonV2(crate::mir::loop_recipe_contract::CommonV2IssuerRejectV1),
}

#[derive(Debug)]
pub(crate) struct InstalledNormalCallableSemanticPackageV1 {
    catalog_brand: SameModuleCallableCatalogBrandV1,
    batch: crate::mir::callable_semantic_batch::VerifiedResolvedCallableSemanticBatchV1,
    app_main_direct_call_loan: Option<super::direct_call_loan::AppMainDirectCallDispositionLoanV1>,
    ordinary_new_claim_ledger: Rc<OrdinaryNewClaimLedgerV1>,
    instance_constructors:
        super::instance_constructor_semantic::VerifiedInstanceConstructorSemanticBatchV1,
    selected: VerifiedSelectedCallableBatchMapV1,
    parameter_contracts: Box<[OwnedCallableParameterContractDeclarationV1]>,
    result_contracts: VerifiedCallableResultContractCohortV1,
    physical_signature: VerifiedCallablePhysicalSignatureCohortV1,
    declared_instance_call_locators:
        super::declared_instance_locator::DeclaredInstanceCallPackageLocatorDispositionV1,
    s6c_child: Option<super::s6c_child::VerifiedS6CSemanticChildV1>,
    s6c_storage_header: Option<VerifiedS6CStorageHeaderProjectionV1>,
    physical_header: VerifiedCallablePhysicalHeaderCohortV1,
    dynamic: NormalCallableDynamicProjectionV1,
    dynamic_physical_header: RefCell<Option<CatalogedBoxMethodPhysicalHeaderProjectionV1>>,
}

pub(crate) struct SelectedCallableLoweringInputRefV1<'loan> {
    source: ResolvedFunctionLoweringInputV1<'loan>,
    parameter_contracts: &'loan [super::model::OwnedCallableParameterContractV1],
    block_expr_expectation: &'loan VerifiedResolvedBlockExpressionExpectationV1,
    physical_header: Option<CallablePhysicalHeaderRefV1<'loan>>,
    result_contract: Option<CallableResultContractRefV1<'loan>>,
    semantic: SelectedCallableSemanticRefV1<'loan>,
    source_identity: VerifiedResolvedCallableSourceIdentityV1,
    selected_key: SelectedNormalCallableKeyV1,
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
    pub(in crate::mir) fn into_lowering_and_admission(
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
    pub(in crate::mir) fn with_selected_and_admission<R>(
        &self,
        callback: impl for<'view> FnOnce(
            &'view SelectedCallableLoweringInputRefV1<'loan>,
            &'view NormalCatalogedBoxMethodDraftAdmissionV1,
        ) -> R,
    ) -> R {
        callback(&self.selected, &self.admission)
    }

    pub(in crate::mir) fn into_lowering_and_admission(
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
pub(in crate::mir) enum SelectedCallableSemanticRefV1<'loan> {
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
#[must_use]
pub(crate) struct NormalCallableSemanticPackagePortV1<'package> {
    pub(super) installed: &'package InstalledNormalCallableSemanticPackageV1,
    pub(super) app_main_direct_call_loan:
        Option<super::direct_call_loan::AppMainDirectCallDispositionLoanV1>,
    consumed: BTreeSet<SelectedNormalCallableKeyV1>,
    declared_instance_consumed: BTreeSet<u32>,
    s6c_child_consumed: bool,
    main_root_consumed: bool,
}

impl NormalCallableSemanticPackagePortV1<'_> {
    pub(in crate::mir) fn take_object_definitions(
        &mut self,
        context: &CompilationContext,
    ) -> Result<Box<[crate::mir::function::CanonicalObjectDefinitionV1]>, String> {
        if !self.installed.installed_in(context) {
            return Err("[freeze:contract][mir/object-definitions/foreign-package]".into());
        }
        self.installed.instance_constructors.take_object_definitions()
            .ok_or_else(|| "[freeze:contract][mir/object-definitions/already-taken]".into())
    }

    /// Lend the already-issued locator without exposing package ownership or
    /// allowing the view to outlive this callback.  This is transport only;
    /// selected-C admission remains a separate downstream boundary.
    pub(in crate::mir) fn with_declared_instance_call_locators<R>(
        &mut self,
        callback: impl for<'view> FnOnce(DeclaredInstanceCallLocatorScopeV1<'view>) -> R,
    ) -> R {
        let consumed = &mut self.declared_instance_consumed;
        self.installed.with_declared_instance_call_locators(|view| {
            callback(DeclaredInstanceCallLocatorScopeV1::new(view, consumed))
        })
    }

    /// Move the package-owned App Main inventory into the root raw session.
    /// There is no package-only fallback once this succeeds.
    pub(in crate::mir) fn take_app_main_direct_call_loan(
        &mut self,
    ) -> Option<super::direct_call_loan::AppMainDirectCallDispositionLoanV1> {
        self.app_main_direct_call_loan.take()
    }
}

impl VerifiedNormalCallableSemanticPackageV1 {
    pub(in crate::mir) fn with_normal_callable_install_once(
        self,
        context: &mut CompilationContext,
        consumer: BuilderInstallConsumerV1,
    ) -> Result<
        BuilderPrivateInstalledCallablePackageBundleV1,
        NormalCallableSemanticPackageInstallIssueV1,
    > {
        let prepared = self
            .prepare_install(context)
            .map_err(|_package| NormalCallableSemanticPackageInstallIssueV1::CatalogSlotOccupied)?;
        let installed = prepared.commit();
        Ok(consumer.seal(installed, BuilderInstallTokenV1::issue()))
    }

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
            root_execution,
            catalog,
            batch,
            app_main_direct_call_loan,
            ordinary_new_claim_ledger,
            instance_constructors,
            selected,
            parameter_contracts,
            result_contracts,
            physical_signature,
            s6c_child,
            s6c_storage_header,
            physical_header,
            dynamic,
            dynamic_physical_header,
            declared_instance_call_locators,
        } = self.package;
        match root_execution {
            NormalRootExecutionPackageStateV1::Prepared(root) => root.discard_unconnected(),
            NormalRootExecutionPackageStateV1::MovedToLowering => {}
        }
        let catalog_brand = catalog.catalog().brand().clone();
        self.context
            .install_callable_declaration_catalog_preflighted(catalog.into_catalog());
        InstalledNormalCallableSemanticPackageV1 {
            catalog_brand,
            batch,
            app_main_direct_call_loan,
            ordinary_new_claim_ledger,
            instance_constructors,
            selected,
            parameter_contracts,
            result_contracts,
            physical_signature,
            declared_instance_call_locators,
            s6c_child,
            s6c_storage_header,
            physical_header,
            dynamic,
            dynamic_physical_header: RefCell::new(dynamic_physical_header),
        }
    }
}

impl InstalledNormalCallableSemanticPackageV1 {
    pub(in crate::mir) fn with_declared_instance_call_locators<R>(
        &self,
        callback: impl for<'view> FnOnce(DeclaredInstanceCallLocatorViewV1<'view>) -> R,
    ) -> R {
        let source = self.batch.declared_instance_call_source();
        callback(DeclaredInstanceCallLocatorViewV1::new(
            &self.declared_instance_call_locators,
            source,
        ))
    }

    pub(crate) fn take_ordinary_new_claim(
        &self,
        site: &crate::mir::resolved_semantics::OwnedExprSiteV1,
        class: &str,
        arity: usize,
    ) -> Result<OrdinaryNewAdmissionClaimV1, NormalCallableSemanticPackageInstallIssueV1> {
        match self
            .ordinary_new_claim_ledger
            .try_take(site, class, arity)
            .map_err(|error| match error {
                OrdinaryNewClaimTakeErrorV1::Unavailable => {
                    NormalCallableSemanticPackageInstallIssueV1::OrdinaryNewClaimUnavailable
                }
                OrdinaryNewClaimTakeErrorV1::Mismatch => {
                    NormalCallableSemanticPackageInstallIssueV1::OrdinaryNewClaimMismatch
                }
            })? {
            Some(claim) => Ok(claim),
            None => Err(NormalCallableSemanticPackageInstallIssueV1::OrdinaryNewClaimUnavailable),
        }
    }

    pub(crate) fn ordinary_box_is_covered(&self, class: &str) -> bool {
        self.batch.ordinary_box_coverage().contains_box(class)
    }

    pub(crate) fn ordinary_new_claim_ledger(&self) -> Rc<OrdinaryNewClaimLedgerV1> {
        Rc::clone(&self.ordinary_new_claim_ledger)
    }

    /// Reborrow the same parser-owned Program source authority after install.
    /// The HRTB keeps the AST loan scoped; callers may only bind already-owned
    /// pre-effect facts and may not carry this borrowed wrapper across install.
    pub(crate) fn with_normal_program_source_loan<R>(
        &self,
        callback: impl for<'source> FnOnce(ParserNormalProgramSourceLoanV1<'source>) -> R,
    ) -> Result<R, ParserNormalProgramSourceLoanRejectV1> {
        self.batch.with_normal_program_source_loan(callback)
    }

    pub(crate) fn instance_constructors(
        &self,
    ) -> &super::instance_constructor_semantic::VerifiedInstanceConstructorSemanticBatchV1 {
        &self.instance_constructors
    }

    pub(crate) fn installed_in(&self, context: &CompilationContext) -> bool {
        context
            .callable_declaration_catalog()
            .is_ok_and(|catalog| catalog.brand().is_same(&self.catalog_brand))
    }

    pub(crate) fn open_lowering_port(
        &self,
        context: &CompilationContext,
        app_main_direct_call_loan: Option<
            super::direct_call_loan::AppMainDirectCallDispositionLoanV1,
        >,
    ) -> Result<NormalCallableSemanticPackagePortV1<'_>, NormalCallableSemanticPackageInstallIssueV1>
    {
        if !self.installed_in(context) {
            return Err(NormalCallableSemanticPackageInstallIssueV1::ForeignCatalog);
        }
        self.open_lowering_port_after_install(app_main_direct_call_loan)
    }

    pub(crate) fn open_lowering_port_after_install(
        &self,
        app_main_direct_call_loan: Option<
            super::direct_call_loan::AppMainDirectCallDispositionLoanV1,
        >,
    ) -> Result<NormalCallableSemanticPackagePortV1<'_>, NormalCallableSemanticPackageInstallIssueV1>
    {
        if self.app_main_direct_call_loan.is_some() {
            return Err(NormalCallableSemanticPackageInstallIssueV1::DirectCallLoanNotConsumed);
        }
        Ok(NormalCallableSemanticPackagePortV1 {
            installed: self,
            app_main_direct_call_loan,
            consumed: BTreeSet::new(),
            declared_instance_consumed: BTreeSet::new(),
            s6c_child_consumed: false,
            main_root_consumed: false,
        })
    }

    #[cfg(test)]
    pub(crate) fn begin_lowering(
        &self,
        context: &CompilationContext,
    ) -> Result<NormalCallableSemanticPackagePortV1<'_>, NormalCallableSemanticPackageInstallIssueV1>
    {
        self.open_lowering_port(context, None)
    }

    pub(in crate::mir) fn take_app_main_direct_call_loan(
        &mut self,
    ) -> Option<super::direct_call_loan::AppMainDirectCallDispositionLoanV1> {
        self.app_main_direct_call_loan.take()
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
        let result_contract = match key {
            // The S6C child consumed this row's Completion seed exclusively
            // before the generic retention cohort was assembled. Its selected
            // view remains valid, but the generic result contract is absent by
            // construction; S6C owns the matching completion through its
            // child loan instead.
            SelectedNormalCallableKeyV1::Cataloged(_) if self.selected.is_main_child_key(key) => {
                None
            }
            SelectedNormalCallableKeyV1::Cataloged(_) => {
                let row = self.result_contracts.row(batch_slot).ok_or(
                    NormalCallableSemanticPackageInstallIssueV1::ResultContractUnavailable,
                )?;
                let Some(expected_identity) = self.selected.identity_for_batch_slot(batch_slot)
                else {
                    return Err(
                        NormalCallableSemanticPackageInstallIssueV1::ResultContractMismatch,
                    );
                };
                let Some(expected_role) = self.selected.role_for_batch_slot(batch_slot) else {
                    return Err(
                        NormalCallableSemanticPackageInstallIssueV1::ResultContractMismatch,
                    );
                };
                if !row.identity().same_as(expected_identity) || row.role() != expected_role {
                    return Err(
                        NormalCallableSemanticPackageInstallIssueV1::ResultContractMismatch,
                    );
                }
                Some(row.borrow())
            }
            SelectedNormalCallableKeyV1::TopLevel(_) => None,
        };
        let physical_header = self.physical_header.row(batch_slot, &self.result_contracts);
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
                if result_contract.is_some_and(|contract| contract.owner() != source.owner()) {
                    return Err(
                        NormalCallableSemanticPackageInstallIssueV1::ResultContractMismatch,
                    );
                }
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
                    result_contract,
                    semantic,
                    source_identity,
                    selected_key,
                }))
            })
            .map_err(|_| NormalCallableSemanticPackageInstallIssueV1::BatchLoan)?
    }
}
