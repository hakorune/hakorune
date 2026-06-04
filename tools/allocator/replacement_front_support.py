"""Shared Python helpers for benchmark replacement-front reporting.

This module owns the small deterministic math and classification helpers used
by replacement-front compare/report scripts. It intentionally avoids any C
template text or process execution logic.
"""

from __future__ import annotations


WORKLOAD_HISTOGRAM_MAX_TOTAL_ITERS = 1_000_000


def median_float(values: list[float]) -> float:
    ordered = sorted(values)
    return ordered[len(ordered) // 2]


def positive_int(value: int, label: str) -> None:
    if value < 1:
        raise SystemExit(f"{label} must be positive")


def counter_value(counters: dict[str, object], key: str) -> int:
    value = counters.get(key, "0")
    if isinstance(value, int):
        return value
    if isinstance(value, str) and value.isdigit():
        return int(value)
    return 0


def lcg_next(value: int) -> int:
    return ((value * 1664525) + 1013904223) & 0xFFFFFFFF


def mixed_ws_pick_size(value: int, min_size: int, max_size: int) -> int:
    span = (max_size - min_size + 1) if max_size > min_size else 1
    return min_size + (value % span)


def size_bucket(size: int) -> str:
    if size <= 64:
        return "le_64"
    if size <= 128:
        return "le_128"
    if size <= 256:
        return "le_256"
    if size <= 512:
        return "le_512"
    if size <= 1024:
        return "le_1024"
    return "gt_1024"


def hako_size_class_bin_size(bin_index: int) -> int:
    """Mirror SizeClassBox.bin_size for report-only workload classification."""
    word_size = 8
    max_regular_bin = 72
    if bin_index <= 0:
        return -1
    if bin_index <= 8:
        return bin_index * word_size
    if bin_index > max_regular_bin:
        return -1

    x = bin_index + 3
    bit_group = x // 4
    top = x - (bit_group * 4)
    scale = 1 << max(0, bit_group - 2)
    words = (5 + top) * scale
    return words * word_size


def hako_size_to_bin(size: int) -> int:
    """Mirror SizeClassBox.size_to_bin for report-only workload classification."""
    max_regular_bin = 72
    huge_bin = 73
    n = size if size > 0 else 1
    for bin_index in range(1, max_regular_bin + 1):
        if n <= hako_size_class_bin_size(bin_index):
            return bin_index
    return huge_bin


def hako_good_size(size: int) -> int:
    """Mirror SizeClassBox.good_size for benchmark-only size-class bridging."""
    bin_index = hako_size_to_bin(size)
    if bin_index == 73:
        return -1
    return hako_size_class_bin_size(bin_index)


def mixed_ws_workload_histogram(
    *,
    threads: int,
    iters_per_thread: int,
    working_set: int,
    min_size: int,
    max_size: int,
    replacement_slot_size: int,
) -> dict[str, int | str]:
    sampled_iters_per_thread = iters_per_thread
    if threads * iters_per_thread > WORKLOAD_HISTOGRAM_MAX_TOTAL_ITERS:
        sampled_iters_per_thread = max(1, WORKLOAD_HISTOGRAM_MAX_TOTAL_ITERS // threads)
    exact = sampled_iters_per_thread == iters_per_thread

    buckets = {
        "le_64": 0,
        "le_128": 0,
        "le_256": 0,
        "le_512": 0,
        "le_1024": 0,
        "gt_1024": 0,
    }
    alloc_requests = 0
    free_path_count = 0
    cleanup_free_count = 0
    realloc_requests = 0
    realloc_gt_slot = 0
    realloc_gt_max_size = 0
    memset_le_64_count = 0
    memset_gt_64_count = 0
    size_class_counts: dict[int, int] = {}

    ws = working_set if working_set > 0 else 1

    def record_size_class(request_size: int) -> None:
        bin_index = hako_size_to_bin(request_size)
        size_class_counts[bin_index] = size_class_counts.get(bin_index, 0) + 1

    for thread_index in range(threads):
        seed = 1234 + thread_index
        slots = [False] * ws
        for iteration in range(sampled_iters_per_thread):
            seed = lcg_next(seed)
            idx = seed % ws
            if slots[idx]:
                free_path_count += 1
                slots[idx] = False
                continue

            size = mixed_ws_pick_size(seed, min_size, max_size)
            alloc_requests += 1
            buckets[size_bucket(size)] += 1
            record_size_class(size)
            if (iteration & 0x3F) == 0:
                new_size = size + 16
                realloc_requests += 1
                buckets[size_bucket(new_size)] += 1
                record_size_class(new_size)
                if new_size > replacement_slot_size:
                    realloc_gt_slot += 1
                if new_size > max_size:
                    realloc_gt_max_size += 1
                size = new_size
            if size < 64:
                memset_le_64_count += 1
            else:
                memset_gt_64_count += 1
            slots[idx] = True
        cleanup_free_count += sum(1 for occupied in slots if occupied)

    regular_bins = [bin_index for bin_index in size_class_counts if bin_index != 73]
    regular_bins_sorted = sorted(regular_bins)
    max_bin = max(size_class_counts) if size_class_counts else 0
    max_regular_seen = max(regular_bins) if regular_bins else 0

    return {
        "source": "deterministic_prefix_exact" if exact else "deterministic_prefix_sampled",
        "sampled_iters_per_thread": sampled_iters_per_thread,
        "sampled_total_iterations": sampled_iters_per_thread * threads,
        "full_total_iterations": iters_per_thread * threads,
        "sample_exact": 1 if exact else 0,
        "alloc_request_count": alloc_requests,
        "free_path_count": free_path_count,
        "cleanup_free_count": cleanup_free_count,
        "realloc_request_count": realloc_requests,
        "realloc_request_gt_replacement_slot_size": realloc_gt_slot,
        "realloc_request_gt_max_size": realloc_gt_max_size,
        "memset_le_64_count": memset_le_64_count,
        "memset_gt_64_count": memset_gt_64_count,
        "size_class_policy_source": "hako_size_class_box_report_mirror",
        "size_class_distinct_count": len(size_class_counts),
        "size_class_regular_distinct_count": len(regular_bins_sorted),
        "size_class_regular_bins": ",".join(str(bin_index) for bin_index in regular_bins_sorted)
        or "none",
        "size_class_max_bin": max_bin,
        "size_class_max_good_size": hako_size_class_bin_size(max_regular_seen),
        "size_class_huge_count": size_class_counts.get(73, 0),
        "size_class_regular_request_count": sum(
            count for bin_index, count in size_class_counts.items() if bin_index != 73
        ),
        **{f"request_{bucket}": count for bucket, count in buckets.items()},
    }
