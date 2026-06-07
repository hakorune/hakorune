#!/usr/bin/env python3
"""Tail helpers for FastMemory capability inventory report construction."""

from __future__ import annotations

from collections import defaultdict
from typing import Any

from fastmem_capability_inventory_common import (
    MIMALLOC_COVERAGE_DEFAULT_THRESHOLD,
    MIMALLOC_SAFETY_DEFAULT_THRESHOLD,
    MIMALLOC_SHAPE_COMPONENT_POINTS,
    MIMALLOC_SHAPE_DEFAULT_THRESHOLD,
    base_inventory,
    classify_remote_memory_order,
    first_subject_value,
    int_subject_value,
    speed_score_from_ratio,
)
from report_kv import first_value, int_value


def build_inventory_report(state: dict[str, Any]) -> dict[str, Any]:
    rows = state["rows"]
    replacement = defaultdict(int, state["replacement"])
    idx = state["idx"]

    free_path_route = state["free_path_route"]
    bridge_kind = state["bridge_kind"]
    remote_route = state["remote_route"]
    allocator_tls_enabled = state["allocator_tls_enabled"]
    owner_shadow_counters = state["owner_shadow_counters"]
    same_owner_route_enabled = state["same_owner_route_enabled"]
    smoke_remote_overflow_count = state["smoke_remote_overflow_count"]
    remote_free_push_count = state["remote_free_push_count"]
    remote_free_drain_count = state["remote_free_drain_count"]
    tls_arena_count = state["tls_arena_count"]
    atomic_remote_head_plan = state["atomic_remote_head_plan"]
    atomic_remote_enabled = state["atomic_remote_enabled"]
    page_map_bridge_present = state["page_map_bridge_present"]
    typed_meta_handle = state["typed_meta_handle"]
    typed_meta_fields = state["typed_meta_fields"]
    typed_meta_field_count = state["typed_meta_field_count"]
    typed_meta_missing_count = state["typed_meta_missing_count"]
    typed_meta_layout_verified = state["typed_meta_layout_verified"]
    typed_meta_layout_id = state["typed_meta_layout_id"]
    typed_meta_layout_hash = state["typed_meta_layout_hash"]
    alloc_owner_id_capability = state["alloc_owner_id_capability"]
    alloc_owner_id_kind = state["alloc_owner_id_kind"]
    alloc_owner_id_source = state["alloc_owner_id_source"]
    alloc_owner_id_width_bits = state["alloc_owner_id_width_bits"]
    alloc_owner_id_generation_enabled = state["alloc_owner_id_generation_enabled"]
    alloc_owner_id_zero_is_unowned = state["alloc_owner_id_zero_is_unowned"]
    worker_id_capability = state["worker_id_capability"]
    worker_id_kind = state["worker_id_kind"]
    worker_id_source = state["worker_id_source"]
    tls_arena_init_count = state["tls_arena_init_count"]
    tls_arena_live_count = state["tls_arena_live_count"]
    tls_arena_peak_count = state["tls_arena_peak_count"]
    page_owner_same_count = state["page_owner_same_count"]
    page_owner_remote_count = state["page_owner_remote_count"]
    page_owner_unowned_count = state["page_owner_unowned_count"]
    page_owner_stale_count = state["page_owner_stale_count"]
    page_owner_invalid_count = state["page_owner_invalid_count"]
    page_owner_check_count = state["page_owner_check_count"]
    same_owner_push_count = state["same_owner_push_count"]
    same_owner_fallback_default = state["same_owner_fallback_default"]
    remote_owner_candidate_default = state["remote_owner_candidate_default"]
    remote_owner_push_default = state["remote_owner_push_default"]
    remote_owner_fallback_default = state["remote_owner_fallback_default"]
    allocator_thread_exit_flush_count = state["allocator_thread_exit_flush_count"]
    allocator_abandoned_owner_count = state["allocator_abandoned_owner_count"]
    allocator_owner_lifecycle_state_machine = state["allocator_owner_lifecycle_state_machine"]
    allocator_owner_generation_enabled = state["allocator_owner_generation_enabled"]
    allocator_owner_id_kind = state["allocator_owner_id_kind"]
    allocator_owner_active_count = state["allocator_owner_active_count"]
    allocator_owner_exiting_flush_count = state["allocator_owner_exiting_flush_count"]
    allocator_owner_abandoned_count = state["allocator_owner_abandoned_count"]
    allocator_owner_reclaimed_count = state["allocator_owner_reclaimed_count"]
    remote_free_drain_supported = state["remote_free_drain_supported"]
    safe_wrapper_plan = int_subject_value(rows, idx, "safe_capability_wrapper_plan", 0)
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
        keeper_eligible = 0
    elif shape_score < shape_threshold:
        keeper_eligible = 0
    elif safety_score < safety_threshold:
        keeper_eligible = 0
    elif coverage_score < coverage_threshold:
        keeper_eligible = 0
    else:
        keeper_eligible = 1
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
    for blocker in str(replacement["replacement_front_product_shaped_bridge_missing"]).split(","):
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
    report.update(replacement)
    report.update(
        {
            "replacement_front_subowner": (
                "remote_free_queue" if atomic_remote_enabled else replacement["likely_next_owner"]
            ),
            "hako_hot_path_claim": replacement["hako_hot_path_claim"],
            "hako_source_thread_support_claim": replacement[
                "hako_source_thread_support_claim"
            ],
            "hako_source_hot_path_claim": 0,
            "mir_builder_hot_path_claim": 0,
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
            "allocator_owner_id_repr": "packed_u64_slot_generation"
            if allocator_owner_generation_enabled
            else "unknown",
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
                replacement["allocator_owner_invalid_transition_count"],
            ),
            "allocator_owner_stale_generation_count": int_subject_value(
                rows,
                idx,
                "allocator_owner_stale_generation_count",
                replacement["allocator_owner_stale_generation_count"],
            ),
            "allocator_owner_generation_bump_count": int_subject_value(
                rows,
                idx,
                "allocator_owner_generation_bump_count",
                replacement["allocator_owner_generation_bump_count"],
            ),
            "allocator_owner_reuse_without_generation_bump_count": int_subject_value(
                rows,
                idx,
                "allocator_owner_reuse_without_generation_bump_count",
                replacement["allocator_owner_reuse_without_generation_bump_count"],
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
            "worker_id_escape_count": worker_id_escape_count,
            "allocator_tls_arena_enabled": allocator_tls_enabled,
            "allocator_tls_arena_mode": "benchmark_c_tls" if alloc_owner_id_capability else "unknown",
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
            "allocator_tls_arena_count": tls_arena_count,
            "allocator_thread_exit_observed_count": int_subject_value(
                rows,
                idx,
                "allocator_thread_exit_observed_count",
                replacement["allocator_thread_exit_observed_count"],
            ),
            "allocator_thread_exit_flush_supported": int_subject_value(
                rows,
                idx,
                "allocator_thread_exit_flush_supported",
                replacement["allocator_thread_exit_flush_supported"],
            ),
            "allocator_thread_exit_flush_count": allocator_thread_exit_flush_count,
            "allocator_thread_exit_flush_page_count": int_subject_value(
                rows,
                idx,
                "allocator_thread_exit_flush_page_count",
                replacement["allocator_thread_exit_flush_page_count"],
            ),
            "allocator_thread_exit_local_free_drain_count": int_subject_value(
                rows,
                idx,
                "allocator_thread_exit_local_free_drain_count",
                replacement["allocator_thread_exit_local_free_drain_count"],
            ),
            "allocator_thread_exit_remote_candidate_seen_count": int_subject_value(
                rows,
                idx,
                "allocator_thread_exit_remote_candidate_seen_count",
                replacement["allocator_thread_exit_remote_candidate_seen_count"],
            ),
            "allocator_abandoned_owner_count": allocator_abandoned_owner_count,
            "allocator_abandoned_page_count": int_subject_value(
                rows,
                idx,
                "allocator_abandoned_page_count",
                replacement["allocator_abandoned_page_count"],
            ),
            "allocator_abandoned_live_page_count": int_subject_value(
                rows,
                idx,
                "allocator_abandoned_live_page_count",
                replacement["allocator_abandoned_live_page_count"],
            ),
            "allocator_abandoned_empty_page_count": int_subject_value(
                rows,
                idx,
                "allocator_abandoned_empty_page_count",
                replacement["allocator_abandoned_empty_page_count"],
            ),
            "allocator_abandoned_remote_candidate_count": int_subject_value(
                rows,
                idx,
                "allocator_abandoned_remote_candidate_count",
                replacement["allocator_abandoned_remote_candidate_count"],
            ),
            "allocator_abandoned_reclaim_attempt_count": int_subject_value(
                rows,
                idx,
                "allocator_abandoned_reclaim_attempt_count",
                replacement["allocator_abandoned_reclaim_attempt_count"],
            ),
            "allocator_abandoned_reclaim_success_count": int_subject_value(
                rows,
                idx,
                "allocator_abandoned_reclaim_success_count",
                replacement["allocator_abandoned_reclaim_success_count"],
            ),
            "allocator_abandoned_reclaim_blocked_count": int_subject_value(
                rows,
                idx,
                "allocator_abandoned_reclaim_blocked_count",
                replacement["allocator_abandoned_reclaim_blocked_count"],
            ),
            "allocator_abandoned_reclaim_blocked_remote_count": int_subject_value(
                rows,
                idx,
                "allocator_abandoned_reclaim_blocked_remote_count",
                replacement["allocator_abandoned_reclaim_blocked_remote_count"],
            ),
            "remote_candidate_unhandled_reclaim_block_count": int_subject_value(
                rows,
                idx,
                "remote_candidate_unhandled_reclaim_block_count",
                replacement["remote_candidate_unhandled_reclaim_block_count"],
            ),
            "page_reclaimed_with_remote_candidates": int_subject_value(
                rows,
                idx,
                "page_reclaimed_with_remote_candidates",
                replacement["page_reclaimed_with_remote_candidates"],
            ),
            "allocator_exiting_owner_page_claim_count": int_subject_value(
                rows,
                idx,
                "allocator_exiting_owner_page_claim_count",
                replacement["allocator_exiting_owner_page_claim_count"],
            ),
            "allocator_abandoned_owner_local_free_count": int_subject_value(
                rows,
                idx,
                "allocator_abandoned_owner_local_free_count",
                replacement["allocator_abandoned_owner_local_free_count"],
            ),
            "replacement_front_owner_shadow_counters": owner_shadow_counters,
            "page_owner_check_enabled": int_subject_value(
                rows, idx, "page_owner_check_enabled", owner_shadow_counters
            ),
            "page_owner_check_route": "page_meta_owner_worker_id"
            if owner_shadow_counters
            else "none",
            "page_owner_check_count": page_owner_check_count,
            "page_owner_same_count": page_owner_same_count,
            "page_owner_remote_count": page_owner_remote_count,
            "page_owner_unowned_count": page_owner_unowned_count,
            "page_owner_stale_generation_count": page_owner_stale_count,
            "page_owner_invalid_count": page_owner_invalid_count,
            "page_owner_count_mismatch": int(
                page_owner_check_count
                != (
                    page_owner_same_count
                    + page_owner_remote_count
                    + page_owner_unowned_count
                    + page_owner_stale_count
                    + page_owner_invalid_count
                )
            ),
            "same_owner_free_local_candidate_count": int_subject_value(
                rows, idx, "same_owner_free_local_candidate_count", page_owner_same_count
            ),
            "same_owner_free_local_route_enabled": same_owner_route_enabled,
            "replacement_front_same_owner_local_free_route": "page_meta_owner_local_free"
            if same_owner_route_enabled
            else "disabled",
            "same_owner_free_local_push_count": same_owner_push_count,
            "same_owner_free_local_fallback_count": int_subject_value(
                rows, idx, "same_owner_free_local_fallback_count", same_owner_fallback_default
            ),
            "remote_owner_free_remote_candidate_count": int_subject_value(
                rows, idx, "remote_owner_free_remote_candidate_count", remote_owner_candidate_default
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
            "atomic_remote_head_route": "page_remote_head_cas"
            if atomic_remote_head_plan
            else "none",
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
            "replacement_front_cross_thread_free_arena_registry_overflow_count": smoke_remote_overflow_count,
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
            "mimalloc_keeper_block_reason": (
                "not_candidate"
                if not keeper_candidate
                else "shape_below_threshold"
                if shape_score < shape_threshold
                else "safety_below_threshold"
                if safety_score < safety_threshold
                else "coverage_below_threshold"
                if coverage_score < coverage_threshold
                else "eligible"
            ),
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
            "replacement_front_product_shaped_bridge_evidence_ready": product_bridge_evidence_ready,
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
            "replacement_front_product_shaped_bridge_no_host_passthrough": product_bridge_no_host_passthrough,
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
            "replacement_front_producer_slice_selection_v0": replacement[
                "replacement_front_producer_slice_selection_v0"
            ],
            "replacement_front_next_producer_slice": replacement[
                "replacement_front_next_producer_slice"
            ],
            "replacement_front_selected_memop_family": replacement[
                "replacement_front_selected_memop_family"
            ],
            "replacement_front_selected_memop_kinds": replacement[
                "replacement_front_selected_memop_kinds"
            ],
            "replacement_front_deferred_memop_family": replacement[
                "replacement_front_deferred_memop_family"
            ],
            "replacement_front_deferred_memop_kinds": replacement[
                "replacement_front_deferred_memop_kinds"
            ],
            "replacement_front_selection_behavior_change": replacement[
                "replacement_front_selection_behavior_change"
            ],
            "replacement_front_selection_product_activation": replacement[
                "replacement_front_selection_product_activation"
            ],
            "replacement_front_selection_bridge_retirement_allowed": replacement[
                "replacement_front_selection_bridge_retirement_allowed"
            ],
            "mir_fmem_008b_layout_table_producer_pilot": replacement[
                "mir_fmem_008b_layout_table_producer_pilot"
            ],
            "memop_table_index_lowered_count": replacement[
                "memop_table_index_lowered_count"
            ],
            "memop_field_load_lowered_count": replacement["memop_field_load_lowered_count"],
            "memop_field_store_lowered_count": replacement["memop_field_store_lowered_count"],
            "memop_current_alloc_owner_id_lowered_count": replacement[
                "memop_current_alloc_owner_id_lowered_count"
            ],
            "memop_owner_eq_lowered_count": replacement["memop_owner_eq_lowered_count"],
            "memop_atomic_remote_head_lowered_count": replacement[
                "memop_atomic_remote_head_lowered_count"
            ],
            "fastmem_verified_mem_access_plan_count": replacement[
                "fastmem_verified_mem_access_plan_count"
            ],
            "fastmem_verified_field_access_count": replacement[
                "fastmem_verified_field_access_count"
            ],
            "fastmem_verified_table_access_count": replacement[
                "fastmem_verified_table_access_count"
            ],
            "fastmem_field_id_missing_count": replacement["fastmem_field_id_missing_count"],
            "fastmem_table_id_missing_count": replacement["fastmem_table_id_missing_count"],
            "fastmem_unverified_layout_access_count": replacement[
                "fastmem_unverified_layout_access_count"
            ],
            "fastmem_table_index_unchecked_count": replacement[
                "fastmem_table_index_unchecked_count"
            ],
            "fastmem_table_access_proof_incomplete_count": replacement[
                "fastmem_table_access_proof_incomplete_count"
            ],
            "fastmem_table_overflow_proof_missing_count": replacement[
                "fastmem_table_overflow_proof_missing_count"
            ],
            "fastmem_unknown_alignment_count": replacement["fastmem_unknown_alignment_count"],
            "fastmem_atomic_field_plain_store_count": replacement[
                "fastmem_atomic_field_plain_store_count"
            ],
            "fastmem_layout_ref_escape_count": replacement["fastmem_layout_ref_escape_count"],
            "fastmem_lowering_recomputed_layout_offset_count": replacement[
                "fastmem_lowering_recomputed_layout_offset_count"
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
            "summary": "ok" if replacement["benchmark_front_class"] else "failed",
        }
    )

    return report
