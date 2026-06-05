"""Shared route labels for benchmark replacement-front reports.

This module is control-plane only. It must not decide allocator execution or
product activation; runners and generated C own those paths.
"""

from __future__ import annotations

from typing import Any


def has_thread_local_arena(args: Any) -> bool:
    return bool(args.replacement_front_thread_local_mode or args.replacement_front_tls_page_arena_mode)


def has_multithread_safe_route(args: Any) -> bool:
    return bool(args.replacement_front_lock_mode or has_thread_local_arena(args))


def hotcore_route(args: Any) -> str:
    if args.replacement_front_tls_page_arena_mode:
        return "benchmark_page_bins_hotcore_tls"
    if args.replacement_front_hotcore_page_model_mode:
        return "benchmark_page_bins_hotcore_page_model"
    return "not_consumed_by_replacement_front"


def thread_local_hotcore_route(args: Any) -> str:
    return "benchmark_page_bins_hotcore_tls" if args.replacement_front_tls_page_arena_mode else "not_consumed"


def remote_free_route(args: Any) -> str:
    if args.replacement_front_remote_free_queue_mode:
        return "atomic_page_remote_head"
    if args.replacement_front_tls_page_arena_mode:
        return "disabled"
    if args.replacement_front_thread_local_mode:
        return "remote_queue"
    return "global_lock_or_not_applicable"


def global_lock_hot_path_expected(args: Any) -> int | str:
    if args.replacement_front_tls_page_arena_mode:
        return 0
    if args.replacement_front_lock_mode:
        return "lock_enter_count"
    return 0


def page_from_ptr_route(args: Any, *, replacement_front_bins_mode: bool) -> str:
    if args.replacement_front_page_from_ptr_bridge_mode:
        return "side_table_direct"
    if args.replacement_front_product_pages_nonlinear_mode:
        return "indexed_page_table"
    if replacement_front_bins_mode:
        return "range_scan"
    return "not_consumed"


def page_bins_lookup_route(args: Any) -> str:
    if args.replacement_front_page_from_ptr_bridge_mode:
        return "page_from_ptr_bridge"
    if args.replacement_front_product_pages_nonlinear_mode:
        return "indexed_page_table"
    if args.replacement_front_page_bins_mode:
        return "range_scan"
    return "not_consumed"


def product_bins_route(args: Any) -> str:
    if args.replacement_front_tls_page_arena_mode:
        return "benchmark_page_bins_hotcore_tls"
    if args.replacement_front_hotcore_page_model_mode:
        return "benchmark_page_bins_hotcore_page_model"
    if args.replacement_front_page_bins_mode:
        return "benchmark_page_bins"
    if args.replacement_front_native_bins_mode:
        return "benchmark_native_bins"
    return "not_consumed"


def page_bins_route(args: Any) -> str:
    if args.replacement_front_tls_page_arena_mode:
        return "benchmark_page_bins_hotcore_tls"
    if args.replacement_front_hotcore_page_model_mode:
        return "benchmark_page_bins_hotcore_page_model"
    if args.replacement_front_page_bins_mode:
        return "benchmark_page_bins"
    return "not_consumed"


def algorithm_shape(args: Any) -> str:
    if (
        args.replacement_front_tls_page_arena_mode
        and args.replacement_front_page_from_ptr_bridge_mode
        and args.replacement_front_remote_free_queue_mode
    ):
        return "page_bin_hotcore_tls_page_arena_page_from_ptr_remote_free_benchmark_front"
    if (
        args.replacement_front_tls_page_arena_mode
        and args.replacement_front_page_from_ptr_bridge_mode
    ):
        return "page_bin_hotcore_tls_page_arena_page_from_ptr_benchmark_front"
    if args.replacement_front_tls_page_arena_mode:
        return "page_bin_hotcore_tls_page_arena_benchmark_front"
    if (
        args.replacement_front_product_pages_nonlinear_mode
        and args.replacement_front_hotcore_page_model_mode
    ):
        return "page_bin_hotcore_page_model_product_pages_nonlinear_benchmark_front"
    if args.replacement_front_product_pages_nonlinear_mode:
        return "page_bin_product_pages_nonlinear_benchmark_front"
    if args.replacement_front_hotcore_page_model_mode:
        return "page_bin_hotcore_page_model_benchmark_front"
    if args.replacement_front_page_bins_mode:
        return "page_bin_benchmark_front"
    if args.replacement_front_native_bins_mode:
        return "multi_bin_native_benchmark_front"
    return "fixed_slot_native_benchmark_front"
