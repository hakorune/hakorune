//! Test-only await lowering and checkpoint observations.

use super::*;

#[test]
fn test_lowering_await_expression() {
    // Build AST: await 1  (semantic is nonsensical but should emit Await)
    let ast = ASTNode::Program {
        statements: vec![ASTNode::AwaitExpression {
            expression: Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(1),
                span: crate::ast::Span::unknown(),
            }),
            span: crate::ast::Span::unknown(),
        }],
        span: crate::ast::Span::unknown(),
    };
    let mut compiler = MirCompiler::new();
    let result = crate::test_support::with_env_vars(&[("NYASH_MIR_UNIFIED_CALL", None)], || {
        compiler.compile(ast)
    })
    .expect("compile should succeed");
    let dump = MirPrinter::new().print_module(&result.module);
    assert!(
        dump.contains("await"),
        "Expected await in MIR dump. Got:\n{}",
        dump
    );
}
