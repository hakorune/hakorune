use super::*;

#[test]
fn test_canonicalize_rejects_non_loop() {
    let not_loop = ASTNode::Literal {
        value: LiteralValue::Bool(true),
        span: Span::unknown(),
    };

    let result = canonicalize_loop_expr(&not_loop);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Expected Loop node"));
}
