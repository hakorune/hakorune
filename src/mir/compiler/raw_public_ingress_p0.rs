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

#[test]
fn raw_public_ingress_reuses_one_compiler_for_two_successes() {
    let mut compiler = MirCompiler::new();

    for source_file in [Some("raw-public-1.hako"), Some("raw-public-2.hako")] {
        let result = compiler
            .compile_raw_with_source(empty_script(), source_file)
            .expect("NarrowV1 should permit repeated empty Script compilation");
        assert_eq!(result.module.name, "main");
        assert!(compiler.builder.current_module.is_none());
    }
}

#[test]
fn raw_public_ingress_failure_is_discarded_before_reuse() {
    let mut compiler = MirCompiler::new();
    let failure = ASTNode::Program {
        statements: vec![ASTNode::Variable {
            name: "missing".into(),
            span: Span::unknown(),
        }],
        span: Span::unknown(),
    };

    let error = compiler
        .compile_raw_with_source(failure, None)
        .expect_err("undefined Raw variable should reject");
    assert!(error.starts_with("[raw-public/body/rejected]"));
    assert!(compiler.builder.current_module.is_none());

    compiler
        .compile_raw_with_source(empty_script(), None)
        .expect("discarded Raw failure must not poison compiler reuse");
    assert!(compiler.builder.current_module.is_none());
}

#[test]
fn raw_public_ingress_failure_preserves_live_imports() {
    let mut compiler = MirCompiler::new();
    compiler
        .builder
        .comp_ctx
        .using_import_boxes
        .insert("Alias".into(), "Imported".into());
    let before = compiler.builder.comp_ctx.using_import_boxes.clone();
    let failure = ASTNode::Program {
        statements: vec![ASTNode::Variable {
            name: "missing".into(),
            span: Span::unknown(),
        }],
        span: Span::unknown(),
    };

    let error = compiler
        .compile_raw_with_source(failure, None)
        .expect_err("undefined variable must reject without mutating live imports");
    assert!(error.starts_with("[raw-public/body/rejected]"));
    assert_eq!(compiler.builder.comp_ctx.using_import_boxes, before);
}
