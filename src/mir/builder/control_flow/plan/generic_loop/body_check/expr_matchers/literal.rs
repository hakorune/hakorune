use crate::ast::{ASTNode, LiteralValue};
use crate::mir::builder::control_flow::plan::generic_loop::facts::stmt_classifier::is_local_init;

/// Matches local initialization with literal value.
pub(in crate::mir::builder) fn matches_local_init_literal(
    stmt: &ASTNode,
    loop_var: &str,
    literal: i64,
) -> Option<String> {
    if !is_local_init(stmt, loop_var) {
        return None;
    }
    let ASTNode::Local {
        variables,
        initial_values,
        ..
    } = stmt
    else {
        return None;
    };
    if variables.len() != 1 || initial_values.len() != 1 {
        return None;
    }
    let name = variables[0].clone();
    if name == loop_var {
        return None;
    }
    let Some(init) = initial_values[0].as_ref() else {
        return None;
    };
    if !matches!(
        init.as_ref(),
        ASTNode::Literal {
            value: LiteralValue::Integer(v),
            ..
        } if *v == literal
    ) {
        return None;
    }
    Some(name)
}
