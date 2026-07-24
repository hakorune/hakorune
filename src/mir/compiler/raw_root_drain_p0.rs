//! Focused DRAIN0-S0 route and one-shot fixtures.

use super::raw_root_decl_access::RawRootBatchCompleteInvocationV1;
use super::raw_source_binding::RawCallableMainSelectionV1;
use super::{LegacyModuleLoweringInputV1, MirCompiler};
use crate::ast::{ASTNode, DeclarationAttrs, Span};
use crate::mir::builder::MirBuilder;
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

fn app() -> ASTNode {
    let mut methods = HashMap::new();
    methods.insert("main".into(), function("main"));
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

fn batch(
    source: ASTNode,
    selection: RawCallableMainSelectionV1,
) -> RawRootBatchCompleteInvocationV1 {
    let mut compiler = MirCompiler::new();
    compiler
        .bind_raw_source(
            LegacyModuleLoweringInputV1::bare_ast(source),
            None,
            "drain0",
            selection,
        )
        .unwrap()
        .into_root_package()
        .unwrap()
        .prepare_eligibility()
        .unwrap()
        .open_physical(&MirBuilder::new())
        .unwrap()
        .prepare_children()
        .unwrap()
        .complete_all()
        .unwrap()
        .finish_callable_main()
        .unwrap()
        .declare_environment()
        .unwrap()
        .begin_body()
        .unwrap()
        .prepare_root_batch()
        .unwrap()
}

#[test]
fn empty_script_drains_as_script_product() {
    let drained = batch(
        ASTNode::Program {
            statements: Vec::new(),
            span: Span::unknown(),
        },
        RawCallableMainSelectionV1::Omitted,
    )
    .prepare_drain()
    .unwrap()
    .drain();
    assert!(matches!(
        drained,
        super::raw_root_drain::RawDrainedInvocationV1::Script(_)
    ));
}

#[test]
fn app_omitted_drains_without_callable_main_row() {
    let drained = batch(app(), RawCallableMainSelectionV1::Omitted)
        .prepare_drain()
        .unwrap()
        .drain();
    assert!(matches!(
        drained,
        super::raw_root_drain::RawDrainedInvocationV1::App(_)
    ));
}

#[test]
fn app_selected_drains_with_callable_main_evidence() {
    let drained = batch(app(), RawCallableMainSelectionV1::Required)
        .prepare_drain()
        .unwrap()
        .drain();
    assert!(matches!(
        drained,
        super::raw_root_drain::RawDrainedInvocationV1::App(_)
    ));
}
