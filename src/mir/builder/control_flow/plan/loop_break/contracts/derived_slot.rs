//! Phase 29ab P4: Derived slot contract for loop_break route
//!
//! Responsibility:
//! - Extract a minimal derived-slot recipe for a single LoopBodyLocal variable
//!   used in loop_break conditions.
//! - No JoinIR emission; detection only.

use crate::ast::ASTNode;
use crate::mir::join_ir::lowering::common::body_local_derived_slot_emitter::BodyLocalDerivedSlotRecipe;

pub(crate) fn extract_body_local_derived_slot(
    name: &str,
    body: &[ASTNode],
) -> Result<Option<BodyLocalDerivedSlotRecipe>, String> {
    let break_guard_idx = match find_first_top_level_break_guard_if(body) {
        Some(idx) => idx,
        None => return Ok(None),
    };

    let (decl_idx, base_init_expr) = match find_top_level_local_init(body, name) {
        Some(result) => result,
        None => return Ok(None),
    };

    if decl_idx >= break_guard_idx {
        return Ok(None);
    }

    let (assign_idx, assign_cond, then_expr, else_expr) =
        match find_top_level_conditional_assignment(body, name, break_guard_idx) {
            Some(result) => result,
            None => return Ok(None),
        };

    if assign_idx <= decl_idx {
        return Ok(None);
    }

    if has_other_assignments(body, name, assign_idx) {
        return Ok(None);
    }

    Ok(Some(BodyLocalDerivedSlotRecipe {
        name: name.to_string(),
        base_init_expr,
        assign_cond,
        then_expr,
        else_expr: Some(else_expr),
    }))
}

pub(crate) fn extract_derived_slot_for_conditions(
    body_local_names_in_conditions: &[String],
    body: &[ASTNode],
) -> Result<Option<BodyLocalDerivedSlotRecipe>, String> {
    if body_local_names_in_conditions.len() != 1 {
        return Ok(None);
    }
    extract_body_local_derived_slot(&body_local_names_in_conditions[0], body)
}

fn find_first_top_level_break_guard_if(body: &[ASTNode]) -> Option<usize> {
    for (idx, stmt) in body.iter().enumerate() {
        if let ASTNode::If {
            then_body,
            else_body,
            ..
        } = stmt
        {
            if then_body.iter().any(|n| matches!(n, ASTNode::Break { .. })) {
                return Some(idx);
            }
            if let Some(else_body) = else_body {
                if else_body.iter().any(|n| matches!(n, ASTNode::Break { .. })) {
                    return Some(idx);
                }
            }
        }
    }
    None
}

fn find_top_level_local_init(body: &[ASTNode], name: &str) -> Option<(usize, ASTNode)> {
    for (idx, stmt) in body.iter().enumerate() {
        if let ASTNode::Local {
            variables,
            initial_values,
            ..
        } = stmt
        {
            if variables.len() != 1 {
                continue;
            }
            if variables[0] != name {
                continue;
            }
            let init = initial_values
                .get(0)
                .and_then(|v| v.as_ref())
                .map(|b| (*b.clone()).clone())?;
            return Some((idx, init));
        }
    }
    None
}

fn find_top_level_conditional_assignment(
    body: &[ASTNode],
    name: &str,
    break_guard_idx: usize,
) -> Option<(usize, ASTNode, ASTNode, ASTNode)> {
    for (idx, stmt) in body.iter().enumerate() {
        if idx >= break_guard_idx {
            break;
        }
        let ASTNode::If {
            condition,
            then_body,
            else_body,
            ..
        } = stmt else {
            continue;
        };

        let Some(else_body) = else_body.as_ref() else {
            continue;
        };

        let Some(then_expr) = extract_single_assignment_expr(then_body, name) else {
            continue;
        };
        let Some(else_expr) = extract_single_assignment_expr(else_body, name) else {
            continue;
        };

        return Some((idx, (*condition.clone()), then_expr, else_expr));
    }
    None
}

fn extract_single_assignment_expr(stmts: &[ASTNode], name: &str) -> Option<ASTNode> {
    if stmts.len() != 1 {
        return None;
    }
    match &stmts[0] {
        ASTNode::Assignment { target, value, .. } => {
            if matches!(&**target, ASTNode::Variable { name: n, .. } if n == name) {
                Some((**value).clone())
            } else {
                None
            }
        }
        _ => None,
    }
}

fn has_other_assignments(body: &[ASTNode], name: &str, assign_if_idx: usize) -> bool {
    body.iter().enumerate().any(|(idx, stmt)| {
        if idx == assign_if_idx {
            return false;
        }
        contains_assignment_to_name_in_node(stmt, name)
    })
}

fn contains_assignment_to_name_in_node(node: &ASTNode, name: &str) -> bool {
    match node {
        ASTNode::Assignment { target, value, .. } => {
            if matches!(&**target, ASTNode::Variable { name: n, .. } if n == name) {
                return true;
            }
            contains_assignment_to_name_in_node(target, name)
                || contains_assignment_to_name_in_node(value, name)
        }
        ASTNode::Nowait { variable, .. } => variable == name,
        ASTNode::If {
            condition,
            then_body,
            else_body,
            ..
        } => {
            contains_assignment_to_name_in_node(condition, name)
                || then_body
                    .iter()
                    .any(|n| contains_assignment_to_name_in_node(n, name))
                || else_body.as_ref().is_some_and(|e| {
                    e.iter()
                        .any(|n| contains_assignment_to_name_in_node(n, name))
                })
        }
        ASTNode::Loop { condition, body, .. } => {
            contains_assignment_to_name_in_node(condition, name)
                || body
                    .iter()
                    .any(|n| contains_assignment_to_name_in_node(n, name))
        }
        ASTNode::While { condition, body, .. } => {
            contains_assignment_to_name_in_node(condition, name)
                || body
                    .iter()
                    .any(|n| contains_assignment_to_name_in_node(n, name))
        }
        ASTNode::ForRange { body, .. } => body
            .iter()
            .any(|n| contains_assignment_to_name_in_node(n, name)),
        ASTNode::TryCatch {
            try_body,
            catch_clauses,
            finally_body,
            ..
        } => {
            try_body
                .iter()
                .any(|n| contains_assignment_to_name_in_node(n, name))
                || catch_clauses.iter().any(|c| {
                    c.body
                        .iter()
                        .any(|n| contains_assignment_to_name_in_node(n, name))
                })
                || finally_body.as_ref().is_some_and(|b| {
                    b.iter()
                        .any(|n| contains_assignment_to_name_in_node(n, name))
                })
        }
        ASTNode::ScopeBox { body, .. } => body
            .iter()
            .any(|n| contains_assignment_to_name_in_node(n, name)),
        _ => false,
    }
}
