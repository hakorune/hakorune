#!/usr/bin/env python3
"""Normalize C mimalloc and hako EXE memory evidence into one report."""

from __future__ import annotations

import argparse
from pathlib import Path


def read_kv(path: Path) -> dict[str, str]:
    values: dict[str, str] = {}
    for line in path.read_text(encoding="utf-8", errors="replace").splitlines():
        line = line.strip()
        if not line or line.startswith("#") or "=" not in line:
            continue
        key, value = line.split("=", 1)
        values[key] = value
    return values


def as_int(values: dict[str, str], key: str) -> int:
    text = values.get(key, "0")
    try:
        return int(text)
    except ValueError as exc:
        raise SystemExit(f"{key} must be an integer, got {text!r}") from exc


def require(values: dict[str, str], key: str, expected: str, label: str) -> None:
    actual = values.get(key)
    if actual != expected:
        raise SystemExit(f"{label}: {key} expected {expected!r}, got {actual!r}")


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Build a stable hako-vs-C mimalloc memory evidence report."
    )
    parser.add_argument("--hako", required=True, type=Path)
    parser.add_argument("--c", required=True, type=Path, dest="c_path")
    parser.add_argument("--out", required=True, type=Path)
    args = parser.parse_args()

    hako = read_kv(args.hako)
    c = read_kv(args.c_path)

    require(hako, "output_contract", "hako-exe-memory-evidence-v0", "hako")
    require(c, "output_contract", "allocator-comparison-c-mimalloc-explicit-runner-v0", "c")
    require(hako, "summary", "ok", "hako")
    require(c, "summary", "ok", "c")
    require(hako, "provider_activation", "0", "hako")
    require(hako, "host_replacement", "0", "hako")
    require(hako, "hook_installed", "0", "hako")
    require(hako, "global_allocator_installed", "0", "hako")
    require(c, "process_replacement_executed", "0", "c")
    require(c, "hook_installed", "0", "c")
    require(c, "backend_matcher_added", "0", "c")
    require(c, "global_allocator_installed", "0", "c")
    require(c, "hidden_discovery_used", "0", "c")
    require(c, "provider_package_generated", "0", "c")

    hako_requested = as_int(hako, "requested_bytes")
    c_requested = as_int(c, "requested_bytes")
    hako_allocation_count = as_int(hako, "allocation_count")
    c_allocation_count = as_int(c, "allocation_count")
    hako_free_count = as_int(hako, "free_count")
    c_free_count = as_int(c, "free_count")
    hako_peak = as_int(hako, "peak_rss_bytes")
    c_peak = as_int(c, "peak_rss_bytes")

    hako_workload = hako.get("workload", "")
    c_workload = c.get("workload", "")
    workload_match = 1 if hako_workload == c_workload else 0
    hako_operation_family = hako.get("operation_family", "")
    c_operation_family = c.get("operation_family", "")
    operation_family_match = 1 if hako_operation_family == c_operation_family else 0
    hako_operation_sequence_id = hako.get("operation_sequence_id", "")
    c_operation_sequence_id = c.get("operation_sequence_id", "")
    operation_sequence_match = 1 if hako_operation_sequence_id == c_operation_sequence_id else 0
    hako_free_order_id = hako.get("free_order_id", "")
    c_free_order_id = c.get("free_order_id", "")
    free_order_match = 1 if hako_free_order_id == c_free_order_id else 0
    hako_realloc_count = as_int(hako, "realloc_count")
    c_realloc_count = as_int(c, "realloc_count")
    hako_aligned_alloc_count = as_int(hako, "aligned_alloc_count")
    c_aligned_alloc_count = as_int(c, "aligned_alloc_count")
    hako_alignment_request_count = as_int(hako, "alignment_request_count")
    c_alignment_request_count = as_int(c, "alignment_request_count")
    hako_alignment_ok_count = as_int(hako, "alignment_ok_count")
    c_alignment_ok_count = as_int(c, "alignment_ok_count")
    hako_alignment_reject_count = as_int(hako, "alignment_reject_count")
    c_alignment_reject_count = as_int(c, "alignment_reject_count")

    lines = [
        "mimalloc_comparison_memory_report=1",
        "output_contract=mimalloc-comparison-memory-report-v0",
        f"hako_workload={hako_workload}",
        f"c_workload={c_workload}",
        f"workload_match={workload_match}",
        f"hako_operation_family={hako_operation_family}",
        f"c_operation_family={c_operation_family}",
        f"operation_family_match={operation_family_match}",
        f"hako_operation_sequence_id={hako_operation_sequence_id}",
        f"c_operation_sequence_id={c_operation_sequence_id}",
        f"operation_sequence_match={operation_sequence_match}",
        f"hako_free_order_id={hako_free_order_id}",
        f"c_free_order_id={c_free_order_id}",
        f"free_order_match={free_order_match}",
        f"hako_result_code={as_int(hako, 'result_code')}",
        f"c_result_code={as_int(c, 'result_code')}",
        f"hako_run_count={as_int(hako, 'run_count')}",
        f"c_run_count={as_int(c, 'run_count')}",
        f"hako_allocation_count={hako_allocation_count}",
        f"c_allocation_count={c_allocation_count}",
        f"allocation_count_delta={hako_allocation_count - c_allocation_count}",
        f"hako_free_count={hako_free_count}",
        f"c_free_count={c_free_count}",
        f"free_count_delta={hako_free_count - c_free_count}",
        f"hako_realloc_count={hako_realloc_count}",
        f"c_realloc_count={c_realloc_count}",
        f"realloc_count_delta={hako_realloc_count - c_realloc_count}",
        f"hako_aligned_alloc_count={hako_aligned_alloc_count}",
        f"c_aligned_alloc_count={c_aligned_alloc_count}",
        f"aligned_alloc_count_delta={hako_aligned_alloc_count - c_aligned_alloc_count}",
        f"hako_alignment_request_count={hako_alignment_request_count}",
        f"c_alignment_request_count={c_alignment_request_count}",
        f"alignment_request_count_delta={hako_alignment_request_count - c_alignment_request_count}",
        f"hako_alignment_ok_count={hako_alignment_ok_count}",
        f"c_alignment_ok_count={c_alignment_ok_count}",
        f"alignment_ok_count_delta={hako_alignment_ok_count - c_alignment_ok_count}",
        f"hako_alignment_reject_count={hako_alignment_reject_count}",
        f"c_alignment_reject_count={c_alignment_reject_count}",
        f"alignment_reject_count_delta={hako_alignment_reject_count - c_alignment_reject_count}",
        f"hako_realloc_same_ptr_count={as_int(hako, 'realloc_same_ptr_count')}",
        f"c_realloc_same_ptr_count={as_int(c, 'realloc_same_ptr_count')}",
        f"hako_realloc_moved_count={as_int(hako, 'realloc_moved_count')}",
        f"c_realloc_moved_count={as_int(c, 'realloc_moved_count')}",
        f"hako_copied_bytes={as_int(hako, 'copied_bytes')}",
        f"c_copied_bytes={as_int(c, 'copied_bytes')}",
        f"hako_requested_bytes={hako_requested}",
        f"c_requested_bytes={c_requested}",
        f"hako_peak_rss_bytes={hako_peak}",
        f"c_peak_rss_bytes={c_peak}",
        f"requested_bytes_delta={hako_requested - c_requested}",
        f"peak_rss_bytes_delta={hako_peak - c_peak}",
        f"hako_memory_usage_evidence={as_int(hako, 'memory_usage_evidence')}",
        f"c_memory_usage_evidence={as_int(c, 'memory_usage_evidence')}",
        f"hako_output_summary_ok={as_int(hako, 'output_summary_ok')}",
        "provider_activation=0",
        "host_replacement=0",
        "hook_installed=0",
        "global_allocator_installed=0",
        "winner_claim=0",
        "summary=ok",
    ]
    args.out.write_text("\n".join(lines) + "\n", encoding="utf-8")
    print(args.out.read_text(encoding="utf-8"), end="")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
