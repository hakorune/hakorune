//! Focused POST-CARRIER handoff fixtures.

use super::module_postprocess::ModulePostprocessOwnerV1;
use super::raw_root_finalization_p0::{app, drained};
use super::raw_root_postprocess::RawPostprocessedInvocationV1;
use super::raw_source_binding::RawCallableMainSelectionV1;
use crate::ast::{ASTNode, Span};
use crate::mir::builder::RawPostprocessProgressV1;
use crate::mir::verification::MirVerifier;

fn run(source: ASTNode, selection: RawCallableMainSelectionV1) -> RawPostprocessedInvocationV1 {
    let finalized = drained(source, selection).prepare_finalization().unwrap();
    let ready = finalized.prepare_postprocess();
    let mut verifier = MirVerifier::new();
    ModulePostprocessOwnerV1::new(&mut verifier, false)
        .run_raw_ready(ready)
        .unwrap()
}

#[test]
fn empty_script_runs_through_shared_raw_postprocess_kernel() {
    let processed = run(
        ASTNode::Program {
            statements: Vec::new(),
            span: Span::unknown(),
        },
        RawCallableMainSelectionV1::Omitted,
    );
    let RawPostprocessedInvocationV1::Script(script) = processed else {
        panic!("expected Script postprocess owner");
    };
    assert_eq!(
        script.core.physical.progress(),
        RawPostprocessProgressV1::ParitySealed
    );
}

#[test]
fn app_not_selected_keeps_app_route_after_postprocess() {
    let processed = run(app(), RawCallableMainSelectionV1::Omitted);
    assert!(matches!(processed, RawPostprocessedInvocationV1::App(_)));
}

#[test]
fn app_selected_keeps_callable_main_route_evidence() {
    let processed = run(app(), RawCallableMainSelectionV1::Required);
    let RawPostprocessedInvocationV1::App(app) = processed else {
        panic!("expected App postprocess owner");
    };
    let super::raw_root_postprocess::RawPostprocessRouteEvidenceV1::App { callable_main, .. } =
        app.core.stage_evidence.route
    else {
        panic!("expected App route evidence");
    };
    assert!(matches!(
        callable_main,
        super::raw_root_callable_main::RawAppCallableMainOutcomeV1::Selected { .. }
    ));
}
