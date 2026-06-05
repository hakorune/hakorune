"""Static replacement-front subject metadata lines for the mixed-ws compare report."""

from __future__ import annotations

from typing import Any

from replacement_front_report import (
    product_activation_contract_subject_fields,
    product_preflight_subject_fields,
)


def build_replacement_front_subject_static_lines(
    ctx: dict[str, Any],
    *,
    index: int,
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
    tls_page_arena = args.replacement_front_tls_page_arena_mode
    has_thread_local_arena = args.replacement_front_thread_local_mode or tls_page_arena
    has_multithread_safe_route = args.replacement_front_lock_mode or has_thread_local_arena
    cross_thread_free_policy = (
        "atomic_page_remote_head"
        if args.replacement_front_remote_free_queue_mode
        else "disabled"
        if tls_page_arena
        else "remote_queue"
        if args.replacement_front_thread_local_mode
        else "global_lock_or_not_applicable"
    )
    hotcore_route = (
        "benchmark_page_bins_hotcore_tls"
        if tls_page_arena
        else "benchmark_page_bins_hotcore_page_model"
        if args.replacement_front_hotcore_page_model_mode
        else "not_consumed_by_replacement_front"
    )
    thread_local_hotcore_route = (
        "benchmark_page_bins_hotcore_tls" if tls_page_arena else "not_consumed"
    )
    global_lock_hot_path_expected: int | str = (
        0
        if tls_page_arena
        else "lock_enter_count"
        if args.replacement_front_lock_mode
        else 0
    )
    page_from_ptr_route = (
        "side_table_direct"
        if args.replacement_front_page_from_ptr_bridge_mode
        else "indexed_page_table"
        if args.replacement_front_product_pages_nonlinear_mode
        else "range_scan"
        if replacement_front_bins_mode
        else "not_consumed"
    )
    hot_atomic_rmw = int(
        (
            replacement_front_bins_mode
            and args.replacement_front_lock_mode
            and not args.replacement_front_skip_hot_counters
        )
        or (
            not replacement_front_bins_mode
            and not (
                args.replacement_front_skip_hot_counters
                or args.replacement_front_tls_counter_mode
            )
        )
    )

    return [
        f"subject_{index}_provider_table_dispatch=0",
        f"subject_{index}_function_pointer_hot_call=0",
        f"subject_{index}_owns_check_hot_path=0",
        f"subject_{index}_tracking_hot_path=0",
        f"subject_{index}_direct_core_call=1",
        f"subject_{index}_single_thread_replacement_front_smoke={1 if args.threads == 1 else 0}",
        "subject_"
        f"{index}_multithread_replacement_front_smoke="
        f"{1 if args.threads > 1 and has_multithread_safe_route else 0}",
        f"subject_{index}_thread_local_replacement_front_smoke={1 if args.threads > 1 and has_thread_local_arena else 0}",
        f"subject_{index}_thread_safety_claim={'measured' if (args.threads > 1 and has_multithread_safe_route) else 'none'}",
        f"subject_{index}_thread_local_arena={1 if has_thread_local_arena else 0}",
        f"subject_{index}_cross_thread_free_policy={cross_thread_free_policy}",
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
        f"{1 if args.replacement_front_native_bins_mode or args.replacement_front_page_bins_mode else 0}",
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
        f"{hotcore_route}",
        f"subject_{index}_hako_mimalloc_algorithm_claim=0",
        f"subject_{index}_mimalloc_fidelity_guard=1",
        f"subject_{index}_mimalloc_fidelity_guard_passed=0",
        "subject_"
        f"{index}_replacement_front_thread_local_page_bins_mode="
        f"{1 if tls_page_arena else 0}",
        "subject_"
        f"{index}_replacement_front_thread_local_hotcore_route="
        f"{thread_local_hotcore_route}",
        "subject_"
        f"{index}_replacement_front_page_from_ptr_bridge_mode="
        f"{1 if args.replacement_front_page_from_ptr_bridge_mode else 0}",
        "subject_"
        f"{index}_replacement_front_page_from_ptr_route="
        f"{page_from_ptr_route}",
        f"subject_{index}_replacement_front_remote_free_queue_plan_v0=1",
        f"subject_{index}_replacement_front_remote_free_queue_report_only=1",
        "subject_"
        f"{index}_replacement_front_remote_free_queue_mode="
        f"{1 if args.replacement_front_remote_free_queue_mode else 0}",
        "subject_"
        f"{index}_replacement_front_remote_free_route="
        f"{cross_thread_free_policy}",
        "subject_"
        f"{index}_replacement_front_global_lock_hot_path_expected="
        f"{global_lock_hot_path_expected}",
        f"subject_{index}_replacement_front_hotpath_plan_v0=1",
        f"subject_{index}_replacement_front_hotpath_report_only=1",
        f"subject_{index}_tls_get_addr_hot_path={1 if has_thread_local_arena and not tls_initial_exec_model_enabled else 0}",
        f"subject_{index}_hot_atomic_rmw={hot_atomic_rmw}",
        "subject_"
        f"{index}_remote_free_drain_hot_path=0",
        "subject_"
        f"{index}_remote_owner_publication_after_local_fail="
        f"{1 if args.replacement_front_thread_local_mode or args.replacement_front_remote_free_queue_mode else 0}",
        f"subject_{index}_cold_init_in_hot_path=0",
        "subject_"
        f"{index}_register_thread_arena_hot_path=0",
        f"subject_{index}_fast_cold_split_plan=1",
        f"subject_{index}_tls_arena_fast_alloc_plan=1",
        f"subject_{index}_tls_arena_local_free_plan=1",
        f"subject_{index}_free_local_first=1",
        f"subject_{index}_free_remote_path_after_local_fail={1 if args.replacement_front_thread_local_mode or args.replacement_front_remote_free_queue_mode else 0}",
        f"subject_{index}_free_hot_remote_queue_call={1 if args.replacement_front_remote_free_queue_mode else 0}",
        f"subject_{index}_replacement_entry_inline_plan=1",
        f"subject_{index}_malloc_to_direct_alloc_boundary=always_inline",
        f"subject_{index}_free_to_direct_free_boundary=always_inline",
        f"subject_{index}_replacement_front_inplace_realloc_within_slot_plan=1",
        f"subject_{index}_replacement_front_slot_size={replacement_slot_size}",
    ]
