use crate::ast::{ASTNode, BinaryOperator, LiteralValue};
use crate::mir::analysis::expr_view;
use crate::mir::join_ir::lowering::error_tags;

#[derive(Debug)]
pub(super) struct DepthScanShapeSummary {
    pub(super) ch_name: String,
    pub(super) depth_name: String,
    pub(super) declared_locals: std::collections::BTreeSet<String>,
}

pub(super) fn extract_bounded_loop_counter(condition: &ASTNode) -> Option<(String, String)> {
    match condition {
        ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left,
            right,
            ..
        } => match (left.as_ref(), right.as_ref()) {
            (ASTNode::Variable { name: i, .. }, ASTNode::Variable { name: n, .. }) => {
                Some((i.clone(), n.clone()))
            }
            _ => None,
        },
        _ => None,
    }
}

pub(super) fn extract_depth_scan_shape(
    body: &[ASTNode],
    loop_counter_name: &str,
    open: &str,
    close: &str,
) -> Result<Option<DepthScanShapeSummary>, String> {
    if body.is_empty() {
        return Err(error_tags::freeze(
            "[phase107/balanced_depth_scan/contract/empty_body] empty loop body",
        ));
    }

    // Collect declared locals to protect derived slot names.
    let mut declared_locals: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    for stmt in body {
        if let ASTNode::Local { variables, .. } = stmt {
            for v in variables {
                declared_locals.insert(v.clone());
            }
        }
    }

    // Find `local ch = s.substring(i, i+1)` (name may vary, but must be a single local).
    // If missing, this is not a depth-scan candidate.
    let Some(ch_name) = find_substring_body_local(body, loop_counter_name) else {
        return Ok(None);
    };

    // Find open/close branches and extract `depth` name.
    let (depth_from_open, depth_from_close, has_return_i) =
        find_depth_branches(body, &ch_name, loop_counter_name, open, close)?;

    let (Some(depth_from_open), Some(depth_from_close)) = (depth_from_open, depth_from_close)
    else {
        // Not a depth-scan candidate: no `if ch == open/close` pair.
        return Ok(None);
    };

    if depth_from_open != depth_from_close {
        return Err(error_tags::freeze(&format!(
            "[phase107/balanced_depth_scan/contract/depth_mismatch] depth variable differs: open='{}', close='{}'",
            depth_from_open, depth_from_close
        )));
    }

    if !has_return_i {
        return Err(error_tags::freeze_with_hint(
            "phase107/balanced_depth_scan/missing_return_i",
            "missing `if depth == 0 { return i }` inside close branch",
            "ensure 'if depth == 0 { return i }' is inside close-branch",
        ));
    }

    // Require a tail `i = i + 1` at top-level (keeps the family narrow).
    let has_tail_inc = body.iter().any(|n| is_inc_assign(n, loop_counter_name, 1));
    if !has_tail_inc {
        return Err(error_tags::freeze_with_hint(
            "phase107/balanced_depth_scan/missing_tail_inc",
            "missing `i = i + 1` tail update",
            "add tail update 'i = i + 1' at top-level",
        ));
    }

    // Reject other breaks/continues and non-matching returns (fail-fast).
    let mut return_count = 0usize;
    for stmt in body {
        scan_control_flow(stmt, &mut return_count)?;
    }
    if return_count != 1 {
        return Err(error_tags::freeze(&format!(
            "[phase107/balanced_depth_scan/contract/return_count] expected exactly 1 return in loop body, got {}",
            return_count
        )));
    }

    Ok(Some(DepthScanShapeSummary {
        ch_name,
        depth_name: depth_from_open,
        declared_locals,
    }))
}

fn scan_control_flow(node: &ASTNode, return_count: &mut usize) -> Result<(), String> {
    match node {
        ASTNode::Break { .. } => Err(error_tags::freeze_with_hint(
            "phase107/balanced_depth_scan/unexpected_break",
            "break is not allowed in this family (return-in-loop only)",
            "use 'return i' form (no break) in this family",
        )),
        ASTNode::Continue { .. } => Err(error_tags::freeze_with_hint(
            "phase107/balanced_depth_scan/unexpected_continue",
            "continue is not allowed in this family",
            "remove continue, use tail increment instead",
        )),
        ASTNode::Return { .. } => {
            *return_count += 1;
            Ok(())
        }
        ASTNode::If {
            then_body,
            else_body,
            ..
        } => {
            for s in then_body {
                scan_control_flow(s, return_count)?;
            }
            if let Some(else_body) = else_body {
                for s in else_body {
                    scan_control_flow(s, return_count)?;
                }
            }
            Ok(())
        }
        _ => Ok(()),
    }
}

fn find_substring_body_local(body: &[ASTNode], loop_counter_name: &str) -> Option<String> {
    for stmt in body {
        let (name, init) = match stmt {
            ASTNode::Local {
                variables,
                initial_values,
                ..
            } if variables.len() == 1 && initial_values.len() == 1 => {
                (variables[0].clone(), initial_values[0].as_deref()?)
            }
            _ => continue,
        };

        let (object, method, args) = match init {
            ASTNode::MethodCall {
                object,
                method,
                arguments,
                ..
            } => (object.as_ref(), method.as_str(), arguments.as_slice()),
            _ => continue,
        };
        if method != "substring" {
            continue;
        }
        if !matches!(object, ASTNode::Variable { .. }) {
            continue;
        }
        if args.len() != 2 {
            continue;
        }
        if !matches!(&args[0], ASTNode::Variable { name, .. } if name == loop_counter_name) {
            continue;
        }
        if !is_var_plus_int(&args[1], loop_counter_name, 1) {
            continue;
        }
        return Some(name);
    }
    None
}

fn find_depth_branches(
    body: &[ASTNode],
    ch_name: &str,
    loop_counter_name: &str,
    open: &str,
    close: &str,
) -> Result<(Option<String>, Option<String>, bool), String> {
    let mut open_depth: Option<String> = None;
    let mut close_depth: Option<String> = None;
    let mut has_return_i = false;

    for stmt in body {
        let (cond, then_body) = match stmt {
            ASTNode::If {
                condition,
                then_body,
                else_body,
                ..
            } if else_body.is_none() => (condition.as_ref(), then_body.as_slice()),
            _ => continue,
        };

        let lit = match cond {
            ASTNode::BinaryOp {
                operator: BinaryOperator::Equal,
                left,
                right,
                ..
            } if matches!(left.as_ref(), ASTNode::Variable { name, .. } if name == ch_name) => {
                match right.as_ref() {
                    ASTNode::Literal {
                        value: LiteralValue::String(s),
                        ..
                    } => s.as_str(),
                    _ => continue,
                }
            }
            _ => continue,
        };

        if lit == open {
            let depth_name = find_depth_delta_assign(then_body, 1)?;
            open_depth = Some(depth_name);
        } else if lit == close {
            let depth_name = find_depth_delta_assign(then_body, -1)?;
            close_depth = Some(depth_name.clone());
            has_return_i = find_depth_zero_return(then_body, &depth_name, loop_counter_name);
        }
    }
    Ok((open_depth, close_depth, has_return_i))
}

fn find_depth_delta_assign(stmts: &[ASTNode], delta: i64) -> Result<String, String> {
    if let Some(v) =
        expr_view::find_single_self_update_assign_by_const_any_target(stmts, delta, true)
    {
        return Ok(v.target_var.to_string());
    }

    // Diagnostic summary when enabled (single line only).
    if crate::config::env::joinir_dev::debug_enabled() {
        let ring0 = crate::runtime::get_global_ring0();
        ring0.log.debug(&format!(
            "[phase107/balanced_depth_scan/debug] missing_depth_update delta={} stmts={}",
            delta,
            stmts.len()
        ));
    }

    Err(error_tags::freeze(&format!(
        "[phase107/balanced_depth_scan/contract/missing_depth_update] missing `depth = depth {} 1` in branch",
        if delta == 1 { "+" } else { "-" }
    )))
}

fn find_depth_zero_return(stmts: &[ASTNode], depth_name: &str, loop_counter_name: &str) -> bool {
    for stmt in stmts {
        let (cond, then_body) = match stmt {
            ASTNode::If {
                condition,
                then_body,
                else_body,
                ..
            } if else_body.is_none() => (condition.as_ref(), then_body.as_slice()),
            _ => continue,
        };

        let is_depth_zero = matches!(
            cond,
            ASTNode::BinaryOp {
                operator: BinaryOperator::Equal,
                left,
                right,
                ..
            }
            if matches!(left.as_ref(), ASTNode::Variable { name, .. } if name == depth_name)
                && matches!(right.as_ref(), ASTNode::Literal { value: LiteralValue::Integer(0), .. })
        );
        if !is_depth_zero {
            continue;
        }

        if then_body.iter().any(|n| {
            matches!(
                n,
                ASTNode::Return { value: Some(v), .. }
                if matches!(v.as_ref(), ASTNode::Variable { name, .. } if name == loop_counter_name)
            )
        }) {
            return true;
        }
    }
    false
}

fn is_inc_assign(node: &ASTNode, var_name: &str, step: i64) -> bool {
    let Some((target_name, rhs)) = expr_view::match_assignment_to_var(node) else {
        return false;
    };
    if target_name != var_name {
        return false;
    }
    expr_view::match_add_by_const(rhs, var_name, step, true).is_some()
}

fn is_var_plus_int(node: &ASTNode, var_name: &str, n: i64) -> bool {
    expr_view::match_add_by_const(node, var_name, n, true).is_some()
}
