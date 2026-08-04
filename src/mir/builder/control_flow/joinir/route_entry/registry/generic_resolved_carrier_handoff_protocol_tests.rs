//! Design-stop protocol evidence for the resolved Generic carrier handoff.
//!
//! Every type in this file is private to `cfg(test)`.  This is deliberately a
//! protocol model, not a production selector or capability implementation. It
//! freezes the facts/capability pairing boundary before a production owner is
//! authorized by the handoff design card.

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
enum TestLegacyDispositionV1 {
    NotApplicable,
    ProvenOutsideTarget,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TestFactsSnapshotV1 {
    source_id: u16,
    facts_id: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TestPreflightSeedV1 {
    source_id: u16,
    facts_id: u16,
    frame_id: u16,
    strict_or_dev: bool,
    planner_required: bool,
    base_schedule: Vec<TestRouteV1>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TestEligibilityV1 {
    source_id: u16,
    frame_id: u16,
    write_binding: u16,
    read_binding: u16,
    carrier: TestCarrierDispositionV1,
}

#[derive(Debug, PartialEq, Eq)]
struct TestInvocationSealV1 {
    invocation_id: u64,
    source_id: u16,
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

fn natural_input(strict_or_dev: bool) -> TestSelectionInputV1 {
    let facts = TestFactsSnapshotV1 {
        source_id: 7,
        facts_id: 11,
    };
    let eligibility = TestEligibilityV1 {
        source_id: 7,
        frame_id: 13,
        write_binding: 17,
        read_binding: 17,
        carrier: TestCarrierDispositionV1::CompleteRecursiveCarrier,
    };
    let seed = TestPreflightSeedV1 {
        source_id: 7,
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
        facts_id: 11,
    };
    let seed = TestPreflightSeedV1 {
        source_id: 7,
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
            frame_id: 13,
            write_binding: 17,
            read_binding: 17,
            carrier: TestCarrierDispositionV1::CompleteRecursiveCarrier,
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
            frame_id: 13,
            write_binding: 17,
            read_binding: 17,
            carrier: TestCarrierDispositionV1::CompleteRecursiveCarrier,
        },
        TestPreflightSeedV1 {
            source_id: 7,
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
            facts_id: 11,
        };
        let input = issue_resolved(
            facts,
            TestEligibilityV1 {
                source_id: 7,
                frame_id: 13,
                write_binding,
                read_binding,
                carrier,
            },
            TestPreflightSeedV1 {
                source_id: 7,
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
