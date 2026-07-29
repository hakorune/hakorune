//! Behavior-neutral recursive child-lowering port.
//!
//! This module owns the typed body, statement, and expression entry boundary.
//! It owns no source navigation, callable-result plan, location, ledger,
//! MethodCall route, or result-publication policy.

use crate::ast::{ASTNode, DeclarationAttrs, ParamDecl};
use crate::mir::{MirBuilder, ValueId};

use super::calls::LegacyFunctionPendingSessionV1;
use super::function_signature_lookup::FunctionSignatureLookupV1;
use super::me_call_header_observation::{
    MeCallHeaderObservationPortV1, MeCallHeaderSourceV1, MeCallParameterObservationV1,
};
use super::module_lowering_invocation::{
    LegacyChildDraftAdmissionV1, LoweringHeaderPortV1, ModuleLoweringPortChildErrorV1,
    ModuleLoweringPortV1,
};
use super::port_aware_function_draft_impl::PortAwarePreparedDraftBodyV1;
use super::raw_expression_dispatch::RawExpressionDispatchPortV1;
use super::raw_loop_child_entry::{
    classify_raw_loop_child_entry_v1, RawLoopChildEntryDispositionV1,
};
use super::raw_static_main_compat_batch::PreparedRawStaticMainBoxCompatibilityV1;

const MAX_RAW_EXPRESSION_RECURSION_DEPTH: usize = 200;

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
}

/// Raw AST specialization shared by the legacy facade and the
/// invocation-aware carrier.
///
/// Located/source-branded ports intentionally do not implement this marker.
/// It permits raw syntax adapters to have one blanket implementation without
/// fabricating a second AST representation or copying any source policy.
pub(in crate::mir::builder) trait RawAstChildLoweringPortV1:
    RecursiveChildLoweringPortV1<
    BodyInput = Vec<ASTNode>,
    StatementInput = ASTNode,
    ExpressionInput = ASTNode,
>
{
}

/// Optional completed-header capability for raw terminals.  The legacy raw
/// port returns no view; the invocation port supplies a short collector loan.
pub(in crate::mir::builder) trait RawFunctionHeaderLookupPortV1 {
    fn with_function_headers<R>(
        &mut self,
        observe: impl for<'headers> FnOnce(Option<&'headers dyn FunctionSignatureLookupV1>) -> R,
    ) -> R;
}

/// One raw Box method-child terminal capability.
///
/// The raw dispatcher keeps one AST match tree and delegates only the child
/// function terminal here.  Legacy callers retain their existing publication
/// route; invocation callers use the collector-backed legacy terminal.
pub(in crate::mir::builder) trait RawBoxMethodChildPortV1 {
    /// Lower the special `Main` static box entry.
    ///
    /// `Main` is a root-only surface for invocation sessions.  Keeping this
    /// decision on the same port makes the invocation implementation reject
    /// it before any root-main mutation can occur, while the legacy adapter
    /// retains the existing inline-main behavior.
    fn lower_static_main_box(
        &mut self,
        builder: &mut MirBuilder,
        box_name: String,
        methods: std::collections::HashMap<String, ASTNode>,
    ) -> Result<ValueId, String>;

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
    ) -> Result<(), String>;

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
    ) -> Result<(), String>;
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
        condition: ASTNode,
        body: Vec<ASTNode>,
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
    module_port: &'port mut ModuleLoweringPortV1<'collector>,
    _seal: RawInvocationChildPortSealV1,
}

struct RawInvocationChildPortSealV1;

impl<'port, 'collector> RawInvocationChildPortV1<'port, 'collector> {
    /// Start one raw recursive frame from the exact invocation port.
    pub(in crate::mir::builder) fn new(
        module_port: &'port mut ModuleLoweringPortV1<'collector>,
    ) -> Self {
        Self {
            module_port,
            _seal: RawInvocationChildPortSealV1,
        }
    }

    /// Reborrow the same invocation capability for one nested raw frame.
    ///
    /// No header borrow crosses this boundary: `with_headers` consumes the
    /// observation closure before the next descendant can mutate state.
    pub(in crate::mir::builder) fn reborrow(&mut self) -> RawInvocationChildPortV1<'_, 'collector> {
        RawInvocationChildPortV1::new(&mut *self.module_port)
    }

    /// Lend the exact collector-backed header view for one observation only.
    pub(in crate::mir::builder) fn with_headers<R>(
        &self,
        observe: impl for<'header> FnOnce(&'header LoweringHeaderPortV1<'header>) -> R,
    ) -> R {
        self.module_port.with_headers(observe)
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

    pub(in crate::mir::builder) fn complete_static_box_method_branded(
        &mut self,
        builder: &mut MirBuilder,
        admission: LegacyChildDraftAdmissionV1,
        function_name: String,
        params: Vec<String>,
        param_decls: Vec<ParamDecl>,
        return_type_name: Option<String>,
        body: Vec<ASTNode>,
        uses: Vec<String>,
        attrs: DeclarationAttrs,
    ) -> Result<
        super::module_invocation_owner_chain::InvocationBranded<
            super::module_draft_collector::CollectedDraftAdmissionReceiptV1,
        >,
        ModuleLoweringPortChildErrorV1,
    > {
        let pending = self.capture_static_box_method_pending_v1(
            builder,
            function_name,
            params,
            param_decls,
            return_type_name,
            body,
            uses,
            attrs,
        )?;
        self.module_port
            .commit_legacy_pending_branded(pending, admission)
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

impl RecursiveChildLoweringPortV1 for RawInvocationChildPortV1<'_, '_> {
    type BodyInput = Vec<ASTNode>;
    type StatementInput = ASTNode;
    type ExpressionInput = ASTNode;

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

impl RecursiveChildLoweringPortV1 for RawLegacyChildLoweringPortV1 {
    type BodyInput = Vec<ASTNode>;
    type StatementInput = ASTNode;
    type ExpressionInput = ASTNode;

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
        methods: std::collections::HashMap<String, ASTNode>,
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
        condition: ASTNode,
        body: Vec<ASTNode>,
    ) -> Result<ValueId, String> {
        super::control_flow::joinir::routing::lower_loop_or_freeze_v1(builder, condition, body)
    }
}

impl RawBoxMethodChildPortV1 for RawInvocationChildPortV1<'_, '_> {
    fn lower_static_main_box(
        &mut self,
        _builder: &mut MirBuilder,
        box_name: String,
        _methods: std::collections::HashMap<String, ASTNode>,
    ) -> Result<ValueId, String> {
        Err(
            super::control_flow::lower::Freeze::contract(&format!(
                "raw_invocation_main_box: root-only Main box cannot be lowered as a nested child name={box_name}"
            ))
            .to_string(),
        )
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
        builder.observe_legacy_method_lowering_v1(&function_name, &body, None);
        let expected_arity = params.len();
        let admission =
            LegacyChildDraftAdmissionV1::legacy_symbol(function_name.clone(), expected_arity);
        let pending = self
            .capture_static_box_method_pending_v1(
                builder,
                function_name,
                params,
                param_decls,
                return_type_name,
                body,
                uses,
                attrs,
            )
            .map_err(|error| error.to_string())?;
        self.module_port
            .commit_legacy_pending(pending, admission)
            .map_err(|error| error.to_string())
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
        let params =
            super::calls::lowering::normalize_instance_method_params(&function_name, params);
        let param_decls = super::calls::lowering::normalize_instance_method_param_decls(
            &function_name,
            param_decls,
        );
        builder.observe_legacy_method_lowering_v1(&function_name, &body, Some(&box_name));
        let expected_arity = params.len() + 1;
        let admission =
            LegacyChildDraftAdmissionV1::legacy_symbol(function_name.clone(), expected_arity);
        let pending = self
            .capture_normalized_instance_box_method_pending_v1(
                builder,
                function_name,
                box_name,
                params,
                param_decls,
                return_type_name,
                body,
                uses,
                attrs,
            )
            .map_err(|error| error.to_string())?;
        self.module_port
            .commit_legacy_pending(pending, admission)
            .map_err(|error| error.to_string())
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
        condition: ASTNode,
        body: Vec<ASTNode>,
    ) -> Result<ValueId, String> {
        match classify_raw_loop_child_entry_v1(&condition, &body) {
            RawLoopChildEntryDispositionV1::NoChildFunctionEntry => {
                super::control_flow::joinir::routing::lower_loop_or_freeze_v1(
                    builder, condition, body,
                )
            }
            RawLoopChildEntryDispositionV1::ReachableBoxDeclaration => Err(
                super::control_flow::lower::Freeze::contract(
                    "raw_loop_child_entry: reachable BoxDeclaration requires a pure-plan/function-session bridge",
                )
                .to_string(),
            ),
        }
    }
}

fn lower_raw_expression_with_recursion_guard_v1<Port>(
    builder: &mut MirBuilder,
    port: &mut Port,
    input: ASTNode,
) -> Result<ValueId, String>
where
    Port: RawExpressionDispatchPortV1,
{
    let node_kind = std::mem::discriminant(&input);
    with_legacy_expression_recursion_guard_v1(builder, node_kind, move |builder| {
        builder.build_expression_impl_with_port_v1(port, input)
    })
}

pub(in crate::mir::builder) fn with_legacy_expression_recursion_guard_v1<F>(
    builder: &mut MirBuilder,
    node_kind: std::mem::Discriminant<ASTNode>,
    lower: F,
) -> Result<ValueId, String>
where
    F: FnOnce(&mut MirBuilder) -> Result<ValueId, String>,
{
    builder.recursion_depth += 1;
    let current_depth = builder.recursion_depth;
    if current_depth > MAX_RAW_EXPRESSION_RECURSION_DEPTH {
        let ring0 = crate::runtime::get_global_ring0();
        ring0
            .log
            .error("\n[FATAL] ============================================");
        ring0.log.error(&format!(
            "[FATAL] Recursion depth exceeded {} in build_expression",
            MAX_RAW_EXPRESSION_RECURSION_DEPTH
        ));
        ring0
            .log
            .error(&format!("[FATAL] Current depth: {current_depth}"));
        ring0
            .log
            .error(&format!("[FATAL] AST node type: {:?}", node_kind));
        ring0
            .log
            .error("[FATAL] ============================================\n");
        builder.recursion_depth -= 1;
        return Err(format!(
            "Recursion depth exceeded: {current_depth} (possible infinite loop)"
        ));
    }

    let result = lower(builder);
    builder.recursion_depth -= 1;
    result
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
