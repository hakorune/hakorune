#![cfg(feature = "vm-reference")]

use crate::ast::{ASTNode, DeclarationAttrs, LiteralValue, Span};
use crate::mir::compiler::capability::{CanonicalFirstFamilyPlanV1, CanonicalLoweringPreflightV1};
use crate::mir::compiler::VerifiedResolvedSourceUnitV1;

fn literal(value: LiteralValue) -> ASTNode {
    ASTNode::Literal {
        value,
        span: Span::unknown(),
    }
}

fn null() -> ASTNode {
    literal(LiteralValue::Null)
}

fn boolean(value: bool) -> ASTNode {
    literal(LiteralValue::Bool(value))
}

fn integer(value: i64) -> ASTNode {
    literal(LiteralValue::Integer(value))
}

fn variable(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.into(),
        span: Span::unknown(),
    }
}

fn equal(left: ASTNode, right: ASTNode) -> ASTNode {
    ASTNode::BinaryOp {
        operator: BinaryOperator::Equal,
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

fn function(name: &str, mut body: Vec<ASTNode>, result: ASTNode) -> ASTNode {
    body.push(ASTNode::Return {
        value: Some(Box::new(result)),
        span: Span::unknown(),
    });
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

#[test]
fn null_return_outbox_and_mixed_merge_stay_whole_unit_a_plus() {
    let fixtures = [
        function("null_return", Vec::new(), null()),
        function(
            "mixed_null",
            vec![
                local("x", null()),
                if_(
                    boolean(true),
                    vec![assignment("x", integer(1))],
                    Some(vec![assignment("x", null())]),
                ),
            ],
            equal(variable("x"), null()),
        ),
        function(
            "outbox_stays_aplus",
            vec![ASTNode::Outbox {
                variables: vec!["result".into()],
                initial_values: vec![None],
                span: Span::unknown(),
            }],
            boolean(true),
        ),
    ];

    for fixture in fixtures {
        let unit = VerifiedResolvedSourceUnitV1::resolve_function(fixture).unwrap();
        let plan = CanonicalLoweringPreflightV1::verify(&unit).unwrap();
        assert!(matches!(
            plan,
            CanonicalFirstFamilyPlanV1::CurrentCanonicalAPlus(_)
        ));
    }
}
