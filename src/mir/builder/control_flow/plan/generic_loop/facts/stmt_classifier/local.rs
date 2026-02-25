use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::facts::expr_generic_loop::is_supported_value_expr_for_generic_loop;

/// Checks if a statement is a local declaration with initializer
pub(in crate::mir::builder) fn is_local_init(stmt: &ASTNode, loop_var: &str) -> bool {
    is_local_decl(stmt, loop_var)
        && matches!(
            stmt,
            ASTNode::Local {
                initial_values,
                ..
            } if initial_values.iter().all(|v| v.is_some())
        )
}

/// Checks if a statement is a local declaration
pub(in crate::mir::builder) fn is_local_decl(stmt: &ASTNode, loop_var: &str) -> bool {
    let ASTNode::Local {
        variables,
        initial_values,
        ..
    } = stmt
    else {
        return false;
    };
    if initial_values.is_empty() {
        return variables.iter().all(|name| name != loop_var);
    }
    if variables.len() != initial_values.len() {
        return false;
    }
    for (name, init) in variables.iter().zip(initial_values.iter()) {
        if name == loop_var {
            return false;
        }
        match init.as_ref() {
            Some(init) => {
                if !is_supported_value_expr_for_generic_loop(init) {
                    return false;
                }
            }
            None => {
                // initializer-less locals are allowed in generic_loop bodies.
            }
        }
    }
    true
}
