//! Focused FINAL0 direct-DRAIN handoff fixtures.

use super::raw_root_callable_main::RawAppCallableMainOutcomeV1;
use super::raw_root_drain::RawDrainedInvocationV1;
use super::raw_root_finalization::{RawFinalizationErrorV1, RawFinalizedInvocationV1};
use super::raw_source_binding::RawCallableMainSelectionV1;
use super::{LegacyModuleLoweringInputV1, MirCompiler};
use crate::ast::{ASTNode, DeclarationAttrs, Span};
use crate::mir::builder::{MirBuilder, RawRootPhysicalFinalizationErrorV1};
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

pub(super) fn app() -> ASTNode {
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

pub(super) fn drained(
    source: ASTNode,
    selection: RawCallableMainSelectionV1,
) -> RawDrainedInvocationV1 {
    let mut compiler = MirCompiler::new();
    compiler
        .bind_raw_source(
            LegacyModuleLoweringInputV1::bare_ast(source),
            None,
            "final0",
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
        .prepare_drain()
        .unwrap()
        .drain()
}

#[test]
fn empty_script_finalizes_directly_from_drain() {
    let finalized = drained(
        ASTNode::Program {
            statements: Vec::new(),
            span: Span::unknown(),
        },
        RawCallableMainSelectionV1::Omitted,
    )
    .prepare_finalization()
    .unwrap();
    assert!(matches!(finalized, RawFinalizedInvocationV1::Script(_)));
}

#[test]
fn app_not_selected_finalizes_without_callable_row() {
    let finalized = drained(app(), RawCallableMainSelectionV1::Omitted)
        .prepare_finalization()
        .unwrap();
    assert!(matches!(finalized, RawFinalizedInvocationV1::App(_)));
}

#[test]
fn app_selected_finalizes_with_callable_evidence() {
    let finalized = drained(app(), RawCallableMainSelectionV1::Required)
        .prepare_finalization()
        .unwrap();
    let RawFinalizedInvocationV1::App(app) = finalized else {
        panic!("expected App finalization");
    };
    assert!(matches!(
        app.callable_main,
        RawAppCallableMainOutcomeV1::Selected { .. }
    ));
}

#[test]
fn module_name_drift_rejects_and_retains_the_drained_owner() {
    let RawDrainedInvocationV1::Script(mut script) = drained(
        ASTNode::Program {
            statements: Vec::new(),
            span: Span::unknown(),
        },
        RawCallableMainSelectionV1::Omitted,
    ) else {
        panic!("expected Script drained owner");
    };
    script.core.module_name = "drifted".into();

    let rejected = RawDrainedInvocationV1::Script(script)
        .prepare_finalization()
        .expect_err("module-name drift must reject before physical commit");
    assert!(matches!(
        rejected.error(),
        RawFinalizationErrorV1::Physical(
            RawRootPhysicalFinalizationErrorV1::ModuleNameMismatch { .. }
        )
    ));
    rejected.discard();
}
