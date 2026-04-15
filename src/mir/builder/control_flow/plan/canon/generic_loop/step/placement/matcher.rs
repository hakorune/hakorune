use crate::ast::ASTNode;

pub(super) fn collect_direct_step_indices(
    body: &[ASTNode],
    loop_var: &str,
    loop_increment: &ASTNode,
) -> Vec<usize> {
    crate::mir::builder::control_flow::facts::canon::generic_loop::step::placement::collect_direct_step_indices(
        body,
        loop_var,
        loop_increment,
    )
}

pub(super) fn collect_conditional_step_indices(
    body: &[ASTNode],
    loop_var: &str,
    loop_increment: &ASTNode,
) -> (Vec<usize>, Vec<usize>) {
    crate::mir::builder::control_flow::facts::canon::generic_loop::step::placement::collect_conditional_step_indices(
        body,
        loop_var,
        loop_increment,
    )
}

pub(crate) fn matches_loop_increment(
    stmt: &ASTNode,
    loop_var: &str,
    loop_increment: &ASTNode,
) -> bool {
    crate::mir::builder::control_flow::facts::canon::generic_loop::step::placement::matches_loop_increment(
        stmt,
        loop_var,
        loop_increment,
    )
}

pub(crate) fn is_continue_if_with_increment(
    stmt: &ASTNode,
    loop_var: &str,
    loop_increment: &ASTNode,
) -> bool {
    crate::mir::builder::control_flow::facts::canon::generic_loop::step::placement::is_continue_if_with_increment(
        stmt,
        loop_var,
        loop_increment,
    )
}

pub(crate) fn is_break_else_if_with_increment(
    stmt: &ASTNode,
    loop_var: &str,
    loop_increment: &ASTNode,
) -> bool {
    crate::mir::builder::control_flow::facts::canon::generic_loop::step::placement::is_break_else_if_with_increment(
        stmt,
        loop_var,
        loop_increment,
    )
}
