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
    explicit_parameter_count: usize,
}

impl PreparedInstanceMethodDraftBodyV1 {
    pub(in crate::mir::builder) fn body(&self) -> &[ASTNode] {
        &self.body
    }

    pub(in crate::mir::builder) const fn explicit_parameter_count(&self) -> usize {
        self.explicit_parameter_count
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

    let explicit_parameter_count = params.len();
    builder.create_method_skeleton(function_name, &box_name, &params, &body)?;
    builder.set_current_function_declared_signature(
        mir_method_param_decls_from_source(&box_name, &params, &param_decls),
        return_type_name,
    );
    builder.set_current_function_runes(&attrs);
    builder.set_current_function_declared_capability_uses(&uses);
    builder.setup_method_params(&box_name, &params)?;

    Ok(PreparedInstanceMethodDraftBodyV1 {
        body,
        explicit_parameter_count,
    })
}

pub(in crate::mir::builder) fn run_function_body_step_tree_guard_v1(
    _builder: &mut MirBuilder,
    body: &[ASTNode],
    function_name: &str,
) -> Result<(), String> {
    let strict = crate::config::env::joinir_dev::strict_enabled();
    let dev = crate::config::env::joinir_dev_enabled();
    if !strict && !dev {
        return Ok(());
    }
    let tree = crate::mir::control_tree::StepTreeBuilderBox::build_from_block(body);
    if crate::config::env::joinir_dev::strict_planner_required_enabled() {
        let message = format!("[phase132/gate] StepTree root for '{}'\n", function_name);
        let _ = crate::runtime::get_global_ring0()
            .io
            .stderr_write(message.as_bytes());
    }
    if crate::config::env::joinir_dev::debug_enabled() {
        crate::runtime::get_global_ring0().log.debug(&format!(
            "[phase132/debug] StepTree root for '{}': {:?}",
            function_name, tree.root
        ));
    }
    if dev {
        crate::mir::builder::control_flow::joinir::trace::trace()
            .dev("control_tree/step_tree", &tree.to_compact_string());
    }
    crate::mir::builder::check_step_tree_capabilities(&tree, function_name, strict, dev)
}
