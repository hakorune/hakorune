//! Design-stop protocol evidence for the resolved Generic carrier handoff.
//!
//! Every type in this file is private to `cfg(test)`.  This is deliberately a
//! protocol model, not a production selector or capability implementation. It
//! freezes the facts/capability pairing boundary before a production owner is
//! authorized by the handoff design card.

use super::generic_resolved_carrier_projector_tests::{
    issue_projector_handoff_for_test, ProjectorHandoffObservationV1, ProjectorRejectV1,
    NESTED_IF_SOURCE, SOURCE,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TestRouteV1 {
    V0,
    V1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TestCarrierDispositionV1 {
    CompleteRecursiveCarrier,
    NoRecursive,
    Unavailable,
    Ambiguous,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TestShapeV1 {
    Supported,
    NestedWrapper,
    DuplicateWrite,
    Index,
    Program,
    CompoundAssignment,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TestBindingRelationV1 {
    Exact,
    Shadowing,
    OwnerMismatch,
    FrameMismatch,
    SourceMismatch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TestLegacyDispositionV1 {
    NotApplicable,
    ProvenOutsideTarget,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TestFactsSnapshotV1 {
    source_id: u16,
    owner_id: u16,
    facts_id: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TestPreflightSeedV1 {
    source_id: u16,
    owner_id: u16,
    facts_id: u16,
    frame_id: u16,
    strict_or_dev: bool,
    planner_required: bool,
    base_schedule: Vec<TestRouteV1>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TestEligibilityV1 {
    source_id: u16,
    owner_id: u16,
    frame_id: u16,
    write_binding: u16,
    read_binding: u16,
    carrier: TestCarrierDispositionV1,
    shape: TestShapeV1,
}

#[derive(Debug, PartialEq, Eq)]
struct TestInvocationSealV1 {
    invocation_id: u64,
    source_id: u16,
    owner_id: u16,
    facts_id: u16,
    frame_id: u16,
}

impl TestInvocationSealV1 {
    fn matches(
        &self,
        facts: TestFactsSnapshotV1,
        eligibility: TestEligibilityV1,
        seed: &TestPreflightSeedV1,
    ) -> bool {
        self.source_id == facts.source_id
            && self.source_id == eligibility.source_id
            && self.source_id == seed.source_id
            && self.owner_id == facts.owner_id
            && self.owner_id == eligibility.owner_id
            && self.owner_id == seed.owner_id
            && self.facts_id == facts.facts_id
            && self.facts_id == seed.facts_id
            && self.frame_id == eligibility.frame_id
            && self.frame_id == seed.frame_id
    }
}

/// The intended production shape: no public constructor, no parts accessor,
/// and no Clone.  Facts and eligibility therefore cannot be re-paired after
/// the source invocation has been sealed.
#[derive(Debug, PartialEq, Eq)]
struct TestResolvedCarrierSelectionInputV1 {
    facts: TestFactsSnapshotV1,
    eligibility: TestEligibilityV1,
    seed: TestPreflightSeedV1,
    invocation: TestInvocationSealV1,
}

#[derive(Debug, PartialEq, Eq)]
enum TestSelectionInputV1 {
    Legacy(TestLegacyDispositionV1),
    Resolved(TestResolvedCarrierSelectionInputV1),
}

/// Source-backed bridge input. The actual resolver/projector witness is kept
/// beside the synthetic protocol policy; no typed source identity is cast into
/// the policy's test-only integer fields.
#[derive(Debug)]
struct TestSourceBackedSelectionInputV1 {
    projector: ProjectorHandoffObservationV1,
    policy: TestSelectionInputV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TestSourceBackedBridgeErrorV1 {
    Projector(ProjectorRejectV1),
    Handoff(TestHandoffErrorV1),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TestHandoffErrorV1 {
    UnresolvedStop,
}

#[derive(Debug, PartialEq, Eq)]
struct TestSelectionReceiptV1 {
    selected: TestRouteV1,
    base_schedule: Vec<TestRouteV1>,
    v0_attempts: u8,
    strict_or_dev: bool,
    invocation_id: u64,
}

fn issue_resolved(
    facts: TestFactsSnapshotV1,
    eligibility: TestEligibilityV1,
    seed: TestPreflightSeedV1,
    invocation_id: u64,
) -> Result<TestResolvedCarrierSelectionInputV1, TestHandoffErrorV1> {
    let identity_matches = facts.source_id == eligibility.source_id
        && facts.source_id == seed.source_id
        && facts.owner_id == eligibility.owner_id
        && facts.owner_id == seed.owner_id
        && facts.facts_id == seed.facts_id
        && eligibility.frame_id == seed.frame_id;
    if !identity_matches {
        return Err(TestHandoffErrorV1::UnresolvedStop);
    }

    Ok(TestResolvedCarrierSelectionInputV1 {
        facts,
        eligibility,
        seed: seed.clone(),
        invocation: TestInvocationSealV1 {
            invocation_id,
            source_id: facts.source_id,
            owner_id: facts.owner_id,
            facts_id: facts.facts_id,
            frame_id: seed.frame_id,
        },
    })
}

fn select(input: TestSelectionInputV1) -> Result<TestSelectionReceiptV1, TestHandoffErrorV1> {
    match input {
        TestSelectionInputV1::Legacy(disposition) => match disposition {
            TestLegacyDispositionV1::NotApplicable
            | TestLegacyDispositionV1::ProvenOutsideTarget => Ok(TestSelectionReceiptV1 {
                selected: TestRouteV1::V0,
                base_schedule: Vec::new(),
                v0_attempts: 1,
                strict_or_dev: false,
                invocation_id: 0,
            }),
        },
        TestSelectionInputV1::Resolved(input) => {
            if !input
                .invocation
                .matches(input.facts, input.eligibility, &input.seed)
            {
                return Err(TestHandoffErrorV1::UnresolvedStop);
            }
            let eligible = input.eligibility.write_binding == input.eligibility.read_binding
                && matches!(
                    input.eligibility.carrier,
                    TestCarrierDispositionV1::CompleteRecursiveCarrier
                )
                && matches!(input.eligibility.shape, TestShapeV1::Supported)
                && input.seed.base_schedule == [TestRouteV1::V0, TestRouteV1::V1]
                && !input.seed.planner_required;
            if !eligible {
                return Err(TestHandoffErrorV1::UnresolvedStop);
            }
            Ok(TestSelectionReceiptV1 {
                selected: TestRouteV1::V1,
                base_schedule: input.seed.base_schedule,
                v0_attempts: 0,
                strict_or_dev: input.seed.strict_or_dev,
                invocation_id: input.invocation.invocation_id,
            })
        }
    }
}

fn source_backed_input(
    strict_or_dev: bool,
) -> Result<TestSourceBackedSelectionInputV1, TestSourceBackedBridgeErrorV1> {
    let projector = issue_projector_handoff_for_test(NESTED_IF_SOURCE)
        .map_err(TestSourceBackedBridgeErrorV1::Projector)?;
    if !projector.is_natural_both() {
        return Err(TestSourceBackedBridgeErrorV1::Handoff(
            TestHandoffErrorV1::UnresolvedStop,
        ));
    }
    Ok(TestSourceBackedSelectionInputV1 {
        projector,
        policy: natural_input(strict_or_dev),
    })
}

fn select_source_backed(
    input: TestSourceBackedSelectionInputV1,
) -> Result<TestSelectionReceiptV1, TestSourceBackedBridgeErrorV1> {
    if !input.projector.is_natural_both() {
        return Err(TestSourceBackedBridgeErrorV1::Handoff(
            TestHandoffErrorV1::UnresolvedStop,
        ));
    }
    select(input.policy).map_err(TestSourceBackedBridgeErrorV1::Handoff)
}

fn natural_input(strict_or_dev: bool) -> TestSelectionInputV1 {
    let facts = TestFactsSnapshotV1 {
        source_id: 7,
        owner_id: 5,
        facts_id: 11,
    };
    let eligibility = TestEligibilityV1 {
        source_id: 7,
        owner_id: 5,
        frame_id: 13,
        write_binding: 17,
        read_binding: 17,
        carrier: TestCarrierDispositionV1::CompleteRecursiveCarrier,
        shape: TestShapeV1::Supported,
    };
    let seed = TestPreflightSeedV1 {
        source_id: 7,
        owner_id: 5,
        facts_id: 11,
        frame_id: 13,
        strict_or_dev,
        planner_required: false,
        base_schedule: vec![TestRouteV1::V0, TestRouteV1::V1],
    };
    TestSelectionInputV1::Resolved(
        issue_resolved(facts, eligibility, seed, u64::from(strict_or_dev))
            .expect("natural Both handoff must seal"),
    )
}

fn target_input(
    capability: Option<TestResolvedCarrierSelectionInputV1>,
) -> Result<TestSelectionInputV1, TestHandoffErrorV1> {
    capability
        .map(TestSelectionInputV1::Resolved)
        .ok_or(TestHandoffErrorV1::UnresolvedStop)
}

fn matrix_input(
    schedule: Vec<TestRouteV1>,
    strict_or_dev: bool,
    planner_required: bool,
    relation: TestBindingRelationV1,
    carrier: TestCarrierDispositionV1,
    shape: TestShapeV1,
) -> Result<TestResolvedCarrierSelectionInputV1, TestHandoffErrorV1> {
    let (source_id, owner_id, frame_id, write_binding, read_binding) = match relation {
        TestBindingRelationV1::Exact => (7, 5, 13, 17, 17),
        TestBindingRelationV1::Shadowing => (7, 5, 13, 17, 23),
        TestBindingRelationV1::OwnerMismatch => (7, 6, 13, 17, 17),
        TestBindingRelationV1::FrameMismatch => (7, 5, 99, 17, 17),
        TestBindingRelationV1::SourceMismatch => (8, 5, 13, 17, 17),
    };
    issue_resolved(
        TestFactsSnapshotV1 {
            source_id: 7,
            owner_id: 5,
            facts_id: 11,
        },
        TestEligibilityV1 {
            source_id,
            owner_id,
            frame_id,
            write_binding,
            read_binding,
            carrier,
            shape,
        },
        TestPreflightSeedV1 {
            source_id: 7,
            owner_id: 5,
            facts_id: 11,
            frame_id: 13,
            strict_or_dev,
            planner_required,
            base_schedule: schedule,
        },
        99,
    )
}

#[test]
fn generic_handoff_protocol_accepts_natural_both_in_release_and_strict() {
    for strict_or_dev in [false, true] {
        let receipt = select(natural_input(strict_or_dev)).expect("natural Both must select");
        assert_eq!(receipt.selected, TestRouteV1::V1);
        assert_eq!(receipt.base_schedule, [TestRouteV1::V0, TestRouteV1::V1]);
        assert_eq!(receipt.v0_attempts, 0);
        assert_eq!(receipt.strict_or_dev, strict_or_dev);
    }
}

#[test]
fn generic_handoff_source_bridge_accepts_release_and_strict() {
    for strict_or_dev in [false, true] {
        let input = source_backed_input(strict_or_dev).expect("source-backed projector bridge");
        assert_eq!(
            input.projector.raw_schedule(),
            [
                super::route_id::LoopRouteId::GenericLoopV0,
                super::route_id::LoopRouteId::GenericLoopV1,
            ]
        );
        assert_eq!(input.projector.source_forest_len(), 2);
        assert_eq!(input.projector.frame_flags(), (false, false));
        let receipt = select_source_backed(input).expect("source-backed selection");
        assert_eq!(receipt.selected, TestRouteV1::V1);
        assert_eq!(
            receipt.base_schedule,
            vec![TestRouteV1::V0, TestRouteV1::V1]
        );
        assert_eq!(receipt.v0_attempts, 0);
        assert_eq!(receipt.strict_or_dev, strict_or_dev);
    }
}

#[test]
fn generic_handoff_source_bridge_rejects_cross_invocation_before_selection() {
    let first = issue_projector_handoff_for_test(SOURCE).expect("first source witness");
    let second = issue_projector_handoff_for_test(NESTED_IF_SOURCE).expect("second source witness");
    assert!(matches!(
        first.co_sealed_with(&second),
        Err(ProjectorRejectV1::FactsIdentityMismatch)
    ));
}

#[test]
fn generic_handoff_protocol_rejects_target_rows_without_resolved_input() {
    assert_eq!(
        target_input(None).and_then(select),
        Err(TestHandoffErrorV1::UnresolvedStop)
    );
}

#[test]
fn generic_handoff_protocol_allows_legacy_only_for_typed_non_target_rows() {
    for disposition in [
        TestLegacyDispositionV1::NotApplicable,
        TestLegacyDispositionV1::ProvenOutsideTarget,
    ] {
        let receipt = select(TestSelectionInputV1::Legacy(disposition))
            .expect("typed non-target legacy row must remain available");
        assert_eq!(receipt.v0_attempts, 1);
    }
}

#[test]
fn generic_handoff_protocol_rejects_identity_and_seal_mismatches_before_selection() {
    let facts = TestFactsSnapshotV1 {
        source_id: 7,
        owner_id: 5,
        facts_id: 11,
    };
    let seed = TestPreflightSeedV1 {
        source_id: 7,
        owner_id: 5,
        facts_id: 11,
        frame_id: 13,
        strict_or_dev: true,
        planner_required: false,
        base_schedule: vec![TestRouteV1::V0, TestRouteV1::V1],
    };
    let mut input = issue_resolved(
        facts,
        TestEligibilityV1 {
            source_id: 7,
            owner_id: 5,
            frame_id: 13,
            write_binding: 17,
            read_binding: 17,
            carrier: TestCarrierDispositionV1::CompleteRecursiveCarrier,
            shape: TestShapeV1::Supported,
        },
        seed,
        1,
    )
    .expect("matching identity must seal");
    input.invocation.frame_id = 99;
    assert_eq!(
        select(TestSelectionInputV1::Resolved(input)),
        Err(TestHandoffErrorV1::UnresolvedStop)
    );

    let mismatch = issue_resolved(
        facts,
        TestEligibilityV1 {
            source_id: 8,
            owner_id: 5,
            frame_id: 13,
            write_binding: 17,
            read_binding: 17,
            carrier: TestCarrierDispositionV1::CompleteRecursiveCarrier,
            shape: TestShapeV1::Supported,
        },
        TestPreflightSeedV1 {
            source_id: 7,
            owner_id: 5,
            facts_id: 11,
            frame_id: 13,
            strict_or_dev: true,
            planner_required: false,
            base_schedule: vec![TestRouteV1::V0, TestRouteV1::V1],
        },
        2,
    );
    assert_eq!(mismatch, Err(TestHandoffErrorV1::UnresolvedStop));
}

#[test]
fn generic_handoff_protocol_rejects_shadowing_planner_and_nonrecursive_rows() {
    let cases = [
        (
            17,
            23,
            TestCarrierDispositionV1::CompleteRecursiveCarrier,
            false,
        ),
        (17, 17, TestCarrierDispositionV1::NoRecursive, false),
        (17, 17, TestCarrierDispositionV1::Unavailable, false),
        (17, 17, TestCarrierDispositionV1::Ambiguous, false),
        (
            17,
            17,
            TestCarrierDispositionV1::CompleteRecursiveCarrier,
            true,
        ),
    ];
    for (write_binding, read_binding, carrier, planner_required) in cases {
        let facts = TestFactsSnapshotV1 {
            source_id: 7,
            owner_id: 5,
            facts_id: 11,
        };
        let input = issue_resolved(
            facts,
            TestEligibilityV1 {
                source_id: 7,
                owner_id: 5,
                frame_id: 13,
                write_binding,
                read_binding,
                carrier,
                shape: TestShapeV1::Supported,
            },
            TestPreflightSeedV1 {
                source_id: 7,
                owner_id: 5,
                facts_id: 11,
                frame_id: 13,
                strict_or_dev: true,
                planner_required,
                base_schedule: vec![TestRouteV1::V0, TestRouteV1::V1],
            },
            3,
        )
        .expect("identity-matching negative row must reach typed selection");
        assert_eq!(
            select(TestSelectionInputV1::Resolved(input)),
            Err(TestHandoffErrorV1::UnresolvedStop)
        );
    }
}

#[test]
fn generic_handoff_protocol_exhaustive_typed_matrix_is_explicit() {
    let schedules = [
        vec![TestRouteV1::V0],
        vec![TestRouteV1::V1],
        vec![TestRouteV1::V0, TestRouteV1::V1],
        Vec::new(),
    ];
    let modes = [(false, false), (true, false), (true, true)];
    let relations = [
        TestBindingRelationV1::Exact,
        TestBindingRelationV1::Shadowing,
        TestBindingRelationV1::OwnerMismatch,
        TestBindingRelationV1::FrameMismatch,
        TestBindingRelationV1::SourceMismatch,
    ];
    let carriers = [
        TestCarrierDispositionV1::CompleteRecursiveCarrier,
        TestCarrierDispositionV1::NoRecursive,
        TestCarrierDispositionV1::Unavailable,
        TestCarrierDispositionV1::Ambiguous,
    ];
    let shapes = [
        TestShapeV1::Supported,
        TestShapeV1::NestedWrapper,
        TestShapeV1::DuplicateWrite,
        TestShapeV1::Index,
        TestShapeV1::Program,
        TestShapeV1::CompoundAssignment,
    ];
    let mut rows = 0;
    for schedule in &schedules {
        for (strict_or_dev, planner_required) in modes {
            for relation in relations {
                for carrier in carriers {
                    for shape in shapes {
                        let expected_candidate = schedule.as_slice()
                            == [TestRouteV1::V0, TestRouteV1::V1]
                            && !planner_required
                            && relation == TestBindingRelationV1::Exact
                            && carrier == TestCarrierDispositionV1::CompleteRecursiveCarrier
                            && shape == TestShapeV1::Supported;
                        match matrix_input(
                            schedule.clone(),
                            strict_or_dev,
                            planner_required,
                            relation,
                            carrier,
                            shape,
                        ) {
                            Ok(input) => {
                                let result = select(TestSelectionInputV1::Resolved(input));
                                if expected_candidate {
                                    let receipt =
                                        result.expect("only exact complete Both row selects");
                                    assert_eq!(receipt.selected, TestRouteV1::V1);
                                    assert_eq!(receipt.v0_attempts, 0);
                                } else {
                                    assert_eq!(result, Err(TestHandoffErrorV1::UnresolvedStop));
                                }
                            }
                            Err(error) => {
                                assert!(!expected_candidate);
                                assert_eq!(error, TestHandoffErrorV1::UnresolvedStop);
                            }
                        }
                        rows += 1;
                    }
                }
            }
        }
    }
    assert_eq!(rows, 4 * 3 * 5 * 4 * 6);
}
