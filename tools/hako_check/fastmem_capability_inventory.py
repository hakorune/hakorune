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

from replacement_front_report import (
    build_report as build_replacement_report,
    emit_kv,
    first_value,
    format_value,
    int_value,
    prefixed,
    read_kv,
)


def route_value(rows: dict[str, str], subject_idx: int, suffix: str, default: str = "") -> str:
    return prefixed(rows, subject_idx, suffix, default)


def classify_page_lookup_route(rows: dict[str, str], replacement: dict[str, Any]) -> str:
    idx = int(replacement["benchmark_subject_index"])
    lookup_route = route_value(rows, idx, "replacement_front_page_bins_lookup_route")
    page_from_ptr_route = route_value(rows, idx, "replacement_front_page_from_ptr_route")

    if lookup_route == "range_scan" or replacement["page_from_ptr_range_scan_count_total"] > 0:
        return "range_scan"
    if lookup_route in {"page_from_ptr_bridge", "indexed_page_table", "page_map_lookup"}:
        return "page_map_bridge"
    if page_from_ptr_route in {"side_table_direct", "page_base_mask", "header_backptr"}:
        return "page_map_bridge"
    if replacement["page_index_probe_count_total"] > 0:
        return "page_index_side_table"
    if replacement["page_from_ptr_count_total"] > 0:
        return "page_index_side_table"
    return "unknown"


def classify_page_map_bridge_kind(rows: dict[str, str], replacement: dict[str, Any]) -> str:
    idx = int(replacement["benchmark_subject_index"])
    page_from_ptr_route = route_value(rows, idx, "replacement_front_page_from_ptr_route")
    lookup_route = route_value(rows, idx, "replacement_front_page_bins_lookup_route")

    if page_from_ptr_route == "page_base_mask":
        return "page_base_mask"
    if page_from_ptr_route == "header_backptr":
        return "header_backptr"
    if page_from_ptr_route == "side_table_direct":
        return "flat_side_table"
    if lookup_route in {"indexed_page_table", "page_map_lookup", "page_from_ptr_bridge"}:
        return "flat_side_table"
    return "none"


def classify_remote_memory_order(rows: dict[str, str], replacement: dict[str, Any]) -> str:
    idx = int(replacement["benchmark_subject_index"])
    explicit = first_value(
        rows,
        [
            f"subject_{idx}_replacement_front_remote_free_memory_order",
            "replacement_front_remote_free_memory_order",
            f"subject_{idx}_remote_free_memory_order",
            "remote_free_memory_order",
        ],
    )
    if explicit:
        return explicit
    remote_route = route_value(rows, idx, "replacement_front_remote_free_route")
    if remote_route == "atomic_page_remote_head":
        return "acq_rel"
    return "missing"


def int_route_flag(rows: dict[str, str], replacement: dict[str, Any], suffix: str) -> int:
    idx = int(replacement["benchmark_subject_index"])
    return int_value(rows, [f"subject_{idx}_{suffix}", suffix], 0)


def build_inventory(rows: dict[str, str]) -> dict[str, Any]:
    replacement = build_replacement_report(rows, None)
    idx = int(replacement["benchmark_subject_index"])

    free_path_route = classify_page_lookup_route(rows, replacement)
    bridge_kind = classify_page_map_bridge_kind(rows, replacement)
    remote_route = route_value(rows, idx, "replacement_front_remote_free_route")

    allocator_tls_enabled = int(
        int_route_flag(rows, replacement, "replacement_front_thread_local_page_bins_mode") == 1
        or replacement["same_thread_alloc_local_count_total"] > 0
        or replacement["same_thread_free_local_count_total"] > 0
    )
    atomic_remote_enabled = int(
        remote_route == "atomic_page_remote_head"
        or replacement["remote_free_push_count_total"] > 0
        or replacement["remote_free_drain_count_total"] > 0
    )
    page_map_bridge_present = int(free_path_route == "page_map_bridge")

    shape_score = 0
    shape_score += 20 if allocator_tls_enabled else 0
    shape_score += 20 if page_map_bridge_present else 0
    shape_score += 20 if atomic_remote_enabled else 0
    shape_score += 20 if replacement["global_lock_hot_path_count_total"] == 0 else 0
    shape_score += 20 if replacement["replacement_front_is_full_hako_algorithm"] == 1 else 0

    report: dict[str, Any] = {
        "output_contract": "hako-check-fastmem-capability-inventory-v0",
        "input_kind": "benchmark_kv_report",
        "tool_surface": "hako_check_fastmem_capability_inventory",
        "observation_only": 1,
        "rewrite_executed": 0,
        "source_rewrite_executed": 0,
        "benchmark_run_executed": 0,
        "keeper_selection": 0,
        "provider_activation": 0,
        "hook_installed": 0,
        "global_allocator_product_claim": 0,
        "winner_claim": 0,
        "measured_hot_path_owner": replacement["measured_hot_path_owner"],
        "replacement_front_subowner": replacement["likely_next_owner"],
        "benchmark_subject_index": replacement["benchmark_subject_index"],
        "benchmark_front_class": replacement["benchmark_front_class"],
        "benchmark_threads": replacement["benchmark_threads"],
        "benchmark_thread_origin": replacement["benchmark_thread_origin"],
        "hako_hot_path_claim": replacement["hako_hot_path_claim"],
        "hako_source_thread_support_claim": replacement["hako_source_thread_support_claim"],
        "hako_source_hot_path_claim": 0,
        "mir_builder_hot_path_claim": 0,
        "type_abi_hot_path_lookup_count": replacement["type_abi_hot_path_lookup_count"],
        "provider_dispatch_hot_path": replacement["provider_dispatch_hot_path"],
        "fastmem_region_count": 0,
        "fastmem_contract_count": 0,
        "fastmem_contract_id": "unknown",
        "fastmem_contract_family": "unknown",
        "fastmem_general_rawptr_type": 0,
        "fastmem_general_deref_outside_region": 0,
        "fastmem_general_pointer_arithmetic_outside_region": 0,
        "fastmem_region_pointer_arithmetic_count": 0,
        "fastmem_region_typed_load_count": 0,
        "fastmem_region_typed_store_count": 0,
        "fastmem_region_atomic_op_count": 0,
        "fastmem_escape_count": 0,
        "fastmem_metadata_ptr_escape_count": 0,
        "fastmem_user_ptr_abi_return_count": 0,
        "fastmem_closure_capture_count": 0,
        "fastmem_box_field_store_count": 0,
        "fastmem_array_store_count": 0,
        "fastmem_layout_verified": 0,
        "fastmem_layout_id": "unknown",
        "fastmem_layout_hash": "unknown",
        "fastmem_unverified_offset_load_count": 0,
        "fastmem_contract_runtime_lookup_count": 0,
        "fastmem_memop_region_begin_count": 0,
        "fastmem_memop_region_end_count": 0,
        "fastmem_memop_unbalanced_region_count": 0,
        "fastmem_memop_unclassified_count": 0,
        "fastmem_memop_addr_of_count": 0,
        "fastmem_memop_add_count": 0,
        "fastmem_memop_sub_count": 0,
        "fastmem_memop_logical_shr_count": 0,
        "fastmem_memop_and_count": 0,
        "fastmem_memop_table_index_count": 0,
        "fastmem_memop_field_load_count": 0,
        "fastmem_memop_field_store_count": 0,
        "fastmem_memop_typed_load_count": 0,
        "fastmem_memop_typed_store_count": 0,
        "fastmem_memop_atomic_cas_count": 0,
        "fastmem_memop_atomic_exchange_count": 0,
        "fastmem_memop_atomic_fetch_add_count": 0,
        "fastmem_forbidden_allocation_count": 0,
        "fastmem_forbidden_safepoint_count": 0,
        "fastmem_forbidden_await_count": 0,
        "fastmem_forbidden_nowait_count": 0,
        "fastmem_forbidden_call_count": 0,
        "fastmem_type_abi_hot_lookup_count": 0,
        "fastmem_provider_abi_crossing_count": 0,
        "address_token_capability": 0,
        "address_token_escape_check": "missing",
        "address_token_deref_allowed": 0,
        "address_token_pointer_arithmetic_allowed": 0,
        "page_key_capability": int_value(
            rows, [f"subject_{idx}_page_key_capability", "page_key_capability"], 0
        ),
        "page_key_numeric_route": first_value(
            rows,
            [f"subject_{idx}_page_key_numeric_route", "page_key_numeric_route"],
            "missing",
        ),
        "page_key_shift_count_trap": int_value(
            rows, [f"subject_{idx}_page_key_shift_count_trap", "page_key_shift_count_trap"], 0
        ),
        "page_key_segment_shift": first_value(
            rows,
            [f"subject_{idx}_page_key_segment_shift", "page_key_segment_shift"],
            "unknown",
        ),
        "page_key_page_shift": first_value(
            rows,
            [f"subject_{idx}_page_key_page_shift", "page_key_page_shift"],
            "unknown",
        ),
        "page_key_mask": first_value(
            rows,
            [f"subject_{idx}_page_key_mask", "page_key_mask"],
            "unknown",
        ),
        "free_path_page_lookup_route": free_path_route,
        "free_path_page_lookup_range_scan_count": replacement[
            "page_from_ptr_range_scan_count_total"
        ],
        "page_map_bridge_kind": bridge_kind,
        "page_map_bridge_type_abi_hot_lookup_count": replacement[
            "type_abi_hot_path_lookup_count"
        ],
        "page_map_bridge_provider_abi_hot_dispatch_count": replacement[
            "provider_dispatch_hot_path"
        ],
        "typed_page_meta_handle": 0,
        "typed_page_table_mode": "side_table" if page_map_bridge_present else "none",
        "worker_id_capability": 0,
        "allocator_tls_arena_enabled": allocator_tls_enabled,
        "allocator_tls_arena_count": int_route_flag(
            rows, replacement, "replacement_front_tls_arena_count"
        ),
        "allocator_thread_exit_flush_count": int_route_flag(
            rows, replacement, "replacement_front_thread_exit_arena_flush_count"
        ),
        "allocator_abandoned_owner_count": int_route_flag(
            rows, replacement, "replacement_front_abandoned_owner_count"
        ),
        "atomic_remote_head_enabled": atomic_remote_enabled,
        "remote_free_push_count": replacement["remote_free_push_count_total"],
        "remote_free_drain_count": replacement["remote_free_drain_count_total"],
        "remote_free_cas_retry_count": replacement["remote_free_cas_retry_count_total"],
        "remote_free_memory_order": classify_remote_memory_order(rows, replacement),
        "mimalloc_shape_page_free_lists": (
            "free_local_remote" if atomic_remote_enabled else "free_only"
        ),
        "mimalloc_shape_thread_local_heap": allocator_tls_enabled,
        "mimalloc_shape_segment_slice_lookup": int(bridge_kind == "two_level_segment_table"),
        "mimalloc_shape_score": shape_score,
        "safety_score": 100,
        "coverage_score": shape_score,
        "replacement_front_is_full_hako_algorithm": replacement[
            "replacement_front_is_full_hako_algorithm"
        ],
        "hako_mimalloc_algorithm_claim": int_value(
            rows,
            [
                f"subject_{idx}_hako_mimalloc_algorithm_claim",
                "hako_mimalloc_algorithm_claim",
            ],
            0,
        ),
        "product_activation_ready": replacement["replacement_front_product_activation_ready"],
    }
    report["summary"] = "ok" if report["benchmark_front_class"] else "failed"
    return report


def emit_summary(report: dict[str, Any]) -> str:
    lines = [
        f"contract: {report['output_contract']}",
        f"front: {report['benchmark_front_class']} threads={report['benchmark_threads']}",
        (
            "fastmem: "
            f"regions={report['fastmem_region_count']} "
            f"contracts={report['fastmem_contract_count']} "
            f"runtime_lookup={report['fastmem_contract_runtime_lookup_count']}"
        ),
        (
            "lookup: "
            f"route={report['free_path_page_lookup_route']} "
            f"bridge={report['page_map_bridge_kind']} "
            f"range_scan={report['free_path_page_lookup_range_scan_count']}"
        ),
        (
            "claims: "
            f"type_abi_hot_lookup={report['type_abi_hot_path_lookup_count']} "
            f"provider_hot_dispatch={report['provider_dispatch_hot_path']} "
            f"product_activation={report['provider_activation']}"
        ),
        (
            "shape: "
            f"score={report['mimalloc_shape_score']} "
            f"tls={report['allocator_tls_arena_enabled']} "
            f"remote={report['atomic_remote_head_enabled']}"
        ),
        f"summary: {report['summary']}",
    ]
    return "\n".join(lines) + "\n"


def write_output(text: str, out: Path | None) -> None:
    if out is None:
        print(text, end="")
        return
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text(text, encoding="utf-8")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--report", required=True, type=Path)
    parser.add_argument("--format", choices=("kv", "summary", "json"), default="kv")
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    rows = read_kv(args.report)
    report = build_inventory(rows)

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
