"""Report assembly for mimalloc algorithm coverage."""

from __future__ import annotations

from pathlib import Path

from allocator_field_buckets import (
    bucket_for_field,
    fields_from_context,
    fields_from_hint,
    format_field_buckets,
)
from hako_mimalloc_algorithm_coverage_support import (
    CoverageRow,
    ROOT,
    HAKO_ALLOC,
    REPLACEMENT_FRONT,
    REPLACEMENT_TEMPLATES,
    count_member_calls,
    has_all,
    has_file,
    hako_file,
    int_field,
    page_model_field_names,
    read_fastpath_counts,
    read_kv_report,
    read_text,
    str_field,
)
from hako_mimalloc_algorithm_coverage_owner_state import (
    CoverageOwnerStateInputs,
    derive_owner_state,
)
from hako_mimalloc_algorithm_coverage_route_state import (
    CoverageRouteStateInputs,
    derive_route_state,
)
from hako_mimalloc_algorithm_coverage_rows import refine_rows


def report_dict(
    rows: list[CoverageRow],
    *,
    benchmark_report: Path | None = None,
    fastpath_report: Path | None = None,
    state_report: Path | None = None,
    perf_attribution_report: Path | None = None,
    accumulator_report: Path | None = None,
) -> dict[str, object]:
    page_box = read_text(hako_file("page_box.hako"))
    hot_core = read_text(hako_file("object_lifecycle_hot_core_box.hako"))
    page_map = read_text(hako_file("page_map_box.hako"))
    page_map_release = read_text(hako_file("page_map_release_box.hako"))
    realloc_same = read_text(hako_file("page_map_realloc_same_class_box.hako"))
    realloc_grow = read_text(hako_file("page_map_realloc_alloc_copy_release_box.hako"))
    huge_model = read_text(hako_file("huge_page_model_box.hako"))
    osvm_source = read_text(hako_file("osvm_page_source_pilot_box.hako"))
    replacement = read_text(REPLACEMENT_FRONT) + "\n" + read_text(REPLACEMENT_TEMPLATES)
    hot_array_fields = ["free", "local_free", "block_used"]
    hot_array_ops = {
        name: {
            "get": count_member_calls(page_box, name, "get"),
            "set": count_member_calls(page_box, name, "set"),
            "push": count_member_calls(page_box, name, "push"),
        }
        for name in hot_array_fields
    }
    hot_array_get_count = sum(ops["get"] for ops in hot_array_ops.values())
    hot_array_set_count = sum(ops["set"] for ops in hot_array_ops.values())
    hot_array_push_count = sum(ops["push"] for ops in hot_array_ops.values())
    hot_array_arraybox_fields = [
        name for name in hot_array_fields if f"{name}: ArrayBox" in page_box
    ]
    hot_array_direct_fields = [
        name for name in hot_array_fields if f"{name}: DirectArrayI64" in page_box
    ]
    hot_array_source_type_ready = int(
        not hot_array_arraybox_fields and len(hot_array_direct_fields) == len(hot_array_fields)
    )
    hot_array_birth_contract_ready = int(
        hot_array_source_type_ready
        and has_all(page_box, ["new DirectArrayI64", ".set("])
        and hot_array_push_count == 0
    )
    hot_array_source_migration_selected = int(
        hot_array_source_type_ready and hot_array_birth_contract_ready
    )
    if hot_array_source_type_ready:
        migration_blocker = "none" if hot_array_birth_contract_ready else "directarray_i64_birth_contract_unverified"
    elif hot_array_push_count:
        migration_blocker = "push_or_initialized_len_contract"
    else:
        migration_blocker = "field_type_and_birth_contract_unverified"
    hotcore_methods = [
        method
        for method in ("objectLifecycleSmallAlloc", "objectLifecycleReleaseBlock")
        if method in hot_core
    ]
    hotcore_small_alloc_calls_acquire_fresh_small = int(
        "page.acquireFreshSmall(" in hot_core
    )
    hotcore_release_calls_release_local_known_live = int(
        "page.releaseLocalKnownLive(" in hot_core
    )
    page_model_hot_methods_ready = int(
        has_all(page_box, ["acquireFreshSmall", "releaseLocalKnownLive"])
    )
    page_map_source_ready = int(
        has_all(page_map, ["findIndex", "register", "lookup", "unregister"])
    )
    page_map_release_source_ready = int(
        has_all(page_map_release, ["releasePtr", "page_map.lookup", "page.releaseLocal", "page_map.unregister"])
    )
    realloc_same_class_source_ready = int(
        has_all(realloc_same, ["tryReallocSameClass", "page_map.lookup", "blockIsLive", "requested_size > page.block_size"])
    )
    realloc_grow_copy_release_source_ready = int(
        has_all(realloc_grow, ["page_map.lookup", "copy", "page_map.register"])
    )
    huge_page_source_ready = int(
        has_all(huge_model, ["register", "lookup", "huge"])
        or has_all(huge_model, ["allocateHuge", "markReleased", "requestedSizeFor"])
    )
    osvm_page_source_pilot_ready = int(
        has_all(osvm_source, ["osvm", "page"]) and has_file(hako_file("osvm_page_source_pilot_box.hako"))
    )
    size_class_single_bridge_supported = has_all(
        replacement,
        [
            "--replacement-front-match-hako-size-class",
            "hako_good_size",
            "hako_good_size_request_ceiling",
        ],
    )
    page_bins_bridge_supported = has_all(
        replacement,
        [
            "--replacement-front-page-bins-mode",
            "page_shaped",
            "HakoReplacement",
            "Page",
            "benchmark_page_bins",
        ],
    )
    locked_front = has_all(
        replacement,
        [
            "HAKO_REPLACEMENT_FRONT_LOCKED",
            "lock_arena",
            "pthread_mutex_lock(&arena_lock)",
        ],
    )
    tls_front = has_all(
        replacement,
        [
            "HAKO_REPLACEMENT_FRONT_THREAD_LOCAL",
            "remote_free_to_owner",
            "arena_registry",
        ],
    )
    replacement_full_hako = int(
        all(row.replacement_front for row in rows if row.area in {
            "size_class_policy",
            "page_local_free_stack",
            "same_thread_local_free",
            "object_lifecycle_hot_core",
            "page_map_lookup",
        })
    )
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
    benchmark_subject = "none"
    benchmark_subject_prefix = ""
    for prefix in ("subject_2", "subject_3", "subject_4"):
        subject_id = benchmark.get(f"{prefix}_id") or benchmark.get(f"{prefix}_name")
        if subject_id == "hakorune_replacement_front_ldpreload":
            benchmark_subject = subject_id
            benchmark_subject_prefix = prefix
            break
    if (
        benchmark_report_consumed
        and benchmark_subject == "none"
        and int_field(benchmark, "replacement_front_page_bins_consumer_enabled", 0)
    ):
        benchmark_subject = "hakorune_replacement_front_ldpreload"
    page_bins_consumer_enabled = int_field(
        benchmark,
        f"{benchmark_subject_prefix}_replacement_front_page_bins_consumer_enabled"
        if benchmark_subject_prefix
        else "replacement_front_page_bins_consumer_enabled",
        0,
    )
    page_bins_route = benchmark.get(
        f"{benchmark_subject_prefix}_replacement_front_page_bins_route"
        if benchmark_subject_prefix
        else "replacement_front_page_bins_route",
        "not_consumed",
    )
    page_bins_lookup_route = benchmark.get(
        f"{benchmark_subject_prefix}_replacement_front_page_bins_lookup_route"
        if benchmark_subject_prefix
        else "replacement_front_page_bins_lookup_route",
        "not_recorded" if page_bins_consumer_enabled else "not_consumed",
    )
    product_bins_consumer_enabled = int_field(
        benchmark,
        f"{benchmark_subject_prefix}_replacement_front_product_bins_consumer_enabled"
        if benchmark_subject_prefix
        else "replacement_front_product_bins_consumer_enabled",
        0,
    )
    product_bins_route = benchmark.get(
        f"{benchmark_subject_prefix}_replacement_front_product_bins_route"
        if benchmark_subject_prefix
        else "replacement_front_product_bins_route",
        "not_consumed",
    )
    product_pages_consumer_enabled = int_field(
        benchmark,
        f"{benchmark_subject_prefix}_replacement_front_product_pages_consumer_enabled"
        if benchmark_subject_prefix
        else "replacement_front_product_pages_consumer_enabled",
        0,
    )
    product_pages_route = benchmark.get(
        f"{benchmark_subject_prefix}_replacement_front_product_pages_route"
        if benchmark_subject_prefix
        else "replacement_front_product_pages_route",
        "not_consumed",
    )
    algorithm_shape = benchmark.get(
        f"{benchmark_subject_prefix}_replacement_front_algorithm_shape"
        if benchmark_subject_prefix
        else "replacement_front_algorithm_shape",
        "not_consumed",
    )
    hotcore_consumer_enabled = int_field(
        benchmark,
        f"{benchmark_subject_prefix}_replacement_front_hotcore_consumer_enabled"
        if benchmark_subject_prefix
        else "replacement_front_hotcore_consumer_enabled",
        0,
    )
    hotcore_route = benchmark.get(
        f"{benchmark_subject_prefix}_replacement_front_hotcore_route"
        if benchmark_subject_prefix
        else "replacement_front_hotcore_route",
        "not_consumed_by_replacement_front",
    )
    hotcore_median_ops_per_sec = str_field(
        benchmark,
        f"{benchmark_subject_prefix}_throughput_median_ops_per_sec"
        if benchmark_subject_prefix
        else "throughput_median_ops_per_sec",
        "0",
    )
    hotcore_measurement_reported = int(
        hotcore_consumer_enabled and hotcore_median_ops_per_sec != "0"
    )
    fastpath_direct_array_plan_count = int_field(
        fastpath, "direct_array_access_plan_count", 0
    )
    fastpath_route_decision_count = int_field(fastpath, "route_decision_count", 0)
    fastpath_fast_selected_count = int_field(
        fastpath, "route_decision_fast_selected_count", 0
    )
    fastpath_slow_selected_count = int_field(
        fastpath, "route_decision_slow_selected_count", 0
    )
    fastpath_generic_dispatch_count = int_field(
        fastpath, "generic_method_dispatch_count", 0
    )
    fastpath_dynamic_route_count = int_field(fastpath, "dynamic_route_count", 0)
    fastpath_boxed_fallback_count = int_field(fastpath, "boxed_fallback_count", 0)
    fastpath_clean = int_field(fastpath, "clean", 0)
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
    perf_delta_plan = int_field(
        perf_attribution, "page_model_hot_array_perf_delta_measurement_plan_v0", 0
    )
    perf_delta_ready = int_field(
        perf_attribution, "page_model_hot_array_perf_delta_ready", 0
    )
    perf_delta_blocker = str_field(
        perf_attribution,
        "page_model_hot_array_perf_delta_blocker",
        "perf_attribution_report_not_consumed"
        if not perf_attribution_report_consumed
        else "unknown",
    )
    perf_delta_next_bridge = str_field(
        perf_attribution,
        "page_model_hot_array_perf_delta_next_bridge",
        "run_hako_mimalloc_direct_exact_app_perf_asm"
        if not perf_attribution_report_consumed
        else "inspect_perf_attribution",
    )
    instruction_attribution_available = int_field(
        perf_attribution, "instruction_attribution_available", 0
    )
    perf_top_instruction_field_hints = str_field(
        perf_attribution, "top_instruction_field_hints", "none"
    )
    perf_hot_instruction_0_field_hints = str_field(
        perf_attribution, "hot_instruction_0_field_hints", "none"
    )
    perf_hot_instruction_0_context = str_field(
        perf_attribution, "hot_instruction_0_context", "none"
    )
    backend_store_shape_ready = int_field(
        perf_attribution, "backend_store_shape_ready", 0
    )
    backend_store_shape_selected = str_field(
        perf_attribution, "backend_store_shape_selected", "none"
    )
    backend_store_shape_next_bridge = str_field(
        perf_attribution,
        "backend_store_shape_next_bridge",
        "split_symbol_or_classify_backend_store_shape",
    )
    directarray_owner_instruction_shape_selected = str_field(
        perf_attribution, "directarray_owner_instruction_shape_selected", "none"
    )
    directarray_owner_instruction_shape_next_bridge = str_field(
        perf_attribution,
        "directarray_owner_instruction_shape_next_bridge",
        "collect_directarray_owner_instruction",
    )
    inlined_hot_body_selected = str_field(
        perf_attribution, "inlined_hot_body_selected", "none"
    )
    inlined_hot_body_next_bridge = str_field(
        perf_attribution,
        "inlined_hot_body_next_bridge",
        "rerun_perf_with_wider_context_or_symbol_split",
    )
    inlined_hot_body_split_next_bridge = str_field(
        perf_attribution,
        "inlined_hot_body_split_next_bridge",
        inlined_hot_body_next_bridge,
    )
    hot_field_names: list[str] = []
    for field in fields_from_hint(perf_top_instruction_field_hints):
        if field not in hot_field_names:
            hot_field_names.append(field)
    for field in fields_from_hint(perf_hot_instruction_0_field_hints):
        if field not in hot_field_names:
            hot_field_names.append(field)
    for field in fields_from_context(perf_hot_instruction_0_context):
        if field not in hot_field_names:
            hot_field_names.append(field)
    hot_field_bucket_names = [bucket_for_field(field) for field in hot_field_names]
    primitive_hot_state_field_count = sum(
        1 for bucket in hot_field_bucket_names if bucket == "primitive_hot_state"
    )
    public_or_proof_field_count = sum(
        1
        for bucket in hot_field_bucket_names
        if "public_semantics" in bucket or "proof_evidence" in bucket
    )
    observer_counter_field_count = sum(
        1 for bucket in hot_field_bucket_names if bucket == "observer_counter"
    )
    direct_array_owner_field_count = sum(
        1 for bucket in hot_field_bucket_names if bucket == "direct_array_owner"
    )
    hot_field_top = hot_field_names[0] if hot_field_names else "none"
    hot_field_top_bucket = (
        bucket_for_field(hot_field_top) if hot_field_top != "none" else "none"
    )
    hot_field_plan_ready = int(
        perf_attribution_report_consumed
        and instruction_attribution_available
        and primitive_hot_state_field_count > 0
    )
    hot_field_next_bridge = (
        "record_state_residence_plan_report"
        if hot_field_plan_ready
        else (
            "collect_perf_field_hints"
            if perf_attribution_report_consumed
            else "run_hako_mimalloc_direct_exact_app_perf_asm"
        )
    )
    page_model_fields = page_model_field_names(page_box)
    record_state_static_candidates = [
        field
        for field in page_model_fields
        if bucket_for_field(field) == "primitive_hot_state"
    ]
    record_state_observed_candidates = [
        field
        for field in hot_field_names
        if bucket_for_field(field) == "primitive_hot_state"
    ]
    record_state_observed_rejections = [
        field
        for field in hot_field_names
        if bucket_for_field(field) != "primitive_hot_state"
    ]
    record_state_field_access_plan_count = int_field(
        state, "record_state_field_access_plan_count", 0
    )
    record_state_field_access_ready = int(
        state_report_consumed and record_state_field_access_plan_count > 0
    )
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
    record_state_report_ready = int(
        hot_field_plan_ready and bool(record_state_observed_candidates)
    )
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
    return {
        "output_contract": "hako-mimalloc-algorithm-coverage-v0",
        "hako_alloc_root": str(HAKO_ALLOC.relative_to(ROOT)),
        "replacement_front": str(REPLACEMENT_FRONT.relative_to(ROOT)),
        "replacement_front_is_full_hako_algorithm": replacement_full_hako,
        "provider_activation": 0,
        "production_replacement_active": 0,
        "winner_claim": 0,
        "benchmark_report": str(benchmark_report) if benchmark_report is not None else "none",
        "benchmark_report_consumed": benchmark_report_consumed,
        "benchmark_replacement_subject": benchmark_subject,
        "fastpath_report": str(fastpath_report) if fastpath_report is not None else "none",
        "fastpath_report_consumed": fastpath_report_consumed,
        "state_report": str(state_report) if state_report is not None else "none",
        "state_report_consumed": state_report_consumed,
        "perf_attribution_report": str(perf_attribution_report)
        if perf_attribution_report is not None
        else "none",
        "perf_attribution_report_consumed": perf_attribution_report_consumed,
        "accumulator_report": str(accumulator_report)
        if accumulator_report is not None
        else "none",
        "accumulator_report_consumed": accumulator_report_consumed,
        "requested_bytes_accumulator_contract_v0": accumulator_contract_ready,
        "requested_bytes_accumulator_expected_no_overflow": int_field(
            accumulator, "expected_no_overflow", 0
        ),
        "requested_bytes_accumulator_observed_no_overflow": int_field(
            accumulator, "observed_no_overflow", 0
        ),
        "requested_bytes_accumulator_general_no_overflow_proof": int_field(
            accumulator, "general_no_overflow_proof", 0
        ),
        "requested_bytes_accumulator_source_reorder_allowed": int_field(
            accumulator, "source_reorder_allowed", 0
        ),
        "requested_bytes_accumulator_next_bridge": str_field(
            accumulator,
            "next_bridge",
            "add_public_proof_accumulator_overflow_policy_before_source_reorder",
        ),
        "area_count": len(rows),
        "hako_model_area_count": sum(row.hako_model for row in rows),
        "replacement_front_area_count": sum(row.replacement_front for row in rows),
        "model_only_area_count": sum(1 for row in rows if row.status == "model_only"),
        "split_model_and_fixed_front_area_count": sum(
            1 for row in rows if row.status == "split_model_and_fixed_front"
        ),
        "open_area_count": sum(1 for row in rows if row.status == "open"),
        "size_class_policy_bridge_plan_v0": 1,
        "size_class_policy_product_bins_connected": product_bins_consumer_enabled,
        "size_class_policy_single_class_benchmark_bridge_supported": int(
            size_class_single_bridge_supported
        ),
        "size_class_policy_single_class_bridge_mode": "hako_good_size_request_ceiling"
        if size_class_single_bridge_supported
        else "none",
        "size_class_policy_next_bridge": "product_replacement_bins_pages",
        "replacement_front_page_bins_plan_v0": 1,
        "replacement_front_page_bins_supported": int(page_bins_bridge_supported),
        "replacement_front_page_bins_consumer_enabled": page_bins_consumer_enabled,
        "replacement_front_page_bins_route": page_bins_route,
        "replacement_front_page_bins_lookup_route": page_bins_lookup_route,
        "replacement_front_page_bins_owner": "benchmark_only",
        "replacement_front_page_bins_product_claim": 0,
        "replacement_front_benchmark_algorithm_shape": algorithm_shape,
        "replacement_front_product_bins_consumer_enabled": product_bins_consumer_enabled,
        "replacement_front_product_bins_route": product_bins_route,
        "replacement_front_product_pages_bridge_plan_v0": 1,
        "replacement_front_product_pages_bridge_report_only": 1,
        "replacement_front_product_pages_consumer_enabled": product_pages_consumer_enabled,
        "replacement_front_product_pages_route": product_pages_route,
        "replacement_front_product_pages_source_ready": product_pages_source_ready,
        "replacement_front_product_pages_full_source_ready": product_pages_full_source_ready,
        "replacement_front_product_pages_bridge_blocker": product_pages_bridge_blocker,
        "replacement_front_product_pages_next_bridge": product_pages_next_bridge,
        "replacement_front_product_pages_non_linear_lookup_plan_v0": product_pages_non_linear_lookup_plan,
        "replacement_front_product_pages_linear_probe_closed": product_pages_linear_probe_closed,
        "replacement_front_product_pages_non_linear_lookup_probe_closed": product_pages_non_linear_lookup_probe_closed,
        "replacement_front_product_pages_non_linear_lookup_decision": product_pages_non_linear_lookup_decision,
        "replacement_front_product_pages_non_linear_lookup_strategy": product_pages_non_linear_lookup_strategy,
        "replacement_front_product_pages_non_linear_next_bridge": product_pages_non_linear_next_bridge,
        "page_map_source_ready": page_map_source_ready,
        "page_map_release_source_ready": page_map_release_source_ready,
        "realloc_same_class_source_ready": realloc_same_class_source_ready,
        "realloc_grow_copy_release_source_ready": realloc_grow_copy_release_source_ready,
        "huge_page_source_ready": huge_page_source_ready,
        "osvm_page_source_pilot_ready": osvm_page_source_pilot_ready,
        "replacement_front_locked_global_multithread_supported": int(locked_front),
        "replacement_front_thread_local_multithread_supported": int(tls_front),
        "replacement_front_multithread_claim": 0,
        "structural_owner_selection_plan_v0": 1,
        "structural_owner_refresh_required": structural_owner_refresh_required,
        "structural_owner_selected": structural_owner_selected,
        "structural_owner_selected_reason": structural_owner_reason,
        "structural_owner_next_action": structural_owner_next_action,
        "structural_owner_candidate_0": "page_model_hot_array_source_route_measurement",
        "structural_owner_candidate_0_ready": page_model_hot_array_measurement_ready,
        "structural_owner_candidate_1": "product_pages_bridge_non_linear_owner_lookup",
        "structural_owner_candidate_1_ready": product_pages_non_linear_owner_candidate_ready,
        "next_perf_owner_selection_plan_v0": next_perf_owner_selection_plan,
        "next_perf_owner_selected": next_perf_owner_selected,
        "next_perf_owner_selected_reason": next_perf_owner_reason,
        "next_perf_owner_next_bridge": next_perf_owner_next_bridge,
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
        "page_model_hot_field_traffic_plan_v0": 1,
        "page_model_hot_field_traffic_ready": hot_field_plan_ready,
        "page_model_hot_field_top": hot_field_top,
        "page_model_hot_field_top_bucket": hot_field_top_bucket,
        "page_model_hot_field_buckets": format_field_buckets(hot_field_names),
        "page_model_hot_field_primitive_hot_state_count": primitive_hot_state_field_count,
        "page_model_hot_field_public_or_proof_count": public_or_proof_field_count,
        "page_model_hot_field_observer_counter_count": observer_counter_field_count,
        "page_model_hot_field_counter_deletion_allowed": 0,
        "page_model_hot_field_next_bridge": hot_field_next_bridge,
        "record_state_residence_plan_v0": 1,
        "record_state_residence_report_only": 1,
        "record_state_residence_ready": record_state_report_ready,
        "record_state_field_access_plan_count": record_state_field_access_plan_count,
        "record_state_field_access_ready": record_state_field_access_ready,
        "record_state_field_access_lowering_enabled": int_field(
            state, "record_state_field_access_lowering_enabled", 0
        ),
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
        "record_state_residence_static_candidate_fields": ",".join(
            record_state_static_candidates
        )
        or "none",
        "record_state_residence_observed_candidate_fields": ",".join(
            record_state_observed_candidates
        )
        or "none",
        "record_state_residence_rejected_observed_fields": format_field_buckets(
            record_state_observed_rejections
        ),
        "record_state_residence_source_migration_allowed": 0,
        "record_state_residence_next_bridge": record_state_next_bridge,
        "perf_top_symbol": str_field(perf_attribution, "top_symbol", "none"),
        "perf_top_symbol_percent": str_field(perf_attribution, "top_symbol_percent", "0.00"),
        "perf_symbol_collapse_detected": int_field(
            perf_attribution, "symbol_collapse_detected", 0
        ),
        "perf_symbol_attribution_available": int_field(
            perf_attribution, "symbol_attribution_available", 0
        ),
        "perf_instruction_attribution_available": int_field(
            perf_attribution, "instruction_attribution_available", 0
        ),
        "perf_annotate_nonzero_instruction_count": int_field(
            perf_attribution, "annotate_nonzero_instruction_count", 0
        ),
        "perf_top_instruction_percent": str_field(
            perf_attribution, "top_instruction_percent", "0.00"
        ),
        "perf_top_instruction_mnemonic": str_field(
            perf_attribution, "top_instruction_mnemonic", "none"
        ),
        "perf_top_instruction_category": str_field(
            perf_attribution, "top_instruction_category", "none"
        ),
        "perf_top_instruction_field_hints": perf_top_instruction_field_hints,
        "perf_hot_instruction_report_count": int_field(
            perf_attribution, "hot_instruction_report_count", 0
        ),
        "perf_hot_instruction_0_category": str_field(
            perf_attribution, "hot_instruction_0_category", "none"
        ),
        "perf_hot_instruction_0_field_hints": perf_hot_instruction_0_field_hints,
        "perf_hot_instruction_0_asm": str_field(
            perf_attribution, "hot_instruction_0_asm", "none"
        ),
        "perf_hot_instruction_0_context_categories": str_field(
            perf_attribution, "hot_instruction_0_context_categories", "none"
        ),
        "perf_hot_instruction_0_context_count": int_field(
            perf_attribution, "hot_instruction_0_context_count", 0
        ),
        "perf_backend_store_shape_classifier_v0": int_field(
            perf_attribution, "backend_store_shape_classifier_v0", 0
        ),
        "perf_backend_store_shape_ready": backend_store_shape_ready,
        "perf_backend_store_shape_selected": backend_store_shape_selected,
        "perf_backend_store_shape_next_bridge": backend_store_shape_next_bridge,
        "perf_backend_store_shape_hot_store_field_buckets": str_field(
            perf_attribution, "backend_store_shape_hot_store_field_buckets", "none"
        ),
        "perf_backend_store_shape_context_field_buckets": str_field(
            perf_attribution, "backend_store_shape_context_field_buckets", "none"
        ),
        "perf_backend_store_shape_weighted_dominant_bucket": str_field(
            perf_attribution, "backend_store_shape_weighted_dominant_bucket", "none"
        ),
        "perf_backend_store_shape_primitive_hot_state_store_percent": str_field(
            perf_attribution,
            "backend_store_shape_primitive_hot_state_store_percent",
            "0.00",
        ),
        "perf_backend_store_shape_public_or_proof_store_percent": str_field(
            perf_attribution,
            "backend_store_shape_public_or_proof_store_percent",
            "0.00",
        ),
        "perf_backend_store_shape_direct_array_owner_store_percent": str_field(
            perf_attribution,
            "backend_store_shape_direct_array_owner_store_percent",
            "0.00",
        ),
        "perf_backend_store_shape_observer_counter_store_percent": str_field(
            perf_attribution,
            "backend_store_shape_observer_counter_store_percent",
            "0.00",
        ),
        "perf_directarray_owner_instruction_shape_classifier_v0": int_field(
            perf_attribution, "directarray_owner_instruction_shape_classifier_v0", 0
        ),
        "perf_directarray_owner_instruction_shape_selected": str_field(
            perf_attribution, "directarray_owner_instruction_shape_selected", "none"
        ),
        "perf_directarray_owner_instruction_shape_next_bridge": str_field(
            perf_attribution,
            "directarray_owner_instruction_shape_next_bridge",
            "collect_directarray_owner_instruction",
        ),
        "perf_inlined_hot_body_classifier_v0": int_field(
            perf_attribution, "inlined_hot_body_classifier_v0", 0
        ),
        "perf_inlined_hot_body_selected": inlined_hot_body_selected,
        "perf_inlined_hot_body_next_bridge": inlined_hot_body_next_bridge,
        "perf_inlined_hot_body_split_ready": int_field(
            perf_attribution, "inlined_hot_body_split_ready", 0
        ),
        "perf_inlined_hot_body_split_blocker": str_field(
            perf_attribution, "inlined_hot_body_split_blocker", "none"
        ),
        "perf_inlined_hot_body_split_next_bridge": inlined_hot_body_split_next_bridge,
        "perf_inlined_hot_body_acquire_fresh_small_percent": str_field(
            perf_attribution,
            "inlined_hot_body_acquire_fresh_small_percent",
            "0.00",
        ),
        "perf_inlined_hot_body_release_local_known_live_percent": str_field(
            perf_attribution,
            "inlined_hot_body_release_local_known_live_percent",
            "0.00",
        ),
        "perf_inlined_hot_body_init_public_store_percent": str_field(
            perf_attribution,
            "inlined_hot_body_init_public_store_percent",
            "0.00",
        ),
        "perf_inlined_hot_body_mixed_percent": str_field(
            perf_attribution,
            "inlined_hot_body_mixed_percent",
            "0.00",
        ),
        "perf_public_proof_accumulator_plan_v0": int_field(
            perf_attribution, "public_proof_accumulator_plan_v0", 0
        ),
        "perf_public_proof_accumulator_fields": str_field(
            perf_attribution, "public_proof_accumulator_fields", "none"
        ),
        "perf_public_proof_accumulator_policy": str_field(
            perf_attribution, "public_proof_accumulator_policy", "none"
        ),
        "perf_public_proof_accumulator_source_reorder_allowed": int_field(
            perf_attribution, "public_proof_accumulator_source_reorder_allowed", 0
        ),
        "perf_public_proof_accumulator_observed_requested_bytes": str_field(
            perf_attribution, "public_proof_accumulator_observed_requested_bytes", "none"
        ),
        "perf_public_proof_accumulator_observed_no_overflow": int_field(
            perf_attribution, "public_proof_accumulator_observed_no_overflow", 0
        ),
        "perf_public_proof_accumulator_observed_i64_margin": str_field(
            perf_attribution, "public_proof_accumulator_observed_i64_margin", "none"
        ),
        "perf_public_proof_accumulator_general_no_overflow_proof": int_field(
            perf_attribution, "public_proof_accumulator_general_no_overflow_proof", 0
        ),
        "perf_public_proof_accumulator_next_bridge": str_field(
            perf_attribution, "public_proof_accumulator_next_bridge", "none"
        ),
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
        "rows": [row.__dict__ for row in rows],
    }
