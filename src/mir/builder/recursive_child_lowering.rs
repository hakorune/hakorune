//! Behavior-neutral recursive child-lowering port.
//! This module owns the typed body, statement, and expression entry boundary.
//! It owns no source navigation, callable-result plan, location, ledger,
//! MethodCall route, or result-publication policy.
use crate::ast::{ASTNode, BoxMethodInventoryV1, DeclarationAttrs, ParamDecl};
use crate::mir::resolved_semantics::{BodyChildRoleV1, ExprChildRoleV1};
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
use super::normal_callable_loop_handoff::VerifiedCallableSemanticLoopBindingScheduleV1;
use super::normal_callable_semantic_lowering_state::CallableSemanticLoweringState;
use super::normal_script_semantic_lowering_state::ScriptSemanticLoweringState;
use super::port_aware_function_draft_impl::PortAwarePreparedDraftBodyV1;
use super::raw_expression_dispatch::RawExpressionDispatchPortV1;
pub(in crate::mir::builder) use super::raw_expression_recursion_guard::with_legacy_expression_recursion_guard_v1;
use super::raw_invocation_source_transport::{
    RawInvocationRootLineageV1, RawInvocationSourceContextV1, RawInvocationSourceTransportV1,
    RawSourceTransportPortV1,
};
use super::raw_loop_child_entry::PreparedLocatedRawLoopChildEntryV1;
use super::raw_static_main_compat_batch::PreparedRawStaticMainBoxCompatibilityV1;
use super::raw_structured_child_scope::PreparedRawChildSourceV1;
use crate::parser::CallableMethodSourceObservationV1;

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

pub(in crate::mir::builder) trait RecursiveChildLoweringPortV1 {
    type BodyInput;
    type StatementInput;
    type ExpressionInput;

    fn lower_body(
        &mut self,
        builder: &mut MirBuilder,
        input: Self::BodyInput,
    ) -> Result<ValueId, String>;

    fn lower_statement(
        &mut self,
        builder: &mut MirBuilder,
        input: Self::StatementInput,
    ) -> Result<ValueId, String>;

    fn lower_expression(
        &mut self,
        builder: &mut MirBuilder,
        input: Self::ExpressionInput,
    ) -> Result<ValueId, String>;

    /// Optional source-bound static-call terminal.  The default keeps all
    /// compatibility/test ports on the ordinary terminal; the live raw
    /// invocation overrides this only after exact source identity and target
    /// proof have been sealed.
    fn try_emit_source_bound_static_call_result_v1(
        &mut self,
        _builder: &mut MirBuilder,
        _owner: &str,
        _method: &str,
        _checked_source_arity: u32,
        _arguments: &[ValueId],
    ) -> Result<Option<ValueId>, String> {
        Ok(None)
    }

    /// Isolated test-only ports deny cleanup exits unless they explicitly
    /// provide an operation policy. Production ports must override this.
    fn cleanup_exit_policy_v1(&self) -> CleanupExitPolicyV1 {
        CleanupExitPolicyV1::default()
    }

    fn prepare_expression_child_source_v1(
        &self,
        _parent: &ASTNode,
        _role: ExprChildRoleV1,
    ) -> Result<PreparedRawChildSourceV1, String> {
        Ok(PreparedRawChildSourceV1::Preserve)
    }
    fn prepare_body_child_source_v1(
        &self,
        _parent: &ASTNode,
        _role: BodyChildRoleV1,
    ) -> Result<PreparedRawChildSourceV1, String> {
        Ok(PreparedRawChildSourceV1::Preserve)
    }
    fn prepare_body_statement_source_v1(
        &self,
        _statement: &ASTNode,
        _index: usize,
    ) -> Result<PreparedRawChildSourceV1, String> {
        Ok(PreparedRawChildSourceV1::Preserve)
    }
    fn with_prepared_child_source_v1<R>(
        &mut self,
        _source: PreparedRawChildSourceV1,
        execute: impl FnOnce(&mut Self) -> R,
    ) -> R {
        execute(self)
    }

    fn with_call_argument_source_v1<R>(
        &mut self,
        _index: usize,
        execute: impl FnOnce(&mut Self) -> R,
    ) -> R {
        execute(self)
    }
}
pub(in crate::mir::builder) trait RawAstChildLoweringPortV1:
    RecursiveChildLoweringPortV1<
    BodyInput = Vec<ASTNode>,
    StatementInput = ASTNode,
    ExpressionInput = ASTNode,
>
{
}
pub(in crate::mir::builder) trait RawFunctionHeaderLookupPortV1 {
    fn with_function_headers<R>(
        &mut self,
        observe: impl for<'headers> FnOnce(Option<&'headers dyn FunctionSignatureLookupV1>) -> R,
    ) -> R;
}
pub(in crate::mir::builder) trait RawBoxMethodChildPortV1 {
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

/// One raw Loop child-entry boundary.
///
/// This boundary owns only the decision whether a raw invocation may delegate
/// to the existing JoinIR route owner. It does not pass the invocation port
/// into recipe composition, normalization, or plan lowering.
pub(in crate::mir::builder) trait RawLoopChildEntryPortV1 {
    fn lower_loop(
        &mut self,
        builder: &mut MirBuilder,
        loop_node: ASTNode,
    ) -> Result<ValueId, String>;
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

pub(in crate::mir::builder) struct RawLegacyChildLoweringPortV1;

impl MeCallHeaderObservationPortV1 for RawLegacyChildLoweringPortV1 {
    fn observe_me_call_parameters(
        &mut self,
        builder: &MirBuilder,
        symbol: &str,
    ) -> MeCallParameterObservationV1 {
        MeCallParameterObservationV1::from_optional_lookup(
            MeCallHeaderSourceV1::ModuleCompatibility,
            symbol,
            builder
                .current_module
                .as_ref()
                .map(|module| module as &dyn FunctionSignatureLookupV1),
        )
    }
}

/// Stack-owned raw-recursion capability for one module-lowering invocation.
///
/// Ordinary root lowering now uses this carrier for body, statement, and
/// expression descent while callable drafts accumulate in one invocation-local
/// collector. It owns neither a Builder, collector, header view, AST cache, nor
/// child-terminal authority; all it can do is reborrow the exact invocation
/// port for a shorter recursive frame.
///
/// Keeping this wrapper separate from `RawLegacyChildLoweringPortV1` makes the
/// collector-backed production route and the direct compatibility facade
/// mechanically distinct.
pub(in crate::mir::builder) struct RawInvocationChildPortV1<'port, 'collector> {
    pub(super) module_port: &'port mut ModuleLoweringPortV1<'collector>,
    pub(super) active_source: Option<RawInvocationSourceContextV1>,
    pub(super) semantic_ledger: Option<Rc<RefCell<ScriptSemanticLoweringState>>>,
    pub(super) callable_ledger: Option<Rc<RefCell<CallableSemanticLoweringState>>>,
    pub(super) generic_loop_diagnostic: GenericLoopAdmissionDiagnosticStateV1,
    pub(super) cleanup_exit_policy: CleanupExitPolicyV1,
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
        Self {
            module_port,
            active_source: None,
            semantic_ledger: None,
            callable_ledger: None,
            generic_loop_diagnostic: GenericLoopAdmissionDiagnosticStateV1::new(),
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
            generic_loop_diagnostic: self.generic_loop_diagnostic.reborrow(),
            cleanup_exit_policy: self.cleanup_exit_policy,
            _seal: RawInvocationChildPortSealV1,
        }
    }

    /// Lend the exact collector-backed header view for one observation only.
    pub(in crate::mir::builder) fn with_headers<R>(
        &self,
        observe: impl for<'header> FnOnce(&'header LoweringHeaderPortV1<'header>) -> R,
    ) -> R {
        self.module_port.with_headers(observe)
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

    pub(in crate::mir::builder) fn issue_callable_loop_binding_schedule_v1(
        &self,
    ) -> Result<Option<VerifiedCallableSemanticLoopBindingScheduleV1>, String> {
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
            .project(loop_site)
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

    pub(in crate::mir::builder) fn commit_normal_top_level_function_pending_v1(
        &mut self,
        pending: LegacyFunctionPendingSessionV1<'_>,
        admission: super::normal_top_level_function_admission::NormalTopLevelFunctionDraftAdmissionV1,
    ) -> Result<(), ModuleLoweringPortChildErrorV1> {
        self.module_port
            .commit_normal_top_level_function_pending(pending, admission)
    }

    pub(in crate::mir::builder) fn commit_normal_instance_constructor_pending_v1(
        &mut self,
        pending: LegacyFunctionPendingSessionV1<'_>,
        admission: super::normal_instance_constructor_admission::NormalInstanceConstructorDraftAdmissionV1,
    ) -> Result<(), ModuleLoweringPortChildErrorV1> {
        self.module_port
            .commit_normal_instance_constructor_pending(pending, admission)
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

    pub(in crate::mir::builder) fn complete_raw_root_static_child_branded(
        &mut self,
        builder: &mut MirBuilder,
        prepared: super::PreparedRawRootStaticChildDraftV1,
    ) -> Result<
        super::module_invocation_owner_chain::InvocationBranded<
            super::module_draft_collector::CollectedDraftAdmissionReceiptV1,
        >,
        ModuleLoweringPortChildErrorV1,
    > {
        let (admission, lowering) = prepared.into_parts();
        let source_root = RawInvocationRootLineageV1::Main(admission.source_locator().clone());
        let pending = self.with_source_transport_v1(
            RawInvocationSourceTransportV1::root((), source_root),
            |port, ()| {
                port.capture_static_box_method_pending_v1(
                    builder,
                    lowering.function_name,
                    lowering.params,
                    lowering.param_decls,
                    lowering.return_type_name,
                    lowering.body,
                    lowering.uses,
                    lowering.attrs,
                )
            },
        )?;
        self.module_port
            .commit_legacy_symbol_pending_branded(pending, admission.into_collector_parts())
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
                        child_port.with_headers(|headers| {
                            builder.finalize_function_draft_with_headers(prepared, headers)
                        })
                    },
                )
                .map_err(ModuleLoweringPortChildErrorV1::Session)?
        };
        Ok(pending)
    }
}

impl RecursiveChildLoweringPortV1 for RawLegacyChildLoweringPortV1 {
    type BodyInput = Vec<ASTNode>;
    type StatementInput = ASTNode;
    type ExpressionInput = ASTNode;

    fn cleanup_exit_policy_v1(&self) -> CleanupExitPolicyV1 {
        CleanupExitPolicyV1::capture_from_environment()
    }

    fn lower_body(
        &mut self,
        builder: &mut MirBuilder,
        input: Self::BodyInput,
    ) -> Result<ValueId, String> {
        super::stmts::block_stmt::build_block_with_port_v1(builder, self, input)
    }

    fn lower_statement(
        &mut self,
        builder: &mut MirBuilder,
        input: Self::StatementInput,
    ) -> Result<ValueId, String> {
        super::stmts::block_stmt::build_statement_with_port_v1(builder, self, input)
    }

    fn lower_expression(
        &mut self,
        builder: &mut MirBuilder,
        input: Self::ExpressionInput,
    ) -> Result<ValueId, String> {
        lower_raw_expression_with_recursion_guard_v1(builder, self, input)
    }
}

impl RawBoxMethodChildPortV1 for RawLegacyChildLoweringPortV1 {
    fn lower_static_main_box(
        &mut self,
        builder: &mut MirBuilder,
        box_name: String,
        methods: BoxMethodInventoryV1,
    ) -> Result<ValueId, String> {
        PreparedRawStaticMainBoxCompatibilityV1::prepare(box_name, methods)
            .lower_with_port_v1(builder, self)
            .map_err(|error| error.to_string())
    }

    fn lower_static_box_method(
        &mut self,
        builder: &mut MirBuilder,
        function_name: String,
        params: Vec<String>,
        param_decls: Vec<ParamDecl>,
        return_type_name: Option<String>,
        body: Vec<ASTNode>,
        uses: Vec<String>,
        attrs: DeclarationAttrs,
    ) -> Result<(), String> {
        builder.lower_static_method_as_function(
            function_name,
            params,
            param_decls,
            return_type_name,
            body,
            uses,
            attrs,
        )
    }

    fn lower_instance_box_method(
        &mut self,
        builder: &mut MirBuilder,
        function_name: String,
        box_name: String,
        params: Vec<String>,
        param_decls: Vec<ParamDecl>,
        return_type_name: Option<String>,
        body: Vec<ASTNode>,
        uses: Vec<String>,
        attrs: DeclarationAttrs,
    ) -> Result<(), String> {
        builder.lower_method_as_function(
            function_name,
            box_name,
            params,
            param_decls,
            return_type_name,
            body,
            uses,
            attrs,
        )
    }
}

impl RawFunctionHeaderLookupPortV1 for RawLegacyChildLoweringPortV1 {
    fn with_function_headers<R>(
        &mut self,
        observe: impl for<'headers> FnOnce(Option<&'headers dyn FunctionSignatureLookupV1>) -> R,
    ) -> R {
        observe(None)
    }
}

impl RawLoopChildEntryPortV1 for RawLegacyChildLoweringPortV1 {
    fn lower_loop(
        &mut self,
        builder: &mut MirBuilder,
        loop_node: ASTNode,
    ) -> Result<ValueId, String> {
        let ASTNode::Loop {
            condition, body, ..
        } = loop_node
        else {
            return Err("[freeze:contract][raw-loop-child-entry/expected-loop]".to_owned());
        };
        super::control_flow::joinir::routing::lower_loop_or_freeze_v1(builder, *condition, body)
    }
}

impl RawBoxMethodChildPortV1 for RawInvocationChildPortV1<'_, '_> {
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

impl RawLoopChildEntryPortV1 for RawInvocationChildPortV1<'_, '_> {
    fn lower_loop(
        &mut self,
        builder: &mut MirBuilder,
        loop_node: ASTNode,
    ) -> Result<ValueId, String> {
        let source = self.active_source.as_ref().ok_or_else(|| {
            "[freeze:contract][raw-loop-child-entry/missing-located-source]".to_owned()
        })?;
        let callable_handoff = self.issue_callable_loop_binding_schedule_v1()?;
        let admission_observation = self.generic_loop_diagnostic.issue_for_loop(source);
        PreparedLocatedRawLoopChildEntryV1::prepare_with_method_source_observation(
            source,
            loop_node,
            callable_handoff,
            self.generic_loop_diagnostic.method_source().cloned(),
            admission_observation,
        )?
        .lower_with_existing_route_v1(builder)
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

pub(in crate::mir::builder) fn drive_raw_legacy_body_v1(
    builder: &mut MirBuilder,
    input: Vec<ASTNode>,
) -> Result<ValueId, String> {
    let mut port = RawLegacyChildLoweringPortV1;
    drive_legacy_body_v1(builder, &mut port, input)
}

pub(in crate::mir::builder) fn drive_raw_legacy_statement_v1(
    builder: &mut MirBuilder,
    input: ASTNode,
) -> Result<ValueId, String> {
    let mut port = RawLegacyChildLoweringPortV1;
    drive_legacy_statement_v1(builder, &mut port, input)
}

pub(in crate::mir::builder) fn drive_raw_legacy_expression_v1(
    builder: &mut MirBuilder,
    input: ASTNode,
) -> Result<ValueId, String> {
    let mut port = RawLegacyChildLoweringPortV1;
    drive_legacy_expression_v1(builder, &mut port, input)
}
