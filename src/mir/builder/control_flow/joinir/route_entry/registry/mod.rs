//! Phase 1: Registry-ize recipe-first routing (router becomes thin).
//! This module defines the ordered recipe-first entries and their handlers.

use crate::mir::builder::control_flow::lower::normalize::CanonicalLoopFacts;
use crate::mir::builder::control_flow::lower::PlanBuildOutcome;
use crate::mir::builder::MirBuilder;

use super::router::LoopRouteContext;

mod direct_simple_while_terminality;
mod handlers;
mod legacy_observer;
mod logical_demand;
mod predicates;
mod route_id;
mod selection;
mod types;
mod utils;

use handlers::*;
use predicates::*;
use route_id::{entry_keys, LoopRouteId};
pub(crate) use selection::{select_recipe_first_routes, RecipeFirstRouteSelectionV1};
use types::LegacyRouteSuccess;
pub(crate) use types::{Entry, RouterEnv};

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
        route: Some(route_generic_loop_v0),
    },
    Entry {
        id: LoopRouteId::GenericLoopV1,
        name: entry_keys::GENERIC_LOOP_V1,
        predicate: pred_generic_loop_v1,
        route: Some(route_generic_loop_v1),
    },
];

pub(crate) fn collect_candidates(facts: Option<&CanonicalLoopFacts>) -> Vec<&'static str> {
    select_recipe_first_routes(facts).diagnostic_effective_names()
}

pub(crate) fn try_route_recipe_first_with_success(
    builder: &mut MirBuilder,
    ctx: &LoopRouteContext,
    outcome: &PlanBuildOutcome,
    env: &RouterEnv,
) -> Result<Option<LegacyRouteSuccess>, String> {
    let selection = select_recipe_first_routes(outcome.facts.as_ref());
    try_execute_recipe_first_selection(builder, ctx, outcome, env, &selection)
}

pub(crate) fn try_execute_recipe_first_selection(
    builder: &mut MirBuilder,
    ctx: &LoopRouteContext,
    outcome: &PlanBuildOutcome,
    env: &RouterEnv,
    selection: &RecipeFirstRouteSelectionV1,
) -> Result<Option<LegacyRouteSuccess>, String> {
    execute_selected_routes_in_order(selection.raw_execution_routes(), |route_id| {
        let entry = ENTRIES
            .iter()
            .find(|entry| entry.id == route_id)
            .expect("recipe-first selection route must be present in ENTRIES");
        let Some(route) = entry.route else {
            return Ok(None);
        };
        if let Some(value) = route(builder, ctx, outcome, env)? {
            return Ok(Some(value));
        }
        Ok(None)
    })
    .map(|success| success.map(|(route, value)| LegacyRouteSuccess { route, value }))
}

fn execute_selected_routes_in_order<T, E>(
    routes: &[LoopRouteId],
    mut execute: impl FnMut(LoopRouteId) -> Result<Option<T>, E>,
) -> Result<Option<(LoopRouteId, T)>, E> {
    for route in routes {
        if let Some(value) = execute(*route)? {
            return Ok(Some((*route, value)));
        }
    }
    Ok(None)
}

#[cfg(test)]
mod effect_order_matrix_tests;

#[cfg(test)]
mod tests {
    use super::{execute_selected_routes_in_order, select_recipe_first_routes};
    use crate::mir::builder::control_flow::joinir::route_entry::registry::route_id::LoopRouteId;

    #[test]
    fn empty_facts_select_no_recipe_first_routes() {
        let selection = select_recipe_first_routes(None);

        assert!(!selection.facts_present());
        assert!(selection.matched_routes().is_empty());
        assert!(selection.raw_execution_routes().is_empty());
        assert!(selection.diagnostic_effective_routes().is_empty());
    }

    #[test]
    fn raw_execution_continues_after_a_selected_route_returns_none() {
        let routes = [LoopRouteId::LoopSimpleWhile, LoopRouteId::GenericLoopV1];
        let mut attempted = Vec::new();

        let result = execute_selected_routes_in_order(&routes, |route| {
            attempted.push(route);
            Ok::<_, ()>((route == LoopRouteId::GenericLoopV1).then_some(7_u8))
        });

        assert_eq!(result, Ok(Some((LoopRouteId::GenericLoopV1, 7_u8))));
        assert_eq!(attempted, routes);
    }

    #[test]
    fn raw_execution_propagates_error_without_trying_later_routes() {
        let routes = [LoopRouteId::LoopSimpleWhile, LoopRouteId::GenericLoopV1];
        let mut attempted = Vec::new();

        let result = execute_selected_routes_in_order(&routes, |route| {
            attempted.push(route);
            if route == LoopRouteId::LoopSimpleWhile {
                Err("route failed")
            } else {
                Ok(Some(7_u8))
            }
        });

        assert_eq!(result, Err("route failed"));
        assert_eq!(attempted, [LoopRouteId::LoopSimpleWhile]);
    }
}
