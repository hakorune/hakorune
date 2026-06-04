"""Render the report for the Hakozuna mixed-ws compare tool."""

from __future__ import annotations

import argparse
from pathlib import Path

from hakozuna_mixed_ws_report_subjects import build_subject_lines
from hakozuna_mixed_ws_report_support import (
    load_manifest_metadata,
    provider_ldpreload_route_metadata,
)
from hakozuna_mixed_ws_report_smoke_fields import replacement_front_smoke_pack_fields
from replacement_front_report import (
    ReplacementFrontPreflight,
    product_activation_contract_fields,
    product_activation_contract_subject_fields,
    product_preflight_fields,
    product_preflight_subject_fields,
)
from replacement_front_support import (
    WORKLOAD_HISTOGRAM_MAX_TOTAL_ITERS,
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
    lines.extend(replacement_front_smoke_pack_fields(replacement_front_smokes))
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
    subject_lines = build_subject_lines(
        {
            "args": args,
            "subject_specs": subject_specs,
            "reports": reports,
            "c_mimalloc_median": c_mimalloc_median,
            "replacement_front_bins_mode": replacement_front_bins_mode,
            "replacement_slot_size": replacement_slot_size,
            "replacement_front_size_class_request_ceiling": replacement_front_size_class_request_ceiling,
            "replacement_front_size_class_selected_bin": replacement_front_size_class_selected_bin,
            "replacement_front_size_class_selected_good_size": replacement_front_size_class_selected_good_size,
            "replacement_front_size_class_policy_source": replacement_front_size_class_policy_source,
            "replacement_front_product_pages_consumer_enabled": replacement_front_product_pages_consumer_enabled,
            "replacement_front_algorithm_shape": replacement_front_algorithm_shape,
            "replacement_front_product_bins_route": replacement_front_product_bins_route,
            "replacement_front_product_pages_route": replacement_front_product_pages_route,
            "replacement_front_product_pages_non_linear_lookup_selected": replacement_front_product_pages_non_linear_lookup_selected,
            "replacement_front_page_bins_route": replacement_front_page_bins_route,
            "replacement_front_page_bins_lookup_route": replacement_front_page_bins_lookup_route,
            "replacement_front_size_class_bridge_enabled": replacement_front_size_class_bridge_enabled,
            "replacement_front_size_class_bridge_mode": replacement_front_size_class_bridge_mode,
            "replacement_front_evidence_owner": replacement_front_evidence_owner,
            "replacement_front_multithread_perf_candidate": replacement_front_multithread_perf_candidate,
            "replacement_front_thread_local_perf_candidate": replacement_front_thread_local_perf_candidate,
            "replacement_front_correctness_smoke": replacement_front_correctness_smoke,
            "replacement_front_preflight": replacement_front_preflight,
            "workload_histogram": workload_histogram,
            "provider_route_metadata": provider_route_metadata,
        }
    )
    lines.extend(subject_lines)
    lines.append("summary=ok")
    return "\n".join(lines) + "\n"
