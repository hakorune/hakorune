#!/usr/bin/env python3
"""Shared helpers for FastMemory capability inventory."""

from __future__ import annotations

from pathlib import Path
from typing import Any, Iterable

from report_kv import first_value, int_value, prefixed

PAGE_META_FIELDS = (
    "owner_worker_id",
    "block_size",
    "free_head",
    "local_free_head",
    "remote_head",
    "capacity",
    "used",
)

MIMALLOC_SHAPE_COMPONENT_POINTS = 10
MIMALLOC_SHAPE_DEFAULT_THRESHOLD = 80
MIMALLOC_SAFETY_DEFAULT_THRESHOLD = 100
MIMALLOC_COVERAGE_DEFAULT_THRESHOLD = 80

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
    smoke_policy = first_value(rows, ["replacement_front_cross_thread_free_policy"])
    if remote_route == "atomic_page_remote_head" or smoke_policy == "remote_queue":
        return "acq_rel"
    return "missing"


def int_route_flag(rows: dict[str, str], replacement: dict[str, Any], suffix: str) -> int:
    idx = int(replacement["benchmark_subject_index"])
    return int_value(
        rows,
        [
            f"subject_{idx}_{suffix}",
            f"subject_{idx}_{suffix}_total",
            suffix,
            f"{suffix}_total",
        ],
        0,
    )


def first_subject_value(
    rows: dict[str, str],
    subject_idx: int,
    suffix: str,
    default: str = "",
) -> str:
    return first_value(rows, [f"subject_{subject_idx}_{suffix}", suffix], default)


def int_subject_value(
    rows: dict[str, str],
    subject_idx: int,
    suffix: str,
    default: int = 0,
) -> int:
    return int_value(rows, [f"subject_{subject_idx}_{suffix}", suffix], default)


def speed_score_from_ratio(ratio: Any) -> int:
    try:
        value = float(ratio)
    except (TypeError, ValueError):
        return 0
    if value >= 0.90:
        return 100
    if value >= 0.75:
        return 80
    if value >= 0.50:
        return 60
    if value >= 0.25:
        return 40
    if value > 0:
        return 20
    return 0


def typed_page_meta_fields(rows: dict[str, str], subject_idx: int) -> dict[str, int]:
    return {
        field: int_subject_value(rows, subject_idx, f"typed_page_meta_field_{field}", 0)
        for field in PAGE_META_FIELDS
    }


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
        elif is_mem_method_call(expr, "currentAllocOwnerId"):
            add_count(counts, "fastmem_memop_current_alloc_owner_id_count")
        elif is_mem_method_call(expr, "ownerEq"):
            add_count(counts, "fastmem_memop_owner_eq_count")
        elif is_mem_method_call(expr, "localFreePush"):
            add_count(counts, "fastmem_memop_local_free_push_count")
        elif is_mem_method_call(expr, "localFreePop"):
            add_count(counts, "fastmem_memop_local_free_pop_count")
        elif is_mem_method_call(expr, "freeHeadPop"):
            add_count(counts, "fastmem_memop_free_head_pop_count")
        elif (
            is_mem_method_call(expr, "assumeTableLength")
            or is_mem_method_call(expr, "assumeIndexInRange")
            or is_mem_method_call(expr, "assumeSameOwner")
            or is_mem_method_call(expr, "assumeLocalFreeBlockNext")
            or is_mem_method_call(expr, "assumeLocalFreeNonEmpty")
        ):
            pass
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
        elif name == "mem.currentAllocOwnerId":
            add_count(counts, "fastmem_memop_current_alloc_owner_id_count")
        elif name == "mem.ownerEq":
            add_count(counts, "fastmem_memop_owner_eq_count")
        elif name == "mem.localFreePush":
            add_count(counts, "fastmem_memop_local_free_push_count")
        elif name == "mem.localFreePop":
            add_count(counts, "fastmem_memop_local_free_pop_count")
        elif name == "mem.freeHeadPop":
            add_count(counts, "fastmem_memop_free_head_pop_count")
        elif name in {
            "mem.assumeTableLength",
            "mem.assumeIndexInRange",
            "mem.assumeSameOwner",
            "mem.assumeLocalFreeBlockNext",
            "mem.assumeLocalFreeNonEmpty",
        }:
            pass
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

    if is_node(expr, "FieldAccess"):
        add_count(counts, "fastmem_memop_field_load_count")
        analyze_expr(child_expr(expr, "object", "receiver", "target"), counts)
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
        target = child_expr(stmt, "target")
        if not isinstance(target, dict):
            target = child_expr(stmt, "lhs")
        if isinstance(target, dict):
            if is_node(target, "FieldAccess"):
                add_count(counts, "fastmem_memop_field_store_count")
                analyze_expr(child_expr(target, "object", "receiver", "target"), counts)
            elif is_node(target, "Index"):
                add_count(counts, "fastmem_memop_table_index_count")
                analyze_expr(child_expr(target, "target"), counts)
                analyze_expr(child_expr(target, "index"), counts)
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
        "fastmem_memop_current_alloc_owner_id_count": 0,
        "fastmem_memop_owner_eq_count": 0,
        "fastmem_memop_local_free_push_count": 0,
        "fastmem_memop_local_free_pop_count": 0,
        "fastmem_memop_free_head_pop_count": 0,
        "fastmem_local_free_list_plan": 0,
        "fastmem_local_free_push_plan_count": 0,
        "fastmem_local_free_pop_plan_count": 0,
        "fastmem_local_free_nonlowerable_count": 0,
        "fastmem_local_free_push_lowerable_count": 0,
        "fastmem_local_free_pop_lowerable_count": 0,
        "fastmem_local_free_head_access_resolved_count": 0,
        "fastmem_local_free_block_next_access_resolved_count": 0,
        "fastmem_local_free_access_plan_incomplete_count": 0,
        "fastmem_same_owner_fact_count": 0,
        "fastmem_block_next_fact_count": 0,
        "fastmem_local_free_non_empty_fact_count": 0,
        "fastmem_local_free_same_owner_required": 0,
        "fastmem_local_free_same_owner_missing_count": 0,
        "fastmem_local_free_non_empty_required": 0,
        "fastmem_local_free_non_empty_missing_count": 0,
        "fastmem_local_free_remote_owner_rejected_count": 0,
        "fastmem_local_free_block_next_proof_missing_count": 0,
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
        "typed_page_meta_layout_verified": 0,
        "typed_page_meta_layout_id": "unknown",
        "typed_page_meta_layout_hash": "unknown",
        "typed_page_meta_field_count": 0,
        "typed_page_meta_required_field_missing_count": 0,
        "typed_page_meta_field_owner_worker_id": 0,
        "typed_page_meta_field_block_size": 0,
        "typed_page_meta_field_free_head": 0,
        "typed_page_meta_field_local_free_head": 0,
        "typed_page_meta_field_remote_head": 0,
        "typed_page_meta_field_capacity": 0,
        "typed_page_meta_field_used": 0,
        "typed_page_table_mode": "none",
        "alloc_owner_id_capability": 0,
        "alloc_owner_id_kind": "unknown",
        "alloc_owner_id_source": "unknown",
        "alloc_owner_id_width_bits": 0,
        "alloc_owner_id_generation_enabled": 0,
        "alloc_owner_id_zero_is_unowned": 1,
        "alloc_owner_id_escape_count": 0,
        "worker_id_capability": 0,
        "worker_id_kind": "unknown",
        "worker_id_source": "unknown",
        "worker_id_equals_os_thread_id_claim": 0,
        "worker_id_equals_runtime_worker_id_claim": 0,
        "worker_id_equals_hako_task_id_claim": 0,
        "worker_id_escape_count": 0,
        "allocator_tls_arena_enabled": 0,
        "allocator_tls_arena_mode": "unknown",
        "allocator_tls_arena_init_count": 0,
        "allocator_tls_arena_live_count": 0,
        "allocator_tls_arena_peak_count": 0,
        "allocator_tls_arena_reuse_count": 0,
        "allocator_tls_arena_init_fail_count": 0,
        "allocator_tls_arena_fallback_count": 0,
        "allocator_tls_arena_count": 0,
        "allocator_thread_exit_flush_supported": 0,
        "allocator_thread_exit_flush_count": 0,
        "allocator_abandoned_owner_count": 0,
        "replacement_front_owner_shadow_counters": 0,
        "page_owner_check_enabled": 0,
        "page_owner_check_route": "none",
        "page_owner_check_count": 0,
        "page_owner_same_count": 0,
        "page_owner_remote_count": 0,
        "page_owner_unowned_count": 0,
        "page_owner_stale_generation_count": 0,
        "page_owner_invalid_count": 0,
        "page_owner_count_mismatch": 0,
        "same_owner_free_local_candidate_count": 0,
        "same_owner_free_local_route_enabled": 0,
        "replacement_front_same_owner_local_free_route": "disabled",
        "same_owner_free_local_push_count": 0,
        "same_owner_free_local_fallback_count": 0,
        "remote_owner_free_remote_candidate_count": 0,
        "remote_owner_free_remote_push_count": 0,
        "remote_owner_free_fallback_lock_count": 0,
        "atomic_remote_head_plan": 0,
        "atomic_remote_head_route": "none",
        "atomic_remote_head_pilot_enabled": 0,
        "atomic_remote_head_enabled": 0,
        "remote_free_push_count": 0,
        "remote_free_drain_count": 0,
        "remote_free_cas_retry_count": 0,
        "remote_free_memory_order": "missing",
        "replacement_front_cross_thread_free_smoke_ok": 0,
        "replacement_front_cross_thread_free_arena_registry_overflow_count": 0,
        "safe_capability_wrapper_plan": 0,
        "safe_capability_wrapper_route": "none",
        "safe_capability_wrapper_lowering_route": "none",
        "safe_capability_wrapper_memop_equivalence": 0,
        "safe_capability_wrapper_count": 0,
        "safe_capability_wrapper_missing_count": 0,
        "safe_capability_wrapper_rawptr_surface": 0,
        "safe_capability_wrapper_deref_surface": 0,
        "safe_capability_wrapper_escape_count": 0,
        "address_token_wrapper": 0,
        "page_key_wrapper": 0,
        "page_map_bridge_wrapper": 0,
        "page_meta_handle_wrapper": 0,
        "alloc_owner_id_wrapper": 0,
        "atomic_remote_head_wrapper": 0,
        "mimalloc_shape_page_free_lists": "missing",
        "mimalloc_shape_thread_local_heap": 0,
        "mimalloc_shape_segment_slice_lookup": 0,
        "mimalloc_shape_component_count": 0,
        "mimalloc_shape_component_page_map_bridge": 0,
        "mimalloc_shape_component_typed_page_meta": 0,
        "mimalloc_shape_component_tls_arena": 0,
        "mimalloc_shape_component_alloc_owner": 0,
        "mimalloc_shape_component_owner_check": 0,
        "mimalloc_shape_component_same_owner_local_free": 0,
        "mimalloc_shape_component_atomic_remote_head": 0,
        "mimalloc_shape_component_safe_wrappers": 0,
        "mimalloc_shape_component_no_global_lock_hot_path": 0,
        "mimalloc_shape_component_no_range_scan_hot_path": 0,
        "mimalloc_speed_score": 0,
        "mimalloc_shape_score": 0,
        "mimalloc_safety_score": 100,
        "mimalloc_coverage_score": 0,
        "mimalloc_shape_threshold": MIMALLOC_SHAPE_DEFAULT_THRESHOLD,
        "mimalloc_safety_threshold": MIMALLOC_SAFETY_DEFAULT_THRESHOLD,
        "mimalloc_coverage_threshold": MIMALLOC_COVERAGE_DEFAULT_THRESHOLD,
        "mimalloc_keeper_candidate": 0,
        "mimalloc_keeper_eligible": 0,
        "mimalloc_keeper_block_reason": "not_candidate",
        "safety_score": 100,
        "coverage_score": 0,
        "replacement_front_product_shaped_bridge_v0": 0,
        "replacement_front_product_shaped_bridge_non_activating": 1,
        "replacement_front_product_shaped_bridge_report_only": 1,
        "replacement_front_product_shaped_bridge_route": "none",
        "replacement_front_product_shaped_bridge_source_truth": "unknown",
        "replacement_front_product_shaped_bridge_evidence_ready": 0,
        "replacement_front_product_shaped_bridge_activation_ready": 0,
        "replacement_front_product_shaped_bridge_block_reason": "missing_bridge_evidence",
        "replacement_front_product_shaped_bridge_missing": "source_truth,preflight",
        "replacement_front_product_shaped_bridge_shape_ok": 0,
        "replacement_front_product_shaped_bridge_safety_ok": 0,
        "replacement_front_product_shaped_bridge_coverage_ok": 0,
        "replacement_front_product_shaped_bridge_preflight_ok": 0,
        "replacement_front_product_shaped_bridge_no_type_abi_hot_lookup": 0,
        "replacement_front_product_shaped_bridge_no_provider_dispatch": 0,
        "replacement_front_product_shaped_bridge_no_global_lock_hot_path": 0,
        "replacement_front_product_shaped_bridge_no_range_scan_hot_path": 0,
        "replacement_front_product_shaped_bridge_no_host_passthrough": 0,
        "replacement_front_product_shaped_bridge_requires_activation_row": 1,
        "replacement_front_product_shaped_bridge_requires_product_gate_open": 1,
        "replacement_front_size_class_bridge_v0": 0,
        "replacement_front_size_class_bridge_report_only": 1,
        "replacement_front_size_class_bridge_source_truth": "unknown",
        "replacement_front_size_class_bridge_source_file": (
            "lang/src/hako_alloc/memory/size_class_box.hako"
        ),
        "replacement_front_size_class_bridge_mirror_source": "unknown",
        "replacement_front_size_class_bridge_bound": 0,
        "replacement_front_size_class_bridge_missing": "source_file,mirror_source",
        "replacement_front_size_class_required_method_count": 0,
        "replacement_front_size_class_required_methods_present": 0,
        "replacement_front_size_class_missing_methods": "unknown",
        "replacement_front_size_class_word_size": 0,
        "replacement_front_size_class_max_regular_bin": 0,
        "replacement_front_size_class_huge_bin": 0,
        "replacement_front_size_class_huge_sentinel": 0,
        "replacement_front_size_class_usize_facades_present": 0,
        "replacement_front_size_class_policy_methods_covered": 0,
        "replacement_front_size_class_policy_constants_covered": 0,
        "replacement_front_size_class_policy_huge_sentinel_covered": 0,
        "replacement_front_size_class_policy_mirror_matches_source": 0,
        "replacement_front_page_local_bridge_v0": 0,
        "replacement_front_page_local_bridge_report_only": 1,
        "replacement_front_page_local_bridge_source_truth": "unknown",
        "replacement_front_page_local_bridge_source_file": (
            "lang/src/hako_alloc/memory/page_box.hako"
        ),
        "replacement_front_page_local_bridge_mirror_source": "unknown",
        "replacement_front_page_local_bridge_bound": 0,
        "replacement_front_page_local_bridge_missing": "source_file,mirror_source",
        "replacement_front_page_local_required_field_count": 0,
        "replacement_front_page_local_required_fields_present": 0,
        "replacement_front_page_local_missing_fields": "unknown",
        "replacement_front_page_local_required_method_count": 0,
        "replacement_front_page_local_required_methods_present": 0,
        "replacement_front_page_local_missing_methods": "unknown",
        "replacement_front_page_local_directarray_fields_present": 0,
        "replacement_front_page_local_counter_fields_present": 0,
        "replacement_front_page_local_acquire_release_methods_present": 0,
        "replacement_front_page_local_lifecycle_methods_present": 0,
        "replacement_front_page_local_typed_meta_matches_source": 0,
        "replacement_front_page_local_same_owner_route_matches_source": 0,
        "replacement_front_page_local_no_remote_free_claim": 1,
        "replacement_front_producer_taxonomy_v0": 0,
        "replacement_front_producer": "unknown",
        "replacement_front_backend_artifact": "unknown",
        "replacement_front_source_truth": "unknown",
        "replacement_front_python_template_c_semantic_ssot": 0,
        "replacement_front_python_template_c_retirement_required": 0,
        "replacement_front_mir_memop_enabled": 0,
        "replacement_front_mir_fastmem_region_enabled": 0,
        "replacement_front_mirbuilder_representation_only": 1,
        "replacement_front_mirbuilder_route_decision_count": 0,
        "replacement_front_producer_transition_state": "unknown",
        "replacement_front_producer_slice_selection_v0": 0,
        "replacement_front_next_producer_slice": "unknown",
        "replacement_front_selected_memop_family": "unknown",
        "replacement_front_selected_memop_kinds": "unknown",
        "replacement_front_deferred_memop_family": "unknown",
        "replacement_front_deferred_memop_kinds": "unknown",
        "replacement_front_selection_behavior_change": 0,
        "replacement_front_selection_product_activation": 0,
        "replacement_front_selection_bridge_retirement_allowed": 0,
        "mir_fmem_008b_layout_table_producer_pilot": 0,
        "memop_table_index_lowered_count": 0,
        "memop_field_load_lowered_count": 0,
        "memop_field_store_lowered_count": 0,
        "memop_current_alloc_owner_id_lowered_count": 0,
        "memop_owner_eq_lowered_count": 0,
        "memop_atomic_remote_head_lowered_count": 0,
        "fastmem_verified_mem_access_plan_count": 0,
        "fastmem_verified_field_access_count": 0,
        "fastmem_verified_table_access_count": 0,
        "fastmem_field_id_missing_count": 0,
        "fastmem_table_id_missing_count": 0,
        "fastmem_unverified_layout_access_count": 0,
        "fastmem_table_index_unchecked_count": 0,
        "fastmem_table_access_proof_incomplete_count": 0,
        "fastmem_table_overflow_proof_missing_count": 0,
        "fastmem_unknown_alignment_count": 0,
        "fastmem_atomic_field_plain_store_count": 0,
        "fastmem_layout_ref_escape_count": 0,
        "fastmem_lowering_recomputed_layout_offset_count": 0,
        "replacement_front_is_full_hako_algorithm": 0,
        "hako_mimalloc_algorithm_claim": 0,
        "product_activation_ready": 0,
        "summary": "ok",
    }


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
