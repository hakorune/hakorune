use crate::ast::ASTNode;
use std::collections::BTreeSet;

pub(super) fn collect_assigned_var_names(node: &ASTNode, out: &mut BTreeSet<String>) {
    crate::mir::builder::control_flow::facts::canon::generic_loop::step::extract::shared::collect_assigned_var_names(node, out)
}

pub(super) fn contains_var_name(expr: &ASTNode, target_var: &str) -> bool {
    crate::mir::builder::control_flow::facts::canon::generic_loop::step::extract::shared::contains_var_name(
        expr,
        target_var,
    )
}
