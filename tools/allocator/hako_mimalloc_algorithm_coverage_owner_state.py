"""Owner selection policy for mimalloc algorithm coverage reports."""

from __future__ import annotations

from dataclasses import dataclass


@dataclass(frozen=True)
class CoverageOwnerStateInputs:
    page_model_hot_array_measurement_ready: int
    page_model_hot_array_source_route_measured: int
    perf_attribution_report_consumed: int
    perf_delta_ready: int
    perf_delta_next_bridge: str
    structural_owner_refresh_required: int
    product_pages_source_ready: int
    product_pages_consumer_enabled: int
    product_pages_non_linear_lookup_probe_closed: int
    record_state_report_ready: int
    record_state_field_access_ready: int
    record_state_lowering_owner_selected: str
    record_state_access_exact_slot_missing_count: int
    record_state_lowering_owner_next_bridge: str
    hot_field_plan_ready: int
    hot_field_next_bridge: str
    backend_store_shape_ready: int
    backend_store_shape_selected: str
    backend_store_shape_next_bridge: str
    inlined_hot_body_selected: str
    inlined_hot_body_split_next_bridge: str
    directarray_owner_instruction_shape_selected: str
    directarray_owner_instruction_shape_next_bridge: str
    instruction_attribution_available: int
    primitive_hot_state_field_count: int
    direct_array_owner_field_count: int


@dataclass(frozen=True)
class CoverageOwnerState:
    record_state_representation_delta_ready: int
    record_state_representation_delta_positive_candidate: int
    record_state_representation_delta_blocker: str
    record_state_representation_delta_next_bridge: str
    record_state_next_bridge: str
    next_perf_owner_selection_plan: int
    next_perf_owner_selected: str
    next_perf_owner_reason: str
    next_perf_owner_next_bridge: str
    product_pages_non_linear_owner_candidate_ready: int
    structural_owner_selected: str
    structural_owner_reason: str
    structural_owner_next_action: str


def derive_owner_state(inputs: CoverageOwnerStateInputs) -> CoverageOwnerState:
    record_state_representation_delta_ready = int(
        inputs.record_state_field_access_ready
        and inputs.record_state_lowering_owner_selected == "typed_object_exact_slot_existing"
        and inputs.record_state_access_exact_slot_missing_count == 0
    )
    record_state_representation_delta_positive_candidate = 0
    record_state_representation_delta_blocker = (
        "typed_object_exact_slot_already_covers_record_state_access"
        if record_state_representation_delta_ready
        else "record_state_lowering_owner_not_selected"
    )
    record_state_representation_delta_next_bridge = (
        "select_next_perf_owner"
        if record_state_representation_delta_ready
        else inputs.record_state_lowering_owner_next_bridge
    )
    record_state_next_bridge = (
        "record_state_field_access_site_measurement"
        if inputs.record_state_report_ready and not inputs.record_state_field_access_ready
        else record_state_representation_delta_next_bridge
        if inputs.record_state_field_access_ready
        else inputs.hot_field_next_bridge
    )
    next_perf_owner_selection_plan = int(
        inputs.structural_owner_refresh_required
        and inputs.product_pages_non_linear_lookup_probe_closed
        and record_state_representation_delta_ready
    )
    if not inputs.perf_attribution_report_consumed:
        next_perf_owner_selected = "perf_attribution_collection"
        next_perf_owner_reason = "perf_attribution_report_not_consumed"
        next_perf_owner_next_bridge = "run_hako_mimalloc_direct_exact_app_perf_asm"
    elif inputs.perf_delta_ready:
        next_perf_owner_selected = "owner_delta_measurement"
        next_perf_owner_reason = "symbol_attribution_available"
        next_perf_owner_next_bridge = inputs.perf_delta_next_bridge
    elif inputs.backend_store_shape_ready:
        next_perf_owner_selected = inputs.backend_store_shape_selected
        next_perf_owner_reason = "backend_store_shape_classifier_ready"
        if (
            inputs.backend_store_shape_selected == "primitive_dominant_mixed_store_shape"
            and inputs.inlined_hot_body_selected != "none"
        ):
            next_perf_owner_next_bridge = inputs.inlined_hot_body_split_next_bridge
        elif (
            inputs.backend_store_shape_selected == "direct_array_dominant_mixed_store_shape"
            and inputs.directarray_owner_instruction_shape_selected != "none"
        ):
            next_perf_owner_next_bridge = inputs.directarray_owner_instruction_shape_next_bridge
        else:
            next_perf_owner_next_bridge = inputs.backend_store_shape_next_bridge
    elif (
        inputs.primitive_hot_state_field_count > 0
        and record_state_representation_delta_ready
    ):
        next_perf_owner_selected = "asm_symbol_split_or_backend_store_shape"
        next_perf_owner_reason = (
            "primitive_state_store_like_hot_but_exact_slot_already_covers_record_state"
        )
        next_perf_owner_next_bridge = "split_symbol_or_classify_backend_store_shape"
    elif inputs.direct_array_owner_field_count > 0:
        next_perf_owner_selected = "directarray_owner_instruction_shape"
        next_perf_owner_reason = "direct_array_owner_field_hints_present"
        next_perf_owner_next_bridge = inputs.directarray_owner_instruction_shape_next_bridge
    elif inputs.instruction_attribution_available:
        next_perf_owner_selected = inputs.perf_delta_next_bridge
        next_perf_owner_reason = "instruction_attribution_without_known_owner"
        next_perf_owner_next_bridge = inputs.perf_delta_next_bridge
    else:
        next_perf_owner_selected = "none"
        next_perf_owner_reason = "missing_perf_instruction_attribution"
        next_perf_owner_next_bridge = "rerun_perf_with_higher_repeat_or_symbol"
    product_pages_non_linear_owner_candidate_ready = int(
        inputs.structural_owner_refresh_required
        and inputs.product_pages_source_ready
        and not inputs.product_pages_consumer_enabled
        and not inputs.product_pages_non_linear_lookup_probe_closed
    )
    if (
        inputs.page_model_hot_array_measurement_ready
        and record_state_representation_delta_ready
        and product_pages_non_linear_owner_candidate_ready
    ):
        structural_owner_selected = "product_pages_bridge_non_linear_owner_lookup"
        structural_owner_reason = "record_state_delta_closed_and_product_pages_source_ready"
        structural_owner_next_action = "design_non_linear_product_pages_bridge"
    elif inputs.page_model_hot_array_measurement_ready:
        structural_owner_selected = "page_model_hot_array_source_route_measurement"
        structural_owner_reason = "hotcore_measured_and_directarray_source_ready"
        structural_owner_next_action = (
            "measure_page_model_hot_array_perf_delta"
            if inputs.page_model_hot_array_source_route_measured
            else "measure_page_model_hot_array_source_route"
        )
        if inputs.page_model_hot_array_source_route_measured and inputs.perf_attribution_report_consumed:
            if inputs.perf_delta_ready:
                structural_owner_next_action = "select_next_perf_owner"
            elif inputs.record_state_report_ready:
                structural_owner_next_action = record_state_next_bridge
            elif inputs.hot_field_plan_ready:
                structural_owner_next_action = inputs.hot_field_next_bridge
            else:
                structural_owner_next_action = inputs.perf_delta_next_bridge
        if (
            structural_owner_next_action == "select_next_perf_owner"
            and next_perf_owner_selection_plan
        ):
            structural_owner_next_action = next_perf_owner_next_bridge
    elif product_pages_non_linear_owner_candidate_ready:
        structural_owner_selected = "product_pages_bridge_non_linear_owner_lookup"
        structural_owner_reason = "hotcore_measured_and_product_pages_source_ready"
        structural_owner_next_action = "design_non_linear_product_pages_bridge"
    elif inputs.structural_owner_refresh_required:
        structural_owner_selected = "none"
        structural_owner_reason = "no_source_ready_structural_owner"
        structural_owner_next_action = "fix_source_shape_before_next_probe"
    else:
        structural_owner_selected = "none"
        structural_owner_reason = "hotcore_measurement_not_reported"
        structural_owner_next_action = "measure_hotcore_replacement_consumer"
    return CoverageOwnerState(
        record_state_representation_delta_ready=record_state_representation_delta_ready,
        record_state_representation_delta_positive_candidate=record_state_representation_delta_positive_candidate,
        record_state_representation_delta_blocker=record_state_representation_delta_blocker,
        record_state_representation_delta_next_bridge=record_state_representation_delta_next_bridge,
        record_state_next_bridge=record_state_next_bridge,
        next_perf_owner_selection_plan=next_perf_owner_selection_plan,
        next_perf_owner_selected=next_perf_owner_selected,
        next_perf_owner_reason=next_perf_owner_reason,
        next_perf_owner_next_bridge=next_perf_owner_next_bridge,
        product_pages_non_linear_owner_candidate_ready=product_pages_non_linear_owner_candidate_ready,
        structural_owner_selected=structural_owner_selected,
        structural_owner_reason=structural_owner_reason,
        structural_owner_next_action=structural_owner_next_action,
    )
