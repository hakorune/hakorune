use crate::ast::ASTNode;

pub(super) fn extract_next_i_increment(body: &[ASTNode], loop_var: &str) -> Option<ASTNode> {
    crate::mir::builder::control_flow::facts::canon::generic_loop::step::extract::next_step::extract_next_i_increment(
        body, loop_var,
    )
}
