//! Test-only string corridor facts and candidates.

use super::*;

#[test]
fn test_compile_attaches_string_corridor_fact_for_string_length() {
    let ast = ASTNode::MethodCall {
        object: Box::new(ASTNode::Literal {
            value: LiteralValue::String("hello".to_string()),
            span: crate::ast::Span::unknown(),
        }),
        method: "length".to_string(),
        arguments: vec![],
        span: crate::ast::Span::unknown(),
    };

    let mut compiler = MirCompiler::new();
    let result = compiler.compile(ast).expect("compile should succeed");

    let len_fact_count = result
        .module
        .functions
        .values()
        .flat_map(|function| function.metadata.string_corridor_facts.values())
        .filter(|fact| fact.op == StringCorridorOp::StrLen)
        .count();

    assert!(
        len_fact_count >= 1,
        "expected at least one str.len fact in compiled MIR"
    );
}

#[test]
fn test_compile_attaches_string_corridor_candidate_for_string_length() {
    let ast = ASTNode::MethodCall {
        object: Box::new(ASTNode::Literal {
            value: LiteralValue::String("hello".to_string()),
            span: crate::ast::Span::unknown(),
        }),
        method: "length".to_string(),
        arguments: vec![],
        span: crate::ast::Span::unknown(),
    };

    let mut compiler = MirCompiler::new();
    let result = compiler.compile(ast).expect("compile should succeed");

    let direct_kernel_candidate_count = result
        .module
        .functions
        .values()
        .flat_map(|function| function.metadata.string_corridor_candidates.values())
        .flatten()
        .filter(|candidate| candidate.kind == StringCorridorCandidateKind::DirectKernelEntry)
        .count();

    assert!(
        direct_kernel_candidate_count >= 1,
        "expected at least one direct-kernel-entry candidate in compiled MIR"
    );
}
