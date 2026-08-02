//! N0 census for a possible Generic-isolated production bridge.
//!
//! This is evidence only. It uses the same facts/selection/frame boundary as
//! production and never calls a composer, physicalizer, or legacy witness.
//! A singleton row is a pilot candidate, not a cutover authorization.

use super::super::router::{test_issue_live_preflight_frame, LoopRouteContext};
use super::route_id::LoopRouteId;
use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
use crate::mir::builder::control_flow::plan::single_planner::try_build_outcome;

fn variable(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.into(),
        span: Span::unknown(),
    }
}

fn integer(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

fn add(left: ASTNode, right: ASTNode) -> ASTNode {
    ASTNode::BinaryOp {
        operator: BinaryOperator::Add,
        left: Box::new(left),
        right: Box::new(right),
        span: Span::unknown(),
    }
}

fn less(left: ASTNode, right: ASTNode) -> ASTNode {
    ASTNode::BinaryOp {
        operator: BinaryOperator::Less,
        left: Box::new(left),
        right: Box::new(right),
        span: Span::unknown(),
    }
}

fn assignment(name: &str, value: ASTNode) -> ASTNode {
    ASTNode::Assignment {
        target: Box::new(variable(name)),
        value: Box::new(value),
        span: Span::unknown(),
    }
}

fn progression_condition() -> ASTNode {
    less(variable("i"), integer(3))
}

fn progression_step() -> ASTNode {
    assignment("i", add(variable("i"), integer(1)))
}

fn is_generic(route: &LoopRouteId) -> bool {
    matches!(
        route,
        LoopRouteId::GenericLoopV0 | LoopRouteId::GenericLoopV1
    )
}

fn raw_schedule(condition: ASTNode, body: Vec<ASTNode>, name: &str) -> Vec<LoopRouteId> {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    let ctx = LoopRouteContext::new(&condition, &body, name, false, false);
    let outcome = try_build_outcome(&ctx).expect("N0 fixture must build facts");
    let frame = test_issue_live_preflight_frame(&ctx, &outcome, false, false);
    frame.test_raw_schedule().to_vec()
}

fn direct_accum_body() -> Vec<ASTNode> {
    vec![
        assignment("sum", add(variable("sum"), integer(1))),
        progression_step(),
    ]
}

fn nested_body() -> Vec<ASTNode> {
    vec![
        ASTNode::Loop {
            condition: Box::new(less(variable("j"), integer(3))),
            body: vec![progression_step()],
            span: Span::unknown(),
        },
        progression_step(),
    ]
}

#[test]
fn n0_census_keeps_only_a_known_accum_singleton_candidate() {
    let accum = raw_schedule(
        progression_condition(),
        direct_accum_body(),
        "scoped_nongeneric/accum",
    );
    assert_eq!(accum, vec![LoopRouteId::AccumConstLoop]);
    assert!(!accum.iter().any(is_generic));

    let simple_while = raw_schedule(
        progression_condition(),
        vec![progression_step()],
        "scoped_nongeneric/simple-while",
    );
    assert_eq!(
        simple_while,
        vec![LoopRouteId::LoopSimpleWhile, LoopRouteId::GenericLoopV0]
    );
    assert!(simple_while.iter().any(is_generic));

    let overlap = raw_schedule(
        progression_condition(),
        nested_body(),
        "scoped_nongeneric/overlap",
    );
    assert_eq!(
        overlap,
        vec![LoopRouteId::GenericLoopV0, LoopRouteId::GenericLoopV1]
    );
    assert!(overlap.len() > 1);
}
