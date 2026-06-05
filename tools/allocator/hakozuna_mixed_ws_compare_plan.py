"""Argument validation and derived plan for the Hakozuna mixed-ws compare tool."""

from __future__ import annotations

import argparse
from dataclasses import dataclass

from replacement_front_support import (
    hako_good_size,
    hako_size_to_bin,
    mixed_ws_workload_histogram,
    positive_int,
)


@dataclass(frozen=True)
class HakozunaMixedWsComparePlan:
    replacement_front_bins_mode: bool
    replacement_slot_size: int
    workload_histogram: dict[str, int | str]
    required_regular_bins: list[int]
    replacement_front_size_class_request_ceiling: int
    replacement_front_size_class_selected_bin: int
    replacement_front_size_class_selected_good_size: int


def build_compare_plan(args: argparse.Namespace) -> HakozunaMixedWsComparePlan:
    positive_int(args.sample_count, "--sample-count")
    if args.warmup_count < 0:
        raise SystemExit("--warmup-count must be non-negative")
    if args.min_sample_seconds < 0.0:
        raise SystemExit("--min-sample-seconds must be non-negative")
    positive_int(args.threads, "--threads")
    positive_int(args.iters_per_thread, "--iters-per-thread")
    positive_int(args.working_set, "--working-set")
    positive_int(args.min_size, "--min-size")
    positive_int(args.max_size, "--max-size")
    if args.max_size < args.min_size:
        raise SystemExit("--max-size must be >= --min-size")

    replacement_shape_modes = sum(
        1
        for enabled in (
            args.replacement_front_native_slot_mode,
            args.replacement_front_native_bins_mode,
            args.replacement_front_page_bins_mode,
        )
        if enabled
    )
    if replacement_shape_modes > 1:
        raise SystemExit(
            "--replacement-front-native-slot-mode, "
            "--replacement-front-native-bins-mode, and "
            "--replacement-front-page-bins-mode are exclusive"
        )

    match_modes = sum(
        1
        for enabled in (
            args.replacement_front_slot_size is not None,
            args.replacement_front_match_workload_realloc_size,
            args.replacement_front_match_hako_size_class,
        )
        if enabled
    )
    if match_modes > 1:
        raise SystemExit(
            "--replacement-front-slot-size, "
            "--replacement-front-match-workload-realloc-size, and "
            "--replacement-front-match-hako-size-class are mutually exclusive"
        )

    replacement_front_size_class_request_ceiling = args.max_size + 16
    replacement_front_size_class_selected_bin = hako_size_to_bin(
        replacement_front_size_class_request_ceiling
    )
    replacement_front_size_class_selected_good_size = hako_good_size(
        replacement_front_size_class_request_ceiling
    )

    replacement_front_slot_size = (
        2048 if args.replacement_front_slot_size is None else args.replacement_front_slot_size
    )
    if args.replacement_front_match_workload_realloc_size:
        replacement_front_slot_size = args.max_size + 16
    if args.replacement_front_match_hako_size_class:
        if replacement_front_size_class_selected_good_size <= 0:
            raise SystemExit(
                "--replacement-front-match-hako-size-class selected huge bin; "
                "use --replacement-front-slot-size explicitly for this workload"
            )
        replacement_front_slot_size = replacement_front_size_class_selected_good_size
    positive_int(replacement_front_slot_size, "--replacement-front-slot-size")
    if replacement_front_slot_size < args.max_size:
        raise SystemExit("--replacement-front-slot-size must be >= --max-size")

    if args.provider_assume_owned_mode and not args.provider_usable_size_mode:
        raise SystemExit("--provider-assume-owned-mode requires --provider-usable-size-mode")
    if args.replacement_front_lock_mode and not (
        args.replacement_front_native_slot_mode
        or args.replacement_front_native_bins_mode
        or args.replacement_front_page_bins_mode
    ):
        raise SystemExit(
            "--replacement-front-lock-mode requires a replacement-front mode"
        )
    if args.replacement_front_thread_local_mode and not args.replacement_front_native_slot_mode:
        raise SystemExit(
            "--replacement-front-thread-local-mode requires --replacement-front-native-slot-mode"
        )
    if args.replacement_front_lock_mode and args.replacement_front_thread_local_mode:
        raise SystemExit(
            "--replacement-front-lock-mode and --replacement-front-thread-local-mode are exclusive"
        )
    if args.replacement_front_cross_thread_smoke and not args.replacement_front_thread_local_mode:
        raise SystemExit(
            "--replacement-front-cross-thread-smoke requires "
            "--replacement-front-thread-local-mode"
        )
    if args.replacement_front_skip_hot_counters and not (
        args.replacement_front_native_slot_mode
        or args.replacement_front_native_bins_mode
        or args.replacement_front_page_bins_mode
    ):
        raise SystemExit(
            "--replacement-front-skip-hot-counters requires "
            "a replacement-front mode"
        )
    if args.replacement_front_tls_counter_mode and not args.replacement_front_thread_local_mode:
        raise SystemExit(
            "--replacement-front-tls-counter-mode requires "
            "--replacement-front-thread-local-mode"
        )
    if args.replacement_front_tls_counter_mode and args.replacement_front_skip_hot_counters:
        raise SystemExit(
            "--replacement-front-tls-counter-mode and "
            "--replacement-front-skip-hot-counters are exclusive"
        )
    if args.replacement_front_slot_size is not None and not args.replacement_front_native_slot_mode:
        raise SystemExit(
            "--replacement-front-slot-size requires --replacement-front-native-slot-mode"
        )
    if (
        args.replacement_front_match_workload_realloc_size
        and not args.replacement_front_native_slot_mode
    ):
        raise SystemExit(
            "--replacement-front-match-workload-realloc-size requires "
            "--replacement-front-native-slot-mode"
        )
    if (
        args.replacement_front_match_hako_size_class
        and not args.replacement_front_native_slot_mode
    ):
        raise SystemExit(
            "--replacement-front-match-hako-size-class requires "
            "--replacement-front-native-slot-mode"
        )
    if args.replacement_front_cross_thread_smoke and args.replacement_front_skip_hot_counters:
        raise SystemExit(
            "--replacement-front-cross-thread-smoke cannot be combined with "
            "--replacement-front-skip-hot-counters because the smoke validates counters"
        )
    if (
        args.replacement_front_hotcore_page_model_mode
        and not args.replacement_front_page_bins_mode
    ):
        raise SystemExit(
            "--replacement-front-hotcore-page-model-mode requires "
            "--replacement-front-page-bins-mode"
        )
    if args.replacement_front_tls_page_arena_mode and not (
        args.replacement_front_page_bins_mode
        and args.replacement_front_hotcore_page_model_mode
    ):
        raise SystemExit(
            "--replacement-front-tls-page-arena-mode requires "
            "--replacement-front-page-bins-mode and "
            "--replacement-front-hotcore-page-model-mode"
        )
    if args.replacement_front_tls_page_arena_mode and args.replacement_front_lock_mode:
        raise SystemExit(
            "--replacement-front-tls-page-arena-mode and "
            "--replacement-front-lock-mode are exclusive"
        )
    if (
        args.replacement_front_tls_page_arena_mode
        and args.replacement_front_product_pages_nonlinear_mode
    ):
        raise SystemExit(
            "--replacement-front-tls-page-arena-mode cannot be combined with "
            "--replacement-front-product-pages-nonlinear-mode in this slice"
        )
    if (
        args.replacement_front_product_pages_nonlinear_mode
        and not args.replacement_front_page_bins_mode
    ):
        raise SystemExit(
            "--replacement-front-product-pages-nonlinear-mode requires "
            "--replacement-front-page-bins-mode"
        )
    if args.replacement_front_size_class_table_mode and not (
        args.replacement_front_native_bins_mode or args.replacement_front_page_bins_mode
    ):
        raise SystemExit(
            "--replacement-front-size-class-table-mode requires "
            "--replacement-front-native-bins-mode or --replacement-front-page-bins-mode"
        )
    if args.replacement_front_eager_init_mode and not (
        args.replacement_front_native_bins_mode or args.replacement_front_page_bins_mode
    ):
        raise SystemExit(
            "--replacement-front-eager-init-mode requires "
            "--replacement-front-native-bins-mode or --replacement-front-page-bins-mode"
        )

    replacement_front_bins_mode = (
        args.replacement_front_native_bins_mode or args.replacement_front_page_bins_mode
    )
    if replacement_front_bins_mode:
        if (
            args.threads != 1
            and not args.replacement_front_lock_mode
            and not args.replacement_front_tls_page_arena_mode
        ):
            raise SystemExit(
                "--replacement-front-native-bins-mode and "
                "--replacement-front-page-bins-mode require "
                "--replacement-front-lock-mode or "
                "--replacement-front-tls-page-arena-mode when --threads > 1"
            )
        if (
            args.replacement_front_thread_local_mode
            or args.replacement_front_cross_thread_smoke
            or args.replacement_front_tls_counter_mode
            or args.replacement_front_slot_size is not None
        ):
            raise SystemExit(
                "--replacement-front-native-bins-mode and "
                "--replacement-front-page-bins-mode cannot be combined with "
                "slot/thread-local/counter replacement-front modifiers in v0; "
                "use --replacement-front-tls-page-arena-mode for the page-bins "
                "TLS arena route"
            )

    workload_histogram = mixed_ws_workload_histogram(
        threads=args.threads,
        iters_per_thread=args.iters_per_thread,
        working_set=args.working_set,
        min_size=args.min_size,
        max_size=args.max_size,
        replacement_slot_size=replacement_front_slot_size,
    )
    required_regular_bins = [
        int(part)
        for part in str(workload_histogram["size_class_regular_bins"]).split(",")
        if part and part != "none"
    ]
    return HakozunaMixedWsComparePlan(
        replacement_front_bins_mode=replacement_front_bins_mode,
        replacement_slot_size=replacement_front_slot_size,
        workload_histogram=workload_histogram,
        required_regular_bins=required_regular_bins,
        replacement_front_size_class_request_ceiling=replacement_front_size_class_request_ceiling,
        replacement_front_size_class_selected_bin=replacement_front_size_class_selected_bin,
        replacement_front_size_class_selected_good_size=replacement_front_size_class_selected_good_size,
    )
