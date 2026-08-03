use super::super::super::facts_types::GenericLoopCarrierObservationV1;
use crate::ast::ASTNode;
use crate::mir::builder::control_flow::generic_loop_canon::matches_loop_increment;
use std::collections::BTreeSet;

/// Returns true when the loop body writes to variables other than the loop var.
pub(in crate::mir::builder) fn body_writes_non_loop_vars(
    body: &[ASTNode],
    loop_var: &str,
    loop_increment: &ASTNode,
) -> bool {
    for stmt in body {
        if matches_loop_increment(stmt, loop_var, loop_increment) {
            continue;
        }
        match stmt {
            ASTNode::Assignment { target, .. } => match target.as_ref() {
                ASTNode::Variable { name, .. } if name == loop_var => {}
                _ => return true,
            },
            ASTNode::Local { variables, .. } => {
                if variables.iter().any(|name| name != loop_var) {
                    return true;
                }
            }
            _ => {}
        }
    }
    false
}

/// Observe nested writes without consulting Builder state or choosing a route.
pub(in crate::mir::builder) fn observe_generic_loop_carrier_observation(
    body: &[ASTNode],
    loop_var: &str,
) -> GenericLoopCarrierObservationV1 {
    let mut targets = BTreeSet::new();
    match collect_recursive_carrier_targets(body, loop_var, false, &mut targets) {
        Ok(()) if targets.is_empty() => GenericLoopCarrierObservationV1::CompleteNoRecursiveCarrier,
        Ok(()) => {
            GenericLoopCarrierObservationV1::CompleteRecursiveCarrier(targets.into_iter().collect())
        }
        Err(CarrierObservationError::Unavailable(container)) => {
            GenericLoopCarrierObservationV1::Unavailable(container.to_string())
        }
        Err(CarrierObservationError::Ambiguous(reason)) => {
            GenericLoopCarrierObservationV1::Ambiguous(reason.to_string())
        }
    }
}

enum CarrierObservationError {
    Unavailable(&'static str),
    Ambiguous(&'static str),
}

fn collect_recursive_carrier_targets(
    body: &[ASTNode],
    loop_var: &str,
    nested: bool,
    targets: &mut BTreeSet<String>,
) -> Result<(), CarrierObservationError> {
    for stmt in body {
        match stmt {
            ASTNode::Assignment { target, .. } if nested => match target.as_ref() {
                ASTNode::Variable { name, .. } if name != loop_var => {
                    targets.insert(name.clone());
                }
                ASTNode::Variable { .. } => {}
                _ => return Err(CarrierObservationError::Ambiguous("assignment target")),
            },
            // The V1 physical carrier collector/lowerer does not consume
            // compound assignments yet.  Keep this boundary unavailable
            // instead of claiming a recursive carrier the consumer misses.
            ASTNode::CompoundAssignment { .. } if nested => {
                return Err(CarrierObservationError::Unavailable("CompoundAssignment"))
            }
            ASTNode::If {
                then_body,
                else_body,
                ..
            } => {
                collect_recursive_carrier_targets(then_body, loop_var, true, targets)?;
                if let Some(else_body) = else_body {
                    collect_recursive_carrier_targets(else_body, loop_var, true, targets)?;
                }
            }
            ASTNode::Loop { body, .. } => {
                collect_recursive_carrier_targets(body, loop_var, true, targets)?;
            }
            ASTNode::ScopeBox { body, .. } => {
                collect_recursive_carrier_targets(body, loop_var, nested, targets)?;
            }
            // Program is preserved by scope flattening, but the V1 carrier
            // consumer has no matching arm.  Do not silently recurse here.
            ASTNode::Program { .. } => return Err(CarrierObservationError::Unavailable("Program")),
            ASTNode::LoopRange { .. } => {
                return Err(CarrierObservationError::Unavailable("LoopRange"))
            }
            ASTNode::Lambda { .. } => return Err(CarrierObservationError::Unavailable("Lambda")),
            ASTNode::BlockExpr { .. } => {
                return Err(CarrierObservationError::Unavailable("BlockExpr"))
            }
            ASTNode::TryCatch { .. } => {
                return Err(CarrierObservationError::Unavailable("TryCatch"))
            }
            ASTNode::TaskScope { .. } => {
                return Err(CarrierObservationError::Unavailable("TaskScope"))
            }
            ASTNode::ContextScope { .. } => {
                return Err(CarrierObservationError::Unavailable("ContextScope"))
            }
            ASTNode::FastMemRegion { .. } => {
                return Err(CarrierObservationError::Unavailable("FastMemRegion"))
            }
            ASTNode::BuildGate { .. } => {
                return Err(CarrierObservationError::Unavailable("BuildGate"))
            }
            _ => {}
        }
    }
    Ok(())
}

/// Collect loop var candidates from body by finding variables used in increment expressions.
pub(in crate::mir::builder) fn collect_loop_var_candidates_from_body(
    body: &[ASTNode],
) -> Vec<String> {
    let mut out = Vec::new();
    fn walk(stmt: &ASTNode, out: &mut Vec<String>) {
        match stmt {
            ASTNode::Assignment { target, value, .. } => {
                if let ASTNode::Variable { name, .. } = target.as_ref() {
                    if let ASTNode::BinaryOp {
                        operator,
                        left,
                        right,
                        ..
                    } = value.as_ref()
                    {
                        if matches!(
                            operator,
                            crate::ast::BinaryOperator::Add | crate::ast::BinaryOperator::Subtract
                        ) && (matches!(left.as_ref(), ASTNode::Variable { name: ln, .. } if ln == name)
                            || matches!(right.as_ref(), ASTNode::Variable { name: rn, .. } if rn == name))
                        {
                            if !out.iter().any(|v| v == name) {
                                out.push(name.clone());
                            }
                        }
                    }
                }
            }
            ASTNode::MethodCall { object, .. } => {
                if let Some(name) = receiver_candidate_name(object.as_ref()) {
                    if !out.iter().any(|v| v == &name) {
                        out.push(name);
                    }
                }
            }
            ASTNode::Local { initial_values, .. } => {
                for init in initial_values.iter().flatten() {
                    walk(init.as_ref(), out);
                }
            }
            ASTNode::If {
                then_body,
                else_body,
                ..
            } => {
                for s in then_body {
                    walk(s, out);
                }
                if let Some(eb) = else_body {
                    for s in eb {
                        walk(s, out);
                    }
                }
            }
            ASTNode::Loop { body, .. } => {
                for s in body {
                    walk(s, out);
                }
            }
            ASTNode::Program { statements, .. } => {
                for s in statements {
                    walk(s, out);
                }
            }
            _ => {}
        }
    }
    for stmt in body {
        walk(stmt, &mut out);
    }
    out
}

pub(in crate::mir::builder) fn collect_increment_loop_var_candidates_from_body(
    body: &[ASTNode],
) -> Vec<String> {
    let mut out = Vec::new();
    fn walk(stmt: &ASTNode, out: &mut Vec<String>) {
        match stmt {
            ASTNode::Assignment { target, value, .. } => {
                if let ASTNode::Variable { name, .. } = target.as_ref() {
                    if assignment_value_increments_var(value.as_ref(), name)
                        && !out.iter().any(|v| v == name)
                    {
                        out.push(name.clone());
                    }
                }
            }
            ASTNode::Local { initial_values, .. } => {
                for init in initial_values.iter().flatten() {
                    walk(init.as_ref(), out);
                }
            }
            ASTNode::If {
                then_body,
                else_body,
                ..
            } => {
                for s in then_body {
                    walk(s, out);
                }
                if let Some(eb) = else_body {
                    for s in eb {
                        walk(s, out);
                    }
                }
            }
            ASTNode::Loop { body, .. } => {
                for s in body {
                    walk(s, out);
                }
            }
            ASTNode::Program { statements, .. } => {
                for s in statements {
                    walk(s, out);
                }
            }
            _ => {}
        }
    }
    for stmt in body {
        walk(stmt, &mut out);
    }
    out
}

fn assignment_value_increments_var(value: &ASTNode, name: &str) -> bool {
    let ASTNode::BinaryOp {
        operator,
        left,
        right,
        ..
    } = value
    else {
        return false;
    };
    matches!(
        operator,
        crate::ast::BinaryOperator::Add | crate::ast::BinaryOperator::Subtract
    ) && (matches!(left.as_ref(), ASTNode::Variable { name: ln, .. } if ln == name)
        || matches!(right.as_ref(), ASTNode::Variable { name: rn, .. } if rn == name))
}

fn receiver_candidate_name(object: &ASTNode) -> Option<String> {
    match object {
        ASTNode::Variable { name, .. } => Some(name.clone()),
        ASTNode::Me { .. } => Some("me".to_string()),
        _ => None,
    }
}

pub(in crate::mir::builder) fn body_has_break_or_continue_stmt(stmt: &ASTNode) -> bool {
    match stmt {
        ASTNode::Break { .. } | ASTNode::Continue { .. } => true,
        ASTNode::If {
            then_body,
            else_body,
            ..
        } => {
            then_body.iter().any(body_has_break_or_continue_stmt)
                || else_body
                    .as_ref()
                    .is_some_and(|body| body.iter().any(body_has_break_or_continue_stmt))
        }
        ASTNode::Loop { body, .. } => body.iter().any(body_has_break_or_continue_stmt),
        ASTNode::Program { statements, .. } => {
            statements.iter().any(body_has_break_or_continue_stmt)
        }
        _ => false,
    }
}
