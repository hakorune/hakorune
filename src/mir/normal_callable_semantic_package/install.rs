//! Consuming source-backed catalog installation and scoped selected loans.

mod loop_break_source;
mod lowering_port;
#[path = "install_map_preflight.rs"]
mod map_preflight;
#[path = "selected_input.rs"]
mod selected_input;
mod selected_lowering_input;
mod signature_loan;
mod types;

use std::{
    cell::{Cell, RefCell},
    collections::BTreeSet,
    rc::Rc,
};

use crate::mir::builder::{
    BuilderInstallConsumerV1, BuilderPrivateInstalledCallablePackageBundleV1,
    CatalogedBoxMethodPhysicalHeaderProjectionV1, CompilationContext,
    NormalCatalogedBoxMethodDraftAdmissionV1, SameModuleCallableCatalogBrandV1,
    SelectedNormalCallableKeyV1,
};
use crate::mir::callable_semantic_batch::VerifiedResolvedCallableSourceIdentityV1;
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

pub(in crate::mir) use loop_break_source::LoopBreakSourcePackageTakeHandle;
pub(crate) use signature_loan::ResolvedCallablePhysicalSignatureLoanV1;
pub(crate) use types::PreparedNormalCallableSemanticPackageInstallV1;
pub(in crate::mir) use types::SelectedCallableSemanticRefV1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum NormalCallableSemanticPackageInstallIssueV1 {
    ForeignCatalog,
    MapLifecycleConsumerMissing,
    MapReadPhysicalConsumerMissing {
        site: crate::mir::resolved_semantics::OwnedExprSiteV1,
    },
    /// The sealed obligation describe failed before capability matching:
    /// keep the typed owner/site cause instead of flattening it.
    MapObligationDescribe(super::map_lifecycle_undertaking::MapObligationDescribeIssueV1),
    /// The described obligations exist but the declared consumer
    /// capability cannot cover them; the typed operation stays visible.
    MapLifecycleUndertaking(super::map_lifecycle_undertaking::MapLifecycleUndertakingIssueV1),
    MapLocalAnnotation(Box<str>),
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
    LoweringNotCompleted,
    CoreMethodSource(Box<str>),
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
    lowering_completed: std::cell::Cell<bool>,
    catalog_brand: SameModuleCallableCatalogBrandV1,
    batch: crate::mir::callable_semantic_batch::VerifiedResolvedCallableSemanticBatchV1,
    direct_call_loans: Option<super::direct_call_loan::DirectCallDispositionLoansV1>,
    map_read_facts: super::map_read_fact::MapReadFactsV1,
    /// Sealed pre-install proof that every owner's described Map
    /// obligations are covered by the selected consumer's declared
    /// capability. `None` when no member carries Map obligations.
    map_lifecycle_undertaking: Option<super::MapLifecycleUndertakingV1>,
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
    loop_break_source: RefCell<super::loop_break_source::VerifiedLoopBreakSourcePackageV1>,
    named_array_emissions: Rc<super::NamedArrayEmissionCollectorV1>,
    source_core_method_calls: RefCell<
        std::collections::BTreeMap<
            SelectedNormalCallableKeyV1,
            std::collections::BTreeMap<
                crate::mir::resolved_semantics::SourceExprSiteV1,
                crate::mir::normal_callable_semantic_package::SelectedSourceCoreMethodCallV1,
            >,
        >,
    >,
    app_main_qualified_receiver_catalog:
        RefCell<Option<super::model::VerifiedQualifiedReceiverCatalogRelationV1>>,
    app_main_qualified_receiver_catalog_taken: Cell<bool>,
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
    signature: ResolvedCallablePhysicalSignatureLoanV1<'loan>,
    _role: crate::mir::builder::SelectedCallableConsumptionRoleV1,
    _catalog_brand: SameModuleCallableCatalogBrandV1,
}

impl<'loan> MainStaticChildLoweringInputV1<'loan> {
    pub(in crate::mir) fn into_lowering_and_admission(
        self,
    ) -> (
        SelectedCallableLoweringInputRefV1<'loan>,
        NormalCatalogedBoxMethodDraftAdmissionV1,
        ResolvedCallablePhysicalSignatureLoanV1<'loan>,
    ) {
        (self.selected, self.admission, self.signature)
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

/// Exactly-once lowering surface for one installed package.
///
/// The port borrows the whole installed package and never reveals a batch
/// slot.  It records selected-key consumption and must be completed before
/// the outer lowering transaction can close.
#[must_use]
pub(crate) struct NormalCallableSemanticPackagePortV1<'package> {
    pub(super) installed: &'package InstalledNormalCallableSemanticPackageV1,
    pub(super) direct_call_loans: Option<super::direct_call_loan::DirectCallDispositionLoansV1>,
    consumed: BTreeSet<SelectedNormalCallableKeyV1>,
    declared_instance_consumed: RefCell<BTreeSet<u32>>,
    s6c_child_consumed: bool,
    main_root_consumed: bool,
}

impl NormalCallableSemanticPackagePortV1<'_> {
    pub(in crate::mir) fn map_read_facts_snapshot(&self) -> super::map_read_fact::MapReadFactsV1 {
        self.installed.map_read_facts().clone()
    }

    pub(in crate::mir) fn take_object_definitions(
        &mut self,
        context: &CompilationContext,
    ) -> Result<Box<[crate::mir::function::CanonicalObjectDefinitionV1]>, String> {
        if !self.installed.installed_in(context) {
            return Err("[freeze:contract][mir/object-definitions/foreign-package]".into());
        }
        self.installed
            .instance_constructors
            .take_object_definitions()
            .ok_or_else(|| "[freeze:contract][mir/object-definitions/already-taken]".into())
    }

    /// Lend the already-issued locator without exposing package ownership or
    /// allowing the view to outlive this callback.  This is transport only;
    /// selected-C admission remains a separate downstream boundary.
    pub(in crate::mir) fn with_declared_instance_call_locators<R>(
        &mut self,
        callback: impl for<'view> FnOnce(DeclaredInstanceCallLocatorScopeV1<'view>) -> R,
    ) -> R {
        let consumed = &self.declared_instance_consumed;
        self.installed.with_declared_instance_call_locators(|view| {
            callback(DeclaredInstanceCallLocatorScopeV1::new(view, consumed))
        })
    }

    /// Move the package-owned per-owner inventories into the root raw
    /// session.  There is no package-only fallback once this succeeds.
    pub(in crate::mir) fn take_direct_call_loans(
        &mut self,
    ) -> Option<super::direct_call_loan::DirectCallDispositionLoansV1> {
        self.direct_call_loans.take()
    }

    pub(in crate::mir) fn take_app_main_qualified_receiver_catalog(
        &mut self,
    ) -> Result<Option<super::model::VerifiedQualifiedReceiverCatalogRelationV1>, Box<str>> {
        self.installed.take_app_main_qualified_receiver_catalog()
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
            .prepare_install_with_consumer(context, &consumer)
            .map_err(|(_, issue)| issue)?;
        let installed = prepared.commit();
        Ok(consumer.seal(installed, BuilderInstallTokenV1::issue()))
    }

    pub(crate) fn prepare_install<'context>(
        self,
        context: &'context mut CompilationContext,
    ) -> Result<
        PreparedNormalCallableSemanticPackageInstallV1<'context>,
        (Self, NormalCallableSemanticPackageInstallIssueV1),
    > {
        let map_lifecycle_undertaking = match self.preflight_map_install(None) {
            Ok(undertaking) => undertaking,
            Err(issue) => return Err((self, issue)),
        };
        if !context.callable_declaration_catalog_vacant() {
            return Err((
                self,
                NormalCallableSemanticPackageInstallIssueV1::CatalogSlotOccupied,
            ));
        }
        Ok(PreparedNormalCallableSemanticPackageInstallV1 {
            context,
            package: self,
            map_lifecycle_undertaking,
        })
    }

    pub(in crate::mir) fn prepare_install_with_consumer<'context>(
        self,
        context: &'context mut CompilationContext,
        consumer: &BuilderInstallConsumerV1,
    ) -> Result<
        PreparedNormalCallableSemanticPackageInstallV1<'context>,
        (Self, NormalCallableSemanticPackageInstallIssueV1),
    > {
        let map_lifecycle_undertaking = match self.preflight_map_install(Some(consumer)) {
            Ok(undertaking) => undertaking,
            Err(issue) => return Err((self, issue)),
        };
        if !context.callable_declaration_catalog_vacant() {
            return Err((
                self,
                NormalCallableSemanticPackageInstallIssueV1::CatalogSlotOccupied,
            ));
        }
        Ok(PreparedNormalCallableSemanticPackageInstallV1 {
            context,
            package: self,
            map_lifecycle_undertaking,
        })
    }
}

impl PreparedNormalCallableSemanticPackageInstallV1<'_> {
    pub(crate) fn commit(self) -> InstalledNormalCallableSemanticPackageV1 {
        let VerifiedNormalCallableSemanticPackageV1 {
            root_execution,
            catalog,
            batch,
            direct_call_loans,
            map_read_facts,
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
            loop_break_source,
            declared_instance_call_locators,
            source_core_method_calls,
            app_main_qualified_receiver_catalog,
        } = self.package;
        match root_execution {
            NormalRootExecutionPackageStateV1::Prepared(root) => root.discard_unconnected(),
            NormalRootExecutionPackageStateV1::MovedToLowering => {}
        }
        let named_array_emissions = Rc::new(
            super::NamedArrayEmissionCollectorV1::from_source_rows(&source_core_method_calls),
        );
        let catalog_brand = catalog.catalog().brand().clone();
        self.context
            .install_callable_declaration_catalog_preflighted(catalog.into_catalog());
        InstalledNormalCallableSemanticPackageV1 {
            lowering_completed: std::cell::Cell::new(false),
            catalog_brand,
            batch,
            direct_call_loans,
            map_read_facts,
            map_lifecycle_undertaking: self.map_lifecycle_undertaking,
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
            loop_break_source: RefCell::new(loop_break_source),
            named_array_emissions,
            source_core_method_calls: RefCell::new(source_core_method_calls),
            app_main_qualified_receiver_catalog: RefCell::new(app_main_qualified_receiver_catalog),
            app_main_qualified_receiver_catalog_taken: Cell::new(false),
        }
    }
}

impl InstalledNormalCallableSemanticPackageV1 {
    pub(in crate::mir) fn take_app_main_qualified_receiver_catalog(
        &self,
    ) -> Result<Option<super::model::VerifiedQualifiedReceiverCatalogRelationV1>, Box<str>> {
        if self.app_main_qualified_receiver_catalog_taken.replace(true) {
            return Err("[freeze:contract][mir/main-qualified-relation/already-taken]".into());
        }
        Ok(self.app_main_qualified_receiver_catalog.borrow_mut().take())
    }

    pub(crate) fn take_source_core_method_calls(
        &self,
        key: &SelectedNormalCallableKeyV1,
    ) -> std::collections::BTreeMap<
        crate::mir::resolved_semantics::SourceExprSiteV1,
        crate::mir::normal_callable_semantic_package::SelectedSourceCoreMethodCallV1,
    > {
        self.source_core_method_calls
            .borrow_mut()
            .remove(key)
            .unwrap_or_default()
    }

    pub(in crate::mir) fn map_read_facts(&self) -> &super::map_read_fact::MapReadFactsV1 {
        &self.map_read_facts
    }

    /// The undertaking sealed at pre-install: every owner's described Map
    /// obligations covered by the selected consumer's declared capability.
    pub(in crate::mir) fn map_lifecycle_undertaking(
        &self,
    ) -> Option<&super::MapLifecycleUndertakingV1> {
        self.map_lifecycle_undertaking.as_ref()
    }

    pub(in crate::mir) fn finish_lowering(
        self,
    ) -> Result<VerifiedCallableResultContractCohortV1, NormalCallableSemanticPackageInstallIssueV1>
    {
        if !self.lowering_completed.get() {
            return Err(NormalCallableSemanticPackageInstallIssueV1::LoweringNotCompleted);
        }
        let rows = Rc::try_unwrap(self.named_array_emissions)
            .map_err(|_| {
                NormalCallableSemanticPackageInstallIssueV1::CoreMethodSource(
                    "named-array-collector-loan".into(),
                )
            })?
            .finish()
            .map_err(|error| {
                NormalCallableSemanticPackageInstallIssueV1::CoreMethodSource(error.into())
            })?;
        self.result_contracts
            .retain_completed_context(self.selected, self.parameter_contracts)?
            .retain_named_array_emissions(rows)
    }

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
        direct_call_loans: Option<super::direct_call_loan::DirectCallDispositionLoansV1>,
    ) -> Result<NormalCallableSemanticPackagePortV1<'_>, NormalCallableSemanticPackageInstallIssueV1>
    {
        if !self.installed_in(context) {
            return Err(NormalCallableSemanticPackageInstallIssueV1::ForeignCatalog);
        }
        self.open_lowering_port_after_install(direct_call_loans)
    }

    pub(crate) fn open_lowering_port_after_install(
        &self,
        direct_call_loans: Option<super::direct_call_loan::DirectCallDispositionLoansV1>,
    ) -> Result<NormalCallableSemanticPackagePortV1<'_>, NormalCallableSemanticPackageInstallIssueV1>
    {
        if self.direct_call_loans.is_some() {
            return Err(NormalCallableSemanticPackageInstallIssueV1::DirectCallLoanNotConsumed);
        }
        Ok(NormalCallableSemanticPackagePortV1 {
            installed: self,
            direct_call_loans,
            consumed: BTreeSet::new(),
            declared_instance_consumed: RefCell::new(BTreeSet::new()),
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

    pub(in crate::mir) fn take_direct_call_loans(
        &mut self,
    ) -> Option<super::direct_call_loan::DirectCallDispositionLoansV1> {
        self.direct_call_loans.take()
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
}
