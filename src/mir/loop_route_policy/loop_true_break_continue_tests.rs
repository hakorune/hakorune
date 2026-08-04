use super::loop_true_break_continue::{
    issue_loop_true_break_continue_policy_demand_v1,
    seal_loop_true_break_continue_policy_demand_for_test,
    LoopTrueBreakContinuePolicyDemandRejectV1,
};
use super::{
    freeze_loop_route_schedule_v1, FrozenLoopRouteObservationV1, LoopGlobalEntryDispositionV1,
    LoopModeReleaseSnapshotV1, LoopPolicyBlockedReasonV1, LoopReleaseAdmissionObservationV1,
    LoopRouteCandidateFactsV1, LoopRoutePolicyBlockReasonV1, LoopRoutePolicyEvidenceV1,
    LoopRoutePolicySourceDeclineReasonV1, LoopRouteSourceDispositionV1,
    LoopRouteSuppressionDispositionV1, CANONICAL_LOOP_ROUTE_ORDER_V1,
};
use crate::ast::{ASTNode, BinaryOperator, DeclarationAttrs, LiteralValue, Span};
use crate::mir::compiler::loop_true_break_continue_projection::{
    issue_loop_true_break_continue_source_projection_v1,
    VerifiedLoopTrueBreakContinueSourceProjectionV1,
};
use crate::mir::compiler::VerifiedResolvedSourceUnitV1;
use crate::mir::loop_recipe_contract::route_id::LoopRouteId;

fn variable(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.to_owned(),
        span: Span::unknown(),
    }
}

fn integer(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

fn positive_function() -> ASTNode {
    ASTNode::FunctionDeclaration {
        name: "loop_true_policy".into(),
        params: Vec::new(),
        param_decls: Vec::new(),
        return_type_name: None,
        body: vec![
            ASTNode::Local {
                variables: vec!["flag".into()],
                initial_values: vec![Some(Box::new(integer(1)))],
                declared_type_names: vec![None],
                span: Span::unknown(),
            },
            ASTNode::Loop {
                condition: Box::new(ASTNode::Literal {
                    value: LiteralValue::Bool(true),
                    span: Span::unknown(),
                }),
                body: vec![ASTNode::If {
                    condition: Box::new(ASTNode::BinaryOp {
                        operator: BinaryOperator::Equal,
                        left: Box::new(variable("flag")),
                        right: Box::new(integer(1)),
                        span: Span::unknown(),
                    }),
                    then_body: vec![ASTNode::Break {
                        span: Span::unknown(),
                    }],
                    else_body: Some(vec![ASTNode::Continue {
                        span: Span::unknown(),
                    }]),
                    span: Span::unknown(),
                }],
                span: Span::unknown(),
            },
        ],
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static: true,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    }
}

fn projection() -> VerifiedLoopTrueBreakContinueSourceProjectionV1 {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(positive_function()).unwrap();
    let input = unit.root_function_input().unwrap();
    let body = input.source().root_body().unwrap();
    let loop_stmt = input.source().body_stmt(&body, 1).unwrap();
    let source = input
        .function()
        .resolved_loop_source(loop_stmt.site())
        .unwrap();
    issue_loop_true_break_continue_source_projection_v1(input, &loop_stmt, source).unwrap()
}

fn target_cursor() -> usize {
    CANONICAL_LOOP_ROUTE_ORDER_V1
        .iter()
        .position(|route| *route == LoopRouteId::LoopTrueBreakContinue)
        .unwrap()
}

fn schedule_at(
    cursor: usize,
    evidence: LoopRoutePolicyEvidenceV1,
) -> super::FrozenLoopRouteScheduleV1 {
    let observations = CANONICAL_LOOP_ROUTE_ORDER_V1
        .iter()
        .enumerate()
        .map(|(index, _)| {
            FrozenLoopRouteObservationV1::new(
                LoopRouteSuppressionDispositionV1::Retained,
                LoopModeReleaseSnapshotV1::Release {
                    admission: LoopReleaseAdmissionObservationV1::Allowed,
                },
                LoopGlobalEntryDispositionV1::Allowed,
                LoopRouteSourceDispositionV1::Available,
                if index == cursor {
                    evidence
                } else {
                    LoopRoutePolicyEvidenceV1::SourceDeclined(
                        LoopRoutePolicySourceDeclineReasonV1::PreEffectDeclined,
                    )
                },
            )
        })
        .collect::<Box<[_]>>();
    freeze_loop_route_schedule_v1(CANONICAL_LOOP_ROUTE_ORDER_V1.into(), observations).unwrap()
}

#[test]
fn policy_demand_consumes_schedule_and_retains_only_receipt_projection() {
    let projection = projection();
    let frame = projection.root_frame_key().clone();
    let demand = issue_loop_true_break_continue_policy_demand_v1(
        projection,
        schedule_at(
            target_cursor(),
            LoopRoutePolicyEvidenceV1::Candidate(LoopRouteCandidateFactsV1::SourceAvailable),
        ),
    )
    .unwrap();
    let (receipt, projection) = demand.into_parts();
    assert!(receipt.frame_key().matches(&frame));
    assert!(projection.root_frame_key().matches(&frame));
}

#[test]
fn policy_demand_rejects_an_earlier_winner() {
    assert_eq!(
        issue_loop_true_break_continue_policy_demand_v1(
            projection(),
            schedule_at(
                0,
                LoopRoutePolicyEvidenceV1::Candidate(LoopRouteCandidateFactsV1::SourceAvailable,),
            ),
        ),
        Err(
            LoopTrueBreakContinuePolicyDemandRejectV1::WrongWinnerCursor {
                expected: target_cursor(),
                actual: 0,
            },
        )
    );
}

#[test]
fn policy_demand_rejects_blocked_and_exhausted_schedule() {
    assert_eq!(
        issue_loop_true_break_continue_policy_demand_v1(
            projection(),
            schedule_at(
                target_cursor(),
                LoopRoutePolicyEvidenceV1::PolicyBlocked(
                    LoopRoutePolicyBlockReasonV1::PolicyAndTerminalityUnavailable,
                ),
            ),
        ),
        Err(LoopTrueBreakContinuePolicyDemandRejectV1::PolicyBlocked(
            LoopPolicyBlockedReasonV1::Policy(
                LoopRoutePolicyBlockReasonV1::PolicyAndTerminalityUnavailable,
            ),
        ))
    );
    let observations = CANONICAL_LOOP_ROUTE_ORDER_V1
        .iter()
        .map(|_| {
            FrozenLoopRouteObservationV1::new(
                LoopRouteSuppressionDispositionV1::Retained,
                LoopModeReleaseSnapshotV1::Release {
                    admission: LoopReleaseAdmissionObservationV1::Allowed,
                },
                LoopGlobalEntryDispositionV1::Allowed,
                LoopRouteSourceDispositionV1::Available,
                LoopRoutePolicyEvidenceV1::SourceDeclined(
                    LoopRoutePolicySourceDeclineReasonV1::PreEffectDeclined,
                ),
            )
        })
        .collect::<Box<[_]>>();
    assert_eq!(
        issue_loop_true_break_continue_policy_demand_v1(
            projection(),
            freeze_loop_route_schedule_v1(CANONICAL_LOOP_ROUTE_ORDER_V1.into(), observations)
                .unwrap(),
        ),
        Err(LoopTrueBreakContinuePolicyDemandRejectV1::Exhausted)
    );
}

#[test]
fn private_policy_seal_rejects_foreign_winner_frame() {
    let projection = projection();
    let foreign_frame = crate::mir::resolved_semantics::loop_execution_frame_key_for_test();
    let winner =
        super::policy::issue_policy_winner_for_test_with_frame(target_cursor(), &foreign_frame);
    assert_eq!(
        seal_loop_true_break_continue_policy_demand_for_test(projection, winner),
        Err(LoopTrueBreakContinuePolicyDemandRejectV1::ExecutionFrameMismatch)
    );
}
