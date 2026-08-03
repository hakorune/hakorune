use super::nested_predicate_producer::{
    produce_nested_predicate_recipe_v1, NestedPredicateRecipeProducerRejectV1,
};
use super::nested_predicate_projection::issue_nested_predicate_source_projection_v1;
use crate::ast::{ASTNode, BinaryOperator, DeclarationAttrs, LiteralValue, Span};
use crate::mir::compiler::VerifiedResolvedSourceUnitV1;
use crate::mir::loop_recipe_contract::{
    LoopJoinSigElaboratorV1, LoopRecipeV1, LoopRecipeVerifierV1,
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

fn assign(name: &str, value: i64) -> ASTNode {
    ASTNode::Assignment {
        target: Box::new(variable(name)),
        value: Box::new(integer(value)),
        span: Span::unknown(),
    }
}

fn increment(name: &str) -> ASTNode {
    ASTNode::Assignment {
        target: Box::new(variable(name)),
        value: Box::new(ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            left: Box::new(variable(name)),
            right: Box::new(integer(1)),
            span: Span::unknown(),
        }),
        span: Span::unknown(),
    }
}

fn nested_function() -> ASTNode {
    let child = ASTNode::Loop {
        condition: Box::new(ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(variable("j")),
            right: Box::new(integer(3)),
            span: Span::unknown(),
        }),
        body: vec![increment("sum"), increment("j")],
        span: Span::unknown(),
    };
    ASTNode::FunctionDeclaration {
        name: "nested_loop_minimal".into(),
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
                body: vec![
                    ASTNode::Local {
                        variables: vec!["j".into()],
                        initial_values: vec![None],
                        declared_type_names: vec![None],
                        span: Span::unknown(),
                    },
                    assign("j", 0),
                    child,
                    increment("i"),
                ],
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

fn projection_for(
    tree: ASTNode,
) -> super::nested_predicate_projection::VerifiedNestedLoopSourceProjectionV1 {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(tree).unwrap();
    let input = unit.root_function_input().expect("root input");
    let body = input.source().root_body().expect("function body");
    let root = input.source().body_stmt(&body, 1).expect("root loop");
    issue_nested_predicate_source_projection_v1(input, &root).expect("source projection")
}

#[test]
fn nested_producer_emits_verified_recipe_and_joinsig() {
    let product = produce_nested_predicate_recipe_v1(projection_for(nested_function()))
        .expect("nested producer");
    assert_eq!(product.recipe().as_recipe().loops.len(), 2);
    assert_eq!(product.join_sig().as_sig().loops.len(), 2);
    assert_eq!(product.recipe().as_recipe().inputs.len(), 2);
}

#[test]
fn nested_producer_matches_existing_recipe_and_joinsig_oracle() {
    let product = produce_nested_predicate_recipe_v1(projection_for(nested_function()))
        .expect("nested producer");
    let json: serde_json::Value = serde_json::from_str(include_str!(
        "../loop_recipe_contract/fixtures/nested_predicate_v1.json"
    ))
    .expect("nested recipe fixture");
    let mut expected: LoopRecipeV1 =
        serde_json::from_value(json["recipe"].clone()).expect("nested semantic recipe");
    expected.bindings[0].label = "root_0".into();
    expected.bindings[1].label = "root_1".into();
    expected.bindings[2].label = "child_0".into();
    assert_eq!(product.recipe().as_recipe(), &expected);
    let expected_verified = LoopRecipeVerifierV1::verify(expected).expect("fixture verifies");
    let expected_join_sig =
        LoopJoinSigElaboratorV1::elaborate(&expected_verified).expect("fixture JoinSig");
    assert_eq!(product.join_sig().as_sig(), expected_join_sig.as_sig());
}

#[test]
fn nested_producer_rejects_nonzero_root_initializer() {
    let mut tree = nested_function();
    let ASTNode::FunctionDeclaration { body, .. } = &mut tree else {
        unreachable!();
    };
    let ASTNode::Local { initial_values, .. } = &mut body[0] else {
        unreachable!();
    };
    initial_values[0] = Some(Box::new(integer(1)));
    assert!(matches!(
        produce_nested_predicate_recipe_v1(projection_for(tree)),
        Err(NestedPredicateRecipeProducerRejectV1::RootInitializerValue { index: 0, value: 1 })
    ));
}

#[test]
fn nested_producer_rejects_nonzero_child_initializer() {
    let mut tree = nested_function();
    let ASTNode::FunctionDeclaration { body, .. } = &mut tree else {
        unreachable!();
    };
    let ASTNode::Loop {
        body: root_body, ..
    } = &mut body[1]
    else {
        unreachable!();
    };
    root_body[1] = assign("j", 1);
    assert!(matches!(
        produce_nested_predicate_recipe_v1(projection_for(tree)),
        Err(NestedPredicateRecipeProducerRejectV1::ChildInitializerValue { value: 1 })
    ));
}
