//! Focused DECLACCESS0-S0 fixtures.

use super::raw_root_callable_main::RawCallableMainReadyInvocationV1;
use super::raw_root_decl_access::{DeclaredRawRootInvocationV1, RawRootEnvironmentErrorV1};
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

fn ready(
    source: ASTNode,
    selection: RawCallableMainSelectionV1,
) -> RawCallableMainReadyInvocationV1 {
    let mut compiler = MirCompiler::new();
    compiler
        .bind_raw_source(
            LegacyModuleLoweringInputV1::bare_ast(source),
            None,
            "declaccess0",
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
}

#[test]
fn script_declaration_installs_environment_once() {
    let declared = ready(
        ASTNode::Program {
            statements: Vec::new(),
            span: Span::unknown(),
        },
        RawCallableMainSelectionV1::Omitted,
    )
    .declare_environment()
    .unwrap();
    assert!(matches!(declared, DeclaredRawRootInvocationV1::Script(_)));
    assert!(declared.catalog_installed());
    assert_eq!(declared.tracker_completed_children(), 0);
}

#[test]
fn app_omitted_declaration_keeps_main_unselected() {
    let declared = ready(app(), RawCallableMainSelectionV1::Omitted)
        .declare_environment()
        .unwrap();
    assert!(matches!(declared, DeclaredRawRootInvocationV1::App(_)));
    assert!(declared.catalog_installed());
    assert!(declared.app_callable_main_not_selected());
    assert_eq!(declared.tracker_completed_children(), 0);
}

#[test]
fn app_required_declaration_keeps_callable_main_evidence() {
    let declared = ready(app(), RawCallableMainSelectionV1::Required)
        .declare_environment()
        .unwrap();
    assert!(declared.catalog_installed());
    assert!(declared.app_callable_main_selected());
    assert_eq!(declared.tracker_completed_children(), 0);
}

#[test]
fn dirty_builder_rejects_before_environment_commit() {
    let rejected = ready(app(), RawCallableMainSelectionV1::Omitted)
        .dirty_builder_for_decl_access()
        .declare_environment()
        .expect_err("dirty Builder destination must reject");
    assert!(matches!(
        rejected.error(),
        RawRootEnvironmentErrorV1::Install(
            crate::mir::builder::RawRootEnvironmentInstallErrorV1::BuilderEnvironmentNotVacant
        )
    ));
    rejected.discard();
}
