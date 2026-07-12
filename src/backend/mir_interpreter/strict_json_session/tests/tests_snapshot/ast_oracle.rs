use super::*;
use crate::analysis::bounded_body_snapshot_v0::ast_wire_oracle_v0::observe_ast_body_v0;
use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span, UnaryOperator};

fn span() -> Span {
    Span::unknown()
}

fn int(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: span(),
    }
}

fn var(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.to_string(),
        span: span(),
    }
}

fn parity_body() -> Vec<ASTNode> {
    vec![
        ASTNode::Local {
            variables: vec!["i".to_string()],
            initial_values: vec![Some(Box::new(int(0)))],
            declared_type_names: vec![None],
            span: span(),
        },
        ASTNode::Print {
            expression: Box::new(var("i")),
            span: span(),
        },
        ASTNode::Assignment {
            target: Box::new(var("i")),
            value: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Add,
                left: Box::new(var("i")),
                right: Box::new(int(1)),
                span: span(),
            }),
            span: span(),
        },
        ASTNode::If {
            condition: Box::new(ASTNode::Literal {
                value: LiteralValue::Bool(true),
                span: span(),
            }),
            then_body: vec![ASTNode::Return {
                value: None,
                span: span(),
            }],
            else_body: Some(vec![ASTNode::Return {
                value: Some(Box::new(int(0))),
                span: span(),
            }]),
            span: span(),
        },
        ASTNode::Loop {
            condition: Box::new(ASTNode::Literal {
                value: LiteralValue::Bool(false),
                span: span(),
            }),
            body: vec![],
            span: span(),
        },
        ASTNode::Return {
            value: Some(Box::new(ASTNode::UnaryOp {
                operator: UnaryOperator::Minus,
                operand: Box::new(int(1)),
                span: span(),
            })),
            span: span(),
        },
    ]
}

fn ast_oracle_direct_parity_impl() {
    let source = r#"
static box Main {
  main() {
local i = 0
print(i)
i = i + 1
if true {
return
} else {
return 0
}
loop false {
}
return -1
  }
}
"#;
    let program_json =
        crate::stage1::program_json_v0::emit_program_json_v0_for_strict_authority_source(source)
            .expect("authoritative ProgramV0 serializer");
    let oracle = observe_ast_body_v0(&parity_body()).expect("AST wire observation");
    assert_eq!(oracle, rust_snapshot(&program_json));

    let module = compile_fixture();
    let mut interpreter = MirInterpreter::new();
    assert_eq!(
        run(
            &mut interpreter,
            &module,
            &program_json,
            "SnapshotDirectReaderFixtureV0Box.snapshot_signature/2",
        ),
        VMValue::String(snapshot_signature(&oracle)),
    );
    assert!(!interpreter.strict_json_session_active());
}

#[test]
fn rust_ast_oracle_matches_authoritative_serializer_through_hako_reader() {
    std::thread::Builder::new()
        .name("rust-ast-wire-oracle-direct-parity".to_string())
        .stack_size(32 * 1024 * 1024)
        .spawn(ast_oracle_direct_parity_impl)
        .expect("spawn AST oracle parity thread")
        .join()
        .expect("AST oracle parity thread");
}
