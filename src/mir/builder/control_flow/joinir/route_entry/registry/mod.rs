//! Phase 1: Registry-ize recipe-first routing (router becomes thin).
//! This module defines the ordered recipe-first entries and their handlers.

use crate::mir::builder::control_flow::lower::normalize::CanonicalLoopFacts;
use crate::mir::builder::MirBuilder;
use crate::mir::ValueId;

use super::router::LoopRouteContext;

mod direct_accum_const_loop_terminality;
mod direct_if_phi_join_terminality;
mod direct_loop_break_terminality;
mod direct_loop_continue_only_terminality;
mod direct_simple_while_terminality;
mod execution_witness;
mod handlers;
mod legacy_observer;
mod legacy_receipt;
pub(in crate::mir::builder) mod live_ordered_terminality;
mod live_preflight_frame;
mod loop_preflight;
mod predicates;
/// Compatibility facade; producer identity now lives beside the portable
/// artifact and remains non-semantic provenance.
pub(crate) mod route_id {
    pub(crate) use crate::mir::loop_recipe_contract::route_id::{entry_keys, LoopRouteId};
}
mod selection;
mod types;
mod utils;

pub(crate) use execution_witness::RouteExecutionWitnessV1;
#[cfg(test)]
pub(crate) use execution_witness::{
    execute_legacy_policy_parity_v1, LegacyPolicyAttemptDispositionV1, LegacyPolicyParityReceiptV1,
};
use execution_witness::{RouteAttemptOutcomeV1, RouteExecutionResultV1};
use handlers::*;
#[cfg(test)]
pub(crate) use legacy_observer::effective_route_for_test;
pub(crate) use live_preflight_frame::LivePreflightFrameV1;
pub(crate) use live_preflight_frame::{issue_live_preflight_frame, observe_all_route_preflight_v1};
use predicates::*;
use route_id::{entry_keys, LoopRouteId};
pub(crate) use selection::{select_recipe_first_routes, RecipeFirstRouteSelectionV1};
use types::LegacyRouteSuccess;
pub(crate) use types::{Entry, RouterEnv, SharedAbsentContractDeclineRouteV1};

pub(crate) fn collect_b_lite_shadow_report(
    selection: &RecipeFirstRouteSelectionV1,
) -> legacy_observer::LoopRouteShadowReport {
    legacy_observer::shadow_report(selection)
}

pub(crate) const ENTRIES: &[Entry] = &[
    Entry {
        id: LoopRouteId::LoopBreakRecipe,
        name: entry_keys::LOOP_BREAK_RECIPE,
        predicate: pred_loop_break_recipe,
        route: Some(route_loop_break_recipe),
    },
    Entry {
        id: LoopRouteId::IfPhiJoin,
        name: entry_keys::IF_PHI_JOIN,
        predicate: pred_if_phi_join,
        route: Some(route_if_phi_join),
    },
    Entry {
        id: LoopRouteId::LoopContinueOnly,
        name: entry_keys::LOOP_CONTINUE_ONLY,
        predicate: pred_loop_continue_only,
        route: Some(route_loop_continue_only),
    },
    Entry {
        id: LoopRouteId::LoopTrueEarlyExit,
        name: entry_keys::LOOP_TRUE_EARLY_EXIT,
        predicate: pred_loop_true_early_exit,
        route: Some(route_loop_true_early_exit),
    },
    Entry {
        id: LoopRouteId::LoopSimpleWhile,
        name: entry_keys::LOOP_SIMPLE_WHILE,
        predicate: pred_loop_simple_while,
        route: Some(route_loop_simple_while),
    },
    Entry {
        id: LoopRouteId::LoopCharMap,
        name: entry_keys::LOOP_CHAR_MAP,
        predicate: pred_loop_char_map,
        route: Some(route_loop_char_map),
    },
    Entry {
        id: LoopRouteId::LoopArrayJoin,
        name: entry_keys::LOOP_ARRAY_JOIN,
        predicate: pred_loop_array_join,
        route: Some(route_loop_array_join),
    },
    Entry {
        id: LoopRouteId::ScanWithInit,
        name: entry_keys::SCAN_WITH_INIT,
        predicate: pred_scan_with_init,
        route: Some(route_scan_with_init),
    },
    Entry {
        id: LoopRouteId::SplitScan,
        name: entry_keys::SPLIT_SCAN,
        predicate: pred_split_scan,
        route: Some(route_split_scan),
    },
    Entry {
        id: LoopRouteId::BoolPredicateScan,
        name: entry_keys::BOOL_PREDICATE_SCAN,
        predicate: pred_bool_predicate_scan,
        route: Some(route_bool_predicate_scan),
    },
    Entry {
        id: LoopRouteId::AccumConstLoop,
        name: entry_keys::ACCUM_CONST_LOOP,
        predicate: pred_accum_const_loop,
        route: Some(route_accum_const_loop),
    },
    Entry {
        id: LoopRouteId::NestedLoopMinimal,
        name: entry_keys::NESTED_LOOP_MINIMAL,
        predicate: pred_nested_loop_minimal,
        route: Some(route_nested_loop_minimal),
    },
    Entry {
        id: LoopRouteId::LoopTrueBreakContinue,
        name: entry_keys::LOOP_TRUE_BREAK_CONTINUE,
        predicate: pred_loop_true_break_continue,
        route: Some(route_loop_true_break_continue),
    },
    Entry {
        id: LoopRouteId::LoopCondBreakContinue,
        name: entry_keys::LOOP_COND_BREAK_CONTINUE,
        predicate: pred_loop_cond_break_continue,
        route: Some(route_loop_cond_break_continue),
    },
    Entry {
        id: LoopRouteId::LoopCondContinueOnly,
        name: entry_keys::LOOP_COND_CONTINUE_ONLY,
        predicate: pred_loop_cond_continue_only,
        route: Some(route_loop_cond_continue_only),
    },
    Entry {
        id: LoopRouteId::LoopCondContinueWithReturn,
        name: entry_keys::LOOP_COND_CONTINUE_WITH_RETURN,
        predicate: pred_loop_cond_continue_with_return,
        route: Some(route_loop_cond_continue_with_return),
    },
    Entry {
        id: LoopRouteId::LoopCondReturnInBody,
        name: entry_keys::LOOP_COND_RETURN_IN_BODY,
        predicate: pred_loop_cond_return_in_body,
        route: Some(route_loop_cond_return_in_body),
    },
    Entry {
        id: LoopRouteId::GenericLoopV0,
        name: entry_keys::GENERIC_LOOP_V0,
        predicate: pred_generic_loop_v0,
        route: Some(route_generic_loop_v0_at_attempt),
    },
    Entry {
        id: LoopRouteId::GenericLoopV1,
        name: entry_keys::GENERIC_LOOP_V1,
        predicate: pred_generic_loop_v1,
        route: Some(route_generic_loop_v1_at_attempt),
    },
];

pub(crate) fn collect_candidates(facts: Option<&CanonicalLoopFacts>) -> Vec<&'static str> {
    select_recipe_first_routes(facts).diagnostic_effective_names()
}

pub(crate) fn try_execute_route_execution_witness(
    builder: &mut MirBuilder,
    ctx: &LoopRouteContext,
    compose_facts: Option<&CanonicalLoopFacts>,
    witness: RouteExecutionWitnessV1<'_>,
) -> Result<Option<LegacyRouteSuccess>, String> {
    witness
        .execute_selected_in_order(|_execution, attempt| {
            dispatch_entry(builder, ctx, compose_facts, attempt)
        })
        .map(|result| match result {
            RouteExecutionResultV1::Succeeded { route, value } => {
                Some(LegacyRouteSuccess { route, value })
            }
            RouteExecutionResultV1::Exhausted(_) => None,
        })
}

fn dispatch_entry(
    builder: &mut MirBuilder,
    ctx: &LoopRouteContext,
    compose_facts: Option<&CanonicalLoopFacts>,
    attempt: &execution_witness::RouteExecutionAttemptV1<'_, '_>,
) -> Result<RouteAttemptOutcomeV1<ValueId>, String> {
    let route_id = attempt.current_route();
    let entry = ENTRIES
        .iter()
        .find(|entry| entry.id == route_id)
        .expect("recipe-first selection route must be present in ENTRIES");
    let Some(route) = entry.route else {
        return Err(crate::mir::builder::control_flow::lower::Freeze::contract(
            "selected recipe-first route has no execution handler",
        )
        .to_string());
    };
    route(builder, ctx, compose_facts, attempt)
}

#[cfg(test)]
mod effect_order_matrix_tests;
#[cfg(test)]
mod generic_accepted_plan_reachability_tests;
#[cfg(test)]
mod generic_nested_carrier_bindingref_tests;
#[cfg(test)]
mod generic_nested_carrier_winner_tests;
#[cfg(test)]
mod generic_nested_if_carrier_evidence_tests;
#[cfg(test)]
mod generic_resolved_carrier_both_norecursive_tests;
#[cfg(test)]
mod generic_resolved_carrier_candidate_stage_source_bridge_tests;
#[cfg(test)]
mod generic_resolved_carrier_compound_unavailable_tests;
#[cfg(test)]
mod generic_resolved_carrier_eligibility_protocol_tests;
#[cfg(test)]
mod generic_resolved_carrier_handoff_protocol_tests;
#[cfg(test)]
mod generic_resolved_carrier_index_ambiguous_tests;
#[cfg(test)]
mod generic_resolved_carrier_planner_suppression_tests;
#[cfg(test)]
mod generic_resolved_carrier_projector_tests;
#[cfg(test)]
mod generic_resolved_carrier_provenance_observation_tests;
#[cfg(test)]
mod generic_resolved_carrier_provenance_product_tests;
#[cfg(test)]
mod generic_resolved_carrier_facts_snapshot_tests;
#[cfg(test)]
mod generic_resolved_carrier_toplevel_compound_premise_tests;
#[cfg(test)]
mod generic_resolved_carrier_v1only_local_tests;
#[cfg(test)]
mod generic_selection_matrix_tests;
#[cfg(test)]
mod generic_semantic_digest_tests;
#[cfg(test)]
mod generic_stage_matrix_tests;
#[cfg(test)]
mod generic_stage_observer_tests;
#[cfg(test)]
mod nested_effective_winner_tests;
#[cfg(test)]
mod scoped_nongeneric_cutover_tests;

#[cfg(test)]
mod tests {
    use super::select_recipe_first_routes;

    #[test]
    fn empty_facts_select_no_recipe_first_routes() {
        let selection = select_recipe_first_routes(None);

        assert!(!selection.facts_present());
        assert!(selection.matched_routes().is_empty());
        assert!(selection.raw_execution_routes().is_empty());
        assert!(selection.diagnostic_effective_routes().is_empty());
    }
}

#[cfg(test)]
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum LegacyEffectiveWinnerReceiptV1 {
    Succeeded {
        winner: LoopRouteId,
        attempted: Box<[LoopRouteId]>,
    },
    Exhausted {
        attempted: Box<[LoopRouteId]>,
    },
}

#[cfg(test)]
pub(crate) fn test_legacy_effective_winner_v1(
    builder: &mut MirBuilder,
    ctx: &LoopRouteContext<'_>,
    strict_or_dev: bool,
    planner_required: bool,
) -> Result<LegacyEffectiveWinnerReceiptV1, String> {
    let outcome = crate::mir::builder::control_flow::plan::single_planner::try_build_outcome(ctx)?;
    let frame = super::router::test_issue_live_preflight_frame(
        ctx,
        &outcome,
        strict_or_dev,
        planner_required,
    );
    let Some(witness) = frame.test_witness_if_allowed() else {
        return Err("legacy effective-winner oracle was preflight-blocked".to_string());
    };
    let mut attempted = Vec::new();
    let result = witness.execute_selected_in_order(|_, attempt| {
        attempted.push(attempt.current_route());
        dispatch_entry(builder, ctx, outcome.facts.as_ref(), attempt)
    })?;
    Ok(match result {
        RouteExecutionResultV1::Succeeded { route, .. } => {
            LegacyEffectiveWinnerReceiptV1::Succeeded {
                winner: route,
                attempted: attempted.into_boxed_slice(),
            }
        }
        RouteExecutionResultV1::Exhausted(_) => LegacyEffectiveWinnerReceiptV1::Exhausted {
            attempted: attempted.into_boxed_slice(),
        },
    })
}
