//! Phase 29aj P2: loop_simple_while facts (legacy type: Pattern1SimpleWhileFacts, SSOT)

use crate::ast::{ASTNode, BinaryOperator, LiteralValue};
use crate::mir::builder::control_flow::plan::planner::Freeze;
use crate::mir::builder::control_flow::plan::extractors::common_helpers::{
    extract_loop_increment_plan, has_break_statement, has_continue_statement, has_if_else_statement,
    has_return_statement,
};
use crate::mir::builder::control_flow::plan::policies::pattern1_subset_policy::is_pattern1_step_only_body;

#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct Pattern1SimpleWhileFacts {
    pub loop_var: String,
    pub condition: ASTNode,
    pub loop_increment: ASTNode,
}

pub(in crate::mir::builder) fn try_extract_pattern1_simplewhile_facts(
    condition: &ASTNode,
    body: &[ASTNode],
) -> Result<Option<Pattern1SimpleWhileFacts>, Freeze> {
    let Some(loop_var) = extract_loop_var_for_subset(condition) else {
        return Ok(None);
    };

    if has_break_statement(body) || has_continue_statement(body) || has_return_statement(body) {
        return Ok(None);
    }

    if has_if_else_statement(body) {
        return Ok(None);
    }

    let loop_increment = match extract_loop_increment_plan(body, &loop_var) {
        Ok(Some(inc)) => inc,
        _ => return Ok(None),
    };

    if !is_pattern1_step_only_body(body, &loop_var) {
        return Ok(None);
    }

    if !is_increment_step_one(&loop_increment, &loop_var) {
        return Ok(None);
    }

    Ok(Some(Pattern1SimpleWhileFacts {
        loop_var,
        condition: condition.clone(),
        loop_increment,
    }))
}

fn extract_loop_var_for_subset(condition: &ASTNode) -> Option<String> {
    let ASTNode::BinaryOp {
        operator: BinaryOperator::Less,
        left,
        right,
        ..
    } = condition
    else {
        return None;
    };

    let ASTNode::Variable { name, .. } = left.as_ref() else {
        return None;
    };

    if !matches!(
        right.as_ref(),
        ASTNode::Literal {
            value: LiteralValue::Integer(_),
            ..
        }
    ) {
        return None;
    }

    Some(name.clone())
}

fn is_increment_step_one(loop_increment: &ASTNode, loop_var: &str) -> bool {
    let ASTNode::BinaryOp {
        operator: BinaryOperator::Add,
        left,
        right,
        ..
    } = loop_increment
    else {
        return false;
    };

    if !matches!(left.as_ref(), ASTNode::Variable { name, .. } if name == loop_var) {
        return false;
    }

    matches!(
        right.as_ref(),
        ASTNode::Literal {
            value: LiteralValue::Integer(1),
            ..
        }
    )
}
