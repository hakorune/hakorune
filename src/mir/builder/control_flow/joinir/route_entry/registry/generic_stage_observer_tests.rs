//! M4-D3 test-only observation of the real Generic handler path.
//!
//! This is deliberately an observer, not a second scheduler.  It reuses the
//! source-to-selection result from the A1 Both fixture, then invokes the same
//! ENTRIES dispatch used by production through the existing witness executor.

use super::dispatch_entry;
use super::execution_witness::{
    PostEffectRetryDebtV1, RouteAttemptOutcomeV1, RouteExecutionResultV1,
};
use super::generic_accepted_plan_reachability_tests::{
    evaluate_nested_carrier_policy_probe, observe_both_direct_stage, EffectOwnerV1,
    GenericCarrierPolicyDispositionV1, GenericCarrierPolicyFrameV1, GenericDirectStageEvidenceV1,
    PlanStageV1,
};
use super::generic_selection_matrix_tests::{
    both_body, effect_without_local_body, progression_condition, v1_only_effect_body,
};
use super::route_id::LoopRouteId;
use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
use crate::mir::builder::control_flow::joinir::route_entry::router::{
    test_issue_live_preflight_frame, LoopRouteContext,
};
use crate::mir::builder::control_flow::plan::facts::GenericLoopCarrierObservationV1;
use crate::mir::builder::control_flow::plan::single_planner::try_build_outcome;
use crate::mir::builder::vars::lexical_scope::LexicalScopeGuard;
use crate::mir::builder::MirBuilder;
use crate::mir::MirType;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ObserverModeV1 {
    Release,
    Strict,
    StrictPlannerRequired,
}

impl ObserverModeV1 {
    pub(super) fn strict_or_dev(self) -> bool {
        !matches!(self, Self::Release)
    }

    pub(super) fn planner_required(self) -> bool {
        matches!(self, Self::StrictPlannerRequired)
    }

    pub(super) fn config(self) -> crate::test_support::ScopedTestConfig {
        crate::test_support::ScopedTestConfig::apply(&[
            (
                "HAKO_JOINIR_STRICT",
                if matches!(self, Self::Release) {
                    None
                } else {
                    Some("1")
                },
            ),
            (
                "HAKO_JOINIR_PLANNER_REQUIRED",
                if matches!(self, Self::StrictPlannerRequired) {
                    Some("1")
                } else {
                    None
                },
            ),
            ("NYASH_JOINIR_STRICT", None),
        ])
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct AttemptTraceV1 {
    pub(super) route: LoopRouteId,
    pub(super) cursor: usize,
    pub(super) suffix: Vec<LoopRouteId>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct GenericDebtTraceV1 {
    pub(super) route: LoopRouteId,
    pub(super) composer: super::legacy_receipt::LegacyGenericComposerV1,
    pub(super) result: super::legacy_receipt::LegacyGenericResultKindV1,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum TerminalTraceV1 {
    Succeeded(LoopRouteId),
    Exhausted,
    Blocked,
    Error(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct FrameTraceV1 {
    pub(super) strict_or_dev: bool,
    pub(super) planner_required: bool,
    pub(super) has_body_local: bool,
    pub(super) recipe_contract_present: bool,
    pub(super) recipe_first_allowed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct GenericStageTraceV1 {
    pub(super) frame: FrameTraceV1,
    pub(super) raw_schedule: Vec<LoopRouteId>,
    pub(super) carrier_observation: Option<GenericLoopCarrierObservationV1>,
    pub(super) attempted: Vec<AttemptTraceV1>,
    pub(super) generic_debts: Vec<GenericDebtTraceV1>,
    pub(super) terminal: TerminalTraceV1,
}

#[derive(Debug, PartialEq)]
pub(super) struct GenericOverlapEvidenceRowV1 {
    pub(super) mode: ObserverModeV1,
    pub(super) direct: Vec<GenericDirectStageEvidenceV1>,
    pub(super) witness: GenericStageTraceV1,
}

fn seeded_builder() -> MirBuilder {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("generic_stage_observer/0".to_string());
    for name in ["i", "j"] {
        let value = builder.alloc_typed(MirType::Integer);
        builder
            .function_state
            .variable_ctx
            .variable_map
            .insert(name.to_string(), value);
    }
    builder
}

pub(super) fn observe_selected_fixture(
    mode: ObserverModeV1,
    condition: crate::ast::ASTNode,
    body: Vec<crate::ast::ASTNode>,
    function_name: &str,
) -> GenericStageTraceV1 {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    let _config = mode.config();
    let ctx = LoopRouteContext::new(&condition, &body, function_name, false, false);
    let outcome = try_build_outcome(&ctx).expect("Both fixture must build facts");
    let facts = outcome
        .facts
        .as_ref()
        .expect("Both fixture must produce canonical facts");
    let frame = test_issue_live_preflight_frame(
        &ctx,
        &outcome,
        mode.strict_or_dev(),
        mode.planner_required(),
    );
    let env = frame.test_env();
    let frame_trace = FrameTraceV1 {
        strict_or_dev: env.strict_or_dev,
        planner_required: env.planner_required,
        has_body_local: env.has_body_local,
        recipe_contract_present: frame.test_recipe_contract_present(),
        recipe_first_allowed: frame.test_recipe_first_allowed(),
    };
    assert!(
        !frame_trace.planner_required || frame_trace.strict_or_dev,
        "planner-required frame must imply strict/dev"
    );
    assert_eq!(
        frame_trace.has_body_local,
        facts.facts.loop_break_body_local().is_some(),
        "frame body-local flag must come from canonical facts"
    );
    assert_eq!(
        frame_trace.recipe_contract_present,
        outcome.recipe_contract.is_some(),
        "frame contract flag must come from the planner outcome"
    );
    let frame_raw_schedule = frame.test_raw_schedule().to_vec();
    let carrier_observation = facts
        .facts
        .generic_loop_v1()
        .map(|v1| v1.carrier_observation.clone());
    let Some(witness) = frame.test_witness_if_allowed() else {
        return GenericStageTraceV1 {
            frame: frame_trace,
            raw_schedule: frame_raw_schedule,
            carrier_observation,
            attempted: Vec::new(),
            generic_debts: Vec::new(),
            terminal: TerminalTraceV1::Blocked,
        };
    };
    let raw_schedule = witness.raw_schedule().to_vec();
    assert_eq!(
        raw_schedule, frame_raw_schedule,
        "witness must borrow the exact frame raw schedule"
    );
    let mut builder = seeded_builder();
    let _scope = LexicalScopeGuard::new(&mut builder);
    let mut attempted = Vec::new();
    let mut generic_debts = Vec::new();
    let result = witness.execute_selected_in_order(|_, attempt| {
        attempted.push(AttemptTraceV1 {
            route: attempt.current_route(),
            cursor: attempt.cursor(),
            suffix: attempt.exact_after_current_suffix().to_vec(),
        });
        let outcome = dispatch_entry(&mut builder, &ctx, Some(facts), attempt);
        if let Ok(RouteAttemptOutcomeV1::PostEffectRetryDebt(PostEffectRetryDebtV1::Generic(
            receipt,
        ))) = &outcome
        {
            generic_debts.push(GenericDebtTraceV1 {
                route: attempt.current_route(),
                composer: receipt.composer(),
                result: receipt.result_kind(),
            });
        }
        outcome
    });
    let terminal = match result {
        Ok(RouteExecutionResultV1::Succeeded { route, .. }) => TerminalTraceV1::Succeeded(route),
        Ok(RouteExecutionResultV1::Exhausted(_)) => TerminalTraceV1::Exhausted,
        Err(error) => TerminalTraceV1::Error(error),
    };
    assert!(
        attempted.len() <= raw_schedule.len(),
        "attempted prefix cannot exceed the captured raw schedule"
    );
    for (index, attempt) in attempted.iter().enumerate() {
        assert_eq!(
            attempt.route, raw_schedule[index],
            "attempted route must remain the captured raw prefix"
        );
        assert_eq!(
            attempt.cursor, index,
            "attempt cursor must remain the captured raw prefix order"
        );
        assert_eq!(
            attempt.suffix,
            raw_schedule[index + 1..],
            "attempt suffix must remain the captured raw suffix"
        );
    }
    GenericStageTraceV1 {
        frame: frame_trace,
        raw_schedule,
        carrier_observation,
        attempted,
        generic_debts,
        terminal,
    }
}

fn observe_both_fixture(mode: ObserverModeV1) -> GenericStageTraceV1 {
    observe_selected_fixture(
        mode,
        progression_condition(),
        both_body(),
        "generic_stage_observer/0",
    )
}

fn observe_v1_effect_fixture(mode: ObserverModeV1) -> GenericStageTraceV1 {
    observe_selected_fixture(
        mode,
        progression_condition(),
        v1_only_effect_body(),
        "generic_stage_observer/v1-effect",
    )
}

fn observe_effect_without_local_fixture(mode: ObserverModeV1) -> GenericStageTraceV1 {
    observe_selected_fixture(
        mode,
        progression_condition(),
        effect_without_local_body(),
        "generic_stage_observer/effect-no-local",
    )
}

pub(super) fn observe_both_evidence(mode: ObserverModeV1) -> GenericOverlapEvidenceRowV1 {
    GenericOverlapEvidenceRowV1 {
        mode,
        direct: observe_both_direct_stage(mode.strict_or_dev(), mode.planner_required()),
        witness: observe_both_fixture(mode),
    }
}

#[cfg(test)]
mod semantic_parity_matrix {
    use super::*;
    use crate::mir::builder::control_flow::plan::facts::GenericLoopCarrierObservationV1;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum ParityDispositionV1 {
        UnresolvedStop,
    }

    #[derive(Debug, Clone, PartialEq)]
    struct ParityRowV1 {
        mode: ObserverModeV1,
        direct: Vec<GenericDirectStageEvidenceV1>,
        witness: GenericStageTraceV1,
        pure_probe: GenericCarrierPolicyDispositionV1,
        disposition: ParityDispositionV1,
    }

    fn parity_row(mode: ObserverModeV1) -> ParityRowV1 {
        let evidence = observe_both_evidence(mode);
        let observation = evidence
            .witness
            .carrier_observation
            .as_ref()
            .expect("Both must expose the Generic carrier observation");
        let v1_stage_accepted = evidence.direct.iter().any(|row| {
            row.route == LoopRouteId::GenericLoopV1 && matches!(row.stage, PlanStageV1::LowerSome)
        });
        let pure_probe = evaluate_nested_carrier_policy_probe(
            observation,
            GenericCarrierPolicyFrameV1 {
                has_overlap: evidence.witness.raw_schedule.as_slice()
                    == [LoopRouteId::GenericLoopV0, LoopRouteId::GenericLoopV1],
                strict_or_dev: evidence.witness.frame.strict_or_dev,
                planner_required: evidence.witness.frame.planner_required,
                contract_present: evidence.witness.frame.recipe_contract_present,
                v1_stage_accepted,
            },
        );
        assert!(matches!(
            observation,
            GenericLoopCarrierObservationV1::CompleteRecursiveCarrier(_)
        ));
        ParityRowV1 {
            mode,
            direct: evidence.direct,
            witness: evidence.witness,
            pure_probe,
            disposition: ParityDispositionV1::UnresolvedStop,
        }
    }

    #[test]
    fn generic_both_semantic_parity_matrix_is_fresh_and_explicit() {
        let modes = [
            ObserverModeV1::Release,
            ObserverModeV1::Strict,
            ObserverModeV1::StrictPlannerRequired,
        ];
        let rows = modes.map(parity_row);
        let repeat = modes.map(parity_row);
        assert_eq!(
            rows, repeat,
            "Both parity matrix drifted on fresh candidates"
        );

        for row in rows {
            assert_eq!(row.disposition, ParityDispositionV1::UnresolvedStop);
            assert_eq!(row.witness.generic_debts.len(), 0);
            assert!(row.witness.frame.recipe_first_allowed);
            assert!(row
                .direct
                .iter()
                .all(|stage| { stage.first_effect_owner == EffectOwnerV1::GenericComposer }));
            match row.mode {
                ObserverModeV1::Release | ObserverModeV1::Strict => {
                    assert_eq!(
                        row.direct
                            .iter()
                            .map(|stage| stage.route)
                            .collect::<Vec<_>>(),
                        vec![LoopRouteId::GenericLoopV0, LoopRouteId::GenericLoopV1]
                    );
                    assert!(row
                        .direct
                        .iter()
                        .all(|stage| { matches!(stage.stage, PlanStageV1::LowerSome) }));
                    assert_eq!(row.direct.len(), 2);
                    assert_ne!(
                        row.direct[0].semantic_digest, row.direct[1].semantic_digest,
                        "nested carrier digest mismatch must remain visible"
                    );
                    assert_eq!(
                        row.witness
                            .attempted
                            .iter()
                            .map(|attempt| attempt.route)
                            .collect::<Vec<_>>(),
                        vec![LoopRouteId::GenericLoopV0]
                    );
                    assert_eq!(
                        row.witness.terminal,
                        TerminalTraceV1::Succeeded(LoopRouteId::GenericLoopV0)
                    );
                }
                ObserverModeV1::StrictPlannerRequired => {
                    assert_eq!(
                        row.direct
                            .iter()
                            .map(|stage| stage.route)
                            .collect::<Vec<_>>(),
                        vec![LoopRouteId::GenericLoopV1]
                    );
                    assert!(row
                        .direct
                        .iter()
                        .all(|stage| { matches!(stage.stage, PlanStageV1::LowerSome) }));
                    assert_eq!(row.witness.raw_schedule, vec![LoopRouteId::GenericLoopV1]);
                }
            }
            assert_eq!(
                row.pure_probe,
                GenericCarrierPolicyDispositionV1::UnresolvedStop,
                "pure probe cannot become a winner without the shared contract"
            );
        }
    }
}

#[test]
fn generic_both_fixture_reaches_actual_entries_handler_path() {
    let trace = observe_both_fixture(ObserverModeV1::Release);

    assert_eq!(
        trace.raw_schedule,
        vec![LoopRouteId::GenericLoopV0, LoopRouteId::GenericLoopV1]
    );
    assert_eq!(
        trace.attempted.first().map(|row| row.route),
        Some(LoopRouteId::GenericLoopV0)
    );
    assert_eq!(trace.attempted.first().map(|row| row.cursor), Some(0));
    assert_eq!(
        trace.attempted.first().map(|row| row.suffix.as_slice()),
        Some([LoopRouteId::GenericLoopV1].as_slice())
    );

    // This is an observed V0 success, not a proof that Generic V0 is
    // pre-effect-qualified.  The absence of a debt-to-V1 trace keeps D3 open.
    assert!(trace.generic_debts.is_empty());
    assert_eq!(
        trace.terminal,
        TerminalTraceV1::Succeeded(LoopRouteId::GenericLoopV0)
    );
}

#[test]
fn generic_both_fixture_records_mode_specific_witness_boundaries() {
    let modes = [
        ObserverModeV1::Release,
        ObserverModeV1::Strict,
        ObserverModeV1::StrictPlannerRequired,
    ];

    for mode in modes {
        let trace = observe_both_fixture(mode);
        let repeat = observe_both_fixture(mode);
        assert_eq!(trace, repeat, "mode-specific witness drift: {mode:?}");
        assert_eq!(
            trace.frame.strict_or_dev,
            !matches!(mode, ObserverModeV1::Release),
            "frame must capture the production strict/dev mode"
        );
        assert_eq!(
            trace.frame.planner_required,
            matches!(mode, ObserverModeV1::StrictPlannerRequired),
            "frame must capture the production planner-required mode"
        );
        assert!(
            trace.frame.recipe_first_allowed,
            "Generic Both fixture must remain recipe-first allowed"
        );

        assert!(
            !trace.raw_schedule.is_empty(),
            "Both fixture must retain a selected route in {mode:?}"
        );
        match mode {
            ObserverModeV1::Release | ObserverModeV1::Strict => assert_eq!(
                trace.raw_schedule,
                vec![LoopRouteId::GenericLoopV0, LoopRouteId::GenericLoopV1],
                "release/strict Both selection must retain the V0/V1 overlap"
            ),
            ObserverModeV1::StrictPlannerRequired => assert_eq!(
                trace.raw_schedule,
                vec![LoopRouteId::GenericLoopV1],
                "planner-required selection must suppress Generic V0 before the witness"
            ),
        }
        assert_eq!(
            trace.attempted.first().map(|row| row.route),
            trace.raw_schedule.first().copied(),
            "witness must attempt the captured prefix in {mode:?}"
        );

        if trace.raw_schedule == [LoopRouteId::GenericLoopV0, LoopRouteId::GenericLoopV1] {
            assert_eq!(
                trace.attempted,
                vec![AttemptTraceV1 {
                    route: LoopRouteId::GenericLoopV0,
                    cursor: 0,
                    suffix: vec![LoopRouteId::GenericLoopV1],
                }],
                "a V0 terminal/error must not silently continue to V1: {mode:?}"
            );
            assert!(
                trace.generic_debts.is_empty(),
                "no Generic debt receipt was observed for {mode:?}: {trace:?}"
            );
        }
    }
}

#[test]
fn generic_both_evidence_matrix_keeps_direct_stage_and_witness_separate() {
    let matrix = [
        observe_both_evidence(ObserverModeV1::Release),
        observe_both_evidence(ObserverModeV1::Strict),
        observe_both_evidence(ObserverModeV1::StrictPlannerRequired),
    ];
    for row in matrix {
        assert!(
            row.direct.iter().all(|evidence| evidence.first_effect_owner
                == super::generic_accepted_plan_reachability_tests::EffectOwnerV1::GenericComposer),
            "direct stage must record a Generic composer effect owner: {row:?}"
        );
        assert!(
            row.witness.generic_debts.is_empty(),
            "Both evidence must not turn absence of a debt receipt into a proof: {row:?}"
        );
        match row.mode {
            ObserverModeV1::Release | ObserverModeV1::Strict => {
                assert_eq!(
                    row.direct
                        .iter()
                        .map(|evidence| evidence.route)
                        .collect::<Vec<_>>(),
                    vec![LoopRouteId::GenericLoopV0, LoopRouteId::GenericLoopV1]
                );
                assert_eq!(
                    row.witness.raw_schedule,
                    vec![LoopRouteId::GenericLoopV0, LoopRouteId::GenericLoopV1]
                );
                assert_eq!(
                    row.witness.terminal,
                    TerminalTraceV1::Succeeded(LoopRouteId::GenericLoopV0)
                );
            }
            ObserverModeV1::StrictPlannerRequired => {
                assert_eq!(
                    row.direct
                        .iter()
                        .map(|evidence| evidence.route)
                        .collect::<Vec<_>>(),
                    vec![LoopRouteId::GenericLoopV1]
                );
                assert_eq!(row.witness.raw_schedule, vec![LoopRouteId::GenericLoopV1]);
            }
        }
    }
    // This matrix records the real pair of observations; it is not a pure
    // winner oracle.  D2-B still needs pre-effect policy equivalence or a
    // production-derived disjointness proof.
}

#[test]
fn generic_both_alpha_digest_keeps_parity_stop_explicit() {
    for mode in [ObserverModeV1::Release, ObserverModeV1::Strict] {
        let row = observe_both_evidence(mode);
        let repeat = observe_both_evidence(mode);
        let digests = row
            .direct
            .iter()
            .map(|evidence| {
                evidence
                    .semantic_digest
                    .clone()
                    .expect("accepted plan digest")
            })
            .collect::<Vec<_>>();
        let repeat_digests = repeat
            .direct
            .iter()
            .map(|evidence| {
                evidence
                    .semantic_digest
                    .clone()
                    .expect("accepted plan digest")
            })
            .collect::<Vec<_>>();
        assert_eq!(digests, repeat_digests, "fresh digest drift: {mode:?}");
        assert_eq!(digests.len(), 2, "Both must retain V0 and V1: {mode:?}");
        assert_ne!(
            digests[0], digests[1],
            "semantic digest difference must keep D2-B2 unresolved: {mode:?}"
        );
    }

    let planner = observe_both_evidence(ObserverModeV1::StrictPlannerRequired);
    assert_eq!(planner.direct.len(), 1);
    assert!(planner.direct[0].semantic_digest.is_some());
}

#[test]
fn generic_v1_effect_fixture_stops_at_actual_handler_error_without_retry() {
    for mode in [
        ObserverModeV1::Release,
        ObserverModeV1::Strict,
        ObserverModeV1::StrictPlannerRequired,
    ] {
        let trace = observe_v1_effect_fixture(mode);
        let repeat = observe_v1_effect_fixture(mode);
        assert_eq!(trace, repeat, "V1 effect witness drift: {mode:?}");
        assert_eq!(trace.raw_schedule, vec![LoopRouteId::GenericLoopV1]);
        assert_eq!(
            trace.attempted,
            vec![AttemptTraceV1 {
                route: LoopRouteId::GenericLoopV1,
                cursor: 0,
                suffix: Vec::new(),
            }]
        );
        assert!(trace.generic_debts.is_empty());
        assert!(
            matches!(trace.terminal, TerminalTraceV1::Error(_)),
            "effect-call row must stop at the actual handler error: {trace:?}"
        );
    }
}

#[test]
fn generic_effect_without_local_fixture_is_not_both_and_stops_without_retry() {
    for mode in [
        ObserverModeV1::Release,
        ObserverModeV1::Strict,
        ObserverModeV1::StrictPlannerRequired,
    ] {
        let trace = observe_effect_without_local_fixture(mode);
        let repeat = observe_effect_without_local_fixture(mode);
        assert_eq!(trace, repeat, "effect boundary drift: {mode:?}");
        assert_eq!(trace.raw_schedule, vec![LoopRouteId::GenericLoopV1]);
        assert_eq!(trace.attempted.len(), 1);
        assert_eq!(trace.attempted[0].route, LoopRouteId::GenericLoopV1);
        assert!(trace.generic_debts.is_empty());
        assert!(
            matches!(trace.terminal, TerminalTraceV1::Error(_)),
            "effect boundary must stop at the actual handler error: {trace:?}"
        );
    }
}

#[test]
fn generic_both_policy_witness_mismatch_remains_unresolved() {
    for mode in [
        ObserverModeV1::Release,
        ObserverModeV1::Strict,
        ObserverModeV1::StrictPlannerRequired,
    ] {
        let trace = observe_both_fixture(mode);
        let observation = trace
            .carrier_observation
            .as_ref()
            .expect("Both frame must expose Generic V1 carrier observation");
        let raw_schedule = &trace.raw_schedule;
        let v1_stage_accepted =
            observe_both_direct_stage(trace.frame.strict_or_dev, trace.frame.planner_required)
                .iter()
                .any(|row| {
                    row.route == LoopRouteId::GenericLoopV1
                        && matches!(row.stage, PlanStageV1::LowerSome)
                });
        let disposition = evaluate_nested_carrier_policy_probe(
            observation,
            GenericCarrierPolicyFrameV1 {
                has_overlap: raw_schedule.as_slice()
                    == [LoopRouteId::GenericLoopV0, LoopRouteId::GenericLoopV1],
                strict_or_dev: trace.frame.strict_or_dev,
                planner_required: trace.frame.planner_required,
                contract_present: trace.frame.recipe_contract_present,
                v1_stage_accepted,
            },
        );
        let unresolved = disposition == GenericCarrierPolicyDispositionV1::UnresolvedStop
            || (raw_schedule.as_slice()
                == [LoopRouteId::GenericLoopV0, LoopRouteId::GenericLoopV1]
                && disposition == GenericCarrierPolicyDispositionV1::V1ForNestedCarriers
                && trace.terminal == TerminalTraceV1::Succeeded(LoopRouteId::GenericLoopV0));
        assert!(
            unresolved,
            "policy/witness reconciliation must stop: {mode:?}"
        );
        if raw_schedule.as_slice() == [LoopRouteId::GenericLoopV0, LoopRouteId::GenericLoopV1] {
            assert_eq!(
                disposition,
                GenericCarrierPolicyDispositionV1::UnresolvedStop,
                "the current Generic handler has no recipe contract receipt; the pure policy probe must remain unresolved"
            );
            assert_eq!(
                trace.terminal,
                TerminalTraceV1::Succeeded(LoopRouteId::GenericLoopV0),
                "policy/witness route mismatch must remain visible"
            );
        } else {
            assert_eq!(
                disposition,
                GenericCarrierPolicyDispositionV1::UnresolvedStop
            );
        }
    }
}

#[test]
fn generic_carrier_observation_does_not_overclaim_unhandled_consumers() {
    let assignment = ASTNode::Assignment {
        target: Box::new(ASTNode::Variable {
            name: "j".into(),
            span: Span::unknown(),
        }),
        value: Box::new(ASTNode::Literal {
            value: LiteralValue::Integer(1),
            span: Span::unknown(),
        }),
        span: Span::unknown(),
    };
    let program = vec![ASTNode::Program {
        statements: vec![assignment.clone()],
        span: Span::unknown(),
    }];
    let compound = ASTNode::Loop {
        condition: Box::new(ASTNode::Literal {
            value: LiteralValue::Bool(true),
            span: Span::unknown(),
        }),
        body: vec![ASTNode::CompoundAssignment {
            target: Box::new(ASTNode::Variable {
                name: "j".into(),
                span: Span::unknown(),
            }),
            operator: BinaryOperator::Add,
            value: Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(1),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }],
        span: Span::unknown(),
    };
    for (body, container) in [(program, "Program"), (vec![compound], "CompoundAssignment")] {
        assert_eq!(
            crate::mir::builder::control_flow::plan::facts::observe_generic_loop_carrier_observation(
                &body, "i"
            ),
            crate::mir::builder::control_flow::plan::facts::GenericLoopCarrierObservationV1::Unavailable(
                container.into()
            )
        );
    }
}
