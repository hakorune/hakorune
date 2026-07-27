//! Shared, behavior-neutral instance-method draft preparation.
//!
//! This owner opens the existing method skeleton and installs declaration and
//! parameter facts. Body lowering and draft finalization deliberately remain
//! with their legacy or port-aware callers.

use crate::ast::{ASTNode, DeclarationAttrs, ParamDecl};
use crate::mir::builder::MirBuilder;

use super::lowering::mir_method_param_decls_from_source;

pub(in crate::mir::builder) struct InstanceMethodDraftPreparationRequestV1 {
    function_name: String,
    box_name: String,
    params: Vec<String>,
    param_decls: Vec<ParamDecl>,
    return_type_name: Option<String>,
    body: Vec<ASTNode>,
    uses: Vec<String>,
    attrs: DeclarationAttrs,
}

impl InstanceMethodDraftPreparationRequestV1 {
    #[allow(clippy::too_many_arguments)]
    pub(in crate::mir::builder) fn new(
        function_name: String,
        box_name: String,
        params: Vec<String>,
        param_decls: Vec<ParamDecl>,
        return_type_name: Option<String>,
        body: Vec<ASTNode>,
        uses: Vec<String>,
        attrs: DeclarationAttrs,
    ) -> Self {
        Self {
            function_name,
            box_name,
            params,
            param_decls,
            return_type_name,
            body,
            uses,
            attrs,
        }
    }
}

pub(in crate::mir::builder) struct PreparedInstanceMethodDraftBodyV1 {
    body: Vec<ASTNode>,
}

impl PreparedInstanceMethodDraftBodyV1 {
    pub(in crate::mir::builder) fn body(&self) -> &[ASTNode] {
        &self.body
    }

    pub(in crate::mir::builder) fn into_body(self) -> Vec<ASTNode> {
        self.body
    }
}

pub(in crate::mir::builder) fn prepare_instance_method_draft_body_v1(
    builder: &mut MirBuilder,
    request: InstanceMethodDraftPreparationRequestV1,
) -> Result<PreparedInstanceMethodDraftBodyV1, String> {
    let InstanceMethodDraftPreparationRequestV1 {
        function_name,
        box_name,
        params,
        param_decls,
        return_type_name,
        body,
        uses,
        attrs,
    } = request;

    builder.create_method_skeleton(function_name, &box_name, &params, &body)?;
    builder.set_current_function_declared_signature(
        mir_method_param_decls_from_source(&box_name, &params, &param_decls),
        return_type_name,
    );
    builder.set_current_function_runes(&attrs);
    builder.set_current_function_declared_capability_uses(&uses);
    builder.setup_method_params(&box_name, &params)?;

    Ok(PreparedInstanceMethodDraftBodyV1 { body })
}

pub(in crate::mir::builder) fn run_function_body_step_tree_guard_v1(
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
