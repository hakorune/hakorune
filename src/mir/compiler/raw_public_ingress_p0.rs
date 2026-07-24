//! Focused PUBLIC-INGRESS0 fixtures.

use super::MirCompiler;
use crate::ast::{ASTNode, Span};

fn empty_script() -> ASTNode {
    ASTNode::Program {
        statements: Vec::new(),
        span: Span::unknown(),
    }
}

#[test]
fn raw_public_ingress_compiles_empty_script_without_legacy_fallback() {
    let mut compiler = MirCompiler::new();
    let result = compiler
        .compile_raw_with_source(empty_script(), Some("raw-public.hako"))
        .expect("narrow Raw ingress should compile empty Script");

    assert_eq!(result.module.name, "main");
    assert!(result.module.functions.contains_key("main"));
    assert!(compiler.builder.current_module.is_none());
}

#[test]
fn raw_public_ingress_rejects_repl_before_source_binding() {
    let mut compiler = MirCompiler::new();
    compiler.set_repl_mode(true);
    let error = compiler
        .compile_raw_with_source(empty_script(), None)
        .expect_err("NarrowV1 must reject REPL mode");

    assert!(error.starts_with("[raw-public/source-binding/repl-unsupported]"));
    assert!(compiler.builder.current_module.is_none());
}
