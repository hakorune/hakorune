//! Compatibility-only recursive child port.
//!
//! The parent module owns the shared recursive traits and the collector-backed
//! invocation port. This child keeps the direct legacy facade in its own
//! physical shelf so later source-aware loans cannot accidentally reuse it.

use crate::ast::{ASTNode, BoxMethodInventoryV1, DeclarationAttrs, ParamDecl};
use crate::mir::{MirBuilder, ValueId};

use super::super::control_flow::cleanup::CleanupExitPolicyV1;
use super::super::function_signature_lookup::FunctionSignatureLookupV1;
use super::super::me_call_header_observation::{
    MeCallHeaderObservationPortV1, MeCallHeaderSourceV1, MeCallParameterObservationV1,
};
use super::super::raw_static_main_compat_batch::PreparedRawStaticMainBoxCompatibilityV1;
use super::{RawBoxMethodChildPortV1, RawFunctionHeaderLookupPortV1, RecursiveChildLoweringPortV1};

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
        super::super::stmts::block_stmt::build_block_with_port_v1(builder, self, input)
    }

    fn lower_statement(
        &mut self,
        builder: &mut MirBuilder,
        input: Self::StatementInput,
    ) -> Result<ValueId, String> {
        super::super::stmts::block_stmt::build_statement_with_port_v1(builder, self, input)
    }

    fn lower_expression(
        &mut self,
        builder: &mut MirBuilder,
        input: Self::ExpressionInput,
    ) -> Result<ValueId, String> {
        super::lower_raw_expression_with_recursion_guard_v1(builder, self, input)
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

pub(in crate::mir::builder) fn drive_raw_legacy_body_v1(
    builder: &mut MirBuilder,
    input: Vec<ASTNode>,
) -> Result<ValueId, String> {
    let mut port = RawLegacyChildLoweringPortV1;
    super::drive_legacy_body_v1(builder, &mut port, input)
}

pub(in crate::mir::builder) fn drive_raw_legacy_statement_v1(
    builder: &mut MirBuilder,
    input: ASTNode,
) -> Result<ValueId, String> {
    let mut port = RawLegacyChildLoweringPortV1;
    super::drive_legacy_statement_v1(builder, &mut port, input)
}

pub(in crate::mir::builder) fn drive_raw_legacy_expression_v1(
    builder: &mut MirBuilder,
    input: ASTNode,
) -> Result<ValueId, String> {
    let mut port = RawLegacyChildLoweringPortV1;
    super::drive_legacy_expression_v1(builder, &mut port, input)
}
