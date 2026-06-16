"""Static FastPath route priority vocabulary.

This module is intentionally small and data-only. It does not inspect MIR and
does not select routes; consumers use it to explain priority consistently.
"""

from __future__ import annotations

from dataclasses import dataclass


@dataclass(frozen=True)
class RoutePriority:
    family: str
    priority: int
    route_owner: str
    route_name: str
    condition: str
    selected_before: str


ROUTE_PRIORITIES: tuple[RoutePriority, ...] = (
    RoutePriority(
        family="exact_seed",
        priority=10,
        route_owner="function_level_exact_seed",
        route_name="exact_seed_backend_route",
        condition="metadata.exact_seed_backend_route is explicitly selected",
        selected_before="local_fastpath_fact,generic_metadata_consumer,runtime_helper_fallback",
    ),
    RoutePriority(
        family="local_fastpath_fact",
        priority=20,
        route_owner="LocalFastPathFact",
        route_name="local_fastpath_fact",
        condition="positive backend-consumable LocalFastPathFact is selected",
        selected_before="generic_metadata_consumer,runtime_helper_fallback",
    ),
    RoutePriority(
        family="string_dead_text_region",
        priority=30,
        route_owner="generic_metadata_consumer",
        route_name="StringDeadTextRegionPlan",
        condition="string_dead_text_region_plans candidate is selected by route owner",
        selected_before="runtime_helper_fallback",
    ),
    RoutePriority(
        family="runtime_helper_fallback",
        priority=90,
        route_owner="runtime_helper_fallback",
        route_name="runtime_helper_fallback",
        condition="no selected exact/local/generic fast-path route",
        selected_before="none",
    ),
)


def priority_for_family(family: str) -> RoutePriority | None:
    for row in ROUTE_PRIORITIES:
        if row.family == family:
            return row
    return None


def priority_value_for_family(family: str, default: int = 1000) -> int:
    row = priority_for_family(family)
    return row.priority if row is not None else default
