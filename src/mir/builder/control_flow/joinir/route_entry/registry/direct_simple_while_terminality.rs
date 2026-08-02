//! Pre-effect terminality proof for the direct SimpleWhile route.
//!
//! This proves legacy scheduler terminality only: after the route's nested
//! pre-gate, it may return `Some` or `Err`, but not `Ok(None)`.

use crate::mir::builder::control_flow::joinir::route_entry::registry::route_id::LoopRouteId;
use crate::mir::builder::control_flow::plan::facts::LoopFacts;

/// Opaque proof that one direct source shape bypasses SimpleWhile's only None gate.
#[derive(Debug)]
pub(crate) struct DirectSimpleWhileTerminalityV1 {
    route: LoopRouteId,
}

impl DirectSimpleWhileTerminalityV1 {
    pub(crate) fn route(&self) -> LoopRouteId {
        self.route
    }
}

/// Issues no product and runs no selection. It only certifies the source/facts
/// preconditions that make the legacy nested pre-gate unreachable.
pub(crate) fn certify_direct_simple_while_terminality(
    facts: &LoopFacts,
) -> Option<DirectSimpleWhileTerminalityV1> {
    let simple_while = facts.loop_simple_while()?;
    let topology = simple_while.source_topology.as_ref()?;
    let step = topology.step();
    let direct_single_body = facts.source_receipt().raw_body_statement_count() == Some(1)
        && step.raw_body_index() == 0
        && step.scope_box_children().is_empty();

    if !direct_single_body || facts.features.nested_loop {
        return None;
    }

    Some(DirectSimpleWhileTerminalityV1 {
        route: LoopRouteId::LoopSimpleWhile,
    })
}

#[cfg(test)]
mod tests {
    use super::certify_direct_simple_while_terminality;
    use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
    use crate::mir::builder::control_flow::joinir::route_entry::registry::route_id::LoopRouteId;
    use crate::mir::builder::control_flow::plan::facts::try_build_loop_facts;

    fn variable(name: &str) -> ASTNode {
        ASTNode::Variable {
            name: name.to_string(),
            span: Span::unknown(),
        }
    }

    fn integer(value: i64) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Integer(value),
            span: Span::unknown(),
        }
    }

    fn condition() -> ASTNode {
        ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(variable("i")),
            right: Box::new(integer(3)),
            span: Span::unknown(),
        }
    }

    fn increment() -> ASTNode {
        ASTNode::Assignment {
            target: Box::new(variable("i")),
            value: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Add,
                left: Box::new(variable("i")),
                right: Box::new(integer(1)),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }
    }

    #[test]
    fn direct_single_statement_simple_while_is_terminal() {
        let condition = condition();
        let body = vec![increment()];
        let facts = try_build_loop_facts(&condition, &body)
            .expect("no freeze")
            .expect("facts");

        assert_eq!(
            certify_direct_simple_while_terminality(&facts)
                .expect("direct terminality")
                .route(),
            LoopRouteId::LoopSimpleWhile
        );
    }

    #[test]
    fn scope_box_simple_while_is_not_direct_terminality() {
        let condition = condition();
        let body = vec![ASTNode::ScopeBox {
            body: vec![increment()],
            span: Span::unknown(),
        }];
        let facts = try_build_loop_facts(&condition, &body)
            .expect("no freeze")
            .expect("facts");

        assert!(certify_direct_simple_while_terminality(&facts).is_none());
    }
}
