#!/usr/bin/env python3
"""Shared constants for FastMemory verifier checks."""

from __future__ import annotations

FAIL_FIELDS = [
    "fastmem_general_rawptr_type",
    "fastmem_general_deref_outside_region",
    "fastmem_general_pointer_arithmetic_outside_region",
    "fastmem_escape_count",
    "fastmem_metadata_ptr_escape_count",
    "fastmem_closure_capture_count",
    "fastmem_box_field_store_count",
    "fastmem_array_store_count",
    "fastmem_unverified_offset_load_count",
    "typed_page_meta_required_field_missing_count",
    "fastmem_contract_runtime_lookup_count",
    "fastmem_memop_unbalanced_region_count",
    "fastmem_memop_unclassified_count",
    "fastmem_forbidden_allocation_count",
    "fastmem_forbidden_safepoint_count",
    "fastmem_forbidden_await_count",
    "fastmem_forbidden_nowait_count",
    "fastmem_forbidden_call_count",
    "fastmem_type_abi_hot_lookup_count",
    "fastmem_provider_abi_crossing_count",
    "type_abi_hot_path_lookup_count",
    "provider_dispatch_hot_path",
    "page_map_bridge_type_abi_hot_lookup_count",
    "page_map_bridge_provider_abi_hot_dispatch_count",
    "free_path_page_lookup_range_scan_count",
    "alloc_owner_id_escape_count",
    "worker_id_escape_count",
    "worker_id_equals_os_thread_id_claim",
    "worker_id_equals_runtime_worker_id_claim",
    "worker_id_equals_hako_task_id_claim",
    "allocator_tls_arena_init_fail_count",
    "page_owner_count_mismatch",
    "page_owner_stale_generation_count",
    "page_owner_unowned_count",
    "allocator_owner_invalid_transition_count",
    "allocator_owner_stale_generation_count",
    "allocator_owner_reuse_without_generation_bump_count",
    "allocator_exiting_owner_page_claim_count",
    "allocator_abandoned_owner_local_free_count",
    "page_reclaimed_with_remote_candidates",
    "hako_source_thread_support_claim",
    "replacement_front_cross_thread_free_arena_registry_overflow_count",
    "safe_capability_wrapper_missing_count",
    "safe_capability_wrapper_rawptr_surface",
    "safe_capability_wrapper_deref_surface",
    "safe_capability_wrapper_escape_count",
    "fastmem_free_head_access_plan_incomplete_count",
]

FAIL_STRING_FIELDS = {
    "free_path_page_lookup_route": {"range_scan"},
}

PRODUCER_SLICE_EXPECTED_STRINGS = {
    "replacement_front_next_producer_slice": "layout_table_producer_pilot",
    "replacement_front_selected_memop_family": "layout_table",
    "replacement_front_selected_memop_kinds": "TableIndex,FieldLoad,FieldStore",
    "replacement_front_deferred_memop_family": "owner_runtime",
    "replacement_front_deferred_memop_kinds": "CurrentAllocOwnerId,OwnerEq",
}

PRODUCER_SLICE_EXPECTED_ZERO = (
    "replacement_front_selection_behavior_change",
    "replacement_front_selection_product_activation",
    "replacement_front_selection_bridge_retirement_allowed",
)
LAYOUT_TABLE_PRODUCER_EXPECTED_ZERO = (
    "memop_current_alloc_owner_id_lowered_count",
    "memop_owner_eq_lowered_count",
    "memop_atomic_remote_head_lowered_count",
    "fastmem_field_id_missing_count",
    "fastmem_table_id_missing_count",
    "fastmem_unverified_layout_access_count",
    "fastmem_table_index_unchecked_count",
    "fastmem_table_access_proof_incomplete_count",
    "fastmem_table_overflow_proof_missing_count",
    "fastmem_unknown_alignment_count",
    "fastmem_atomic_field_plain_store_count",
    "fastmem_layout_ref_escape_count",
    "fastmem_lowering_recomputed_layout_offset_count",
)
LAYOUT_TABLE_PRODUCER_EXPECTED_POSITIVE = (
    "memop_table_index_lowered_count",
    "memop_field_load_lowered_count",
    "memop_field_store_lowered_count",
)
OWNER_RUNTIME_PRODUCER_EXPECTED_ZERO = (
    "tls_backing_transfer_enabled",
    "allocator_owner_slot_reuse_enabled",
    "memop_atomic_remote_head_lowered_count",
    "type_abi_hot_lookup_count",
    "provider_abi_hot_dispatch_count",
    "product_activation",
    "hook_install",
    "global_allocator_claim",
    "winner_claim",
)
OWNER_RUNTIME_PRODUCER_EXPECTED_POSITIVE = (
    "memop_current_alloc_owner_id_lowered_count",
    "memop_owner_eq_lowered_count",
)
LOCAL_FREE_PRODUCER_EXPECTED_ZERO = (
    "memop_atomic_remote_head_lowered_count",
    "fastmem_local_free_access_plan_incomplete_count",
    "fastmem_free_head_access_plan_incomplete_count",
    "fastmem_local_free_head_plain_store_lowered_count",
    "fastmem_free_head_plain_store_lowered_count",
    "page_local_alloc_route_branch_claim",
    "page_local_alloc_route_cfg_lowering_enabled",
    "page_local_free_route_branch_claim",
    "page_local_free_route_cfg_lowering_enabled",
    "tls_backing_transfer_enabled",
    "allocator_owner_slot_reuse_enabled",
    "type_abi_hot_lookup_count",
    "provider_abi_hot_dispatch_count",
    "product_activation",
    "hook_install",
    "global_allocator_claim",
    "winner_claim",
)
LOCAL_FREE_PRODUCER_EXPECTED_POSITIVE = (
    "memop_table_index_lowered_count",
    "memop_field_load_lowered_count",
)
