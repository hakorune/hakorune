//! HEADERPORT0-REENTRANT-TERM0-P0: port-aware draft/body siblings.
//!
//! This module reuses the existing skeleton, parameter, and finalizer owners.
//! Only the recursive body driver and the short-lived signature view differ
//! from the legacy facade.  The methods remain disconnected until I0.

use crate::ast::{ASTNode, DeclarationAttrs, ParamDecl};
use crate::mir::{MirBuilder, MirFunction, MirType};

use super::calls::function_lowering;
use super::calls::lowering::{mir_method_param_decls_from_source, mir_param_decls_from_source};
use super::module_lowering_invocation::LoweringHeaderPortV1;
use super::raw_expression_dispatch::RawExpressionDispatchPortV1;

/// Body completion token which deliberately contains no header loan.
#[derive(Debug)]
pub(in crate::mir::builder) struct PortAwarePreparedDraftBodyV1 {
    returns_value: bool,
}

fn run_step_tree_guard(
    builder: &mut MirBuilder,
    body: &[ASTNode],
    function_name: &str,
) -> Result<(), String> {
    let trace = crate::mir::builder::control_flow::joinir::trace::trace();
    let strict = crate::config::env::joinir_dev::strict_enabled();
    let dev = crate::config::env::joinir_dev_enabled();

    struct TraceAdapter<'a> {
        trace: &'a crate::mir::builder::control_flow::joinir::trace::JoinLoopTrace,
    }
    impl crate::mir::control_tree::normalized_shadow::dev_pipeline::DevTrace for TraceAdapter<'_> {
        fn dev(&self, tag: &str, msg: &str) {
            self.trace.dev(tag, msg)
        }
    }

    let adapter = TraceAdapter { trace: &trace };
    crate::mir::control_tree::normalized_shadow::dev_pipeline::StepTreeDevPipelineBox::run(
        builder,
        body,
        function_name,
        strict,
        dev,
        &adapter,
    )
}

impl MirBuilder {
    /// Port-aware static draft sibling.  No collector or module is touched.
    #[allow(dead_code)]
    pub(in crate::mir::builder) fn build_static_method_draft_with_port_v1<Port>(
        &mut self,
        port: &mut Port,
        function_name: String,
        params: Vec<String>,
        param_decls: Vec<ParamDecl>,
        return_type_name: Option<String>,
        body: Vec<ASTNode>,
        uses: Vec<String>,
        attrs: DeclarationAttrs,
    ) -> Result<PortAwarePreparedDraftBodyV1, String>
    where
        Port: RawExpressionDispatchPortV1,
    {
        self.create_function_skeleton(function_name, &params, &body)?;
        self.set_current_function_declared_signature(
            mir_param_decls_from_source(&params, &param_decls),
            return_type_name,
        );
        self.set_current_function_runes(&attrs);
        self.set_current_function_declared_capability_uses(&uses);
        self.setup_function_params(&params)?;
        run_step_tree_guard(self, &body, &self.current_function_name_for_port()?)?;
        let program_ast = function_lowering::wrap_in_program(body);
        let _ = self.build_expression_impl_with_port_v1(port, program_ast)?;
        let returns_value = self
            .function_state
            .current_function
            .as_ref()
            .is_some_and(|function| !matches!(function.signature.return_type, MirType::Void));
        Ok(PortAwarePreparedDraftBodyV1 { returns_value })
    }

    /// Port-aware instance draft sibling.  The body keeps the same recursive
    /// port, while the finalizer borrows headers only for its call lookup.
    #[allow(dead_code)]
    pub(in crate::mir::builder) fn build_instance_method_draft_with_port_v1<Port>(
        &mut self,
        port: &mut Port,
        function_name: String,
        box_name: String,
        params: Vec<String>,
        param_decls: Vec<ParamDecl>,
        return_type_name: Option<String>,
        body: Vec<ASTNode>,
        uses: Vec<String>,
        attrs: DeclarationAttrs,
    ) -> Result<PortAwarePreparedDraftBodyV1, String>
    where
        Port: RawExpressionDispatchPortV1,
    {
        self.create_method_skeleton(function_name, &box_name, &params, &body)?;
        self.set_current_function_declared_signature(
            mir_method_param_decls_from_source(&box_name, &params, &param_decls),
            return_type_name,
        );
        self.set_current_function_runes(&attrs);
        self.set_current_function_declared_capability_uses(&uses);
        self.setup_method_params(&box_name, &params)?;
        run_step_tree_guard(self, &body, &self.current_function_name_for_port()?)?;
        let _ = super::stmts::block_stmt::build_block_with_port_v1(self, port, body)?;
        let returns_value = self
            .function_state
            .current_function
            .as_ref()
            .is_some_and(|function| !matches!(function.signature.return_type, MirType::Void));
        Ok(PortAwarePreparedDraftBodyV1 { returns_value })
    }

    /// Port-aware finalizer entrypoint; the header loan is explicit and short.
    #[allow(dead_code)]
    pub(in crate::mir::builder) fn finalize_function_draft_with_headers(
        &mut self,
        prepared: PortAwarePreparedDraftBodyV1,
        headers: &LoweringHeaderPortV1<'_>,
    ) -> Result<MirFunction, String> {
        self.finalize_function_draft_with_lookup(prepared.returns_value, Some(headers))
    }

    fn current_function_name_for_port(&self) -> Result<String, String> {
        self.function_state
            .current_function
            .as_ref()
            .map(|function| function.signature.name.clone())
            .ok_or_else(|| "[freeze:contract][headerport/no_current_function]".to_owned())
    }
}
