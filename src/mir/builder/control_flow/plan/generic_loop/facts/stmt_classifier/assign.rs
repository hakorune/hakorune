use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::facts::expr_generic_loop::is_supported_value_expr_for_generic_loop;

/// Checks if a statement is a simple assignment (not to loop var)
pub(in crate::mir::builder) fn is_simple_assignment(stmt: &ASTNode, loop_var: &str) -> bool {
    let ASTNode::Assignment { target, value, .. } = stmt else {
        return false;
    };
    let ASTNode::Variable { name, .. } = target.as_ref() else {
        return false;
    };
    if name == loop_var {
        return false;
    }
    is_supported_value_expr_for_generic_loop(value)
}
