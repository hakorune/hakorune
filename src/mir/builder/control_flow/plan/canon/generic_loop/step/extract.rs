use crate::ast::ASTNode;

mod complex_step;
mod next_step;
mod shared;
mod var_step;

pub(crate) fn canon_loop_increment_for_var(
    body: &[ASTNode],
    loop_var: &str,
    allow_var_step: bool,
) -> Option<ASTNode> {
    crate::mir::builder::control_flow::facts::canon::generic_loop::step::extract::canon_loop_increment_for_var(
        body,
        loop_var,
        allow_var_step,
    )
}
