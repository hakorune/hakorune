"""Model / hotcore / record-state report fields for coverage reports."""

from __future__ import annotations

from typing import Mapping, cast

from hako_mimalloc_algorithm_coverage_field_state import CoverageFieldState
from hako_mimalloc_algorithm_coverage_measurement_state import CoverageMeasurementState
from hako_mimalloc_algorithm_coverage_owner_state import CoverageOwnerState
from hako_mimalloc_algorithm_coverage_route_state import CoverageRouteState
from hako_mimalloc_algorithm_coverage_support import CoverageRow


def build_model_report_fields(
    *,
    context: Mapping[str, object],
) -> dict[str, object]:
    measurement_state = cast(CoverageMeasurementState, context["measurement_state"])
    route_state = cast(CoverageRouteState, context["route_state"])
    field_state = cast(CoverageFieldState, context["field_state"])
    owner_state = cast(CoverageOwnerState, context["owner_state"])
    rows = cast(list[CoverageRow], context["rows"])
    hot_core_source = cast(str, context["hot_core_source"])
    hot_array_fields = cast(list[str], context["hot_array_fields"])
    hot_array_ops = cast(Mapping[str, Mapping[str, int]], context["hot_array_ops"])
    hot_array_arraybox_fields = cast(
        list[str],
        context["hot_array_arraybox_fields"],
    )
    hot_array_direct_fields = cast(list[str], context["hot_array_direct_fields"])
    hot_array_get_count = cast(int, context["hot_array_get_count"])
    hot_array_set_count = cast(int, context["hot_array_set_count"])
    hot_array_push_count = cast(int, context["hot_array_push_count"])
    hot_field_top = cast(str, context["hot_field_top"])
    hot_field_top_bucket = cast(str, context["hot_field_top_bucket"])
    primitive_hot_state_field_count = cast(
        int, context["primitive_hot_state_field_count"]
    )
    public_or_proof_field_count = cast(int, context["public_or_proof_field_count"])
    observer_counter_field_count = cast(int, context["observer_counter_field_count"])
    hot_field_plan_ready = cast(int, context["hot_field_plan_ready"])
    hot_field_next_bridge = cast(str, context["hot_field_next_bridge"])
    record_state_field_access_plan_count = cast(
        int, context["record_state_field_access_plan_count"]
    )
    record_state_field_access_ready = cast(
        int, context["record_state_field_access_ready"]
    )
    record_state_lowering_owner_selected = cast(
        str, context["record_state_lowering_owner_selected"]
    )
    record_state_access_exact_slot_covered_count = cast(
        int, context["record_state_access_exact_slot_covered_count"]
    )
    record_state_access_exact_slot_missing_count = cast(
        int, context["record_state_access_exact_slot_missing_count"]
    )
    record_state_lowering_owner_next_bridge = cast(
        str, context["record_state_lowering_owner_next_bridge"]
    )
    hot_array_source_migration_selected = cast(
        int, context["page_model_hot_array_source_migration_selected"]
    )
    hot_array_source_type_ready = cast(
        int, context["page_model_hot_array_source_type_ready"]
    )
    hot_array_birth_contract_ready = cast(
        int, context["page_model_hot_array_birth_contract_ready"]
    )
    migration_blocker = cast(str, context["page_model_hot_array_source_migration_blocker"])
    page_model_hot_array_source_route_measured = cast(
        int, context["page_model_hot_array_source_route_measured"]
    )
    hot_array_route_measurement_blocker = cast(
        str, context["page_model_hot_array_source_route_measurement_blocker"]
    )
    hot_array_route_next_bridge = cast(
        str, context["page_model_hot_array_source_route_next_bridge"]
    )
    fastpath_direct_array_plan_count = cast(
        int, context["page_model_hot_array_fastpath_direct_array_plan_count"]
    )
    fastpath_route_decision_count = cast(
        int, context["page_model_hot_array_fastpath_route_decision_count"]
    )
    fastpath_fast_selected_count = cast(
        int, context["page_model_hot_array_fastpath_fast_selected_count"]
    )
    fastpath_slow_selected_count = cast(
        int, context["page_model_hot_array_fastpath_slow_selected_count"]
    )
    perf_delta_plan = cast(int, context["page_model_hot_array_perf_delta_measurement_plan_v0"])
    perf_delta_ready = cast(int, context["page_model_hot_array_perf_delta_ready"])
    perf_delta_blocker = cast(str, context["page_model_hot_array_perf_delta_blocker"])
    perf_delta_next_bridge = cast(str, context["page_model_hot_array_perf_delta_next_bridge"])
    hotcore_consumer_enabled = measurement_state.hotcore_consumer_enabled
    hotcore_replacement_shape_ready = route_state.hotcore_replacement_shape_ready
    hotcore_bridge_blocker = route_state.hotcore_bridge_blocker
    hotcore_next_bridge = route_state.hotcore_next_bridge
    hotcore_measurement_reported = measurement_state.hotcore_measurement_reported
    hotcore_median_ops_per_sec = measurement_state.hotcore_median_ops_per_sec
    hotcore_page_model_source_ready = route_state.hotcore_page_model_source_ready
    hotcore_small_alloc_calls_acquire_fresh_small = (
        measurement_state.hotcore_small_alloc_calls_acquire_fresh_small
    )
    hotcore_release_calls_release_local_known_live = (
        measurement_state.hotcore_release_calls_release_local_known_live
    )
    page_model_hot_methods_ready = measurement_state.page_model_hot_methods_ready
    hotcore_methods = [
        method
        for method in ("objectLifecycleSmallAlloc", "objectLifecycleReleaseBlock")
        if method in hot_core_source
    ]
    hotcore_route = measurement_state.hotcore_route
    page_model_hot_array_measurement_ready = route_state.page_model_hot_array_measurement_ready
    structural_owner_refresh_required = route_state.structural_owner_refresh_required
    product_pages_non_linear_owner_candidate_ready = (
        owner_state.product_pages_non_linear_owner_candidate_ready
    )
    record_state_representation_delta_ready = (
        owner_state.record_state_representation_delta_ready
    )
    record_state_representation_delta_positive_candidate = (
        owner_state.record_state_representation_delta_positive_candidate
    )
    record_state_representation_delta_blocker = (
        owner_state.record_state_representation_delta_blocker
    )
    record_state_representation_delta_next_bridge = (
        owner_state.record_state_representation_delta_next_bridge
    )
    record_state_next_bridge = owner_state.record_state_next_bridge
    next_perf_owner_selection_plan = owner_state.next_perf_owner_selection_plan
    next_perf_owner_selected = owner_state.next_perf_owner_selected
    next_perf_owner_reason = owner_state.next_perf_owner_reason
    next_perf_owner_next_bridge = owner_state.next_perf_owner_next_bridge
    structural_owner_selected = owner_state.structural_owner_selected
    structural_owner_reason = owner_state.structural_owner_reason
    structural_owner_next_action = owner_state.structural_owner_next_action

    return {
        "page_model_hot_array_seed_push_blocker": int(hot_array_push_count > 0),
        "page_model_hot_array_field_count": len(hot_array_fields),
        "page_model_hot_array_arraybox_field_count": len(hot_array_arraybox_fields),
        "page_model_hot_array_directarray_field_count": len(hot_array_direct_fields),
        "page_model_hot_array_arraybox_fields": ",".join(hot_array_arraybox_fields) or "none",
        "page_model_hot_array_directarray_fields": ",".join(hot_array_direct_fields) or "none",
        "page_model_hot_array_get_count": hot_array_get_count,
        "page_model_hot_array_set_count": hot_array_set_count,
        "page_model_hot_array_push_count": hot_array_push_count,
        "page_model_hot_array_op_summary": ",".join(
            f"{name}:get={ops['get']}:set={ops['set']}:push={ops['push']}"
            for name, ops in hot_array_ops.items()
        ),
        "hotcore_replacement_bridge_plan_v0": 1,
        "hotcore_replacement_bridge_report_only": 1,
        "hotcore_replacement_consumer_enabled": hotcore_consumer_enabled,
        "hotcore_replacement_shape_ready": hotcore_replacement_shape_ready,
        "hotcore_replacement_bridge_blocker": hotcore_bridge_blocker,
        "hotcore_replacement_next_bridge": hotcore_next_bridge,
        "hotcore_replacement_measurement_reported": hotcore_measurement_reported,
        "hotcore_replacement_median_ops_per_sec": hotcore_median_ops_per_sec,
        "hotcore_page_model_source_ready": hotcore_page_model_source_ready,
        "hotcore_small_alloc_calls_acquire_fresh_small": hotcore_small_alloc_calls_acquire_fresh_small,
        "hotcore_release_calls_release_local_known_live": hotcore_release_calls_release_local_known_live,
        "page_model_hot_methods_ready": page_model_hot_methods_ready,
        "hotcore_source_method_count": len(hotcore_methods),
        "hotcore_source_methods": ",".join(hotcore_methods) or "none",
        "hotcore_replacement_route": hotcore_route,
        "page_model_hot_field_traffic_plan_v0": 1,
        "page_model_hot_field_traffic_ready": hot_field_plan_ready,
        "page_model_hot_field_top": hot_field_top,
        "page_model_hot_field_top_bucket": hot_field_top_bucket,
        "page_model_hot_field_buckets": field_state.hot_field_buckets,
        "page_model_hot_field_primitive_hot_state_count": primitive_hot_state_field_count,
        "page_model_hot_field_public_or_proof_count": public_or_proof_field_count,
        "page_model_hot_field_observer_counter_count": observer_counter_field_count,
        "page_model_hot_field_counter_deletion_allowed": 0,
        "page_model_hot_field_next_bridge": hot_field_next_bridge,
        "record_state_residence_plan_v0": 1,
        "record_state_residence_report_only": 1,
        "record_state_residence_ready": field_state.record_state_report_ready,
        "record_state_field_access_plan_count": record_state_field_access_plan_count,
        "record_state_field_access_ready": record_state_field_access_ready,
        "record_state_field_access_lowering_enabled": 0,
        "record_state_route_decision_enabled": 0,
        "record_state_lowering_owner_selected": record_state_lowering_owner_selected,
        "record_state_access_exact_slot_covered_count": record_state_access_exact_slot_covered_count,
        "record_state_access_exact_slot_missing_count": record_state_access_exact_slot_missing_count,
        "record_state_lowering_owner_next_bridge": record_state_lowering_owner_next_bridge,
        "record_state_representation_delta_plan_v0": 1,
        "record_state_representation_delta_ready": record_state_representation_delta_ready,
        "record_state_representation_delta_positive_candidate": record_state_representation_delta_positive_candidate,
        "record_state_representation_delta_blocker": record_state_representation_delta_blocker,
        "record_state_representation_delta_next_bridge": record_state_representation_delta_next_bridge,
        "record_state_residence_owner_box": "HakoAllocPageModel",
        "record_state_residence_candidate_record": "PageState",
        "record_state_residence_static_candidate_fields": field_state.record_state_static_candidate_fields,
        "record_state_residence_observed_candidate_fields": field_state.record_state_observed_candidate_fields,
        "record_state_residence_rejected_observed_fields": field_state.record_state_rejected_observed_fields,
        "record_state_residence_source_migration_allowed": 0,
        "record_state_residence_next_bridge": record_state_next_bridge,
        "next_perf_owner_selection_plan_v0": next_perf_owner_selection_plan,
        "next_perf_owner_selected": next_perf_owner_selected,
        "next_perf_owner_selected_reason": next_perf_owner_reason,
        "next_perf_owner_next_bridge": next_perf_owner_next_bridge,
        "structural_owner_selection_plan_v0": 1,
        "structural_owner_refresh_required": structural_owner_refresh_required,
        "structural_owner_selected": structural_owner_selected,
        "structural_owner_selected_reason": structural_owner_reason,
        "structural_owner_next_action": structural_owner_next_action,
        "structural_owner_candidate_0": "page_model_hot_array_source_route_measurement",
        "structural_owner_candidate_0_ready": page_model_hot_array_measurement_ready,
        "structural_owner_candidate_1": "product_pages_bridge_non_linear_owner_lookup",
        "structural_owner_candidate_1_ready": product_pages_non_linear_owner_candidate_ready,
        "page_model_hot_array_bridge_plan_v0": 1,
        "page_model_hot_array_access_plan_v0": 1,
        "page_model_hot_array_access_static_scan": 1,
        "page_model_hot_array_source_migration_selected": hot_array_source_migration_selected,
        "page_model_hot_array_source_type_ready": hot_array_source_type_ready,
        "page_model_hot_array_birth_contract_ready": hot_array_birth_contract_ready,
        "page_model_hot_array_source_migration_blocker": migration_blocker,
        "page_model_hot_array_next_bridge": "directarray_i64_field_type_and_birth_fixture"
        if migration_blocker != "none"
        else "source_migration_measurement",
        "page_model_hot_array_candidate_type": "DirectArrayI64",
        "page_model_hot_array_directarray_supported_ops": "get,set",
        "page_model_hot_array_directarray_missing_ops": "push_or_birth_with_initialized_len"
        if hot_array_push_count
        else "none",
        "page_model_hot_array_source_route_measurement_plan_v0": 1,
        "page_model_hot_array_source_route_measured": page_model_hot_array_source_route_measured,
        "page_model_hot_array_source_route_measurement_blocker": hot_array_route_measurement_blocker,
        "page_model_hot_array_source_route_next_bridge": hot_array_route_next_bridge,
        "page_model_hot_array_fastpath_direct_array_plan_count": fastpath_direct_array_plan_count,
        "page_model_hot_array_fastpath_route_decision_count": fastpath_route_decision_count,
        "page_model_hot_array_fastpath_fast_selected_count": fastpath_fast_selected_count,
        "page_model_hot_array_fastpath_slow_selected_count": fastpath_slow_selected_count,
        "page_model_hot_array_perf_delta_measurement_plan_v0": perf_delta_plan,
        "page_model_hot_array_perf_delta_ready": perf_delta_ready,
        "page_model_hot_array_perf_delta_blocker": perf_delta_blocker,
        "page_model_hot_array_perf_delta_next_bridge": perf_delta_next_bridge,
        "rows": [row.__dict__ for row in rows],
    }
