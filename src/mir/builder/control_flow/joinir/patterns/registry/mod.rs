//! Phase 1: Registry-ize recipe-first routing (router becomes thin).
//! This module defines the ordered recipe-first entries and their handlers.

use crate::mir::builder::MirBuilder;
use crate::mir::ValueId;
use crate::mir::builder::control_flow::plan::normalize::CanonicalLoopFacts;
use crate::mir::builder::control_flow::plan::planner::PlanBuildOutcome;

use super::router::LoopPatternContext;

mod types;
mod predicates;
mod handlers;
mod utils;

pub(crate) use types::{RouterEnv, Entry};
use predicates::*;
use handlers::*;

pub(crate) const ENTRIES: &[Entry] = &[
    Entry {
        name: "loop_break_recipe",
        predicate: pred_pattern2_break,
        route: Some(route_loop_break_recipe),
    },
    Entry {
        name: "if_phi_join",
        predicate: pred_pattern3_ifphi,
        route: Some(route_if_phi_join),
    },
    Entry {
        name: "loop_continue_only",
        predicate: pred_pattern4_continue,
        route: Some(route_loop_continue_only),
    },
    Entry {
        name: "loop_true_early_exit",
        predicate: pred_pattern5_infinite_early_exit,
        route: Some(route_loop_true_early_exit),
    },
    Entry {
        name: "loop_simple_while",
        predicate: pred_pattern1_simplewhile,
        route: Some(route_loop_simple_while),
    },
    Entry {
        name: "loop_char_map",
        predicate: pred_pattern1_char_map,
        route: Some(route_loop_char_map),
    },
    Entry {
        name: "loop_array_join",
        predicate: pred_pattern1_array_join,
        route: Some(route_loop_array_join),
    },
    Entry {
        name: "scan_with_init",
        predicate: pred_scan_with_init,
        route: Some(route_scan_with_init),
    },
    Entry {
        name: "split_scan",
        predicate: pred_split_scan,
        route: Some(route_split_scan),
    },
    Entry {
        name: "bool_predicate_scan",
        predicate: pred_pattern8_bool_predicate_scan,
        route: Some(route_bool_predicate_scan),
    },
    Entry {
        name: "accum_const_loop",
        predicate: pred_pattern9_accum_const_loop,
        route: Some(route_accum_const_loop),
    },
    Entry {
        name: "loop_scan_methods_v0",
        predicate: pred_loop_scan_methods_v0,
        route: Some(route_loop_scan_methods_v0),
    },
    Entry {
        name: "loop_scan_methods_block_v0",
        predicate: pred_loop_scan_methods_block_v0,
        route: Some(route_loop_scan_methods_block_v0),
    },
    Entry {
        name: "loop_scan_phi_vars_v0",
        predicate: pred_loop_scan_phi_vars_v0,
        route: Some(route_loop_scan_phi_vars_v0),
    },
    Entry {
        name: "loop_scan_v0",
        predicate: pred_loop_scan_v0,
        route: Some(route_loop_scan_v0),
    },
    Entry {
        name: "loop_collect_using_entries_v0",
        predicate: pred_loop_collect_using_entries_v0,
        route: Some(route_loop_collect_using_entries_v0),
    },
    Entry {
        name: "loop_bundle_resolver_v0",
        predicate: pred_loop_bundle_resolver_v0,
        route: Some(route_loop_bundle_resolver_v0),
    },
    Entry {
        name: "loop_true_break_continue",
        predicate: pred_loop_true_break_continue,
        route: Some(route_loop_true_break_continue),
    },
    Entry {
        name: "loop_cond_break_continue",
        predicate: pred_loop_cond_break_continue,
        route: Some(route_loop_cond_break_continue),
    },
    Entry {
        name: "loop_cond_continue_only",
        predicate: pred_loop_cond_continue_only,
        route: Some(route_loop_cond_continue_only),
    },
    Entry {
        name: "loop_cond_continue_with_return",
        predicate: pred_loop_cond_continue_with_return,
        route: Some(route_loop_cond_continue_with_return),
    },
    Entry {
        name: "loop_cond_return_in_body",
        predicate: pred_loop_cond_return_in_body,
        route: Some(route_loop_cond_return_in_body),
    },
    Entry {
        name: "generic_loop_v0",
        predicate: pred_generic_loop_v0,
        route: Some(route_generic_loop_v0),
    },
    Entry {
        name: "generic_loop_v1",
        predicate: pred_generic_loop_v1,
        route: Some(route_generic_loop_v1),
    },
];

pub(crate) fn collect_candidates(facts: Option<&CanonicalLoopFacts>) -> Vec<&'static str> {
    let Some(facts) = facts else {
        return Vec::new();
    };
    let mut names = Vec::new();
    let scan_methods_candidate =
        pred_loop_scan_methods_block_v0(facts) || pred_loop_scan_methods_v0(facts);
    let pattern3_candidate = pred_pattern3_ifphi(facts);
    let pattern4_candidate = pred_pattern4_continue(facts);
    let pattern5_candidate = pred_pattern5_infinite_early_exit(facts);
    let array_join_candidate = pred_pattern1_array_join(facts);
    let char_map_candidate = pred_pattern1_char_map(facts);
    for entry in ENTRIES {
        if entry.name == "loop_cond_break_continue" && scan_methods_candidate {
            continue;
        }
        if entry.name == "loop_cond_break_continue" && pattern3_candidate {
            continue;
        }
        if entry.name == "loop_cond_break_continue" && pattern4_candidate {
            continue;
        }
        if entry.name == "loop_cond_break_continue" && array_join_candidate {
            continue;
        }
        if entry.name == "loop_cond_continue_only" && pattern4_candidate {
            continue;
        }
        if entry.name == "loop_true_break_continue" && pattern5_candidate {
            continue;
        }
        if (entry.predicate)(facts) {
            names.push(entry.name);
        }
    }
    if char_map_candidate {
        names.retain(|name| *name != "generic_loop_v1");
    }
    if pred_pattern1_simplewhile(facts) {
        names.retain(|name| *name != "generic_loop_v1");
    }
    if pred_loop_bundle_resolver_v0(facts) {
        names.retain(|name| *name != "generic_loop_v1");
    }
    names
}

pub(crate) fn try_route_recipe_first(
    builder: &mut MirBuilder,
    ctx: &LoopPatternContext,
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