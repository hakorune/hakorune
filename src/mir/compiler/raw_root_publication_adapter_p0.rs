//! Focused PUBLICATION-ADAPTER0 fixtures.

use super::module_postprocess::{ModulePostprocessOwnerV1, ModuleVerificationEvidenceV1};
use super::raw_root_external_commit::PreparedRawExternalCommitV1;
use super::raw_root_finalization_p0::{app, drained};
use super::raw_root_publication::RawPublishedInvocationV1;
use super::raw_source_binding::RawCallableMainSelectionV1;
use super::MirCompiler;
use crate::ast::{ASTNode, Span};
use crate::mir::verification::MirVerifier;

fn prepared_script() -> PreparedRawExternalCommitV1 {
    let drained = drained(
        ASTNode::Program {
            statements: Vec::new(),
            span: Span::unknown(),
        },
        RawCallableMainSelectionV1::Omitted,
    );
    let ready = drained
        .prepare_finalization()
        .unwrap()
        .prepare_postprocess();
    let mut verifier = MirVerifier::new();
    ModulePostprocessOwnerV1::new(&mut verifier, false)
        .run_raw_ready(ready)
        .unwrap()
        .prepare_external_commit()
        .unwrap()
}

fn prepared_app(selection: RawCallableMainSelectionV1) -> PreparedRawExternalCommitV1 {
    let ready = drained(app(), selection)
        .prepare_finalization()
        .unwrap()
        .prepare_postprocess();
    let mut verifier = MirVerifier::new();
    ModulePostprocessOwnerV1::new(&mut verifier, false)
        .run_raw_ready(ready)
        .unwrap()
        .prepare_external_commit()
        .unwrap()
}

fn adapter_result(
    compiler: &mut MirCompiler,
    prepared: PreparedRawExternalCommitV1,
) -> super::MirCompileResult {
    compiler
        .publish_raw_direct(prepared)
        .unwrap()
        .into_compatibility_envelope()
        .into_compatibility()
}

#[test]
fn raw_adapter_owns_published_script_module_once() {
    let mut compiler = MirCompiler::new();
    let result = adapter_result(&mut compiler, prepared_script());

    assert_eq!(result.module.name, "final0");
    assert!(result.module.functions.contains_key("main"));
    assert!(compiler.builder.current_module.is_none());
}

#[test]
fn raw_adapter_preserves_app_route_products() {
    let mut compiler = MirCompiler::new();
    let result = adapter_result(
        &mut compiler,
        prepared_app(RawCallableMainSelectionV1::Omitted),
    );

    assert_eq!(result.module.name, "final0");
    assert!(result.module.functions.contains_key("main"));
}

#[test]
fn raw_adapter_accepts_selected_app_fixture() {
    let mut compiler = MirCompiler::new();
    let result = adapter_result(
        &mut compiler,
        prepared_app(RawCallableMainSelectionV1::Required),
    );

    assert!(result.module.functions.contains_key("main"));
}

#[test]
fn raw_adapter_moves_reportable_verifier_errors_once() {
    let mut prepared = prepared_script();
    let super::raw_root_external_commit::PreparedRawExternalCommitV1::Script(script) =
        &mut prepared
    else {
        panic!("expected Script prepared owner");
    };
    script.core.evidence.verification = ModuleVerificationEvidenceV1::Raw {
        pre_transform: Err(Vec::new().into_boxed_slice()),
    };

    let mut compiler = MirCompiler::new();
    let result = adapter_result(&mut compiler, prepared);
    assert!(result.verification_result.is_err());
    assert!(compiler.builder.current_module.is_none());
}

#[test]
fn raw_adapter_is_the_only_consuming_result_boundary() {
    let mut compiler = MirCompiler::new();
    let published = compiler.publish_raw_direct(prepared_script()).unwrap();
    let envelope = published.into_compatibility_envelope();
    let result = envelope.into_compatibility();
    assert_eq!(result.module.name, "final0");
}

#[allow(dead_code)]
fn _assert_published_route_is_typed(value: RawPublishedInvocationV1) {
    match value {
        RawPublishedInvocationV1::Script(_) | RawPublishedInvocationV1::App(_) => {}
    }
}
