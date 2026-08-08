#![cfg(test)]

use super::ids::{LoopItemKeyV1, LoopValueKeyV1};
use super::join_sig::{LoopJoinBranchArmV1, LoopJoinEdgeRoleV1};
use super::loop_true_break_continue_producer::{
    produce_loop_true_break_continue_recipe_v1, VerifiedLoopTrueBreakContinueRecipeProductV1,
};
use super::schema::{LoopCompareI64OpV1, LoopOperationV1, LoopRecipeItemV1};
use crate::ast::{ASTNode, BinaryOperator, DeclarationAttrs, LiteralValue, Span};
use crate::mir::compiler::loop_true_break_continue_projection::{
    issue_loop_true_break_continue_source_projection_v1,
    VerifiedLoopTrueBreakContinueSourceProjectionV1,
};
use crate::mir::compiler::VerifiedResolvedSourceUnitV1;
use crate::mir::loop_recipe_contract::route_id::LoopRouteId;
use crate::mir::loop_route_policy::{
    freeze_loop_route_schedule_v1, issue_loop_true_break_continue_policy_demand_v1,
    FrozenLoopRouteObservationV1, LoopGlobalEntryDispositionV1, LoopModeReleaseSnapshotV1,
    LoopReleaseAdmissionObservationV1, LoopRouteCandidateFactsV1, LoopRoutePolicyEvidenceV1,
    LoopRoutePolicySourceDeclineReasonV1, LoopRouteSourceDispositionV1,
    LoopRouteSuppressionDispositionV1, CANONICAL_LOOP_ROUTE_ORDER_V1,
};

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
        name: "loop_true_recipe".into(),
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

fn demand() -> crate::mir::loop_route_policy::VerifiedLoopTrueBreakContinuePolicyDemandV1 {
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
                if index == target_cursor() {
                    LoopRoutePolicyEvidenceV1::Candidate(LoopRouteCandidateFactsV1::SourceAvailable)
                } else {
                    LoopRoutePolicyEvidenceV1::SourceDeclined(
                        LoopRoutePolicySourceDeclineReasonV1::PreEffectDeclined,
                    )
                },
            )
        })
        .collect::<Box<[_]>>();
    issue_loop_true_break_continue_policy_demand_v1(
        projection(),
        freeze_loop_route_schedule_v1(CANONICAL_LOOP_ROUTE_ORDER_V1.into(), observations).unwrap(),
    )
    .unwrap()
}

fn product() -> VerifiedLoopTrueBreakContinueRecipeProductV1 {
    produce_loop_true_break_continue_recipe_v1(demand()).unwrap()
}

#[test]
fn producer_emits_exact_verified_recipe_and_join_sig() {
    let product = product();
    let recipe = product.recipe().as_recipe();
    assert_eq!(recipe.loops.len(), 1);
    assert_eq!(recipe.blocks.len(), 3);
    assert_eq!(recipe.items.len(), 6);
    assert_eq!(recipe.bindings.len(), 1);
    assert_eq!(recipe.inputs, vec![LoopValueKeyV1::new(0)]);
    assert_eq!(recipe.values.len(), 4);
    assert_eq!(recipe.carriers.len(), 1);
    assert_eq!(recipe.exits.len(), 2);
    assert_eq!(recipe.items[3].key, LoopItemKeyV1::new(3));
    assert!(matches!(
        recipe.items[1].item,
        LoopRecipeItemV1::Operation {
            operation: LoopOperationV1::ConstI64 { value: 1, .. }
        }
    ));
    let LoopRecipeItemV1::Operation {
        operation:
            LoopOperationV1::CompareI64 {
                op,
                left,
                right,
                result,
            },
    } = &recipe.items[2].item
    else {
        panic!("LoopTrue recipe must compare the branch binding with its bound");
    };
    assert_eq!(*op, LoopCompareI64OpV1::Equal);
    assert_eq!(*left, LoopValueKeyV1::new(1));
    assert_eq!(*right, LoopValueKeyV1::new(2));
    assert_eq!(*result, LoopValueKeyV1::new(3));
    let sig = product.join_sig().as_sig();
    assert_eq!(sig.branches.len(), 1);
    assert_eq!(
        sig.loops[0]
            .edges
            .iter()
            .map(|edge| edge.role)
            .collect::<Vec<_>>(),
        vec![
            LoopJoinEdgeRoleV1::Enter,
            LoopJoinEdgeRoleV1::BodyEntry,
            LoopJoinEdgeRoleV1::Break,
            LoopJoinEdgeRoleV1::Continue,
        ]
    );
    assert!(!sig.loops[0]
        .edges
        .iter()
        .any(|edge| edge.role == LoopJoinEdgeRoleV1::Backedge));
    let LoopJoinBranchArmV1::Exit(then_exit) = &sig.branches[0].then_arm else {
        panic!("then arm must be a direct exit");
    };
    let LoopJoinBranchArmV1::Exit(else_exit) = &sig.branches[0].else_arm else {
        panic!("else arm must be a direct exit");
    };
    assert_eq!(then_exit.payload, else_exit.payload);
}

#[test]
fn producer_is_deterministic_and_retains_policy_frame_receipt() {
    let first = product();
    let second = product();
    assert_eq!(first.recipe().as_recipe(), second.recipe().as_recipe());
    assert_eq!(first.join_sig().as_sig(), second.join_sig().as_sig());
    let frame = projection().root_frame_key().clone();
    assert!(first.policy_receipt().frame_key().matches(&frame));
}
