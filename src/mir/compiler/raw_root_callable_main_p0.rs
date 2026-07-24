//! Focused CALLMAIN0-S0 fixtures.

use super::raw_root_callable_main::{
    RawCallableMainFailureStageV1, RawCallableMainReadyInvocationV1,
};
use super::raw_root_eligibility::RawRootInvocationV1;
use super::raw_source_binding::RawCallableMainSelectionV1;
use super::{LegacyModuleLoweringInputV1, MirCompiler};
use crate::ast::{ASTNode, DeclarationAttrs, Span};
use crate::mir::builder::MirBuilder;
use std::collections::HashMap;

fn function(name: &str, body: Vec<ASTNode>) -> ASTNode {
    ASTNode::FunctionDeclaration {
        name: name.into(),
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
    }
}

fn app(main_body: Vec<ASTNode>, helpers: &[&str]) -> ASTNode {
    let mut methods = HashMap::new();
    methods.insert("main".into(), function("main", main_body));
    for helper in helpers {
        methods.insert((*helper).into(), function(helper, Vec::new()));
    }
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

fn bound(source: ASTNode, selection: RawCallableMainSelectionV1) -> RawRootInvocationV1 {
    let mut compiler = MirCompiler::new();
    compiler
        .bind_raw_source(
            LegacyModuleLoweringInputV1::bare_ast(source),
            None,
            "callmain0",
            selection,
        )
        .unwrap()
        .into_root_package()
        .unwrap()
        .prepare_eligibility()
        .unwrap()
        .open_physical(&MirBuilder::new())
        .unwrap()
}

fn finish(
    source: ASTNode,
    selection: RawCallableMainSelectionV1,
) -> RawCallableMainReadyInvocationV1 {
    bound(source, selection)
        .prepare_children()
        .unwrap()
        .complete_all()
        .unwrap()
        .finish_callable_main()
        .unwrap()
}

#[test]
fn app_not_selected_does_not_reserve_or_emit_callable_receipt() {
    let ready = finish(app(Vec::new(), &[]), RawCallableMainSelectionV1::Omitted);
    let outcome = ready.app_outcome().expect("App ready outcome");
    assert!(!outcome.is_selected());
    assert_eq!(outcome.locator().method_name(), "main");
    assert!(outcome.selected_receipt().is_none());
    assert_eq!(ready.tracker_completed_children(), 0);
}

#[test]
fn app_selected_uses_callable_main_role_and_same_brand() {
    let ready = finish(app(Vec::new(), &[]), RawCallableMainSelectionV1::Required);
    let outcome = ready.app_outcome().expect("App ready outcome");
    let receipt = outcome.selected_receipt().expect("selected receipt");
    assert!(outcome.is_selected());
    assert_eq!(receipt.locator().method_name(), "main");
    assert_eq!(receipt.locator().arity(), 0);
    assert_eq!(
        receipt.role(),
        super::raw_root_callable_main::RawCallableMainRoleV1::CallableMainCompatibility
    );
    assert_eq!(receipt.receipt_brand(), ready.physical_brand());
    assert_eq!(receipt.receipt_brand(), ready.session_brand());
    assert_eq!(receipt.receipt_brand(), ready.token_brand());
    assert_eq!(ready.tracker_completed_children(), 0);
}

#[test]
fn selected_failure_retains_prefix_and_blocks_body_entry() {
    let source = app(
        vec![ASTNode::Variable {
            name: "missing".into(),
            span: Span::unknown(),
        }],
        &["alpha"],
    );
    let rejected = bound(source, RawCallableMainSelectionV1::Required)
        .prepare_children()
        .unwrap()
        .complete_all()
        .unwrap()
        .finish_callable_main()
        .unwrap_err();
    assert_eq!(rejected.stage(), RawCallableMainFailureStageV1::Physical);
    assert_eq!(rejected.helper_receipt_count(), 1);
    assert_eq!(rejected.failed_locator().unwrap().method_name(), "main");
    assert!(matches!(
        rejected.error(),
        super::raw_root_callable_main::RawCallableMainErrorV1::Physical(_)
    ));
    rejected.discard();
}
