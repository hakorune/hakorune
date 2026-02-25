use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::facts::expr_generic_loop::is_supported_value_expr_for_generic_loop;

/// Checks if a statement uses the loop variable
pub(in crate::mir::builder) fn stmt_uses_loop_var(stmt: &ASTNode, loop_var: &str) -> bool {
    match stmt {
        ASTNode::Variable { name, .. } => name == loop_var,
        ASTNode::Assignment { target, value, .. } => {
            stmt_uses_loop_var(target, loop_var) || stmt_uses_loop_var(value, loop_var)
        }
        ASTNode::BinaryOp { left, right, .. } => {
            stmt_uses_loop_var(left, loop_var) || stmt_uses_loop_var(right, loop_var)
        }
        ASTNode::UnaryOp { operand, .. } => stmt_uses_loop_var(operand, loop_var),
        ASTNode::MethodCall {
            object, arguments, ..
        } => {
            stmt_uses_loop_var(object, loop_var)
                || arguments
                    .iter()
                    .any(|arg| stmt_uses_loop_var(arg, loop_var))
        }
        ASTNode::FunctionCall { arguments, .. } => arguments
            .iter()
            .any(|arg| stmt_uses_loop_var(arg, loop_var)),
        ASTNode::If {
            condition,
            then_body,
            else_body,
            ..
        } => {
            stmt_uses_loop_var(condition, loop_var)
                || then_body
                    .iter()
                    .any(|node| stmt_uses_loop_var(node, loop_var))
                || else_body.as_ref().map_or(false, |body| {
                    body.iter().any(|node| stmt_uses_loop_var(node, loop_var))
                })
        }
        ASTNode::Local { initial_values, .. } => initial_values.iter().any(|init| {
            init.as_ref()
                .map_or(false, |node| stmt_uses_loop_var(node, loop_var))
        }),
        ASTNode::Return { value, .. } => value
            .as_ref()
            .map_or(false, |node| stmt_uses_loop_var(node, loop_var)),
        ASTNode::Loop {
            condition, body, ..
        } => {
            stmt_uses_loop_var(condition, loop_var)
                || body.iter().any(|node| stmt_uses_loop_var(node, loop_var))
        }
        ASTNode::ScopeBox { body, .. } => {
            body.iter().any(|node| stmt_uses_loop_var(node, loop_var))
        }
        ASTNode::Break { .. } | ASTNode::Continue { .. } | ASTNode::Literal { .. } => false,
        _ => true,
    }
}

pub(in crate::mir::builder) fn stmt_kind_name(stmt: &ASTNode) -> &'static str {
    match stmt {
        ASTNode::Program { .. } => "Program",
        ASTNode::ScopeBox { .. } => "ScopeBox",
        ASTNode::Assignment { .. } => "Assignment",
        ASTNode::Print { .. } => "Print",
        ASTNode::If { .. } => "If",
        ASTNode::Loop { .. } => "Loop",
        ASTNode::While { .. } => "While",
        ASTNode::ForRange { .. } => "ForRange",
        ASTNode::Break { .. } => "Break",
        ASTNode::Continue { .. } => "Continue",
        ASTNode::Return { .. } => "Return",
        ASTNode::Local { .. } => "Local",
        ASTNode::MethodCall { .. } => "MethodCall",
        ASTNode::FunctionCall { .. } => "FunctionCall",
        ASTNode::Call { .. } => "Call",
        _ => "Other",
    }
}

/// Returns detailed reason for unsupported statement
pub(in crate::mir::builder) fn unsupported_stmt_detail(stmt: &ASTNode, loop_var: &str) -> &'static str {
    if let ASTNode::Local {
        variables,
        initial_values,
        ..
    } = stmt
    {
        if variables.iter().any(|name| name == loop_var) {
            return "LocalDeclaresLoopVar";
        }
        if !initial_values.is_empty() && variables.len() != initial_values.len() {
            return "LocalInitLenMismatch";
        }
        for init in initial_values {
            if let Some(init) = init.as_ref() {
                if !is_supported_value_expr_for_generic_loop(init) {
                    return "LocalUnsupportedInit";
                }
            }
        }
        return "Local";
    }
    stmt_kind_name(stmt)
}
