"""Workload and size-class preamble lines for the Hakozuna mixed-ws compare report."""

from __future__ import annotations

from typing import Any

from replacement_front_support import WORKLOAD_HISTOGRAM_MAX_TOTAL_ITERS


def build_report_preamble_workload_lines(ctx: dict[str, Any]) -> list[str]:
    args = ctx["args"]
    workload_histogram = ctx["workload_histogram"]
    replacement_front_bins_mode = ctx["replacement_front_bins_mode"]
    hotcore_route = (
        "benchmark_page_bins_hotcore_tls"
        if args.replacement_front_tls_page_arena_mode
        else "benchmark_page_bins_hotcore_page_model"
        if args.replacement_front_hotcore_page_model_mode
        else "not_consumed_by_replacement_front"
    )
    remote_free_route = (
        "disabled"
        if args.replacement_front_tls_page_arena_mode
        else "atomic_page_remote_head"
        if args.replacement_front_thread_local_mode
        else "global_lock_or_not_applicable"
    )
    global_lock_hot_path_expected: int | str = (
        0
        if args.replacement_front_tls_page_arena_mode
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

    return [
        "replacement_front_size_class_bridge_plan_v0=1",
        "replacement_front_size_class_bridge_report_only=1",
        "replacement_front_size_class_policy_bridge="
        f"{ctx['replacement_front_size_class_bridge_enabled']}",
        "replacement_front_size_class_count="
        f"{workload_histogram['size_class_regular_distinct_count'] if replacement_front_bins_mode else 1}",
        "replacement_front_size_class_policy_source="
        f"{ctx['replacement_front_size_class_policy_source']}",
        "replacement_front_size_class_bridge_mode="
        f"{ctx['replacement_front_size_class_bridge_mode']}",
        "replacement_front_size_class_lookup_route="
        f"{'table_8byte_bucket' if args.replacement_front_size_class_table_mode else 'range_scan' if replacement_front_bins_mode else 'not_consumed'}",
        "replacement_front_size_class_request_ceiling="
        f"{ctx['replacement_front_size_class_request_ceiling']}",
        "replacement_front_size_class_selected_bin="
        f"{ctx['replacement_front_size_class_selected_bin']}",
        "replacement_front_size_class_selected_good_size="
        f"{ctx['replacement_front_size_class_selected_good_size']}",
        "replacement_front_product_bins_plan_v0=1",
        "replacement_front_product_bins_report_only=1",
        "replacement_front_product_bins_consumer_enabled="
        f"{1 if replacement_front_bins_mode else 0}",
        "replacement_front_product_bins_connected=0",
        f"replacement_front_product_bins_route={ctx['replacement_front_product_bins_route']}",
        "replacement_front_product_pages_plan_v0=1",
        "replacement_front_product_pages_report_only=1",
        "replacement_front_product_pages_consumer_enabled="
        f"{ctx['replacement_front_product_pages_consumer_enabled']}",
        "replacement_front_benchmark_product_pages_consumer_enabled="
        f"{ctx['replacement_front_product_pages_consumer_enabled']}",
        "replacement_front_product_pages_connected=0",
        "replacement_front_product_pages_product_connected=0",
        "replacement_front_product_pages_next_bridge=design_non_linear_product_pages_bridge",
        "replacement_front_product_pages_non_linear_lookup_plan_v0=1",
        "replacement_front_product_pages_linear_probe_closed=1",
        "replacement_front_product_pages_non_linear_lookup_strategy=range_decision_tree_or_indexed_page_table",
        "replacement_front_product_pages_non_linear_lookup_selected="
        f"{ctx['replacement_front_product_pages_non_linear_lookup_selected']}",
        "replacement_front_product_pages_non_linear_next_bridge=replacement_front_product_pages_non_linear_plan",
        f"replacement_front_product_pages_route={ctx['replacement_front_product_pages_route']}",
        "replacement_front_benchmark_product_pages_route="
        f"{ctx['replacement_front_product_pages_route']}",
        "replacement_front_page_bins_plan_v0=1",
        "replacement_front_page_bins_report_only=1",
        "replacement_front_page_bins_consumer_enabled="
        f"{1 if args.replacement_front_page_bins_mode else 0}",
        f"replacement_front_page_bins_route={ctx['replacement_front_page_bins_route']}",
        f"replacement_front_page_bins_lookup_route={ctx['replacement_front_page_bins_lookup_route']}",
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
        f"replacement_front_hotcore_route={hotcore_route}",
        "hako_mimalloc_algorithm_claim=0",
        f"replacement_front_lock_mode={1 if args.replacement_front_lock_mode else 0}",
        f"replacement_front_thread_local_mode={1 if args.replacement_front_thread_local_mode else 0}",
        "replacement_front_thread_local_page_bins_mode="
        f"{1 if args.replacement_front_tls_page_arena_mode else 0}",
        "replacement_front_thread_local_hotcore_route="
        f"{'benchmark_page_bins_hotcore_tls' if args.replacement_front_tls_page_arena_mode else 'not_consumed'}",
        "replacement_front_page_from_ptr_bridge_mode="
        f"{1 if args.replacement_front_page_from_ptr_bridge_mode else 0}",
        f"replacement_front_page_from_ptr_route={page_from_ptr_route}",
        f"replacement_front_remote_free_route={remote_free_route}",
        "replacement_front_global_lock_hot_path_expected="
        f"{global_lock_hot_path_expected}",
        "mimalloc_fidelity_guard=1",
        "mimalloc_fidelity_guard_passed=0",
        f"replacement_front_evidence_owner={ctx['replacement_front_evidence_owner']}",
        "replacement_front_multithread_perf_candidate="
        f"{ctx['replacement_front_multithread_perf_candidate']}",
        "replacement_front_thread_local_perf_candidate="
        f"{ctx['replacement_front_thread_local_perf_candidate']}",
        f"replacement_front_correctness_smoke={ctx['replacement_front_correctness_smoke']}",
        f"replacement_front_cross_thread_smoke={1 if args.replacement_front_cross_thread_smoke else 0}",
        f"replacement_front_skip_hot_counters={1 if args.replacement_front_skip_hot_counters else 0}",
        f"replacement_front_tls_counter_mode={1 if args.replacement_front_tls_counter_mode else 0}",
        f"replacement_front_slot_size={ctx['replacement_slot_size']}",
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
