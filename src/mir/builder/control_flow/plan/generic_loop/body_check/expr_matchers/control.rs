use crate::ast::{ASTNode, LiteralValue};
use super::compare::matches_loop_var_equal_literal;
use super::literal::matches_local_init_literal;

/// Matches `if (loop_var == literal) return literal` pattern.
pub(in crate::mir::builder) fn matches_if_return_literal(stmt: &ASTNode, loop_var: &str, literal: i64) -> bool {
    let ASTNode::If {
        condition,
        then_body,
        else_body,
        ..
    } = stmt
    else {
        return false;
    };
    if else_body.is_some() || then_body.len() != 1 {
        return false;
    }
    if !matches_loop_var_equal_literal(condition, loop_var, literal) {
        return false;
    }
    let ASTNode::Return { value, .. } = &then_body[0] else {
        return false;
    };
    let Some(value) = value.as_ref() else {
        return false;
    };
    matches!(
        value.as_ref(),
        ASTNode::Literal {
            value: LiteralValue::Integer(v),
            ..
        } if *v == literal
    )
}

/// Matches `if (loop_var == 0) return var` pattern.
pub(in crate::mir::builder) fn matches_if_return_var(stmt: &ASTNode, loop_var: &str, return_var: &str) -> bool {
    let ASTNode::If {
        condition,
        then_body,
        else_body,
        ..
    } = stmt
    else {
        return false;
    };
    if else_body.is_some() || then_body.len() != 1 {
        return false;
    }
    if !matches_loop_var_equal_literal(condition, loop_var, 0) {
        return false;
    }
    matches!(
        then_body[0],
        ASTNode::Return {
            value: Some(ref value),
            ..
        } if matches!(value.as_ref(), ASTNode::Variable { name, .. } if name == return_var)
    )
}

/// Matches `if (loop_var == literal) { local x = literal; return x; }` pattern.
pub(in crate::mir::builder) fn matches_if_return_local(stmt: &ASTNode, loop_var: &str, literal: i64) -> bool {
    let ASTNode::If {
        condition,
        then_body,
        else_body,
        ..
    } = stmt
    else {
        return false;
    };
    if else_body.is_some() || then_body.len() != 2 {
        return false;
    }
    if !matches_loop_var_equal_literal(condition, loop_var, literal) {
        return false;
    }
    let Some(local_name) = matches_local_init_literal(&then_body[0], loop_var, literal) else {
        return false;
    };
    matches!(
        then_body[1],
        ASTNode::Return {
            value: Some(ref value),
            ..
        } if matches!(value.as_ref(), ASTNode::Variable { name, .. } if *name == local_name)
    )
}

/// Matches `if (loop_var == literal) { return literal; } else { return literal; }` pattern.
pub(in crate::mir::builder) fn matches_if_else_return_literal(stmt: &ASTNode, loop_var: &str, literal: i64) -> bool {
    let ASTNode::If {
        condition,
        then_body,
        else_body,
        ..
    } = stmt
    else {
        return false;
    };
    let Some(else_body) = else_body else {
        return false;
    };
    if then_body.len() != 1 || else_body.len() != 1 {
        return false;
    }
    if !matches_loop_var_equal_literal(condition, loop_var, literal) {
        return false;
    }
    let then_ok = matches!(
        then_body[0],
        ASTNode::Return {
            value: Some(ref value),
            ..
        } if matches!(value.as_ref(), ASTNode::Literal {
            value: LiteralValue::Integer(v),
            ..
        } if *v == literal)
    );
    if !then_ok {
        return false;
    }
    matches!(
        else_body[0],
        ASTNode::Return {
            value: Some(ref value),
            ..
        } if matches!(value.as_ref(), ASTNode::Literal {
            value: LiteralValue::Integer(v),
            ..
        } if *v == literal)
    )
}

/// Matches `if (loop_var == literal) { return literal; } else { return var; }` pattern.
pub(in crate::mir::builder) fn matches_if_else_return_literal_var(
    stmt: &ASTNode,
    loop_var: &str,
    literal: i64,
) -> Option<String> {
    let ASTNode::If {
        condition,
        then_body,
        else_body,
        ..
    } = stmt
    else {
        return None;
    };
    let Some(else_body) = else_body else {
        return None;
    };
    if then_body.len() != 1 || else_body.len() != 1 {
        return None;
    }
    if !matches_loop_var_equal_literal(condition, loop_var, literal) {
        return None;
    }
    let then_ok = matches!(
        then_body[0],
        ASTNode::Return {
            value: Some(ref value),
            ..
        } if matches!(value.as_ref(), ASTNode::Literal {
            value: LiteralValue::Integer(v),
            ..
        } if *v == literal)
    );
    if !then_ok {
        return None;
    }
    let ASTNode::Return {
        value: Some(ref else_value),
        ..
    } = else_body[0]
    else {
        return None;
    };
    let ASTNode::Variable { name, .. } = else_value.as_ref() else {
        return None;
    };
    Some(name.clone())
}

/// Matches `if (loop_var == literal) { local x = literal; return x; } else { return literal; }` pattern.
pub(in crate::mir::builder) fn matches_if_else_return_literal_local(
    stmt: &ASTNode,
    loop_var: &str,
    literal: i64,
) -> bool {
    let ASTNode::If {
        condition,
        then_body,
        else_body,
        ..
    } = stmt
    else {
        return false;
    };
    let Some(else_body) = else_body else {
        return false;
    };
    if then_body.len() != 2 || else_body.len() != 1 {
        return false;
    }
    if !matches_loop_var_equal_literal(condition, loop_var, literal) {
        return false;
    }
    let Some(local_name) = matches_local_init_literal(&then_body[0], loop_var, literal) else {
        return false;
    };
    let then_ok = matches!(
        then_body[1],
        ASTNode::Return {
            value: Some(ref value),
            ..
        } if matches!(value.as_ref(), ASTNode::Variable { name, .. } if *name == local_name)
    );
    if !then_ok {
        return false;
    }
    matches!(
        else_body[0],
        ASTNode::Return {
            value: Some(ref value),
            ..
        } if matches!(value.as_ref(), ASTNode::Literal {
            value: LiteralValue::Integer(v),
            ..
        } if *v == literal)
    )
}
