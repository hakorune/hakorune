//! Phase 29aj P2: loop_simple_while facts (SSOT)

use crate::ast::{ASTNode, BinaryOperator, LiteralValue};
use crate::mir::builder::control_flow::cleanup::policies::loop_simple_while_subset_policy::is_loop_simple_while_step_only_body;
use crate::mir::builder::control_flow::facts::extractors::common_helpers::{
    extract_loop_increment_plan, has_break_statement, has_continue_statement,
    has_if_else_statement, has_return_statement,
};
use crate::mir::builder::control_flow::facts::stmt_view::{
    LoopSourceBodySiteV1, LoopSourceProjectionV1,
};
use crate::mir::builder::control_flow::plan::facts::feature_facts::detect_nested_loop;
use crate::mir::builder::control_flow::plan::planner::Freeze;

#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct LoopSimpleWhileFacts {
    pub loop_var: String,
    pub condition: ASTNode,
    pub loop_increment: ASTNode,
    pub source_topology: Option<LoopSimpleWhileSourceTopologyV1>,
}

/// Route-local provenance for the simple, one-statement loop body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) struct LoopSimpleWhileSourceTopologyV1 {
    step: LoopSourceBodySiteV1,
}

impl LoopSimpleWhileSourceTopologyV1 {
    pub(in crate::mir::builder) fn step(&self) -> &LoopSourceBodySiteV1 {
        &self.step
    }
}

pub(in crate::mir::builder) fn try_extract_loop_simple_while_facts(
    condition: &ASTNode,
    body: &[ASTNode],
) -> Result<Option<LoopSimpleWhileFacts>, Freeze> {
    try_extract_loop_simple_while_facts_with_projection(
        condition,
        body,
        &LoopSourceProjectionV1::default(),
    )
}

pub(in crate::mir::builder) fn try_extract_loop_simple_while_facts_with_projection(
    condition: &ASTNode,
    body: &[ASTNode],
    source_projection: &LoopSourceProjectionV1,
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
        source_topology: source_projection
            .site_for_flattened_index(0)
            .filter(|_| source_projection.flattened_body_len() == Some(body.len()))
            .cloned()
            .map(|step| LoopSimpleWhileSourceTopologyV1 { step }),
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
    use super::{
        try_extract_loop_simple_while_facts, try_extract_loop_simple_while_facts_with_projection,
    };
    use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
    use crate::mir::builder::control_flow::facts::stmt_view::flatten_scope_boxes_with_projection;
    use crate::mir::builder::control_flow::plan::facts::try_build_loop_facts;

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

    #[test]
    fn simple_while_topology_keeps_nested_scope_box_step_coordinate() {
        let condition = cond_lt("i", 3);
        let raw_body = vec![ASTNode::ScopeBox {
            body: vec![ASTNode::ScopeBox {
                body: vec![inc_stmt("i")],
                span: Span::unknown(),
            }],
            span: Span::unknown(),
        }];
        let (body, projection) = flatten_scope_boxes_with_projection(&raw_body).into_parts();

        let facts =
            try_extract_loop_simple_while_facts_with_projection(&condition, &body, &projection)
                .expect("no freeze")
                .expect("simple while facts");

        let topology = facts.source_topology.expect("aligned topology");
        assert_eq!(topology.step().raw_body_index(), 0);
        assert_eq!(topology.step().scope_box_children(), &[0, 0]);
    }

    #[test]
    fn loop_facts_builder_carries_simple_while_topology() {
        let condition = cond_lt("i", 3);
        let raw_body = vec![ASTNode::ScopeBox {
            body: vec![inc_stmt("i")],
            span: Span::unknown(),
        }];

        let facts = try_build_loop_facts(&condition, &raw_body)
            .expect("no freeze")
            .expect("loop facts");
        let topology = facts
            .loop_simple_while()
            .expect("simple while facts")
            .source_topology
            .as_ref()
            .expect("source topology");

        assert_eq!(topology.step().raw_body_index(), 0);
        assert_eq!(topology.step().scope_box_children(), &[0]);
    }
}
