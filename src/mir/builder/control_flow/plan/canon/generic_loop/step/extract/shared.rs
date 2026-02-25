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
