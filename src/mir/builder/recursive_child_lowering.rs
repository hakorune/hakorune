//! Recursive child-lowering capability carriers.
//!
//! The raw invocation carrier owns one invocation-local borrow of the module
//! lowering state and forwards source, ledger, route, and cleanup capabilities
//! into shorter recursive frames. It is not a semantic issuer: it does not
//! invent source identity, targets, or fallback policy. The small legacy port
//! below remains a separate compatibility facade.
use crate::ast::{ASTNode, BoxMethodInventoryV1, DeclarationAttrs, ParamDecl};
use crate::mir::resolved_semantics::{ScriptResolverDeferredV1, SourceNodeSiteV1};
use crate::mir::{MirBuilder, ValueId};
use std::cell::RefCell;
use std::rc::Rc;

use super::calls::LegacyFunctionPendingSessionV1;
use super::control_flow::cleanup::CleanupExitPolicyV1;
use super::function_signature_lookup::FunctionSignatureLookupV1;
use super::generic_loop_admission_observation::GenericLoopAdmissionDiagnosticStateV1;
use super::me_call_header_observation::{
    MeCallHeaderObservationPortV1, MeCallHeaderSourceV1, MeCallParameterObservationV1,
};
use super::module_lowering_invocation::{
    LoweringHeaderPortV1, ModuleLoweringPortChildErrorV1, ModuleLoweringPortV1,
};
use super::normal_callable_semantic_lowering_state::CallableSemanticLoweringState;
use super::normal_script_semantic_lowering_state::ScriptSemanticLoweringState;
use super::port_aware_function_draft_impl::PortAwarePreparedDraftBodyV1;
use super::raw_compat_runtime_box_fate::{
    RawCompatibilityRuntimeBoxFateV1, RawRuntimeBoxFateDispositionV1, RuntimeBoxFateScopeV1,
};
use super::raw_expression_dispatch::RawExpressionDispatchPortV1;
pub(in crate::mir::builder) use super::raw_expression_recursion_guard::with_legacy_expression_recursion_guard_v1;
use super::raw_invocation_source_transport::{
    RawInvocationRootLineageV1, RawInvocationSourceContextV1, RawInvocationSourceTransportV1,
    RawSourceTransportPortV1,
};
use crate::parser::CallableMethodSourceObservationV1;

mod legacy_port;
#[path = "recursive_child_lowering/pending_helpers.rs"]
mod pending_helpers;
#[path = "raw_ordinary_new_claim.rs"]
mod raw_ordinary_new_claim;
#[path = "normal_script_direct_static_claim_transport.rs"]
mod script_direct_static_claim_transport;

pub(in crate::mir::builder) use legacy_port::{
    drive_raw_legacy_body_v1, drive_raw_legacy_expression_v1, drive_raw_legacy_statement_v1,
    RawLegacyChildLoweringPortV1,
};
pub(in crate::mir::builder) use raw_ordinary_new_claim::RawOrdinaryNewClaimPortV1;

pub(in crate::mir::builder) use super::raw_loop_child_port::RawLoopChildEntryPortV1;
pub(in crate::mir::builder) use super::recursive_child_lowering_port::{
    AppMainDirectCallDispositionPortV1, DeclaredInstanceReceiverIngressV1,
    RawAstChildLoweringPortV1, RecursiveChildLoweringPortV1,
};

pub(in crate::mir::builder) fn normalize_instance_box_method_input_v1(
    function_name: &str,
    params: Vec<String>,
    param_decls: Vec<ParamDecl>,
) -> (Vec<String>, Vec<ParamDecl>) {
    (
        super::calls::lowering::normalize_instance_method_params(function_name, params),
        super::calls::lowering::normalize_instance_method_param_decls(function_name, param_decls),
    )
}

pub(in crate::mir::builder) trait RawFunctionHeaderLookupPortV1 {
    fn with_function_headers<R>(
        &mut self,
        observe: impl for<'headers> FnOnce(Option<&'headers dyn FunctionSignatureLookupV1>) -> R,
    ) -> R;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::mir::builder) enum RawNestedMainFateV1 {
    ContinueExistingTerminal,
    RetireRawLegacyBeforeEffects,
}

pub(in crate::mir::builder) trait RawBoxMethodChildPortV1 {
    /// These two finite fate queries are the complete route-policy surface of
    /// this legacy port. Future retirements use an owner-specific caller
    /// switch or capability instead of adding more policy methods here.
    fn take_runtime_box_fate_v1(&mut self) -> Result<RawRuntimeBoxFateDispositionV1, String> {
        Ok(RawRuntimeBoxFateDispositionV1::Continue)
    }

    fn nested_main_fate_v1(&mut self) -> RawNestedMainFateV1 {
        RawNestedMainFateV1::ContinueExistingTerminal
    }

    fn lower_static_main_box(
        &mut self,
        builder: &mut MirBuilder,
        box_name: String,
        methods: BoxMethodInventoryV1,
    ) -> Result<ValueId, String>;

    fn lower_static_box_method(
        &mut self,
        _builder: &mut MirBuilder,
        _function_name: String,
        _params: Vec<String>,
        _param_decls: Vec<ParamDecl>,
        _return_type_name: Option<String>,
        _body: Vec<ASTNode>,
        _uses: Vec<String>,
        _attrs: DeclarationAttrs,
    ) -> Result<(), String> {
        Err("[freeze:contract][raw-box-method/loose-static-input]".to_owned())
    }

    fn lower_nested_box_method(
        &mut self,
        builder: &mut MirBuilder,
        input: super::nested_box_method_source::NestedBoxMethodLoweringInputV1,
    ) -> Result<(), String> {
        let (_, function_name, kind, params, param_decls, return_type_name, body, uses, attrs) =
            input.into_parts();
        match kind {
            super::nested_box_method_source::NestedBoxMethodKindV1::Static => self
                .lower_static_box_method(
                    builder,
                    function_name,
                    params,
                    param_decls,
                    return_type_name,
                    body,
                    uses,
                    attrs,
                ),
            super::nested_box_method_source::NestedBoxMethodKindV1::Instance { owner } => self
                .lower_instance_box_method(
                    builder,
                    function_name,
                    owner,
                    params,
                    param_decls,
                    return_type_name,
                    body,
                    uses,
                    attrs,
                ),
        }
    }

    fn lower_instance_box_method(
        &mut self,
        _builder: &mut MirBuilder,
        _function_name: String,
        _box_name: String,
        _params: Vec<String>,
        _param_decls: Vec<ParamDecl>,
        _return_type_name: Option<String>,
        _body: Vec<ASTNode>,
        _uses: Vec<String>,
        _attrs: DeclarationAttrs,
    ) -> Result<(), String> {
        Err("[freeze:contract][raw-box-method/loose-instance-input]".to_owned())
    }
}

impl<Port> RawAstChildLoweringPortV1 for Port where
    Port: RecursiveChildLoweringPortV1<
        BodyInput = Vec<ASTNode>,
        StatementInput = ASTNode,
        ExpressionInput = ASTNode,
    >
{
}

pub(in crate::mir::builder) fn drive_legacy_body_v1<Port>(
    builder: &mut MirBuilder,
    port: &mut Port,
    input: Port::BodyInput,
) -> Result<ValueId, String>
where
    Port: RecursiveChildLoweringPortV1,
{
    port.lower_body(builder, input)
}

pub(in crate::mir::builder) fn drive_legacy_statement_v1<Port>(
    builder: &mut MirBuilder,
    port: &mut Port,
    input: Port::StatementInput,
) -> Result<ValueId, String>
where
    Port: RecursiveChildLoweringPortV1,
{
    port.lower_statement(builder, input)
}

pub(in crate::mir::builder) fn drive_legacy_expression_v1<Port>(
    builder: &mut MirBuilder,
    port: &mut Port,
    input: Port::ExpressionInput,
) -> Result<ValueId, String>
where
    Port: RecursiveChildLoweringPortV1,
{
    port.lower_expression(builder, input)
}

/// Stack-owned raw-recursion capability for one module-lowering invocation.
///
/// Ordinary root lowering now uses this carrier for body, statement, and
/// expression descent while callable drafts accumulate in one invocation-local
/// collector. It owns neither a Builder, collector, header view, nor AST cache.
/// It does carry finite route capabilities borrowed from their existing owners
/// (for example the selected App Main direct-call loan and runtime-box fate),
/// and only reborrows those capabilities for a shorter recursive frame.
///
/// Keeping this wrapper separate from `RawLegacyChildLoweringPortV1` makes the
/// collector-backed production route and the direct compatibility facade
/// mechanically distinct.
pub(in crate::mir::builder) struct RawInvocationChildPortV1<'port, 'collector> {
    pub(in crate::mir::builder) module_port: &'port mut ModuleLoweringPortV1<'collector>,
    pub(in crate::mir::builder) active_source: Option<RawInvocationSourceContextV1>,
    pub(in crate::mir::builder) semantic_ledger: Option<Rc<RefCell<ScriptSemanticLoweringState>>>,
    pub(in crate::mir::builder) callable_ledger: Option<Rc<RefCell<CallableSemanticLoweringState>>>,
    pub(in crate::mir::builder) ordinary_new_claim_ledger:
        Option<Rc<crate::mir::normal_callable_semantic_package::OrdinaryNewClaimLedgerV1>>,
    pub(in crate::mir::builder) generic_loop_diagnostic: GenericLoopAdmissionDiagnosticStateV1,
    /// Source-only Script resolver deferral carried through the existing raw
    /// runtime owner. It does not select a route or issue a fallback.
    pub(in crate::mir::builder) script_deferred_observation: Option<ScriptResolverDeferredV1>,
    /// Root-scoped permission for the source-aware callable Loop consumer.
    ///
    /// This is borrowed from the existing unpublished module invocation and
    /// is propagated through recursive frames. It is not a second Builder or
    /// function session and cannot outlive the root callback.
    pub(in crate::mir::builder) callable_loop_root_scope:
        Option<&'port mut super::UnpublishedCallableLoopRootScopeV1>,
    /// Exact App Main direct-call dispositions borrowed from the installed
    /// package for this invocation only.  Children receive a short reborrow;
    /// no clone or second inventory is created.
    pub(in crate::mir::builder) direct_call_loan: Option<
        &'port mut crate::mir::normal_callable_semantic_package::AppMainDirectCallDispositionLoanV1,
    >,
    /// Short-lived package locator capability for the selected DeclaredInstance
    /// root. Compatibility frames remain explicitly unarmed.
    pub(in crate::mir::builder) declared_instance_locator: Option<
        crate::mir::normal_callable_semantic_package::DeclaredInstanceCallLocatorScopeV1<'port>,
    >,
    pub(in crate::mir::builder) runtime_box_fate: RuntimeBoxFateScopeV1<'port>,
    pub(in crate::mir::builder) cleanup_exit_policy: CleanupExitPolicyV1,
    _seal: RawInvocationChildPortSealV1,
}

struct RawInvocationChildPortSealV1;

impl<'port, 'collector> RawInvocationChildPortV1<'port, 'collector> {
    /// Start one raw recursive frame from the exact invocation port.
    pub(in crate::mir::builder) fn new(
        module_port: &'port mut ModuleLoweringPortV1<'collector>,
    ) -> Self {
        Self::new_with_cleanup_exit_policy(
            module_port,
            CleanupExitPolicyV1::capture_from_environment(),
        )
    }

    pub(in crate::mir::builder) fn new_with_cleanup_exit_policy(
        module_port: &'port mut ModuleLoweringPortV1<'collector>,
        cleanup_exit_policy: CleanupExitPolicyV1,
    ) -> Self {
        Self::new_with_optional_callable_loop_root_scope(module_port, cleanup_exit_policy, None)
    }

    pub(in crate::mir::builder) fn new_with_cleanup_exit_policy_and_callable_loop_scope(
        module_port: &'port mut ModuleLoweringPortV1<'collector>,
        cleanup_exit_policy: CleanupExitPolicyV1,
        callable_loop_root_scope: &'port mut super::UnpublishedCallableLoopRootScopeV1,
    ) -> Self {
        Self::new_with_optional_callable_loop_root_scope(
            module_port,
            cleanup_exit_policy,
            Some(callable_loop_root_scope),
        )
    }

    pub(in crate::mir::builder) fn new_with_cleanup_exit_policy_and_callable_loop_scope_and_direct_call_loan(
        module_port: &'port mut ModuleLoweringPortV1<'collector>,
        cleanup_exit_policy: CleanupExitPolicyV1,
        callable_loop_root_scope: &'port mut super::UnpublishedCallableLoopRootScopeV1,
        direct_call_loan: Option<&'port mut crate::mir::normal_callable_semantic_package::AppMainDirectCallDispositionLoanV1>,
    ) -> Self {
        Self::new_with_optional_callable_loop_root_scope_and_direct_call_loan(
            module_port,
            cleanup_exit_policy,
            Some(callable_loop_root_scope),
            direct_call_loan,
        )
    }

    fn new_with_optional_callable_loop_root_scope(
        module_port: &'port mut ModuleLoweringPortV1<'collector>,
        cleanup_exit_policy: CleanupExitPolicyV1,
        callable_loop_root_scope: Option<&'port mut super::UnpublishedCallableLoopRootScopeV1>,
    ) -> Self {
        Self::new_with_optional_callable_loop_root_scope_and_direct_call_loan(
            module_port,
            cleanup_exit_policy,
            callable_loop_root_scope,
            None,
        )
    }

    fn new_with_optional_callable_loop_root_scope_and_direct_call_loan(
        module_port: &'port mut ModuleLoweringPortV1<'collector>,
        cleanup_exit_policy: CleanupExitPolicyV1,
        callable_loop_root_scope: Option<&'port mut super::UnpublishedCallableLoopRootScopeV1>,
        direct_call_loan: Option<&'port mut crate::mir::normal_callable_semantic_package::AppMainDirectCallDispositionLoanV1>,
    ) -> Self {
        Self {
            module_port,
            active_source: None,
            semantic_ledger: None,
            callable_ledger: None,
            ordinary_new_claim_ledger: None,
            generic_loop_diagnostic: GenericLoopAdmissionDiagnosticStateV1::new(),
            script_deferred_observation: None,
            callable_loop_root_scope,
            direct_call_loan,
            declared_instance_locator: None,
            runtime_box_fate: RuntimeBoxFateScopeV1::Unarmed,
            cleanup_exit_policy,
            _seal: RawInvocationChildPortSealV1,
        }
    }

    /// Reborrow the same invocation capability for one nested raw frame.
    ///
    /// No header borrow crosses this boundary: `with_headers` consumes the
    /// observation closure before the next descendant can mutate state.
    pub(in crate::mir::builder) fn reborrow(&mut self) -> RawInvocationChildPortV1<'_, 'collector> {
        RawInvocationChildPortV1 {
            module_port: &mut *self.module_port,
            active_source: self.active_source.clone(),
            semantic_ledger: self.semantic_ledger.clone(),
            callable_ledger: self.callable_ledger.clone(),
            ordinary_new_claim_ledger: self.ordinary_new_claim_ledger.clone(),
            generic_loop_diagnostic: self.generic_loop_diagnostic.reborrow(),
            script_deferred_observation: self.script_deferred_observation.clone(),
            callable_loop_root_scope: self.callable_loop_root_scope.as_deref_mut(),
            direct_call_loan: self.direct_call_loan.as_deref_mut(),
            declared_instance_locator: self
                .declared_instance_locator
                .as_mut()
                .map(|locator| locator.reborrow()),
            runtime_box_fate: self.runtime_box_fate.reborrow(),
            cleanup_exit_policy: self.cleanup_exit_policy,
            _seal: RawInvocationChildPortSealV1,
        }
    }

    /// Arm one narrowly scoped phase2160 RawCompatibility runtime-Box
    /// lowering frame.  The capability is local to this callback and is
    /// reborrowed by recursive children; no generic/raw-legacy route inherits
    /// it.
    pub(in crate::mir::builder) fn with_phase2160_raw_compat_runtime_box_fate_v1<R>(
        &mut self,
        execute: impl for<'scope> FnOnce(
            &mut RawInvocationChildPortV1<'scope, 'collector>,
        ) -> Result<R, String>,
    ) -> Result<R, String> {
        if self.runtime_box_fate.is_armed() {
            return Err("[freeze:contract][raw-compat/runtime-box-fate-scope]".to_owned());
        }
        let mut fate = RawCompatibilityRuntimeBoxFateV1::issue_retire();
        let mut scoped = self.reborrow();
        scoped.runtime_box_fate = RuntimeBoxFateScopeV1::Phase2160(&mut fate);
        execute(&mut scoped)
    }

    /// Run one selected DeclaredInstance body with a package-owned locator
    /// capability.  The capability is installed only on this short raw frame;
    /// recursive children receive a reborrow and compatibility frames remain
    /// unarmed.
    pub(in crate::mir::builder) fn with_declared_instance_locator_scope<R>(
        &mut self,
        locator: crate::mir::normal_callable_semantic_package::DeclaredInstanceCallLocatorScopeV1<
            '_,
        >,
        execute: impl for<'scope> FnOnce(
            &mut RawInvocationChildPortV1<'scope, 'collector>,
        ) -> Result<R, String>,
    ) -> Result<R, String> {
        let mut scoped = self.reborrow();
        scoped.declared_instance_locator = Some(locator);
        execute(&mut scoped)
    }

    /// Take the exact source-backed receiver for the current method-call site.
    /// The locator row is consumed once; the callable state only reads the
    /// already-materialized ValueId by the relation's BindingRef.
    pub(in crate::mir::builder) fn take_declared_instance_receiver_value_inner_v1(
        &mut self,
        _builder: &MirBuilder,
    ) -> Result<DeclaredInstanceReceiverIngressV1, String> {
        if self.declared_instance_locator.is_none() {
            return Ok(DeclaredInstanceReceiverIngressV1::Unarmed);
        }
        let owner = self
            .callable_owner_v1()
            .ok_or_else(|| "[freeze:contract][declared-instance/owner-unavailable]".to_owned())?;
        let site = self.current_source_site_v1().ok_or_else(|| {
            "[freeze:contract][declared-instance/call-site-unavailable]".to_owned()
        })?;
        let expected_site = crate::mir::resolved_semantics::OwnedExprSiteV1::new(
            owner,
            crate::mir::resolved_semantics::SourceExprSiteV1::from_node(site),
        );
        let ledger = self
            .callable_ledger
            .as_ref()
            .ok_or_else(|| "[freeze:contract][declared-instance/state-unavailable]".to_owned())?;
        let locator = self
            .declared_instance_locator
            .as_mut()
            .expect("checked DeclaredInstance locator capability");
        locator
            .take_exact_relation(&expected_site, |relation| {
                if relation.caller_owner() != owner || relation.call_site() != expected_site.site()
                {
                    return Err("[freeze:contract][declared-instance/relation-mismatch]".to_owned());
                }
                let mut state = ledger.borrow_mut();
                state
                    .take_exact_receiver_value(
                        owner,
                        relation.receiver_site().node(),
                        relation.receiver_binding(),
                    )
                    .map(|receiver| DeclaredInstanceReceiverIngressV1::Ready {
                        key: relation.target_key().clone(),
                        receiver,
                    })
                    .map_err(|error| error.to_string())
            })
            .map_err(|error| format!("[freeze:contract][declared-instance/locator/{error:?}]"))
    }

    pub(in crate::mir::builder) fn take_runtime_box_fate_v1(
        &mut self,
    ) -> Result<RawRuntimeBoxFateDispositionV1, String> {
        self.runtime_box_fate.take_retire()
    }

    pub(in crate::mir::builder) fn with_script_deferred_observation<R>(
        &mut self,
        observation: ScriptResolverDeferredV1,
        execute: impl FnOnce(&mut Self) -> R,
    ) -> R {
        let parent = self.script_deferred_observation.replace(observation);
        let result = execute(self);
        self.script_deferred_observation = parent;
        result
    }

    /// Lend the exact collector-backed header view for one observation only.
    pub(in crate::mir::builder) fn with_headers<R>(
        &self,
        observe: impl for<'header> FnOnce(&'header LoweringHeaderPortV1<'header>) -> R,
    ) -> R {
        self.module_port.with_headers(observe)
    }

    /// Delegate the invocation-owned brand without retaining it in the raw
    /// child port. This keeps provider admission tied to its collector.
    pub(in crate::mir::builder) fn with_invocation_brand<R>(
        &self,
        observe: impl FnOnce(crate::mir::module_invocation_identity::ModuleInvocationBrandV1) -> R,
    ) -> Result<R, super::module_draft_collector::CollectorReceiptBrandErrorV1> {
        self.module_port.with_invocation_brand(observe)
    }

    /// Transport one parser-issued method observation through the raw child
    /// port for exactly one callable lowering scope.  This is diagnostic
    /// provenance only; it never selects a route or repairs source identity.
    pub(in crate::mir::builder) fn with_callable_method_source_observation<R>(
        &mut self,
        observation: Option<CallableMethodSourceObservationV1>,
        execute: impl FnOnce(&mut Self) -> R,
    ) -> R {
        let previous = self
            .generic_loop_diagnostic
            .replace_method_source(observation);
        let result = execute(self);
        self.generic_loop_diagnostic.replace_method_source(previous);
        result
    }

    pub(in crate::mir::builder) fn local_initializer_observation_sink(
        &self,
    ) -> super::stmts::LocalInitializerObservationSinkV1 {
        self.generic_loop_diagnostic.local_initializer_sink()
    }

    pub(in crate::mir::builder) fn current_source_site_v1(&self) -> Option<SourceNodeSiteV1> {
        self.active_source
            .as_ref()
            .and_then(RawInvocationSourceContextV1::site)
            .cloned()
    }

    pub(in crate::mir::builder) fn callable_owner_v1(
        &self,
    ) -> Option<crate::mir::resolved_semantics::FunctionOwnerIdV1> {
        self.callable_ledger
            .as_ref()
            .map(|ledger| ledger.borrow().owner())
    }

    pub(in crate::mir::builder) fn issue_callable_loop_binding_schedule_v1(
        &self,
    ) -> Result<
        Option<super::normal_callable_loop_handoff::CallableLoopBindingProjectionDispositionV1>,
        String,
    > {
        let Some(ledger) = self.callable_ledger.as_ref() else {
            return Ok(None);
        };
        let loop_site = self
            .active_source
            .as_ref()
            .and_then(RawInvocationSourceContextV1::site)
            .cloned()
            .ok_or_else(|| {
                "[freeze:contract][callable-loop-handoff/missing-loop-source]".to_owned()
            })?;
        let state = ledger.borrow();
        state
            .loop_binding_source_projection()
            .project_disposition(loop_site)
            .map(Some)
    }

    /// Capture one raw static child while the same invocation port remains
    /// available to every recursive body descendant.  The header loan starts
    /// only after body descent has returned.
    pub(in crate::mir::builder) fn capture_static_box_method_pending_v1<'builder>(
        &mut self,
        builder: &'builder mut MirBuilder,
        function_name: String,
        params: Vec<String>,
        param_decls: Vec<ParamDecl>,
        return_type_name: Option<String>,
        body: Vec<ASTNode>,
        uses: Vec<String>,
        attrs: DeclarationAttrs,
    ) -> Result<LegacyFunctionPendingSessionV1<'builder>, ModuleLoweringPortChildErrorV1> {
        let body_snapshot = body.clone();
        let session_name = function_name.clone();
        let pending = {
            let mut child_port = self.reborrow();
            builder
                .capture_legacy_function_pending_session_v1(
                    &session_name,
                    body_snapshot,
                    move |builder| {
                        let prepared: PortAwarePreparedDraftBodyV1 = builder
                            .build_static_method_draft_with_port_v1(
                                &mut child_port,
                                function_name,
                                params,
                                param_decls,
                                return_type_name,
                                body,
                                uses,
                                attrs,
                            )?;
                        child_port.with_headers(|headers| {
                            builder.finalize_function_draft_with_headers(prepared, headers)
                        })
                    },
                )
                .map_err(ModuleLoweringPortChildErrorV1::Session)?
        };
        Ok(pending)
    }

    pub(in crate::mir::builder) fn commit_legacy_nested_box_method_symbol_pending_v1(
        &mut self,
        pending: LegacyFunctionPendingSessionV1<'_>,
        symbol: String,
        arity: usize,
    ) -> Result<(), ModuleLoweringPortChildErrorV1> {
        self.module_port.commit_legacy_symbol_pending(
            pending,
            (
                super::module_draft_collector::FunctionDraftKeyV1::LegacySymbol(symbol.clone()),
                symbol,
                arity,
            ),
        )
    }

    /// Instance counterpart of the port-aware capture seam.
    pub(in crate::mir::builder) fn capture_normalized_instance_box_method_pending_v1<'builder>(
        &mut self,
        builder: &'builder mut MirBuilder,
        function_name: String,
        box_name: String,
        params: Vec<String>,
        param_decls: Vec<ParamDecl>,
        return_type_name: Option<String>,
        body: Vec<ASTNode>,
        uses: Vec<String>,
        attrs: DeclarationAttrs,
    ) -> Result<LegacyFunctionPendingSessionV1<'builder>, ModuleLoweringPortChildErrorV1> {
        let body_snapshot = body.clone();
        let session_name = function_name.clone();
        let pending = {
            let mut child_port = self.reborrow();
            builder
                .capture_legacy_function_pending_session_v1(
                    &session_name,
                    body_snapshot,
                    move |builder| {
                        let prepared: PortAwarePreparedDraftBodyV1 = builder
                            .build_instance_method_draft_with_port_v1(
                                &mut child_port,
                                function_name,
                                box_name,
                                params,
                                param_decls,
                                return_type_name,
                                body,
                                uses,
                                attrs,
                            )?;
                        let function = child_port.with_headers(|headers| {
                            builder.finalize_function_draft_with_headers(prepared, headers)
                        })?;
                        if let Some(ledger) = &child_port.callable_ledger {
                            ledger
                                .borrow()
                                .validate_finalized_construction_stores(&function)?;
                            if let Some(news) = &child_port.ordinary_new_claim_ledger {
                                news.validate_new_emissions(ledger.borrow().owner(), &function)?;
                            }
                        }
                        Ok(function)
                    },
                )
                .map_err(ModuleLoweringPortChildErrorV1::Session)?
        };
        Ok(pending)
    }
}

impl AppMainDirectCallDispositionPortV1 for RawInvocationChildPortV1<'_, '_> {
    fn take_app_main_direct_call_disposition_v1(
        &mut self,
    ) -> Result<
        crate::mir::normal_callable_semantic_package::AppMainDirectCallDispositionRowV1,
        String,
    > {
        if !self.is_app_main_direct_call_scope_v1() {
            return Err("[freeze:contract][app-main-direct-call/scope-mismatch]".to_owned());
        }
        self.take_app_main_direct_call_disposition_inner_v1()
    }

    fn validate_current_call_argument_site_v1(
        &self,
        expected: &crate::mir::resolved_semantics::SourceExprSiteV1,
    ) -> Result<(), String> {
        let actual = self
            .current_source_site_v1()
            .map(crate::mir::resolved_semantics::SourceExprSiteV1::from_node)
            .ok_or_else(|| {
                "[freeze:contract][app-main-direct-call/argument-site-missing]".to_owned()
            })?;
        if &actual != expected {
            return Err(
                "[freeze:contract][app-main-direct-call/argument-site-mismatch]".to_owned(),
            );
        }
        Ok(())
    }
}

impl RawBoxMethodChildPortV1 for RawInvocationChildPortV1<'_, '_> {
    fn take_runtime_box_fate_v1(&mut self) -> Result<RawRuntimeBoxFateDispositionV1, String> {
        RawInvocationChildPortV1::take_runtime_box_fate_v1(self)
    }

    fn lower_static_main_box(
        &mut self,
        _builder: &mut MirBuilder,
        box_name: String,
        _methods: BoxMethodInventoryV1,
    ) -> Result<ValueId, String> {
        Err(
            super::control_flow::lower::Freeze::contract(&format!(
                "raw_invocation_main_box: root-only Main box cannot be lowered as a nested child name={box_name}"
            ))
            .to_string(),
        )
    }

    fn lower_nested_box_method(
        &mut self,
        builder: &mut MirBuilder,
        input: super::nested_box_method_source::NestedBoxMethodLoweringInputV1,
    ) -> Result<(), String> {
        super::nested_box_method_source::lower_nested_box_method_v1(self, builder, input)
    }
}

impl RawFunctionHeaderLookupPortV1 for RawInvocationChildPortV1<'_, '_> {
    fn with_function_headers<R>(
        &mut self,
        observe: impl for<'headers> FnOnce(Option<&'headers dyn FunctionSignatureLookupV1>) -> R,
    ) -> R {
        self.with_headers(|headers| observe(Some(headers)))
    }
}

impl MeCallHeaderObservationPortV1 for RawInvocationChildPortV1<'_, '_> {
    fn observe_me_call_parameters(
        &mut self,
        _builder: &MirBuilder,
        symbol: &str,
    ) -> MeCallParameterObservationV1 {
        self.with_function_headers(|lookup| {
            MeCallParameterObservationV1::from_optional_lookup(
                MeCallHeaderSourceV1::InvocationCollector,
                symbol,
                lookup,
            )
        })
    }
}

pub(super) fn lower_raw_expression_with_recursion_guard_v1<Port>(
    builder: &mut MirBuilder,
    port: &mut Port,
    input: ASTNode,
) -> Result<ValueId, String>
where
    Port: RawExpressionDispatchPortV1,
{
    let node_kind = std::mem::discriminant(&input);
    super::raw_expression_recursion_guard::with_legacy_expression_recursion_guard_v1(
        builder,
        node_kind,
        move |builder| builder.build_expression_impl_with_port_v1(port, input),
    )
}
