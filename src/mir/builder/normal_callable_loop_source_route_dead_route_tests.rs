use super::{CallableLoopRouteMatchV1, CallableLoopSoleFamilyV1};
use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
use crate::mir::builder::control_flow::plan::single_planner::{
    self, CallableLoopFactsPlannerInputV1,
};
use crate::mir::builder::control_flow::plan::LoopFactsPolicyFrameV1;
use crate::mir::loop_recipe_contract::route_id::LoopRouteId;

fn route_match_from(matched: &[LoopRouteId]) -> CallableLoopRouteMatchV1 {
    CallableLoopRouteMatchV1 {
        matched: matched.to_vec().into_boxed_slice(),
    }
}

/// `LoopBreakRecipe` is owner-less (`TypedDeclined` wire coverage): a
/// matched row is provenance only and must not contend for the sole family.
#[test]
fn sole_family_ignores_owner_less_loop_break_recipe() {
    let selection = route_match_from(&[
        LoopRouteId::LoopBreakRecipe,
        LoopRouteId::LoopCondBreakContinue,
    ]);
    assert_eq!(
        selection.sole_family(),
        Some(CallableLoopSoleFamilyV1::LoopCondBreakContinue)
    );
    let selection = route_match_from(&[
        LoopRouteId::LoopBreakRecipe,
        LoopRouteId::LoopTrueBreakContinue,
    ]);
    assert_eq!(
        selection.sole_family(),
        Some(CallableLoopSoleFamilyV1::LoopTrueBreakContinue)
    );
    let selection = route_match_from(&[LoopRouteId::LoopBreakRecipe]);
    assert_eq!(selection.sole_family(), None);
}

/// `LoopSimpleWhile` keeps live wire coverage (`V1Parity`): its overlap
/// with LoopCond still declines front-selection (the no-exit S0 recorded
/// boundary — `loop(i<literal){i=i+1}` stays on the node route).
#[test]
fn loop_simple_while_overlap_keeps_sole_family_none() {
    let selection = route_match_from(&[
        LoopRouteId::LoopSimpleWhile,
        LoopRouteId::LoopCondBreakContinue,
    ]);
    assert_eq!(selection.sole_family(), None);
}

/// The trim-header `loop_break` subset facts must not suppress the
/// designed `LoopCondBreakContinue{NoExitBody}` route: `matched` keeps the
/// provenance row and `sole_family` still front-selects the live owner.
#[test]
fn trim_header_dead_route_does_not_hide_loop_cond_sole_family() {
    let variable = |name: &str| ASTNode::Variable {
        name: name.into(),
        span: Span::unknown(),
    };
    let substring = ASTNode::MethodCall {
        object: Box::new(variable("s")),
        method: "substring".into(),
        arguments: vec![
            variable("i"),
            ASTNode::BinaryOp {
                operator: BinaryOperator::Add,
                left: Box::new(variable("i")),
                right: Box::new(ASTNode::Literal {
                    value: LiteralValue::Integer(1),
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            },
        ],
        span: Span::unknown(),
    };
    let condition = ASTNode::BinaryOp {
        operator: BinaryOperator::And,
        left: Box::new(ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(variable("i")),
            right: Box::new(variable("n")),
            span: Span::unknown(),
        }),
        right: Box::new(ASTNode::BinaryOp {
            operator: BinaryOperator::Equal,
            left: Box::new(substring),
            right: Box::new(ASTNode::Literal {
                value: LiteralValue::String(" ".into()),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }),
        span: Span::unknown(),
    };
    let body = vec![ASTNode::Assignment {
        target: Box::new(variable("i")),
        value: Box::new(ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            left: Box::new(variable("i")),
            right: Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(1),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }),
        span: Span::unknown(),
    }];
    crate::runtime::ring0::ensure_global_ring0_initialized();
    let policy = LoopFactsPolicyFrameV1::from_values(true, true, false, true, true, true);
    let outcome = single_planner::try_build_source_outcome(CallableLoopFactsPlannerInputV1::new(
        &condition,
        &body,
        policy,
        "trim-header-fixture".into(),
        false,
    ))
    .expect("planner outcome");
    let selection = CallableLoopRouteMatchV1::issue(outcome.facts.as_ref().expect("facts"));
    assert!(selection
        .matched_routes()
        .contains(&LoopRouteId::LoopBreakRecipe));
    assert!(selection
        .matched_routes()
        .contains(&LoopRouteId::LoopCondBreakContinue));
    assert_eq!(
        selection.sole_family(),
        Some(CallableLoopSoleFamilyV1::LoopCondBreakContinue)
    );
}
