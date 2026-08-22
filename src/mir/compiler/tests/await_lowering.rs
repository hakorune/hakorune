//! Test-only await lowering and checkpoint observations.

use super::*;

#[test]
fn test_lowering_await_expression() {
    if crate::config::env::mir_core13_pure() {
        if crate::config::env::joinir_dev::debug_enabled() {
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.debug("[TEST] skip await under Core-13 pure mode");
        }
        return;
    }
    // Build AST: await 1  (semantic is nonsensical but should emit Await)
    let ast = ASTNode::AwaitExpression {
        expression: Box::new(ASTNode::Literal {
            value: LiteralValue::Integer(1),
            span: crate::ast::Span::unknown(),
        }),
        span: crate::ast::Span::unknown(),
    };
    let mut compiler = MirCompiler::new();
    let result = compiler.compile(ast).expect("compile should succeed");
    let dump = MirPrinter::new().print_module(&result.module);
    assert!(
        dump.contains("await"),
        "Expected await in MIR dump. Got:\n{}",
        dump
    );
}

// Legacy await / safepoint モデルのテスト（Core-13/Pure 以降とは挙動差あり）.

#[test]
#[ignore]
fn test_await_has_checkpoints() {
    if crate::config::env::mir_core13_pure() {
        if crate::config::env::joinir_dev::debug_enabled() {
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.debug("[TEST] skip await under Core-13 pure mode");
        }
        return;
    }
    use crate::ast::{LiteralValue, Span};
    // Build: await 1
    let ast = ASTNode::AwaitExpression {
        expression: Box::new(ASTNode::Literal {
            value: LiteralValue::Integer(1),
            span: Span::unknown(),
        }),
        span: Span::unknown(),
    };
    let mut compiler = MirCompiler::new();
    let result = compiler.compile(ast).expect("compile");
    // Verifier should pass (await flanked by safepoints)
    assert!(
        result.verification_result.is_ok(),
        "Verifier failed for await checkpoints: {:?}",
        result.verification_result
    );
    let dump = compiler.dump_mir(&result.module);
    // Expect at least two safepoints in the function (before/after await)
    let sp_count = dump.matches("safepoint").count();
    assert!(
        sp_count >= 2,
        "Expected >=2 safepoints around await, got {}. Dump:\n{}",
        sp_count,
        dump
    );
}

// Legacy await rewrite テスト（現行の Future 統合とは独立にアーカイブ扱い）.

#[test]
#[ignore]
fn test_rewritten_await_still_checkpoints() {
    if crate::config::env::mir_core13_pure() {
        if crate::config::env::joinir_dev::debug_enabled() {
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.debug("[TEST] skip await under Core-13 pure mode");
        }
        return;
    }
    use crate::ast::{LiteralValue, Span};
    // Enable rewrite so Await → ExternCall(env.future.await)
    std::env::set_var("NYASH_REWRITE_FUTURE", "1");
    let ast = ASTNode::AwaitExpression {
        expression: Box::new(ASTNode::Literal {
            value: LiteralValue::Integer(1),
            span: Span::unknown(),
        }),
        span: Span::unknown(),
    };
    let mut compiler = MirCompiler::new();
    let result = compiler.compile(ast).expect("compile");
    // Verifier should still pass (checkpoint verification includes ExternCall await)
    assert!(
        result.verification_result.is_ok(),
        "Verifier failed for rewritten await checkpoints: {:?}",
        result.verification_result
    );
    let dump = compiler.dump_mir(&result.module);
    assert!(
        dump.contains("env.future.await"),
        "Expected rewritten await extern call. Dump:\n{}",
        dump
    );
    let sp_count = dump.matches("safepoint").count();
    assert!(
        sp_count >= 2,
        "Expected >=2 safepoints around rewritten await, got {}. Dump:\n{}",
        sp_count,
        dump
    );
    // Cleanup env
    std::env::remove_var("NYASH_REWRITE_FUTURE");
}
