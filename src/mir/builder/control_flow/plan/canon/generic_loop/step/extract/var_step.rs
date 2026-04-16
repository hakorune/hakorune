use crate::ast::ASTNode;

pub(super) fn extract_var_step_increment(body: &[ASTNode], loop_var: &str) -> Option<ASTNode> {
    crate::mir::builder::control_flow::facts::canon::generic_loop::step::extract::var_step::extract_var_step_increment(
        body, loop_var,
    )
}
