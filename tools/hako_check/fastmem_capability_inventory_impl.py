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

    replacement_subowner = (
        "remote_free_queue" if atomic_remote_enabled else replacement["likely_next_owner"]
    )
    address_token_capability = int_value(
        rows, [f"subject_{idx}_address_token_capability", "address_token_capability"], 0
    )
    address_token_escape_check = first_value(
        rows,
        [f"subject_{idx}_address_token_escape_check", "address_token_escape_check"],
        "pass" if address_token_capability else "missing",
    )
    address_token_deref_allowed = int_value(
        rows,
        [f"subject_{idx}_address_token_deref_allowed", "address_token_deref_allowed"],
        0,
    )
    address_token_pointer_arithmetic_allowed = int_value(
        rows,
        [
            f"subject_{idx}_address_token_pointer_arithmetic_allowed",
            "address_token_pointer_arithmetic_allowed",
        ],
        0,
    )
    page_key_capability = int_value(
        rows, [f"subject_{idx}_page_key_capability", "page_key_capability"], 0
    )
    safe_wrapper_plan = int_subject_value(rows, idx, "safe_capability_wrapper_plan", 0)
    address_token_wrapper = int_subject_value(
        rows, idx, "address_token_wrapper", address_token_capability
    )
    page_key_wrapper = int_subject_value(rows, idx, "page_key_wrapper", page_key_capability)
    page_map_bridge_wrapper = int_subject_value(
        rows, idx, "page_map_bridge_wrapper", page_map_bridge_present
    )
    page_meta_handle_wrapper = int_subject_value(
        rows, idx, "page_meta_handle_wrapper", typed_meta_handle
    )
    alloc_owner_id_wrapper = int_subject_value(
        rows, idx, "alloc_owner_id_wrapper", alloc_owner_id_capability
    )
    atomic_remote_head_wrapper = int_subject_value(
        rows, idx, "atomic_remote_head_wrapper", atomic_remote_enabled
    )
    safe_wrapper_values = (
        address_token_wrapper,
        page_key_wrapper,
        page_map_bridge_wrapper,
        page_meta_handle_wrapper,
        alloc_owner_id_wrapper,
        atomic_remote_head_wrapper,
    )
    safe_wrapper_count = sum(1 for value in safe_wrapper_values if value)
    safe_wrapper_missing_count = (
        sum(1 for value in safe_wrapper_values if not value) if safe_wrapper_plan else 0
    )
    safe_wrapper_route = first_subject_value(
        rows,
        idx,
        "safe_capability_wrapper_route",
        "fastmem_memop_alias" if safe_wrapper_plan else "none",
    )
    safe_wrapper_lowering_route = first_subject_value(
        rows,
        idx,
        "safe_capability_wrapper_lowering_route",
        "fastmem_memop_alias" if safe_wrapper_plan else "none",
    )
    safe_wrapper_rawptr_surface = int_subject_value(
        rows, idx, "safe_capability_wrapper_rawptr_surface", 0
    )
    safe_wrapper_deref_surface = int_subject_value(
        rows, idx, "safe_capability_wrapper_deref_surface", 0
    )
    address_token_escape_count = int(
        address_token_escape_check != "pass" and address_token_capability > 0
    )
    alloc_owner_escape_count = int_subject_value(rows, idx, "alloc_owner_id_escape_count", 0)
    worker_id_escape_count = int_subject_value(rows, idx, "worker_id_escape_count", 0)
    safe_wrapper_escape_default = (
        address_token_escape_count + alloc_owner_escape_count + worker_id_escape_count
    )
    safe_wrapper_escape_count = int_subject_value(
        rows,
        idx,
        "safe_capability_wrapper_escape_count",
        safe_wrapper_escape_default,
    )
    safe_wrapper_memop_equivalence = int_subject_value(
        rows,
        idx,
        "safe_capability_wrapper_memop_equivalence",
        int(
            safe_wrapper_plan
            and safe_wrapper_route == "fastmem_memop_alias"
            and safe_wrapper_lowering_route == "fastmem_memop_alias"
            and safe_wrapper_missing_count == 0
            and safe_wrapper_rawptr_surface == 0
            and safe_wrapper_deref_surface == 0
            and safe_wrapper_escape_count == 0
        ),
    )

    safe_wrapper_shape_component = int(
        safe_wrapper_plan
        and safe_wrapper_memop_equivalence
        and safe_wrapper_missing_count == 0
    )
    shape_components = {
        "page_map_bridge": page_map_bridge_present,
        "typed_page_meta": typed_meta_handle,
        "tls_arena": allocator_tls_enabled,
        "alloc_owner": alloc_owner_id_capability,
        "owner_check": int(page_owner_check_count > 0),
        "same_owner_local_free": int(same_owner_route_enabled and same_owner_push_count > 0),
        "atomic_remote_head": atomic_remote_enabled,
        "safe_wrappers": safe_wrapper_shape_component,
        "no_global_lock_hot_path": int(replacement["global_lock_hot_path_count_total"] == 0),
        "no_range_scan_hot_path": int(replacement["page_from_ptr_range_scan_count_total"] == 0),
    }
    shape_component_count = sum(1 for value in shape_components.values() if value)
    shape_score = shape_component_count * MIMALLOC_SHAPE_COMPONENT_POINTS
    speed_score = speed_score_from_ratio(replacement["throughput_vs_c_mimalloc"])
    safety_penalty_count = sum(
        1
        for failed in [
            address_token_deref_allowed,
            address_token_pointer_arithmetic_allowed,
            address_token_escape_count,
            alloc_owner_escape_count,
            worker_id_escape_count,
            page_owner_unowned_count,
            page_owner_stale_count,
            page_owner_invalid_count,
            smoke_remote_overflow_count,
            safe_wrapper_rawptr_surface,
            safe_wrapper_deref_surface,
            safe_wrapper_escape_count,
            replacement["type_abi_hot_path_lookup_count"],
            replacement["provider_dispatch_hot_path"],
        ]
        if failed
    )
    safety_score = max(0, 100 - safety_penalty_count * 20)
    coverage_score = shape_score
    shape_threshold = int_subject_value(
        rows, idx, "mimalloc_shape_threshold", MIMALLOC_SHAPE_DEFAULT_THRESHOLD
    )
    safety_threshold = int_subject_value(
        rows, idx, "mimalloc_safety_threshold", MIMALLOC_SAFETY_DEFAULT_THRESHOLD
    )
    coverage_threshold = int_subject_value(
        rows, idx, "mimalloc_coverage_threshold", MIMALLOC_COVERAGE_DEFAULT_THRESHOLD
    )
    keeper_candidate = int_subject_value(rows, idx, "mimalloc_keeper_candidate", 0)
    if not keeper_candidate:
        keeper_block_reason = "not_candidate"
    elif shape_score < shape_threshold:
        keeper_block_reason = "shape_below_threshold"
    elif safety_score < safety_threshold:
        keeper_block_reason = "safety_below_threshold"
    elif coverage_score < coverage_threshold:
        keeper_block_reason = "coverage_below_threshold"
    else:
        keeper_block_reason = "eligible"
    keeper_eligible = int(keeper_candidate and keeper_block_reason == "eligible")
    product_bridge_shape_ok = int(
        shape_score >= shape_threshold
        and replacement["replacement_front_product_shaped_bridge_no_global_lock_hot_path"]
        and replacement["replacement_front_product_shaped_bridge_no_range_scan_hot_path"]
    )
    product_bridge_safety_ok = int(
        safety_score >= safety_threshold
        and replacement["replacement_front_product_shaped_bridge_no_type_abi_hot_lookup"]
        and replacement["replacement_front_product_shaped_bridge_no_provider_dispatch"]
        and replacement["replacement_front_product_activation_ready"] == 0
    )
    product_bridge_preflight_ok = replacement[
        "replacement_front_product_shaped_bridge_preflight_ok"
    ]
    product_bridge_coverage_ok = int(
        coverage_score >= coverage_threshold
        and replacement["replacement_front_product_shaped_bridge_source_truth"]
        == "hako_alloc.size_class_box"
    )
    product_bridge_no_host_passthrough = replacement[
        "replacement_front_product_shaped_bridge_no_host_passthrough"
    ]
    product_bridge_missing_parts = [
        part
        for part, missing in [
            ("shape", not product_bridge_shape_ok),
            ("safety", not product_bridge_safety_ok),
            ("coverage", not product_bridge_coverage_ok),
            ("preflight", not product_bridge_preflight_ok),
            ("host_passthrough_zero", not product_bridge_no_host_passthrough),
        ]
        if missing
    ]
    for blocker in str(
        replacement["replacement_front_product_shaped_bridge_missing"]
    ).split(","):
        if blocker and blocker != "none" and blocker not in product_bridge_missing_parts:
            product_bridge_missing_parts.append(blocker)
    product_bridge_evidence_ready = int(
        product_bridge_shape_ok
        and product_bridge_safety_ok
        and product_bridge_coverage_ok
        and product_bridge_preflight_ok
        and product_bridge_no_host_passthrough
    )
    product_bridge_missing = (
        ",".join(product_bridge_missing_parts) if product_bridge_missing_parts else "none"
    )
    product_bridge_block_reason = (
        "activation_row_required" if product_bridge_evidence_ready else "missing_bridge_evidence"
    )

    report: dict[str, Any] = base_inventory("benchmark_kv_report")
    report.update({
        "measured_hot_path_owner": replacement["measured_hot_path_owner"],
        "replacement_front_subowner": replacement_subowner,
        "benchmark_subject_index": replacement["benchmark_subject_index"],
        "benchmark_front_class": replacement["benchmark_front_class"],
        "benchmark_threads": replacement["benchmark_threads"],
        "benchmark_thread_origin": replacement["benchmark_thread_origin"],
        "hako_hot_path_claim": replacement["hako_hot_path_claim"],
        "hako_source_thread_support_claim": replacement["hako_source_thread_support_claim"],
        "hako_source_hot_path_claim": 0,
        "mir_builder_hot_path_claim": 0,
        "address_token_capability": address_token_capability,
        "address_token_escape_check": address_token_escape_check,
        "address_token_deref_allowed": address_token_deref_allowed,
        "address_token_pointer_arithmetic_allowed": address_token_pointer_arithmetic_allowed,
        "page_key_capability": page_key_capability,
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
        "typed_page_meta_handle": typed_meta_handle,
        "typed_page_meta_layout_verified": typed_meta_layout_verified,
        "typed_page_meta_layout_id": typed_meta_layout_id,
        "typed_page_meta_layout_hash": typed_meta_layout_hash,
        "typed_page_meta_field_count": typed_meta_field_count,
        "typed_page_meta_required_field_missing_count": typed_meta_missing_count,
        "typed_page_meta_field_owner_worker_id": typed_meta_fields["owner_worker_id"],
        "typed_page_meta_field_block_size": typed_meta_fields["block_size"],
        "typed_page_meta_field_free_head": typed_meta_fields["free_head"],
        "typed_page_meta_field_local_free_head": typed_meta_fields["local_free_head"],
        "typed_page_meta_field_remote_head": typed_meta_fields["remote_head"],
        "typed_page_meta_field_capacity": typed_meta_fields["capacity"],
        "typed_page_meta_field_used": typed_meta_fields["used"],
        "fastmem_layout_verified": typed_meta_layout_verified,
        "fastmem_layout_id": typed_meta_layout_id,
        "fastmem_layout_hash": typed_meta_layout_hash,
        "fastmem_unverified_offset_load_count": int_subject_value(
            rows, idx, "fastmem_unverified_offset_load_count", 0
        ),
        "typed_page_table_mode": "side_table" if page_map_bridge_present else "none",
        "alloc_owner_id_capability": alloc_owner_id_capability,
        "alloc_owner_id_kind": alloc_owner_id_kind,
        "alloc_owner_id_source": alloc_owner_id_source,
        "alloc_owner_id_width_bits": alloc_owner_id_width_bits,
        "alloc_owner_id_generation_enabled": alloc_owner_id_generation_enabled,
        "alloc_owner_id_zero_is_unowned": alloc_owner_id_zero_is_unowned,
        "alloc_owner_id_escape_count": int_subject_value(
            rows, idx, "alloc_owner_id_escape_count", 0
        ),
        "allocator_owner_lifecycle_state_machine": allocator_owner_lifecycle_state_machine,
        "allocator_owner_generation_enabled": allocator_owner_generation_enabled,
        "allocator_owner_id_kind": allocator_owner_id_kind,
        "allocator_owner_id_repr": first_subject_value(
            rows,
            idx,
            "allocator_owner_id_repr",
            "packed_u64_slot_generation" if allocator_owner_generation_enabled else "unknown",
        ),
        "allocator_owner_slot_bits": int_subject_value(
            rows, idx, "allocator_owner_slot_bits", 32 if allocator_owner_generation_enabled else 0
        ),
        "allocator_owner_generation_bits": int_subject_value(
            rows,
            idx,
            "allocator_owner_generation_bits",
            32 if allocator_owner_generation_enabled else 0,
        ),
        "allocator_owner_zero_is_invalid": int_subject_value(
            rows, idx, "allocator_owner_zero_is_invalid", alloc_owner_id_zero_is_unowned
        ),
        "allocator_owner_active_count": allocator_owner_active_count,
        "allocator_owner_exiting_flush_count": allocator_owner_exiting_flush_count,
        "allocator_owner_abandoned_count": allocator_owner_abandoned_count,
        "allocator_owner_reclaimed_count": allocator_owner_reclaimed_count,
        "allocator_owner_invalid_transition_count": int_subject_value(
            rows,
            idx,
            "allocator_owner_invalid_transition_count",
            replacement_counter("allocator_owner_invalid_transition_count"),
        ),
        "allocator_owner_stale_generation_count": int_subject_value(
            rows,
            idx,
            "allocator_owner_stale_generation_count",
            replacement_counter("allocator_owner_stale_generation_count")
            or page_owner_stale_count,
        ),
        "allocator_owner_generation_bump_count": int_subject_value(
            rows,
            idx,
            "allocator_owner_generation_bump_count",
            replacement_counter("allocator_owner_generation_bump_count"),
        ),
        "allocator_owner_reuse_without_generation_bump_count": int_subject_value(
            rows,
            idx,
            "allocator_owner_reuse_without_generation_bump_count",
            replacement_counter("allocator_owner_reuse_without_generation_bump_count"),
        ),
        "worker_id_capability": worker_id_capability,
        "worker_id_kind": worker_id_kind,
        "worker_id_source": worker_id_source,
        "worker_id_equals_os_thread_id_claim": int_subject_value(
            rows, idx, "worker_id_equals_os_thread_id_claim", 0
        ),
        "worker_id_equals_runtime_worker_id_claim": int_subject_value(
            rows, idx, "worker_id_equals_runtime_worker_id_claim", 0
        ),
        "worker_id_equals_hako_task_id_claim": int_subject_value(
            rows, idx, "worker_id_equals_hako_task_id_claim", 0
        ),
        "worker_id_escape_count": int_subject_value(rows, idx, "worker_id_escape_count", 0),
        "allocator_tls_arena_enabled": allocator_tls_enabled,
        "allocator_tls_arena_mode": first_subject_value(
            rows,
            idx,
            "allocator_tls_arena_mode",
            "benchmark_c_tls" if alloc_owner_id_capability else "unknown",
        ),
        "allocator_tls_arena_init_count": tls_arena_init_count,
        "allocator_tls_arena_live_count": tls_arena_live_count,
        "allocator_tls_arena_peak_count": tls_arena_peak_count,
        "allocator_tls_arena_reuse_count": int_subject_value(
            rows, idx, "allocator_tls_arena_reuse_count", 0
        ),
        "allocator_tls_arena_init_fail_count": int_subject_value(
            rows, idx, "allocator_tls_arena_init_fail_count", 0
        ),
        "allocator_tls_arena_fallback_count": int_subject_value(
            rows, idx, "allocator_tls_arena_fallback_count", 0
        ),
        "allocator_tls_arena_count": int_route_flag(
            rows, replacement, "replacement_front_tls_arena_count"
        ),
        "allocator_thread_exit_observed_count": int_subject_value(
            rows,
            idx,
            "allocator_thread_exit_observed_count",
            replacement_counter("allocator_thread_exit_observed_count")
            or int(allocator_thread_exit_flush_count > 0),
        ),
        "allocator_thread_exit_flush_supported": int_subject_value(
            rows,
            idx,
            "allocator_thread_exit_flush_supported",
            replacement_counter("allocator_thread_exit_flush_supported"),
        ),
        "allocator_thread_exit_flush_count": allocator_thread_exit_flush_count,
        "allocator_thread_exit_flush_page_count": int_subject_value(
            rows,
            idx,
            "allocator_thread_exit_flush_page_count",
            replacement_counter("allocator_thread_exit_flush_page_count"),
        ),
        "allocator_thread_exit_local_free_drain_count": int_subject_value(
            rows,
            idx,
            "allocator_thread_exit_local_free_drain_count",
            replacement_counter("allocator_thread_exit_local_free_drain_count"),
        ),
        "allocator_thread_exit_remote_candidate_seen_count": int_subject_value(
            rows,
            idx,
            "allocator_thread_exit_remote_candidate_seen_count",
            replacement_counter("allocator_thread_exit_remote_candidate_seen_count"),
        ),
        "allocator_abandoned_owner_count": allocator_abandoned_owner_count,
        "allocator_abandoned_page_count": int_subject_value(
            rows,
            idx,
            "allocator_abandoned_page_count",
            replacement_counter("allocator_abandoned_page_count"),
        ),
        "allocator_abandoned_live_page_count": int_subject_value(
            rows,
            idx,
            "allocator_abandoned_live_page_count",
            replacement_counter("allocator_abandoned_live_page_count"),
        ),
        "allocator_abandoned_empty_page_count": int_subject_value(
            rows,
            idx,
            "allocator_abandoned_empty_page_count",
            replacement_counter("allocator_abandoned_empty_page_count"),
        ),
        "allocator_abandoned_remote_candidate_count": int_subject_value(
            rows,
            idx,
            "allocator_abandoned_remote_candidate_count",
            max(
                replacement_counter("allocator_abandoned_remote_candidate_count"),
                int_route_flag(
                    rows,
                    replacement,
                    "replacement_front_abandoned_remote_free_count",
                ),
            ),
        ),
        "allocator_abandoned_reclaim_attempt_count": int_subject_value(
            rows,
            idx,
            "allocator_abandoned_reclaim_attempt_count",
            replacement_counter("allocator_abandoned_reclaim_attempt_count"),
        ),
        "allocator_abandoned_reclaim_success_count": int_subject_value(
            rows,
            idx,
            "allocator_abandoned_reclaim_success_count",
            replacement_counter("allocator_abandoned_reclaim_success_count"),
        ),
        "allocator_abandoned_reclaim_blocked_count": int_subject_value(
            rows,
            idx,
            "allocator_abandoned_reclaim_blocked_count",
            replacement_counter("allocator_abandoned_reclaim_blocked_count"),
        ),
        "allocator_abandoned_reclaim_blocked_remote_count": int_subject_value(
            rows,
            idx,
            "allocator_abandoned_reclaim_blocked_remote_count",
            replacement_counter("allocator_abandoned_reclaim_blocked_remote_count"),
        ),
        "remote_candidate_unhandled_reclaim_block_count": int_subject_value(
            rows,
            idx,
            "remote_candidate_unhandled_reclaim_block_count",
            replacement_counter("remote_candidate_unhandled_reclaim_block_count"),
        ),
        "page_reclaimed_with_remote_candidates": int_subject_value(
            rows,
            idx,
            "page_reclaimed_with_remote_candidates",
            replacement_counter("page_reclaimed_with_remote_candidates"),
        ),
        "allocator_exiting_owner_page_claim_count": int_subject_value(
            rows,
            idx,
            "allocator_exiting_owner_page_claim_count",
            replacement_counter("allocator_exiting_owner_page_claim_count"),
        ),
        "allocator_abandoned_owner_local_free_count": int_subject_value(
            rows,
            idx,
            "allocator_abandoned_owner_local_free_count",
            replacement_counter("allocator_abandoned_owner_local_free_count"),
        ),
        "replacement_front_owner_shadow_counters": owner_shadow_counters,
        "page_owner_check_enabled": int_subject_value(
            rows, idx, "page_owner_check_enabled", owner_shadow_counters
        ),
        "page_owner_check_route": first_subject_value(
            rows,
            idx,
            "page_owner_check_route",
            "page_meta_owner_worker_id" if owner_shadow_counters else "none",
        ),
        "page_owner_check_count": page_owner_check_count,
        "page_owner_same_count": page_owner_same_count,
        "page_owner_remote_count": page_owner_remote_count,
        "page_owner_unowned_count": page_owner_unowned_count,
        "page_owner_stale_generation_count": page_owner_stale_count,
        "page_owner_invalid_count": page_owner_invalid_count,
        "page_owner_count_mismatch": int(page_owner_check_count != page_owner_sum),
        "same_owner_free_local_candidate_count": int_subject_value(
            rows, idx, "same_owner_free_local_candidate_count", page_owner_same_count
        ),
        "same_owner_free_local_route_enabled": same_owner_route_enabled,
        "replacement_front_same_owner_local_free_route": first_subject_value(
            rows,
            idx,
            "replacement_front_same_owner_local_free_route",
            "page_meta_owner_local_free" if same_owner_route_enabled else "disabled",
        ),
        "same_owner_free_local_push_count": same_owner_push_count,
        "same_owner_free_local_fallback_count": int_subject_value(
            rows, idx, "same_owner_free_local_fallback_count", same_owner_fallback_default
        ),
        "remote_owner_free_remote_candidate_count": int_subject_value(
            rows,
            idx,
            "remote_owner_free_remote_candidate_count",
            remote_owner_candidate_default,
        ),
        "remote_owner_free_remote_push_count": int_subject_value(
            rows, idx, "remote_owner_free_remote_push_count", remote_owner_push_default
        ),
        "remote_owner_free_fallback_lock_count": int_subject_value(
            rows,
            idx,
            "remote_owner_free_fallback_lock_count",
            remote_owner_fallback_default,
        ),
        "atomic_remote_head_plan": atomic_remote_head_plan,
        "atomic_remote_head_route": first_subject_value(
            rows,
            idx,
            "atomic_remote_head_route",
            "page_remote_head_cas" if atomic_remote_head_plan else "none",
        ),
        "atomic_remote_head_pilot_enabled": atomic_remote_enabled,
        "atomic_remote_head_enabled": atomic_remote_enabled,
        "remote_free_push_count": remote_free_push_count,
        "remote_free_drain_count": remote_free_drain_count,
        "remote_free_drain_supported": remote_free_drain_supported,
        "remote_free_cas_retry_count": replacement["remote_free_cas_retry_count_total"],
        "remote_free_memory_order": classify_remote_memory_order(rows, replacement),
        "replacement_front_cross_thread_free_smoke_ok": int_value(
            rows, ["replacement_front_cross_thread_free_smoke_ok"], 0
        ),
        "replacement_front_cross_thread_free_arena_registry_overflow_count": (
            smoke_remote_overflow_count
        ),
        "safe_capability_wrapper_plan": safe_wrapper_plan,
        "safe_capability_wrapper_route": safe_wrapper_route,
        "safe_capability_wrapper_lowering_route": safe_wrapper_lowering_route,
        "safe_capability_wrapper_memop_equivalence": safe_wrapper_memop_equivalence,
        "safe_capability_wrapper_count": safe_wrapper_count,
        "safe_capability_wrapper_missing_count": safe_wrapper_missing_count,
        "safe_capability_wrapper_rawptr_surface": safe_wrapper_rawptr_surface,
        "safe_capability_wrapper_deref_surface": safe_wrapper_deref_surface,
        "safe_capability_wrapper_escape_count": safe_wrapper_escape_count,
        "address_token_wrapper": address_token_wrapper,
        "page_key_wrapper": page_key_wrapper,
        "page_map_bridge_wrapper": page_map_bridge_wrapper,
        "page_meta_handle_wrapper": page_meta_handle_wrapper,
        "alloc_owner_id_wrapper": alloc_owner_id_wrapper,
        "atomic_remote_head_wrapper": atomic_remote_head_wrapper,
        "mimalloc_shape_page_free_lists": (
            "free_local_remote" if atomic_remote_enabled else "free_only"
        ),
        "mimalloc_shape_thread_local_heap": allocator_tls_enabled,
        "mimalloc_shape_segment_slice_lookup": int(bridge_kind == "two_level_segment_table"),
        "mimalloc_shape_component_count": shape_component_count,
        "mimalloc_shape_component_page_map_bridge": shape_components["page_map_bridge"],
        "mimalloc_shape_component_typed_page_meta": shape_components["typed_page_meta"],
        "mimalloc_shape_component_tls_arena": shape_components["tls_arena"],
        "mimalloc_shape_component_alloc_owner": shape_components["alloc_owner"],
        "mimalloc_shape_component_owner_check": shape_components["owner_check"],
        "mimalloc_shape_component_same_owner_local_free": shape_components[
            "same_owner_local_free"
        ],
        "mimalloc_shape_component_atomic_remote_head": shape_components[
            "atomic_remote_head"
        ],
        "mimalloc_shape_component_safe_wrappers": shape_components["safe_wrappers"],
        "mimalloc_shape_component_no_global_lock_hot_path": shape_components[
            "no_global_lock_hot_path"
        ],
        "mimalloc_shape_component_no_range_scan_hot_path": shape_components[
            "no_range_scan_hot_path"
        ],
        "mimalloc_speed_score": speed_score,
        "mimalloc_shape_score": shape_score,
        "mimalloc_safety_score": safety_score,
        "mimalloc_coverage_score": coverage_score,
        "mimalloc_shape_threshold": shape_threshold,
        "mimalloc_safety_threshold": safety_threshold,
        "mimalloc_coverage_threshold": coverage_threshold,
        "mimalloc_keeper_candidate": keeper_candidate,
        "mimalloc_keeper_eligible": keeper_eligible,
        "mimalloc_keeper_block_reason": keeper_block_reason,
        "safety_score": safety_score,
        "coverage_score": coverage_score,
        "replacement_front_product_shaped_bridge_v0": replacement[
            "replacement_front_product_shaped_bridge_v0"
        ],
        "replacement_front_product_shaped_bridge_non_activating": replacement[
            "replacement_front_product_shaped_bridge_non_activating"
        ],
        "replacement_front_product_shaped_bridge_report_only": replacement[
            "replacement_front_product_shaped_bridge_report_only"
        ],
        "replacement_front_product_shaped_bridge_route": replacement[
            "replacement_front_product_shaped_bridge_route"
        ],
        "replacement_front_product_shaped_bridge_source_truth": replacement[
            "replacement_front_product_shaped_bridge_source_truth"
        ],
        "replacement_front_product_shaped_bridge_evidence_ready": (
            product_bridge_evidence_ready
        ),
        "replacement_front_product_shaped_bridge_activation_ready": 0,
        "replacement_front_product_shaped_bridge_block_reason": product_bridge_block_reason,
        "replacement_front_product_shaped_bridge_missing": product_bridge_missing,
        "replacement_front_product_shaped_bridge_shape_ok": product_bridge_shape_ok,
        "replacement_front_product_shaped_bridge_safety_ok": product_bridge_safety_ok,
        "replacement_front_product_shaped_bridge_coverage_ok": product_bridge_coverage_ok,
        "replacement_front_product_shaped_bridge_preflight_ok": product_bridge_preflight_ok,
        "replacement_front_product_shaped_bridge_no_type_abi_hot_lookup": replacement[
            "replacement_front_product_shaped_bridge_no_type_abi_hot_lookup"
        ],
        "replacement_front_product_shaped_bridge_no_provider_dispatch": replacement[
            "replacement_front_product_shaped_bridge_no_provider_dispatch"
        ],
        "replacement_front_product_shaped_bridge_no_global_lock_hot_path": replacement[
            "replacement_front_product_shaped_bridge_no_global_lock_hot_path"
        ],
        "replacement_front_product_shaped_bridge_no_range_scan_hot_path": replacement[
            "replacement_front_product_shaped_bridge_no_range_scan_hot_path"
        ],
        "replacement_front_product_shaped_bridge_no_host_passthrough": (
            product_bridge_no_host_passthrough
        ),
        "replacement_front_product_shaped_bridge_requires_activation_row": replacement[
            "replacement_front_product_shaped_bridge_requires_activation_row"
        ],
        "replacement_front_product_shaped_bridge_requires_product_gate_open": replacement[
            "replacement_front_product_shaped_bridge_requires_product_gate_open"
        ],
        "replacement_front_size_class_bridge_v0": replacement[
            "replacement_front_size_class_bridge_v0"
        ],
        "replacement_front_size_class_bridge_report_only": replacement[
            "replacement_front_size_class_bridge_report_only"
        ],
        "replacement_front_size_class_bridge_source_truth": replacement[
            "replacement_front_size_class_bridge_source_truth"
        ],
        "replacement_front_size_class_bridge_source_file": replacement[
            "replacement_front_size_class_bridge_source_file"
        ],
        "replacement_front_size_class_bridge_mirror_source": replacement[
            "replacement_front_size_class_bridge_mirror_source"
        ],
        "replacement_front_size_class_bridge_bound": replacement[
            "replacement_front_size_class_bridge_bound"
        ],
        "replacement_front_size_class_bridge_missing": replacement[
            "replacement_front_size_class_bridge_missing"
        ],
        "replacement_front_size_class_required_method_count": replacement[
            "replacement_front_size_class_required_method_count"
        ],
        "replacement_front_size_class_required_methods_present": replacement[
            "replacement_front_size_class_required_methods_present"
        ],
        "replacement_front_size_class_missing_methods": replacement[
            "replacement_front_size_class_missing_methods"
        ],
        "replacement_front_size_class_word_size": replacement[
            "replacement_front_size_class_word_size"
        ],
        "replacement_front_size_class_max_regular_bin": replacement[
            "replacement_front_size_class_max_regular_bin"
        ],
        "replacement_front_size_class_huge_bin": replacement[
            "replacement_front_size_class_huge_bin"
        ],
        "replacement_front_size_class_huge_sentinel": replacement[
            "replacement_front_size_class_huge_sentinel"
        ],
        "replacement_front_size_class_usize_facades_present": replacement[
            "replacement_front_size_class_usize_facades_present"
        ],
        "replacement_front_size_class_policy_methods_covered": replacement[
            "replacement_front_size_class_policy_methods_covered"
        ],
        "replacement_front_size_class_policy_constants_covered": replacement[
            "replacement_front_size_class_policy_constants_covered"
        ],
        "replacement_front_size_class_policy_huge_sentinel_covered": replacement[
            "replacement_front_size_class_policy_huge_sentinel_covered"
        ],
        "replacement_front_size_class_policy_mirror_matches_source": replacement[
            "replacement_front_size_class_policy_mirror_matches_source"
        ],
        "replacement_front_page_local_bridge_v0": replacement[
            "replacement_front_page_local_bridge_v0"
        ],
        "replacement_front_page_local_bridge_report_only": replacement[
            "replacement_front_page_local_bridge_report_only"
        ],
        "replacement_front_page_local_bridge_source_truth": replacement[
            "replacement_front_page_local_bridge_source_truth"
        ],
        "replacement_front_page_local_bridge_source_file": replacement[
            "replacement_front_page_local_bridge_source_file"
        ],
        "replacement_front_page_local_bridge_mirror_source": replacement[
            "replacement_front_page_local_bridge_mirror_source"
        ],
        "replacement_front_page_local_bridge_bound": replacement[
            "replacement_front_page_local_bridge_bound"
        ],
        "replacement_front_page_local_bridge_missing": replacement[
            "replacement_front_page_local_bridge_missing"
        ],
        "replacement_front_page_local_required_field_count": replacement[
            "replacement_front_page_local_required_field_count"
        ],
        "replacement_front_page_local_required_fields_present": replacement[
            "replacement_front_page_local_required_fields_present"
        ],
        "replacement_front_page_local_missing_fields": replacement[
            "replacement_front_page_local_missing_fields"
        ],
        "replacement_front_page_local_required_method_count": replacement[
            "replacement_front_page_local_required_method_count"
        ],
        "replacement_front_page_local_required_methods_present": replacement[
            "replacement_front_page_local_required_methods_present"
        ],
        "replacement_front_page_local_missing_methods": replacement[
            "replacement_front_page_local_missing_methods"
        ],
        "replacement_front_page_local_directarray_fields_present": replacement[
            "replacement_front_page_local_directarray_fields_present"
        ],
        "replacement_front_page_local_counter_fields_present": replacement[
            "replacement_front_page_local_counter_fields_present"
        ],
        "replacement_front_page_local_acquire_release_methods_present": replacement[
            "replacement_front_page_local_acquire_release_methods_present"
        ],
        "replacement_front_page_local_lifecycle_methods_present": replacement[
            "replacement_front_page_local_lifecycle_methods_present"
        ],
        "replacement_front_page_local_typed_meta_matches_source": replacement[
            "replacement_front_page_local_typed_meta_matches_source"
        ],
        "replacement_front_page_local_same_owner_route_matches_source": replacement[
            "replacement_front_page_local_same_owner_route_matches_source"
        ],
        "replacement_front_page_local_no_remote_free_claim": replacement[
            "replacement_front_page_local_no_remote_free_claim"
        ],
        "replacement_front_producer_taxonomy_v0": replacement[
            "replacement_front_producer_taxonomy_v0"
        ],
        "replacement_front_producer": replacement["replacement_front_producer"],
        "replacement_front_backend_artifact": replacement[
            "replacement_front_backend_artifact"
        ],
        "replacement_front_source_truth": replacement["replacement_front_source_truth"],
        "replacement_front_python_template_c_semantic_ssot": replacement[
            "replacement_front_python_template_c_semantic_ssot"
        ],
        "replacement_front_python_template_c_retirement_required": replacement[
            "replacement_front_python_template_c_retirement_required"
        ],
        "replacement_front_mir_memop_enabled": replacement[
            "replacement_front_mir_memop_enabled"
        ],
        "replacement_front_mir_fastmem_region_enabled": replacement[
            "replacement_front_mir_fastmem_region_enabled"
        ],
        "replacement_front_mirbuilder_representation_only": replacement[
            "replacement_front_mirbuilder_representation_only"
        ],
        "replacement_front_mirbuilder_route_decision_count": replacement[
            "replacement_front_mirbuilder_route_decision_count"
        ],
        "replacement_front_producer_transition_state": replacement[
            "replacement_front_producer_transition_state"
        ],
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
