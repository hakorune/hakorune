//! Focused COMMIT0 RawDirect preparation fixtures.

use super::module_postprocess::ModulePostprocessOwnerV1;
use super::raw_root_finalization_p0::{app, drained};
use super::raw_root_external_commit::{
    PreparedRawExternalCommitV1, RawExternalCommitFailureStageV1,
};
use super::raw_root_postprocess::RawPostprocessedInvocationV1;
use super::raw_root_postprocess::RawAppPostprocessedInvocationV1;
use super::raw_source_binding::RawCallableMainSelectionV1;
use crate::ast::{ASTNode, Span};
use crate::mir::verification::MirVerifier;
use crate::mir::builder::RawPostprocessProgressV1;

fn run(source: ASTNode, selection: RawCallableMainSelectionV1) -> RawPostprocessedInvocationV1 {
    let finalized = drained(source, selection).prepare_finalization().unwrap();
    let ready = finalized.prepare_postprocess();
    let mut verifier = MirVerifier::new();
    ModulePostprocessOwnerV1::new(&mut verifier, false)
        .run_raw_ready(ready)
        .unwrap()
}

#[test]
fn script_prepares_typed_raw_external_commit() {
    let prepared = run(
        ASTNode::Program {
            statements: Vec::new(),
            span: Span::unknown(),
        },
        RawCallableMainSelectionV1::Omitted,
    )
    .prepare_external_commit()
    .unwrap();
    assert!(matches!(
        prepared,
        PreparedRawExternalCommitV1::Script(_)
    ));
}

#[test]
fn app_not_selected_prepares_without_callable_selection() {
    let prepared = run(app(), RawCallableMainSelectionV1::Omitted)
        .prepare_external_commit()
        .unwrap();
    assert!(matches!(prepared, PreparedRawExternalCommitV1::App(_)));
}

#[test]
fn app_selected_prepares_without_opening_publication() {
    let prepared = run(app(), RawCallableMainSelectionV1::Required)
        .prepare_external_commit()
        .unwrap();
    assert!(matches!(prepared, PreparedRawExternalCommitV1::App(_)));
}

#[test]
fn crossed_route_is_rejected_with_the_complete_owner() {
    let RawPostprocessedInvocationV1::Script(script) = run(
        ASTNode::Program {
            statements: Vec::new(),
            span: Span::unknown(),
        },
        RawCallableMainSelectionV1::Omitted,
    ) else {
        panic!("expected Script postprocess owner");
    };
    let crossed = RawPostprocessedInvocationV1::App(RawAppPostprocessedInvocationV1 {
        core: script.core,
    });
    let rejected = crossed
        .prepare_external_commit()
        .expect_err("crossed route must reject before handoff");
    assert_eq!(rejected.stage(), RawExternalCommitFailureStageV1::RouteEvidence);
    rejected.discard();
}

#[test]
fn prepared_product_retains_complete_evidence() {
    let PreparedRawExternalCommitV1::Script(prepared) = run(
        ASTNode::Program {
            statements: Vec::new(),
            span: Span::unknown(),
        },
        RawCallableMainSelectionV1::Omitted,
    )
    .prepare_external_commit()
    .unwrap() else {
        panic!("expected Script prepared owner");
    };
    assert_eq!(
        prepared.core.evidence.progress,
        RawPostprocessProgressV1::ParitySealed
    );
    assert!(matches!(
        prepared.core.evidence.route,
        super::raw_root_postprocess::RawPostprocessRouteEvidenceV1::Script { .. }
    ));
}
