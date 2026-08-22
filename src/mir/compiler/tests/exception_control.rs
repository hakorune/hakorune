//! Test-only throw, loop, and try/catch observations.

use super::*;

#[test]
#[ignore = "MIR13 migration: throw/safepoint expectations pending"]
fn test_throw_compilation() {
    let mut compiler = MirCompiler::new();

    let throw_ast = ASTNode::Throw {
        expression: Box::new(ASTNode::Literal {
            value: LiteralValue::String("Test exception".to_string()),
            span: crate::ast::Span::unknown(),
        }),
        span: crate::ast::Span::unknown(),
    };

    let result = compiler.compile(throw_ast);
    assert!(result.is_ok(), "Throw compilation should succeed");

    let compile_result = result.unwrap();
    let mir_dump = compiler.dump_mir(&compile_result.module);
    assert!(
        mir_dump.contains("throw"),
        "MIR should contain throw instruction"
    );
    assert!(
        mir_dump.contains("safepoint"),
        "MIR should contain safepoint instruction"
    );
}

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

#[test]
fn test_try_catch_compilation() {
    // Core-13 pure モードでは Try/Catch 命令は許容集合外のためスキップ
    if crate::config::env::mir_core13_pure() {
        if crate::config::env::joinir_dev::debug_enabled() {
            let ring0 = crate::runtime::get_global_ring0();
            ring0
                .log
                .debug("[TEST] skip try/catch under Core-13 pure mode");
        }
        return;
    }
    let mut compiler = MirCompiler::new();

    let try_catch_ast = ASTNode::Program {
        statements: vec![ASTNode::TryCatch {
            try_body: vec![ASTNode::Print {
                expression: Box::new(ASTNode::Literal {
                    value: LiteralValue::String("Try block".to_string()),
                    span: crate::ast::Span::unknown(),
                }),
                span: crate::ast::Span::unknown(),
            }],
            catch_clauses: vec![crate::ast::CatchClause {
                exception_type: Some("Exception".to_string()),
                variable_name: Some("e".to_string()),
                body: vec![ASTNode::Print {
                    expression: Box::new(ASTNode::Literal {
                        value: LiteralValue::String("Catch block".to_string()),
                        span: crate::ast::Span::unknown(),
                    }),
                    span: crate::ast::Span::unknown(),
                }],
                span: crate::ast::Span::unknown(),
            }],
            finally_body: None,
            span: crate::ast::Span::unknown(),
        }],
        span: crate::ast::Span::unknown(),
    };

    let result = compiler.compile(try_catch_ast);
    assert!(
        result.is_ok(),
        "TryCatch compilation should succeed: {result:?}"
    );

    let compile_result = result.unwrap();
    let mir_dump = compiler.dump_mir(&compile_result.module);
    assert!(
        mir_dump.contains("catch"),
        "MIR should contain catch instruction"
    );
}
