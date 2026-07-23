use super::raw_draft_invocation::RawDraftInvocationErrorV1;
use super::RawDraftInvocationV1;
use crate::ast::{ASTNode, DeclarationAttrs, Span};
use crate::mir::compiler::{LegacyModuleLoweringInputV1, MirCompiler};
use crate::mir::compiler::raw_source_binding::RawCallableMainSelectionV1;
use std::collections::HashMap;

fn function(name: &str) -> ASTNode {
    ASTNode::FunctionDeclaration {
        name: name.into(),
        params: Vec::new(),
        param_decls: Vec::new(),
        return_type_name: None,
        body: Vec::new(),
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static: true,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    }
}

fn app_with_static_child() -> ASTNode {
    let mut methods = HashMap::new();
    methods.insert("main".into(), function("main"));
    methods.insert("helper".into(), function("helper"));
    ASTNode::Program {
        statements: vec![ASTNode::BoxDeclaration {
            name: "Main".into(),
            methods,
            is_static: true,
            fields: Vec::new(),
            field_decls: Vec::new(),
            public_fields: Vec::new(),
            private_fields: Vec::new(),
            constructors: HashMap::new(),
            init_fields: Vec::new(),
            weak_fields: Vec::new(),
            delegates: Vec::new(),
            invariants: Vec::new(),
            transitions: Vec::new(),
            is_interface: false,
            is_sync: false,
            is_record: false,
            type_parameters: Vec::new(),
            extends: Vec::new(),
            implements: Vec::new(),
            static_init: None,
            attrs: DeclarationAttrs::default(),
            span: Span::unknown(),
        }],
        span: Span::unknown(),
    }
}

#[test]
fn raw_s0_child_uses_one_source_to_collector_to_ledger_chain() {
    let mut compiler = MirCompiler::new();
    let package = compiler
        .bind_raw_source(
            LegacyModuleLoweringInputV1::bare_ast(app_with_static_child()),
            Some("raw-s0.hako"),
            "raw-s0",
            RawCallableMainSelectionV1::Omitted,
        )
        .unwrap();
    let owner = compiler.begin_raw_draft(package);
    let brand = owner.brand();
    let step = owner.lower_first_static_child().unwrap();
    let (owner, receipt) = step.into_parts();
    assert_eq!(receipt.brand(), brand);
    assert_eq!(receipt.symbol(), "Main.helper/0");
    assert_eq!(owner.brand(), brand);
}

#[test]
fn raw_s0_missing_child_rejects_without_a_lowering_fallback() {
    let mut compiler = MirCompiler::new();
    let package = compiler
        .bind_raw_source(
            LegacyModuleLoweringInputV1::bare_ast(ASTNode::Program {
                statements: Vec::new(),
                span: Span::unknown(),
            }),
            None,
            "raw-s0-empty",
            RawCallableMainSelectionV1::Omitted,
        )
        .unwrap();
    let owner = compiler.begin_raw_draft(package);
    let rejected = owner.lower_first_static_child().unwrap_err();
    assert!(matches!(
        rejected.error(),
        RawDraftInvocationErrorV1::NoStaticChild
    ));
    rejected.discard();
}
