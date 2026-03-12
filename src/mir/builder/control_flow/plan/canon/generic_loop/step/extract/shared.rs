use crate::ast::ASTNode;
use std::collections::BTreeSet;

pub(super) fn collect_assigned_var_names(node: &ASTNode, out: &mut BTreeSet<String>) {
    match node {
        ASTNode::Assignment { target, .. } => {
            if let ASTNode::Variable { name, .. } = target.as_ref() {
                out.insert(name.clone());
            }
        }
        ASTNode::If {
            then_body,
            else_body,
            ..
        } => {
            for stmt in then_body {
                collect_assigned_var_names(stmt, out);
            }
            if let Some(body) = else_body {
                for stmt in body {
                    collect_assigned_var_names(stmt, out);
                }
            }
        }
        ASTNode::Program { statements, .. } => {
            for stmt in statements {
                collect_assigned_var_names(stmt, out);
            }
        }
        ASTNode::ScopeBox { body, .. } => {
            for stmt in body {
                collect_assigned_var_names(stmt, out);
            }
        }
        ASTNode::Loop { body, .. }
        | ASTNode::While { body, .. }
        | ASTNode::ForRange { body, .. } => {
            for stmt in body {
                collect_assigned_var_names(stmt, out);
            }
        }
        _ => {}
    }
}

pub(super) fn contains_var_name(expr: &ASTNode, target_var: &str) -> bool {
    match expr {
        ASTNode::Variable { name, .. } => name == target_var,
        ASTNode::BinaryOp { left, right, .. } => {
            contains_var_name(left.as_ref(), target_var)
                || contains_var_name(right.as_ref(), target_var)
        }
        ASTNode::UnaryOp { operand, .. } => contains_var_name(operand.as_ref(), target_var),
        ASTNode::MethodCall {
            object, arguments, ..
        } => {
            contains_var_name(object.as_ref(), target_var)
                || arguments
                    .iter()
                    .any(|arg| contains_var_name(arg, target_var))
        }
        ASTNode::FunctionCall { arguments, .. } => arguments
            .iter()
            .any(|arg| contains_var_name(arg, target_var)),
        ASTNode::FieldAccess { object, .. } => contains_var_name(object.as_ref(), target_var),
        ASTNode::Index { target, index, .. } => {
            contains_var_name(target.as_ref(), target_var)
                || contains_var_name(index.as_ref(), target_var)
        }
        _ => false,
    }
}
