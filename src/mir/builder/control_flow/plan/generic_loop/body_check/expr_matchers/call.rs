use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::generic_loop::body_check_extractors::{
    is_loop_var_minus_one, is_loop_var_plus_one,
};

/// Matches `_is_space` method call with loop_var substring.
pub(in crate::mir::builder) fn matches_is_space_call(expr: &ASTNode, loop_var: &str) -> bool {
    let ASTNode::MethodCall {
        method, arguments, ..
    } = expr
    else {
        return false;
    };
    if method == "_is_space"
        && arguments
            .iter()
            .any(|arg| matches_substring_call_with_loop_var(arg, loop_var))
    {
        return true;
    }
    false
}

/// Matches `substring(loop_var +/- 1, ...)` method call pattern.
pub(in crate::mir::builder) fn matches_substring_call_with_loop_var(
    expr: &ASTNode,
    loop_var: &str,
) -> bool {
    let ASTNode::MethodCall {
        method, arguments, ..
    } = expr
    else {
        return false;
    };
    if method != "substring" {
        return false;
    }
    if arguments.len() != 2 {
        return false;
    }
    let arg0 = &arguments[0];
    let arg1 = &arguments[1];
    let is_loop_var =
        |node: &ASTNode| matches!(node, ASTNode::Variable { name, .. } if name == loop_var);
    (is_loop_var(arg0) && is_loop_var_plus_one(arg1, loop_var))
        || (is_loop_var_minus_one(arg0, loop_var) && is_loop_var(arg1))
}
