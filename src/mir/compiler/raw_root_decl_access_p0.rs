//! Focused DECLACCESS0-S0 fixtures.

use super::raw_root_callable_main::RawCallableMainReadyInvocationV1;
use super::raw_root_decl_access::{
    DeclaredRawRootInvocationV1, RawRootBatchCompleteInvocationV1, RawRootBodyCompleteInvocationV1,
    RawRootBodyFailureStageV1, RawRootEnvironmentErrorV1,
};
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

fn script(statements: Vec<ASTNode>) -> ASTNode {
    ASTNode::Program {
        statements,
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

#[test]
fn body_entry_consumes_declared_script_into_unpublished_completion() {
    let completed = ready(
        ASTNode::Program {
            statements: Vec::new(),
            span: Span::unknown(),
        },
        RawCallableMainSelectionV1::Omitted,
    )
    .declare_environment()
    .unwrap()
    .begin_body()
    .unwrap();
    assert!(matches!(
        completed,
        RawRootBodyCompleteInvocationV1::Script(_)
    ));
}

#[test]
fn script_scalar_tail_uses_source_terminal_and_commits_body() {
    let completed = ready(
        script(vec![ASTNode::Literal {
            value: crate::ast::LiteralValue::Integer(7),
            span: Span::unknown(),
        }]),
        RawCallableMainSelectionV1::Omitted,
    )
    .declare_environment()
    .unwrap()
    .begin_body()
    .expect("source-classified scalar terminal should lower");
    assert!(matches!(
        completed,
        RawRootBodyCompleteInvocationV1::Script(_)
    ));
}

#[test]
fn script_statement_tail_commits_unit_result() {
    let completed = ready(
        script(vec![ASTNode::Print {
            expression: Box::new(ASTNode::Literal {
                value: crate::ast::LiteralValue::Integer(7),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }]),
        RawCallableMainSelectionV1::Omitted,
    )
    .declare_environment()
    .unwrap()
    .begin_body()
    .expect("statement terminal should become Unit");
    assert!(matches!(
        completed,
        RawRootBodyCompleteInvocationV1::Script(_)
    ));
}

#[test]
fn script_void_tail_reuses_evaluated_unit_operand() {
    let completed = ready(
        script(vec![ASTNode::Literal {
            value: crate::ast::LiteralValue::Void,
            span: Span::unknown(),
        }]),
        RawCallableMainSelectionV1::Omitted,
    )
    .declare_environment()
    .unwrap()
    .begin_body()
    .expect("Void expression is a Unit-valued Script terminal");
    assert!(matches!(
        completed,
        RawRootBodyCompleteInvocationV1::Script(_)
    ));
}

#[test]
fn script_prelude_value_does_not_become_terminal_result() {
    let completed = ready(
        script(vec![
            ASTNode::Literal {
                value: crate::ast::LiteralValue::Integer(1),
                span: Span::unknown(),
            },
            ASTNode::Print {
                expression: Box::new(ASTNode::Literal {
                    value: crate::ast::LiteralValue::Integer(2),
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            },
        ]),
        RawCallableMainSelectionV1::Omitted,
    )
    .declare_environment()
    .unwrap()
    .begin_body()
    .expect("prelude values are evaluated but not returned");
    assert!(matches!(
        completed,
        RawRootBodyCompleteInvocationV1::Script(_)
    ));
}

#[test]
fn root_batch_entry_consumes_body_completion_into_route_product() {
    let completed = ready(
        ASTNode::Program {
            statements: Vec::new(),
            span: Span::unknown(),
        },
        RawCallableMainSelectionV1::Omitted,
    )
    .declare_environment()
    .unwrap()
    .begin_body()
    .unwrap();
    let batched = completed
        .prepare_root_batch()
        .expect("BODY0 completion should admit the required root pair");
    assert!(matches!(
        batched,
        RawRootBatchCompleteInvocationV1::Script(_)
    ));
}

#[test]
fn body_entry_preserves_typed_lower_failure_without_retry() {
    let rejected = ready(
        ASTNode::Program {
            statements: vec![ASTNode::Variable {
                name: "missing".into(),
                span: Span::unknown(),
            }],
            span: Span::unknown(),
        },
        RawCallableMainSelectionV1::Omitted,
    )
    .declare_environment()
    .unwrap()
    .begin_body()
    .expect_err("undefined recipe variable must reject after unpublished lowering");
    assert_eq!(rejected.stage(), RawRootBodyFailureStageV1::Lower);
    rejected.discard();
}
