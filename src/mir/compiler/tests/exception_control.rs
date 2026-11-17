//! Test-only throw, loop, and try/catch observations.

use super::*;


#[test]
#[ignore = "MIR13 migration: loop safepoint expectation pending"]
fn test_loop_compilation() {
    let mut compiler = MirCompiler::new();

    let loop_ast = ASTNode::Loop {
        condition: Box::new(ASTNode::Literal {
            value: LiteralValue::Bool(true),
            span: crate::ast::Span::unknown(),
        }),
        body: vec![ASTNode::Print {
            expression: Box::new(ASTNode::Literal {
                value: LiteralValue::String("Loop body".to_string()),
                span: crate::ast::Span::unknown(),
            }),
            span: crate::ast::Span::unknown(),
        }],
        span: crate::ast::Span::unknown(),
    };

    let result = compiler.compile(loop_ast);
    assert!(result.is_ok(), "Loop compilation should succeed");

    let compile_result = result.unwrap();
    let mir_dump = compiler.dump_mir(&compile_result.module);
    assert!(
        mir_dump.contains("br"),
        "MIR should contain branch instructions"
    );
    assert!(
        mir_dump.contains("safepoint"),
        "MIR should contain safepoint instructions"
    );
}
