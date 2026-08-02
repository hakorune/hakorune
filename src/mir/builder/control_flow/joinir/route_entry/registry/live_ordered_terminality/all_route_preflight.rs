use crate::mir::builder::control_flow::joinir::route_entry::registry::loop_preflight::{
    LoopPreflightDispositionV1, LoopPreflightRejectV1,
};
use crate::mir::builder::control_flow::joinir::route_entry::registry::route_id::LoopRouteId;
use crate::mir::builder::control_flow::lower::normalize::CanonicalLoopFacts;
use crate::mir::builder::control_flow::plan::facts::LoopFacts;
mod split_scan;
use split_scan::classify_split_scan;
mod bool_predicate_scan;
use bool_predicate_scan::classify_bool_predicate_scan;
pub(crate) fn observe_selected_preflight_v1(
    facts: Option<&CanonicalLoopFacts>,
    raw_schedule: &[LoopRouteId],
) -> LoopPreflightDispositionV1 {
    let Some(&front) = raw_schedule.first() else {
        return LoopPreflightDispositionV1::NoCandidate;
    };
    let Some(facts) = facts else {
        return LoopPreflightDispositionV1::Rejected(
            LoopPreflightRejectV1::SourceTopologyUnavailable { route: front },
        );
    };
    LoopPreflightDispositionV1::Rejected(classify_front(&facts.facts, front))
}
#[cfg(test)]
pub(crate) fn issue_all_route_preflight_v1(
    live: super::LiveLoopFactsV1<'_>,
) -> LoopPreflightDispositionV1 {
    let canonical =
        crate::mir::builder::control_flow::lower::normalize::canonicalize_loop_facts(live.facts);
    let selection = crate::mir::builder::control_flow::joinir::route_entry::registry::selection::select_recipe_first_routes(Some(&canonical));
    observe_selected_preflight_v1(Some(&canonical), selection.raw_execution_routes())
}
fn classify_front(facts: &LoopFacts, route: LoopRouteId) -> LoopPreflightRejectV1 {
    match route {
        LoopRouteId::LoopBreakRecipe => classify_loop_break(facts),
        LoopRouteId::IfPhiJoin => classify_if_phi_join(facts),
        LoopRouteId::LoopContinueOnly => classify_loop_continue_only(facts),
        LoopRouteId::LoopTrueEarlyExit => classify_loop_true_early_exit(facts),
        LoopRouteId::LoopCharMap => classify_loop_char_map(facts),
        LoopRouteId::LoopArrayJoin => classify_loop_array_join(facts),
        LoopRouteId::NestedLoopMinimal => classify_nested_loop_minimal(facts),
        LoopRouteId::ScanWithInit => classify_scan_with_init(facts),
        LoopRouteId::SplitScan => classify_split_scan(facts),
        LoopRouteId::BoolPredicateScan => classify_bool_predicate_scan(facts),
        LoopRouteId::LoopSimpleWhile => classify_simple_while(facts),
        LoopRouteId::AccumConstLoop => classify_accum_const(facts),
        LoopRouteId::GenericLoopV0 | LoopRouteId::GenericLoopV1 => {
            LoopPreflightRejectV1::PostEffectRetryDebt { route }
        }
        _ => LoopPreflightRejectV1::SourceTopologyUnavailable { route },
    }
}

fn classify_scan_with_init(facts: &LoopFacts) -> LoopPreflightRejectV1 {
    let Some(scan) = facts.scan_with_init() else {
        return LoopPreflightRejectV1::SourceTopologyUnavailable {
            route: LoopRouteId::ScanWithInit,
        };
    };
    let Some(topology) = scan.source_topology.as_ref() else {
        return LoopPreflightRejectV1::SourceTopologyUnavailable {
            route: LoopRouteId::ScanWithInit,
        };
    };
    if topology.has_scope_box_lineage() {
        return LoopPreflightRejectV1::ScopeBoxLineageNotBorrowable {
            route: LoopRouteId::ScanWithInit,
        };
    }
    LoopPreflightRejectV1::PolicyAndTerminalityUnavailable {
        route: LoopRouteId::ScanWithInit,
    }
}

fn classify_nested_loop_minimal(facts: &LoopFacts) -> LoopPreflightRejectV1 {
    let Some(nested) = facts.nested_loop_minimal() else {
        return LoopPreflightRejectV1::SourceTopologyUnavailable {
            route: LoopRouteId::NestedLoopMinimal,
        };
    };
    let Some(topology) = nested.source_topology.as_ref() else {
        return LoopPreflightRejectV1::SourceTopologyUnavailable {
            route: LoopRouteId::NestedLoopMinimal,
        };
    };
    if topology.has_scope_box_lineage() {
        return LoopPreflightRejectV1::ScopeBoxLineageNotBorrowable {
            route: LoopRouteId::NestedLoopMinimal,
        };
    }
    LoopPreflightRejectV1::PolicyAndTerminalityUnavailable {
        route: LoopRouteId::NestedLoopMinimal,
    }
}

fn classify_loop_array_join(facts: &LoopFacts) -> LoopPreflightRejectV1 {
    let Some(array_join) = facts.loop_array_join() else {
        return LoopPreflightRejectV1::SourceTopologyUnavailable {
            route: LoopRouteId::LoopArrayJoin,
        };
    };
    let Some(topology) = array_join.source_topology.as_ref() else {
        return LoopPreflightRejectV1::SourceTopologyUnavailable {
            route: LoopRouteId::LoopArrayJoin,
        };
    };
    if topology.has_scope_box_lineage() {
        return LoopPreflightRejectV1::ScopeBoxLineageNotBorrowable {
            route: LoopRouteId::LoopArrayJoin,
        };
    }
    LoopPreflightRejectV1::PolicyAndTerminalityUnavailable {
        route: LoopRouteId::LoopArrayJoin,
    }
}

fn classify_loop_char_map(facts: &LoopFacts) -> LoopPreflightRejectV1 {
    let Some(char_map) = facts.loop_char_map() else {
        return LoopPreflightRejectV1::SourceTopologyUnavailable {
            route: LoopRouteId::LoopCharMap,
        };
    };
    let Some(topology) = char_map.source_topology.as_ref() else {
        return LoopPreflightRejectV1::SourceTopologyUnavailable {
            route: LoopRouteId::LoopCharMap,
        };
    };
    if topology.has_scope_box_lineage() {
        return LoopPreflightRejectV1::ScopeBoxLineageNotBorrowable {
            route: LoopRouteId::LoopCharMap,
        };
    }
    LoopPreflightRejectV1::PolicyAndTerminalityUnavailable {
        route: LoopRouteId::LoopCharMap,
    }
}

fn classify_loop_true_early_exit(facts: &LoopFacts) -> LoopPreflightRejectV1 {
    let Some(early_exit) = facts.loop_true_early_exit() else {
        return LoopPreflightRejectV1::SourceTopologyUnavailable {
            route: LoopRouteId::LoopTrueEarlyExit,
        };
    };
    let Some(topology) = early_exit.source_topology.as_ref() else {
        return LoopPreflightRejectV1::SourceTopologyUnavailable {
            route: LoopRouteId::LoopTrueEarlyExit,
        };
    };
    if topology.has_scope_box_lineage() {
        return LoopPreflightRejectV1::ScopeBoxLineageNotBorrowable {
            route: LoopRouteId::LoopTrueEarlyExit,
        };
    }
    LoopPreflightRejectV1::PolicyAndTerminalityUnavailable {
        route: LoopRouteId::LoopTrueEarlyExit,
    }
}

fn classify_loop_continue_only(facts: &LoopFacts) -> LoopPreflightRejectV1 {
    let Some(continue_only) = facts.loop_continue_only() else {
        return LoopPreflightRejectV1::SourceTopologyUnavailable {
            route: LoopRouteId::LoopContinueOnly,
        };
    };
    let Some(topology) = continue_only.source_topology.as_ref() else {
        return LoopPreflightRejectV1::SourceTopologyUnavailable {
            route: LoopRouteId::LoopContinueOnly,
        };
    };
    if topology.has_scope_box_lineage() {
        return LoopPreflightRejectV1::ScopeBoxLineageNotBorrowable {
            route: LoopRouteId::LoopContinueOnly,
        };
    }
    LoopPreflightRejectV1::PolicyAndTerminalityUnavailable {
        route: LoopRouteId::LoopContinueOnly,
    }
}

fn classify_if_phi_join(facts: &LoopFacts) -> LoopPreflightRejectV1 {
    let Some(if_phi_join) = facts.if_phi_join() else {
        return LoopPreflightRejectV1::SourceTopologyUnavailable {
            route: LoopRouteId::IfPhiJoin,
        };
    };
    let Some(topology) = if_phi_join.source_topology.as_ref() else {
        return LoopPreflightRejectV1::SourceTopologyUnavailable {
            route: LoopRouteId::IfPhiJoin,
        };
    };
    if !topology.if_else().scope_box_children().is_empty()
        || !topology.step().scope_box_children().is_empty()
    {
        return LoopPreflightRejectV1::ScopeBoxLineageNotBorrowable {
            route: LoopRouteId::IfPhiJoin,
        };
    }
    LoopPreflightRejectV1::PolicyAndTerminalityUnavailable {
        route: LoopRouteId::IfPhiJoin,
    }
}

fn classify_loop_break(facts: &LoopFacts) -> LoopPreflightRejectV1 {
    let Some(loop_break) = facts.loop_break() else {
        return LoopPreflightRejectV1::SourceTopologyUnavailable {
            route: LoopRouteId::LoopBreakRecipe,
        };
    };
    let Some(topology) = loop_break.source_topology.as_ref() else {
        return LoopPreflightRejectV1::SourceTopologyUnavailable {
            route: LoopRouteId::LoopBreakRecipe,
        };
    };
    if !topology.break_if().scope_box_children().is_empty()
        || !topology.carrier_update().scope_box_children().is_empty()
        || !topology.step().scope_box_children().is_empty()
    {
        return LoopPreflightRejectV1::ScopeBoxLineageNotBorrowable {
            route: LoopRouteId::LoopBreakRecipe,
        };
    }
    LoopPreflightRejectV1::PolicyAndTerminalityUnavailable {
        route: LoopRouteId::LoopBreakRecipe,
    }
}

fn classify_simple_while(facts: &LoopFacts) -> LoopPreflightRejectV1 {
    let Some(simple) = facts.loop_simple_while() else {
        return LoopPreflightRejectV1::SourceTopologyUnavailable {
            route: LoopRouteId::LoopSimpleWhile,
        };
    };
    let Some(topology) = simple.source_topology.as_ref() else {
        return LoopPreflightRejectV1::SourceTopologyUnavailable {
            route: LoopRouteId::LoopSimpleWhile,
        };
    };
    if !topology.step().scope_box_children().is_empty() {
        return LoopPreflightRejectV1::ScopeBoxLineageNotBorrowable {
            route: LoopRouteId::LoopSimpleWhile,
        };
    }
    LoopPreflightRejectV1::PolicyAndTerminalityUnavailable {
        route: LoopRouteId::LoopSimpleWhile,
    }
}

fn classify_accum_const(facts: &LoopFacts) -> LoopPreflightRejectV1 {
    let Some(accum) = facts.accum_const_loop() else {
        return LoopPreflightRejectV1::SourceTopologyUnavailable {
            route: LoopRouteId::AccumConstLoop,
        };
    };
    let Some(topology) = accum.source_topology.as_ref() else {
        return LoopPreflightRejectV1::SourceTopologyUnavailable {
            route: LoopRouteId::AccumConstLoop,
        };
    };
    if !topology.acc_update().scope_box_children().is_empty()
        || !topology.step().scope_box_children().is_empty()
    {
        return LoopPreflightRejectV1::ScopeBoxLineageNotBorrowable {
            route: LoopRouteId::AccumConstLoop,
        };
    }
    LoopPreflightRejectV1::PolicyAndTerminalityUnavailable {
        route: LoopRouteId::AccumConstLoop,
    }
}

#[cfg(test)]
mod array_join_tests;
#[cfg(test)]
mod bool_predicate_scan_tests;
#[cfg(test)]
mod split_scan_tests;

#[cfg(test)]
mod char_map_tests;

#[cfg(test)]
mod nested_loop_tests;

#[cfg(test)]
mod scan_with_init_tests;

#[cfg(test)]
mod tests;
