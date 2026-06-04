"""Coverage report context derivation for mimalloc algorithm coverage."""

from __future__ import annotations

from pathlib import Path

from hako_mimalloc_algorithm_coverage_field_state import (
    CoverageFieldStateInputs,
    derive_field_state,
)
from hako_mimalloc_algorithm_coverage_measurement_state import (
    CoverageMeasurementStateInputs,
    derive_measurement_state,
)
from hako_mimalloc_algorithm_coverage_owner_state import (
    CoverageOwnerStateInputs,
    derive_owner_state,
)
from hako_mimalloc_algorithm_coverage_route_state import (
    CoverageRouteStateInputs,
    derive_route_state,
)
from hako_mimalloc_algorithm_coverage_source_state import derive_source_state
from hako_mimalloc_algorithm_coverage_support import (
    CoverageRow,
    int_field,
    read_fastpath_counts,
    read_kv_report,
    str_field,
)
from hako_mimalloc_algorithm_coverage_rows import refine_rows


def build_coverage_report_context(
    rows: list[CoverageRow],
    *,
    benchmark_report: Path | None = None,
    fastpath_report: Path | None = None,
    state_report: Path | None = None,
    perf_attribution_report: Path | None = None,
    accumulator_report: Path | None = None,
) -> dict[str, object]:
    source_state = derive_source_state(rows)
    page_box = source_state["page_box"]
    hot_core = source_state["hot_core"]
    page_map = source_state["page_map"]
    page_map_release = source_state["page_map_release"]
    realloc_same = source_state["realloc_same"]
    realloc_grow = source_state["realloc_grow"]
    huge_model = source_state["huge_model"]
    osvm_source = source_state["osvm_source"]
    replacement = source_state["replacement"]
    hot_array_fields = source_state["hot_array_fields"]
    hot_array_ops = source_state["hot_array_ops"]
    hot_array_get_count = source_state["hot_array_get_count"]
    hot_array_set_count = source_state["hot_array_set_count"]
    hot_array_push_count = source_state["hot_array_push_count"]
    hot_array_arraybox_fields = source_state["hot_array_arraybox_fields"]
    hot_array_direct_fields = source_state["hot_array_direct_fields"]
    hot_array_source_type_ready = source_state["hot_array_source_type_ready"]
    hot_array_birth_contract_ready = source_state["hot_array_birth_contract_ready"]
    hot_array_source_migration_selected = source_state["hot_array_source_migration_selected"]
    direct_array_owner_field_count = source_state["direct_array_owner_field_count"]
    migration_blocker = source_state["migration_blocker"]
    hotcore_methods = source_state["hotcore_methods"]
    hotcore_small_alloc_calls_acquire_fresh_small = source_state[
        "hotcore_small_alloc_calls_acquire_fresh_small"
    ]
    hotcore_release_calls_release_local_known_live = source_state[
        "hotcore_release_calls_release_local_known_live"
    ]
    page_model_hot_methods_ready = source_state["page_model_hot_methods_ready"]
    page_map_source_ready = source_state["page_map_source_ready"]
    page_map_release_source_ready = source_state["page_map_release_source_ready"]
    realloc_same_class_source_ready = source_state["realloc_same_class_source_ready"]
    realloc_grow_copy_release_source_ready = source_state[
        "realloc_grow_copy_release_source_ready"
    ]
    huge_page_source_ready = source_state["huge_page_source_ready"]
    osvm_page_source_pilot_ready = source_state["osvm_page_source_pilot_ready"]
    size_class_single_bridge_supported = source_state["size_class_single_bridge_supported"]
    page_bins_bridge_supported = source_state["page_bins_bridge_supported"]
    locked_front = source_state["locked_front"]
    tls_front = source_state["tls_front"]
    replacement_full_hako = source_state["replacement_full_hako"]
    benchmark = read_kv_report(benchmark_report)
    fastpath = read_fastpath_counts(fastpath_report)
    state = read_kv_report(state_report)
    perf_attribution = read_kv_report(perf_attribution_report)
    accumulator = read_kv_report(accumulator_report)
    benchmark_report_consumed = int(bool(benchmark))
    fastpath_report_consumed = int(bool(fastpath))
    state_report_consumed = int(bool(state))
    perf_attribution_report_consumed = int(bool(perf_attribution))
    accumulator_report_consumed = int(bool(accumulator))
    accumulator_contract_ready = int(
        str_field(accumulator, "output_contract", "")
        == "hako-mimalloc-requested-bytes-accumulator-contract-v0"
    )
    measurement_state = derive_measurement_state(
        CoverageMeasurementStateInputs(
            benchmark=benchmark,
            fastpath=fastpath,
            perf_attribution=perf_attribution,
            accumulator=accumulator,
            benchmark_report_consumed=benchmark_report_consumed,
            fastpath_report_consumed=fastpath_report_consumed,
            perf_attribution_report_consumed=perf_attribution_report_consumed,
            accumulator_report_consumed=accumulator_report_consumed,
        )
    )
    benchmark_subject = measurement_state.benchmark_replacement_subject
    page_bins_consumer_enabled = measurement_state.page_bins_consumer_enabled
    page_bins_route = measurement_state.page_bins_route
    page_bins_lookup_route = measurement_state.page_bins_lookup_route
    product_bins_consumer_enabled = measurement_state.product_bins_consumer_enabled
    product_bins_route = measurement_state.product_bins_route
    product_pages_consumer_enabled = measurement_state.product_pages_consumer_enabled
    product_pages_route = measurement_state.product_pages_route
    algorithm_shape = measurement_state.algorithm_shape
    hotcore_consumer_enabled = measurement_state.hotcore_consumer_enabled
    hotcore_route = measurement_state.hotcore_route
    hotcore_median_ops_per_sec = measurement_state.hotcore_median_ops_per_sec
    hotcore_measurement_reported = measurement_state.hotcore_measurement_reported
    fastpath_direct_array_plan_count = measurement_state.fastpath_direct_array_plan_count
    fastpath_route_decision_count = measurement_state.fastpath_route_decision_count
    fastpath_fast_selected_count = measurement_state.fastpath_fast_selected_count
    fastpath_slow_selected_count = measurement_state.fastpath_slow_selected_count
    fastpath_generic_dispatch_count = measurement_state.fastpath_generic_dispatch_count
    fastpath_dynamic_route_count = measurement_state.fastpath_dynamic_route_count
    fastpath_boxed_fallback_count = measurement_state.fastpath_boxed_fallback_count
    fastpath_clean = measurement_state.fastpath_clean
    route_state = derive_route_state(
        CoverageRouteStateInputs(
            hotcore_consumer_enabled=hotcore_consumer_enabled,
            hotcore_measurement_reported=hotcore_measurement_reported,
            hot_array_source_migration_selected=hot_array_source_migration_selected,
            page_model_hot_methods_ready=page_model_hot_methods_ready,
            fastpath_report_consumed=fastpath_report_consumed,
            fastpath_direct_array_plan_count=fastpath_direct_array_plan_count,
            fastpath_route_decision_count=fastpath_route_decision_count,
            fastpath_fast_selected_count=fastpath_fast_selected_count,
            fastpath_slow_selected_count=fastpath_slow_selected_count,
            fastpath_generic_dispatch_count=fastpath_generic_dispatch_count,
            fastpath_dynamic_route_count=fastpath_dynamic_route_count,
            fastpath_boxed_fallback_count=fastpath_boxed_fallback_count,
            fastpath_clean=fastpath_clean,
            page_map_source_ready=page_map_source_ready,
            page_map_release_source_ready=page_map_release_source_ready,
            realloc_same_class_source_ready=realloc_same_class_source_ready,
            realloc_grow_copy_release_source_ready=realloc_grow_copy_release_source_ready,
            huge_page_source_ready=huge_page_source_ready,
            osvm_page_source_pilot_ready=osvm_page_source_pilot_ready,
        )
    )
    page_model_hot_array_source_route_measured = (
        route_state.page_model_hot_array_source_route_measured
    )
    hot_array_route_measurement_blocker = route_state.hot_array_route_measurement_blocker
    hot_array_route_next_bridge = route_state.hot_array_route_next_bridge
    hotcore_page_model_source_ready = route_state.hotcore_page_model_source_ready
    hotcore_replacement_shape_ready = route_state.hotcore_replacement_shape_ready
    hotcore_bridge_blocker = route_state.hotcore_bridge_blocker
    hotcore_next_bridge = route_state.hotcore_next_bridge
    product_pages_source_ready = route_state.product_pages_source_ready
    product_pages_full_source_ready = route_state.product_pages_full_source_ready
    product_pages_bridge_blocker = route_state.product_pages_bridge_blocker
    product_pages_next_bridge = route_state.product_pages_next_bridge
    product_pages_non_linear_lookup_probe_closed = (
        route_state.product_pages_non_linear_lookup_probe_closed
    )
    product_pages_non_linear_lookup_decision = (
        route_state.product_pages_non_linear_lookup_decision
    )
    product_pages_non_linear_lookup_plan = route_state.product_pages_non_linear_lookup_plan
    product_pages_linear_probe_closed = route_state.product_pages_linear_probe_closed
    product_pages_non_linear_lookup_strategy = (
        route_state.product_pages_non_linear_lookup_strategy
    )
    product_pages_non_linear_next_bridge = route_state.product_pages_non_linear_next_bridge
    structural_owner_refresh_required = route_state.structural_owner_refresh_required
    page_model_hot_array_measurement_ready = (
        route_state.page_model_hot_array_measurement_ready
    )
    perf_delta_plan = measurement_state.perf_delta_plan
    perf_delta_ready = measurement_state.perf_delta_ready
    perf_delta_blocker = measurement_state.perf_delta_blocker
    perf_delta_next_bridge = measurement_state.perf_delta_next_bridge
    instruction_attribution_available = measurement_state.instruction_attribution_available
    backend_store_shape_ready = measurement_state.backend_store_shape_ready
    backend_store_shape_selected = measurement_state.backend_store_shape_selected
    backend_store_shape_next_bridge = measurement_state.backend_store_shape_next_bridge
    directarray_owner_instruction_shape_selected = (
        measurement_state.directarray_owner_instruction_shape_selected
    )
    directarray_owner_instruction_shape_next_bridge = (
        measurement_state.directarray_owner_instruction_shape_next_bridge
    )
    inlined_hot_body_selected = measurement_state.inlined_hot_body_selected
    inlined_hot_body_next_bridge = measurement_state.inlined_hot_body_next_bridge
    inlined_hot_body_split_next_bridge = (
        measurement_state.inlined_hot_body_split_next_bridge
    )
    field_state = derive_field_state(
        CoverageFieldStateInputs(
            page_box=page_box,
            perf_attribution=perf_attribution,
            perf_attribution_report_consumed=perf_attribution_report_consumed,
            state=state,
            state_report_consumed=state_report_consumed,
        )
    )
    hot_field_top = field_state.hot_field_top
    hot_field_top_bucket = field_state.hot_field_top_bucket
    primitive_hot_state_field_count = field_state.hot_field_primitive_hot_state_count
    public_or_proof_field_count = field_state.hot_field_public_or_proof_count
    observer_counter_field_count = field_state.hot_field_observer_counter_count
    hot_field_plan_ready = field_state.hot_field_plan_ready
    hot_field_next_bridge = field_state.hot_field_next_bridge
    record_state_static_candidates = field_state.record_state_static_candidate_fields
    record_state_observed_candidates = field_state.record_state_observed_candidate_fields
    record_state_observed_rejections = field_state.record_state_rejected_observed_fields
    record_state_field_access_plan_count = int_field(
        state, "record_state_field_access_plan_count", 0
    )
    record_state_field_access_ready = field_state.record_state_field_access_ready
    record_state_lowering_owner_selected = str_field(
        state, "record_state_lowering_owner_selected", "none"
    )
    record_state_access_exact_slot_covered_count = int_field(
        state, "record_state_access_exact_slot_covered_count", 0
    )
    record_state_access_exact_slot_missing_count = int_field(
        state, "record_state_access_exact_slot_missing_count", 0
    )
    record_state_lowering_owner_next_bridge = str_field(
        state,
        "record_state_lowering_owner_next_bridge",
        "select_record_state_lowering_owner",
    )
    record_state_report_ready = field_state.record_state_report_ready
    owner_state = derive_owner_state(
        CoverageOwnerStateInputs(
            page_model_hot_array_measurement_ready=page_model_hot_array_measurement_ready,
            page_model_hot_array_source_route_measured=page_model_hot_array_source_route_measured,
            perf_attribution_report_consumed=perf_attribution_report_consumed,
            perf_delta_ready=perf_delta_ready,
            perf_delta_next_bridge=perf_delta_next_bridge,
            structural_owner_refresh_required=structural_owner_refresh_required,
            product_pages_source_ready=product_pages_source_ready,
            product_pages_consumer_enabled=product_pages_consumer_enabled,
            product_pages_non_linear_lookup_probe_closed=product_pages_non_linear_lookup_probe_closed,
            record_state_report_ready=record_state_report_ready,
            record_state_field_access_ready=record_state_field_access_ready,
            record_state_lowering_owner_selected=record_state_lowering_owner_selected,
            record_state_access_exact_slot_missing_count=record_state_access_exact_slot_missing_count,
            record_state_lowering_owner_next_bridge=record_state_lowering_owner_next_bridge,
            hot_field_plan_ready=hot_field_plan_ready,
            hot_field_next_bridge=hot_field_next_bridge,
            backend_store_shape_ready=backend_store_shape_ready,
            backend_store_shape_selected=backend_store_shape_selected,
            backend_store_shape_next_bridge=backend_store_shape_next_bridge,
            inlined_hot_body_selected=inlined_hot_body_selected,
            inlined_hot_body_split_next_bridge=inlined_hot_body_split_next_bridge,
            directarray_owner_instruction_shape_selected=directarray_owner_instruction_shape_selected,
            directarray_owner_instruction_shape_next_bridge=directarray_owner_instruction_shape_next_bridge,
            instruction_attribution_available=instruction_attribution_available,
            primitive_hot_state_field_count=primitive_hot_state_field_count,
            direct_array_owner_field_count=direct_array_owner_field_count,
        )
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
    product_pages_non_linear_owner_candidate_ready = (
        owner_state.product_pages_non_linear_owner_candidate_ready
    )
    structural_owner_selected = owner_state.structural_owner_selected
    structural_owner_reason = owner_state.structural_owner_reason
    structural_owner_next_action = owner_state.structural_owner_next_action
    rows = refine_rows(
        rows,
        product_bins_consumer_enabled=product_bins_consumer_enabled,
        hotcore_consumer_enabled=hotcore_consumer_enabled,
        hotcore_next_bridge=hotcore_next_bridge,
    )
    return locals().copy()
