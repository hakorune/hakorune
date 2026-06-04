"""Replacement-front subject report lines for the Hakozuna mixed-ws compare report."""

from __future__ import annotations

from typing import Any

from hakozuna_mixed_ws_report_support import (
    format_per_operation,
    init_fallback_dominates_provider_ops,
)
from replacement_front_report import (
    product_activation_contract_subject_fields,
    product_preflight_subject_fields,
)


def build_replacement_front_subject_lines(
    ctx: dict[str, Any],
    *,
    index: int,
    samples: list[float],
    sample_seconds: list[float],
    counters: dict[str, int],
    front_class: str,
    hako_hot_path_claim: str,
    declared_route: str,
    execution_route: str,
    tls_initial_exec_model_enabled: bool,
) -> list[str]:
    args = ctx["args"]
    replacement_front_bins_mode = ctx["replacement_front_bins_mode"]
    replacement_slot_size = ctx["replacement_slot_size"]
    replacement_front_size_class_request_ceiling = ctx[
        "replacement_front_size_class_request_ceiling"
    ]
    replacement_front_size_class_selected_bin = ctx[
        "replacement_front_size_class_selected_bin"
    ]
    replacement_front_size_class_selected_good_size = ctx[
        "replacement_front_size_class_selected_good_size"
    ]
    replacement_front_size_class_policy_source = ctx[
        "replacement_front_size_class_policy_source"
    ]
    replacement_front_product_pages_consumer_enabled = ctx[
        "replacement_front_product_pages_consumer_enabled"
    ]
    replacement_front_algorithm_shape = ctx["replacement_front_algorithm_shape"]
    replacement_front_product_bins_route = ctx["replacement_front_product_bins_route"]
    replacement_front_product_pages_route = ctx["replacement_front_product_pages_route"]
    replacement_front_product_pages_non_linear_lookup_selected = ctx[
        "replacement_front_product_pages_non_linear_lookup_selected"
    ]
    replacement_front_page_bins_route = ctx["replacement_front_page_bins_route"]
    replacement_front_page_bins_lookup_route = ctx["replacement_front_page_bins_lookup_route"]
    replacement_front_size_class_bridge_enabled = ctx[
        "replacement_front_size_class_bridge_enabled"
    ]
    replacement_front_size_class_bridge_mode = ctx["replacement_front_size_class_bridge_mode"]
    replacement_front_evidence_owner = ctx["replacement_front_evidence_owner"]
    replacement_front_multithread_perf_candidate = ctx[
        "replacement_front_multithread_perf_candidate"
    ]
    replacement_front_thread_local_perf_candidate = ctx[
        "replacement_front_thread_local_perf_candidate"
    ]
    replacement_front_correctness_smoke = ctx["replacement_front_correctness_smoke"]
    replacement_front_preflight = ctx["replacement_front_preflight"]
    workload_histogram = ctx["workload_histogram"]

    lines = [
        f"subject_{index}_provider_table_dispatch=0",
        f"subject_{index}_function_pointer_hot_call=0",
        f"subject_{index}_owns_check_hot_path=0",
        f"subject_{index}_tracking_hot_path=0",
        f"subject_{index}_direct_core_call=1",
        f"subject_{index}_single_thread_replacement_front_smoke={1 if args.threads == 1 else 0}",
        "subject_"
        f"{index}_multithread_replacement_front_smoke="
        f"{1 if args.threads > 1 and (args.replacement_front_lock_mode or args.replacement_front_thread_local_mode) else 0}",
        f"subject_{index}_thread_local_replacement_front_smoke={1 if args.threads > 1 and args.replacement_front_thread_local_mode else 0}",
        f"subject_{index}_thread_safety_claim={'measured' if (args.threads > 1 and (args.replacement_front_lock_mode or args.replacement_front_thread_local_mode)) else 'none'}",
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
        *product_preflight_subject_fields(index, replacement_front_preflight),
        f"subject_{index}_replacement_front_algorithm_shape={replacement_front_algorithm_shape}",
        f"subject_{index}_replacement_front_evidence_owner={replacement_front_evidence_owner}",
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
        f"subject_{index}_tls_get_addr_hot_path={1 if args.replacement_front_thread_local_mode and not tls_initial_exec_model_enabled else 0}",
        f"subject_{index}_hot_atomic_rmw={1 if not (replacement_front_bins_mode or args.replacement_front_skip_hot_counters or args.replacement_front_tls_counter_mode) else 0}",
        "subject_"
        f"{index}_remote_free_drain_hot_path=0",
        "subject_"
        f"{index}_remote_owner_publication_after_local_fail="
        f"{1 if args.replacement_front_thread_local_mode else 0}",
        f"subject_{index}_cold_init_in_hot_path=0",
        "subject_"
        f"{index}_register_thread_arena_hot_path=0",
        f"subject_{index}_fast_cold_split_plan=1",
        f"subject_{index}_tls_arena_fast_alloc_plan=1",
        f"subject_{index}_tls_arena_local_free_plan=1",
        f"subject_{index}_free_local_first=1",
        f"subject_{index}_free_remote_path_after_local_fail={1 if args.replacement_front_thread_local_mode else 0}",
        f"subject_{index}_free_hot_remote_queue_call=0",
        f"subject_{index}_replacement_entry_inline_plan=1",
        f"subject_{index}_malloc_to_direct_alloc_boundary=always_inline",
        f"subject_{index}_free_to_direct_free_boundary=always_inline",
        f"subject_{index}_replacement_front_inplace_realloc_within_slot_plan=1",
        f"subject_{index}_replacement_front_slot_size={replacement_slot_size}",
    ]

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

    return lines
