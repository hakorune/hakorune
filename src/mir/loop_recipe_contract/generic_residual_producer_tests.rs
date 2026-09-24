#![cfg(test)]

use super::generic_residual_producer::{
    produce_generic_residual_recipe_v1, VerifiedGenericResidualRecipeProductV1,
};
use super::ids::{LoopBindingKeyV1, LoopValueKeyV1};
use super::join_sig::LoopJoinEdgeRoleV1;
use super::schema::{
    LoopBinaryI64OpV1, LoopCompareI64OpV1, LoopConditionV1, LoopOperationV1, LoopRecipeItemV1,
};
use crate::ast::{ASTNode, BinaryOperator, DeclarationAttrs, LiteralValue, Span};
use crate::mir::compiler::generic_residual_projection::issue_generic_residual_source_projection_v1;
use crate::mir::compiler::generic_residual_typed_map::{
    GenericResidualBodyRowV1, GenericResidualBoundV1, GenericResidualTypedSourceMapRejectV1,
    VerifiedGenericResidualTypedSourceMapV1,
};
use crate::mir::compiler::generic_residual_typed_map_issue::issue_generic_residual_typed_source_map_v1;
use crate::mir::compiler::VerifiedResolvedSourceUnitV1;
use crate::mir::loop_recipe_contract::route_id::LoopRouteId;
use crate::mir::loop_route_policy::{
    freeze_loop_route_schedule_v1, issue_generic_residual_policy_demand_v1,
    FrozenLoopRouteObservationV1, FrozenLoopRouteScheduleV1, LoopGlobalEntryDispositionV1,
    LoopModeReleaseSnapshotV1, LoopReleaseAdmissionObservationV1, LoopRouteCandidateFactsV1,
    LoopRoutePolicyEvidenceV1, LoopRoutePolicySourceDeclineReasonV1, LoopRouteSourceDispositionV1,
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

fn binary(operator: BinaryOperator, left: ASTNode, right: ASTNode) -> ASTNode {
    ASTNode::BinaryOp {
        operator,
        left: Box::new(left),
        right: Box::new(right),
        span: Span::unknown(),
    }
}

fn local_expr(name: &str, init: ASTNode) -> ASTNode {
    ASTNode::Local {
        variables: vec![name.into()],
        initial_values: vec![Some(Box::new(init))],
        declared_type_names: Vec::new(),
        span: Span::unknown(),
    }
}

fn step(name: &str, operator: BinaryOperator, delta: i64) -> ASTNode {
    ASTNode::Assignment {
        target: Box::new(variable(name)),
        value: Box::new(binary(operator, variable(name), integer(delta))),
        span: Span::unknown(),
    }
}

fn fixture(condition: ASTNode, body: Vec<ASTNode>, tail: Vec<ASTNode>) -> ASTNode {
    let mut root = vec![ASTNode::Loop {
        condition: Box::new(condition),
        body,
        span: Span::unknown(),
    }];
    root.extend(tail);
    ASTNode::FunctionDeclaration {
        name: "generic_residual_producer".into(),
        params: vec!["i".into(), "limit".into(), "j".into()],
        param_decls: Vec::new(),
        return_type_name: None,
        body: root,
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static: true,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    }
}

fn typed_map_for(node: ASTNode) -> Result<
    VerifiedGenericResidualTypedSourceMapV1,
    GenericResidualTypedSourceMapRejectV1,
> {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(node).expect("fixture resolves");
    let input = unit.root_function_input().expect("function input");
    let body = input.source().root_body().expect("function body");
    let loop_stmt = input.source().body_stmt(&body, 0).expect("root loop");
    let source = input
        .function()
        .resolved_loop_source(loop_stmt.site())
        .expect("root source");
    let projection = issue_generic_residual_source_projection_v1(input, &loop_stmt, source)
        .expect("bounded source projection");
    issue_generic_residual_typed_source_map_v1(input, projection)
}

pub(super) fn typed_map() -> VerifiedGenericResidualTypedSourceMapV1 {
    typed_map_for(crate::mir::compiler::generic_residual_function_for_test())
        .expect("typed source map")
}

pub(super) fn target_cursor() -> usize {
    CANONICAL_LOOP_ROUTE_ORDER_V1
        .iter()
        .position(|route| *route == LoopRouteId::GenericLoopV1)
        .unwrap()
}

pub(super) fn schedule_with_winner(winner: Option<usize>) -> FrozenLoopRouteScheduleV1 {
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
                if Some(index) == winner {
                    LoopRoutePolicyEvidenceV1::Candidate(LoopRouteCandidateFactsV1::SourceAvailable)
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

pub(super) fn demand() -> crate::mir::loop_route_policy::VerifiedGenericResidualPolicyDemandV1 {
    issue_generic_residual_policy_demand_v1(
        typed_map(),
        schedule_with_winner(Some(target_cursor())),
    )
    .unwrap()
}

fn product() -> VerifiedGenericResidualRecipeProductV1 {
    produce_generic_residual_recipe_v1(demand()).unwrap()
}

#[test]
fn typed_map_seals_bounded_profile_rows() {
    let map = typed_map();
    let condition = map.condition();
    assert!(matches!(
        condition.bound,
        GenericResidualBoundV1::Binding { .. }
    ));
    assert_eq!(condition.binding, map.carrier());
    assert_eq!(map.body_rows().len(), 1);
    let GenericResidualBodyRowV1::Declaration(decl) = &map.body_rows()[0] else {
        panic!("first body row must be the local declaration")
    };
    assert_eq!(decl.literal, 0);
    assert_eq!(map.carrier_step().binding, map.carrier());
    assert_eq!(map.carrier_step().delta, 1);
}

#[test]
fn producer_emits_exact_verified_recipe_and_join_sig() {
    let product = product();
    let recipe = product.recipe().as_recipe();
    assert_eq!(recipe.loops.len(), 1);
    assert_eq!(recipe.blocks.len(), 2);
    assert_eq!(recipe.items.len(), 8);
    // Two boundary bindings: the induction carrier and the read-only
    // bound variable. The body-local `tmp` is a per-iteration SSA value
    // and never becomes a binding, carrier, or input.
    assert_eq!(recipe.bindings.len(), 2);
    assert_eq!(
        recipe.inputs,
        vec![LoopValueKeyV1::new(0), LoopValueKeyV1::new(1)]
    );
    assert_eq!(recipe.values.len(), 9);
    assert_eq!(recipe.carriers.len(), 2);
    assert_eq!(recipe.exits.len(), 0);
    let LoopConditionV1::Predicate { block, value } = recipe.loops[0].condition else {
        panic!("Generic residual recipe must carry a predicate loop condition")
    };
    assert_eq!(block.raw(), 0);
    assert_eq!(value, LoopValueKeyV1::new(4));

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
        panic!("loop predicate must be a compare")
    };
    assert_eq!(*op, LoopCompareI64OpV1::Less);
    assert_eq!(*left, LoopValueKeyV1::new(2));
    assert_eq!(*right, LoopValueKeyV1::new(3));
    assert_eq!(*result, LoopValueKeyV1::new(4));

    // local tmp = 0 stays a pure per-iteration constant — no boundary
    // write is minted for a body-local.
    assert!(matches!(
        recipe.items[3].item,
        LoopRecipeItemV1::Operation {
            operation: LoopOperationV1::ConstI64 { value: 0, .. }
        }
    ));
    // i = i + 1 is the only boundary write.
    let LoopRecipeItemV1::Operation {
        operation:
            LoopOperationV1::BinaryI64 {
                op,
                left,
                right,
                result,
            },
    } = &recipe.items[6].item
    else {
        panic!("carrier step must be a binary i64 operation")
    };
    assert_eq!(*op, LoopBinaryI64OpV1::Add);
    assert_eq!(*left, LoopValueKeyV1::new(6));
    assert_eq!(*right, LoopValueKeyV1::new(7));
    assert_eq!(*result, LoopValueKeyV1::new(8));
    assert!(matches!(
        recipe.items[7].item,
        LoopRecipeItemV1::Operation {
            operation: LoopOperationV1::WriteBinding {
                binding,
                ..
            }
        } if binding == LoopBindingKeyV1::new(0)
    ));

    let carrier = &recipe.carriers[0];
    assert_eq!(carrier.binding, LoopBindingKeyV1::new(0));
    assert_eq!(carrier.entry_value, LoopValueKeyV1::new(0));
    let bound = &recipe.carriers[1];
    assert_eq!(bound.binding, LoopBindingKeyV1::new(1));
    assert_eq!(bound.entry_value, LoopValueKeyV1::new(1));

    let sig = product.join_sig().as_sig();
    let roles = sig.loops[0]
        .edges
        .iter()
        .map(|edge| edge.role)
        .collect::<Vec<_>>();
    for expected in [
        LoopJoinEdgeRoleV1::PredicateTrue,
        LoopJoinEdgeRoleV1::PredicateFalse,
    ] {
        assert!(
            roles.contains(&expected),
            "JoinSig must carry {:?} boundary, got {:?}",
            expected,
            roles
        );
    }
}

#[test]
fn producer_is_deterministic_and_retains_policy_frame_receipt() {
    let first = product();
    let second = product();
    assert_eq!(first.recipe().as_recipe(), second.recipe().as_recipe());
    assert_eq!(first.join_sig().as_sig(), second.join_sig().as_sig());
    let frame = typed_map().root_frame_key().clone();
    assert!(first.policy_receipt().frame_key().matches(&frame));
}

#[test]
fn demand_rejects_schedule_without_generic_candidate() {
    let result =
        issue_generic_residual_policy_demand_v1(typed_map(), schedule_with_winner(None));
    assert!(
        result.is_err(),
        "no GenericLoopV1 candidate must not be admitted"
    );
}

#[test]
fn demand_rejects_v0_overlap_winner_cursor() {
    // A [V0, V1] overlap selection wins at GenericLoopV0 (cursor 17); the
    // first cohort demands the exact GenericLoopV1 winner (cursor 18).
    let result = issue_generic_residual_policy_demand_v1(
        typed_map(),
        schedule_with_winner(Some(target_cursor() - 1)),
    );
    assert!(result.is_err(), "a V0 winner must not seal a V1 demand");
}

#[test]
fn typed_map_rejects_non_compare_condition() {
    let result = typed_map_for(fixture(
        variable("i"),
        vec![step("i", BinaryOperator::Add, 1)],
        Vec::new(),
    ));
    assert_eq!(
        result.unwrap_err(),
        GenericResidualTypedSourceMapRejectV1::ConditionShape
    );
}

#[test]
fn typed_map_rejects_out_of_vocabulary_operator() {
    let result = typed_map_for(fixture(
        binary(BinaryOperator::Greater, variable("i"), variable("limit")),
        vec![step("i", BinaryOperator::Add, 1)],
        Vec::new(),
    ));
    assert_eq!(
        result.unwrap_err(),
        GenericResidualTypedSourceMapRejectV1::UnsupportedOperator
    );
}

#[test]
fn typed_map_rejects_derived_bound_shape() {
    let result = typed_map_for(fixture(
        binary(
            BinaryOperator::Less,
            variable("i"),
            binary(BinaryOperator::Add, variable("i"), integer(1)),
        ),
        vec![step("i", BinaryOperator::Add, 1)],
        Vec::new(),
    ));
    assert_eq!(
        result.unwrap_err(),
        GenericResidualTypedSourceMapRejectV1::ConditionBoundShape
    );
}

#[test]
fn typed_map_rejects_non_integer_initializer() {
    let result = typed_map_for(fixture(
        binary(BinaryOperator::Less, variable("i"), variable("limit")),
        vec![local_expr("tmp", variable("i")), step("i", BinaryOperator::Add, 1)],
        Vec::new(),
    ));
    assert_eq!(
        result.unwrap_err(),
        GenericResidualTypedSourceMapRejectV1::InitializerNotInteger
    );
}

#[test]
fn typed_map_rejects_rebind_outside_body_locals() {
    let result = typed_map_for(fixture(
        binary(BinaryOperator::Less, variable("i"), variable("limit")),
        vec![
            local_expr("tmp", integer(0)),
            step("j", BinaryOperator::Add, 1),
            step("i", BinaryOperator::Add, 1),
        ],
        Vec::new(),
    ));
    assert_eq!(
        result.unwrap_err(),
        GenericResidualTypedSourceMapRejectV1::RebindTargetForeign
    );
}

#[test]
fn typed_map_rejects_missing_carrier_step() {
    let result = typed_map_for(fixture(
        binary(BinaryOperator::Less, variable("i"), variable("limit")),
        vec![local_expr("tmp", integer(0)), step("tmp", BinaryOperator::Add, 1)],
        Vec::new(),
    ));
    assert_eq!(
        result.unwrap_err(),
        GenericResidualTypedSourceMapRejectV1::CarrierStepMissing
    );
}

#[test]
fn typed_map_rejects_residual_function_exit() {
    let result = typed_map_for(fixture(
        binary(BinaryOperator::Less, variable("i"), variable("limit")),
        vec![step("i", BinaryOperator::Add, 1)],
        vec![ASTNode::Return {
            value: Some(Box::new(integer(0))),
            span: Span::unknown(),
        }],
    ));
    assert_eq!(
        result.unwrap_err(),
        GenericResidualTypedSourceMapRejectV1::ResidualExit
    );
}
