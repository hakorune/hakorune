#!/usr/bin/env python3
"""Inventory FastMemory capability coverage from benchmark reports.

This adapter is observation-only. It reads existing replacement-front report
key/value files and reports whether fastmem/capability surfaces are present.
It does not run benchmarks, rewrite source, choose keepers, or activate
allocator replacement.
"""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from fastmem_capability_inventory_common import (
    MIMALLOC_COVERAGE_DEFAULT_THRESHOLD,
    MIMALLOC_SAFETY_DEFAULT_THRESHOLD,
    MIMALLOC_SHAPE_COMPONENT_POINTS,
    MIMALLOC_SHAPE_DEFAULT_THRESHOLD,
    PAGE_META_FIELDS,
    add_count,
    analyze_expr,
    analyze_stmt,
    base_inventory,
    build_source_inventory,
    call_name,
    child_expr,
    classify_remote_memory_order,
    contract_family,
    emit_summary,
    first_subject_value,
    int_route_flag,
    int_subject_value,
    iter_fastmem_regions,
    iter_nodes,
    is_mem_method_call,
    is_node,
    route_value,
    speed_score_from_ratio,
    typed_page_meta_fields,
    write_output,
)
from report_kv import first_value, int_value, prefixed, read_kv
from replacement_front_report import (
    build_report as build_replacement_report,
    emit_kv,
    format_value,
    page_lookup_route,
    page_map_bridge_kind,
)
from fastmem_capability_inventory_mir import build_mir_metadata_inventory
from fastmem_capability_inventory_report import build_inventory_report

def build_inventory(rows: dict[str, str]) -> dict[str, Any]:
    replacement = build_replacement_report(rows, None)
    idx = int(replacement["benchmark_subject_index"])

    def replacement_counter(suffix: str) -> int:
        return int_route_flag(rows, replacement, f"replacement_front_{suffix}")

    free_path_route = page_lookup_route(rows, idx, replacement)
    bridge_kind = page_map_bridge_kind(rows, idx)
    remote_route = route_value(rows, idx, "replacement_front_remote_free_route")

    allocator_tls_enabled = int(
        int_route_flag(rows, replacement, "replacement_front_thread_local_page_bins_mode") == 1
        or replacement["same_thread_alloc_local_count_total"] > 0
        or replacement["same_thread_free_local_count_total"] > 0
    )
    owner_shadow_counters = int(replacement["owner_thread_id_lookup_count_total"] > 0)
    same_owner_route_enabled = int_subject_value(
        rows,
        idx,
        "same_owner_free_local_route_enabled",
        int_subject_value(rows, idx, "replacement_front_same_owner_local_free_route_enabled", 0),
    )
    smoke_remote_push_count = int_value(
        rows, ["replacement_front_cross_thread_free_remote_free_push_count"], 0
    )
    smoke_remote_drain_count = int_value(
        rows, ["replacement_front_cross_thread_free_remote_free_drain_count"], 0
    )
    smoke_remote_overflow_count = int_value(
        rows, ["replacement_front_cross_thread_free_arena_registry_overflow_count"], 0
    )
    remote_free_push_count = max(
        replacement["remote_free_push_count_total"], smoke_remote_push_count
    )
    remote_free_drain_count = max(
        replacement["remote_free_drain_count_total"], smoke_remote_drain_count
    )
    atomic_remote_head_plan = int_subject_value(
        rows,
        idx,
        "atomic_remote_head_plan",
        int_subject_value(rows, idx, "replacement_front_remote_free_queue_plan_v0", 0),
    )
    atomic_remote_enabled = int(
        remote_route == "atomic_page_remote_head"
        or remote_free_push_count > 0
        or remote_free_drain_count > 0
    )
    page_map_bridge_present = int(free_path_route == "page_map_bridge")
    typed_meta_handle = int_subject_value(rows, idx, "typed_page_meta_handle", 0)
    typed_meta_fields = typed_page_meta_fields(rows, idx)
    typed_meta_field_count = int_subject_value(
        rows,
        idx,
        "typed_page_meta_field_count",
        sum(1 for present in typed_meta_fields.values() if present),
    )
    typed_meta_missing_count = (
        sum(1 for present in typed_meta_fields.values() if not present)
        if typed_meta_handle
        else 0
    )
    typed_meta_layout_verified = int_subject_value(
        rows,
        idx,
        "typed_page_meta_layout_verified",
        int_subject_value(rows, idx, "fastmem_layout_verified", 0),
    )
    typed_meta_layout_id = first_subject_value(
        rows,
        idx,
        "typed_page_meta_layout_id",
        first_subject_value(rows, idx, "fastmem_layout_id", "unknown"),
    )
    typed_meta_layout_hash = first_subject_value(
        rows,
        idx,
        "typed_page_meta_layout_hash",
        first_subject_value(rows, idx, "fastmem_layout_hash", "unknown"),
    )
    alloc_owner_id_capability = int_subject_value(
        rows, idx, "alloc_owner_id_capability", owner_shadow_counters
    )
    alloc_owner_id_kind = first_subject_value(
        rows,
        idx,
        "alloc_owner_id_kind",
        "allocator_arena_owner" if alloc_owner_id_capability else "unknown",
    )
    alloc_owner_id_source = first_subject_value(
        rows,
        idx,
        "alloc_owner_id_source",
        "benchmark_c_pthread_tls" if alloc_owner_id_capability else "unknown",
    )
    alloc_owner_id_width_bits = int_subject_value(
        rows, idx, "alloc_owner_id_width_bits", 64 if alloc_owner_id_capability else 0
    )
    replacement_owner_generation_enabled = replacement_counter(
        "allocator_owner_generation_enabled"
    )
    alloc_owner_id_generation_enabled = int_subject_value(
        rows,
        idx,
        "alloc_owner_id_generation_enabled",
        replacement_owner_generation_enabled,
    )
    alloc_owner_id_zero_is_unowned = int_subject_value(
        rows, idx, "alloc_owner_id_zero_is_unowned", 1
    )
    worker_id_capability = int_subject_value(
        rows, idx, "worker_id_capability", alloc_owner_id_capability
    )
    worker_id_kind = first_subject_value(
        rows,
        idx,
        "worker_id_kind",
        alloc_owner_id_kind,
    )
    worker_id_source = first_subject_value(
        rows,
        idx,
        "worker_id_source",
        alloc_owner_id_source,
    )
    tls_arena_count = int_route_flag(rows, replacement, "replacement_front_tls_arena_count")
    tls_arena_peak_reported = int_route_flag(
        rows, replacement, "replacement_front_tls_arena_peak_count"
    )
    tls_arena_init_count = int_subject_value(
        rows,
        idx,
        "allocator_tls_arena_init_count",
        tls_arena_count if tls_arena_count > 0 else allocator_tls_enabled,
    )
    tls_arena_live_count = int_subject_value(
        rows, idx, "allocator_tls_arena_live_count", tls_arena_count
    )
    tls_arena_peak_count = int_subject_value(
        rows,
        idx,
        "allocator_tls_arena_peak_count",
        tls_arena_peak_reported,
    )
    owner_same_count = replacement["owner_thread_id_same_count_total"]
    if owner_same_count == 0:
        owner_same_count = max(
            0,
            replacement["owner_thread_id_lookup_count_total"]
            - replacement["owner_thread_id_remote_count_total"],
        )
    page_owner_same_count = int_subject_value(
        rows,
        idx,
        "page_owner_same_count",
        owner_same_count,
    )
    page_owner_remote_count = int_subject_value(
        rows,
        idx,
        "page_owner_remote_count",
        replacement["owner_thread_id_remote_count_total"],
    )
    if page_owner_remote_count == 0 and remote_free_push_count > 0:
        page_owner_remote_count = remote_free_push_count
    page_owner_unowned_count = int_subject_value(rows, idx, "page_owner_unowned_count", 0)
    page_owner_stale_count = int_subject_value(
        rows, idx, "page_owner_stale_generation_count", 0
    )
    page_owner_invalid_count = int_subject_value(rows, idx, "page_owner_invalid_count", 0)
    page_owner_check_count = int_subject_value(
        rows,
        idx,
        "page_owner_check_count",
        page_owner_same_count
        + page_owner_remote_count
        + page_owner_unowned_count
        + page_owner_stale_count
        + page_owner_invalid_count,
    )
    page_owner_sum = (
        page_owner_same_count
        + page_owner_remote_count
        + page_owner_unowned_count
        + page_owner_stale_count
        + page_owner_invalid_count
    )
    same_owner_push_default = (
        min(page_owner_same_count, replacement["same_thread_free_local_count_total"])
        if same_owner_route_enabled
        else 0
    )
    same_owner_push_count = int_subject_value(
        rows, idx, "same_owner_free_local_push_count", same_owner_push_default
    )
    same_owner_fallback_default = (
        max(0, page_owner_same_count - same_owner_push_count)
        if same_owner_route_enabled
        else 0
    )
    remote_owner_candidate_default = page_owner_remote_count
    remote_owner_push_default = remote_free_push_count if atomic_remote_enabled else 0
    remote_owner_fallback_default = (
        max(0, remote_owner_candidate_default - remote_owner_push_default)
        if atomic_remote_enabled
        else 0
    )
    replacement_thread_exit_flush_count = max(
        replacement_counter("allocator_thread_exit_flush_count"),
        int_route_flag(rows, replacement, "replacement_front_thread_exit_arena_flush_count"),
    )
    allocator_thread_exit_flush_count = int_subject_value(
        rows,
        idx,
        "allocator_thread_exit_flush_count",
        replacement_thread_exit_flush_count,
    )
    replacement_abandoned_owner_count = max(
        replacement_counter("allocator_owner_abandoned_count"),
        int_route_flag(rows, replacement, "replacement_front_abandoned_owner_count"),
    )
    allocator_abandoned_owner_count = int_subject_value(
        rows,
        idx,
        "allocator_abandoned_owner_count",
        replacement_abandoned_owner_count,
    )
    allocator_owner_lifecycle_state_machine = int_subject_value(
        rows,
        idx,
        "allocator_owner_lifecycle_state_machine",
        replacement_counter("allocator_owner_lifecycle_state_machine"),
    )
    allocator_owner_generation_enabled = int_subject_value(
        rows,
        idx,
        "allocator_owner_generation_enabled",
        replacement_owner_generation_enabled or alloc_owner_id_generation_enabled,
    )
    allocator_owner_id_kind = first_subject_value(
        rows,
        idx,
        "allocator_owner_id_kind",
        "arena_owner" if alloc_owner_id_kind == "allocator_arena_owner" else "unknown",
    )
    allocator_owner_active_count = int_subject_value(
        rows,
        idx,
        "allocator_owner_active_count",
        replacement_counter("allocator_owner_active_count") or tls_arena_live_count,
    )
    allocator_owner_exiting_flush_count = int_subject_value(
        rows,
        idx,
        "allocator_owner_exiting_flush_count",
        replacement_counter("allocator_owner_exiting_flush_count"),
    )
    allocator_owner_abandoned_count = int_subject_value(
        rows,
        idx,
        "allocator_owner_abandoned_count",
        replacement_abandoned_owner_count or allocator_abandoned_owner_count,
    )
    allocator_owner_reclaimed_count = int_subject_value(
        rows,
        idx,
        "allocator_owner_reclaimed_count",
        replacement_counter("allocator_owner_reclaimed_count"),
    )
    remote_free_drain_supported = int_subject_value(
        rows,
        idx,
        "remote_free_drain_supported",
        replacement_counter("remote_free_drain_supported") or int(atomic_remote_enabled),
    )


    state = locals().copy()
    return build_inventory_report(state)
def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    source = parser.add_mutually_exclusive_group(required=True)
    source.add_argument("--report", type=Path, help="Read a benchmark key/value report.")
    source.add_argument("--ast-json", type=Path, help="Read Rust AST JSON containing FastMemRegion nodes.")
    source.add_argument(
        "--program-json",
        type=Path,
        help="Read Program(JSON v0) containing FastMemRegion nodes.",
    )
    source.add_argument(
        "--mir-json",
        type=Path,
        help="Read MIR JSON containing FastMemory metadata/access-plan rows.",
    )
    parser.add_argument("--format", choices=("kv", "summary", "json"), default="kv")
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    if args.report:
        rows = read_kv(args.report)
        report = build_inventory(rows)
    elif args.ast_json:
        report = build_source_inventory(
            json.loads(args.ast_json.read_text(encoding="utf-8")),
            "ast_json",
        )
    elif args.program_json:
        report = build_source_inventory(
            json.loads(args.program_json.read_text(encoding="utf-8")),
            "program_json_v0",
        )
    else:
        report = build_mir_metadata_inventory(
            json.loads(args.mir_json.read_text(encoding="utf-8"))
        )

    if args.format == "json":
        text = json.dumps(report, indent=2, sort_keys=True) + "\n"
    elif args.format == "summary":
        text = emit_summary(report)
    else:
        text = emit_kv(report)
    write_output(text, args.out)
    return 0 if report["summary"] == "ok" else 1


if __name__ == "__main__":
    raise SystemExit(main())
