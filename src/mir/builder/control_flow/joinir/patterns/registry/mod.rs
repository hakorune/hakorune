//! Phase 1: Registry-ize recipe-first routing (router becomes thin).
//! This module defines the ordered recipe-first entries and their handlers.

use crate::mir::builder::MirBuilder;
use crate::mir::ValueId;
use crate::mir::builder::control_flow::plan::normalize::CanonicalLoopFacts;
use crate::mir::builder::control_flow::plan::planner::PlanBuildOutcome;

use super::router::LoopRouteContext;

mod types;
mod predicates;
mod handlers;
mod utils;

pub(crate) use types::{RouterEnv, Entry};
use types::entry_keys;
use predicates::*;
use handlers::*;

pub(crate) const ENTRIES: &[Entry] = &[
    Entry {
        name: entry_keys::LOOP_BREAK_RECIPE,
        predicate: pred_loop_break_recipe,
        route: Some(route_loop_break_recipe),
    },
    Entry {
        name: entry_keys::IF_PHI_JOIN,
        predicate: pred_if_phi_join,
        route: Some(route_if_phi_join),
    },
    Entry {
        name: entry_keys::LOOP_CONTINUE_ONLY,
        predicate: pred_loop_continue_only_recipe,
        route: Some(route_loop_continue_only),
    },
    Entry {
        name: entry_keys::LOOP_TRUE_EARLY_EXIT,
        predicate: pred_loop_true_early_exit,
        route: Some(route_loop_true_early_exit),
    },
    Entry {
        name: entry_keys::LOOP_SIMPLE_WHILE,
        predicate: pred_loop_simple_while,
        route: Some(route_loop_simple_while),
    },
    Entry {
        name: entry_keys::LOOP_CHAR_MAP,
        predicate: pred_loop_char_map,
        route: Some(route_loop_char_map),
    },
    Entry {
        name: entry_keys::LOOP_ARRAY_JOIN,
        predicate: pred_loop_array_join,
        route: Some(route_loop_array_join),
    },
    Entry {
        name: entry_keys::SCAN_WITH_INIT,
        predicate: pred_scan_with_init,
        route: Some(route_scan_with_init),
    },
    Entry {
        name: entry_keys::SPLIT_SCAN,
        predicate: pred_split_scan,
        route: Some(route_split_scan),
    },
    Entry {
        name: entry_keys::BOOL_PREDICATE_SCAN,
        predicate: pred_bool_predicate_scan,
        route: Some(route_bool_predicate_scan),
    },
    Entry {
        name: entry_keys::ACCUM_CONST_LOOP,
        predicate: pred_accum_const_loop,
        route: Some(route_accum_const_loop),
    },
    Entry {
        name: entry_keys::LOOP_SCAN_METHODS_V0,
        predicate: pred_loop_scan_methods_v0,
        route: Some(route_loop_scan_methods_v0),
    },
    Entry {
        name: entry_keys::LOOP_SCAN_METHODS_BLOCK_V0,
        predicate: pred_loop_scan_methods_block_v0,
        route: Some(route_loop_scan_methods_block_v0),
    },
    Entry {
        name: entry_keys::LOOP_SCAN_PHI_VARS_V0,
        predicate: pred_loop_scan_phi_vars_v0,
        route: Some(route_loop_scan_phi_vars_v0),
    },
    Entry {
        name: entry_keys::LOOP_SCAN_V0,
        predicate: pred_loop_scan_v0,
        route: Some(route_loop_scan_v0),
    },
    Entry {
        name: entry_keys::LOOP_COLLECT_USING_ENTRIES_V0,
        predicate: pred_loop_collect_using_entries_v0,
        route: Some(route_loop_collect_using_entries_v0),
    },
    Entry {
        name: entry_keys::NESTED_LOOP_MINIMAL,
        predicate: pred_nested_loop_minimal,
        route: Some(route_nested_loop_minimal),
    },
    Entry {
        name: entry_keys::LOOP_BUNDLE_RESOLVER_V0,
        predicate: pred_loop_bundle_resolver_v0,
        route: Some(route_loop_bundle_resolver_v0),
    },
    Entry {
        name: entry_keys::LOOP_TRUE_BREAK_CONTINUE,
        predicate: pred_loop_true_break_continue,
        route: Some(route_loop_true_break_continue),
    },
    Entry {
        name: entry_keys::LOOP_COND_BREAK_CONTINUE,
        predicate: pred_loop_cond_break_continue,
        route: Some(route_loop_cond_break_continue),
    },
    Entry {
        name: entry_keys::LOOP_COND_CONTINUE_ONLY,
        predicate: pred_loop_cond_continue_only,
        route: Some(route_loop_cond_continue_only),
    },
    Entry {
        name: entry_keys::LOOP_COND_CONTINUE_WITH_RETURN,
        predicate: pred_loop_cond_continue_with_return,
        route: Some(route_loop_cond_continue_with_return),
    },
    Entry {
        name: entry_keys::LOOP_COND_RETURN_IN_BODY,
        predicate: pred_loop_cond_return_in_body,
        route: Some(route_loop_cond_return_in_body),
    },
    Entry {
        name: entry_keys::GENERIC_LOOP_V0,
        predicate: pred_generic_loop_v0,
        route: Some(route_generic_loop_v0),
    },
    Entry {
        name: entry_keys::GENERIC_LOOP_V1,
        predicate: pred_generic_loop_v1,
        route: Some(route_generic_loop_v1),
    },
];

struct CandidateSuppression {
    scan_methods_candidate: bool,
    if_phi_join_candidate: bool,
    loop_continue_only_candidate: bool,
    loop_true_early_exit_candidate: bool,
    array_join_candidate: bool,
}

fn should_skip_candidate(name: &str, suppression: &CandidateSuppression) -> bool {
    match name {
        entry_keys::LOOP_COND_BREAK_CONTINUE => {
            suppression.scan_methods_candidate
                || suppression.if_phi_join_candidate
                || suppression.loop_continue_only_candidate
                || suppression.array_join_candidate
        }
        entry_keys::LOOP_COND_CONTINUE_ONLY => suppression.loop_continue_only_candidate,
        entry_keys::LOOP_TRUE_BREAK_CONTINUE => suppression.loop_true_early_exit_candidate,
        _ => false,
    }
}

pub(crate) fn collect_candidates(facts: Option<&CanonicalLoopFacts>) -> Vec<&'static str> {
    let Some(facts) = facts else {
        return Vec::new();
    };
    let mut names = Vec::new();
    let suppression = CandidateSuppression {
        scan_methods_candidate:
            pred_loop_scan_methods_block_v0(facts) || pred_loop_scan_methods_v0(facts),
        if_phi_join_candidate: pred_if_phi_join(facts),
        loop_continue_only_candidate: pred_loop_continue_only_recipe(facts),
        loop_true_early_exit_candidate: pred_loop_true_early_exit(facts),
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

    let block_generic_loop_v1 = char_map_candidate
        || pred_loop_simple_while(facts)
        || pred_loop_bundle_resolver_v0(facts)
        || pred_nested_loop_minimal(facts);
    if block_generic_loop_v1 {
        names.retain(|name| *name != entry_keys::GENERIC_LOOP_V1);
    }
    names
}

pub(crate) fn try_route_recipe_first(
    builder: &mut MirBuilder,
    ctx: &LoopRouteContext,
    outcome: &PlanBuildOutcome,
    env: &RouterEnv,
) -> Result<Option<ValueId>, String> {
    let Some(facts) = outcome.facts.as_ref() else {
        return Ok(None);
    };
    for entry in ENTRIES {
        if !(entry.predicate)(facts) {
            continue;
        }
        let Some(route) = entry.route else {
            continue;
        };
        if let Some(value) = route(builder, ctx, outcome, env)? {
            return Ok(Some(value));
        }
    }
    Ok(None)
}
