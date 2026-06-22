//! Phase 1: Registry-ize recipe-first routing (router becomes thin).
//! This module defines the ordered recipe-first entries and their handlers.

use crate::mir::builder::control_flow::lower::normalize::CanonicalLoopFacts;
use crate::mir::builder::control_flow::lower::PlanBuildOutcome;
use crate::mir::builder::MirBuilder;

use super::router::LoopRouteContext;

mod handlers;
mod legacy_observer;
mod predicates;
mod types;
mod utils;

use handlers::*;
use predicates::*;
use types::{entry_keys, LegacyRouteSuccess, LoopRouteId};
pub(crate) use types::{Entry, RouterEnv};

pub(crate) fn collect_b_lite_shadow_report(
    facts: Option<&CanonicalLoopFacts>,
) -> legacy_observer::LoopRouteShadowReport {
    legacy_observer::shadow_report(facts)
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
        route: Some(route_generic_loop_v0),
    },
    Entry {
        id: LoopRouteId::GenericLoopV1,
        name: entry_keys::GENERIC_LOOP_V1,
        predicate: pred_generic_loop_v1,
        route: Some(route_generic_loop_v1),
    },
];

struct CandidateSuppression {
    if_phi_join_candidate: bool,
    loop_continue_only_candidate: bool,
    loop_cond_continue_only_candidate: bool,
    loop_true_early_exit_candidate: bool,
    loop_true_break_continue_candidate: bool,
    array_join_candidate: bool,
}

fn should_skip_candidate(name: &str, suppression: &CandidateSuppression) -> bool {
    match name {
        entry_keys::LOOP_COND_BREAK_CONTINUE => {
            suppression.if_phi_join_candidate
                || suppression.loop_continue_only_candidate
                || suppression.loop_cond_continue_only_candidate
                || suppression.array_join_candidate
        }
        entry_keys::LOOP_COND_CONTINUE_ONLY => suppression.loop_continue_only_candidate,
        entry_keys::LOOP_TRUE_BREAK_CONTINUE => suppression.loop_true_early_exit_candidate,
        entry_keys::GENERIC_LOOP_V1 => suppression.loop_true_break_continue_candidate,
        _ => false,
    }
}

pub(crate) fn collect_candidates(facts: Option<&CanonicalLoopFacts>) -> Vec<&'static str> {
    let Some(facts) = facts else {
        return Vec::new();
    };
    let mut names = Vec::new();
    let suppression = CandidateSuppression {
        if_phi_join_candidate: pred_if_phi_join(facts),
        loop_continue_only_candidate: pred_loop_continue_only(facts),
        loop_cond_continue_only_candidate: pred_loop_cond_continue_only(facts),
        loop_true_early_exit_candidate: pred_loop_true_early_exit(facts),
        loop_true_break_continue_candidate: pred_loop_true_break_continue(facts),
        array_join_candidate: pred_loop_array_join(facts),
    };
    let char_map_candidate = pred_loop_char_map(facts);

    for entry in ENTRIES {
        if should_skip_candidate(entry.name, &suppression) {
            continue;
        }
        if (entry.predicate)(facts) {
            names.push(entry.name);
        }
    }

    let block_generic_loop_v1 =
        char_map_candidate || pred_loop_simple_while(facts) || pred_nested_loop_minimal(facts);
    if block_generic_loop_v1 {
        names.retain(|name| *name != entry_keys::GENERIC_LOOP_V1);
    }
    names
}

pub(crate) fn try_route_recipe_first_with_success(
    builder: &mut MirBuilder,
    ctx: &LoopRouteContext,
    outcome: &PlanBuildOutcome,
    env: &RouterEnv,
) -> Result<Option<LegacyRouteSuccess>, String> {
    let Some(facts) = outcome.facts.as_ref() else {
        return Ok(None);
    };
    let suppression = CandidateSuppression {
        if_phi_join_candidate: pred_if_phi_join(facts),
        loop_continue_only_candidate: pred_loop_continue_only(facts),
        loop_cond_continue_only_candidate: pred_loop_cond_continue_only(facts),
        loop_true_early_exit_candidate: pred_loop_true_early_exit(facts),
        loop_true_break_continue_candidate: pred_loop_true_break_continue(facts),
        array_join_candidate: pred_loop_array_join(facts),
    };
    for entry in ENTRIES {
        if should_skip_candidate(entry.name, &suppression) {
            continue;
        }
        if !(entry.predicate)(facts) {
            continue;
        }
        let Some(route) = entry.route else {
            continue;
        };
        if let Some(value) = route(builder, ctx, outcome, env)? {
            return Ok(Some(LegacyRouteSuccess {
                route: entry.id,
                value,
            }));
        }
    }
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::{entry_keys, should_skip_candidate, CandidateSuppression};

    fn suppression_with_loop_continue_only() -> CandidateSuppression {
        CandidateSuppression {
            if_phi_join_candidate: false,
            loop_continue_only_candidate: true,
            loop_cond_continue_only_candidate: false,
            loop_true_early_exit_candidate: false,
            loop_true_break_continue_candidate: false,
            array_join_candidate: false,
        }
    }

    #[test]
    fn loop_cond_continue_only_keeps_existing_loop_continue_only_suppression() {
        let suppression = suppression_with_loop_continue_only();

        assert!(should_skip_candidate(
            entry_keys::LOOP_COND_CONTINUE_ONLY,
            &suppression
        ));
    }

    #[test]
    fn loop_cond_break_continue_keeps_existing_loop_continue_only_suppression() {
        let suppression = suppression_with_loop_continue_only();

        assert!(should_skip_candidate(
            entry_keys::LOOP_COND_BREAK_CONTINUE,
            &suppression
        ));
    }

    #[test]
    fn generic_loop_v1_is_suppressed_when_loop_true_break_continue_owns_shape() {
        let suppression = CandidateSuppression {
            if_phi_join_candidate: false,
            loop_continue_only_candidate: false,
            loop_cond_continue_only_candidate: false,
            loop_true_early_exit_candidate: false,
            loop_true_break_continue_candidate: true,
            array_join_candidate: false,
        };

        assert!(should_skip_candidate(
            entry_keys::GENERIC_LOOP_V1,
            &suppression
        ));
        assert!(!should_skip_candidate(
            entry_keys::LOOP_TRUE_BREAK_CONTINUE,
            &suppression
        ));
    }
}
