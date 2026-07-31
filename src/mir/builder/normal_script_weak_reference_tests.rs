use crate::ast::{ASTNode, LiteralValue, Span, UnaryOperator};
use crate::mir::{MirCompiler, MirPrinter, NormalCompileRequestV1};

fn integer(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

fn variable(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.to_owned(),
        span: Span::unknown(),
    }
}

fn weak(operand: ASTNode) -> ASTNode {
    ASTNode::UnaryOp {
        operator: UnaryOperator::Weak,
        operand: Box::new(operand),
        span: Span::unknown(),
    }
}

#[test]
fn weak_reference_uses_complete_script_source_and_matches_legacy() {
    let program = ASTNode::Program {
        statements: vec![weak(integer(1))],
        span: Span::unknown(),
    };
    let mut legacy = MirCompiler::with_options(false);
    let legacy = legacy
        .compile_with_source(program.clone(), Some("script-weak-reference.hako"))
        .expect("legacy Weak");
    let normal = MirCompiler::with_options(false)
        .compile_normal(
            NormalCompileRequestV1::for_mir_mode(
                program,
                Some("script-weak-reference.hako"),
                std::collections::HashMap::new(),
            )
            .expect("normal request"),
        )
        .expect("selected Weak");
    assert_eq!(
        MirPrinter::new().print_module(&normal.module),
        MirPrinter::new().print_module(&legacy.module)
    );
    assert_eq!(normal.verification_result, legacy.verification_result);
}

#[test]
fn weak_missing_operand_stays_deferred_and_allows_fresh_reuse() {
    let mut compiler = MirCompiler::with_options(false);
    let error = compiler
        .compile_normal(
            NormalCompileRequestV1::for_mir_mode(
                ASTNode::Program {
                    statements: vec![weak(variable("missing"))],
                    span: Span::unknown(),
                },
                Some("script-weak-missing.hako"),
                std::collections::HashMap::new(),
            )
            .expect("normal request"),
        )
        .expect_err("missing Weak operand must retain RootLower diagnostic");
    assert!(error.contains("Undefined variable: missing"), "{error}");

    compiler
        .compile_normal(
            NormalCompileRequestV1::for_mir_mode(
                ASTNode::Program {
                    statements: vec![weak(integer(1))],
                    span: Span::unknown(),
                },
                Some("script-weak-reuse.hako"),
                std::collections::HashMap::new(),
            )
            .expect("fresh request"),
        )
        .expect("fresh request must not reuse a Deferred Weak product");
}
