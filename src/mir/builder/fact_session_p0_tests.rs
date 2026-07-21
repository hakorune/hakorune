//! FACTSESSION0-P0-S0 disconnected lifecycle observation fixtures.
//!
//! The fact-session harness and the existing Builder session run beside one
//! another here. This is intentionally not a production bridge: the test
//! proves their current ordering surfaces without claiming that Builder facts
//! have moved into `ModuleFactSessionV1`.

use super::fact_session::p0_test_support::FactSessionP0HarnessV1;
use super::fact_session::FactSessionIssuerV1;
use super::MirBuilder;
use crate::mir::{
    BasicBlockId, EffectMask, FunctionSignature, MirFunction, MirModule, MirType, ValueId,
};

fn draft(name: &str) -> MirFunction {
    MirFunction::new(
        FunctionSignature {
            name: name.to_string(),
            params: Vec::new(),
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    )
}

#[test]
fn p0_harness_collects_success_and_consumes_abort_without_builder_connection() {
    let mut issuer = FactSessionIssuerV1::default();
    let mut harness = FactSessionP0HarnessV1::open(&mut issuer).unwrap();

    let completed = harness
        .collect_success(draft("p0/main/0"), ValueId::new(1))
        .unwrap();
    let aborted = harness.abort_seeded(ValueId::new(1)).unwrap();

    assert_ne!(completed, aborted);
    assert_eq!(harness.completed_count(), 1);
}

#[test]
fn p0_harness_observes_existing_child_restore_without_claiming_fact_transport() {
    let mut builder = MirBuilder::new();
    builder.current_module = Some(MirModule::new("fact-session-p0".to_string()));
    builder.enter_function_for_test("outer/0".to_string());

    let outer_name = builder
        .function_state
        .current_function
        .as_ref()
        .unwrap()
        .signature
        .name
        .clone();
    builder
        .observe_function_restore_for_p0_test("child/0", |_child| Ok(draft("child/0")))
        .unwrap();
    assert_eq!(
        builder
            .function_state
            .current_function
            .as_ref()
            .unwrap()
            .signature
            .name,
        outer_name
    );

    let mut issuer = FactSessionIssuerV1::default();
    let mut harness = FactSessionP0HarnessV1::open(&mut issuer).unwrap();
    harness
        .collect_success(draft("p0/observed-child/0"), ValueId::new(1))
        .unwrap();
    assert_eq!(harness.completed_count(), 1);
}
