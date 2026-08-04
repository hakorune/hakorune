//! D2-B4-S1: test-only certificate evidence for the Generic `Both` row.
//!
//! This module observes existing facts, stage, and legacy-witness products.
//! It issues no route policy and never publishes a Recipe, JoinSig, PHI, or
//! physical candidate.

use super::generic_accepted_plan_reachability_tests::{
    nested_carrier_evidence, observe_generic_carrier_facts, CorpusModeV1, EffectOwnerV1,
    PlanStageV1,
};
use super::generic_selection_matrix_tests::both_body;
use super::generic_stage_observer_tests::{
    observe_both_evidence, GenericOverlapEvidenceRowV1, ObserverModeV1,
};
use super::route_id::LoopRouteId;
use crate::mir::builder::control_flow::plan::facts::GenericLoopCarrierObservationV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CertificateDispositionV1 {
    V1ForNestedCarriers,
    UnresolvedStop,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CertificateInputV1 {
    source_row: &'static str,
    planner_required: bool,
    contract_present: bool,
    frame_source_matches: bool,
    raw_schedule: Vec<LoopRouteId>,
    observation: GenericLoopCarrierObservationV1,
    recursive_labels: Vec<String>,
    outer_final_labels: Vec<String>,
    outer_carrier_final_labels: Vec<String>,
    outer_phi_tags: Vec<String>,
    v1_stage: Option<PlanStageV1>,
    v1_first_effect_owner: Option<EffectOwnerV1>,
    fresh_repeat_stable: bool,
    legacy_attempted: Vec<LoopRouteId>,
    legacy_debt_count: usize,
    legacy_terminal: super::generic_stage_observer_tests::TerminalTraceV1,
}

fn corpus_mode(mode: ObserverModeV1) -> CorpusModeV1 {
    match mode {
        ObserverModeV1::Release => CorpusModeV1::Release,
        ObserverModeV1::Strict => CorpusModeV1::Strict,
        ObserverModeV1::StrictPlannerRequired => CorpusModeV1::StrictPlannerRequired,
    }
}

fn source_labels(observation: &GenericLoopCarrierObservationV1) -> Vec<String> {
    match observation {
        GenericLoopCarrierObservationV1::CompleteRecursiveCarrier(labels) => labels.clone(),
        GenericLoopCarrierObservationV1::CompleteNoRecursiveCarrier
        | GenericLoopCarrierObservationV1::Unavailable(_)
        | GenericLoopCarrierObservationV1::Ambiguous(_) => Vec::new(),
    }
}

fn required_carrier_tags(labels: &[String]) -> Vec<String> {
    labels
        .iter()
        .flat_map(|label| {
            [
                format!("loop_carrier_{label}"),
                format!("loop_step_in_{label}"),
            ]
        })
        .collect()
}

fn carrier_projected_final_labels(final_labels: &[String], phi_tags: &[String]) -> Vec<String> {
    final_labels
        .iter()
        .filter(|label| {
            let carrier = format!("loop_carrier_{label}");
            let step_in = format!("loop_step_in_{label}");
            phi_tags.iter().any(|tag| tag == &carrier) && phi_tags.iter().any(|tag| tag == &step_in)
        })
        .cloned()
        .collect()
}

fn v1_stage_fields(
    evidence: &GenericOverlapEvidenceRowV1,
) -> (Option<PlanStageV1>, Option<EffectOwnerV1>) {
    evidence
        .direct
        .iter()
        .find(|row| row.route == LoopRouteId::GenericLoopV1)
        .map(|row| (Some(row.stage), Some(row.first_effect_owner)))
        .unwrap_or((None, None))
}

fn build_input(mode: ObserverModeV1) -> CertificateInputV1 {
    let (observation, raw_schedule) = observe_generic_carrier_facts(corpus_mode(mode), "both");
    let evidence = observe_both_evidence(mode);
    let v1_plan = nested_carrier_evidence(LoopRouteId::GenericLoopV1);
    let recursive_labels = source_labels(&observation);
    let (v1_stage, v1_first_effect_owner) = v1_stage_fields(&evidence);
    let repeat_observation = observe_generic_carrier_facts(corpus_mode(mode), "both");
    let repeat_evidence = observe_both_evidence(mode);
    let repeat_v1_plan = nested_carrier_evidence(LoopRouteId::GenericLoopV1);
    let fresh_repeat_stable = repeat_observation.0 == observation
        && repeat_observation.1 == raw_schedule
        && v1_stage_fields(&repeat_evidence) == (v1_stage, v1_first_effect_owner)
        && repeat_v1_plan.outer_final_value_names == v1_plan.outer_final_value_names
        && repeat_v1_plan.outer_phi_tags == v1_plan.outer_phi_tags
        && repeat_evidence.witness == evidence.witness;

    assert_eq!(evidence.witness.raw_schedule, raw_schedule);
    assert_eq!(
        evidence.witness.carrier_observation,
        Some(observation.clone())
    );
    assert_eq!(v1_plan.route, LoopRouteId::GenericLoopV1);

    let outer_final_labels = v1_plan.outer_final_value_names;
    let outer_phi_tags = v1_plan.outer_phi_tags;
    let outer_carrier_final_labels =
        carrier_projected_final_labels(&outer_final_labels, &outer_phi_tags);
    CertificateInputV1 {
        source_row: "both",
        planner_required: mode.planner_required(),
        contract_present: evidence.witness.frame.recipe_contract_present,
        frame_source_matches: evidence.witness.carrier_observation == Some(observation.clone()),
        raw_schedule,
        observation,
        recursive_labels,
        outer_final_labels,
        outer_carrier_final_labels,
        outer_phi_tags,
        v1_stage,
        v1_first_effect_owner,
        fresh_repeat_stable,
        legacy_attempted: evidence
            .witness
            .attempted
            .iter()
            .map(|attempt| attempt.route)
            .collect(),
        legacy_debt_count: evidence.witness.generic_debts.len(),
        legacy_terminal: evidence.witness.terminal,
    }
}

fn evaluate_certificate(input: &CertificateInputV1) -> CertificateDispositionV1 {
    let GenericLoopCarrierObservationV1::CompleteRecursiveCarrier(observed_labels) =
        &input.observation
    else {
        return CertificateDispositionV1::UnresolvedStop;
    };
    let projected_outer_labels =
        carrier_projected_final_labels(&input.outer_final_labels, &input.outer_phi_tags);
    if input.source_row != "both"
        || input.planner_required
        || !input.frame_source_matches
        || input.raw_schedule.as_slice() != [LoopRouteId::GenericLoopV0, LoopRouteId::GenericLoopV1]
        || observed_labels != &input.recursive_labels
        || input.outer_carrier_final_labels != projected_outer_labels
        || input.recursive_labels != projected_outer_labels
        || input.v1_stage != Some(PlanStageV1::LowerSome)
        || input.v1_first_effect_owner != Some(EffectOwnerV1::GenericComposer)
        || !input.fresh_repeat_stable
    {
        return CertificateDispositionV1::UnresolvedStop;
    }
    let required_tags = required_carrier_tags(&input.recursive_labels);
    if !required_tags.iter().all(|tag| {
        input
            .outer_phi_tags
            .iter()
            .any(|candidate| candidate == tag)
    }) {
        return CertificateDispositionV1::UnresolvedStop;
    }
    CertificateDispositionV1::V1ForNestedCarriers
}

#[test]
fn generic_d2_b4_both_rows_issue_only_test_certificate() {
    for mode in [ObserverModeV1::Release, ObserverModeV1::Strict] {
        let input = build_input(mode);
        assert!(!input.contract_present, "contract bit is evidence only");
        assert_eq!(
            evaluate_certificate(&input),
            CertificateDispositionV1::V1ForNestedCarriers,
            "natural {mode:?} Both row must qualify as a test-only candidate"
        );
        assert_eq!(input.legacy_debt_count, 0);
        assert_eq!(
            input.legacy_attempted,
            vec![LoopRouteId::GenericLoopV0],
            "legacy witness must retain its V0 prefix"
        );
        assert_eq!(
            input.legacy_terminal,
            super::generic_stage_observer_tests::TerminalTraceV1::Succeeded(
                LoopRouteId::GenericLoopV0
            )
        );
        assert_eq!(
            input.raw_schedule,
            vec![LoopRouteId::GenericLoopV0, LoopRouteId::GenericLoopV1]
        );
    }
}

#[test]
fn generic_d2_b4_planner_required_is_separate_and_unresolved() {
    let input = build_input(ObserverModeV1::StrictPlannerRequired);
    assert_eq!(input.raw_schedule, vec![LoopRouteId::GenericLoopV1]);
    assert!(input.planner_required);
    assert_eq!(
        evaluate_certificate(&input),
        CertificateDispositionV1::UnresolvedStop
    );
}

#[test]
fn generic_d2_b4_certificate_rejects_incomplete_or_mismatched_rows() {
    let baseline = build_input(ObserverModeV1::Strict);
    let mut cases = Vec::new();

    let mut no_recursive = baseline.clone();
    no_recursive.observation = GenericLoopCarrierObservationV1::CompleteNoRecursiveCarrier;
    cases.push(no_recursive);

    let mut unavailable = baseline.clone();
    unavailable.observation = GenericLoopCarrierObservationV1::Unavailable("LoopRange".into());
    cases.push(unavailable);

    let mut ambiguous = baseline.clone();
    ambiguous.observation = GenericLoopCarrierObservationV1::Ambiguous("target".into());
    cases.push(ambiguous);

    let mut no_overlap = baseline.clone();
    no_overlap.raw_schedule = vec![LoopRouteId::GenericLoopV1];
    cases.push(no_overlap);

    let mut stage_missing = baseline.clone();
    stage_missing.v1_stage = None;
    cases.push(stage_missing);

    let mut stage_failed = baseline.clone();
    stage_failed.v1_stage = Some(PlanStageV1::LowerError);
    cases.push(stage_failed);

    let mut target_mismatch = baseline.clone();
    target_mismatch.outer_final_labels = vec!["i".into()];
    cases.push(target_mismatch);

    let mut unstable = baseline.clone();
    unstable.fresh_repeat_stable = false;
    cases.push(unstable);

    for case in cases {
        assert_eq!(
            evaluate_certificate(&case),
            CertificateDispositionV1::UnresolvedStop,
            "invalid certificate input must remain unresolved: {case:?}"
        );
    }
}

#[test]
fn generic_d2_b4_contract_bit_is_recorded_but_not_a_winner_gate() {
    let mut input = build_input(ObserverModeV1::Strict);
    assert!(!input.contract_present);
    input.contract_present = true;
    assert_eq!(
        evaluate_certificate(&input),
        CertificateDispositionV1::V1ForNestedCarriers,
        "contract_present must not become a hidden policy requirement"
    );
}

#[test]
fn generic_d2_b4_source_fixture_remains_the_existing_both_shape() {
    let body = both_body();
    assert!(!body.is_empty());
    let input = build_input(ObserverModeV1::Release);
    assert_eq!(input.source_row, "both");
    assert_eq!(input.recursive_labels, vec!["j".to_string()]);
    assert_eq!(
        input.outer_final_labels,
        vec!["i".to_string(), "j".to_string()]
    );
    assert_eq!(input.outer_carrier_final_labels, vec!["j".to_string()]);
}
