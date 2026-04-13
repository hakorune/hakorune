//! Phase 29aj P2: loop_simple_while facts (SSOT)

use crate::ast::{ASTNode, BinaryOperator, LiteralValue};
use crate::mir::builder::control_flow::plan::extractors::common_helpers::{
    extract_loop_increment_plan, has_break_statement, has_continue_statement,
    has_if_else_statement, has_return_statement,
};
use crate::mir::builder::control_flow::plan::facts::feature_facts::detect_nested_loop;
use crate::mir::builder::control_flow::plan::planner::Freeze;
use crate::mir::builder::control_flow::plan::policies::loop_simple_while_subset_policy::is_loop_simple_while_step_only_body;

#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct LoopSimpleWhileFacts {
    pub loop_var: String,
    pub condition: ASTNode,
    pub loop_increment: ASTNode,
}

pub(in crate::mir::builder) fn try_extract_loop_simple_while_facts(
    condition: &ASTNode,
    body: &[ASTNode],
) -> Result<Option<LoopSimpleWhileFacts>, Freeze> {
    let Some(loop_var) = extract_loop_var_for_subset(condition) else {
        return Ok(None);
    };

    if has_break_statement(body) || has_continue_statement(body) || has_return_statement(body) {
        return Ok(None);
    }

    if has_if_else_statement(body) {
        return Ok(None);
    }

    // loop_simple_while recipe rebuilds the body from the increment stmt only.
    // Nested loops must stay on nested/generic routes so inner control flow is preserved.
    if detect_nested_loop(body) {
        return Ok(None);
    }

    let loop_increment = match extract_loop_increment_plan(body, &loop_var) {
        Ok(Some(inc)) => inc,
        _ => return Ok(None),
    };

    if !is_loop_simple_while_step_only_body(body, &loop_var) {
        return Ok(None);
    }

    if !is_increment_step_one(&loop_increment, &loop_var) {
        return Ok(None);
    }

    Ok(Some(LoopSimpleWhileFacts {
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

#[cfg(test)]
mod tests {
    use super::try_extract_loop_simple_while_facts;
    use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};

    fn lit_int(value: i64) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Integer(value),
            span: Span::unknown(),
        }
    }

    fn var(name: &str) -> ASTNode {
        ASTNode::Variable {
            name: name.to_string(),
            span: Span::unknown(),
        }
    }

    fn cond_lt(name: &str, rhs: i64) -> ASTNode {
        ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(var(name)),
            right: Box::new(lit_int(rhs)),
            span: Span::unknown(),
        }
    }

    fn inc_stmt(name: &str) -> ASTNode {
        ASTNode::Assignment {
            target: Box::new(var(name)),
            value: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Add,
                left: Box::new(var(name)),
                right: Box::new(lit_int(1)),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }
    }

    #[test]
    fn loop_simple_while_facts_reject_nested_loop_even_when_step_exists() {
        let condition = cond_lt("i", 3);
        let body = vec![
            ASTNode::Loop {
                condition: Box::new(cond_lt("j", 2)),
                body: vec![ASTNode::Return {
                    value: Some(Box::new(lit_int(0))),
                    span: Span::unknown(),
                }],
                span: Span::unknown(),
            },
            inc_stmt("i"),
        ];

        let facts = try_extract_loop_simple_while_facts(&condition, &body).expect("ok");
        assert!(facts.is_none());
    }
}
