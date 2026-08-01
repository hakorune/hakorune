use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
use crate::mir::{MirCompiler, MirPrinter, NormalCompileRequestV1};

fn assert_selected_program_parity(program: ASTNode, hint: &str) {
    let mut legacy = MirCompiler::with_options(false);
    let legacy = legacy
        .compile_with_source(program.clone(), Some(hint))
        .expect("legacy compilation");
    let normal = MirCompiler::with_options(false)
        .compile_normal(
            NormalCompileRequestV1::for_mir_mode(
                program,
                Some(hint),
                std::collections::HashMap::new(),
            )
            .expect("normal request"),
        )
        .expect("selected normal compilation");
    assert_eq!(
        MirPrinter::new().print_module(&normal.module),
        MirPrinter::new().print_module(&legacy.module)
    );
    assert_eq!(normal.verification_result, legacy.verification_result);
}

#[test]
fn selected_normal_pure_block_expr_matches_legacy() {
    let program = ASTNode::Program {
        statements: vec![ASTNode::BlockExpr {
            prelude_stmts: vec![ASTNode::Print {
                expression: Box::new(ASTNode::Literal {
                    value: LiteralValue::Integer(2),
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            }],
            tail_expr: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Add,
                left: Box::new(ASTNode::Literal {
                    value: LiteralValue::Integer(2),
                    span: Span::unknown(),
                }),
                right: Box::new(ASTNode::Literal {
                    value: LiteralValue::Integer(1),
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }],
        span: Span::unknown(),
    };

    assert_selected_program_parity(program, "script-blockexpr-pure.hako");
}

#[test]
fn block_expr_variable_or_inner_local_stays_on_existing_lowering_route() {
    let variable_program = ASTNode::Program {
        statements: vec![ASTNode::BlockExpr {
            prelude_stmts: Vec::new(),
            tail_expr: Box::new(ASTNode::Variable {
                name: "missing".to_owned(),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }],
        span: Span::unknown(),
    };
    assert_legacy_error_parity(variable_program, "script-blockexpr-variable.hako");

    let program = ASTNode::Program {
        statements: vec![ASTNode::BlockExpr {
            prelude_stmts: vec![ASTNode::Local {
                variables: vec!["inner".to_owned()],
                initial_values: vec![Some(Box::new(ASTNode::Literal {
                    value: LiteralValue::Integer(1),
                    span: Span::unknown(),
                }))],
                declared_type_names: vec![None],
                span: Span::unknown(),
            }],
            tail_expr: Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(0),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }],
        span: Span::unknown(),
    };
    assert_selected_program_parity(program, "script-blockexpr-inner-local.hako");
}

#[test]
fn block_expr_escaping_exit_stays_deferred_to_existing_preflight() {
    let program = ASTNode::Program {
        statements: vec![ASTNode::BlockExpr {
            prelude_stmts: vec![ASTNode::Return {
                value: Some(Box::new(ASTNode::Literal {
                    value: LiteralValue::Integer(1),
                    span: Span::unknown(),
                })),
                span: Span::unknown(),
            }],
            tail_expr: Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(0),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }],
        span: Span::unknown(),
    };
    assert_legacy_error_parity(program, "script-blockexpr-exit.hako");
}

fn assert_legacy_error_parity(program: ASTNode, hint: &str) {
    let mut legacy = MirCompiler::with_options(false);
    let legacy_error = legacy
        .compile_with_source(program.clone(), Some(hint))
        .expect_err("legacy compilation rejects");
    let mut normal = MirCompiler::with_options(false);
    let normal_error = normal
        .compile_normal(
            NormalCompileRequestV1::for_mir_mode(
                program,
                Some(hint),
                std::collections::HashMap::new(),
            )
            .expect("normal request"),
        )
        .expect_err("normal compilation rejects through existing Lower");
    assert_eq!(normal_error, legacy_error);
}
