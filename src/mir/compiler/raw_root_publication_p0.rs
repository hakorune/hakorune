//! Focused PUBLICATION0 RawDirect fixtures.

use super::module_postprocess::ModulePostprocessOwnerV1;
use super::module_postprocess::ModuleVerificationEvidenceV1;
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
    let ready = drained.prepare_finalization().unwrap().prepare_postprocess();
    let mut verifier = MirVerifier::new();
    ModulePostprocessOwnerV1::new(&mut verifier, false)
        .run_raw_ready(ready)
        .unwrap()
        .prepare_external_commit()
        .unwrap()
}

fn prepared_app(selection: RawCallableMainSelectionV1) -> PreparedRawExternalCommitV1 {
    let drained = drained(app(), selection);
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

#[test]
fn raw_publication_replaces_live_builder_once() {
    let mut compiler = MirCompiler::new();
    let published = compiler.publish_raw_direct(prepared_script()).unwrap();

    assert!(matches!(published, RawPublishedInvocationV1::Script(_)));
    assert!(compiler.builder.current_module.is_some());
}

#[test]
fn raw_reportable_verifier_error_is_published_as_evidence() {
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
    let published = compiler.publish_raw_direct(prepared).unwrap();
    let RawPublishedInvocationV1::Script(script) = published else {
        panic!("expected Script published owner");
    };
    assert!(matches!(
        script.core.evidence.verification,
        ModuleVerificationEvidenceV1::Raw {
            pre_transform: Err(_)
        }
    ));
}

#[test]
fn raw_app_not_selected_publishes_as_app_product() {
    let mut compiler = MirCompiler::new();
    let published = compiler
        .publish_raw_direct(prepared_app(RawCallableMainSelectionV1::Omitted))
        .unwrap();
    assert!(matches!(published, RawPublishedInvocationV1::App(_)));
    assert!(compiler.builder.current_module.is_some());
}

#[test]
fn raw_app_selected_publishes_as_app_product() {
    let mut compiler = MirCompiler::new();
    let published = compiler
        .publish_raw_direct(prepared_app(RawCallableMainSelectionV1::Required))
        .unwrap();
    assert!(matches!(published, RawPublishedInvocationV1::App(_)));
    assert!(compiler.builder.current_module.is_some());
}

#[test]
fn non_quiescent_target_rejects_without_consuming_prepared_owner() {
    let mut compiler = MirCompiler::new();
    compiler.builder.current_module = Some(crate::mir::MirModule::new("dirty".into()));
    let rejected = compiler
        .publish_raw_direct(prepared_script())
        .expect_err("dirty live target must reject before publication");
    assert_eq!(
        rejected.stage(),
        super::raw_root_publication::RawPublicationFailureStageV1::Target
    );
    rejected.discard();
    assert_eq!(compiler.builder.current_module.as_ref().unwrap().name, "dirty");
}
