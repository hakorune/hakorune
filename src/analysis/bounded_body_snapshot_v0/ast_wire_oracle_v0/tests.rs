use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span, UnaryOperator};

use super::{observe_ast_body_v0, AstWireOracleErrorV0};
use crate::analysis::bounded_body_snapshot_v0::{WireExprKindV0, WireNodeKindV0};

fn span() -> Span {
    Span::unknown()
}

fn int(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: span(),
    }
}

fn null() -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Null,
        span: span(),
    }
}

fn var(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.to_string(),
        span: span(),
    }
}

fn local(name: &str, value: Option<ASTNode>) -> ASTNode {
    ASTNode::Local {
        variables: vec![name.to_string()],
        initial_values: vec![value.map(Box::new)],
        declared_type_names: vec![None],
        span: span(),
    }
}

fn assignment(name: &str, value: ASTNode) -> ASTNode {
    ASTNode::Assignment {
        target: Box::new(var(name)),
        value: Box::new(value),
        span: span(),
    }
}

#[test]
fn source_loss_equivalence_classes_are_exact() {
    let pairs = [
        (local("x", Some(int(1))), assignment("x", int(1))),
        (
            ASTNode::Print {
                expression: Box::new(int(1)),
                span: span(),
            },
            ASTNode::FunctionCall {
                name: "env.console.log".to_string(),
                arguments: vec![int(1)],
                span: span(),
            },
        ),
        (
            ASTNode::Return {
                value: None,
                span: span(),
            },
            ASTNode::Return {
                value: Some(Box::new(int(0))),
                span: span(),
            },
        ),
        (
            ASTNode::UnaryOp {
                operator: UnaryOperator::Minus,
                operand: Box::new(int(1)),
                span: span(),
            },
            int(-1),
        ),
        (local("x", None), local("x", Some(null()))),
    ];
    for (left, right) in pairs {
        assert_eq!(
            observe_ast_body_v0(&[left]).unwrap(),
            observe_ast_body_v0(&[right]).unwrap(),
        );
    }
}

#[test]
fn local_binding_expansion_matches_multiple_wire_locals() {
    let multi = ASTNode::Local {
        variables: vec!["a".to_string(), "b".to_string()],
        initial_values: vec![Some(Box::new(int(1))), None],
        declared_type_names: vec![Some("i64".to_string()), None],
        span: span(),
    };
    assert_eq!(
        observe_ast_body_v0(&[multi]).unwrap(),
        observe_ast_body_v0(&[local("a", Some(int(1))), local("b", None)]).unwrap(),
    );
}

#[test]
fn accepted_statement_and_container_shapes_build_complete_snapshot() {
    let body = vec![
        ASTNode::ScopeBox {
            body: vec![local("x", Some(int(1)))],
            span: span(),
        },
        ASTNode::If {
            condition: Box::new(ASTNode::Literal {
                value: LiteralValue::Bool(true),
                span: span(),
            }),
            then_body: vec![ASTNode::Break { span: span() }],
            else_body: Some(vec![ASTNode::Continue { span: span() }]),
            span: span(),
        },
        ASTNode::Loop {
            condition: Box::new(var("keep")),
            body: vec![],
            span: span(),
        },
        ASTNode::LoopRange {
            var_name: "i".to_string(),
            start: Box::new(int(0)),
            end: Box::new(int(2)),
            body: vec![],
            span: span(),
        },
        ASTNode::Return {
            value: Some(Box::new(ASTNode::This { span: span() })),
            span: span(),
        },
    ];
    let snapshot = observe_ast_body_v0(&body).unwrap();
    assert_eq!(snapshot.source_program_version(), 0);
    assert_eq!(snapshot.nodes()[0].path().to_string(), "$.body[0]");
    assert_eq!(snapshot.max_depth_observed(), 2);
}

#[test]
fn all_operators_use_the_closed_wire_partition() {
    let operators = [
        BinaryOperator::Add,
        BinaryOperator::Subtract,
        BinaryOperator::Multiply,
        BinaryOperator::Divide,
        BinaryOperator::Modulo,
        BinaryOperator::BitAnd,
        BinaryOperator::BitOr,
        BinaryOperator::BitXor,
        BinaryOperator::Shl,
        BinaryOperator::Shr,
        BinaryOperator::Equal,
        BinaryOperator::NotEqual,
        BinaryOperator::Less,
        BinaryOperator::Greater,
        BinaryOperator::LessEqual,
        BinaryOperator::GreaterEqual,
        BinaryOperator::And,
        BinaryOperator::Or,
    ];
    for (ordinal, operator) in operators.into_iter().enumerate() {
        let expression = ASTNode::BinaryOp {
            operator,
            left: Box::new(int(1)),
            right: Box::new(int(2)),
            span: span(),
        };
        let snapshot = observe_ast_body_v0(&[expression]).unwrap();
        let kind = snapshot.nodes()[1].kind();
        let expected = if ordinal < 10 {
            WireExprKindV0::Binary
        } else if ordinal < 16 {
            WireExprKindV0::Compare
        } else {
            WireExprKindV0::Logical
        };
        assert_eq!(kind, WireNodeKindV0::Expr(expected));
    }
}

#[test]
fn context_sensitive_and_source_only_shapes_are_explicit_unsupported() {
    for node in [
        ASTNode::FunctionCall {
            name: "f".to_string(),
            arguments: vec![],
            span: span(),
        },
        ASTNode::MethodCall {
            object: Box::new(var("x")),
            method: "m".to_string(),
            arguments: vec![],
            span: span(),
        },
        ASTNode::FieldAccess {
            object: Box::new(var("x")),
            field: "value".to_string(),
            span: span(),
        },
        ASTNode::Literal {
            value: LiteralValue::Float(1.0),
            span: span(),
        },
        ASTNode::Index {
            target: Box::new(var("xs")),
            index: Box::new(int(0)),
            span: span(),
        },
    ] {
        assert!(matches!(
            observe_ast_body_v0(&[node]),
            Err(AstWireOracleErrorV0::Unsupported { .. })
        ));
    }
}
