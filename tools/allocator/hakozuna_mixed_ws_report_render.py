"""Render the report for the Hakozuna mixed-ws compare tool."""

from __future__ import annotations

import argparse
from pathlib import Path

from hakozuna_mixed_ws_report_support import (
    format_per_operation,
    format_ratio,
    init_fallback_dominates_provider_ops,
    load_manifest_metadata,
    provider_front_class,
    provider_kind_from_route,
    provider_ldpreload_route_metadata,
)
from replacement_front_report import (
    ReplacementFrontPreflight,
    product_activation_contract_fields,
    product_activation_contract_subject_fields,
    product_preflight_fields,
    product_preflight_subject_fields,
)
from replacement_front_support import (
    WORKLOAD_HISTOGRAM_MAX_TOTAL_ITERS,
    counter_value,
    hako_good_size,
    hako_size_class_bin_size,
    hako_size_to_bin,
    median_float,
)


def render_hakozuna_mixed_ws_report(
    *,
    args: argparse.Namespace,
    bench: Path,
    root: Path,
    mimalloc_library: Path,
    workload_histogram: dict[str, int | str],
    replacement_front_smokes: dict[str, dict[str, str]],
    subject_specs: list[tuple[str, Path | None, Path | None, bool]],
    reports: dict[str, tuple[list[float], list[float], dict[str, int]]],
    replacement_front_bins_mode: bool,
    replacement_slot_size: int,
    replacement_front_size_class_request_ceiling: int,
    replacement_front_size_class_selected_bin: int,
    replacement_front_size_class_selected_good_size: int,
) -> str:
    provider_manifest_metadata = load_manifest_metadata(
        args.manifest.resolve() if args.manifest is not None else None
    )
    provider_route_metadata = provider_ldpreload_route_metadata(provider_manifest_metadata)

    c_mimalloc_median = median_float(reports["c_mimalloc_ldpreload"][0])
    all_sample_seconds = [
        elapsed
        for _samples, elapsed_samples, _counters in reports.values()
        for elapsed in elapsed_samples
    ]
    min_observed_sample_seconds = min(all_sample_seconds) if all_sample_seconds else 0.0
    median_observed_sample_seconds = (
        median_float(all_sample_seconds) if all_sample_seconds else 0.0
    )
    measurement_quality = (
        "ok"
        if args.min_sample_seconds <= 0.0
        or min_observed_sample_seconds >= args.min_sample_seconds
        else "too_short"
    )
    replacement_front_size_class_policy_source = (
        "hako_size_class_box_report_mirror"
        if args.replacement_front_match_hako_size_class
        or replacement_front_bins_mode
        else "hako_model_not_consumed"
    )
    replacement_front_product_pages_consumer_enabled = int(
        args.replacement_front_product_pages_nonlinear_mode
    )
    if (
        args.replacement_front_product_pages_nonlinear_mode
        and args.replacement_front_hotcore_page_model_mode
    ):
        replacement_front_algorithm_shape = (
            "page_bin_hotcore_page_model_product_pages_nonlinear_benchmark_front"
        )
    elif args.replacement_front_product_pages_nonlinear_mode:
        replacement_front_algorithm_shape = "page_bin_product_pages_nonlinear_benchmark_front"
    elif args.replacement_front_hotcore_page_model_mode:
        replacement_front_algorithm_shape = "page_bin_hotcore_page_model_benchmark_front"
    elif args.replacement_front_page_bins_mode:
        replacement_front_algorithm_shape = "page_bin_benchmark_front"
    elif args.replacement_front_native_bins_mode:
        replacement_front_algorithm_shape = "multi_bin_native_benchmark_front"
    else:
        replacement_front_algorithm_shape = "fixed_slot_native_benchmark_front"
    replacement_front_product_bins_route = (
        "benchmark_page_bins_hotcore_page_model"
        if args.replacement_front_hotcore_page_model_mode
        else "benchmark_page_bins"
        if args.replacement_front_page_bins_mode
        else "benchmark_native_bins"
        if args.replacement_front_native_bins_mode
        else "not_consumed"
    )
    replacement_front_product_pages_route = (
        "benchmark_product_pages_indexed_page_table"
        if args.replacement_front_product_pages_nonlinear_mode
        else "not_consumed"
    )
    replacement_front_product_pages_non_linear_lookup_selected = (
        "indexed_page_table"
        if args.replacement_front_product_pages_nonlinear_mode
        else "not_selected"
    )
    replacement_front_preflight = ReplacementFrontPreflight.from_evidence(
        measurement_quality=measurement_quality,
        has_smoke_pack=bool(replacement_front_smokes),
        thread_local_mode=args.replacement_front_thread_local_mode,
        cross_thread_smoke=args.replacement_front_cross_thread_smoke,
        provider_dispatch_bypass=(
            args.replacement_front_native_slot_mode
            or args.replacement_front_native_bins_mode
            or args.replacement_front_page_bins_mode
        ),
    )
    replacement_front_page_bins_route = (
        "benchmark_page_bins_hotcore_page_model"
        if args.replacement_front_hotcore_page_model_mode
        else "benchmark_page_bins"
        if args.replacement_front_page_bins_mode
        else "not_consumed"
    )
    replacement_front_page_bins_lookup_route = (
        "indexed_page_table"
        if args.replacement_front_product_pages_nonlinear_mode
        else "range_scan"
        if args.replacement_front_page_bins_mode
        else "not_consumed"
    )
    replacement_front_size_class_bridge_enabled = int(
        args.replacement_front_match_hako_size_class
        or replacement_front_bins_mode
    )
    replacement_front_size_class_bridge_mode = (
        "workload_regular_bins_page_shaped_hotcore_page_model_hako_size_class"
        if args.replacement_front_hotcore_page_model_mode
        else "workload_regular_bins_page_shaped_hako_size_class"
        if args.replacement_front_page_bins_mode
        else "workload_regular_bins_hako_size_class"
        if args.replacement_front_native_bins_mode
        else (
            "hako_good_size_request_ceiling"
            if args.replacement_front_match_hako_size_class
            else "none"
        )
    )
    replacement_front_evidence_owner = "none"
    replacement_front_multithread_perf_candidate = 0
    replacement_front_thread_local_perf_candidate = 0
    replacement_front_correctness_smoke = 0
    if args.replacement_front_page_bins_mode:
        replacement_front_evidence_owner = (
            "single_thread_page_bins_hotcore_page_model"
            if args.replacement_front_hotcore_page_model_mode
            else "single_thread_page_bins"
        )
    elif args.replacement_front_native_bins_mode:
        replacement_front_evidence_owner = "single_thread_native_bins"
    elif args.replacement_front_native_slot_mode:
        replacement_front_evidence_owner = "fixed_slot_native_front"
        if args.threads > 1 and args.replacement_front_lock_mode:
            replacement_front_evidence_owner = "locked_global_multithread_front"
            if args.replacement_front_skip_hot_counters:
                replacement_front_multithread_perf_candidate = 1
        elif args.threads > 1 and args.replacement_front_thread_local_mode:
            replacement_front_evidence_owner = "thread_local_multithread_front"
            replacement_front_thread_local_perf_candidate = int(
                args.replacement_front_skip_hot_counters
            )
            replacement_front_correctness_smoke = int(args.replacement_front_cross_thread_smoke)

    lines = [
        "output_contract=hakozuna-mixed-ws-ldpreload-compare-v0",
        "type_abi_route_descriptor_present=1",
        "type_abi_descriptor_plane=route_descriptor_control_plane",
        "type_abi_hot_path_lookup_count=0",
        "benchmark_id=bench_mixed_ws_crt",
        f"benchmark_path={bench}",
        f"hakozuna_root={root}",
        f"mimalloc_library={mimalloc_library}",
        f"provider_manifest={args.manifest.resolve() if args.manifest is not None else 'none'}",
        f"sample_count={args.sample_count}",
        f"warmup_count={args.warmup_count}",
        f"min_sample_seconds_required={args.min_sample_seconds:.6f}",
        f"min_observed_sample_seconds={min_observed_sample_seconds:.6f}",
        f"median_observed_sample_seconds={median_observed_sample_seconds:.6f}",
        f"measurement_quality={measurement_quality}",
        f"benchmark_threads={args.threads}",
        f"benchmark_iters_per_thread={args.iters_per_thread}",
        f"benchmark_working_set={args.working_set}",
        f"benchmark_min_size={args.min_size}",
        f"benchmark_max_size={args.max_size}",
        f"subject_count={len(subject_specs)}",
        "reference_subject=c_mimalloc_ldpreload",
        "provider_activation=0",
        "production_replacement_active=0",
        "hook_installed=0",
        "global_allocator_product_claim=0",
        "winner_claim=0",
        f"provider_usable_size_mode={1 if args.provider_usable_size_mode else 0}",
        f"provider_assume_owned_mode={1 if args.provider_assume_owned_mode else 0}",
        f"replacement_front_native_slot_mode={1 if args.replacement_front_native_slot_mode else 0}",
        f"replacement_front_native_bins_mode={1 if args.replacement_front_native_bins_mode else 0}",
        f"replacement_front_page_bins_mode={1 if args.replacement_front_page_bins_mode else 0}",
        "replacement_front_hotcore_page_model_mode="
        f"{1 if args.replacement_front_hotcore_page_model_mode else 0}",
        "replacement_front_size_class_table_mode="
        f"{1 if args.replacement_front_size_class_table_mode else 0}",
        "replacement_front_eager_init_mode="
        f"{1 if args.replacement_front_eager_init_mode else 0}",
        "replacement_front_product_pages_nonlinear_mode="
        f"{1 if args.replacement_front_product_pages_nonlinear_mode else 0}",
        "replacement_front_is_full_hako_algorithm=0",
        "replacement_front_ordinary_app_route_candidate=replacement_front_product_ldpreload",
        *product_activation_contract_fields(),
        *product_preflight_fields(replacement_front_preflight),
        f"replacement_front_algorithm_shape={replacement_front_algorithm_shape}",
        "replacement_front_size_class_bridge_plan_v0=1",
        "replacement_front_size_class_bridge_report_only=1",
        f"replacement_front_size_class_policy_bridge={replacement_front_size_class_bridge_enabled}",
        "replacement_front_size_class_count="
        f"{workload_histogram['size_class_regular_distinct_count'] if replacement_front_bins_mode else 1}",
        f"replacement_front_size_class_policy_source={replacement_front_size_class_policy_source}",
        f"replacement_front_size_class_bridge_mode={replacement_front_size_class_bridge_mode}",
        "replacement_front_size_class_lookup_route="
        f"{'table_8byte_bucket' if args.replacement_front_size_class_table_mode else 'range_scan' if replacement_front_bins_mode else 'not_consumed'}",
        "replacement_front_size_class_request_ceiling="
        f"{replacement_front_size_class_request_ceiling}",
        "replacement_front_size_class_selected_bin="
        f"{replacement_front_size_class_selected_bin}",
        "replacement_front_size_class_selected_good_size="
        f"{replacement_front_size_class_selected_good_size}",
        "replacement_front_product_bins_plan_v0=1",
        "replacement_front_product_bins_report_only=1",
        "replacement_front_product_bins_consumer_enabled="
        f"{1 if replacement_front_bins_mode else 0}",
        "replacement_front_product_bins_connected=0",
        f"replacement_front_product_bins_route={replacement_front_product_bins_route}",
        "replacement_front_product_pages_plan_v0=1",
        "replacement_front_product_pages_report_only=1",
        "replacement_front_product_pages_consumer_enabled="
        f"{replacement_front_product_pages_consumer_enabled}",
        "replacement_front_benchmark_product_pages_consumer_enabled="
        f"{replacement_front_product_pages_consumer_enabled}",
        "replacement_front_product_pages_connected=0",
        "replacement_front_product_pages_product_connected=0",
        "replacement_front_product_pages_next_bridge=design_non_linear_product_pages_bridge",
        "replacement_front_product_pages_non_linear_lookup_plan_v0=1",
        "replacement_front_product_pages_linear_probe_closed=1",
        "replacement_front_product_pages_non_linear_lookup_strategy=range_decision_tree_or_indexed_page_table",
        "replacement_front_product_pages_non_linear_lookup_selected="
        f"{replacement_front_product_pages_non_linear_lookup_selected}",
        "replacement_front_product_pages_non_linear_next_bridge=replacement_front_product_pages_non_linear_plan",
        f"replacement_front_product_pages_route={replacement_front_product_pages_route}",
        "replacement_front_benchmark_product_pages_route="
        f"{replacement_front_product_pages_route}",
        "replacement_front_page_bins_plan_v0=1",
        "replacement_front_page_bins_report_only=1",
        "replacement_front_page_bins_consumer_enabled="
        f"{1 if args.replacement_front_page_bins_mode else 0}",
        f"replacement_front_page_bins_route={replacement_front_page_bins_route}",
        f"replacement_front_page_bins_lookup_route={replacement_front_page_bins_lookup_route}",
        "replacement_front_page_bins_owner=benchmark_only",
        "replacement_front_page_bins_product_claim=0",
        "replacement_front_product_bins_required_regular_distinct_count="
        f"{workload_histogram['size_class_regular_distinct_count']}",
        "replacement_front_product_bins_required_regular_bins="
        f"{workload_histogram['size_class_regular_bins']}",
        "replacement_front_product_bins_required_max_bin="
        f"{workload_histogram['size_class_max_bin']}",
        "replacement_front_product_bins_huge_route_required="
        f"{1 if int(workload_histogram['size_class_huge_count']) > 0 else 0}",
        "replacement_front_hotcore_bridge_plan_v0=1",
        "replacement_front_hotcore_bridge_report_only=1",
        "replacement_front_hotcore_consumer_enabled="
        f"{1 if args.replacement_front_hotcore_page_model_mode else 0}",
        "replacement_front_hotcore_route="
        f"{'benchmark_page_bins_hotcore_page_model' if args.replacement_front_hotcore_page_model_mode else 'not_consumed_by_replacement_front'}",
        "hako_mimalloc_algorithm_claim=0",
        f"replacement_front_lock_mode={1 if args.replacement_front_lock_mode else 0}",
        f"replacement_front_thread_local_mode={1 if args.replacement_front_thread_local_mode else 0}",
        f"replacement_front_evidence_owner={replacement_front_evidence_owner}",
        "replacement_front_multithread_perf_candidate="
        f"{replacement_front_multithread_perf_candidate}",
        "replacement_front_thread_local_perf_candidate="
        f"{replacement_front_thread_local_perf_candidate}",
        f"replacement_front_correctness_smoke={replacement_front_correctness_smoke}",
        f"replacement_front_cross_thread_smoke={1 if args.replacement_front_cross_thread_smoke else 0}",
        f"replacement_front_skip_hot_counters={1 if args.replacement_front_skip_hot_counters else 0}",
        f"replacement_front_tls_counter_mode={1 if args.replacement_front_tls_counter_mode else 0}",
        f"replacement_front_slot_size={replacement_slot_size}",
        "replacement_front_match_workload_realloc_size="
        f"{1 if args.replacement_front_match_workload_realloc_size else 0}",
        "replacement_front_match_hako_size_class="
        f"{1 if args.replacement_front_match_hako_size_class else 0}",
        f"workload_size_histogram_source={workload_histogram['source']}",
        "workload_size_histogram_max_total_iters="
        f"{WORKLOAD_HISTOGRAM_MAX_TOTAL_ITERS}",
        "workload_size_histogram_sample_exact="
        f"{workload_histogram['sample_exact']}",
        "workload_size_histogram_sampled_iters_per_thread="
        f"{workload_histogram['sampled_iters_per_thread']}",
        "workload_size_histogram_sampled_total_iterations="
        f"{workload_histogram['sampled_total_iterations']}",
        "workload_size_histogram_full_total_iterations="
        f"{workload_histogram['full_total_iterations']}",
        "workload_alloc_request_count="
        f"{workload_histogram['alloc_request_count']}",
        "workload_free_path_count="
        f"{workload_histogram['free_path_count']}",
        "workload_cleanup_free_count="
        f"{workload_histogram['cleanup_free_count']}",
        "workload_realloc_request_count="
        f"{workload_histogram['realloc_request_count']}",
        "workload_realloc_request_gt_replacement_slot_size="
        f"{workload_histogram['realloc_request_gt_replacement_slot_size']}",
        "workload_realloc_request_gt_max_size="
        f"{workload_histogram['realloc_request_gt_max_size']}",
        "workload_memset_le_64_count="
        f"{workload_histogram['memset_le_64_count']}",
        "workload_memset_gt_64_count="
        f"{workload_histogram['memset_gt_64_count']}",
        "workload_size_class_policy_source="
        f"{workload_histogram['size_class_policy_source']}",
        "workload_size_class_distinct_count="
        f"{workload_histogram['size_class_distinct_count']}",
        "workload_size_class_regular_distinct_count="
        f"{workload_histogram['size_class_regular_distinct_count']}",
        "workload_size_class_regular_bins="
        f"{workload_histogram['size_class_regular_bins']}",
        "workload_size_class_max_bin="
        f"{workload_histogram['size_class_max_bin']}",
        "workload_size_class_max_good_size="
        f"{workload_histogram['size_class_max_good_size']}",
        "workload_size_class_huge_count="
        f"{workload_histogram['size_class_huge_count']}",
        "workload_size_class_regular_request_count="
        f"{workload_histogram['size_class_regular_request_count']}",
        "workload_request_le_64="
        f"{workload_histogram['request_le_64']}",
        "workload_request_le_128="
        f"{workload_histogram['request_le_128']}",
        "workload_request_le_256="
        f"{workload_histogram['request_le_256']}",
        "workload_request_le_512="
        f"{workload_histogram['request_le_512']}",
        "workload_request_le_1024="
        f"{workload_histogram['request_le_1024']}",
        "workload_request_gt_1024="
        f"{workload_histogram['request_gt_1024']}",
    ]
    if replacement_front_smokes:
        malloc_family = replacement_front_smokes["malloc_family"]
        cross_thread_free = replacement_front_smokes["cross_thread_free"]
        abandoned_owner = replacement_front_smokes["abandoned_owner"]
        cross_thread_realloc = replacement_front_smokes["cross_thread_realloc"]
        lines.extend(
            [
                "replacement_front_product_smoke_pack_v0=1",
                "replacement_front_product_smoke_pack_non_activating=1",
                "replacement_front_malloc_family_smoke_ok=1",
                "replacement_front_cross_thread_free_smoke_ok=1",
                "replacement_front_abandoned_owner_smoke_ok=1",
                "replacement_front_cross_thread_realloc_smoke_ok=1",
                "replacement_front_malloc_family_null_free_smoke_ok=1",
                "replacement_front_malloc_family_alloc_count="
                f"{counter_value(malloc_family, 'replacement_front_alloc_count')}",
                "replacement_front_malloc_family_calloc_count="
                f"{counter_value(malloc_family, 'replacement_front_calloc_count')}",
                "replacement_front_malloc_family_realloc_count="
                f"{counter_value(malloc_family, 'replacement_front_realloc_count')}",
                "replacement_front_malloc_family_free_count="
                f"{counter_value(malloc_family, 'replacement_front_free_count')}",
                "replacement_front_malloc_family_realloc_inplace_count="
                f"{counter_value(malloc_family, 'replacement_front_realloc_inplace_count')}",
                "replacement_front_malloc_family_calloc_zero_bytes="
                f"{counter_value(malloc_family, 'replacement_front_calloc_zero_bytes')}",
                "replacement_front_malloc_family_host_passthrough_count="
                f"{counter_value(malloc_family, 'replacement_front_host_passthrough_count')}",
                "replacement_front_cross_thread_free_policy=remote_queue",
                "replacement_front_abandoned_owner_policy=mark_abandoned_no_host_free",
                "replacement_front_cross_thread_realloc_policy=unsupported_counted",
                "replacement_front_cross_thread_free_remote_free_push_count="
                f"{counter_value(cross_thread_free, 'replacement_front_remote_free_push_count')}",
                "replacement_front_cross_thread_free_remote_free_drain_count="
                f"{counter_value(cross_thread_free, 'replacement_front_remote_free_drain_count')}",
                "replacement_front_cross_thread_free_arena_registry_overflow_count="
                f"{counter_value(cross_thread_free, 'replacement_front_arena_registry_overflow_count')}",
                "replacement_front_abandoned_owner_abandoned_arena_count="
                f"{counter_value(abandoned_owner, 'replacement_front_abandoned_arena_count')}",
                "replacement_front_abandoned_owner_abandoned_remote_free_count="
                f"{counter_value(abandoned_owner, 'replacement_front_abandoned_remote_free_count')}",
                "replacement_front_abandoned_owner_host_passthrough_count="
                f"{counter_value(abandoned_owner, 'replacement_front_host_passthrough_count')}",
                "replacement_front_cross_thread_realloc_unsupported_count="
                f"{counter_value(cross_thread_realloc, 'replacement_front_cross_thread_realloc_unsupported_count')}",
                "replacement_front_cross_thread_realloc_host_passthrough_count="
                f"{counter_value(cross_thread_realloc, 'replacement_front_host_passthrough_count')}",
            ]
        )
    for key in sorted(provider_manifest_metadata):
        lines.append(f"{key}={provider_manifest_metadata[key]}")
    for key in sorted(provider_route_metadata):
        lines.append(f"{key}={provider_route_metadata[key]}")
    if args.manifest is not None:
        provider_execution_route = provider_route_metadata.get(
            "provider_ldpreload_measurement_route", ""
        )
        lines.extend(
            [
                "provider_ldpreload_benchmark_front_class="
                f"{provider_front_class(provider_execution_route)}",
                "provider_ldpreload_measurement_interpretation=provider_abi_wrapper_and_shim_bridge",
                "provider_ldpreload_is_product_allocator_claim=0",
                "provider_ldpreload_is_hako_core_speed_claim=0",
                "provider_registration_v1_present=1",
                "provider_registration_descriptor_plane=type_abi_route_descriptor",
                "provider_registration_ops_plane=provider_abi_execution_ops",
                "provider_registration_descriptor_ops_pairing=1",
                "provider_registration_hot_path_uses=provider_ops_only",
                "provider_registration_type_abi_hot_path_lookup_count=0",
                "provider_ops_version=1",
                "provider_claim_ops_enabled="
                f"{provider_manifest_metadata.get('provider_manifest_provider_abi_claim_ops_v1', '0')}",
                f"provider_kind={provider_kind_from_route(provider_execution_route)}",
            ]
        )
    for index, (subject, _ld_preload, _provider, replacement_front_mode) in enumerate(subject_specs):
        samples, sample_seconds, counters = reports[subject]
        median = median_float(samples)
        if subject == "system_malloc":
            front_class = "system_malloc"
            hako_hot_path_claim = "0"
            declared_route = "system_malloc"
            execution_route = "system_malloc"
        elif subject == "c_mimalloc_ldpreload":
            front_class = "c_mimalloc_ldpreload"
            hako_hot_path_claim = "0"
            declared_route = "c_mimalloc_ldpreload"
            execution_route = "c_mimalloc_ldpreload"
        elif subject == "hakorune_provider_ldpreload":
            route = provider_route_metadata.get("provider_ldpreload_measurement_route", "")
            front_class = provider_front_class(route)
            hako_hot_path_claim = provider_route_metadata.get(
                "provider_ldpreload_hako_hot_path_claim", "0"
            )
            declared_route = provider_route_metadata.get(
                "provider_ldpreload_declared_route", "provider_ldpreload_unknown"
            )
            execution_route = provider_route_metadata.get(
                "provider_ldpreload_execution_route", route or "provider_ldpreload_unknown"
            )
        elif replacement_front_mode:
            front_class = "replacement_front_c_shim"
            hako_hot_path_claim = "0"
            declared_route = "replacement_front_benchmark"
            execution_route = "replacement_front_benchmark"
        else:
            front_class = "unknown"
            hako_hot_path_claim = "0"
            declared_route = "unknown"
            execution_route = "unknown"
        lines.extend(
            [
                f"subject_{index}_id={subject}",
                f"subject_{index}_declared_route={declared_route}",
                f"subject_{index}_execution_route={execution_route}",
                f"subject_{index}_benchmark_front_class={front_class}",
                f"subject_{index}_hako_hot_path_claim={hako_hot_path_claim}",
                f"subject_{index}_throughput_min_ops_per_sec={min(samples):.3f}",
                f"subject_{index}_throughput_median_ops_per_sec={median:.3f}",
                f"subject_{index}_throughput_max_ops_per_sec={max(samples):.3f}",
                f"subject_{index}_throughput_vs_c_mimalloc={format_ratio(median, c_mimalloc_median)}",
                f"subject_{index}_sample_seconds_min={min(sample_seconds):.6f}",
                f"subject_{index}_sample_seconds_median={median_float(sample_seconds):.6f}",
                f"subject_{index}_sample_seconds_max={max(sample_seconds):.6f}",
                f"subject_{index}_winner_claim=0",
            ]
        )
        if replacement_front_mode:
            single_thread_smoke = args.threads == 1
            thread_local_smoke = args.threads > 1 and args.replacement_front_thread_local_mode
            multithread_smoke = args.threads > 1 and (
                args.replacement_front_lock_mode or args.replacement_front_thread_local_mode
            )
            tls_initial_exec_enabled = (
                counter_value(counters, "replacement_front_tls_initial_exec_model_enabled") > 0
            )
            tls_get_addr_hot_path = (
                args.replacement_front_thread_local_mode and not tls_initial_exec_enabled
            )
            hot_atomic_rmw = not (
                replacement_front_bins_mode
                or args.replacement_front_skip_hot_counters
                or args.replacement_front_tls_counter_mode
            )
            lines.extend(
                [
                    f"subject_{index}_provider_table_dispatch=0",
                    f"subject_{index}_function_pointer_hot_call=0",
                    f"subject_{index}_owns_check_hot_path=0",
                    f"subject_{index}_tracking_hot_path=0",
                    f"subject_{index}_direct_core_call=1",
                    f"subject_{index}_single_thread_replacement_front_smoke={1 if single_thread_smoke else 0}",
                    f"subject_{index}_multithread_replacement_front_smoke={1 if multithread_smoke else 0}",
                    f"subject_{index}_thread_local_replacement_front_smoke={1 if thread_local_smoke else 0}",
                    f"subject_{index}_thread_safety_claim={'measured' if (multithread_smoke or thread_local_smoke) else 'none'}",
                    f"subject_{index}_thread_local_arena={1 if args.replacement_front_thread_local_mode else 0}",
                    "subject_"
                    f"{index}_cross_thread_free_policy="
                    f"{'remote_queue' if args.replacement_front_thread_local_mode else 'global_lock_or_not_applicable'}",
                    f"subject_{index}_provider_api_hot_path_required=0",
                    f"subject_{index}_activation=0",
                    f"subject_{index}_benchmark_only=1",
                    f"subject_{index}_replacement_front_is_full_hako_algorithm=0",
                    "subject_"
                    f"{index}_replacement_front_ordinary_app_route_candidate="
                    "replacement_front_product_ldpreload",
                    *product_activation_contract_subject_fields(index),
                    *product_preflight_subject_fields(
                        index, replacement_front_preflight
                    ),
                    f"subject_{index}_replacement_front_algorithm_shape={replacement_front_algorithm_shape}",
                    f"subject_{index}_replacement_front_evidence_owner="
                    f"{replacement_front_evidence_owner}",
                    "subject_"
                    f"{index}_replacement_front_multithread_perf_candidate="
                    f"{replacement_front_multithread_perf_candidate}",
                    "subject_"
                    f"{index}_replacement_front_thread_local_perf_candidate="
                    f"{replacement_front_thread_local_perf_candidate}",
                    "subject_"
                    f"{index}_replacement_front_correctness_smoke="
                    f"{replacement_front_correctness_smoke}",
                    f"subject_{index}_replacement_front_native_bins_mode={1 if args.replacement_front_native_bins_mode else 0}",
                    f"subject_{index}_replacement_front_page_bins_mode={1 if args.replacement_front_page_bins_mode else 0}",
                    "subject_"
                    f"{index}_replacement_front_hotcore_page_model_mode="
                    f"{1 if args.replacement_front_hotcore_page_model_mode else 0}",
                    "subject_"
                    f"{index}_replacement_front_product_pages_nonlinear_mode="
                    f"{1 if args.replacement_front_product_pages_nonlinear_mode else 0}",
                    f"subject_{index}_replacement_front_size_class_bridge_plan_v0=1",
                    f"subject_{index}_replacement_front_size_class_bridge_report_only=1",
                    "subject_"
                    f"{index}_replacement_front_size_class_policy_bridge="
                    f"{replacement_front_size_class_bridge_enabled}",
                    "subject_"
                    f"{index}_replacement_front_size_class_count="
                    f"{workload_histogram['size_class_regular_distinct_count'] if replacement_front_bins_mode else 1}",
                    "subject_"
                    f"{index}_replacement_front_size_class_policy_source="
                    f"{replacement_front_size_class_policy_source}",
                    "subject_"
                    f"{index}_replacement_front_size_class_bridge_mode="
                    f"{replacement_front_size_class_bridge_mode}",
                    "subject_"
                    f"{index}_replacement_front_size_class_request_ceiling="
                    f"{replacement_front_size_class_request_ceiling}",
                    "subject_"
                    f"{index}_replacement_front_size_class_selected_bin="
                    f"{replacement_front_size_class_selected_bin}",
                    "subject_"
                    f"{index}_replacement_front_size_class_selected_good_size="
                    f"{replacement_front_size_class_selected_good_size}",
                    f"subject_{index}_replacement_front_product_bins_plan_v0=1",
                    f"subject_{index}_replacement_front_product_bins_report_only=1",
                    "subject_"
                    f"{index}_replacement_front_product_bins_consumer_enabled="
                    f"{1 if replacement_front_bins_mode else 0}",
                    f"subject_{index}_replacement_front_product_bins_connected=0",
                    "subject_"
                    f"{index}_replacement_front_product_bins_route="
                    f"{replacement_front_product_bins_route}",
                    f"subject_{index}_replacement_front_product_pages_plan_v0=1",
                    f"subject_{index}_replacement_front_product_pages_report_only=1",
                    "subject_"
                    f"{index}_replacement_front_product_pages_consumer_enabled="
                    f"{replacement_front_product_pages_consumer_enabled}",
                    "subject_"
                    f"{index}_replacement_front_benchmark_product_pages_consumer_enabled="
                    f"{replacement_front_product_pages_consumer_enabled}",
                    f"subject_{index}_replacement_front_product_pages_connected=0",
                    f"subject_{index}_replacement_front_product_pages_product_connected=0",
                    "subject_"
                    f"{index}_replacement_front_product_pages_next_bridge="
                    "design_non_linear_product_pages_bridge",
                    "subject_"
                    f"{index}_replacement_front_product_pages_non_linear_lookup_plan_v0=1",
                    "subject_"
                    f"{index}_replacement_front_product_pages_linear_probe_closed=1",
                    "subject_"
                    f"{index}_replacement_front_product_pages_non_linear_lookup_strategy="
                    "range_decision_tree_or_indexed_page_table",
                    "subject_"
                    f"{index}_replacement_front_product_pages_non_linear_lookup_selected="
                    f"{replacement_front_product_pages_non_linear_lookup_selected}",
                    "subject_"
                    f"{index}_replacement_front_product_pages_non_linear_next_bridge="
                    "replacement_front_product_pages_non_linear_plan",
                    "subject_"
                    f"{index}_replacement_front_product_pages_route="
                    f"{replacement_front_product_pages_route}",
                    "subject_"
                    f"{index}_replacement_front_benchmark_product_pages_route="
                    f"{replacement_front_product_pages_route}",
                    f"subject_{index}_replacement_front_page_bins_plan_v0=1",
                    f"subject_{index}_replacement_front_page_bins_report_only=1",
                    "subject_"
                    f"{index}_replacement_front_page_bins_consumer_enabled="
                    f"{1 if args.replacement_front_page_bins_mode else 0}",
                    "subject_"
                    f"{index}_replacement_front_page_bins_route="
                    f"{replacement_front_page_bins_route}",
                    "subject_"
                    f"{index}_replacement_front_page_bins_lookup_route="
                    f"{replacement_front_page_bins_lookup_route}",
                    f"subject_{index}_replacement_front_page_bins_owner=benchmark_only",
                    f"subject_{index}_replacement_front_page_bins_product_claim=0",
                    "subject_"
                    f"{index}_replacement_front_product_bins_required_regular_distinct_count="
                    f"{workload_histogram['size_class_regular_distinct_count']}",
                    "subject_"
                    f"{index}_replacement_front_product_bins_required_regular_bins="
                    f"{workload_histogram['size_class_regular_bins']}",
                    "subject_"
                    f"{index}_replacement_front_product_bins_required_max_bin="
                    f"{workload_histogram['size_class_max_bin']}",
                    "subject_"
                    f"{index}_replacement_front_product_bins_huge_route_required="
                    f"{1 if int(workload_histogram['size_class_huge_count']) > 0 else 0}",
                    f"subject_{index}_replacement_front_hotcore_bridge_plan_v0=1",
                    f"subject_{index}_replacement_front_hotcore_bridge_report_only=1",
                    "subject_"
                    f"{index}_replacement_front_hotcore_consumer_enabled="
                    f"{1 if args.replacement_front_hotcore_page_model_mode else 0}",
                    "subject_"
                    f"{index}_replacement_front_hotcore_route="
                    f"{'benchmark_page_bins_hotcore_page_model' if args.replacement_front_hotcore_page_model_mode else 'not_consumed_by_replacement_front'}",
                    f"subject_{index}_hako_mimalloc_algorithm_claim=0",
                    f"subject_{index}_replacement_front_hotpath_plan_v0=1",
                    f"subject_{index}_replacement_front_hotpath_report_only=1",
                    f"subject_{index}_tls_get_addr_hot_path={1 if tls_get_addr_hot_path else 0}",
                    f"subject_{index}_hot_atomic_rmw={1 if hot_atomic_rmw else 0}",
                    "subject_"
                    f"{index}_remote_free_drain_hot_path="
                    "0",
                    "subject_"
                    f"{index}_remote_owner_publication_after_local_fail="
                    f"{1 if args.replacement_front_thread_local_mode else 0}",
                    f"subject_{index}_cold_init_in_hot_path=0",
                    "subject_"
                    f"{index}_register_thread_arena_hot_path="
                    "0",
                    f"subject_{index}_fast_cold_split_plan=1",
                    f"subject_{index}_tls_arena_fast_alloc_plan=1",
                    f"subject_{index}_tls_arena_local_free_plan=1",
                    f"subject_{index}_free_local_first=1",
                    f"subject_{index}_free_remote_path_after_local_fail="
                    f"{1 if args.replacement_front_thread_local_mode else 0}",
                    f"subject_{index}_free_hot_remote_queue_call=0",
                    f"subject_{index}_replacement_entry_inline_plan=1",
                    f"subject_{index}_malloc_to_direct_alloc_boundary=always_inline",
                    f"subject_{index}_free_to_direct_free_boundary=always_inline",
                    f"subject_{index}_replacement_front_inplace_realloc_within_slot_plan=1",
                    f"subject_{index}_replacement_front_slot_size="
                    f"{replacement_slot_size}",
                ]
            )
        if counters:
            for key in sorted(counters):
                lines.append(f"subject_{index}_{key}_total={counters[key]}")
            provider_ops = (
                counters.get("shim_provider_alloc_count", 0)
                + counters.get("shim_provider_calloc_count", 0)
                + counters.get("shim_provider_realloc_count", 0)
                + counters.get("shim_provider_free_count", 0)
            )
            init_fallback_dominates = init_fallback_dominates_provider_ops(counters, provider_ops)
            lines.extend(
                [
                    f"subject_{index}_shim_provider_operation_count_total={provider_ops}",
                    "subject_"
                    f"{index}_shim_init_real_fallback_per_provider_operation="
                    f"{format_per_operation(counters.get('shim_init_real_fallback_count', 0), provider_ops)}",
                    "subject_"
                    f"{index}_shim_host_passthrough_per_provider_operation="
                    f"{format_per_operation(counters.get('shim_host_passthrough_count', 0), provider_ops)}",
                    "subject_"
                    f"{index}_shim_init_real_fallback_dominates_provider_ops="
                    f"{1 if init_fallback_dominates else 0}",
                ]
            )
            if init_fallback_dominates:
                lines.extend(
                    [
                        "subject_"
                        f"{index}_next_owner_family=provider_alloc_free_internal_real_malloc_boundary",
                        "subject_"
                        f"{index}_gap_classification=provider_bridge_not_hako_core_speed",
                    ]
                )
            lines.append(f"subject_{index}_shim_init_real_fallback_is_perf_diagnostic=1")
    lines.append("summary=ok")
    return "\n".join(lines) + "\n"
