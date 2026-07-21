//! FACTSESSION0-P0-S0 disconnected lifecycle observation fixtures.
//!
//! The fact-session harness and the existing Builder session run beside one
//! another here. This is intentionally not a production bridge: the test
//! proves their current ordering surfaces without claiming that Builder facts
//! have moved into `ModuleFactSessionV1`.

use super::calls::FunctionSessionP0TerminalV1;
use super::fact_session::p0_test_support::FactSessionP0HarnessV1;
use super::fact_session::{FactSessionIssuerV1, FunctionFactGenerationV1};
use super::MirBuilder;
use crate::ast::LiteralValue;
use crate::mir::{
    BasicBlockId, EffectMask, FunctionSignature, MirFunction, MirModule, MirType, ValueId,
};
use std::panic::{catch_unwind, AssertUnwindSafe};

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

fn seeded_outer_builder() -> MirBuilder {
    let mut builder = MirBuilder::new();
    builder.current_module = Some(MirModule::new("fact-session-p0".to_string()));
    builder.enter_function_for_test("outer/0".to_string());
    builder
}

fn assert_outer_restored(builder: &MirBuilder) {
    assert_eq!(
        builder
            .function_state
            .current_function
            .as_ref()
            .unwrap()
            .signature
            .name,
        "outer/0"
    );
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
fn p0_harness_covers_main_synthetic_and_test_adapter_drafts() {
    let mut issuer = FactSessionIssuerV1::default();
    let mut harness = FactSessionP0HarnessV1::open(&mut issuer).unwrap();

    let main = harness
        .collect_success(draft("main/0"), ValueId::new(1))
        .unwrap();
    let synthetic = harness
        .collect_success(draft("condition_fn/0"), ValueId::new(1))
        .unwrap();
    let adapter = harness
        .collect_success(draft("test-adapter/0"), ValueId::new(1))
        .unwrap();

    assert_eq!(main, FunctionFactGenerationV1::for_test(0, 0));
    assert_eq!(synthetic, FunctionFactGenerationV1::for_test(0, 1));
    assert_eq!(adapter, FunctionFactGenerationV1::for_test(0, 2));
    assert_eq!(harness.completed_count(), 3);
    assert_eq!(harness.completed_lane_counts(), vec![[1; 8]; 3]);
}

#[test]
fn p0_harness_observes_existing_child_restore_without_claiming_fact_transport() {
    let mut builder = seeded_outer_builder();

    let mut issuer = FactSessionIssuerV1::default();
    let mut harness = FactSessionP0HarnessV1::open(&mut issuer).unwrap();
    let attempt = harness.prepare_seeded_attempt(ValueId::new(1)).unwrap();
    builder
        .observe_function_terminal_before_restore_for_p0_test(
            "child/0",
            |_child| Ok(draft("child/0")),
            |terminal, current_before_restore| {
                assert_eq!(current_before_restore, None);
                match terminal {
                    FunctionSessionP0TerminalV1::Success(draft) => attempt
                        .collect_success(draft)
                        .map_err(|error| format!("{error:?}")),
                    FunctionSessionP0TerminalV1::Primary(error) => {
                        panic!("unexpected child error: {error}")
                    }
                    FunctionSessionP0TerminalV1::Cleanup(error) => {
                        panic!("unexpected child cleanup error: {error}")
                    }
                    FunctionSessionP0TerminalV1::Panicked => panic!("unexpected child panic"),
                }
            },
        )
        .unwrap();
    assert_outer_restored(&builder);
    assert_eq!(harness.completed_count(), 1);
}

#[test]
fn p0_harness_aborts_after_child_error_before_observed_parent_restore() {
    let mut builder = seeded_outer_builder();
    let mut issuer = FactSessionIssuerV1::default();
    let mut harness = FactSessionP0HarnessV1::open(&mut issuer).unwrap();

    let attempt = harness.prepare_seeded_attempt(ValueId::new(1)).unwrap();
    let error = builder
        .observe_function_terminal_before_restore_for_p0_test(
            "child-error/0",
            |_child| Err("fixture child error".to_string()),
            |terminal, current_before_restore| {
                assert_eq!(current_before_restore, None);
                match terminal {
                    FunctionSessionP0TerminalV1::Primary(error) => {
                        attempt.abort();
                        Err(error)
                    }
                    FunctionSessionP0TerminalV1::Success(_) => {
                        panic!("unexpected child success")
                    }
                    FunctionSessionP0TerminalV1::Cleanup(error) => {
                        panic!("unexpected child cleanup error: {error}")
                    }
                    FunctionSessionP0TerminalV1::Panicked => panic!("unexpected child panic"),
                }
            },
        )
        .unwrap_err();

    assert!(error.contains("fixture child error"));
    assert_outer_restored(&builder);
    assert_eq!(harness.completed_count(), 0);
}

#[test]
fn p0_harness_aborts_after_child_cleanup_failure_before_parent_restore() {
    let mut builder = seeded_outer_builder();
    let mut issuer = FactSessionIssuerV1::default();
    let mut harness = FactSessionP0HarnessV1::open(&mut issuer).unwrap();

    let attempt = harness.prepare_seeded_attempt(ValueId::new(1)).unwrap();
    let error = builder
        .observe_function_terminal_before_restore_for_p0_test(
            "child-cleanup/0",
            |child| {
                child.enter_function_for_test("unbalanced-child/0".to_string());
                Ok(draft("child-cleanup/0"))
            },
            |terminal, current_before_restore| {
                assert_eq!(
                    current_before_restore.as_deref(),
                    Some("unbalanced-child/0")
                );
                match terminal {
                    FunctionSessionP0TerminalV1::Cleanup(error) => {
                        attempt.abort();
                        Err(error)
                    }
                    FunctionSessionP0TerminalV1::Success(_) => {
                        panic!("unexpected child success")
                    }
                    FunctionSessionP0TerminalV1::Primary(error) => {
                        panic!("unexpected child error: {error}")
                    }
                    FunctionSessionP0TerminalV1::Panicked => panic!("unexpected child panic"),
                }
            },
        )
        .unwrap_err();

    assert!(error.contains("published_draft_still_installed"));
    assert_outer_restored(&builder);
    assert_eq!(harness.completed_count(), 0);
}

#[test]
fn p0_harness_aborts_before_resuming_child_panic_and_keeps_siblings_distinct() {
    let mut builder = seeded_outer_builder();
    let mut issuer = FactSessionIssuerV1::default();
    let mut harness = FactSessionP0HarnessV1::open(&mut issuer).unwrap();

    let attempt = harness.prepare_seeded_attempt(ValueId::new(1)).unwrap();
    let panic = catch_unwind(AssertUnwindSafe(|| {
        let _ = builder.observe_function_terminal_before_restore_for_p0_test(
            "child-panic/0",
            |_child| panic!("fixture child panic"),
            |terminal, current_before_restore| {
                assert_eq!(current_before_restore, None);
                match terminal {
                    FunctionSessionP0TerminalV1::Panicked => {
                        attempt.abort();
                        Ok(())
                    }
                    FunctionSessionP0TerminalV1::Success(_) => {
                        panic!("unexpected child success")
                    }
                    FunctionSessionP0TerminalV1::Primary(error) => {
                        panic!("unexpected child error: {error}")
                    }
                    FunctionSessionP0TerminalV1::Cleanup(error) => {
                        panic!("unexpected child cleanup error: {error}")
                    }
                }
            },
        );
    }));
    assert!(panic.is_err());
    assert_outer_restored(&builder);
    assert_eq!(harness.completed_count(), 0);

    let first = harness
        .collect_success(draft("sibling-a/0"), ValueId::new(1))
        .unwrap();
    let second = harness
        .collect_success(draft("sibling-b/0"), ValueId::new(1))
        .unwrap();
    assert_ne!(
        first, second,
        "reused local ValueId needs fresh generations"
    );
    assert_eq!(harness.completed_count(), 2);
}

#[test]
fn p0_observes_legacy_prepare_module_reuse_without_claiming_isolation() {
    let mut builder = MirBuilder::new();
    builder.prepare_module().expect("first candidate opens");
    let first = builder.build_literal(LiteralValue::Integer(7)).unwrap();
    assert_eq!(builder.value_type(first), Some(&MirType::Integer));

    builder
        .prepare_module()
        .expect("second candidate entry remains legacy-successful");
    let reused = builder.alloc_value_for_test();

    assert_eq!(
        reused, first,
        "the legacy function allocator reuses the local value identity"
    );
    assert_eq!(
        builder.value_type(reused),
        Some(&MirType::Integer),
        "P0 records the current stale-fact baseline; FACTSESSION0-I0 owns its removal"
    );
}
