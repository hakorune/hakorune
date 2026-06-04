"""Route-state readiness helpers for mimalloc algorithm coverage reports."""

from __future__ import annotations

from dataclasses import dataclass


@dataclass(frozen=True)
class CoverageRouteStateInputs:
    hotcore_consumer_enabled: int
    hotcore_measurement_reported: int
    hot_array_source_migration_selected: int
    page_model_hot_methods_ready: int
    fastpath_report_consumed: int
    fastpath_direct_array_plan_count: int
    fastpath_route_decision_count: int
    fastpath_fast_selected_count: int
    fastpath_slow_selected_count: int
    fastpath_generic_dispatch_count: int
    fastpath_dynamic_route_count: int
    fastpath_boxed_fallback_count: int
    fastpath_clean: int
    page_map_source_ready: int
    page_map_release_source_ready: int
    realloc_same_class_source_ready: int
    realloc_grow_copy_release_source_ready: int
    huge_page_source_ready: int
    osvm_page_source_pilot_ready: int


@dataclass(frozen=True)
class CoverageRouteState:
    page_model_hot_array_source_route_measured: int
    hot_array_route_measurement_blocker: str
    hot_array_route_next_bridge: str
    hotcore_page_model_source_ready: int
    hotcore_replacement_shape_ready: int
    hotcore_bridge_blocker: str
    hotcore_next_bridge: str
    product_pages_source_ready: int
    product_pages_full_source_ready: int
    product_pages_bridge_blocker: str
    product_pages_next_bridge: str
    product_pages_non_linear_lookup_probe_closed: int
    product_pages_non_linear_lookup_decision: str
    product_pages_non_linear_lookup_plan: int
    product_pages_linear_probe_closed: int
    product_pages_non_linear_lookup_strategy: str
    product_pages_non_linear_next_bridge: str
    structural_owner_refresh_required: int
    page_model_hot_array_measurement_ready: int


def derive_route_state(inputs: CoverageRouteStateInputs) -> CoverageRouteState:
    page_model_hot_array_source_route_measured = int(
        inputs.fastpath_report_consumed
        and inputs.hot_array_source_migration_selected > 0
        and inputs.page_model_hot_methods_ready > 0
        and inputs.fastpath_direct_array_plan_count > 0
        and inputs.fastpath_route_decision_count > 0
        and inputs.fastpath_fast_selected_count == inputs.fastpath_route_decision_count
        and inputs.fastpath_slow_selected_count == 0
        and inputs.fastpath_generic_dispatch_count == 0
        and inputs.fastpath_dynamic_route_count == 0
        and inputs.fastpath_boxed_fallback_count == 0
        and inputs.fastpath_clean == 1
    )
    if page_model_hot_array_source_route_measured:
        hot_array_route_measurement_blocker = "none"
        hot_array_route_next_bridge = "perf_delta_measurement"
    elif inputs.fastpath_report_consumed:
        hot_array_route_measurement_blocker = "directarray_route_not_clean"
        hot_array_route_next_bridge = "fix_or_explain_directarray_route_miss"
    else:
        hot_array_route_measurement_blocker = "fastpath_report_not_consumed"
        hot_array_route_next_bridge = "run_hako_check_fastpath_explain"
    hotcore_page_model_source_ready = int(
        inputs.hotcore_consumer_enabled
        and inputs.hotcore_measurement_reported
        and inputs.page_model_hot_methods_ready
        and inputs.hot_array_source_migration_selected
    )
    hotcore_replacement_shape_ready = int(hotcore_page_model_source_ready)
    if inputs.hotcore_consumer_enabled:
        hotcore_bridge_blocker = "none"
        hotcore_next_bridge = (
            "select_next_structural_owner"
            if inputs.hotcore_measurement_reported
            else "measure_hotcore_replacement_consumer"
        )
    elif hotcore_replacement_shape_ready:
        hotcore_bridge_blocker = "consumer_not_enabled"
        hotcore_next_bridge = "replacement_front_consume_hotcore_page_model"
    else:
        hotcore_bridge_blocker = "source_shape_not_ready"
        hotcore_next_bridge = "fix_hotcore_page_model_source_shape"
    product_pages_source_ready = int(
        inputs.page_map_source_ready
        and inputs.page_map_release_source_ready
        and inputs.realloc_same_class_source_ready
        and inputs.page_model_hot_methods_ready
    )
    product_pages_full_source_ready = int(
        product_pages_source_ready
        and inputs.realloc_grow_copy_release_source_ready
        and inputs.huge_page_source_ready
        and inputs.osvm_page_source_pilot_ready
    )
    product_pages_non_linear_lookup_probe_closed = 1
    product_pages_non_linear_lookup_decision = "nonkeeper"
    if inputs.hotcore_consumer_enabled:
        product_pages_bridge_blocker = "non_linear_probe_measured_nonkeeper"
        product_pages_next_bridge = "select_next_perf_owner"
    elif product_pages_source_ready:
        product_pages_bridge_blocker = "non_linear_probe_closed_nonkeeper"
        product_pages_next_bridge = "select_next_perf_owner"
    else:
        product_pages_bridge_blocker = "source_shape_not_ready"
        product_pages_next_bridge = "fix_product_pages_source_shape"
    product_pages_non_linear_lookup_plan = int(
        product_pages_source_ready and not inputs.hotcore_consumer_enabled
    )
    product_pages_linear_probe_closed = int(product_pages_non_linear_lookup_plan)
    product_pages_non_linear_lookup_strategy = (
        "range_decision_tree_or_indexed_page_table"
        if product_pages_non_linear_lookup_plan
        else "none"
    )
    product_pages_non_linear_next_bridge = (
        "replacement_front_product_pages_non_linear_plan"
        if product_pages_non_linear_lookup_plan
        else product_pages_next_bridge
    )
    structural_owner_refresh_required = int(
        inputs.hotcore_measurement_reported
        and hotcore_next_bridge == "select_next_structural_owner"
    )
    page_model_hot_array_measurement_ready = int(
        structural_owner_refresh_required and inputs.hot_array_source_migration_selected
    )
    return CoverageRouteState(
        page_model_hot_array_source_route_measured=page_model_hot_array_source_route_measured,
        hot_array_route_measurement_blocker=hot_array_route_measurement_blocker,
        hot_array_route_next_bridge=hot_array_route_next_bridge,
        hotcore_page_model_source_ready=hotcore_page_model_source_ready,
        hotcore_replacement_shape_ready=hotcore_replacement_shape_ready,
        hotcore_bridge_blocker=hotcore_bridge_blocker,
        hotcore_next_bridge=hotcore_next_bridge,
        product_pages_source_ready=product_pages_source_ready,
        product_pages_full_source_ready=product_pages_full_source_ready,
        product_pages_bridge_blocker=product_pages_bridge_blocker,
        product_pages_next_bridge=product_pages_next_bridge,
        product_pages_non_linear_lookup_probe_closed=product_pages_non_linear_lookup_probe_closed,
        product_pages_non_linear_lookup_decision=product_pages_non_linear_lookup_decision,
        product_pages_non_linear_lookup_plan=product_pages_non_linear_lookup_plan,
        product_pages_linear_probe_closed=product_pages_linear_probe_closed,
        product_pages_non_linear_lookup_strategy=product_pages_non_linear_lookup_strategy,
        product_pages_non_linear_next_bridge=product_pages_non_linear_next_bridge,
        structural_owner_refresh_required=structural_owner_refresh_required,
        page_model_hot_array_measurement_ready=page_model_hot_array_measurement_ready,
    )
