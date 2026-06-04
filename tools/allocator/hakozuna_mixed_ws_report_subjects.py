"""Render per-subject sections for the Hakozuna mixed-ws compare report."""

from __future__ import annotations

from typing import Any

from hakozuna_mixed_ws_report_support import (
    format_ratio,
    provider_front_class,
)
from hakozuna_mixed_ws_report_replacement_front_subjects import (
    build_replacement_front_subject_lines,
)
from replacement_front_support import counter_value, median_float


def build_subject_lines(ctx: dict[str, Any]) -> list[str]:
    args = ctx["args"]
    subject_specs = ctx["subject_specs"]
    reports = ctx["reports"]
    c_mimalloc_median = ctx["c_mimalloc_median"]
    replacement_front_bins_mode = ctx["replacement_front_bins_mode"]
    replacement_slot_size = ctx["replacement_slot_size"]
    replacement_front_size_class_request_ceiling = ctx["replacement_front_size_class_request_ceiling"]
    replacement_front_size_class_selected_bin = ctx["replacement_front_size_class_selected_bin"]
    replacement_front_size_class_selected_good_size = ctx["replacement_front_size_class_selected_good_size"]
    replacement_front_size_class_policy_source = ctx["replacement_front_size_class_policy_source"]
    replacement_front_product_pages_consumer_enabled = ctx["replacement_front_product_pages_consumer_enabled"]
    replacement_front_algorithm_shape = ctx["replacement_front_algorithm_shape"]
    replacement_front_product_bins_route = ctx["replacement_front_product_bins_route"]
    replacement_front_product_pages_route = ctx["replacement_front_product_pages_route"]
    replacement_front_product_pages_non_linear_lookup_selected = ctx[
        "replacement_front_product_pages_non_linear_lookup_selected"
    ]
    replacement_front_page_bins_route = ctx["replacement_front_page_bins_route"]
    replacement_front_page_bins_lookup_route = ctx["replacement_front_page_bins_lookup_route"]
    replacement_front_size_class_bridge_enabled = ctx["replacement_front_size_class_bridge_enabled"]
    replacement_front_size_class_bridge_mode = ctx["replacement_front_size_class_bridge_mode"]
    replacement_front_evidence_owner = ctx["replacement_front_evidence_owner"]
    replacement_front_multithread_perf_candidate = ctx["replacement_front_multithread_perf_candidate"]
    replacement_front_thread_local_perf_candidate = ctx["replacement_front_thread_local_perf_candidate"]
    replacement_front_correctness_smoke = ctx["replacement_front_correctness_smoke"]
    replacement_front_preflight = ctx["replacement_front_preflight"]
    workload_histogram = ctx["workload_histogram"]

    lines: list[str] = []
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
            route = ctx["provider_route_metadata"].get("provider_ldpreload_measurement_route", "")
            front_class = provider_front_class(route)
            hako_hot_path_claim = ctx["provider_route_metadata"].get(
                "provider_ldpreload_hako_hot_path_claim", "0"
            )
            declared_route = ctx["provider_route_metadata"].get(
                "provider_ldpreload_declared_route", "provider_ldpreload_unknown"
            )
            execution_route = ctx["provider_route_metadata"].get(
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
            tls_initial_exec_enabled = (
                counter_value(counters, "replacement_front_tls_initial_exec_model_enabled") > 0
            )
            lines.extend(
                build_replacement_front_subject_lines(
                    ctx,
                    index=index,
                    counters=counters,
                    tls_initial_exec_model_enabled=tls_initial_exec_enabled,
                )
            )
    return lines
