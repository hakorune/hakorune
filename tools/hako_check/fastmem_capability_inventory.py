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
from typing import Any, Iterable

from replacement_front_report import (
    build_report as build_replacement_report,
    emit_kv,
    first_value,
    format_value,
    int_value,
    page_lookup_route,
    page_map_bridge_kind,
    prefixed,
    read_kv,
)


def route_value(rows: dict[str, str], subject_idx: int, suffix: str, default: str = "") -> str:
    return prefixed(rows, subject_idx, suffix, default)


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


def contract_family(contract: str) -> str:
    if contract == "PageMapV0":
        return "allocator.page_map"
    if contract.startswith("RemoteFree"):
        return "allocator.remote_free"
    if contract.startswith("TlsArena") or contract.startswith("TLSArena"):
        return "allocator.tls_arena"
    return "unknown"


def is_node(node: dict[str, Any], *names: str) -> bool:
    kind = node.get("kind")
    type_name = node.get("type")
    return any(name == kind or name == type_name for name in names)


def child_expr(node: dict[str, Any], *keys: str) -> Any | None:
    for key in keys:
        if key in node and node[key] is not None:
            return node[key]
    return None


def iter_nodes(value: Any) -> Iterable[dict[str, Any]]:
    if isinstance(value, dict):
        yield value
        for child in value.values():
            yield from iter_nodes(child)
    elif isinstance(value, list):
        for item in value:
            yield from iter_nodes(item)


def iter_fastmem_regions(root: Any) -> Iterable[dict[str, Any]]:
    for node in iter_nodes(root):
        if is_node(node, "FastMemRegion"):
            yield node


def is_mem_method_call(node: dict[str, Any], method: str) -> bool:
    receiver = child_expr(node, "receiver", "object")
    if not isinstance(receiver, dict):
        return False
    if not is_node(receiver, "Variable", "Var"):
        return False
    return receiver.get("name") == "mem" and node.get("method") == method


def call_name(node: dict[str, Any]) -> str:
    name = node.get("name")
    return name if isinstance(name, str) else ""


def add_count(counts: dict[str, int], key: str, amount: int = 1) -> None:
    counts[key] = counts.get(key, 0) + amount


def analyze_expr(expr: Any, counts: dict[str, int]) -> None:
    if not isinstance(expr, dict):
        return

    if is_node(expr, "BinaryOp", "Binary", "Compare"):
        op = expr.get("op")
        if op == ">>":
            add_count(counts, "fastmem_memop_logical_shr_count")
        elif op == "&":
            add_count(counts, "fastmem_memop_and_count")
        elif op == "+":
            add_count(counts, "fastmem_memop_add_count")
        elif op == "-":
            add_count(counts, "fastmem_memop_sub_count")
        analyze_expr(child_expr(expr, "left", "lhs"), counts)
        analyze_expr(child_expr(expr, "right", "rhs"), counts)
        return

    if is_node(expr, "MethodCall", "Method"):
        if is_mem_method_call(expr, "addr"):
            add_count(counts, "fastmem_memop_addr_of_count")
        elif is_mem_method_call(expr, "load"):
            add_count(counts, "fastmem_memop_typed_load_count")
            add_count(counts, "fastmem_region_typed_load_count")
        elif is_mem_method_call(expr, "store"):
            add_count(counts, "fastmem_memop_typed_store_count")
            add_count(counts, "fastmem_region_typed_store_count")
        elif is_mem_method_call(expr, "atomicCas"):
            add_count(counts, "fastmem_memop_atomic_cas_count")
            add_count(counts, "fastmem_region_atomic_op_count")
        elif is_mem_method_call(expr, "atomicExchange"):
            add_count(counts, "fastmem_memop_atomic_exchange_count")
            add_count(counts, "fastmem_region_atomic_op_count")
        elif is_mem_method_call(expr, "atomicFetchAdd"):
            add_count(counts, "fastmem_memop_atomic_fetch_add_count")
            add_count(counts, "fastmem_region_atomic_op_count")
        else:
            add_count(counts, "fastmem_forbidden_call_count")
        for arg in child_expr(expr, "arguments", "args") or []:
            analyze_expr(arg, counts)
        return

    if is_node(expr, "FunctionCall", "Call"):
        name = call_name(expr)
        if name == "mem.addr":
            add_count(counts, "fastmem_memop_addr_of_count")
        elif name == "mem.load":
            add_count(counts, "fastmem_memop_typed_load_count")
            add_count(counts, "fastmem_region_typed_load_count")
        elif name == "mem.store":
            add_count(counts, "fastmem_memop_typed_store_count")
            add_count(counts, "fastmem_region_typed_store_count")
        else:
            add_count(counts, "fastmem_forbidden_call_count")
        for arg in child_expr(expr, "arguments", "args") or []:
            analyze_expr(arg, counts)
        return

    if is_node(expr, "Index"):
        add_count(counts, "fastmem_memop_table_index_count")
        analyze_expr(child_expr(expr, "target"), counts)
        analyze_expr(child_expr(expr, "index"), counts)
        return

    if is_node(expr, "AwaitExpression", "Await"):
        add_count(counts, "fastmem_forbidden_await_count")
        analyze_expr(child_expr(expr, "expression"), counts)
        return

    if is_node(expr, "NowaitExpression", "Nowait"):
        add_count(counts, "fastmem_forbidden_nowait_count")
        analyze_expr(child_expr(expr, "expression"), counts)
        return

    if is_node(expr, "New", "Constructor", "ArrayLiteral", "MapLiteral"):
        add_count(counts, "fastmem_forbidden_allocation_count")
        return

    if is_node(expr, "Unsupported"):
        add_count(counts, "fastmem_memop_unclassified_count")


def analyze_stmt(stmt: Any, counts: dict[str, int]) -> None:
    if not isinstance(stmt, dict):
        return

    if is_node(stmt, "Local"):
        expr = child_expr(stmt, "expr", "value")
        if expr is None:
            inits = child_expr(stmt, "inits") or []
            expr = inits[0] if inits else None
        analyze_expr(expr, counts)
        return

    if is_node(stmt, "Assignment"):
        analyze_expr(child_expr(stmt, "value", "expr"), counts)
        return

    if is_node(stmt, "Return"):
        analyze_expr(child_expr(stmt, "value"), counts)
        return

    if is_node(stmt, "Print"):
        analyze_expr(child_expr(stmt, "expression", "expr"), counts)
        return

    if is_node(stmt, "If"):
        analyze_expr(child_expr(stmt, "condition"), counts)
        for child in child_expr(stmt, "then_body", "then", "thenBody") or []:
            analyze_stmt(child, counts)
        for child in child_expr(stmt, "else_body", "else", "elseBody") or []:
            analyze_stmt(child, counts)
        return

    if is_node(stmt, "FastMemRegion"):
        add_count(counts, "fastmem_memop_unclassified_count")
        return

    analyze_expr(stmt, counts)


def build_source_inventory(root: Any, input_kind: str) -> dict[str, Any]:
    regions = list(iter_fastmem_regions(root))
    counts: dict[str, int] = {}
    contracts: list[str] = []
    for region in regions:
        contract = region.get("contract")
        if isinstance(contract, str) and contract:
            contracts.append(contract)
        add_count(counts, "fastmem_memop_region_begin_count")
        add_count(counts, "fastmem_memop_region_end_count")
        for stmt in region.get("body") or []:
            analyze_stmt(stmt, counts)

    unique_contracts = sorted(set(contracts))
    if len(unique_contracts) == 1:
        contract_id = unique_contracts[0]
    elif unique_contracts:
        contract_id = "multiple"
    else:
        contract_id = "unknown"
    region_count = len(regions)
    contract_count = len(unique_contracts)
    unbalanced = abs(
        counts.get("fastmem_memop_region_begin_count", 0)
        - counts.get("fastmem_memop_region_end_count", 0)
    )

    report = base_inventory(input_kind)
    report.update(
        {
            "measured_hot_path_owner": "hako_source",
            "replacement_front_subowner": "not_applicable",
            "benchmark_subject_index": 0,
            "benchmark_front_class": "hako_source_fastmem_metadata",
            "hako_source_hot_path_claim": 0,
            "fastmem_region_count": region_count,
            "fastmem_contract_count": contract_count,
            "fastmem_contract_id": contract_id,
            "fastmem_contract_family": contract_family(contract_id),
            "fastmem_memop_unbalanced_region_count": unbalanced,
            "typed_page_table_mode": "none",
            "mimalloc_shape_page_free_lists": "missing",
            "summary": "ok",
        }
    )
    report.update(counts)
    return report


def base_inventory(input_kind: str) -> dict[str, Any]:
    return {
        "output_contract": "hako-check-fastmem-capability-inventory-v0",
        "input_kind": input_kind,
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
        "measured_hot_path_owner": "unknown",
        "replacement_front_subowner": "unknown",
        "benchmark_subject_index": "",
        "benchmark_front_class": "",
        "benchmark_threads": 0,
        "benchmark_thread_origin": "none",
        "hako_hot_path_claim": 0,
        "hako_source_thread_support_claim": 0,
        "hako_source_hot_path_claim": 0,
        "mir_builder_hot_path_claim": 0,
        "type_abi_hot_path_lookup_count": 0,
        "provider_dispatch_hot_path": 0,
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
        "page_key_capability": 0,
        "page_key_numeric_route": "missing",
        "page_key_shift_count_trap": 0,
        "page_key_segment_shift": "unknown",
        "page_key_page_shift": "unknown",
        "page_key_mask": "unknown",
        "free_path_page_lookup_route": "unknown",
        "free_path_page_lookup_range_scan_count": 0,
        "page_map_bridge_kind": "none",
        "page_map_bridge_type_abi_hot_lookup_count": 0,
        "page_map_bridge_provider_abi_hot_dispatch_count": 0,
        "typed_page_meta_handle": 0,
        "typed_page_table_mode": "none",
        "worker_id_capability": 0,
        "allocator_tls_arena_enabled": 0,
        "allocator_tls_arena_count": 0,
        "allocator_thread_exit_flush_count": 0,
        "allocator_abandoned_owner_count": 0,
        "atomic_remote_head_enabled": 0,
        "remote_free_push_count": 0,
        "remote_free_drain_count": 0,
        "remote_free_cas_retry_count": 0,
        "remote_free_memory_order": "missing",
        "mimalloc_shape_page_free_lists": "missing",
        "mimalloc_shape_thread_local_heap": 0,
        "mimalloc_shape_segment_slice_lookup": 0,
        "mimalloc_shape_score": 0,
        "safety_score": 100,
        "coverage_score": 0,
        "replacement_front_is_full_hako_algorithm": 0,
        "hako_mimalloc_algorithm_claim": 0,
        "product_activation_ready": 0,
        "summary": "ok",
    }


def build_inventory(rows: dict[str, str]) -> dict[str, Any]:
    replacement = build_replacement_report(rows, None)
    idx = int(replacement["benchmark_subject_index"])

    free_path_route = page_lookup_route(rows, idx, replacement)
    bridge_kind = page_map_bridge_kind(rows, idx)
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

    report: dict[str, Any] = base_inventory("benchmark_kv_report")
    report.update({
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
        "typed_page_table_mode": "side_table" if page_map_bridge_present else "none",
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
    })
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
    source = parser.add_mutually_exclusive_group(required=True)
    source.add_argument("--report", type=Path, help="Read a benchmark key/value report.")
    source.add_argument("--ast-json", type=Path, help="Read Rust AST JSON containing FastMemRegion nodes.")
    source.add_argument(
        "--program-json",
        type=Path,
        help="Read Program(JSON v0) containing FastMemRegion nodes.",
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
    else:
        report = build_source_inventory(
            json.loads(args.program_json.read_text(encoding="utf-8")),
            "program_json_v0",
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
