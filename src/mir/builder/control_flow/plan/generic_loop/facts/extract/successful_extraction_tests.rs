use super::test_support::with_joinir_env;
use super::v1::{try_extract_generic_loop_v1, try_extract_generic_loop_v1_facts};
use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span, UnaryOperator};
use crate::mir::builder::control_flow::generic_loop_canon::StepPlacement;
use crate::mir::builder::control_flow::plan::generic_loop::facts_types::{
    GenericLoopCarrierRoleV1, GenericLoopV1StepDispositionV1,
};

fn var(name: &str) -> ASTNode {
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

fn boolean(value: bool) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Bool(value),
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

fn assignment(name: &str, value: ASTNode) -> ASTNode {
    ASTNode::Assignment {
        target: Box::new(var(name)),
        value: Box::new(value),
        span: Span::unknown(),
    }
}

fn step(name: &str) -> ASTNode {
    assignment(name, binary(BinaryOperator::Add, var(name), integer(1)))
}

fn condition(name: &str) -> ASTNode {
    binary(BinaryOperator::Less, var(name), var("limit"))
}

fn receiver_call(method: &str) -> ASTNode {
    ASTNode::MethodCall {
        object: Box::new(ASTNode::Me {
            span: Span::unknown(),
        }),
        method: method.to_string(),
        arguments: Vec::new(),
        span: Span::unknown(),
    }
}

fn assert_numeric_disposition(
    condition: &ASTNode,
    body: &[ASTNode],
    expected_placement: StepPlacement,
    expected_canonical_body_len: usize,
) {
    let extraction = try_extract_generic_loop_v1(condition, body)
        .expect("extraction must not freeze")
        .expect("fixture must match GenericLoopV1");
    assert_eq!(
        extraction.step(),
        &GenericLoopV1StepDispositionV1::NumericProgression {
            placement: expected_placement,
            canonical_body_len: expected_canonical_body_len,
        }
    );
    assert_eq!(
        extraction.facts().carrier_role,
        GenericLoopCarrierRoleV1::NumericProgression
    );
}

#[test]
fn successful_extraction_retains_last_and_in_body_dispositions() {
    with_joinir_env(None, None, || {
        let condition = condition("i");
        assert_numeric_disposition(&condition, &[step("i")], StepPlacement::Last, 1);
        assert_numeric_disposition(
            &condition,
            &[step("i"), assignment("tmp", integer(1))],
            StepPlacement::InBody(0),
            2,
        );
    });
}

#[test]
fn successful_extraction_retains_conditional_dispositions() {
    with_joinir_env(None, None, || {
        let condition = boolean(true);
        let continue_if = ASTNode::If {
            condition: Box::new(condition.clone()),
            then_body: vec![
                step("i"),
                ASTNode::Continue {
                    span: Span::unknown(),
                },
            ],
            else_body: None,
            span: Span::unknown(),
        };
        assert_numeric_disposition(
            &condition,
            &[continue_if],
            StepPlacement::InContinueIf(0),
            1,
        );

        let break_else_if = ASTNode::If {
            condition: Box::new(condition.clone()),
            then_body: vec![step("i")],
            else_body: Some(vec![ASTNode::Break {
                span: Span::unknown(),
            }]),
            span: Span::unknown(),
        };
        assert_numeric_disposition(
            &condition,
            &[break_else_if],
            StepPlacement::InBreakElseIf(0),
            1,
        );
    });
}

#[test]
fn successful_extraction_retains_flattened_canonical_body_length() {
    with_joinir_env(None, None, || {
        let condition = condition("i");
        let body = vec![ASTNode::ScopeBox {
            body: vec![assignment("tmp", integer(1)), step("i")],
            span: Span::unknown(),
        }];
        assert_numeric_disposition(&condition, &body, StepPlacement::Last, 2);

        let extraction = try_extract_generic_loop_v1(&condition, &body)
            .expect("extraction must not freeze")
            .expect("flattened fixture must match");
        assert_eq!(extraction.facts().body.body.len(), 2);
    });
}

#[test]
fn successful_extraction_retains_body_managed_disposition() {
    with_joinir_env(None, None, || {
        let condition = ASTNode::UnaryOp {
            operator: UnaryOperator::Not,
            operand: Box::new(receiver_call("is_eof")),
            span: Span::unknown(),
        };
        let body = vec![receiver_call("advance")];
        let extraction = try_extract_generic_loop_v1(&condition, &body)
            .expect("extraction must not freeze")
            .expect("receiver-managed fixture must match");

        assert_eq!(
            extraction.step(),
            &GenericLoopV1StepDispositionV1::BodyManagedState
        );
        assert_eq!(
            extraction.facts().carrier_role,
            GenericLoopCarrierRoleV1::BodyManagedState
        );
    });
}

#[test]
fn successful_extraction_retains_post_validation_body_managed_fallback() {
    with_joinir_env(None, None, || {
        let condition = condition("i");
        let body = vec![
            step("i"),
            ASTNode::If {
                condition: Box::new(boolean(true)),
                then_body: vec![assignment("tmp", integer(1))],
                else_body: None,
                span: Span::unknown(),
            },
        ];
        let extraction = try_extract_generic_loop_v1(&condition, &body)
            .expect("extraction must not freeze")
            .expect("post-validation fallback fixture must match");

        assert_eq!(
            extraction.step(),
            &GenericLoopV1StepDispositionV1::BodyManagedState
        );
        assert_eq!(
            extraction.facts().carrier_role,
            GenericLoopCarrierRoleV1::BodyManagedState
        );
    });
}

#[test]
fn facts_facade_projects_the_canonical_extraction() {
    with_joinir_env(Some("1"), Some("1"), || {
        let condition = condition("i");
        let body = vec![assignment("tmp", integer(1)), step("i")];
        let extraction = try_extract_generic_loop_v1(&condition, &body)
            .expect("canonical extraction must not freeze")
            .expect("canonical extraction must match");
        let facts = try_extract_generic_loop_v1_facts(&condition, &body)
            .expect("facts facade must not freeze")
            .expect("facts facade must match");

        assert_eq!(facts.carrier_role, extraction.facts().carrier_role);
        assert_eq!(facts.loop_var, extraction.facts().loop_var);
        assert_eq!(facts.condition, extraction.facts().condition);
        assert_eq!(facts.loop_increment, extraction.facts().loop_increment);
        assert_eq!(
            facts.body_lowering_policy,
            extraction.facts().body_lowering_policy
        );
        assert_eq!(facts.body.body, extraction.facts().body.body);
    });
}
