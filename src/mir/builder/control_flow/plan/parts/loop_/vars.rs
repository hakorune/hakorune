use crate::mir::builder::MirBuilder;
use std::collections::{BTreeMap, BTreeSet};

pub(super) fn collect_carriers_from_condition(
    cond_vars: BTreeSet<String>,
    builder: &MirBuilder,
    current_bindings: &BTreeMap<String, crate::mir::ValueId>,
) -> BTreeSet<String> {
    cond_vars
        .into_iter()
        .filter(|name| {
            current_bindings.contains_key(name)
                || builder.variable_ctx.variable_map.contains_key(name)
        })
        .collect()
}

pub(super) fn condition_vars(condition: &crate::ast::ASTNode) -> BTreeSet<String> {
    let mut vars = BTreeSet::<String>::new();
    collect_condition_vars(condition, &mut vars);
    vars
}

fn collect_condition_vars(ast: &crate::ast::ASTNode, vars: &mut BTreeSet<String>) {
    use crate::ast::ASTNode;
    match ast {
        ASTNode::Variable { name, .. } => {
            vars.insert(name.clone());
        }
        ASTNode::UnaryOp { operand, .. } => collect_condition_vars(operand, vars),
        ASTNode::BinaryOp { left, right, .. } => {
            collect_condition_vars(left, vars);
            collect_condition_vars(right, vars);
        }
        ASTNode::GroupedAssignmentExpr { rhs, .. } => collect_condition_vars(rhs, vars),
        _ => {}
    }
}

pub(super) fn collect_assigned_vars(ast: &crate::ast::ASTNode, vars: &mut BTreeSet<String>) {
    use crate::ast::ASTNode;
    match ast {
        ASTNode::Assignment { target, .. } => {
            if let ASTNode::Variable { name, .. } = target.as_ref() {
                vars.insert(name.clone());
            }
        }
        ASTNode::Program { statements, .. } => {
            for stmt in statements {
                collect_assigned_vars(stmt, vars);
            }
        }
        ASTNode::If {
            then_body,
            else_body,
            ..
        } => {
            for stmt in then_body {
                collect_assigned_vars(stmt, vars);
            }
            if let Some(else_body) = else_body {
                for stmt in else_body {
                    collect_assigned_vars(stmt, vars);
                }
            }
        }
        ASTNode::Loop { body, .. }
        | ASTNode::While { body, .. }
        | ASTNode::ForRange { body, .. } => {
            for stmt in body {
                collect_assigned_vars(stmt, vars);
            }
        }
        ASTNode::ScopeBox { body, .. } => {
            for stmt in body {
                collect_assigned_vars(stmt, vars);
            }
        }
        _ => {}
    }
}

