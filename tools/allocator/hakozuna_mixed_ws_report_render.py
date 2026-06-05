"""Render the report for the Hakozuna mixed-ws compare tool."""

from __future__ import annotations

import argparse
from pathlib import Path

from hakozuna_mixed_ws_report_preamble import build_report_preamble_lines
from hakozuna_mixed_ws_report_subjects import build_subject_lines
from hakozuna_mixed_ws_report_support import (
    load_manifest_metadata,
    provider_ldpreload_route_metadata,
)
from replacement_front_route_plan import algorithm_shape, page_bins_lookup_route
from replacement_front_report import ReplacementFrontPreflight
from replacement_front_support import median_float


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
    replacement_front_algorithm_shape = algorithm_shape(args)
    replacement_front_product_bins_route = (
        "benchmark_page_bins_hotcore_tls"
        if args.replacement_front_tls_page_arena_mode
        else
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
        thread_local_mode=(
            args.replacement_front_thread_local_mode
            or args.replacement_front_tls_page_arena_mode
        ),
        cross_thread_smoke=args.replacement_front_cross_thread_smoke,
        provider_dispatch_bypass=(
            args.replacement_front_native_slot_mode
            or args.replacement_front_native_bins_mode
            or args.replacement_front_page_bins_mode
        ),
    )
    replacement_front_page_bins_route = (
        "benchmark_page_bins_hotcore_tls"
        if args.replacement_front_tls_page_arena_mode
        else
        "benchmark_page_bins_hotcore_page_model"
        if args.replacement_front_hotcore_page_model_mode
        else "benchmark_page_bins"
        if args.replacement_front_page_bins_mode
        else "not_consumed"
    )
    replacement_front_page_bins_lookup_route = page_bins_lookup_route(args)
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
    if (
        args.threads > 1
        and args.replacement_front_lock_mode
        and (
            args.replacement_front_native_slot_mode
            or args.replacement_front_native_bins_mode
            or args.replacement_front_page_bins_mode
        )
    ):
        replacement_front_evidence_owner = "locked_global_multithread_front"
        replacement_front_multithread_perf_candidate = int(
            args.replacement_front_native_bins_mode
            or args.replacement_front_page_bins_mode
            or args.replacement_front_skip_hot_counters
        )
    elif args.replacement_front_tls_page_arena_mode:
        replacement_front_evidence_owner = "thread_local_page_bins_hotcore_tls"
        replacement_front_multithread_perf_candidate = int(args.threads > 1)
        replacement_front_thread_local_perf_candidate = 1
    elif args.replacement_front_page_bins_mode:
        replacement_front_evidence_owner = (
            "single_thread_page_bins_hotcore_page_model"
            if args.replacement_front_hotcore_page_model_mode
            else "single_thread_page_bins"
        )
    elif args.replacement_front_native_bins_mode:
        replacement_front_evidence_owner = "single_thread_native_bins"
    elif args.replacement_front_native_slot_mode:
        replacement_front_evidence_owner = "fixed_slot_native_front"
        if args.threads > 1 and args.replacement_front_thread_local_mode:
            replacement_front_evidence_owner = "thread_local_multithread_front"
            replacement_front_thread_local_perf_candidate = int(
                args.replacement_front_skip_hot_counters
            )
            replacement_front_correctness_smoke = int(args.replacement_front_cross_thread_smoke)

    lines = build_report_preamble_lines(
        locals()
        | {
            "subject_count": len(subject_specs),
            "replacement_front_size_class_request_ceiling": replacement_front_size_class_request_ceiling,
            "replacement_front_size_class_selected_bin": replacement_front_size_class_selected_bin,
            "replacement_front_size_class_selected_good_size": replacement_front_size_class_selected_good_size,
        }
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
