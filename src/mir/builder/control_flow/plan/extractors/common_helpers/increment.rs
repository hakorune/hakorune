use crate::ast::{ASTNode, BinaryOperator};
use crate::mir::builder::control_flow::plan::canon::generic_loop::canon_update_for_loop_var;

/// ============================================================
/// Group 4: Loop Increment Extraction (Common for Plan line)
/// ============================================================

/// Phase 286 P2.2: Extract loop increment for Plan line patterns
///
/// Supports `<var> = <var> ( + | - | * | / ) <int_lit>` pattern only (PoC safety).
pub(crate) fn extract_loop_increment_plan(
    body: &[ASTNode],
    loop_var: &str,
) -> Result<Option<ASTNode>, String> {
    fn extract_increment_value(stmt: &ASTNode, loop_var: &str) -> Option<ASTNode> {
        let ASTNode::Assignment { target, value, .. } = stmt else {
            return None;
        };
        let ASTNode::Variable { name, .. } = target.as_ref() else {
            return None;
        };
        if name != loop_var {
            return None;
        }
        if canon_update_for_loop_var(stmt, loop_var).is_some() {
            return Some(value.as_ref().clone());
        }
        let ASTNode::BinaryOp {
            operator,
            left,
            right,
            ..
        } = value.as_ref()
        else {
            return None;
        };
        match operator {
            BinaryOperator::Add => {
                if let (ASTNode::Variable { name: lname, .. }, ASTNode::Literal { .. }) =
                    (left.as_ref(), right.as_ref())
                {
                    if lname == loop_var {
                        return Some(value.as_ref().clone());
                    }
                }
                if let (ASTNode::Literal { .. }, ASTNode::Variable { name: rname, .. }) =
                    (left.as_ref(), right.as_ref())
                {
                    if rname == loop_var {
                        return Some(value.as_ref().clone());
                    }
                }
                None
            }
            BinaryOperator::Subtract | BinaryOperator::Multiply | BinaryOperator::Divide => {
                let ASTNode::Variable { name: lname, .. } = left.as_ref() else {
                    return None;
                };
                if lname != loop_var {
                    return None;
                }
                if !matches!(right.as_ref(), ASTNode::Literal { .. }) {
                    return None;
                }
                Some(value.as_ref().clone())
            }
            _ => None,
        }
    }

    for stmt in body {
        if let Some(increment) = extract_increment_value(stmt, loop_var) {
            return Ok(Some(increment));
        }
    }

    // Fallback contract:
    // - Only the last top-level statement may supply this fallback step.
    // - The statement must be an assignment to the current loop var.
    // - This is used when canonical +/-/*// literal forms are absent,
    //   mainly for selfhost release-route loops with computed step values.
    if let Some(tail_value) = extract_tail_loop_assignment_value(body, loop_var) {
        return Ok(Some(tail_value));
    }

    let mut found: Option<ASTNode> = None;
    for stmt in body {
        let ASTNode::If {
            then_body,
            else_body,
            ..
        } = stmt
        else {
            continue;
        };
        let is_continue_tail =
            matches!(then_body.last(), Some(ASTNode::Continue { .. })) && else_body.is_none();
        let is_break_else = else_body
            .as_ref()
            .is_some_and(|body| body.len() == 1 && matches!(body[0], ASTNode::Break { .. }));
        if !is_continue_tail && !is_break_else {
            continue;
        }
        for inner in then_body {
            if let Some(increment) = extract_increment_value(inner, loop_var) {
                if found.is_some() {
                    return Ok(None);
                }
                found = Some(increment);
            }
        }
    }
    Ok(found)
}

fn extract_tail_loop_assignment_value(body: &[ASTNode], loop_var: &str) -> Option<ASTNode> {
    let ASTNode::Assignment { target, value, .. } = body.last()? else {
        return None;
    };
    let ASTNode::Variable { name, .. } = target.as_ref() else {
        return None;
    };
    if name != loop_var {
        return None;
    }
    Some(value.as_ref().clone())
}

