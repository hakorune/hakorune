//! Helper functions for AST analysis, reassignment detection, and structural matching

use crate::ast::ASTNode;
use std::collections::BTreeSet;

/// Find the index of a loop statement in the function body
///
/// Returns Some(index) if found, None otherwise.
#[allow(dead_code)]
pub(super) fn find_stmt_index(fn_body: &[ASTNode], loop_ast: &ASTNode) -> Option<usize> {
    // Compare by pointer address (same AST node instance)
    fn_body
        .iter()
        .position(|stmt| std::ptr::eq(stmt as *const ASTNode, loop_ast as *const ASTNode))
}

/// Phase 200-C: Find loop index by structure matching (condition + body comparison)
///
/// Instead of pointer comparison, compare the loop structure.
/// This is useful when the loop AST is constructed dynamically.
pub(super) fn find_loop_index_by_structure(
    fn_body: &[ASTNode],
    target_condition: &ASTNode,
    target_body: &[ASTNode],
) -> Option<usize> {
    for (idx, stmt) in fn_body.iter().enumerate() {
        if let ASTNode::Loop {
            condition, body, ..
        } = stmt
        {
            // Compare condition and body by structure
            if ast_matches(condition, target_condition) && body_matches(body, target_body) {
                return Some(idx);
            }
        }
    }
    None
}

/// Simple structural AST comparison
///
/// Uses Debug string comparison as a heuristic. This is not perfect but
/// works well enough for finding loops by structure.
pub(super) fn ast_matches(a: &ASTNode, b: &ASTNode) -> bool {
    format!("{:?}", a) == format!("{:?}", b)
}

/// Compare two body slices by structure
pub(super) fn body_matches(a: &[ASTNode], b: &[ASTNode]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    a.iter().zip(b.iter()).all(|(x, y)| ast_matches(x, y))
}

/// Collect local variable declarations from statements
///
/// Returns Vec<(name, init_expr)> for each variable declared with `local`.
pub(super) fn collect_local_declarations(stmts: &[ASTNode]) -> Vec<(String, Option<Box<ASTNode>>)> {
    let mut locals = Vec::new();

    for stmt in stmts {
        if let ASTNode::Local {
            variables,
            initial_values,
            ..
        } = stmt
        {
            // Local declaration can have multiple variables (e.g., local a, b, c)
            for (i, name) in variables.iter().enumerate() {
                let init_expr = initial_values.get(i).and_then(|opt| opt.clone());
                locals.push((name.clone(), init_expr));
            }
        }
    }

    locals
}

/// Check if expression is a safe constant (string/integer literal)
///
/// Phase 200-B: Only string and integer literals are allowed.
/// Future: May expand to include other safe constant patterns.
pub(super) fn is_safe_const_init(expr: &Option<Box<ASTNode>>) -> bool {
    match expr {
        Some(boxed) => match boxed.as_ref() {
            ASTNode::Literal { value, .. } => matches!(
                value,
                crate::ast::LiteralValue::String(_) | crate::ast::LiteralValue::Integer(_)
            ),
            _ => false,
        },
        None => false,
    }
}

/// Check if variable is reassigned anywhere in function body
///
/// Walks the entire function body AST to detect any assignments to the variable.
/// Returns true if the variable is reassigned (excluding the initial local declaration).
pub(super) fn is_reassigned_in_fn(fn_body: &[ASTNode], name: &str) -> bool {
    fn check_node(node: &ASTNode, name: &str) -> bool {
        match node {
            // Assignment to this variable
            ASTNode::Assignment { target, value, .. } => {
                // Check if target is the variable we're looking for
                let is_target_match = match target.as_ref() {
                    ASTNode::Variable { name: var_name, .. } => var_name == name,
                    ASTNode::FieldAccess { .. } | ASTNode::Index { .. } => {
                        // Field access or index assignment doesn't count as reassignment
                        false
                    }
                    _ => false,
                };

                is_target_match || check_node(value, name)
            }

            // Grouped assignment expression: (x = expr)
            ASTNode::GroupedAssignmentExpr { lhs, rhs, .. } => lhs == name || check_node(rhs, name),

            // Recursive cases
            ASTNode::If {
                condition,
                then_body,
                else_body,
                ..
            } => {
                check_node(condition, name)
                    || then_body.iter().any(|n| check_node(n, name))
                    || else_body
                        .as_ref()
                        .map_or(false, |body| body.iter().any(|n| check_node(n, name)))
            }

            ASTNode::Loop {
                condition, body, ..
            } => check_node(condition, name) || body.iter().any(|n| check_node(n, name)),

            ASTNode::While {
                condition, body, ..
            } => check_node(condition, name) || body.iter().any(|n| check_node(n, name)),

            ASTNode::TryCatch {
                try_body,
                catch_clauses,
                finally_body,
                ..
            } => {
                try_body.iter().any(|n| check_node(n, name))
                    || catch_clauses
                        .iter()
                        .any(|clause| clause.body.iter().any(|n| check_node(n, name)))
                    || finally_body
                        .as_ref()
                        .map_or(false, |body| body.iter().any(|n| check_node(n, name)))
            }

            ASTNode::UnaryOp { operand, .. } => check_node(operand, name),

            ASTNode::BinaryOp { left, right, .. } => {
                check_node(left, name) || check_node(right, name)
            }

            ASTNode::MethodCall {
                object, arguments, ..
            } => check_node(object, name) || arguments.iter().any(|arg| check_node(arg, name)),

            ASTNode::FunctionCall { arguments, .. } => {
                arguments.iter().any(|arg| check_node(arg, name))
            }

            ASTNode::FieldAccess { object, .. } => check_node(object, name),

            ASTNode::Index { target, index, .. } => {
                check_node(target, name) || check_node(index, name)
            }

            ASTNode::Return { value, .. } => value.as_ref().map_or(false, |v| check_node(v, name)),

            ASTNode::Local { .. } => {
                // Local declarations are not reassignments
                false
            }

            _ => false,
        }
    }

    fn_body.iter().any(|stmt| check_node(stmt, name))
}

/// Check if variable is referenced in loop condition or body
///
/// Returns true if the variable name appears anywhere in the loop AST.
#[allow(dead_code)]
pub(super) fn is_used_in_loop(loop_ast: &ASTNode, name: &str) -> bool {
    fn check_usage(node: &ASTNode, name: &str) -> bool {
        match node {
            ASTNode::Variable { name: var_name, .. } => var_name == name,

            ASTNode::Loop {
                condition, body, ..
            } => check_usage(condition, name) || body.iter().any(|n| check_usage(n, name)),

            ASTNode::If {
                condition,
                then_body,
                else_body,
                ..
            } => {
                check_usage(condition, name)
                    || then_body.iter().any(|n| check_usage(n, name))
                    || else_body
                        .as_ref()
                        .map_or(false, |body| body.iter().any(|n| check_usage(n, name)))
            }

            ASTNode::Assignment { target, value, .. } => {
                check_usage(target, name) || check_usage(value, name)
            }

            ASTNode::UnaryOp { operand, .. } => check_usage(operand, name),

            ASTNode::BinaryOp { left, right, .. } => {
                check_usage(left, name) || check_usage(right, name)
            }

            ASTNode::MethodCall {
                object, arguments, ..
            } => check_usage(object, name) || arguments.iter().any(|arg| check_usage(arg, name)),

            ASTNode::FunctionCall { arguments, .. } => {
                arguments.iter().any(|arg| check_usage(arg, name))
            }

            ASTNode::FieldAccess { object, .. } => check_usage(object, name),

            ASTNode::Index { target, index, .. } => {
                check_usage(target, name) || check_usage(index, name)
            }

            ASTNode::Return { value, .. } => value.as_ref().map_or(false, |v| check_usage(v, name)),

            ASTNode::Local { initial_values, .. } => initial_values
                .iter()
                .any(|opt| opt.as_ref().map_or(false, |init| check_usage(init, name))),

            _ => false,
        }
    }

    check_usage(loop_ast, name)
}

/// Phase 200-C: Check if variable is used in loop condition or body (separate parts)
///
/// This is used by analyze_captured_vars_v2 when condition and body are passed separately.
pub(super) fn is_used_in_loop_parts(condition: &ASTNode, body: &[ASTNode], name: &str) -> bool {
    fn check_usage(node: &ASTNode, name: &str) -> bool {
        match node {
            ASTNode::Variable { name: var_name, .. } => var_name == name,

            ASTNode::Loop {
                condition, body, ..
            } => check_usage(condition, name) || body.iter().any(|n| check_usage(n, name)),

            ASTNode::If {
                condition,
                then_body,
                else_body,
                ..
            } => {
                check_usage(condition, name)
                    || then_body.iter().any(|n| check_usage(n, name))
                    || else_body
                        .as_ref()
                        .map_or(false, |body| body.iter().any(|n| check_usage(n, name)))
            }

            ASTNode::Assignment { target, value, .. } => {
                check_usage(target, name) || check_usage(value, name)
            }

            ASTNode::UnaryOp { operand, .. } => check_usage(operand, name),

            ASTNode::BinaryOp { left, right, .. } => {
                check_usage(left, name) || check_usage(right, name)
            }

            ASTNode::MethodCall {
                object, arguments, ..
            } => check_usage(object, name) || arguments.iter().any(|arg| check_usage(arg, name)),

            ASTNode::FunctionCall { arguments, .. } => {
                arguments.iter().any(|arg| check_usage(arg, name))
            }

            ASTNode::FieldAccess { object, .. } => check_usage(object, name),

            ASTNode::Index { target, index, .. } => {
                check_usage(target, name) || check_usage(index, name)
            }

            ASTNode::Return { value, .. } => value.as_ref().map_or(false, |v| check_usage(v, name)),

            ASTNode::Local { initial_values, .. } => initial_values
                .iter()
                .any(|opt| opt.as_ref().map_or(false, |init| check_usage(init, name))),

            _ => false,
        }
    }

    check_usage(condition, name) || body.iter().any(|n| check_usage(n, name))
}

/// Phase 245C: Collect all variable names used in loop condition and body
///
/// Helper for function parameter capture. Returns a set of all variable names
/// that appear in the loop's condition or body.
pub(super) fn collect_names_in_loop_parts(
    condition: &ASTNode,
    body: &[ASTNode],
) -> BTreeSet<String> {
    fn collect(node: &ASTNode, acc: &mut BTreeSet<String>) {
        match node {
            ASTNode::Variable { name, .. } => {
                acc.insert(name.clone());
            }
            ASTNode::If {
                condition,
                then_body,
                else_body,
                ..
            } => {
                collect(condition, acc);
                for stmt in then_body {
                    collect(stmt, acc);
                }
                if let Some(else_stmts) = else_body {
                    for stmt in else_stmts {
                        collect(stmt, acc);
                    }
                }
            }
            ASTNode::Assignment { target, value, .. } => {
                collect(target, acc);
                collect(value, acc);
            }
            ASTNode::UnaryOp { operand, .. } => {
                collect(operand, acc);
            }
            ASTNode::Return {
                value: Some(operand),
                ..
            } => {
                collect(operand, acc);
            }
            ASTNode::BinaryOp { left, right, .. } => {
                collect(left, acc);
                collect(right, acc);
            }
            ASTNode::MethodCall {
                object, arguments, ..
            } => {
                collect(object, acc);
                for arg in arguments {
                    collect(arg, acc);
                }
            }
            ASTNode::FunctionCall { arguments, .. } => {
                for arg in arguments {
                    collect(arg, acc);
                }
            }
            ASTNode::Local { initial_values, .. } => {
                for init_opt in initial_values {
                    if let Some(val) = init_opt {
                        collect(val, acc);
                    }
                }
            }
            ASTNode::FieldAccess { object, .. } => {
                collect(object, acc);
            }
            ASTNode::Index { target, index, .. } => {
                collect(target, acc);
                collect(index, acc);
            }
            ASTNode::Loop {
                condition, body, ..
            } => {
                collect(condition, acc);
                for stmt in body {
                    collect(stmt, acc);
                }
            }
            _ => {}
        }
    }

    let mut acc = BTreeSet::new();
    collect(condition, &mut acc);
    for stmt in body {
        collect(stmt, &mut acc);
    }
    acc
}
