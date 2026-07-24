//! PARITY0-S0a: the first bounded Legacy-vs-Raw success witness.

use super::raw_public_cutover_parity_snapshot::snapshot_module;
use super::MirCompiler;
use crate::ast::{ASTNode, BinaryOperator, DeclarationAttrs, LiteralValue, Span};
use std::collections::HashMap;

fn empty_script() -> ASTNode {
    ASTNode::Program {
        statements: Vec::new(),
        span: Span::unknown(),
    }
}

fn literal_script(value: LiteralValue) -> ASTNode {
    ASTNode::Program {
        statements: vec![ASTNode::Literal {
            value,
            span: Span::unknown(),
        }],
        span: Span::unknown(),
    }
}

fn binary_script(left: LiteralValue, operator: BinaryOperator, right: LiteralValue) -> ASTNode {
    ASTNode::Program {
        statements: vec![ASTNode::BinaryOp {
            operator,
            left: Box::new(ASTNode::Literal {
                value: left,
                span: Span::unknown(),
            }),
            right: Box::new(ASTNode::Literal {
                value: right,
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }],
        span: Span::unknown(),
    }
}

fn app_with_body(body: Vec<ASTNode>) -> ASTNode {
    let main = ASTNode::FunctionDeclaration {
        name: "main".into(),
        params: Vec::new(),
        param_decls: Vec::new(),
        return_type_name: None,
        body,
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static: true,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    };
    let mut methods = HashMap::new();
    methods.insert("main".into(), main);
    ASTNode::Program {
        statements: vec![ASTNode::BoxDeclaration {
            name: "Main".into(),
            fields: Vec::new(),
            field_decls: Vec::new(),
            public_fields: Vec::new(),
            private_fields: Vec::new(),
            methods,
            constructors: HashMap::new(),
            init_fields: Vec::new(),
            weak_fields: Vec::new(),
            delegates: Vec::new(),
            invariants: Vec::new(),
            transitions: Vec::new(),
            is_interface: false,
            is_record: false,
            extends: Vec::new(),
            implements: Vec::new(),
            type_parameters: Vec::new(),
            is_sync: false,
            is_static: true,
            static_init: None,
            attrs: DeclarationAttrs::default(),
            span: Span::unknown(),
        }],
        span: Span::unknown(),
    }
}

fn empty_app() -> ASTNode {
    app_with_body(Vec::new())
}

fn scalar_app() -> ASTNode {
    app_with_body(vec![ASTNode::Literal {
        value: LiteralValue::Integer(9),
        span: Span::unknown(),
    }])
}

#[test]
fn empty_script_legacy_and_raw_have_the_same_normalized_snapshot() {
    let ast = empty_script();
    let mut legacy = MirCompiler::new();
    let legacy_result = legacy
        .compile_with_source(ast.clone(), Some("parity-empty.hako"))
        .expect("legacy empty Script should compile");
    let mut raw = MirCompiler::new();
    let raw_result = raw
        .compile_raw_with_source(ast, Some("parity-empty.hako"))
        .expect("Raw empty Script should compile");

    let legacy_snapshot = snapshot_module(&legacy_result.module)
        .expect("legacy empty Script must use only the PARITY0 snapshot dialect");
    let raw_snapshot = snapshot_module(&raw_result.module)
        .expect("Raw empty Script must use only the PARITY0 snapshot dialect");
    assert_eq!(legacy_snapshot, raw_snapshot);
}

#[test]
fn integer_literal_script_legacy_and_raw_have_the_same_normalized_snapshot() {
    let ast = literal_script(LiteralValue::Integer(7));
    let mut legacy = MirCompiler::new();
    let legacy_result = legacy
        .compile_with_source(ast.clone(), Some("parity-int.hako"))
        .expect("legacy integer Script should compile");
    let mut raw = MirCompiler::new();
    let raw_result = raw
        .compile_raw_with_source(ast, Some("parity-int.hako"))
        .expect("Raw integer Script should compile");

    let legacy_snapshot = snapshot_module(&legacy_result.module)
        .expect("legacy integer Script must use the PARITY0 snapshot dialect");
    let raw_snapshot = snapshot_module(&raw_result.module)
        .expect("Raw integer Script must use the PARITY0 snapshot dialect");
    assert_eq!(legacy_snapshot, raw_snapshot);
}

#[test]
fn string_literal_script_legacy_and_raw_have_the_same_normalized_snapshot() {
    let ast = literal_script(LiteralValue::String("raw".into()));
    let mut legacy = MirCompiler::new();
    let legacy_result = legacy
        .compile_with_source(ast.clone(), Some("parity-string.hako"))
        .expect("legacy string Script should compile");
    let mut raw = MirCompiler::new();
    let raw_result = raw
        .compile_raw_with_source(ast, Some("parity-string.hako"))
        .expect("Raw string Script should compile");
    assert_eq!(
        snapshot_module(&legacy_result.module).unwrap(),
        snapshot_module(&raw_result.module).unwrap()
    );
}

#[test]
fn integer_binary_script_legacy_and_raw_have_the_same_normalized_snapshot() {
    let ast = binary_script(
        LiteralValue::Integer(2),
        BinaryOperator::Add,
        LiteralValue::Integer(3),
    );
    let mut legacy = MirCompiler::new();
    let legacy_result = legacy
        .compile_with_source(ast.clone(), Some("parity-binary.hako"))
        .expect("legacy binary Script should compile");
    let mut raw = MirCompiler::new();
    let raw_result = raw
        .compile_raw_with_source(ast, Some("parity-binary.hako"))
        .expect("Raw binary Script should compile");
    assert_eq!(
        snapshot_module(&legacy_result.module).unwrap(),
        snapshot_module(&raw_result.module).unwrap()
    );
}

#[test]
fn empty_app_raw_root_main_keeps_fixed_void_contract() {
    let mut raw = MirCompiler::new();
    let result = raw
        .compile_raw_with_source(empty_app(), Some("parity-app.hako"))
        .expect("Raw empty App should compile");
    assert_eq!(
        result.module.functions["main"].signature.return_type,
        crate::mir::MirType::Void
    );
}

#[test]
fn scalar_app_raw_root_main_discards_tail_but_keeps_fixed_void_contract() {
    let mut raw = MirCompiler::new();
    let result = raw
        .compile_raw_with_source(scalar_app(), Some("parity-app-scalar.hako"))
        .expect("Raw scalar App should complete BODY and ROOTBATCH");
    assert_eq!(
        result.module.functions["main"].signature.return_type,
        crate::mir::MirType::Void
    );
}
