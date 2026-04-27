use crate::ast::ASTNode;
use crate::mir::builder::control_flow::facts::scan_common_predicates::{
    as_var_name, is_loop_cond_var_lt_var as shared_is_loop_cond_var_lt_var,
};

pub(in crate::mir::builder) fn release_enabled() -> bool {
    true
}

pub(in crate::mir::builder) fn is_loop_cond_var_lt_var(ast: &ASTNode) -> Option<(String, String)> {
    shared_is_loop_cond_var_lt_var(ast)
}

pub(in crate::mir::builder) fn declares_local_var(stmt: &ASTNode, name: &str) -> bool {
    let ASTNode::Local { variables, .. } = stmt else {
        return false;
    };
    variables.iter().any(|v| v == name)
}

pub(in crate::mir::builder) fn extract_step_var_from_tail(
    stmt: &ASTNode,
    loop_var: &str,
) -> Option<String> {
    let ASTNode::Assignment { target, value, .. } = stmt else {
        return None;
    };
    if as_var_name(target.as_ref()) != Some(loop_var) {
        return None;
    }
    Some(as_var_name(value.as_ref())?.to_string())
}
