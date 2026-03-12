//! Phase 107: Balanced depth-scan policy (json_cur find_balanced_* family)
//!
//! Responsibility (analysis only):
//! - Recognize the `depth` scan loop shape with nested-if + `return i`
//! - Produce a loop-break-compatible break condition + derived recipe inputs
//! - Fail-fast with tagged reasons when the shape is close but unsupported

use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
use crate::mir::analysis::expr_view;
use crate::mir::join_ir::lowering::common::balanced_depth_scan_emitter::BalancedDepthScanRecipe;
use crate::mir::join_ir::lowering::error_tags;
use crate::mir::join_ir::lowering::loop_update_analyzer::{UpdateExpr, UpdateRhs};
use crate::mir::join_ir::BinOpKind;

use super::post_loop_early_return_plan::PostLoopEarlyReturnPlan;
use super::PolicyDecision;
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct BalancedDepthScanPolicyResult {
    pub break_condition_node: ASTNode,
    pub allowed_body_locals_for_conditions: Vec<String>,
    pub carrier_updates_override: BTreeMap<String, UpdateExpr>,
    pub derived_recipe: BalancedDepthScanRecipe,
    pub post_loop_early_return: PostLoopEarlyReturnPlan,
}

pub fn classify_balanced_depth_scan_array_end(
    condition: &ASTNode,
    body: &[ASTNode],
) -> PolicyDecision<BalancedDepthScanPolicyResult> {
    classify_balanced_depth_scan(condition, body, "[", "]")
}

pub fn classify_balanced_depth_scan_object_end(
    condition: &ASTNode,
    body: &[ASTNode],
) -> PolicyDecision<BalancedDepthScanPolicyResult> {
    classify_balanced_depth_scan(condition, body, "{", "}")
}

/// Decide balanced depth-scan family (SSOT ordering).
///
/// IMPORTANT: `Reject` means "close-but-unsupported" for that family, not "not this family".
/// We only return `Reject` if no other family matches.
pub fn decide(
    condition: &ASTNode,
    body: &[ASTNode],
) -> PolicyDecision<BalancedDepthScanPolicyResult> {
    let array = classify_balanced_depth_scan_array_end(condition, body);
    match array {
        PolicyDecision::Use(_) => array,
        PolicyDecision::Reject(_) => {
            let object = classify_balanced_depth_scan_object_end(condition, body);
            match object {
                PolicyDecision::Use(_) => object,
                PolicyDecision::Reject(_) => array,
                PolicyDecision::None => array,
            }
        }
        PolicyDecision::None => classify_balanced_depth_scan_object_end(condition, body),
    }
}

fn var(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.to_string(),
        span: Span::unknown(),
    }
}

fn classify_balanced_depth_scan(
    condition: &ASTNode,
    body: &[ASTNode],
    open: &str,
    close: &str,
) -> PolicyDecision<BalancedDepthScanPolicyResult> {
    // bounded loop: loop(i < n)
    let (loop_counter_name, bound_name) = match extract_bounded_loop_counter(condition) {
        Some(v) => v,
        None => return PolicyDecision::None,
    };

    let summary = match extract_depth_scan_shape(body, &loop_counter_name, open, close) {
        Ok(Some(v)) => v,
        Ok(None) => return PolicyDecision::None,
        Err(reason) => return PolicyDecision::Reject(reason),
    };

    let depth_delta_name = "depth_delta".to_string();
    let depth_next_name = "depth_next".to_string();
    if summary.declared_locals.contains(&depth_delta_name)
        || summary.declared_locals.contains(&depth_next_name)
    {
        return PolicyDecision::Reject(error_tags::freeze(
            "[phase107/balanced_depth_scan/contract/name_conflict] 'depth_delta' or 'depth_next' is already declared in the loop body",
        ));
    }

    let break_condition_node = ASTNode::BinaryOp {
        operator: BinaryOperator::And,
        left: Box::new(eq_str(var(&summary.ch_name), close)),
        right: Box::new(eq_int(var(&depth_next_name), 0)),
        span: Span::unknown(),
    };

    // Carrier update override (SSOT): depth = depth + depth_delta, i = i + 1
    let mut carrier_updates_override: BTreeMap<String, UpdateExpr> = BTreeMap::new();
    carrier_updates_override.insert(loop_counter_name.clone(), UpdateExpr::Const(1));
    carrier_updates_override.insert(
        summary.depth_name.clone(),
        UpdateExpr::BinOp {
            lhs: summary.depth_name.clone(),
            op: BinOpKind::Add,
            rhs: UpdateRhs::Variable(depth_delta_name.clone()),
        },
    );

    PolicyDecision::Use(BalancedDepthScanPolicyResult {
        break_condition_node,
        allowed_body_locals_for_conditions: vec![summary.ch_name.clone(), depth_next_name.clone()],
        carrier_updates_override,
        derived_recipe: BalancedDepthScanRecipe {
            depth_var: summary.depth_name,
            ch_var: summary.ch_name,
            open: open.to_string(),
            close: close.to_string(),
            depth_delta_name,
            depth_next_name,
        },
        post_loop_early_return: PostLoopEarlyReturnPlan {
            cond: ASTNode::BinaryOp {
                operator: BinaryOperator::Less,
                left: Box::new(var(&loop_counter_name)),
                right: Box::new(var(&bound_name)),
                span: Span::unknown(),
            },
            ret_expr: var(&loop_counter_name),
        },
    })
}

#[derive(Debug)]
struct DepthScanShapeSummary {
    ch_name: String,
    depth_name: String,
    declared_locals: std::collections::BTreeSet<String>,
}

fn extract_bounded_loop_counter(condition: &ASTNode) -> Option<(String, String)> {
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

fn extract_depth_scan_shape(
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

// Phase 255 P2: var() function removed; route-entry callers now rely on the
// plan-side helpers exposed through the active `joinir::route_entry` surface.

fn eq_str(left: ASTNode, s: &str) -> ASTNode {
    ASTNode::BinaryOp {
        operator: BinaryOperator::Equal,
        left: Box::new(left),
        right: Box::new(ASTNode::Literal {
            value: LiteralValue::String(s.to_string()),
            span: Span::unknown(),
        }),
        span: Span::unknown(),
    }
}

fn eq_int(left: ASTNode, n: i64) -> ASTNode {
    ASTNode::BinaryOp {
        operator: BinaryOperator::Equal,
        left: Box::new(left),
        right: Box::new(ASTNode::Literal {
            value: LiteralValue::Integer(n),
            span: Span::unknown(),
        }),
        span: Span::unknown(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Span;
    use crate::parser::NyashParser;

    fn span() -> Span {
        Span::unknown()
    }

    fn var_node(name: &str) -> ASTNode {
        ASTNode::Variable {
            name: name.to_string(),
            span: span(),
        }
    }

    fn int_lit(n: i64) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Integer(n),
            span: span(),
        }
    }

    fn str_lit(s: &str) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::String(s.to_string()),
            span: span(),
        }
    }

    fn bin(op: BinaryOperator, left: ASTNode, right: ASTNode) -> ASTNode {
        ASTNode::BinaryOp {
            operator: op,
            left: Box::new(left),
            right: Box::new(right),
            span: span(),
        }
    }

    fn if_then(cond: ASTNode, then_body: Vec<ASTNode>) -> ASTNode {
        ASTNode::If {
            condition: Box::new(cond),
            then_body,
            else_body: None,
            span: span(),
        }
    }

    fn assign(target: ASTNode, value: ASTNode) -> ASTNode {
        ASTNode::Assignment {
            target: Box::new(target),
            value: Box::new(value),
            span: span(),
        }
    }

    fn blockexpr(prelude_stmts: Vec<ASTNode>, tail_expr: ASTNode) -> ASTNode {
        ASTNode::BlockExpr {
            prelude_stmts,
            tail_expr: Box::new(tail_expr),
            span: span(),
        }
    }

    fn find_first_loop<'a>(node: &'a ASTNode) -> Option<(&'a ASTNode, &'a [ASTNode])> {
        match node {
            ASTNode::Loop {
                condition, body, ..
            } => Some((condition.as_ref(), body.as_slice())),
            ASTNode::Program { statements, .. } => statements.iter().find_map(find_first_loop),
            ASTNode::BoxDeclaration {
                methods,
                constructors,
                static_init,
                ..
            } => {
                for v in methods.values() {
                    if let Some(found) = find_first_loop(v) {
                        return Some(found);
                    }
                }
                for v in constructors.values() {
                    if let Some(found) = find_first_loop(v) {
                        return Some(found);
                    }
                }
                if let Some(init) = static_init {
                    if let Some(found) = init.iter().find_map(find_first_loop) {
                        return Some(found);
                    }
                }
                None
            }
            ASTNode::FunctionDeclaration { body, .. } => body.iter().find_map(find_first_loop),
            ASTNode::If {
                then_body,
                else_body,
                ..
            } => then_body
                .iter()
                .find_map(find_first_loop)
                .or_else(|| else_body.as_ref()?.iter().find_map(find_first_loop)),
            _ => None,
        }
    }

    #[test]
    fn detects_balanced_array_end_min_shape() {
        // loop(i < n) {
        //   local ch = s.substring(i, i+1)
        //   if ch == "[" { depth = depth + 1 }
        //   if ch == "]" { depth = depth - 1; if depth == 0 { return i } }
        //   i = i + 1
        // }
        let condition = bin(BinaryOperator::Less, var_node("i"), var_node("n"));
        let local_ch = ASTNode::Local {
            variables: vec!["ch".to_string()],
            initial_values: vec![Some(Box::new(ASTNode::MethodCall {
                object: Box::new(var_node("s")),
                method: "substring".to_string(),
                arguments: vec![
                    var_node("i"),
                    bin(BinaryOperator::Add, var_node("i"), int_lit(1)),
                ],
                span: span(),
            }))],
            span: span(),
        };
        let open_branch = if_then(
            bin(BinaryOperator::Equal, var_node("ch"), str_lit("[")),
            vec![ASTNode::Assignment {
                target: Box::new(var_node("depth")),
                value: Box::new(bin(BinaryOperator::Add, var_node("depth"), int_lit(1))),
                span: span(),
            }],
        );
        let close_branch = if_then(
            bin(BinaryOperator::Equal, var_node("ch"), str_lit("]")),
            vec![
                ASTNode::Assignment {
                    target: Box::new(var_node("depth")),
                    value: Box::new(bin(BinaryOperator::Subtract, var_node("depth"), int_lit(1))),
                    span: span(),
                },
                if_then(
                    bin(BinaryOperator::Equal, var_node("depth"), int_lit(0)),
                    vec![ASTNode::Return {
                        value: Some(Box::new(var_node("i"))),
                        span: span(),
                    }],
                ),
            ],
        );
        let tail_inc = ASTNode::Assignment {
            target: Box::new(var_node("i")),
            value: Box::new(bin(BinaryOperator::Add, var_node("i"), int_lit(1))),
            span: span(),
        };

        let body = vec![local_ch, open_branch, close_branch, tail_inc];
        let decision = classify_balanced_depth_scan_array_end(&condition, &body);
        let result = match decision {
            PolicyDecision::Use(v) => v,
            other => panic!("expected Use, got {:?}", other),
        };

        assert!(
            matches!(
                &result.post_loop_early_return.cond,
                ASTNode::BinaryOp {
                    operator: BinaryOperator::Less,
                    left,
                    right,
                    ..
                } if matches!(left.as_ref(), ASTNode::Variable { name, .. } if name == "i")
                    && matches!(right.as_ref(), ASTNode::Variable { name, .. } if name == "n")
            ),
            "post-loop cond must be `i < n`, got {:?}",
            result.post_loop_early_return.cond
        );
        assert!(
            matches!(
                &result.post_loop_early_return.ret_expr,
                ASTNode::Variable { name, .. } if name == "i"
            ),
            "post-loop ret_expr must be `i`, got {:?}",
            result.post_loop_early_return.ret_expr
        );
        assert!(result
            .allowed_body_locals_for_conditions
            .contains(&"ch".to_string()));
        assert!(result
            .allowed_body_locals_for_conditions
            .contains(&"depth_next".to_string()));
        assert!(result.carrier_updates_override.contains_key("i"));
        assert!(result.carrier_updates_override.contains_key("depth"));
    }

    #[test]
    fn detects_balanced_array_end_min_shape_from_parser_ast() {
        let src = r#"
static box Main {
  f(s, idx) {
    local n = s.length()
    if s.substring(idx, idx+1) != "[" { return -1 }
    local depth = 0
    local i = idx
    loop (i < n) {
      local ch = s.substring(i, i+1)
      if ch == "[" { depth = depth + 1 }
      if ch == "]" { depth = depth - 1  if depth == 0 { return i } }
      i = i + 1
    }
    return -1
  }
}
"#;
        let ast = NyashParser::parse_from_string(src).expect("parse ok");
        let (condition, body) = find_first_loop(&ast).expect("find loop");
        let decision = classify_balanced_depth_scan_array_end(condition, body);
        assert!(
            matches!(decision, PolicyDecision::Use(_)),
            "got {:?}",
            decision
        );
    }

    #[test]
    fn analysis_view_accepts_add_commutative_but_sub_is_non_commutative() {
        let depth_add = vec![assign(
            var_node("depth"),
            bin(BinaryOperator::Add, var_node("depth"), int_lit(1)),
        )];
        let v = expr_view::find_single_self_update_assign_by_const_any_target(&depth_add, 1, true)
            .expect("depth = depth + 1");
        assert_eq!(v.target_var, "depth");
        assert_eq!(v.rhs.op, expr_view::SelfUpdateOp::Add);
        assert_eq!(v.rhs.step, 1);
        assert_eq!(v.rhs.commute, expr_view::Commute::AsWritten);

        let depth_add_swapped = vec![assign(
            var_node("depth"),
            bin(BinaryOperator::Add, int_lit(1), var_node("depth")),
        )];
        let v = expr_view::find_single_self_update_assign_by_const_any_target(
            &depth_add_swapped,
            1,
            true,
        )
        .expect("depth = 1 + depth");
        assert_eq!(v.target_var, "depth");
        assert_eq!(v.rhs.op, expr_view::SelfUpdateOp::Add);
        assert_eq!(v.rhs.step, 1);
        assert_eq!(v.rhs.commute, expr_view::Commute::Swapped);

        let depth_sub = vec![assign(
            var_node("depth"),
            bin(BinaryOperator::Subtract, var_node("depth"), int_lit(1)),
        )];
        let v = expr_view::find_single_self_update_assign_by_const_any_target(&depth_sub, -1, true)
            .expect("depth = depth - 1");
        assert_eq!(v.target_var, "depth");
        assert_eq!(v.rhs.op, expr_view::SelfUpdateOp::Sub);
        assert_eq!(v.rhs.step, 1);
        assert_eq!(v.rhs.commute, expr_view::Commute::AsWritten);

        let depth_sub_wrong = vec![assign(
            var_node("depth"),
            bin(BinaryOperator::Subtract, int_lit(1), var_node("depth")),
        )];
        assert!(
            expr_view::find_single_self_update_assign_by_const_any_target(
                &depth_sub_wrong,
                -1,
                true
            )
            .is_none(),
            "1 - depth must be rejected"
        );
    }

    #[test]
    fn analysis_view_blockexpr_accepts_single_update_and_rejects_ambiguous_or_control_flow_prelude()
    {
        let ok_tail_update = vec![blockexpr(
            vec![ASTNode::Local {
                variables: vec!["t".to_string()],
                initial_values: vec![Some(Box::new(int_lit(0)))],
                span: span(),
            }],
            assign(
                var_node("depth"),
                bin(BinaryOperator::Add, var_node("depth"), int_lit(1)),
            ),
        )];
        assert!(
            expr_view::find_single_self_update_assign_by_const_any_target(&ok_tail_update, 1, true)
                .is_some(),
            "BlockExpr with single update (tail) must be accepted"
        );

        let two_updates = vec![blockexpr(
            vec![
                assign(
                    var_node("depth"),
                    bin(BinaryOperator::Add, var_node("depth"), int_lit(1)),
                ),
                assign(
                    var_node("depth"),
                    bin(BinaryOperator::Add, var_node("depth"), int_lit(1)),
                ),
            ],
            int_lit(0),
        )];
        assert!(
            expr_view::find_single_self_update_assign_by_const_any_target(&two_updates, 1, true)
                .is_none(),
            "BlockExpr with 2 updates must be rejected"
        );

        let prelude_has_if = vec![blockexpr(
            vec![if_then(
                int_lit(1),
                vec![assign(
                    var_node("depth"),
                    bin(BinaryOperator::Add, var_node("depth"), int_lit(1)),
                )],
            )],
            int_lit(0),
        )];
        assert!(
            expr_view::find_single_self_update_assign_by_const_any_target(&prelude_has_if, 1, true)
                .is_none(),
            "BlockExpr prelude with If must be rejected"
        );
    }
}
