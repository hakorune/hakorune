//! M4-D0-S1: test-only Generic V0/V1 stage/disposition matrix.
//!
//! The matrix joins the existing facts/composer/verifier/lowerer observation
//! with the real witness trace.  It is an evidence table, never a selector,
//! policy oracle, or production scheduler.

use super::generic_accepted_plan_reachability_tests::{
    observe_fixture, CandidateSnapshotV1, CorpusModeV1, EffectOwnerV1, PlanStageV1,
};
use super::generic_selection_matrix_tests::{
    additive_body, additive_condition, both_body, neither_body, progression_condition, v1_only_body,
};
use super::generic_stage_observer_tests::{
    observe_selected_fixture, GenericStageTraceV1, ObserverModeV1, TerminalTraceV1,
};
use super::route_id::LoopRouteId;
use crate::ast::ASTNode;
use crate::mir::builder::control_flow::joinir::route_entry::router::LoopRouteContext;
use crate::mir::builder::control_flow::plan::features::generic_loop_body::{
    observe_nested_depth1, NestedStageResultV1,
};
use crate::mir::builder::control_flow::plan::single_planner::try_build_outcome;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FixtureClassV1 {
    V0Only,
    V1Only,
    Both,
    Neither,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MatrixStageV1 {
    FactsAbsentOrNonMatch,
    ComposerPreconditionNoDelta,
    ComposerSuccessFirstDelta,
    ComposerErrAfterDelta,
    StrictShadowSome,
    StrictShadowNone,
    StrictShadowErr,
    ReleaseVerifierOk,
    ReleaseVerifierErr,
    ReleaseLowerSome,
    ReleaseLowerNone,
    ReleaseLowerErr,
    NonLoopRoot,
    NestedFastpath,
    NestedGenericFallback,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MatrixOutcomeV1 {
    Succeeded,
    Failed,
    NotObserved,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MatrixDispositionV1 {
    PreEffectDeclined,
    PreEffectBlocked,
    TerminalFreezeTarget,
    ImpossibleEdge,
    UnresolvedStop,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MatrixEvidenceV1 {
    Observed,
    NotYetObserved,
    UnresolvedStop,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MatrixReceiptV1 {
    GenericComposerDebt,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum MatrixTerminalV1 {
    Succeeded(LoopRouteId),
    Error,
    Blocked,
    Exhausted,
    NotObserved,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct GenericStageDispositionRowV1 {
    fixture: FixtureClassV1,
    source_fixture: &'static str,
    mode: CorpusModeV1,
    route: Option<LoopRouteId>,
    raw_schedule: Vec<LoopRouteId>,
    contract_present: bool,
    stage: MatrixStageV1,
    outcome: MatrixOutcomeV1,
    first_effect_owner: EffectOwnerV1,
    before_compose: Option<CandidateSnapshotV1>,
    before_lower: Option<CandidateSnapshotV1>,
    after_lower: Option<CandidateSnapshotV1>,
    receipt: Option<MatrixReceiptV1>,
    attempted_prefix: Vec<LoopRouteId>,
    terminal: MatrixTerminalV1,
    disposition: Option<MatrixDispositionV1>,
    evidence: MatrixEvidenceV1,
    source_anchor: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SourceSelectionV1 {
    generic_facts_present: bool,
    raw_schedule: Vec<LoopRouteId>,
    contract_present: bool,
}

fn fixture_input(class: FixtureClassV1) -> (ASTNode, Vec<ASTNode>, &'static str) {
    match class {
        FixtureClassV1::V0Only => (additive_condition(), additive_body(), "v0-additive"),
        FixtureClassV1::V1Only => (progression_condition(), v1_only_body(), "v1-only"),
        FixtureClassV1::Both => (progression_condition(), both_body(), "both"),
        FixtureClassV1::Neither => (progression_condition(), neither_body(), "neither"),
    }
}

fn mode_config(mode: CorpusModeV1) -> crate::test_support::ScopedTestConfig {
    let (_, strict) = mode.env();
    crate::test_support::ScopedTestConfig::apply(&[
        ("HAKO_JOINIR_STRICT", strict),
        ("HAKO_JOINIR_PLANNER_REQUIRED", mode.planner_required()),
        ("NYASH_JOINIR_STRICT", None),
    ])
}

fn source_selection(mode: CorpusModeV1, class: FixtureClassV1) -> SourceSelectionV1 {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    let _config = mode_config(mode);
    let (condition, body, _) = fixture_input(class);
    let ctx = LoopRouteContext::new(&condition, &body, "generic_stage_matrix/0", false, false);
    let outcome = match try_build_outcome(&ctx) {
        Ok(outcome) => outcome,
        Err(_) => {
            return SourceSelectionV1 {
                generic_facts_present: false,
                raw_schedule: Vec::new(),
                contract_present: false,
            }
        }
    };
    let Some(facts) = outcome.facts.as_ref() else {
        return SourceSelectionV1 {
            generic_facts_present: false,
            raw_schedule: Vec::new(),
            contract_present: outcome.recipe_contract.is_some(),
        };
    };
    let generic_facts_present =
        facts.facts.generic_loop_v0().is_some() || facts.facts.generic_loop_v1().is_some();
    SourceSelectionV1 {
        generic_facts_present,
        raw_schedule: super::select_recipe_first_routes(Some(facts))
            .raw_execution_routes()
            .to_vec(),
        contract_present: outcome.recipe_contract.is_some(),
    }
}

fn observer_mode(mode: CorpusModeV1) -> ObserverModeV1 {
    match mode {
        CorpusModeV1::Release => ObserverModeV1::Release,
        CorpusModeV1::Strict => ObserverModeV1::Strict,
        CorpusModeV1::StrictPlannerRequired => ObserverModeV1::StrictPlannerRequired,
    }
}

fn witness_trace(mode: CorpusModeV1, class: FixtureClassV1) -> Option<GenericStageTraceV1> {
    let observer_mode = observer_mode(mode);
    let (condition, body, _) = fixture_input(class);
    match class {
        FixtureClassV1::V1Only | FixtureClassV1::Both => Some(observe_selected_fixture(
            observer_mode,
            condition,
            body,
            "generic_stage_matrix/0",
        )),
        FixtureClassV1::V0Only | FixtureClassV1::Neither => None,
    }
}

fn expected_generic_routes(class: FixtureClassV1, mode: CorpusModeV1) -> &'static [LoopRouteId] {
    match class {
        FixtureClassV1::V0Only => &[LoopRouteId::GenericLoopV0],
        FixtureClassV1::V1Only => &[LoopRouteId::GenericLoopV1],
        FixtureClassV1::Both if mode == CorpusModeV1::StrictPlannerRequired => {
            &[LoopRouteId::GenericLoopV1]
        }
        FixtureClassV1::Both => &[LoopRouteId::GenericLoopV0, LoopRouteId::GenericLoopV1],
        FixtureClassV1::Neither => &[],
    }
}

fn generic_routes(raw_schedule: &[LoopRouteId]) -> Vec<LoopRouteId> {
    raw_schedule
        .iter()
        .copied()
        .filter(|route| {
            matches!(
                route,
                LoopRouteId::GenericLoopV0 | LoopRouteId::GenericLoopV1
            )
        })
        .collect()
}

fn fixture_evidence(
    class: FixtureClassV1,
    mode: CorpusModeV1,
    raw_schedule: &[LoopRouteId],
) -> MatrixEvidenceV1 {
    if generic_routes(raw_schedule) == expected_generic_routes(class, mode) {
        MatrixEvidenceV1::Observed
    } else {
        MatrixEvidenceV1::UnresolvedStop
    }
}

fn terminal_from_trace(trace: Option<&GenericStageTraceV1>) -> MatrixTerminalV1 {
    match trace.map(|trace| &trace.terminal) {
        Some(TerminalTraceV1::Succeeded(route)) => MatrixTerminalV1::Succeeded(*route),
        Some(TerminalTraceV1::Error(_)) => MatrixTerminalV1::Error,
        Some(TerminalTraceV1::Blocked) => MatrixTerminalV1::Blocked,
        Some(TerminalTraceV1::Exhausted) => MatrixTerminalV1::Exhausted,
        None => MatrixTerminalV1::NotObserved,
    }
}

fn attempted_from_trace(trace: Option<&GenericStageTraceV1>) -> Vec<LoopRouteId> {
    trace
        .map(|trace| {
            trace
                .attempted
                .iter()
                .map(|attempt| attempt.route)
                .collect()
        })
        .unwrap_or_default()
}

fn receipt_from_trace(trace: Option<&GenericStageTraceV1>) -> Option<MatrixReceiptV1> {
    trace
        .filter(|trace| !trace.generic_debts.is_empty())
        .map(|_| MatrixReceiptV1::GenericComposerDebt)
}

fn disposition_for_direct_stage(
    stage: PlanStageV1,
    first_effect_owner: EffectOwnerV1,
) -> Option<MatrixDispositionV1> {
    match stage {
        PlanStageV1::ComposerError if first_effect_owner == EffectOwnerV1::None => {
            Some(MatrixDispositionV1::PreEffectBlocked)
        }
        PlanStageV1::ComposerError => Some(MatrixDispositionV1::TerminalFreezeTarget),
        PlanStageV1::VerifierRejected | PlanStageV1::LowerError => {
            Some(MatrixDispositionV1::UnresolvedStop)
        }
        PlanStageV1::LowerNone => Some(MatrixDispositionV1::ImpossibleEdge),
        PlanStageV1::NonLoopRoot => Some(MatrixDispositionV1::ImpossibleEdge),
        PlanStageV1::LowerSome => None,
    }
}

fn row_base(
    fixture: FixtureClassV1,
    source_fixture: &'static str,
    mode: CorpusModeV1,
    source: &SourceSelectionV1,
    trace: Option<&GenericStageTraceV1>,
    route: Option<LoopRouteId>,
    stage: MatrixStageV1,
    outcome: MatrixOutcomeV1,
    first_effect_owner: EffectOwnerV1,
    before_compose: Option<CandidateSnapshotV1>,
    before_lower: Option<CandidateSnapshotV1>,
    after_lower: Option<CandidateSnapshotV1>,
    disposition: Option<MatrixDispositionV1>,
    evidence: MatrixEvidenceV1,
    source_anchor: &'static str,
) -> GenericStageDispositionRowV1 {
    GenericStageDispositionRowV1 {
        fixture,
        source_fixture,
        mode,
        route,
        raw_schedule: source.raw_schedule.clone(),
        contract_present: source.contract_present,
        stage,
        outcome,
        first_effect_owner,
        before_compose,
        before_lower,
        after_lower,
        receipt: receipt_from_trace(trace),
        attempted_prefix: attempted_from_trace(trace),
        terminal: terminal_from_trace(trace),
        disposition,
        evidence,
        source_anchor,
    }
}

fn direct_rows(
    fixture: FixtureClassV1,
    mode: CorpusModeV1,
    source: &SourceSelectionV1,
    trace: Option<&GenericStageTraceV1>,
) -> Vec<GenericStageDispositionRowV1> {
    let (_, _, source_fixture) = fixture_input(fixture);
    let evidence = fixture_evidence(fixture, mode, &source.raw_schedule);
    let anchor = "generic_accepted_plan_reachability_tests::observe_fixture";
    let mut rows = Vec::new();
    for direct in observe_fixture(mode, source_fixture) {
        let effectful = direct.before_compose != direct.before_lower;
        let composer_stage = match direct.stage {
            PlanStageV1::ComposerError if effectful => MatrixStageV1::ComposerErrAfterDelta,
            PlanStageV1::ComposerError => MatrixStageV1::ComposerPreconditionNoDelta,
            _ if effectful => MatrixStageV1::ComposerSuccessFirstDelta,
            _ => MatrixStageV1::ComposerPreconditionNoDelta,
        };
        let composer_outcome = if matches!(direct.stage, PlanStageV1::ComposerError) {
            MatrixOutcomeV1::Failed
        } else {
            MatrixOutcomeV1::Succeeded
        };
        rows.push(row_base(
            fixture,
            source_fixture,
            mode,
            source,
            trace,
            Some(direct.route),
            composer_stage,
            composer_outcome,
            direct.first_effect_owner,
            Some(direct.before_compose.clone()),
            Some(direct.before_lower.clone()),
            Some(direct.after_lower.clone()),
            disposition_for_direct_stage(direct.stage, direct.first_effect_owner),
            evidence,
            anchor,
        ));

        let (stage, outcome, disposition) = match direct.stage {
            PlanStageV1::VerifierRejected => (
                if mode.strict_or_dev() {
                    MatrixStageV1::StrictShadowErr
                } else {
                    MatrixStageV1::ReleaseVerifierErr
                },
                MatrixOutcomeV1::Failed,
                Some(MatrixDispositionV1::UnresolvedStop),
            ),
            PlanStageV1::LowerSome => (
                if mode.strict_or_dev() {
                    MatrixStageV1::StrictShadowSome
                } else {
                    MatrixStageV1::ReleaseLowerSome
                },
                MatrixOutcomeV1::Succeeded,
                None,
            ),
            PlanStageV1::LowerNone => (
                if mode.strict_or_dev() {
                    MatrixStageV1::StrictShadowNone
                } else {
                    MatrixStageV1::ReleaseLowerNone
                },
                MatrixOutcomeV1::Failed,
                Some(MatrixDispositionV1::ImpossibleEdge),
            ),
            PlanStageV1::LowerError => (
                if mode.strict_or_dev() {
                    MatrixStageV1::StrictShadowErr
                } else {
                    MatrixStageV1::ReleaseLowerErr
                },
                MatrixOutcomeV1::Failed,
                Some(MatrixDispositionV1::UnresolvedStop),
            ),
            PlanStageV1::NonLoopRoot => (
                MatrixStageV1::NonLoopRoot,
                MatrixOutcomeV1::Failed,
                Some(MatrixDispositionV1::ImpossibleEdge),
            ),
            PlanStageV1::ComposerError => continue,
        };
        rows.push(row_base(
            fixture,
            source_fixture,
            mode,
            source,
            trace,
            Some(direct.route),
            stage,
            outcome,
            direct.first_effect_owner,
            Some(direct.before_compose),
            Some(direct.before_lower),
            Some(direct.after_lower),
            disposition,
            evidence,
            anchor,
        ));
        if !mode.strict_or_dev() && matches!(direct.stage, PlanStageV1::LowerSome) {
            rows.push(row_base(
                fixture,
                source_fixture,
                mode,
                source,
                trace,
                Some(direct.route),
                MatrixStageV1::ReleaseVerifierOk,
                MatrixOutcomeV1::Succeeded,
                direct.first_effect_owner,
                None,
                None,
                None,
                None,
                evidence,
                "src/mir/builder/control_flow/verify/PlanVerifier::verify",
            ));
        }
    }
    rows
}

fn missing_arm_row(
    mode: CorpusModeV1,
    route: LoopRouteId,
    stage: MatrixStageV1,
    source_anchor: &'static str,
) -> GenericStageDispositionRowV1 {
    row_base(
        FixtureClassV1::V0Only,
        "natural corpus has no proven V0-only witness",
        mode,
        &SourceSelectionV1 {
            generic_facts_present: true,
            raw_schedule: Vec::new(),
            contract_present: false,
        },
        None,
        Some(route),
        stage,
        MatrixOutcomeV1::NotObserved,
        EffectOwnerV1::None,
        None,
        None,
        None,
        Some(MatrixDispositionV1::UnresolvedStop),
        MatrixEvidenceV1::NotYetObserved,
        source_anchor,
    )
}

fn nested_snapshot(
    snapshot: &crate::mir::builder::control_flow::plan::features::generic_loop_body::
        NestedBuilderSnapshotV1,
) -> CandidateSnapshotV1 {
    CandidateSnapshotV1 {
        current_block: snapshot.current_block,
        block_count: snapshot.block_count,
        next_value_id: snapshot.next_value_id,
        variable_count: snapshot.variable_count,
        typed_value_count: snapshot.typed_value_count,
    }
}

fn nested_result_outcome(result: NestedStageResultV1) -> MatrixOutcomeV1 {
    match result {
        NestedStageResultV1::Succeeded => MatrixOutcomeV1::Succeeded,
        NestedStageResultV1::ReturnedNone | NestedStageResultV1::ReturnedErr => {
            MatrixOutcomeV1::Failed
        }
        NestedStageResultV1::NotObserved => MatrixOutcomeV1::NotObserved,
    }
}

fn nested_result_disposition(result: NestedStageResultV1) -> Option<MatrixDispositionV1> {
    match result {
        NestedStageResultV1::Succeeded => None,
        NestedStageResultV1::ReturnedNone
        | NestedStageResultV1::ReturnedErr
        | NestedStageResultV1::NotObserved => Some(MatrixDispositionV1::UnresolvedStop),
    }
}

fn nested_result_evidence(result: NestedStageResultV1) -> MatrixEvidenceV1 {
    match result {
        NestedStageResultV1::NotObserved => MatrixEvidenceV1::NotYetObserved,
        NestedStageResultV1::Succeeded
        | NestedStageResultV1::ReturnedNone
        | NestedStageResultV1::ReturnedErr => MatrixEvidenceV1::Observed,
    }
}

fn nested_rows(mode: CorpusModeV1) -> Vec<GenericStageDispositionRowV1> {
    let source = source_selection(mode, FixtureClassV1::Both);
    let observation =
        observe_nested_depth1(mode.strict_or_dev(), mode.planner_required().is_some());
    let source_fixture = "both/nested-depth1";
    let source_anchor = "generic_loop_body::nested_depth_observer_tests::observe_nested_depth1";
    let fastpath_owner = if observation.before_fastpath != observation.after_fastpath {
        EffectOwnerV1::NestedDepth1Fastpath
    } else {
        EffectOwnerV1::None
    };
    let fastpath_snapshots = Some(nested_snapshot(&observation.before_fastpath));
    let fastpath_after = Some(nested_snapshot(&observation.after_fastpath));
    let fastpath_row = GenericStageDispositionRowV1 {
        fixture: FixtureClassV1::Both,
        source_fixture,
        mode,
        route: Some(LoopRouteId::GenericLoopV1),
        raw_schedule: source.raw_schedule.clone(),
        contract_present: source.contract_present,
        stage: MatrixStageV1::NestedFastpath,
        outcome: nested_result_outcome(observation.fastpath),
        first_effect_owner: fastpath_owner,
        before_compose: fastpath_snapshots,
        before_lower: fastpath_after.clone(),
        after_lower: fastpath_after,
        receipt: None,
        attempted_prefix: Vec::new(),
        terminal: if observation.fastpath == NestedStageResultV1::Succeeded {
            MatrixTerminalV1::Succeeded(LoopRouteId::GenericLoopV1)
        } else {
            MatrixTerminalV1::NotObserved
        },
        disposition: nested_result_disposition(observation.fastpath),
        evidence: nested_result_evidence(observation.fastpath),
        source_anchor,
    };

    let fallback_owner = match (
        observation.before_fallback.as_ref(),
        observation.after_fallback.as_ref(),
    ) {
        (Some(before), Some(after)) if before != after => EffectOwnerV1::NestedGenericFallback,
        _ => EffectOwnerV1::None,
    };
    let fallback_row = GenericStageDispositionRowV1 {
        fixture: FixtureClassV1::Both,
        source_fixture,
        mode,
        route: Some(LoopRouteId::GenericLoopV1),
        raw_schedule: source.raw_schedule,
        contract_present: source.contract_present,
        stage: MatrixStageV1::NestedGenericFallback,
        outcome: nested_result_outcome(observation.fallback),
        first_effect_owner: fallback_owner,
        before_compose: observation.before_fallback.as_ref().map(nested_snapshot),
        before_lower: observation.before_fallback.as_ref().map(nested_snapshot),
        after_lower: observation.after_fallback.as_ref().map(nested_snapshot),
        receipt: None,
        attempted_prefix: Vec::new(),
        terminal: MatrixTerminalV1::NotObserved,
        disposition: nested_result_disposition(observation.fallback),
        evidence: nested_result_evidence(observation.fallback),
        source_anchor,
    };
    vec![fastpath_row, fallback_row]
}

fn required_stages(mode: CorpusModeV1) -> &'static [MatrixStageV1] {
    if mode.strict_or_dev() {
        &[
            MatrixStageV1::ComposerPreconditionNoDelta,
            MatrixStageV1::ComposerSuccessFirstDelta,
            MatrixStageV1::ComposerErrAfterDelta,
            MatrixStageV1::StrictShadowSome,
            MatrixStageV1::StrictShadowNone,
            MatrixStageV1::StrictShadowErr,
            MatrixStageV1::NestedFastpath,
            MatrixStageV1::NestedGenericFallback,
        ]
    } else {
        &[
            MatrixStageV1::ComposerPreconditionNoDelta,
            MatrixStageV1::ComposerSuccessFirstDelta,
            MatrixStageV1::ComposerErrAfterDelta,
            MatrixStageV1::ReleaseVerifierOk,
            MatrixStageV1::ReleaseVerifierErr,
            MatrixStageV1::ReleaseLowerSome,
            MatrixStageV1::ReleaseLowerNone,
            MatrixStageV1::ReleaseLowerErr,
            MatrixStageV1::NestedFastpath,
            MatrixStageV1::NestedGenericFallback,
        ]
    }
}

fn build_matrix() -> Vec<GenericStageDispositionRowV1> {
    let mut rows = Vec::new();
    let fixtures = [
        FixtureClassV1::V0Only,
        FixtureClassV1::V1Only,
        FixtureClassV1::Both,
        FixtureClassV1::Neither,
    ];
    let modes = [
        CorpusModeV1::Release,
        CorpusModeV1::Strict,
        CorpusModeV1::StrictPlannerRequired,
    ];
    for mode in modes {
        for fixture in fixtures {
            let source = source_selection(mode, fixture);
            let trace = witness_trace(mode, fixture);
            if !source.generic_facts_present || source.raw_schedule.is_empty() {
                let (_, _, source_fixture) = fixture_input(fixture);
                rows.push(row_base(
                    fixture,
                    source_fixture,
                    mode,
                    &source,
                    trace.as_ref(),
                    None,
                    MatrixStageV1::FactsAbsentOrNonMatch,
                    MatrixOutcomeV1::NotObserved,
                    EffectOwnerV1::None,
                    None,
                    None,
                    None,
                    Some(MatrixDispositionV1::PreEffectDeclined),
                    fixture_evidence(fixture, mode, &source.raw_schedule),
                    "generic_selection_matrix_tests::observe",
                ));
            }
            rows.extend(direct_rows(fixture, mode, &source, trace.as_ref()));
        }
        rows.extend(nested_rows(mode));
    }

    for mode in modes {
        for route in [LoopRouteId::GenericLoopV0, LoopRouteId::GenericLoopV1] {
            for stage in required_stages(mode) {
                if !rows
                    .iter()
                    .any(|row| row.mode == mode && row.route == Some(route) && row.stage == *stage)
                {
                    rows.push(missing_arm_row(
                        mode,
                        route,
                        *stage,
                        "generic_stage_matrix_tests::required_stages",
                    ));
                }
            }
        }
    }
    rows
}

#[test]
fn generic_stage_disposition_matrix_is_complete_and_repeatable() {
    let matrix = build_matrix();
    assert_eq!(
        matrix,
        build_matrix(),
        "Generic stage matrix drifted on fresh runs"
    );

    for mode in [
        CorpusModeV1::Release,
        CorpusModeV1::Strict,
        CorpusModeV1::StrictPlannerRequired,
    ] {
        assert!(
            matrix.iter().any(|row| {
                row.mode == mode && row.stage == MatrixStageV1::FactsAbsentOrNonMatch
            }),
            "facts/no-match row missing for {mode:?}"
        );
        for route in [LoopRouteId::GenericLoopV0, LoopRouteId::GenericLoopV1] {
            for stage in required_stages(mode) {
                assert!(
                    matrix.iter().any(|row| {
                        row.mode == mode && row.route == Some(route) && row.stage == *stage
                    }),
                    "matrix arm missing mode={mode:?} route={route:?} stage={stage:?}"
                );
            }
        }
    }

    assert!(
        matrix.iter().any(|row| {
            row.fixture == FixtureClassV1::V0Only && row.evidence != MatrixEvidenceV1::Observed
        }),
        "the current corpus must keep the unproven V0-only class explicit"
    );
}

#[test]
fn generic_stage_matrix_never_calls_effectful_failure_a_decline() {
    for row in build_matrix() {
        if row.outcome == MatrixOutcomeV1::Failed && row.first_effect_owner != EffectOwnerV1::None {
            assert_ne!(
                row.disposition,
                Some(MatrixDispositionV1::PreEffectDeclined),
                "effectful row was relabelled as decline: {row:?}"
            );
        }
    }
}
