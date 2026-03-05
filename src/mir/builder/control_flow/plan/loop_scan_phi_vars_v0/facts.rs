//! Facts for loop_scan_phi_vars_v0 (one-shape, planner-required only).
//!
//! Accepts the outer loop pattern in PhiInjectorBox._collect_phi_vars/2:
//! loop(i < n) with nested break-search-loop + found-if + collect-loop.

use crate::ast::{ASTNode, BinaryOperator, LiteralValue};
use crate::mir::builder::control_flow::plan::planner::Freeze;
use crate::mir::builder::control_flow::plan::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::plan::facts::no_exit_block::try_build_no_exit_block_recipe;
use crate::mir::builder::control_flow::plan::facts::stmt_view::try_build_stmt_only_block_recipe;
use crate::mir::builder::control_flow::plan::recipes::RecipeBody;
use crate::mir::policies::BodyLoweringPolicy;

use super::recipe::{LoopScanPhiSegment, LoopScanPhiVarsV0Recipe, NestedLoopRecipe};

#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct LoopScanPhiVarsV0Facts {
    pub loop_var: String,
    pub limit_var: String,
    pub condition: ASTNode,
    pub body_lowering_policy: BodyLoweringPolicy,
    pub recipe: LoopScanPhiVarsV0Recipe,
    pub segments: Vec<LoopScanPhiSegment>,
}

fn release_enabled() -> bool {
    true
}

fn as_var_name(ast: &ASTNode) -> Option<&str> {
    match ast {
        ASTNode::Variable { name, .. } => Some(name),
        _ => None,
    }
}

fn is_int_lit(ast: &ASTNode, value: i64) -> bool {
    matches!(ast, ASTNode::Literal { value: LiteralValue::Integer(v), .. } if *v == value)
}

fn is_var_plus_one(ast: &ASTNode, var: &str) -> bool {
    matches!(
        ast,
        ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            left,
            right,
            ..
        } if as_var_name(left.as_ref()) == Some(var) && is_int_lit(right.as_ref(), 1)
    )
}

fn is_var_plus_expr(ast: &ASTNode, var: &str) -> bool {
    matches!(
        ast,
        ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            left,
            ..
        } if as_var_name(left.as_ref()) == Some(var)
    )
}

/// Check if condition is `var < var` (Variable < Variable)
fn is_loop_cond_var_lt_var(ast: &ASTNode) -> Option<(String, String)> {
    match ast {
        ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left,
            right,
            ..
        } => Some((as_var_name(left.as_ref())?.to_string(), as_var_name(right.as_ref())?.to_string())),
        _ => None,
    }
}

/// Check if stmt is a local declaration (any form)
fn is_local_decl(stmt: &ASTNode) -> bool {
    matches!(stmt, ASTNode::Local { .. })
}

/// Check if stmt is `local var = 0`
fn is_local_init_zero(stmt: &ASTNode) -> bool {
    match stmt {
        ASTNode::Local { initial_values, .. } => {
            if initial_values.len() != 1 {
                return false;
            }
            match initial_values[0].as_ref() {
                Some(init) => is_int_lit(init, 0),
                None => false,
            }
        }
        _ => false,
    }
}

/// Check if stmt is a loop with break somewhere in its body
fn is_loop_with_break(stmt: &ASTNode) -> bool {
    match stmt {
        ASTNode::Loop { body, .. } => body_contains_break(body),
        _ => false,
    }
}

fn body_contains_break(body: &[ASTNode]) -> bool {
    for stmt in body {
        match stmt {
            ASTNode::Break { .. } => return true,
            ASTNode::If { then_body, else_body, .. } => {
                if body_contains_break(then_body) {
                    return true;
                }
                if let Some(else_body) = else_body {
                    if body_contains_break(else_body) {
                        return true;
                    }
                }
            }
            _ => {}
        }
    }
    false
}

/// Check if stmt is `if cond { ... }` (condition check)
fn is_if_stmt(stmt: &ASTNode) -> bool {
    matches!(stmt, ASTNode::If { .. })
}

/// Check if stmt is `var = var + 1` for the given variable
fn is_inc_stmt(stmt: &ASTNode, loop_var: &str) -> bool {
    match stmt {
        ASTNode::Assignment { target, value, .. } => {
            as_var_name(target.as_ref()) == Some(loop_var) && is_var_plus_one(value.as_ref(), loop_var)
        }
        _ => false,
    }
}

/// Check if stmt is `var = var + <expr>` where step is non-const (+1 is rejected).
fn is_var_step_stmt_nonconst(stmt: &ASTNode, loop_var: &str) -> bool {
    match stmt {
        ASTNode::Assignment { target, value, .. } => {
            as_var_name(target.as_ref()) == Some(loop_var)
                && is_var_plus_expr(value.as_ref(), loop_var)
                && !is_var_plus_one(value.as_ref(), loop_var)
        }
        _ => false,
    }
}

fn contains_exit_anywhere(stmts: &[ASTNode]) -> bool {
    for stmt in stmts {
        match stmt {
            ASTNode::Break { .. }
            | ASTNode::Continue { .. }
            | ASTNode::Return { .. }
            | ASTNode::Throw { .. } => return true,
            ASTNode::If {
                then_body,
                else_body,
                ..
            } => {
                if contains_exit_anywhere(then_body) {
                    return true;
                }
                if else_body.as_ref().is_some_and(|b| contains_exit_anywhere(b)) {
                    return true;
                }
            }
            ASTNode::Loop { body, .. }
            | ASTNode::While { body, .. }
            | ASTNode::ForRange { body, .. }
            | ASTNode::Program {
                statements: body, ..
            }
            | ASTNode::ScopeBox { body, .. } => {
                if contains_exit_anywhere(body) {
                    return true;
                }
            }
            _ => {}
        }
    }
    false
}

fn is_loop_without_exit(stmt: &ASTNode) -> bool {
    match stmt {
        ASTNode::Loop { body, .. } => !contains_exit_anywhere(body),
        _ => false,
    }
}

fn build_nested_loop_recipe(stmt: &ASTNode) -> Option<NestedLoopRecipe> {
    match stmt {
        ASTNode::Loop {
            condition,
            body: inner_body,
            ..
        } => Some(NestedLoopRecipe {
            cond_view: CondBlockView::from_expr(condition),
            loop_stmt: stmt.clone(),
            body: RecipeBody::new(inner_body.to_vec()),
            body_stmt_only: try_build_stmt_only_block_recipe(inner_body),
        }),
        _ => None,
    }
}

fn contains_exit_outside_nested_loops(stmts: &[ASTNode]) -> bool {
    fn walk(stmts: &[ASTNode]) -> bool {
        for stmt in stmts {
            match stmt {
                ASTNode::Break { .. }
                | ASTNode::Continue { .. }
                | ASTNode::Return { .. }
                | ASTNode::Throw { .. } => return true,
                ASTNode::If {
                    then_body,
                    else_body,
                    ..
                } => {
                    if walk(then_body) {
                        return true;
                    }
                    if else_body.as_ref().is_some_and(|b| walk(b)) {
                        return true;
                    }
                }
                ASTNode::Program { statements, .. } => {
                    if walk(statements) {
                        return true;
                    }
                }
                ASTNode::ScopeBox { body, .. } => {
                    if walk(body) {
                        return true;
                    }
                }
                // NOTE: Exits inside nested loops are allowed; do not recurse.
                ASTNode::Loop { .. } | ASTNode::While { .. } | ASTNode::ForRange { .. } => {}
                _ => {}
            }
        }
        false
    }

    walk(stmts)
}

pub(in crate::mir::builder) fn try_extract_loop_scan_phi_vars_v0_facts(
    condition: &ASTNode,
    body: &[ASTNode],
) -> Result<Option<LoopScanPhiVarsV0Facts>, Freeze> {
    let debug = crate::config::env::joinir_dev::debug_enabled();
    let debug_reject = |reason: &str| {
        if debug {
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.debug(&format!("[plan/reject_detail] box=loop_scan_phi_vars_v0 reason={}", reason));
        }
    };

    // Gate: planner_required only
    if !release_enabled() {
        debug_reject("planner_required_off");
        return Ok(None);
    }

    // Condition: i < n (Variable < Variable)
    let Some((loop_var, limit_var)) = is_loop_cond_var_lt_var(condition) else {
        debug_reject("cond_not_var_lt_var");
        return Ok(None);
    };

    let (prefix_end, nested_idx, step_start, recipe) = match body.len() {
        7 => {
            // Stmt 0: local var_name = ... (any local with init)
            if !is_local_decl(&body[0]) {
                debug_reject("stmt0_not_local");
                return Ok(None);
            }

            // Stmt 1: local j = 0
            if !is_local_init_zero(&body[1]) {
                debug_reject("stmt1_not_local_init_zero");
                return Ok(None);
            }

            // Stmt 2: local m = ...
            if !is_local_decl(&body[2]) {
                debug_reject("stmt2_not_local_m");
                return Ok(None);
            }

            // Stmt 3: local found = 0
            if !is_local_init_zero(&body[3]) {
                debug_reject("stmt3_not_local_found_init_zero");
                return Ok(None);
            }

            // Stmt 4: loop(...) with break
            if !is_loop_with_break(&body[4]) {
                debug_reject("stmt4_not_loop_with_break");
                return Ok(None);
            }

            // Stmt 5: if ... { ... }
            if !is_if_stmt(&body[5]) {
                debug_reject("stmt5_not_if");
                return Ok(None);
            }

            // Stmt 6: i = i + 1
            if !is_inc_stmt(&body[6], &loop_var) {
                debug_reject("stmt6_not_inc");
                return Ok(None);
            }

            (
                4usize,
                4usize,
                6usize,
                LoopScanPhiVarsV0Recipe {
                    local_var_name_stmt: Some(body[0].clone()),
                    local_j_stmt: body[1].clone(),
                    local_m_stmt: body[2].clone(),
                    local_found_stmt: Some(body[3].clone()),
                    inner_loop_search: body[4].clone(),
                    found_if_stmt: Some(body[5].clone()),
                    step_inc_stmt: body[6].clone(),
                },
            )
        }
        4 => {
            // EXT-SHAPE-01:
            //   local j = 0
            //   local m = ...
            //   loop(j < m) { ... no exit ... }
            //   i = i + <non-const expr>
            if !is_local_init_zero(&body[0]) {
                debug_reject("stmt0_not_local_j_init_zero_ext_shape01");
                return Ok(None);
            }
            if !is_local_decl(&body[1]) {
                debug_reject("stmt1_not_local_m_ext_shape01");
                return Ok(None);
            }
            if !is_loop_without_exit(&body[2]) {
                debug_reject("stmt2_not_loop_no_exit_ext_shape01");
                return Ok(None);
            }
            if !is_var_step_stmt_nonconst(&body[3], &loop_var) {
                debug_reject("stmt3_not_nonconst_var_step_ext_shape01");
                return Ok(None);
            }

            (
                2usize,
                2usize,
                3usize,
                LoopScanPhiVarsV0Recipe {
                    local_var_name_stmt: None,
                    local_j_stmt: body[0].clone(),
                    local_m_stmt: body[1].clone(),
                    local_found_stmt: None,
                    inner_loop_search: body[2].clone(),
                    found_if_stmt: None,
                    step_inc_stmt: body[3].clone(),
                },
            )
        }
        _ => {
            debug_reject(&format!("body_len={} expected=7_or_4", body.len()));
            return Ok(None);
        }
    };

    // Outer loop body must not contain exits outside nested loops.
    if contains_exit_outside_nested_loops(body) {
        debug_reject("exit_outside_nested_loops");
        return Ok(None);
    }

    const ALLOW_EXTENDED: bool = true;

    let Some(prefix_linear) = try_build_no_exit_block_recipe(&body[..prefix_end], ALLOW_EXTENDED) else {
        debug_reject("segments_prefix_not_no_exit");
        return Ok(None);
    };

    let Some(nested_loop_search) = build_nested_loop_recipe(&body[nested_idx]) else {
        debug_reject("segments_inner_loop_not_loop");
        return Ok(None);
    };

    let Some(step_linear) = try_build_no_exit_block_recipe(&body[step_start..], ALLOW_EXTENDED) else {
        debug_reject("segments_step_not_no_exit");
        return Ok(None);
    };

    Ok(Some(LoopScanPhiVarsV0Facts {
        loop_var,
        limit_var,
        condition: condition.clone(),
        body_lowering_policy: BodyLoweringPolicy::RecipeOnly,
        recipe,
        segments: vec![
            LoopScanPhiSegment::Linear(prefix_linear),
            LoopScanPhiSegment::NestedLoop(nested_loop_search),
            LoopScanPhiSegment::Linear(step_linear),
        ],
    }))
}

#[cfg(test)]
mod tests {
    use super::try_extract_loop_scan_phi_vars_v0_facts;
    use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
    use crate::mir::policies::BodyLoweringPolicy;

    fn var(name: &str) -> ASTNode {
        ASTNode::Variable {
            name: name.to_string(),
            span: Span::unknown(),
        }
    }

    fn int(value: i64) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Integer(value),
            span: Span::unknown(),
        }
    }

    fn bool_lit(value: bool) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Bool(value),
            span: Span::unknown(),
        }
    }

    fn binop(operator: BinaryOperator, left: ASTNode, right: ASTNode) -> ASTNode {
        ASTNode::BinaryOp {
            operator,
            left: Box::new(left),
            right: Box::new(right),
            span: Span::unknown(),
        }
    }

    fn assign(target: ASTNode, value: ASTNode) -> ASTNode {
        ASTNode::Assignment {
            target: Box::new(target),
            value: Box::new(value),
            span: Span::unknown(),
        }
    }

    fn method_call(object: ASTNode, method: &str, arguments: Vec<ASTNode>) -> ASTNode {
        ASTNode::MethodCall {
            object: Box::new(object),
            method: method.to_string(),
            arguments,
            span: Span::unknown(),
        }
    }

    fn local(name: &str, init: Option<ASTNode>) -> ASTNode {
        ASTNode::Local {
            variables: vec![name.to_string()],
            initial_values: vec![init.map(Box::new)],
            span: Span::unknown(),
        }
    }

    #[test]
    fn policy_recipe_only_for_loop_scan_phi_vars_v0() {
        std::env::set_var("NYASH_JOINIR_DEV", "1");
        std::env::set_var("HAKO_JOINIR_PLANNER_REQUIRED", "1");

        let condition = binop(BinaryOperator::Less, var("i"), var("n"));
        let body = vec![
            local("var_name", Some(int(0))),
            local("j", Some(int(0))),
            local("m", Some(int(1))),
            local("found", Some(int(0))),
            ASTNode::Loop {
                condition: Box::new(binop(BinaryOperator::Less, var("j"), var("m"))),
                body: vec![ASTNode::Break {
                    span: Span::unknown(),
                }],
                span: Span::unknown(),
            },
            ASTNode::If {
                condition: Box::new(bool_lit(true)),
                then_body: vec![assign(var("found"), int(1))],
                else_body: None,
                span: Span::unknown(),
            },
            assign(var("i"), binop(BinaryOperator::Add, var("i"), int(1))),
        ];

        let facts = try_extract_loop_scan_phi_vars_v0_facts(&condition, &body)
            .expect("extract ok")
            .expect("facts");

        assert!(matches!(
            facts.body_lowering_policy,
            BodyLoweringPolicy::RecipeOnly
        ));
        assert!(facts.recipe.found_if_stmt.is_some());
    }

    #[test]
    fn accepts_ext_shape01_body_len4_nested_no_exit_nonconst_var_step() {
        std::env::set_var("NYASH_JOINIR_DEV", "1");
        std::env::set_var("HAKO_JOINIR_PLANNER_REQUIRED", "1");

        let condition = binop(BinaryOperator::Less, var("i"), var("n"));
        let body = vec![
            local("j", Some(int(0))),
            local("m", Some(method_call(var("arr"), "length", vec![]))),
            ASTNode::Loop {
                condition: Box::new(binop(BinaryOperator::Less, var("j"), var("m"))),
                body: vec![assign(var("j"), binop(BinaryOperator::Add, var("j"), int(1)))],
                span: Span::unknown(),
            },
            assign(
                var("i"),
                binop(
                    BinaryOperator::Add,
                    var("i"),
                    method_call(var("arr"), "get", vec![int(0)]),
                ),
            ),
        ];

        let facts = try_extract_loop_scan_phi_vars_v0_facts(&condition, &body)
            .expect("extract ok")
            .expect("facts");

        assert!(matches!(
            facts.body_lowering_policy,
            BodyLoweringPolicy::RecipeOnly
        ));
        assert_eq!(facts.segments.len(), 3);
        assert!(facts.recipe.found_if_stmt.is_none());
    }

    #[test]
    fn rejects_ext_shape01_when_nested_loop_contains_exit() {
        std::env::set_var("NYASH_JOINIR_DEV", "1");
        std::env::set_var("HAKO_JOINIR_PLANNER_REQUIRED", "1");

        let condition = binop(BinaryOperator::Less, var("i"), var("n"));
        let body = vec![
            local("j", Some(int(0))),
            local("m", Some(method_call(var("arr"), "length", vec![]))),
            ASTNode::Loop {
                condition: Box::new(binop(BinaryOperator::Less, var("j"), var("m"))),
                body: vec![ASTNode::Break {
                    span: Span::unknown(),
                }],
                span: Span::unknown(),
            },
            assign(
                var("i"),
                binop(
                    BinaryOperator::Add,
                    var("i"),
                    method_call(var("arr"), "get", vec![int(0)]),
                ),
            ),
        ];

        let facts = try_extract_loop_scan_phi_vars_v0_facts(&condition, &body)
            .expect("extract ok");
        assert!(facts.is_none());
    }

    #[test]
    fn rejects_ext_shape01_when_step_is_const_plus_one() {
        std::env::set_var("NYASH_JOINIR_DEV", "1");
        std::env::set_var("HAKO_JOINIR_PLANNER_REQUIRED", "1");

        let condition = binop(BinaryOperator::Less, var("i"), var("n"));
        let body = vec![
            local("j", Some(int(0))),
            local("m", Some(method_call(var("arr"), "length", vec![]))),
            ASTNode::Loop {
                condition: Box::new(binop(BinaryOperator::Less, var("j"), var("m"))),
                body: vec![assign(var("j"), binop(BinaryOperator::Add, var("j"), int(1)))],
                span: Span::unknown(),
            },
            assign(var("i"), binop(BinaryOperator::Add, var("i"), int(1))),
        ];

        let facts = try_extract_loop_scan_phi_vars_v0_facts(&condition, &body)
            .expect("extract ok");
        assert!(facts.is_none());
    }
}
