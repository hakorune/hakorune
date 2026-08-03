#![cfg(test)]

use super::direct_accum_producer::{
    produce_direct_accum_recipe_v1, DirectAccumRecipeProducerRejectV1,
};
use crate::ast::{ASTNode, BinaryOperator, DeclarationAttrs, LiteralValue, Span};
use crate::mir::compiler::direct_accum_projection::issue_direct_accum_facts_v1;
use crate::mir::compiler::VerifiedResolvedSourceUnitV1;
use crate::mir::loop_route_policy::issue_policy_winner_for_test_with_frame;
use crate::mir::loop_structural_facts::{
    issue_selected_loop_recipe_demand_v1, DirectAccumFactsPayloadRejectV1,
    VerifiedSelectedLoopRecipeDemandV1,
};

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

fn add(name: &str, delta: i64) -> ASTNode {
    ASTNode::Assignment {
        target: Box::new(variable(name)),
        value: Box::new(ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            left: Box::new(variable(name)),
            right: Box::new(integer(delta)),
            span: Span::unknown(),
        }),
        span: Span::unknown(),
    }
}

fn function() -> ASTNode {
    ASTNode::FunctionDeclaration {
        name: "accum".into(),
        params: Vec::new(),
        param_decls: Vec::new(),
        return_type_name: None,
        body: vec![
            ASTNode::Local {
                variables: vec!["i".into(), "sum".into()],
                initial_values: vec![Some(Box::new(integer(0))), Some(Box::new(integer(0)))],
                declared_type_names: vec![None, None],
                span: Span::unknown(),
            },
            ASTNode::Loop {
                condition: Box::new(ASTNode::BinaryOp {
                    operator: BinaryOperator::Less,
                    left: Box::new(variable("i")),
                    right: Box::new(integer(3)),
                    span: Span::unknown(),
                }),
                body: vec![add("sum", 1), add("i", 1)],
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

fn demand() -> VerifiedSelectedLoopRecipeDemandV1 {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(function()).unwrap();
    let input = unit.root_function_input().unwrap();
    let body = input.source().root_body().unwrap();
    let loop_stmt = input.source().body_stmt(&body, 1).unwrap();
    let facts = issue_direct_accum_facts_v1(input, &loop_stmt).unwrap();
    let source = input
        .function()
        .resolved_loop_source(loop_stmt.site())
        .unwrap();
    let frame = source.frame_key();
    issue_selected_loop_recipe_demand_v1(
        issue_policy_winner_for_test_with_frame(4, &frame),
        facts,
        source,
    )
    .unwrap()
}

pub(crate) fn direct_accum_product_for_test() -> super::VerifiedDirectAccumRecipeProductV1 {
    produce_direct_accum_recipe_v1(demand()).expect("direct accum product")
}

#[test]
fn direct_accum_producer_emits_verified_recipe_and_join_sig() {
    let product = produce_direct_accum_recipe_v1(demand()).unwrap();
    assert_eq!(product.recipe().root_loop().raw(), 0);
    assert_eq!(product.join_sig().as_sig().loops.len(), 1);
    assert_eq!(product.join_sig().as_sig().loops[0].edges.len(), 4);
}

#[test]
fn producer_rejects_identity_only_payload_without_retry() {
    let demand = demand();
    let (winner, _facts, source) = demand.into_parts();
    let site = source.into_parts().2;
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(function()).unwrap();
    let input = unit.root_function_input().unwrap();
    let source = input.function().resolved_loop_source(&site).unwrap();
    let frame = source.frame_key();
    let facts =
        crate::mir::loop_structural_facts::verified_loop_structural_facts_for_test_with_frame(
            input.function().function_origin(),
            input.function().source_kind(),
            site,
            frame.clone(),
        );
    let demand = issue_selected_loop_recipe_demand_v1(winner, facts, source).unwrap();
    assert!(matches!(
        produce_direct_accum_recipe_v1(demand),
        Err(DirectAccumRecipeProducerRejectV1::FactsPayload(
            DirectAccumFactsPayloadRejectV1::NotDirectAccum
        ))
    ));
}
