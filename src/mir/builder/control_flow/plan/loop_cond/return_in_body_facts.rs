//! Phase 29bq P2.x: LoopCondReturnInBodyFacts (Facts SSOT)
//!
//! Fixture-derived 1-shape for loop(cond) with nested return and no break/continue.

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::facts::expr_bool::is_supported_bool_expr_with_canon;
use super::return_in_body_recipe::{build_loop_cond_return_in_body_recipe, LoopCondReturnInBodyRecipe};
use crate::mir::builder::control_flow::plan::planner::Freeze;

#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct LoopCondReturnInBodyFacts {
    pub condition: ASTNode,
    pub recipe: LoopCondReturnInBodyRecipe,
}

pub(in crate::mir::builder) fn try_extract_loop_cond_return_in_body_facts(
    condition: &ASTNode,
    body: &[ASTNode],
) -> Result<Option<LoopCondReturnInBodyFacts>, Freeze> {
    // LoopBuilder has been removed; return-in-body loop(cond) must remain available
    // in all modes (not only strict/dev), otherwise common index_of-style loops freeze.
    let debug = super::loop_cond_unified_helpers::debug_enabled();

    if !super::loop_cond_unified_helpers::validate_loop_condition(condition) {
        return Ok(None);
    }

    // Independent observation: continue==0 && break==0 && return>0
    let counts = super::loop_cond_unified_helpers::count_control_flow_with_returns(body);
    if counts.continue_count > 0 {
        return Ok(None);
    }
    if counts.break_count > 0 {
        return Ok(None);
    }
    if counts.return_count == 0 {
        return Ok(None);
    }
    if counts.has_nested_loop {
        return Ok(None);
    }

    let matched = if matches_seek_array_end_shape(body)? {
        "seek_array_end"
    } else if matches_if_else_all_return_shape(body)? {
        "if_else_all_return"
    } else if matches_if_else_if_return_shape(body)? {
        "if_else_if_return"
    } else if matches_brace_balance_shape(body)? {
        "brace_balance"
    } else if matches_simple_if_return_then_step_shape(body)? {
        "simple_if_return_then_step"
    } else if matches_balanced_depth_scan_shape(condition, body)? {
        "balanced_depth_scan"
    } else if matches_scan_with_quote_full_shape(body)? {
        "scan_with_quote_full"
    } else if matches_scan_with_quote_shape(body)? {
        "scan_with_quote"
    } else {
        return Ok(None);
    };

    if debug {
        let ring0 = crate::runtime::get_global_ring0();
        ring0.log.debug(&format!(
            "[loop_cond_return_in_body] MATCHED: return-in-body (fixture-derived shape={})",
            matched
        ));
    }

    let recipe = build_loop_cond_return_in_body_recipe(body.to_vec());

    Ok(Some(LoopCondReturnInBodyFacts {
        condition: condition.clone(),
        recipe,
    }))
}

fn matches_simple_if_return_then_step_shape(body: &[ASTNode]) -> Result<bool, Freeze> {
    if body.len() != 2 {
        return Ok(false);
    }
    if !is_if_with_return(&body[0])? {
        return Ok(false);
    }
    Ok(matches!(body[1], ASTNode::Assignment { .. }))
}

fn matches_if_else_all_return_shape(body: &[ASTNode]) -> Result<bool, Freeze> {
    if body.len() != 1 {
        return Ok(false);
    }
    let ASTNode::If {
        condition,
        then_body,
        else_body,
        ..
    } = &body[0]
    else {
        return Ok(false);
    };
    if !is_supported_bool_expr_with_canon(condition, true) {
        return Ok(false);
    }
    let Some(else_body) = else_body else {
        return Ok(false);
    };
    Ok(block_guarantees_return(then_body)? && block_guarantees_return(else_body)?)
}

fn matches_if_else_if_return_shape(body: &[ASTNode]) -> Result<bool, Freeze> {
    if body.len() != 1 {
        return Ok(false);
    }
    let ASTNode::If {
        condition,
        then_body,
        else_body,
        ..
    } = &body[0]
    else {
        return Ok(false);
    };
    if !is_supported_bool_expr_with_canon(condition, true) {
        return Ok(false);
    }
    if !block_guarantees_return(then_body)? {
        return Ok(false);
    }
    let Some(else_body) = else_body else {
        return Ok(false);
    };
    if else_body.len() != 1 {
        return Ok(false);
    }
    let ASTNode::If {
        condition: nested_cond,
        then_body: nested_then,
        else_body: nested_else,
        ..
    } = &else_body[0]
    else {
        return Ok(false);
    };
    if !is_supported_bool_expr_with_canon(nested_cond, true) {
        return Ok(false);
    }
    if nested_else.is_some() {
        return Ok(false);
    }
    block_guarantees_return(nested_then)
}

fn block_guarantees_return(stmts: &[ASTNode]) -> Result<bool, Freeze> {
    for stmt in stmts {
        match stmt {
            ASTNode::Return { .. } => return Ok(true),
            ASTNode::If {
                condition,
                then_body,
                else_body,
                ..
            } => {
                if !is_supported_bool_expr_with_canon(condition, true) {
                    return Ok(false);
                }
                let Some(else_body) = else_body else {
                    continue;
                };
                if block_guarantees_return(then_body)?
                    && block_guarantees_return(else_body)?
                {
                    return Ok(true);
                }
            }
            _ => {}
        }
    }
    Ok(false)
}

fn matches_balanced_depth_scan_shape(condition: &ASTNode, body: &[ASTNode]) -> Result<bool, Freeze> {
    use crate::mir::policies::balanced_depth_scan;
    use crate::mir::policies::PolicyDecision;

    if body.len() < 3 {
        return Ok(false);
    }

    match balanced_depth_scan::decide(condition, body) {
        PolicyDecision::Use(_) => Ok(true),
        PolicyDecision::Reject(reason) => {
            if crate::config::env::joinir_dev::strict_enabled() {
                return Err(Freeze::contract(reason));
            }
            Ok(false)
        }
        PolicyDecision::None => Ok(false),
    }
}

fn matches_seek_array_end_shape(body: &[ASTNode]) -> Result<bool, Freeze> {
    if body.len() != 3 {
        return Ok(false);
    }
    if !matches!(body[0], ASTNode::Local { .. }) {
        return Ok(false);
    }
    if !matches!(body[2], ASTNode::Assignment { .. }) {
        return Ok(false);
    }

    let ASTNode::If {
        condition,
        then_body,
        else_body,
        ..
    } = &body[1] else {
        return Ok(false);
    };

    if !is_supported_bool_expr_with_canon(condition, true) {
        return Ok(false);
    }
    let Some(else_body) = else_body else {
        return Ok(false);
    };
    if then_body.len() != 2 {
        return Ok(false);
    }
    if else_body.len() != 3 {
        return Ok(false);
    }

    for stmt in then_body {
        if !is_simple_if_with_assignment(stmt)? {
            return Ok(false);
        }
    }
    if !is_simple_if_with_assignment(&else_body[0])? {
        return Ok(false);
    }
    if !is_simple_if_with_assignment(&else_body[1])? {
        return Ok(false);
    }
    if !is_if_with_assignment_and_return(&else_body[2])? {
        return Ok(false);
    }

    Ok(true)
}

fn matches_brace_balance_shape(body: &[ASTNode]) -> Result<bool, Freeze> {
    if body.len() != 3 {
        return Ok(false);
    }
    if !matches!(body[0], ASTNode::Local { .. }) {
        return Ok(false);
    }
    if !matches!(body[2], ASTNode::Assignment { .. }) {
        return Ok(false);
    }

    let ASTNode::If {
        condition,
        then_body,
        else_body,
        ..
    } = &body[1]
    else {
        return Ok(false);
    };

    if !is_supported_bool_expr_with_canon(condition, true) {
        return Ok(false);
    }
    let Some(else_body) = else_body else {
        return Ok(false);
    };
    if then_body.len() != 1 {
        return Ok(false);
    }
    if !matches!(then_body[0], ASTNode::Assignment { .. }) {
        return Ok(false);
    }
    if else_body.len() != 1 {
        return Ok(false);
    }
    if !is_if_with_assignment_and_return(&else_body[0])? {
        return Ok(false);
    }

    Ok(true)
}

fn matches_scan_with_quote_shape(body: &[ASTNode]) -> Result<bool, Freeze> {
    if body.len() != 10 {
        return Ok(false);
    }

    if !is_if_with_return(&body[0])? {
        return Ok(false);
    }
    if !matches!(body[1], ASTNode::Assignment { .. }) {
        return Ok(false);
    }
    if !matches!(body[2], ASTNode::Local { .. }) {
        return Ok(false);
    }
    if !matches!(body[3], ASTNode::Local { .. }) {
        return Ok(false);
    }
    if !matches!(body[4], ASTNode::Local { .. }) {
        return Ok(false);
    }
    if !is_if_with_two_assignments(&body[5])? {
        return Ok(false);
    }
    if !is_if_with_return(&body[6])? {
        return Ok(false);
    }
    if !is_if_with_escape_scan(&body[7])? {
        return Ok(false);
    }
    if !matches!(body[8], ASTNode::Assignment { .. }) {
        return Ok(false);
    }
    if !matches!(body[9], ASTNode::Assignment { .. }) {
        return Ok(false);
    }

    Ok(true)
}

fn matches_scan_with_quote_full_shape(body: &[ASTNode]) -> Result<bool, Freeze> {
    matches_scan_with_quote_shape(body)
}

fn is_simple_if_with_assignment(stmt: &ASTNode) -> Result<bool, Freeze> {
    let ASTNode::If {
        condition,
        then_body,
        else_body,
        ..
    } = stmt else {
        return Ok(false);
    };
    if !is_supported_bool_expr_with_canon(condition, true) {
        return Ok(false);
    }
    if else_body.is_some() {
        return Ok(false);
    }
    if then_body.len() != 1 {
        return Ok(false);
    }
    Ok(matches!(then_body[0], ASTNode::Assignment { .. }))
}

fn is_if_with_return(stmt: &ASTNode) -> Result<bool, Freeze> {
    let ASTNode::If {
        condition,
        then_body,
        else_body,
        ..
    } = stmt
    else {
        return Ok(false);
    };
    if !is_supported_bool_expr_with_canon(condition, true) {
        return Ok(false);
    }
    if else_body.is_some() {
        return Ok(false);
    }
    if then_body.len() != 1 {
        return Ok(false);
    }
    Ok(matches!(then_body[0], ASTNode::Return { .. }))
}

fn is_if_with_two_assignments(stmt: &ASTNode) -> Result<bool, Freeze> {
    let ASTNode::If {
        condition,
        then_body,
        else_body,
        ..
    } = stmt
    else {
        return Ok(false);
    };
    if !is_supported_bool_expr_with_canon(condition, true) {
        return Ok(false);
    }
    if else_body.is_some() {
        return Ok(false);
    }
    if then_body.len() != 2 {
        return Ok(false);
    }
    Ok(matches!(then_body[0], ASTNode::Assignment { .. })
        && matches!(then_body[1], ASTNode::Assignment { .. }))
}

fn is_if_with_escape_scan(stmt: &ASTNode) -> Result<bool, Freeze> {
    let ASTNode::If {
        condition,
        then_body,
        else_body,
        ..
    } = stmt
    else {
        return Ok(false);
    };
    if !is_supported_bool_expr_with_canon(condition, true) {
        return Ok(false);
    }
    if else_body.is_some() {
        return Ok(false);
    }
    if then_body.len() != 2 {
        return Ok(false);
    }
    if !matches!(then_body[0], ASTNode::Assignment { .. }) {
        return Ok(false);
    }

    let ASTNode::If {
        condition: nested_cond,
        then_body: nested_body,
        else_body: nested_else,
        ..
    } = &then_body[1]
    else {
        return Ok(false);
    };
    if !is_supported_bool_expr_with_canon(nested_cond, true) {
        return Ok(false);
    }
    if nested_else.is_some() {
        return Ok(false);
    }
    if nested_body.len() != 4 {
        return Ok(false);
    }
    if !matches!(nested_body[0], ASTNode::Local { .. }) {
        return Ok(false);
    }
    if !matches!(nested_body[1], ASTNode::Local { .. }) {
        return Ok(false);
    }
    if !matches!(nested_body[2], ASTNode::Assignment { .. }) {
        return Ok(false);
    }
    Ok(matches!(nested_body[3], ASTNode::Assignment { .. }))
}

fn is_if_with_assignment_and_return(stmt: &ASTNode) -> Result<bool, Freeze> {
    let ASTNode::If {
        condition,
        then_body,
        else_body,
        ..
    } = stmt else {
        return Ok(false);
    };
    if !is_supported_bool_expr_with_canon(condition, true) {
        return Ok(false);
    }
    if else_body.is_some() {
        return Ok(false);
    }
    if then_body.len() != 2 {
        return Ok(false);
    }
    if !matches!(then_body[0], ASTNode::Assignment { .. }) {
        return Ok(false);
    }
    let ASTNode::If {
        condition,
        then_body,
        else_body,
        ..
    } = &then_body[1] else {
        return Ok(false);
    };
    if !is_supported_bool_expr_with_canon(condition, true) {
        return Ok(false);
    }
    if else_body.is_some() {
        return Ok(false);
    }
    if then_body.len() != 1 {
        return Ok(false);
    }
    Ok(matches!(then_body[0], ASTNode::Return { .. }))
}

#[cfg(test)]
mod tests {
    use super::try_extract_loop_cond_return_in_body_facts;
    use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};

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

    fn string(value: &str) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::String(value.to_string()),
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

    fn local(name: &str, init: ASTNode) -> ASTNode {
        ASTNode::Local {
            variables: vec![name.to_string()],
            initial_values: vec![Some(Box::new(init))],
            span: Span::unknown(),
        }
    }

    #[test]
    fn return_in_body_brace_balance_shape_matches() {
        std::env::set_var("NYASH_JOINIR_DEV", "1");

        let condition = binop(BinaryOperator::Less, var("i"), var("n"));
        let inner_return = ASTNode::If {
            condition: Box::new(binop(BinaryOperator::Equal, var("depth"), int(0))),
            then_body: vec![ASTNode::Return {
                value: Some(Box::new(int(0))),
                span: Span::unknown(),
            }],
            else_body: None,
            span: Span::unknown(),
        };
        let else_if = ASTNode::If {
            condition: Box::new(binop(BinaryOperator::Equal, var("ch"), string("}"))),
            then_body: vec![assign(var("depth"), int(0)), inner_return],
            else_body: None,
            span: Span::unknown(),
        };
        let body = vec![
            local("depth", int(0)),
            ASTNode::If {
                condition: Box::new(binop(BinaryOperator::Equal, var("ch"), string("{"))),
                then_body: vec![assign(var("depth"), int(1))],
                else_body: Some(vec![else_if]),
                span: Span::unknown(),
            },
            assign(var("i"), binop(BinaryOperator::Add, var("i"), int(1))),
        ];

        let facts = try_extract_loop_cond_return_in_body_facts(&condition, &body)
            .expect("extract ok")
            .expect("facts");

        assert_eq!(facts.recipe.body.body.len(), 3);
    }

    #[test]
    fn return_in_body_rejects_continue() {
        std::env::set_var("NYASH_JOINIR_DEV", "1");

        let condition = binop(BinaryOperator::Less, var("i"), var("n"));
        let body = vec![
            local("depth", int(0)),
            ASTNode::If {
                condition: Box::new(binop(BinaryOperator::Equal, var("ch"), string("{"))),
                then_body: vec![assign(var("depth"), int(1))],
                else_body: Some(vec![ASTNode::Continue {
                    span: Span::unknown(),
                }]),
                span: Span::unknown(),
            },
            assign(var("i"), binop(BinaryOperator::Add, var("i"), int(1))),
        ];

        let facts = try_extract_loop_cond_return_in_body_facts(&condition, &body)
            .expect("extract ok");
        assert!(facts.is_none());
    }

    #[test]
    fn return_in_body_rejects_nested_loop() {
        std::env::set_var("NYASH_JOINIR_DEV", "1");

        let condition = binop(BinaryOperator::Less, var("i"), var("n"));
        let body = vec![
            local("depth", int(0)),
            ASTNode::Loop {
                condition: Box::new(binop(BinaryOperator::Less, var("j"), var("n"))),
                body: vec![ASTNode::Return {
                    value: Some(Box::new(int(0))),
                    span: Span::unknown(),
                }],
                span: Span::unknown(),
            },
            assign(var("i"), binop(BinaryOperator::Add, var("i"), int(1))),
        ];

        let facts = try_extract_loop_cond_return_in_body_facts(&condition, &body)
            .expect("extract ok");
        assert!(facts.is_none());
    }

    #[test]
    fn simple_if_return_then_step_shape_detects() {
        let body = vec![
            ASTNode::If {
                condition: Box::new(var("found")),
                then_body: vec![ASTNode::Return {
                    value: Some(Box::new(var("j"))),
                    span: Span::unknown(),
                }],
                else_body: None,
                span: Span::unknown(),
            },
            assign(var("j"), binop(BinaryOperator::Add, var("j"), int(1))),
        ];

        assert!(super::matches_simple_if_return_then_step_shape(&body).expect("shape check"));
    }

    #[test]
    fn return_in_body_simple_if_return_then_step_matches_in_dev_mode() {
        std::env::set_var("NYASH_JOINIR_DEV", "1");

        let condition = binop(BinaryOperator::LessEqual, binop(BinaryOperator::Add, var("j"), var("m")), var("n"));
        let body = vec![
            ASTNode::If {
                condition: Box::new(var("found")),
                then_body: vec![ASTNode::Return {
                    value: Some(Box::new(var("j"))),
                    span: Span::unknown(),
                }],
                else_body: None,
                span: Span::unknown(),
            },
            assign(var("j"), binop(BinaryOperator::Add, var("j"), int(1))),
        ];

        let facts = try_extract_loop_cond_return_in_body_facts(&condition, &body)
            .expect("extract ok")
            .expect("facts");
        assert_eq!(facts.recipe.body.body.len(), 2);
    }

    #[test]
    fn return_in_body_simple_if_return_then_step_with_method_call_condition_matches() {
        std::env::set_var("NYASH_JOINIR_DEV", "1");

        let condition = binop(
            BinaryOperator::LessEqual,
            binop(BinaryOperator::Add, var("j"), var("m")),
            var("n"),
        );
        let body = vec![
            ASTNode::If {
                condition: Box::new(ASTNode::MethodCall {
                    object: Box::new(var("me")),
                    method: "starts_with".to_string(),
                    arguments: vec![var("src"), var("j"), var("pat")],
                    span: Span::unknown(),
                }),
                then_body: vec![ASTNode::Return {
                    value: Some(Box::new(var("j"))),
                    span: Span::unknown(),
                }],
                else_body: None,
                span: Span::unknown(),
            },
            assign(var("j"), binop(BinaryOperator::Add, var("j"), int(1))),
        ];

        let facts = try_extract_loop_cond_return_in_body_facts(&condition, &body)
            .expect("extract ok")
            .expect("facts");
        assert_eq!(facts.recipe.body.body.len(), 2);
    }

    #[test]
    fn return_in_body_if_else_if_return_shape_matches() {
        std::env::set_var("NYASH_JOINIR_DEV", "1");

        let condition = binop(BinaryOperator::Less, var("v"), int(1));
        let body = vec![ASTNode::If {
            condition: Box::new(binop(BinaryOperator::Equal, var("v"), int(0))),
            then_body: vec![ASTNode::Return {
                value: Some(Box::new(int(0))),
                span: Span::unknown(),
            }],
            else_body: Some(vec![ASTNode::If {
                condition: Box::new(binop(BinaryOperator::Equal, var("v"), int(1))),
                then_body: vec![ASTNode::Return {
                    value: Some(Box::new(int(1))),
                    span: Span::unknown(),
                }],
                else_body: None,
                span: Span::unknown(),
            }]),
            span: Span::unknown(),
        }];

        let facts = try_extract_loop_cond_return_in_body_facts(&condition, &body)
            .expect("extract ok")
            .expect("facts");
        assert_eq!(facts.recipe.body.body.len(), 1);
    }
}
