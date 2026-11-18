#![cfg(feature = "vm-reference")]

use crate::ast::{ASTNode, BinaryOperator, DeclarationAttrs, LiteralValue, Span};
use crate::backend::{MirInterpreter, VMValue};
use crate::mir::compiler::capability::{CanonicalFirstFamilyPlanV1, CanonicalLoweringPreflightV1};
use crate::mir::compiler::{MirCompileResult, VerifiedResolvedSourceUnitV1};

fn literal(value: LiteralValue) -> ASTNode {
    ASTNode::Literal {
        value,
        span: Span::unknown(),
    }
}

fn void() -> ASTNode {
    literal(LiteralValue::Void)
}

fn null() -> ASTNode {
    literal(LiteralValue::Null)
}

fn boolean(value: bool) -> ASTNode {
    literal(LiteralValue::Bool(value))
}

fn variable(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.into(),
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

fn local(name: &str, value: ASTNode) -> ASTNode {
    ASTNode::Local {
        variables: vec![name.into()],
        initial_values: vec![Some(Box::new(value))],
        declared_type_names: vec![None],
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

fn if_(condition: ASTNode, then_body: Vec<ASTNode>, else_body: Option<Vec<ASTNode>>) -> ASTNode {
    ASTNode::If {
        condition: Box::new(condition),
        then_body,
        else_body,
        span: Span::unknown(),
    }
}

fn function(name: &str, body: Vec<ASTNode>) -> ASTNode {
    ASTNode::FunctionDeclaration {
        name: name.into(),
        params: Vec::new(),
        param_decls: Vec::new(),
        return_type_name: None,
        body,
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static: true,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    }
}

fn returning(name: &str, mut body: Vec<ASTNode>, value: ASTNode) -> ASTNode {
    body.push(ASTNode::Return {
        value: Some(Box::new(value)),
        span: Span::unknown(),
    });
    function(name, body)
}

fn compile(root: ASTNode) -> MirCompileResult {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(root).unwrap();
    let CanonicalFirstFamilyPlanV1::TrivialBindingSsa(plan) =
        CanonicalLoweringPreflightV1::verify(&unit).unwrap()
    else {
        panic!("fixture must select trivial Binding SSA");
    };
    let module = crate::mir::MirBuilder::new()
        .build_resolved_trivial_function_module(plan)
        .unwrap();
    let verification_result = crate::mir::MirVerifier::new().verify_module(&module);
    crate::mir::compiler::MirCompileResult { module, verification_result }
}

fn execute(root: ASTNode, function_name: &str) -> (MirCompileResult, VMValue) {
    let result = compile(root);
    let value = MirInterpreter::new()
        .execute_function_with_args(&result.module, function_name, &[])
        .unwrap();
    (result, value)
}

fn assert_a_plus(root: ASTNode) {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(root).unwrap();
    let plan = CanonicalLoweringPreflightV1::verify(&unit).unwrap();
    assert!(matches!(
        plan,
        CanonicalFirstFamilyPlanV1::CurrentCanonicalAPlus(_)
    ));
}

#[test]
fn void_comparison_and_no_value_completion_execute_exactly() {
    let cases = [
        (
            returning(
                "void_equal",
                Vec::new(),
                binary(BinaryOperator::Equal, void(), void()),
            ),
            "void_equal/0",
            VMValue::Bool(true),
        ),
        (
            returning(
                "void_not_equal",
                Vec::new(),
                binary(BinaryOperator::NotEqual, void(), void()),
            ),
            "void_not_equal/0",
            VMValue::Bool(false),
        ),
        (
            function(
                "void_empty_return",
                vec![ASTNode::Return {
                    value: None,
                    span: Span::unknown(),
                }],
            ),
            "void_empty_return/0",
            VMValue::Void,
        ),
        (
            function("void_implicit", vec![void()]),
            "void_implicit/0",
            VMValue::Void,
        ),
    ];

    for (root, name, expected) in cases {
        let (_, actual) = execute(root, name);
        assert_eq!(actual, expected);
    }
}

#[test]
fn outbox_null_mixing_void_condition_and_arithmetic_remain_a_plus() {
    assert_a_plus(returning("null_return", Vec::new(), null()));
    assert_a_plus(returning(
        "mixed_void_null",
        vec![
            local("x", void()),
            if_(
                boolean(true),
                vec![assignment("x", null())],
                Some(vec![assignment("x", void())]),
            ),
        ],
        variable("x"),
    ));
    assert_a_plus(function(
        "void_condition",
        vec![if_(void(), Vec::new(), None)],
    ));
    assert_a_plus(returning(
        "void_arithmetic",
        Vec::new(),
        binary(BinaryOperator::Add, void(), void()),
    ));
    assert_a_plus(function(
        "outbox_stays_aplus",
        vec![ASTNode::Outbox {
            variables: vec!["result".into()],
            initial_values: vec![None],
            span: Span::unknown(),
        }],
    ));
}
