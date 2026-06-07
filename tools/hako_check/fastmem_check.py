#!/usr/bin/env python3
"""Check FastMemory capability inventory reports.

This is a verifier adapter over fastmem inventory fields. It fails when a
contract/runtime report contains unclassified MemOps, forbidden operations,
escaping memory values, or Type ABI / Provider ABI hot-path crossings.
"""

from __future__ import annotations

import argparse
import subprocess
import sys
from pathlib import Path
from typing import Any

from report_kv import read_kv

ROOT = Path(__file__).resolve().parents[2]
INVENTORY = ROOT / "tools" / "hako_check" / "fastmem_capability_inventory.py"

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
ATOMIC_REMOTE_HEAD_PREFLIGHT_EXPECTED_ZERO = (
    "atomic_remote_head_cas_lowering_open",
    "atomic_remote_head_push_lowerable_count",
    "atomic_remote_head_remote_owner_missing_count",
    "atomic_remote_head_block_next_missing_count",
    "memop_atomic_remote_head_lowered_count",
    "tls_backing_transfer_enabled",
    "allocator_owner_slot_reuse_enabled",
    "type_abi_hot_lookup_count",
    "provider_abi_hot_dispatch_count",
    "product_activation",
    "hook_install",
    "global_allocator_claim",
    "winner_claim",
)
ATOMIC_REMOTE_HEAD_PREFLIGHT_EXPECTED_POSITIVE = (
    "atomic_remote_head_cas_lowering_selected",
    "atomic_remote_head_push_plan_count",
    "atomic_remote_head_remote_owner_required",
    "atomic_remote_head_block_next_required",
    "atomic_remote_head_access_resolved_count",
    "fastmem_remote_owner_source_assume_count",
    "fastmem_remote_free_block_next_source_assume_count",
)
ATOMIC_REMOTE_HEAD_PRODUCER_EXPECTED_ZERO = (
    "atomic_remote_head_remote_owner_missing_count",
    "atomic_remote_head_block_next_missing_count",
    "tls_backing_transfer_enabled",
    "allocator_owner_slot_reuse_enabled",
    "type_abi_hot_lookup_count",
    "provider_abi_hot_dispatch_count",
    "product_activation",
    "hook_install",
    "global_allocator_claim",
    "winner_claim",
)
ATOMIC_REMOTE_HEAD_PRODUCER_EXPECTED_POSITIVE = (
    "atomic_remote_head_cas_lowering_selected",
    "atomic_remote_head_cas_lowering_open",
    "atomic_remote_head_push_plan_count",
    "atomic_remote_head_push_lowerable_count",
    "atomic_remote_head_remote_owner_required",
    "atomic_remote_head_block_next_required",
    "atomic_remote_head_access_resolved_count",
    "fastmem_remote_owner_source_assume_count",
    "fastmem_remote_free_block_next_source_assume_count",
    "memop_atomic_remote_head_lowered_count",
)
ATOMIC_REMOTE_HEAD_RETRY_PREFLIGHT_EXPECTED_ZERO = (
    "atomic_remote_head_retry_policy_open",
    "atomic_remote_head_retry_lowered_count",
    "atomic_remote_head_drain_open",
    "remote_owner_branch_routing_open",
    "atomic_remote_head_remote_owner_missing_count",
    "atomic_remote_head_block_next_missing_count",
    "tls_backing_transfer_enabled",
    "allocator_owner_slot_reuse_enabled",
    "type_abi_hot_lookup_count",
    "provider_abi_hot_dispatch_count",
    "product_activation",
    "hook_install",
    "global_allocator_claim",
    "winner_claim",
)
ATOMIC_REMOTE_HEAD_RETRY_PREFLIGHT_EXPECTED_POSITIVE = (
    "fastmem_atomic_remote_head_retry_preflight",
    "atomic_remote_head_retry_policy_selected",
    "atomic_remote_head_retry_attempt_limit",
    "atomic_remote_head_cas_lowering_selected",
    "atomic_remote_head_cas_lowering_open",
    "atomic_remote_head_push_plan_count",
    "atomic_remote_head_push_lowerable_count",
    "atomic_remote_head_remote_owner_required",
    "atomic_remote_head_block_next_required",
    "atomic_remote_head_access_resolved_count",
    "fastmem_remote_owner_source_assume_count",
    "fastmem_remote_free_block_next_source_assume_count",
    "memop_atomic_remote_head_lowered_count",
)
ATOMIC_REMOTE_HEAD_RETRY_PRODUCER_EXPECTED_ZERO = (
    "atomic_remote_head_drain_open",
    "remote_owner_branch_routing_open",
    "atomic_remote_head_remote_owner_missing_count",
    "atomic_remote_head_block_next_missing_count",
    "tls_backing_transfer_enabled",
    "allocator_owner_slot_reuse_enabled",
    "type_abi_hot_lookup_count",
    "provider_abi_hot_dispatch_count",
    "product_activation",
    "hook_install",
    "global_allocator_claim",
    "winner_claim",
)
ATOMIC_REMOTE_HEAD_RETRY_PRODUCER_EXPECTED_POSITIVE = (
    "fastmem_atomic_remote_head_retry_producer_pilot",
    "atomic_remote_head_retry_policy_selected",
    "atomic_remote_head_retry_policy_open",
    "atomic_remote_head_retry_attempt_limit",
    "atomic_remote_head_retry_lowered_count",
    "atomic_remote_head_cas_lowering_selected",
    "atomic_remote_head_cas_lowering_open",
    "atomic_remote_head_push_plan_count",
    "atomic_remote_head_push_lowerable_count",
    "atomic_remote_head_remote_owner_required",
    "atomic_remote_head_block_next_required",
    "atomic_remote_head_access_resolved_count",
    "fastmem_remote_owner_source_assume_count",
    "fastmem_remote_free_block_next_source_assume_count",
    "memop_atomic_remote_head_lowered_count",
)
ATOMIC_REMOTE_HEAD_DRAIN_PREFLIGHT_EXPECTED_ZERO = (
    "atomic_remote_head_drain_open",
    "atomic_remote_head_drain_lowerable_count",
    "atomic_remote_head_drain_lowered_count",
    "remote_owner_branch_routing_open",
    "atomic_remote_head_remote_owner_missing_count",
    "atomic_remote_head_block_next_missing_count",
    "tls_backing_transfer_enabled",
    "allocator_owner_slot_reuse_enabled",
    "type_abi_hot_lookup_count",
    "provider_abi_hot_dispatch_count",
    "product_activation",
    "hook_install",
    "global_allocator_claim",
    "winner_claim",
)
ATOMIC_REMOTE_HEAD_DRAIN_PREFLIGHT_EXPECTED_POSITIVE = (
    "fastmem_atomic_remote_head_drain_preflight",
    "atomic_remote_head_drain_selected",
    "atomic_remote_head_retry_policy_open",
    "atomic_remote_head_retry_attempt_limit",
    "atomic_remote_head_retry_lowered_count",
    "atomic_remote_head_cas_lowering_selected",
    "atomic_remote_head_cas_lowering_open",
    "atomic_remote_head_push_plan_count",
    "atomic_remote_head_push_lowerable_count",
    "atomic_remote_head_remote_owner_required",
    "atomic_remote_head_block_next_required",
    "atomic_remote_head_access_resolved_count",
    "fastmem_remote_owner_source_assume_count",
    "fastmem_remote_free_block_next_source_assume_count",
    "memop_atomic_remote_head_lowered_count",
)
ATOMIC_REMOTE_HEAD_DRAIN_EXCHANGE_SELECTION_EXPECTED_ZERO = (
    "atomic_remote_head_drain_open",
    "atomic_remote_head_drain_lowerable_count",
    "atomic_remote_head_drain_lowered_count",
    "atomic_remote_head_drain_to_local_route_open",
    "remote_owner_branch_routing_open",
    "atomic_remote_head_remote_owner_missing_count",
    "atomic_remote_head_block_next_missing_count",
    "tls_backing_transfer_enabled",
    "allocator_owner_slot_reuse_enabled",
    "type_abi_hot_lookup_count",
    "provider_abi_hot_dispatch_count",
    "product_activation",
    "hook_install",
    "global_allocator_claim",
    "winner_claim",
)
ATOMIC_REMOTE_HEAD_DRAIN_EXCHANGE_SELECTION_EXPECTED_POSITIVE = (
    "fastmem_atomic_remote_head_drain_exchange_selection",
    "atomic_remote_head_drain_selected",
    "atomic_remote_head_drain_exchange_selected",
    "atomic_remote_head_retry_policy_open",
    "atomic_remote_head_retry_attempt_limit",
    "atomic_remote_head_retry_lowered_count",
    "atomic_remote_head_cas_lowering_selected",
    "atomic_remote_head_cas_lowering_open",
    "atomic_remote_head_push_plan_count",
    "atomic_remote_head_push_lowerable_count",
    "atomic_remote_head_remote_owner_required",
    "atomic_remote_head_block_next_required",
    "atomic_remote_head_access_resolved_count",
    "fastmem_remote_owner_source_assume_count",
    "fastmem_remote_free_block_next_source_assume_count",
    "memop_atomic_remote_head_lowered_count",
)
ATOMIC_REMOTE_HEAD_DRAIN_EXCHANGE_PRODUCER_EXPECTED_ZERO = (
    "atomic_remote_head_drain_to_local_route_open",
    "remote_owner_branch_routing_open",
    "atomic_remote_head_remote_owner_missing_count",
    "atomic_remote_head_block_next_missing_count",
    "tls_backing_transfer_enabled",
    "allocator_owner_slot_reuse_enabled",
    "type_abi_hot_lookup_count",
    "provider_abi_hot_dispatch_count",
    "product_activation",
    "hook_install",
    "global_allocator_claim",
    "winner_claim",
)
ATOMIC_REMOTE_HEAD_DRAIN_EXCHANGE_PRODUCER_EXPECTED_POSITIVE = (
    "fastmem_atomic_remote_head_drain_exchange_producer_pilot",
    "atomic_remote_head_drain_selected",
    "atomic_remote_head_drain_exchange_selected",
    "atomic_remote_head_drain_open",
    "atomic_remote_head_drain_plan_count",
    "atomic_remote_head_drain_lowerable_count",
    "atomic_remote_head_drain_lowered_count",
    "atomic_remote_head_access_resolved_count",
    "memop_atomic_remote_head_drain_count",
)
ATOMIC_REMOTE_HEAD_DRAIN_TO_LOCAL_SELECTION_EXPECTED_ZERO = (
    "atomic_remote_head_drain_to_local_route_open",
    "remote_owner_branch_routing_open",
    "atomic_remote_head_remote_owner_missing_count",
    "atomic_remote_head_block_next_missing_count",
    "tls_backing_transfer_enabled",
    "allocator_owner_slot_reuse_enabled",
    "type_abi_hot_lookup_count",
    "provider_abi_hot_dispatch_count",
    "product_activation",
    "hook_install",
    "global_allocator_claim",
    "winner_claim",
)
ATOMIC_REMOTE_HEAD_DRAIN_TO_LOCAL_SELECTION_EXPECTED_POSITIVE = (
    "fastmem_atomic_remote_head_drain_to_local_route_selection",
    "atomic_remote_head_drain_selected",
    "atomic_remote_head_drain_exchange_selected",
    "atomic_remote_head_drain_open",
    "atomic_remote_head_drain_plan_count",
    "atomic_remote_head_drain_lowerable_count",
    "atomic_remote_head_drain_lowered_count",
    "atomic_remote_head_drain_to_local_route_selected",
    "atomic_remote_head_access_resolved_count",
    "memop_atomic_remote_head_drain_count",
)
ATOMIC_REMOTE_HEAD_DRAIN_TO_LOCAL_PRODUCER_EXPECTED_ZERO = (
    "remote_owner_branch_routing_open",
    "atomic_remote_head_remote_owner_missing_count",
    "atomic_remote_head_block_next_missing_count",
    "tls_backing_transfer_enabled",
    "allocator_owner_slot_reuse_enabled",
    "type_abi_hot_lookup_count",
    "provider_abi_hot_dispatch_count",
    "product_activation",
    "hook_install",
    "global_allocator_claim",
    "winner_claim",
)
ATOMIC_REMOTE_HEAD_DRAIN_TO_LOCAL_PRODUCER_EXPECTED_POSITIVE = (
    "fastmem_atomic_remote_head_drain_to_local_route_producer_pilot",
    "atomic_remote_head_drain_selected",
    "atomic_remote_head_drain_exchange_selected",
    "atomic_remote_head_drain_open",
    "atomic_remote_head_drain_plan_count",
    "atomic_remote_head_drain_lowerable_count",
    "atomic_remote_head_drain_lowered_count",
    "atomic_remote_head_drain_to_local_route_selected",
    "atomic_remote_head_drain_to_local_route_open",
    "atomic_remote_head_drain_to_local_route_producer_pilot",
    "atomic_remote_head_access_resolved_count",
    "memop_atomic_remote_head_drain_count",
)
ATOMIC_REMOTE_HEAD_DRAIN_LOCAL_LIST_MUTATION_PREFLIGHT_EXPECTED_ZERO = (
    "atomic_remote_head_drain_local_list_mutation_open",
    "remote_owner_branch_routing_open",
    "atomic_remote_head_remote_owner_missing_count",
    "atomic_remote_head_block_next_missing_count",
    "tls_backing_transfer_enabled",
    "allocator_owner_slot_reuse_enabled",
    "type_abi_hot_lookup_count",
    "provider_abi_hot_dispatch_count",
    "product_activation",
    "hook_install",
    "global_allocator_claim",
    "winner_claim",
)
ATOMIC_REMOTE_HEAD_DRAIN_LOCAL_LIST_MUTATION_PREFLIGHT_EXPECTED_POSITIVE = (
    "fastmem_atomic_remote_head_drain_local_list_mutation_preflight",
    "atomic_remote_head_drain_selected",
    "atomic_remote_head_drain_exchange_selected",
    "atomic_remote_head_drain_open",
    "atomic_remote_head_drain_plan_count",
    "atomic_remote_head_drain_lowerable_count",
    "atomic_remote_head_drain_lowered_count",
    "atomic_remote_head_drain_to_local_route_selected",
    "atomic_remote_head_drain_local_list_mutation_selected",
    "atomic_remote_head_access_resolved_count",
    "memop_atomic_remote_head_drain_count",
)
ATOMIC_REMOTE_HEAD_DRAIN_LOCAL_LIST_MUTATION_PROOF_EXPECTED_ZERO = (
    "atomic_remote_head_drain_local_list_mutation_open",
    "atomic_remote_head_drain_local_list_token_escape_count",
    "remote_owner_branch_routing_open",
    "atomic_remote_head_remote_owner_missing_count",
    "atomic_remote_head_block_next_missing_count",
    "tls_backing_transfer_enabled",
    "allocator_owner_slot_reuse_enabled",
    "type_abi_hot_lookup_count",
    "provider_abi_hot_dispatch_count",
    "product_activation",
    "hook_install",
    "global_allocator_claim",
    "winner_claim",
)
ATOMIC_REMOTE_HEAD_DRAIN_LOCAL_LIST_MUTATION_PROOF_EXPECTED_POSITIVE = (
    "fastmem_atomic_remote_head_drain_local_list_mutation_proof",
    "atomic_remote_head_drain_selected",
    "atomic_remote_head_drain_exchange_selected",
    "atomic_remote_head_drain_open",
    "atomic_remote_head_drain_plan_count",
    "atomic_remote_head_drain_lowerable_count",
    "atomic_remote_head_drain_lowered_count",
    "atomic_remote_head_drain_to_local_route_selected",
    "atomic_remote_head_drain_to_local_route_open",
    "atomic_remote_head_drain_local_list_mutation_selected",
    "atomic_remote_head_drain_local_list_head_class_resolved",
    "atomic_remote_head_access_resolved_count",
    "memop_atomic_remote_head_drain_count",
)
ATOMIC_REMOTE_HEAD_DRAIN_LOCAL_LIST_MUTATION_VOCABULARY_EXPECTED_ZERO = (
    "atomic_remote_head_drain_local_list_mutation_open",
    "atomic_remote_head_drain_local_list_token_escape_count",
    "atomic_remote_head_drain_local_list_mutation_lowerable_count",
    "remote_owner_branch_routing_open",
    "atomic_remote_head_remote_owner_missing_count",
    "atomic_remote_head_block_next_missing_count",
    "tls_backing_transfer_enabled",
    "allocator_owner_slot_reuse_enabled",
    "type_abi_hot_lookup_count",
    "provider_abi_hot_dispatch_count",
    "product_activation",
    "hook_install",
    "global_allocator_claim",
    "winner_claim",
)
ATOMIC_REMOTE_HEAD_DRAIN_LOCAL_LIST_MUTATION_VOCABULARY_EXPECTED_POSITIVE = (
    "fastmem_atomic_remote_head_drain_local_list_mutation_vocabulary_preflight",
    "fastmem_memop_drain_remote_list_to_local_count",
    "atomic_remote_head_drain_selected",
    "atomic_remote_head_drain_exchange_selected",
    "atomic_remote_head_drain_open",
    "atomic_remote_head_drain_plan_count",
    "atomic_remote_head_drain_lowerable_count",
    "atomic_remote_head_drain_lowered_count",
    "atomic_remote_head_drain_to_local_route_selected",
    "atomic_remote_head_drain_local_list_mutation_selected",
    "atomic_remote_head_drain_local_list_head_class_resolved",
    "atomic_remote_head_access_resolved_count",
    "memop_atomic_remote_head_drain_count",
)
ATOMIC_REMOTE_HEAD_DRAIN_LOCAL_LIST_MUTATION_VERIFIER_EXPECTED_ZERO = (
    "atomic_remote_head_drain_local_list_mutation_open",
    "atomic_remote_head_drain_local_list_token_escape_count",
    "remote_owner_branch_routing_open",
    "atomic_remote_head_remote_owner_missing_count",
    "atomic_remote_head_block_next_missing_count",
    "tls_backing_transfer_enabled",
    "allocator_owner_slot_reuse_enabled",
    "type_abi_hot_lookup_count",
    "provider_abi_hot_dispatch_count",
    "product_activation",
    "hook_install",
    "global_allocator_claim",
    "winner_claim",
)
ATOMIC_REMOTE_HEAD_DRAIN_LOCAL_LIST_MUTATION_VERIFIER_EXPECTED_POSITIVE = (
    "fastmem_atomic_remote_head_drain_local_list_mutation_verifier_preconditions",
    "fastmem_memop_drain_remote_list_to_local_count",
    "drain_remote_list_to_local_plan_count",
    "drain_remote_list_to_local_token_provenance_valid",
    "drain_remote_list_to_local_page_operand_valid",
    "drain_remote_list_to_local_head_class_resolved",
    "drain_remote_list_to_local_lowerable_count",
    "atomic_remote_head_drain_local_list_mutation_lowerable_count",
    "atomic_remote_head_drain_selected",
    "atomic_remote_head_drain_exchange_selected",
    "atomic_remote_head_drain_open",
    "atomic_remote_head_drain_plan_count",
    "atomic_remote_head_drain_lowerable_count",
    "atomic_remote_head_drain_lowered_count",
    "atomic_remote_head_drain_to_local_route_selected",
    "atomic_remote_head_drain_to_local_route_open",
    "atomic_remote_head_drain_local_list_mutation_selected",
    "atomic_remote_head_drain_local_list_head_class_resolved",
    "atomic_remote_head_access_resolved_count",
    "memop_atomic_remote_head_drain_count",
)
ATOMIC_REMOTE_HEAD_DRAIN_LOCAL_LIST_MUTATION_LOWERING_EXPECTED_ZERO = (
    "atomic_remote_head_drain_local_list_token_escape_count",
    "remote_owner_branch_routing_open",
    "atomic_remote_head_remote_owner_missing_count",
    "atomic_remote_head_block_next_missing_count",
    "tls_backing_transfer_enabled",
    "allocator_owner_slot_reuse_enabled",
    "type_abi_hot_lookup_count",
    "provider_abi_hot_dispatch_count",
    "product_activation",
    "hook_install",
    "global_allocator_claim",
    "winner_claim",
)
ATOMIC_REMOTE_HEAD_DRAIN_LOCAL_LIST_MUTATION_LOWERING_EXPECTED_POSITIVE = (
    "fastmem_atomic_remote_head_drain_local_list_mutation_lowering_producer_pilot",
    "fastmem_memop_drain_remote_list_to_local_count",
    "drain_remote_list_to_local_plan_count",
    "drain_remote_list_to_local_token_provenance_valid",
    "drain_remote_list_to_local_page_operand_valid",
    "drain_remote_list_to_local_head_class_resolved",
    "drain_remote_list_to_local_lowerable_count",
    "atomic_remote_head_drain_local_list_mutation_lowerable_count",
    "atomic_remote_head_drain_local_list_mutation_lowered_count",
    "atomic_remote_head_drain_local_list_mutation_open",
    "atomic_remote_head_drain_selected",
    "atomic_remote_head_drain_exchange_selected",
    "atomic_remote_head_drain_open",
    "atomic_remote_head_drain_plan_count",
    "atomic_remote_head_drain_lowerable_count",
    "atomic_remote_head_drain_lowered_count",
    "atomic_remote_head_drain_to_local_route_selected",
    "atomic_remote_head_drain_local_list_mutation_selected",
    "atomic_remote_head_drain_local_list_head_class_resolved",
    "atomic_remote_head_access_resolved_count",
    "memop_atomic_remote_head_drain_count",
)
REMOTE_OWNER_BRANCH_ROUTING_PREFLIGHT_EXPECTED_ZERO = (
    "remote_owner_branch_routing_open",
    "remote_owner_branch_routing_lowered_count",
    "atomic_remote_head_remote_owner_missing_count",
    "atomic_remote_head_block_next_missing_count",
    "tls_backing_transfer_enabled",
    "allocator_owner_slot_reuse_enabled",
    "type_abi_hot_lookup_count",
    "provider_abi_hot_dispatch_count",
    "product_activation",
    "hook_install",
    "global_allocator_claim",
    "winner_claim",
)
REMOTE_OWNER_BRANCH_ROUTING_PREFLIGHT_EXPECTED_POSITIVE = (
    "fastmem_remote_owner_branch_routing_preflight",
    "remote_owner_branch_routing_selected",
    "remote_owner_branch_routing_preflight_requires_branch_cfg_row",
    "fastmem_memop_drain_remote_list_to_local_count",
    "drain_remote_list_to_local_plan_count",
    "drain_remote_list_to_local_token_provenance_valid",
    "drain_remote_list_to_local_page_operand_valid",
    "drain_remote_list_to_local_head_class_resolved",
    "drain_remote_list_to_local_lowerable_count",
    "atomic_remote_head_drain_local_list_mutation_lowerable_count",
    "atomic_remote_head_drain_local_list_mutation_lowered_count",
    "atomic_remote_head_drain_local_list_mutation_open",
    "atomic_remote_head_drain_selected",
    "atomic_remote_head_drain_exchange_selected",
    "atomic_remote_head_drain_open",
    "atomic_remote_head_drain_plan_count",
    "atomic_remote_head_drain_lowerable_count",
    "atomic_remote_head_drain_lowered_count",
    "atomic_remote_head_drain_to_local_route_selected",
    "atomic_remote_head_drain_local_list_mutation_selected",
    "atomic_remote_head_drain_local_list_head_class_resolved",
    "atomic_remote_head_access_resolved_count",
    "memop_atomic_remote_head_drain_count",
)
REMOTE_OWNER_BRANCH_ROUTING_LOWERING_PREFLIGHT_EXPECTED_ZERO = (
    "remote_owner_branch_routing_open",
    "remote_owner_branch_routing_lowered_count",
    "atomic_remote_head_remote_owner_missing_count",
    "atomic_remote_head_block_next_missing_count",
    "tls_backing_transfer_enabled",
    "allocator_owner_slot_reuse_enabled",
    "type_abi_hot_lookup_count",
    "provider_abi_hot_dispatch_count",
    "product_activation",
    "hook_install",
    "global_allocator_claim",
    "winner_claim",
)
REMOTE_OWNER_BRANCH_ROUTING_LOWERING_PREFLIGHT_EXPECTED_POSITIVE = (
    "fastmem_remote_owner_branch_routing_lowering_preflight",
    "remote_owner_branch_routing_selected",
    "remote_owner_branch_routing_lowering_selected",
    "remote_owner_branch_routing_preflight_requires_branch_cfg_row",
    "fastmem_memop_drain_remote_list_to_local_count",
    "drain_remote_list_to_local_plan_count",
    "drain_remote_list_to_local_token_provenance_valid",
    "drain_remote_list_to_local_page_operand_valid",
    "drain_remote_list_to_local_head_class_resolved",
    "drain_remote_list_to_local_lowerable_count",
    "atomic_remote_head_drain_local_list_mutation_lowerable_count",
    "atomic_remote_head_drain_local_list_mutation_lowered_count",
    "atomic_remote_head_drain_local_list_mutation_open",
    "atomic_remote_head_drain_selected",
    "atomic_remote_head_drain_exchange_selected",
    "atomic_remote_head_drain_open",
    "atomic_remote_head_drain_plan_count",
    "atomic_remote_head_drain_lowerable_count",
    "atomic_remote_head_drain_lowered_count",
    "atomic_remote_head_drain_to_local_route_selected",
    "atomic_remote_head_drain_local_list_mutation_selected",
    "atomic_remote_head_drain_local_list_head_class_resolved",
    "atomic_remote_head_access_resolved_count",
    "memop_atomic_remote_head_drain_count",
)
REMOTE_OWNER_BRANCH_ROUTING_LOWERING_EXPECTED_ZERO = (
    "remote_owner_branch_routing_preflight_requires_branch_cfg_row",
    "atomic_remote_head_remote_owner_missing_count",
    "atomic_remote_head_block_next_missing_count",
    "tls_backing_transfer_enabled",
    "allocator_owner_slot_reuse_enabled",
    "type_abi_hot_lookup_count",
    "provider_abi_hot_dispatch_count",
    "product_activation",
    "hook_install",
    "global_allocator_claim",
    "winner_claim",
    "page_local_alloc_route_branch_claim",
    "page_local_alloc_route_cfg_lowering_enabled",
    "page_local_free_route_branch_claim",
    "page_local_free_route_cfg_lowering_enabled",
)
REMOTE_OWNER_BRANCH_ROUTING_LOWERING_EXPECTED_POSITIVE = (
    "fastmem_remote_owner_branch_routing_lowering_producer_pilot",
    "remote_owner_branch_routing_selected",
    "remote_owner_branch_routing_lowering_selected",
    "remote_owner_branch_routing_open",
    "remote_owner_branch_routing_lowered_count",
    "memop_current_alloc_owner_id_lowered_count",
    "memop_owner_eq_lowered_count",
    "fastmem_memop_drain_remote_list_to_local_count",
    "drain_remote_list_to_local_plan_count",
    "drain_remote_list_to_local_token_provenance_valid",
    "drain_remote_list_to_local_page_operand_valid",
    "drain_remote_list_to_local_head_class_resolved",
    "drain_remote_list_to_local_lowerable_count",
    "atomic_remote_head_drain_local_list_mutation_lowerable_count",
    "atomic_remote_head_drain_local_list_mutation_lowered_count",
    "atomic_remote_head_drain_local_list_mutation_open",
    "atomic_remote_head_drain_selected",
    "atomic_remote_head_drain_exchange_selected",
    "atomic_remote_head_drain_open",
    "atomic_remote_head_drain_plan_count",
    "atomic_remote_head_drain_lowerable_count",
    "atomic_remote_head_drain_lowered_count",
    "atomic_remote_head_drain_to_local_route_selected",
    "atomic_remote_head_drain_local_list_mutation_selected",
    "atomic_remote_head_drain_local_list_head_class_resolved",
    "atomic_remote_head_access_resolved_count",
    "memop_atomic_remote_head_drain_count",
)
REMOTE_OWNER_BRANCH_ROUTE_BODY_PREFLIGHT_EXPECTED_ZERO = (
    "remote_owner_branch_route_body_open",
    "page_local_alloc_route_branch_claim",
    "page_local_alloc_route_cfg_lowering_enabled",
    "page_local_free_route_branch_claim",
    "page_local_free_route_cfg_lowering_enabled",
    "atomic_remote_head_remote_owner_missing_count",
    "atomic_remote_head_block_next_missing_count",
    "tls_backing_transfer_enabled",
    "allocator_owner_slot_reuse_enabled",
    "type_abi_hot_lookup_count",
    "provider_abi_hot_dispatch_count",
    "product_activation",
    "hook_install",
    "global_allocator_claim",
    "winner_claim",
)
REMOTE_OWNER_BRANCH_ROUTE_BODY_PREFLIGHT_EXPECTED_POSITIVE = (
    "fastmem_remote_owner_branch_route_body_preflight",
    "remote_owner_branch_routing_selected",
    "remote_owner_branch_routing_lowering_selected",
    "remote_owner_branch_routing_open",
    "remote_owner_branch_routing_lowered_count",
    "remote_owner_branch_route_body_selected",
    "memop_current_alloc_owner_id_lowered_count",
    "memop_owner_eq_lowered_count",
    "atomic_remote_head_drain_local_list_mutation_lowered_count",
)
FASTMEM_BRANCH_CFG_PREFLIGHT_EXPECTED_ZERO = (
    "fastmem_branch_cfg_open",
    "fastmem_branch_cfg_lowered_count",
    "remote_owner_branch_route_body_open",
    "page_local_alloc_route_branch_claim",
    "page_local_alloc_route_cfg_lowering_enabled",
    "page_local_free_route_branch_claim",
    "page_local_free_route_cfg_lowering_enabled",
    "atomic_remote_head_remote_owner_missing_count",
    "atomic_remote_head_block_next_missing_count",
    "tls_backing_transfer_enabled",
    "allocator_owner_slot_reuse_enabled",
    "type_abi_hot_lookup_count",
    "provider_abi_hot_dispatch_count",
    "product_activation",
    "hook_install",
    "global_allocator_claim",
    "winner_claim",
)
FASTMEM_BRANCH_CFG_PREFLIGHT_EXPECTED_POSITIVE = (
    "fastmem_branch_cfg_preflight",
    "fastmem_branch_cfg_selected",
    "fastmem_branch_cfg_closed_guard",
    "remote_owner_branch_routing_selected",
    "remote_owner_branch_routing_lowering_selected",
    "remote_owner_branch_routing_open",
    "remote_owner_branch_routing_lowered_count",
    "remote_owner_branch_route_body_selected",
    "memop_current_alloc_owner_id_lowered_count",
    "memop_owner_eq_lowered_count",
    "atomic_remote_head_drain_local_list_mutation_lowered_count",
)
FASTMEM_BRANCH_CFG_LOWERING_PREFLIGHT_EXPECTED_ZERO = (
    "fastmem_branch_cfg_open",
    "fastmem_branch_cfg_lowered_count",
    "remote_owner_branch_route_body_open",
    "page_local_alloc_route_branch_claim",
    "page_local_alloc_route_cfg_lowering_enabled",
    "page_local_free_route_branch_claim",
    "page_local_free_route_cfg_lowering_enabled",
    "atomic_remote_head_remote_owner_missing_count",
    "atomic_remote_head_block_next_missing_count",
    "tls_backing_transfer_enabled",
    "allocator_owner_slot_reuse_enabled",
    "type_abi_hot_lookup_count",
    "provider_abi_hot_dispatch_count",
    "product_activation",
    "hook_install",
    "global_allocator_claim",
    "winner_claim",
)
FASTMEM_BRANCH_CFG_LOWERING_PREFLIGHT_EXPECTED_POSITIVE = (
    "fastmem_branch_cfg_lowering_preflight",
    "fastmem_branch_cfg_selected",
    "fastmem_branch_cfg_closed_guard",
    "remote_owner_branch_routing_selected",
    "remote_owner_branch_routing_lowering_selected",
    "remote_owner_branch_routing_open",
    "remote_owner_branch_routing_lowered_count",
    "remote_owner_branch_route_body_selected",
    "memop_current_alloc_owner_id_lowered_count",
    "memop_owner_eq_lowered_count",
    "atomic_remote_head_drain_local_list_mutation_lowered_count",
)
FASTMEM_BRANCH_CFG_LOWERING_PRODUCER_EXPECTED_ZERO = (
    "fastmem_branch_cfg_closed_guard",
    "remote_owner_branch_route_body_open",
    "page_local_alloc_route_branch_claim",
    "page_local_alloc_route_cfg_lowering_enabled",
    "page_local_free_route_branch_claim",
    "page_local_free_route_cfg_lowering_enabled",
    "atomic_remote_head_remote_owner_missing_count",
    "atomic_remote_head_block_next_missing_count",
    "tls_backing_transfer_enabled",
    "allocator_owner_slot_reuse_enabled",
    "type_abi_hot_lookup_count",
    "provider_abi_hot_dispatch_count",
    "product_activation",
    "hook_install",
    "global_allocator_claim",
    "winner_claim",
)
FASTMEM_BRANCH_CFG_LOWERING_PRODUCER_EXPECTED_POSITIVE = (
    "fastmem_branch_cfg_lowering_producer_pilot",
    "fastmem_branch_cfg_selected",
    "fastmem_branch_cfg_open",
    "fastmem_branch_cfg_lowered_count",
    "remote_owner_branch_routing_selected",
    "remote_owner_branch_routing_lowering_selected",
    "remote_owner_branch_routing_open",
    "remote_owner_branch_routing_lowered_count",
    "remote_owner_branch_route_body_selected",
    "memop_current_alloc_owner_id_lowered_count",
    "memop_owner_eq_lowered_count",
    "atomic_remote_head_drain_local_list_mutation_lowered_count",
)


def int_count(rows: dict[str, Any], key: str) -> int:
    value = rows.get(key, "0")
    try:
        return int(float(str(value)))
    except (TypeError, ValueError):
        return 0


def owner_state_profile(rows: dict[str, str]) -> bool:
    return (
        int_count(rows, "alloc_owner_id_capability") > 0
        or int_count(rows, "worker_id_capability") > 0
        or int_count(rows, "page_owner_check_enabled") > 0
    )


def atomic_remote_profile(rows: dict[str, str]) -> bool:
    return (
        int_count(rows, "atomic_remote_head_pilot_enabled") > 0
        or int_count(rows, "atomic_remote_head_enabled") > 0
    )


def owner_lifecycle_profile(rows: dict[str, str]) -> bool:
    return (
        int_count(rows, "allocator_owner_lifecycle_state_machine") > 0
        or int_count(rows, "allocator_owner_exiting_flush_count") > 0
        or int_count(rows, "allocator_owner_abandoned_count") > 0
        or int_count(rows, "allocator_owner_reclaimed_count") > 0
        or int_count(rows, "allocator_thread_exit_observed_count") > 0
        or int_count(rows, "allocator_abandoned_reclaim_attempt_count") > 0
    )


def safe_wrapper_profile(rows: dict[str, str]) -> bool:
    return int_count(rows, "safe_capability_wrapper_plan") > 0


def mimalloc_keeper_profile(rows: dict[str, str]) -> bool:
    return int_count(rows, "mimalloc_keeper_candidate") > 0


def product_shaped_bridge_profile(rows: dict[str, str]) -> bool:
    return int_count(rows, "replacement_front_product_shaped_bridge_v0") > 0


def size_class_bridge_profile(rows: dict[str, str]) -> bool:
    return int_count(rows, "replacement_front_size_class_bridge_v0") > 0


def page_local_bridge_profile(rows: dict[str, str]) -> bool:
    return int_count(rows, "replacement_front_page_local_bridge_v0") > 0


def producer_taxonomy_profile(rows: dict[str, str]) -> bool:
    return int_count(rows, "replacement_front_producer_taxonomy_v0") > 0


def producer_slice_selection_profile(rows: dict[str, str]) -> bool:
    return int_count(rows, "replacement_front_producer_slice_selection_v0") > 0


def layout_table_producer_pilot_profile(rows: dict[str, str]) -> bool:
    return int_count(rows, "mir_fmem_008b_layout_table_producer_pilot") > 0


def owner_runtime_producer_pilot_profile(rows: dict[str, str]) -> bool:
    return int_count(rows, "fastmem_owner_runtime_producer_pilot") > 0


def local_free_producer_pilot_profile(rows: dict[str, str]) -> bool:
    return int_count(rows, "fastmem_local_free_producer_pilot") > 0


def atomic_remote_head_cas_preflight_profile(rows: dict[str, str]) -> bool:
    return int_count(rows, "fastmem_atomic_remote_head_cas_preflight") > 0


def atomic_remote_head_cas_producer_profile(rows: dict[str, str]) -> bool:
    return int_count(rows, "fastmem_atomic_remote_head_cas_producer_pilot") > 0


def atomic_remote_head_retry_preflight_profile(rows: dict[str, str]) -> bool:
    return int_count(rows, "fastmem_atomic_remote_head_retry_preflight") > 0


def atomic_remote_head_retry_producer_profile(rows: dict[str, str]) -> bool:
    return int_count(rows, "fastmem_atomic_remote_head_retry_producer_pilot") > 0


def atomic_remote_head_drain_preflight_profile(rows: dict[str, str]) -> bool:
    return int_count(rows, "fastmem_atomic_remote_head_drain_preflight") > 0


def atomic_remote_head_drain_exchange_selection_profile(rows: dict[str, str]) -> bool:
    return int_count(rows, "fastmem_atomic_remote_head_drain_exchange_selection") > 0


def atomic_remote_head_drain_exchange_producer_profile(rows: dict[str, str]) -> bool:
    return int_count(rows, "fastmem_atomic_remote_head_drain_exchange_producer_pilot") > 0


def atomic_remote_head_drain_to_local_selection_profile(rows: dict[str, str]) -> bool:
    return int_count(rows, "fastmem_atomic_remote_head_drain_to_local_route_selection") > 0


def atomic_remote_head_drain_to_local_producer_profile(rows: dict[str, str]) -> bool:
    return int_count(rows, "fastmem_atomic_remote_head_drain_to_local_route_producer_pilot") > 0


def atomic_remote_head_drain_local_list_mutation_preflight_profile(
    rows: dict[str, str],
) -> bool:
    return (
        int_count(
            rows,
            "fastmem_atomic_remote_head_drain_local_list_mutation_preflight",
        )
        > 0
    )


def atomic_remote_head_drain_local_list_mutation_proof_profile(
    rows: dict[str, str],
) -> bool:
    return (
        int_count(
            rows,
            "fastmem_atomic_remote_head_drain_local_list_mutation_proof",
        )
        > 0
    )


def atomic_remote_head_drain_local_list_mutation_vocabulary_profile(
    rows: dict[str, str],
) -> bool:
    return (
        int_count(
            rows,
            "fastmem_atomic_remote_head_drain_local_list_mutation_vocabulary_preflight",
        )
        > 0
    )


def atomic_remote_head_drain_local_list_mutation_verifier_profile(
    rows: dict[str, str],
) -> bool:
    return (
        int_count(
            rows,
            "fastmem_atomic_remote_head_drain_local_list_mutation_verifier_preconditions",
        )
        > 0
    )


def atomic_remote_head_drain_local_list_mutation_lowering_profile(
    rows: dict[str, str],
) -> bool:
    return (
        int_count(
            rows,
            "fastmem_atomic_remote_head_drain_local_list_mutation_lowering_producer_pilot",
        )
        > 0
    )


def remote_owner_branch_routing_preflight_profile(rows: dict[str, str]) -> bool:
    return int_count(rows, "fastmem_remote_owner_branch_routing_preflight") > 0


def remote_owner_branch_routing_lowering_preflight_profile(rows: dict[str, str]) -> bool:
    return int_count(rows, "fastmem_remote_owner_branch_routing_lowering_preflight") > 0


def remote_owner_branch_routing_lowering_profile(rows: dict[str, str]) -> bool:
    return int_count(rows, "fastmem_remote_owner_branch_routing_lowering_producer_pilot") > 0


def remote_owner_branch_route_body_preflight_profile(rows: dict[str, str]) -> bool:
    return int_count(rows, "fastmem_remote_owner_branch_route_body_preflight") > 0


def fastmem_branch_cfg_preflight_profile(rows: dict[str, str]) -> bool:
    return int_count(rows, "fastmem_branch_cfg_preflight") > 0


def fastmem_branch_cfg_lowering_preflight_profile(rows: dict[str, str]) -> bool:
    return int_count(rows, "fastmem_branch_cfg_lowering_preflight") > 0


def fastmem_branch_cfg_lowering_profile(rows: dict[str, str]) -> bool:
    return int_count(rows, "fastmem_branch_cfg_lowering_producer_pilot") > 0


def complete_layout_table_lowering_candidate(rows: dict[str, str]) -> bool:
    if not layout_table_producer_pilot_profile(rows):
        return False
    if rows.get("replacement_front_producer") != "mir_to_llvm_lowering":
        return False
    if int_count(rows, "fastmem_verified_mem_access_plan_count") <= 0:
        return False
    return all(int_count(rows, key) == 0 for key in LAYOUT_TABLE_PRODUCER_EXPECTED_ZERO)


def expected_mimalloc_keeper_block_reason(rows: dict[str, str]) -> str:
    if int_count(rows, "mimalloc_shape_score") < int_count(rows, "mimalloc_shape_threshold"):
        return "shape_below_threshold"
    if int_count(rows, "mimalloc_safety_score") < int_count(rows, "mimalloc_safety_threshold"):
        return "safety_below_threshold"
    if int_count(rows, "mimalloc_coverage_score") < int_count(rows, "mimalloc_coverage_threshold"):
        return "coverage_below_threshold"
    return "eligible"


def run_inventory(source_flag: str, source_path: Path) -> dict[str, str]:
    cmd = [sys.executable, str(INVENTORY), source_flag, str(source_path)]
    proc = subprocess.run(cmd, check=False, text=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    if proc.returncode != 0:
        if proc.stderr:
            sys.stderr.write(proc.stderr)
        if proc.stdout:
            sys.stderr.write(proc.stdout)
        raise SystemExit(proc.returncode)
    rows: dict[str, str] = {}
    for raw_line in proc.stdout.splitlines():
        line = raw_line.strip()
        if not line or line.startswith("#") or "=" not in line:
            continue
        key, value = line.split("=", 1)
        rows[key.strip()] = value.strip()
    return rows


def failure_reasons(rows: dict[str, str]) -> list[str]:
    reasons: list[str] = []
    for key in FAIL_FIELDS:
        if int_count(rows, key) > 0:
            reasons.append(key)
    for key, forbidden_values in FAIL_STRING_FIELDS.items():
        if rows.get(key) in forbidden_values:
            reasons.append(key)
    if owner_state_profile(rows):
        if rows.get("alloc_owner_id_kind") != "allocator_arena_owner":
            reasons.append("alloc_owner_id_kind")
        if rows.get("worker_id_kind") != "allocator_arena_owner":
            reasons.append("worker_id_kind")
        if int_count(rows, "allocator_tls_arena_enabled") <= 0:
            reasons.append("allocator_tls_arena_enabled")
        if int_count(rows, "allocator_tls_arena_init_count") <= 0:
            reasons.append("allocator_tls_arena_init_count")
        if int_count(rows, "page_owner_check_enabled") <= 0:
            reasons.append("page_owner_check_enabled")
        if rows.get("page_owner_check_route") != "page_meta_owner_worker_id":
            reasons.append("page_owner_check_route")
        if int_count(rows, "page_owner_check_count") <= 0:
            reasons.append("page_owner_check_count")
    if owner_lifecycle_profile(rows):
        if int_count(rows, "allocator_owner_lifecycle_state_machine") != 1:
            reasons.append("allocator_owner_lifecycle_state_machine")
        if int_count(rows, "allocator_owner_generation_enabled") != 1:
            reasons.append("allocator_owner_generation_enabled")
        if rows.get("allocator_owner_id_kind") != "arena_owner":
            reasons.append("allocator_owner_id_kind")
        if rows.get("allocator_owner_id_repr") != "packed_u64_slot_generation":
            reasons.append("allocator_owner_id_repr")
        if int_count(rows, "allocator_owner_slot_bits") != 32:
            reasons.append("allocator_owner_slot_bits")
        if int_count(rows, "allocator_owner_generation_bits") != 32:
            reasons.append("allocator_owner_generation_bits")
        if int_count(rows, "allocator_owner_zero_is_invalid") != 1:
            reasons.append("allocator_owner_zero_is_invalid")
        if (
            int_count(rows, "allocator_abandoned_reclaim_success_count") > 0
            and int_count(rows, "remote_free_drain_supported") <= 0
        ):
            reasons.append("allocator_abandoned_reclaim_success_without_remote_drain")
        if (
            int_count(rows, "allocator_abandoned_reclaim_success_count") > 0
            and int_count(rows, "remote_candidate_unhandled_reclaim_block_count") > 0
        ):
            reasons.append("allocator_abandoned_reclaim_success_with_unhandled_remote")
    if atomic_remote_profile(rows):
        if int_count(rows, "atomic_remote_head_plan") <= 0:
            reasons.append("atomic_remote_head_plan")
        if rows.get("atomic_remote_head_route") != "page_remote_head_cas":
            reasons.append("atomic_remote_head_route")
        if rows.get("remote_free_memory_order") not in {"acq_rel", "release_acquire"}:
            reasons.append("remote_free_memory_order")
        if int_count(rows, "remote_owner_free_remote_candidate_count") <= 0:
            reasons.append("remote_owner_free_remote_candidate_count")
        if int_count(rows, "remote_owner_free_remote_push_count") <= 0:
            reasons.append("remote_owner_free_remote_push_count")
        if int_count(rows, "remote_free_push_count") <= 0:
            reasons.append("remote_free_push_count")
        if int_count(rows, "remote_free_drain_count") <= 0:
            reasons.append("remote_free_drain_count")
    if safe_wrapper_profile(rows):
        if rows.get("safe_capability_wrapper_route") != "fastmem_memop_alias":
            reasons.append("safe_capability_wrapper_route")
        if rows.get("safe_capability_wrapper_lowering_route") != "fastmem_memop_alias":
            reasons.append("safe_capability_wrapper_lowering_route")
        if int_count(rows, "safe_capability_wrapper_memop_equivalence") <= 0:
            reasons.append("safe_capability_wrapper_memop_equivalence")
        for key in [
            "address_token_wrapper",
            "page_key_wrapper",
            "page_map_bridge_wrapper",
            "page_meta_handle_wrapper",
            "alloc_owner_id_wrapper",
            "atomic_remote_head_wrapper",
        ]:
            if int_count(rows, key) <= 0:
                reasons.append(key)
    if mimalloc_keeper_profile(rows):
        if int_count(rows, "mimalloc_shape_score") < int_count(rows, "mimalloc_shape_threshold"):
            reasons.append("mimalloc_shape_score")
        if int_count(rows, "mimalloc_safety_score") < int_count(rows, "mimalloc_safety_threshold"):
            reasons.append("mimalloc_safety_score")
        if int_count(rows, "mimalloc_coverage_score") < int_count(
            rows, "mimalloc_coverage_threshold"
        ):
            reasons.append("mimalloc_coverage_score")
        if int_count(rows, "mimalloc_keeper_eligible") <= 0:
            reasons.append("mimalloc_keeper_eligible")
        if rows.get("mimalloc_keeper_block_reason") != expected_mimalloc_keeper_block_reason(rows):
            reasons.append("mimalloc_keeper_block_reason")
    if product_shaped_bridge_profile(rows):
        if int_count(rows, "replacement_front_product_shaped_bridge_non_activating") != 1:
            reasons.append("replacement_front_product_shaped_bridge_non_activating")
        if int_count(rows, "replacement_front_product_shaped_bridge_report_only") != 1:
            reasons.append("replacement_front_product_shaped_bridge_report_only")
        if int_count(rows, "replacement_front_product_shaped_bridge_activation_ready") != 0:
            reasons.append("replacement_front_product_shaped_bridge_activation_ready")
        if int_count(rows, "product_activation_ready") != 0:
            reasons.append("product_activation_ready")
        if int_count(rows, "replacement_front_product_shaped_bridge_requires_activation_row") != 1:
            reasons.append("replacement_front_product_shaped_bridge_requires_activation_row")
        if int_count(rows, "replacement_front_product_shaped_bridge_requires_product_gate_open") != 1:
            reasons.append("replacement_front_product_shaped_bridge_requires_product_gate_open")
        missing = rows.get("replacement_front_product_shaped_bridge_missing", "")
        if "activation_row" not in missing:
            reasons.append("replacement_front_product_shaped_bridge_missing_activation_row")
        if "product_gate_open" not in missing:
            reasons.append("replacement_front_product_shaped_bridge_missing_product_gate_open")
        if int_count(rows, "replacement_front_product_shaped_bridge_evidence_ready") > 0:
            for key in [
                "replacement_front_product_shaped_bridge_shape_ok",
                "replacement_front_product_shaped_bridge_safety_ok",
                "replacement_front_product_shaped_bridge_coverage_ok",
                "replacement_front_product_shaped_bridge_preflight_ok",
                "replacement_front_product_shaped_bridge_no_type_abi_hot_lookup",
                "replacement_front_product_shaped_bridge_no_provider_dispatch",
                "replacement_front_product_shaped_bridge_no_global_lock_hot_path",
                "replacement_front_product_shaped_bridge_no_range_scan_hot_path",
                "replacement_front_product_shaped_bridge_no_host_passthrough",
            ]:
                if int_count(rows, key) != 1:
                    reasons.append(key)
            if rows.get("replacement_front_product_shaped_bridge_source_truth") != (
                "hako_alloc.size_class_box"
            ):
                reasons.append("replacement_front_product_shaped_bridge_source_truth")
            if rows.get("replacement_front_product_shaped_bridge_block_reason") != (
                "activation_row_required"
            ):
                reasons.append("replacement_front_product_shaped_bridge_block_reason")
    if size_class_bridge_profile(rows):
        if int_count(rows, "replacement_front_size_class_bridge_report_only") != 1:
            reasons.append("replacement_front_size_class_bridge_report_only")
        if rows.get("replacement_front_size_class_bridge_source_truth") != (
            "hako_alloc.size_class_box"
        ):
            reasons.append("replacement_front_size_class_bridge_source_truth")
        if rows.get("replacement_front_size_class_bridge_source_file") != (
            "lang/src/hako_alloc/memory/size_class_box.hako"
        ):
            reasons.append("replacement_front_size_class_bridge_source_file")
        if int_count(rows, "replacement_front_size_class_bridge_bound") != 1:
            reasons.append("replacement_front_size_class_bridge_bound")
        if rows.get("replacement_front_size_class_bridge_missing") != "none":
            reasons.append("replacement_front_size_class_bridge_missing")
        for key in [
            "replacement_front_size_class_required_methods_present",
            "replacement_front_size_class_usize_facades_present",
            "replacement_front_size_class_policy_methods_covered",
            "replacement_front_size_class_policy_constants_covered",
            "replacement_front_size_class_policy_huge_sentinel_covered",
            "replacement_front_size_class_policy_mirror_matches_source",
        ]:
            if int_count(rows, key) != 1:
                reasons.append(key)
        for key, expected in [
            ("replacement_front_size_class_word_size", 8),
            ("replacement_front_size_class_max_regular_bin", 72),
            ("replacement_front_size_class_huge_bin", 73),
            ("replacement_front_size_class_huge_sentinel", -1),
        ]:
            if int_count(rows, key) != expected:
                reasons.append(key)
    if page_local_bridge_profile(rows):
        if int_count(rows, "replacement_front_page_local_bridge_report_only") != 1:
            reasons.append("replacement_front_page_local_bridge_report_only")
        if rows.get("replacement_front_page_local_bridge_source_truth") != "hako_alloc.page_box":
            reasons.append("replacement_front_page_local_bridge_source_truth")
        if rows.get("replacement_front_page_local_bridge_source_file") != (
            "lang/src/hako_alloc/memory/page_box.hako"
        ):
            reasons.append("replacement_front_page_local_bridge_source_file")
        if int_count(rows, "replacement_front_page_local_bridge_bound") != 1:
            reasons.append("replacement_front_page_local_bridge_bound")
        if rows.get("replacement_front_page_local_bridge_missing") != "none":
            reasons.append("replacement_front_page_local_bridge_missing")
        for key in [
            "replacement_front_page_local_required_fields_present",
            "replacement_front_page_local_required_methods_present",
            "replacement_front_page_local_directarray_fields_present",
            "replacement_front_page_local_counter_fields_present",
            "replacement_front_page_local_acquire_release_methods_present",
            "replacement_front_page_local_lifecycle_methods_present",
            "replacement_front_page_local_typed_meta_matches_source",
            "replacement_front_page_local_same_owner_route_matches_source",
            "replacement_front_page_local_no_remote_free_claim",
        ]:
            if int_count(rows, key) != 1:
                reasons.append(key)
    if producer_taxonomy_profile(rows):
        producer = rows.get("replacement_front_producer", "unknown")
        if producer not in {
            "python_template_c_bridge",
            "mir_to_c_lowering",
            "mir_to_llvm_lowering",
        }:
            reasons.append("replacement_front_producer")
        if int_count(rows, "replacement_front_python_template_c_semantic_ssot") != 0:
            reasons.append("replacement_front_python_template_c_semantic_ssot")
        if int_count(rows, "replacement_front_mirbuilder_representation_only") != 1:
            reasons.append("replacement_front_mirbuilder_representation_only")
        if int_count(rows, "replacement_front_mirbuilder_route_decision_count") != 0:
            reasons.append("replacement_front_mirbuilder_route_decision_count")
        if producer == "python_template_c_bridge":
            if rows.get("replacement_front_backend_artifact") != "c":
                reasons.append("replacement_front_backend_artifact")
            if int_count(rows, "replacement_front_python_template_c_retirement_required") != 1:
                reasons.append("replacement_front_python_template_c_retirement_required")
            if int_count(rows, "replacement_front_mir_memop_enabled") != 0:
                reasons.append("replacement_front_mir_memop_enabled")
            if int_count(rows, "replacement_front_mir_fastmem_region_enabled") != 0:
                reasons.append("replacement_front_mir_fastmem_region_enabled")
            if rows.get("replacement_front_producer_transition_state") != "current_bridge":
                reasons.append("replacement_front_producer_transition_state")
        elif producer == "mir_to_c_lowering":
            if rows.get("replacement_front_backend_artifact") != "c":
                reasons.append("replacement_front_backend_artifact")
            if rows.get("replacement_front_producer_transition_state") != (
                "transition_backend_artifact"
            ):
                reasons.append("replacement_front_producer_transition_state")
        elif producer == "mir_to_llvm_lowering":
            if rows.get("replacement_front_backend_artifact") not in {
                "llvm_ir",
                "object",
                "exe",
            }:
                reasons.append("replacement_front_backend_artifact")
            if rows.get("replacement_front_producer_transition_state") != "final_primary":
                reasons.append("replacement_front_producer_transition_state")
    if producer_slice_selection_profile(rows):
        if int_count(rows, "replacement_front_producer_taxonomy_v0") != 1:
            reasons.append("replacement_front_producer_taxonomy_v0")
        for key, expected in PRODUCER_SLICE_EXPECTED_STRINGS.items():
            if rows.get(key) != expected:
                reasons.append(key)
        for key in PRODUCER_SLICE_EXPECTED_ZERO:
            if int_count(rows, key) != 0:
                reasons.append(key)
    if layout_table_producer_pilot_profile(rows):
        if not producer_slice_selection_profile(rows):
            reasons.append("replacement_front_producer_slice_selection_v0")
        if rows.get("replacement_front_selected_memop_kinds") != (
            "TableIndex,FieldLoad,FieldStore"
        ):
            reasons.append("replacement_front_selected_memop_kinds")
        if rows.get("replacement_front_deferred_memop_kinds") != "CurrentAllocOwnerId,OwnerEq":
            reasons.append("replacement_front_deferred_memop_kinds")
        for key in LAYOUT_TABLE_PRODUCER_EXPECTED_ZERO:
            if int_count(rows, key) != 0:
                reasons.append(key)
        if complete_layout_table_lowering_candidate(rows):
            for key in LAYOUT_TABLE_PRODUCER_EXPECTED_POSITIVE:
                if int_count(rows, key) <= 0:
                    reasons.append(key)
    if owner_runtime_producer_pilot_profile(rows):
        if rows.get("replacement_front_producer") != "mir_to_llvm_lowering":
            reasons.append("replacement_front_producer")
        if rows.get("fastmem_owner_runtime_current_owner_source") != (
            "llvm_producer_intrinsic"
        ):
            reasons.append("fastmem_owner_runtime_current_owner_source")
        if rows.get("replacement_front_selected_memop_family") != "owner_runtime":
            reasons.append("replacement_front_selected_memop_family")
        if rows.get("replacement_front_selected_memop_kinds") != (
            "CurrentAllocOwnerId,OwnerEq"
        ):
            reasons.append("replacement_front_selected_memop_kinds")
        for key in OWNER_RUNTIME_PRODUCER_EXPECTED_ZERO:
            if int_count(rows, key) != 0:
                reasons.append(key)
        for key in OWNER_RUNTIME_PRODUCER_EXPECTED_POSITIVE:
            if int_count(rows, key) <= 0:
                reasons.append(key)
    if local_free_producer_pilot_profile(rows):
        if rows.get("replacement_front_producer") != "mir_to_llvm_lowering":
            reasons.append("replacement_front_producer")
        if rows.get("replacement_front_selected_memop_family") != "local_free":
            reasons.append("replacement_front_selected_memop_family")
        selected_local_free = rows.get("replacement_front_selected_memop_kinds")
        selected_parts = set(filter(None, (selected_local_free or "").split(",")))
        allowed_local_free = {
            "LocalFreePush",
            "LocalFreePop",
            "FreeHeadPush",
            "FreeHeadPop",
        }
        if (
            not selected_parts
            or selected_local_free == "none"
            or not selected_parts.issubset(allowed_local_free)
        ):
            reasons.append("replacement_front_selected_memop_kinds")
        deferred_local_free = rows.get("replacement_front_deferred_memop_kinds", "")
        if "AtomicRemoteHead" not in deferred_local_free.split(","):
            reasons.append("replacement_front_deferred_memop_kinds")
        for key in LOCAL_FREE_PRODUCER_EXPECTED_ZERO:
            if int_count(rows, key) != 0:
                reasons.append(key)
        for key in LOCAL_FREE_PRODUCER_EXPECTED_POSITIVE:
            if int_count(rows, key) <= 0:
                reasons.append(key)
        if selected_local_free and "LocalFreePush" in selected_local_free:
            for key in (
                "fastmem_local_free_push_plan_count",
                "memop_local_free_push_lowered_count",
                "memop_local_free_push_layout_ref_consumed_count",
                "fastmem_local_free_push_lowering_uses_verified_plan",
            ):
                if int_count(rows, key) <= 0:
                    reasons.append(key)
        if selected_local_free and "LocalFreePop" in selected_local_free:
            for key in (
                "fastmem_local_free_pop_plan_count",
                "memop_local_free_pop_lowered_count",
                "memop_local_free_pop_layout_ref_consumed_count",
                "fastmem_local_free_pop_lowering_uses_verified_plan",
                "fastmem_local_free_pop_lowering_enabled",
            ):
                if int_count(rows, key) <= 0:
                    reasons.append(key)
        if selected_local_free and "FreeHeadPush" in selected_local_free:
            for key in (
                "fastmem_free_head_push_plan_count",
                "memop_free_head_push_lowered_count",
                "memop_free_head_push_layout_ref_consumed_count",
                "fastmem_free_head_push_lowering_uses_verified_plan",
                "fastmem_free_head_push_lowering_enabled",
            ):
                if int_count(rows, key) <= 0:
                    reasons.append(key)
        if selected_local_free and "FreeHeadPop" in selected_local_free:
            for key in (
                "fastmem_free_head_pop_plan_count",
                "memop_free_head_pop_lowered_count",
                "memop_free_head_pop_layout_ref_consumed_count",
                "fastmem_free_head_pop_lowering_uses_verified_plan",
                "fastmem_free_head_pop_lowering_enabled",
            ):
                if int_count(rows, key) <= 0:
                    reasons.append(key)
    if atomic_remote_head_cas_preflight_profile(rows):
        if rows.get("replacement_front_producer") != "mir_to_llvm_lowering":
            reasons.append("replacement_front_producer")
        if rows.get("replacement_front_selected_memop_family") != "remote_free":
            reasons.append("replacement_front_selected_memop_family")
        if rows.get("replacement_front_selected_memop_kinds") != "AtomicRemoteHeadPush":
            reasons.append("replacement_front_selected_memop_kinds")
        if rows.get("atomic_remote_head_memory_order_policy") != "closed":
            reasons.append("atomic_remote_head_memory_order_policy")
        for key in ATOMIC_REMOTE_HEAD_PREFLIGHT_EXPECTED_ZERO:
            if int_count(rows, key) != 0:
                reasons.append(key)
        for key in ATOMIC_REMOTE_HEAD_PREFLIGHT_EXPECTED_POSITIVE:
            if int_count(rows, key) <= 0:
                reasons.append(key)
    if atomic_remote_head_cas_producer_profile(rows):
        if rows.get("replacement_front_producer") != "mir_to_llvm_lowering":
            reasons.append("replacement_front_producer")
        if rows.get("replacement_front_selected_memop_family") != "remote_free":
            reasons.append("replacement_front_selected_memop_family")
        if rows.get("replacement_front_selected_memop_kinds") != "AtomicRemoteHeadPush":
            reasons.append("replacement_front_selected_memop_kinds")
        if rows.get("atomic_remote_head_memory_order_policy") != "acq_rel":
            reasons.append("atomic_remote_head_memory_order_policy")
        for key in ATOMIC_REMOTE_HEAD_PRODUCER_EXPECTED_ZERO:
            if int_count(rows, key) != 0:
                reasons.append(key)
        for key in ATOMIC_REMOTE_HEAD_PRODUCER_EXPECTED_POSITIVE:
            if int_count(rows, key) <= 0:
                reasons.append(key)
    if atomic_remote_head_retry_preflight_profile(rows):
        if rows.get("replacement_front_producer") != "mir_to_llvm_lowering":
            reasons.append("replacement_front_producer")
        if rows.get("replacement_front_selected_memop_family") != "remote_free":
            reasons.append("replacement_front_selected_memop_family")
        if rows.get("replacement_front_selected_memop_kinds") != "AtomicRemoteHeadPush":
            reasons.append("replacement_front_selected_memop_kinds")
        if rows.get("replacement_front_next_producer_slice") != (
            "atomic_remote_head_retry_policy_preflight"
        ):
            reasons.append("replacement_front_next_producer_slice")
        if "AtomicRemoteHeadRetryLowering" not in rows.get(
            "replacement_front_deferred_memop_kinds", ""
        ).split(","):
            reasons.append("replacement_front_deferred_memop_kinds")
        if rows.get("atomic_remote_head_memory_order_policy") != "acq_rel":
            reasons.append("atomic_remote_head_memory_order_policy")
        for key in ATOMIC_REMOTE_HEAD_RETRY_PREFLIGHT_EXPECTED_ZERO:
            if int_count(rows, key) != 0:
                reasons.append(key)
        for key in ATOMIC_REMOTE_HEAD_RETRY_PREFLIGHT_EXPECTED_POSITIVE:
            if int_count(rows, key) <= 0:
                reasons.append(key)
    if atomic_remote_head_retry_producer_profile(rows):
        if rows.get("replacement_front_producer") != "mir_to_llvm_lowering":
            reasons.append("replacement_front_producer")
        if rows.get("replacement_front_selected_memop_family") != "remote_free":
            reasons.append("replacement_front_selected_memop_family")
        if rows.get("replacement_front_selected_memop_kinds") != "AtomicRemoteHeadPush":
            reasons.append("replacement_front_selected_memop_kinds")
        if rows.get("replacement_front_next_producer_slice") != (
            "atomic_remote_head_retry_lowering_producer_pilot"
        ):
            reasons.append("replacement_front_next_producer_slice")
        if "AtomicRemoteHeadDrain" not in rows.get(
            "replacement_front_deferred_memop_kinds", ""
        ).split(","):
            reasons.append("replacement_front_deferred_memop_kinds")
        if "RemoteOwnerBranchRouting" not in rows.get(
            "replacement_front_deferred_memop_kinds", ""
        ).split(","):
            reasons.append("replacement_front_deferred_memop_kinds")
        if rows.get("atomic_remote_head_memory_order_policy") != "acq_rel":
            reasons.append("atomic_remote_head_memory_order_policy")
        for key in ATOMIC_REMOTE_HEAD_RETRY_PRODUCER_EXPECTED_ZERO:
            if int_count(rows, key) != 0:
                reasons.append(key)
        for key in ATOMIC_REMOTE_HEAD_RETRY_PRODUCER_EXPECTED_POSITIVE:
            if int_count(rows, key) <= 0:
                reasons.append(key)
    if atomic_remote_head_drain_preflight_profile(rows):
        if rows.get("replacement_front_producer") != "mir_to_llvm_lowering":
            reasons.append("replacement_front_producer")
        if rows.get("replacement_front_selected_memop_family") != "remote_free":
            reasons.append("replacement_front_selected_memop_family")
        if rows.get("replacement_front_selected_memop_kinds") != "AtomicRemoteHeadDrain":
            reasons.append("replacement_front_selected_memop_kinds")
        if rows.get("replacement_front_next_producer_slice") != (
            "atomic_remote_head_drain_preflight"
        ):
            reasons.append("replacement_front_next_producer_slice")
        if "AtomicRemoteHeadDrainLowering" not in rows.get(
            "replacement_front_deferred_memop_kinds", ""
        ).split(","):
            reasons.append("replacement_front_deferred_memop_kinds")
        if "RemoteOwnerBranchRouting" not in rows.get(
            "replacement_front_deferred_memop_kinds", ""
        ).split(","):
            reasons.append("replacement_front_deferred_memop_kinds")
        if rows.get("atomic_remote_head_memory_order_policy") != "acq_rel":
            reasons.append("atomic_remote_head_memory_order_policy")
        for key in ATOMIC_REMOTE_HEAD_DRAIN_PREFLIGHT_EXPECTED_ZERO:
            if int_count(rows, key) != 0:
                reasons.append(key)
        for key in ATOMIC_REMOTE_HEAD_DRAIN_PREFLIGHT_EXPECTED_POSITIVE:
            if int_count(rows, key) <= 0:
                reasons.append(key)
    if atomic_remote_head_drain_exchange_selection_profile(rows):
        if rows.get("replacement_front_producer") != "mir_to_llvm_lowering":
            reasons.append("replacement_front_producer")
        if rows.get("replacement_front_selected_memop_family") != "remote_free":
            reasons.append("replacement_front_selected_memop_family")
        if rows.get("replacement_front_selected_memop_kinds") != "AtomicRemoteHeadDrain":
            reasons.append("replacement_front_selected_memop_kinds")
        if rows.get("replacement_front_next_producer_slice") != (
            "atomic_remote_head_drain_exchange_lowering_producer_pilot"
        ):
            reasons.append("replacement_front_next_producer_slice")
        if "AtomicRemoteHeadDrainLowering" not in rows.get(
            "replacement_front_deferred_memop_kinds", ""
        ).split(","):
            reasons.append("replacement_front_deferred_memop_kinds")
        if "DrainToLocalRoute" not in rows.get(
            "replacement_front_deferred_memop_kinds", ""
        ).split(","):
            reasons.append("replacement_front_deferred_memop_kinds")
        if "RemoteOwnerBranchRouting" not in rows.get(
            "replacement_front_deferred_memop_kinds", ""
        ).split(","):
            reasons.append("replacement_front_deferred_memop_kinds")
        if rows.get("atomic_remote_head_drain_exchange_order") != "acquire":
            reasons.append("atomic_remote_head_drain_exchange_order")
        if rows.get("atomic_remote_head_drain_result_kind") != "remote_free_list_token":
            reasons.append("atomic_remote_head_drain_result_kind")
        if rows.get("atomic_remote_head_memory_order_policy") != "acq_rel":
            reasons.append("atomic_remote_head_memory_order_policy")
        for key in ATOMIC_REMOTE_HEAD_DRAIN_EXCHANGE_SELECTION_EXPECTED_ZERO:
            if int_count(rows, key) != 0:
                reasons.append(key)
        for key in ATOMIC_REMOTE_HEAD_DRAIN_EXCHANGE_SELECTION_EXPECTED_POSITIVE:
            if int_count(rows, key) <= 0:
                reasons.append(key)
    if atomic_remote_head_drain_exchange_producer_profile(rows):
        if rows.get("replacement_front_producer") != "mir_to_llvm_lowering":
            reasons.append("replacement_front_producer")
        if rows.get("replacement_front_selected_memop_family") != "remote_free":
            reasons.append("replacement_front_selected_memop_family")
        if rows.get("replacement_front_selected_memop_kinds") != "AtomicRemoteHeadDrain":
            reasons.append("replacement_front_selected_memop_kinds")
        if rows.get("replacement_front_next_producer_slice") != (
            "atomic_remote_head_drain_to_local_route_selection"
        ):
            reasons.append("replacement_front_next_producer_slice")
        if "DrainToLocalRoute" not in rows.get(
            "replacement_front_deferred_memop_kinds", ""
        ).split(","):
            reasons.append("replacement_front_deferred_memop_kinds")
        if "RemoteOwnerBranchRouting" not in rows.get(
            "replacement_front_deferred_memop_kinds", ""
        ).split(","):
            reasons.append("replacement_front_deferred_memop_kinds")
        if rows.get("atomic_remote_head_drain_exchange_order") != "acquire":
            reasons.append("atomic_remote_head_drain_exchange_order")
        if rows.get("atomic_remote_head_drain_result_kind") != "remote_free_list_token":
            reasons.append("atomic_remote_head_drain_result_kind")
        if rows.get("atomic_remote_head_memory_order_policy") != "acquire_exchange":
            reasons.append("atomic_remote_head_memory_order_policy")
        for key in ATOMIC_REMOTE_HEAD_DRAIN_EXCHANGE_PRODUCER_EXPECTED_ZERO:
            if int_count(rows, key) != 0:
                reasons.append(key)
        for key in ATOMIC_REMOTE_HEAD_DRAIN_EXCHANGE_PRODUCER_EXPECTED_POSITIVE:
            if int_count(rows, key) <= 0:
                reasons.append(key)
    if atomic_remote_head_drain_to_local_selection_profile(rows):
        if rows.get("replacement_front_producer") != "mir_to_llvm_lowering":
            reasons.append("replacement_front_producer")
        if rows.get("replacement_front_selected_memop_family") != "remote_free":
            reasons.append("replacement_front_selected_memop_family")
        if rows.get("replacement_front_selected_memop_kinds") != "AtomicRemoteHeadDrain":
            reasons.append("replacement_front_selected_memop_kinds")
        if rows.get("replacement_front_next_producer_slice") != (
            "atomic_remote_head_drain_to_local_route_producer_pilot"
        ):
            reasons.append("replacement_front_next_producer_slice")
        if "DrainToLocalRouteLowering" not in rows.get(
            "replacement_front_deferred_memop_kinds", ""
        ).split(","):
            reasons.append("replacement_front_deferred_memop_kinds")
        if "RemoteOwnerBranchRouting" not in rows.get(
            "replacement_front_deferred_memop_kinds", ""
        ).split(","):
            reasons.append("replacement_front_deferred_memop_kinds")
        if rows.get("atomic_remote_head_drain_exchange_order") != "acquire":
            reasons.append("atomic_remote_head_drain_exchange_order")
        if rows.get("atomic_remote_head_drain_result_kind") != "remote_free_list_token":
            reasons.append("atomic_remote_head_drain_result_kind")
        if rows.get("atomic_remote_head_memory_order_policy") != "acquire_exchange":
            reasons.append("atomic_remote_head_memory_order_policy")
        for key in ATOMIC_REMOTE_HEAD_DRAIN_TO_LOCAL_SELECTION_EXPECTED_ZERO:
            if int_count(rows, key) != 0:
                reasons.append(key)
        for key in ATOMIC_REMOTE_HEAD_DRAIN_TO_LOCAL_SELECTION_EXPECTED_POSITIVE:
            if int_count(rows, key) <= 0:
                reasons.append(key)
    if atomic_remote_head_drain_to_local_producer_profile(rows):
        if rows.get("replacement_front_producer") != "mir_to_llvm_lowering":
            reasons.append("replacement_front_producer")
        if rows.get("replacement_front_selected_memop_family") != "remote_free":
            reasons.append("replacement_front_selected_memop_family")
        if rows.get("replacement_front_selected_memop_kinds") != "AtomicRemoteHeadDrain":
            reasons.append("replacement_front_selected_memop_kinds")
        if rows.get("replacement_front_next_producer_slice") != (
            "atomic_remote_head_drain_local_list_mutation_preflight"
        ):
            reasons.append("replacement_front_next_producer_slice")
        if "DrainToLocalMutation" not in rows.get(
            "replacement_front_deferred_memop_kinds", ""
        ).split(","):
            reasons.append("replacement_front_deferred_memop_kinds")
        if "RemoteOwnerBranchRouting" not in rows.get(
            "replacement_front_deferred_memop_kinds", ""
        ).split(","):
            reasons.append("replacement_front_deferred_memop_kinds")
        if rows.get("atomic_remote_head_drain_exchange_order") != "acquire":
            reasons.append("atomic_remote_head_drain_exchange_order")
        if rows.get("atomic_remote_head_drain_result_kind") != "remote_free_list_token":
            reasons.append("atomic_remote_head_drain_result_kind")
        if rows.get("atomic_remote_head_memory_order_policy") != "acquire_exchange":
            reasons.append("atomic_remote_head_memory_order_policy")
        for key in ATOMIC_REMOTE_HEAD_DRAIN_TO_LOCAL_PRODUCER_EXPECTED_ZERO:
            if int_count(rows, key) != 0:
                reasons.append(key)
        for key in ATOMIC_REMOTE_HEAD_DRAIN_TO_LOCAL_PRODUCER_EXPECTED_POSITIVE:
            if int_count(rows, key) <= 0:
                reasons.append(key)
    if atomic_remote_head_drain_local_list_mutation_preflight_profile(rows):
        if rows.get("replacement_front_producer") != "mir_to_llvm_lowering":
            reasons.append("replacement_front_producer")
        if rows.get("replacement_front_selected_memop_family") != "remote_free":
            reasons.append("replacement_front_selected_memop_family")
        if rows.get("replacement_front_selected_memop_kinds") != "AtomicRemoteHeadDrain":
            reasons.append("replacement_front_selected_memop_kinds")
        if rows.get("replacement_front_next_producer_slice") != (
            "atomic_remote_head_drain_local_list_mutation_proof"
        ):
            reasons.append("replacement_front_next_producer_slice")
        if "DrainLocalListMutation" not in rows.get(
            "replacement_front_deferred_memop_kinds", ""
        ).split(","):
            reasons.append("replacement_front_deferred_memop_kinds")
        if "RemoteOwnerBranchRouting" not in rows.get(
            "replacement_front_deferred_memop_kinds", ""
        ).split(","):
            reasons.append("replacement_front_deferred_memop_kinds")
        if rows.get("atomic_remote_head_drain_exchange_order") != "acquire":
            reasons.append("atomic_remote_head_drain_exchange_order")
        if rows.get("atomic_remote_head_drain_result_kind") != "remote_free_list_token":
            reasons.append("atomic_remote_head_drain_result_kind")
        if rows.get("atomic_remote_head_memory_order_policy") != "acquire_exchange":
            reasons.append("atomic_remote_head_memory_order_policy")
        for key in ATOMIC_REMOTE_HEAD_DRAIN_LOCAL_LIST_MUTATION_PREFLIGHT_EXPECTED_ZERO:
            if int_count(rows, key) != 0:
                reasons.append(key)
        for key in ATOMIC_REMOTE_HEAD_DRAIN_LOCAL_LIST_MUTATION_PREFLIGHT_EXPECTED_POSITIVE:
            if int_count(rows, key) <= 0:
                reasons.append(key)
    if atomic_remote_head_drain_local_list_mutation_proof_profile(rows):
        if rows.get("replacement_front_producer") != "mir_to_llvm_lowering":
            reasons.append("replacement_front_producer")
        if rows.get("replacement_front_selected_memop_family") != "remote_free":
            reasons.append("replacement_front_selected_memop_family")
        if rows.get("replacement_front_selected_memop_kinds") != "AtomicRemoteHeadDrain":
            reasons.append("replacement_front_selected_memop_kinds")
        if rows.get("replacement_front_next_producer_slice") != (
            "atomic_remote_head_drain_local_list_mutation_vocabulary_preflight"
        ):
            reasons.append("replacement_front_next_producer_slice")
        if "DrainLocalListMutationVocabulary" not in rows.get(
            "replacement_front_deferred_memop_kinds", ""
        ).split(","):
            reasons.append("replacement_front_deferred_memop_kinds")
        if "RemoteOwnerBranchRouting" not in rows.get(
            "replacement_front_deferred_memop_kinds", ""
        ).split(","):
            reasons.append("replacement_front_deferred_memop_kinds")
        if rows.get("atomic_remote_head_drain_exchange_order") != "acquire":
            reasons.append("atomic_remote_head_drain_exchange_order")
        if rows.get("atomic_remote_head_drain_result_kind") != "remote_free_list_token":
            reasons.append("atomic_remote_head_drain_result_kind")
        if rows.get("atomic_remote_head_memory_order_policy") != "acquire_exchange":
            reasons.append("atomic_remote_head_memory_order_policy")
        if rows.get("atomic_remote_head_drain_local_list_head_class") != (
            "owner_local_free_or_free_head"
        ):
            reasons.append("atomic_remote_head_drain_local_list_head_class")
        if rows.get("atomic_remote_head_drain_local_list_publication_order") != (
            "verifier_owned_acquire_then_owner_local"
        ):
            reasons.append("atomic_remote_head_drain_local_list_publication_order")
        for key in ATOMIC_REMOTE_HEAD_DRAIN_LOCAL_LIST_MUTATION_PROOF_EXPECTED_ZERO:
            if int_count(rows, key) != 0:
                reasons.append(key)
        for key in ATOMIC_REMOTE_HEAD_DRAIN_LOCAL_LIST_MUTATION_PROOF_EXPECTED_POSITIVE:
            if int_count(rows, key) <= 0:
                reasons.append(key)
    if atomic_remote_head_drain_local_list_mutation_vocabulary_profile(rows):
        if rows.get("replacement_front_producer") != "mir_to_llvm_lowering":
            reasons.append("replacement_front_producer")
        if rows.get("replacement_front_selected_memop_family") != "remote_free":
            reasons.append("replacement_front_selected_memop_family")
        if rows.get("replacement_front_selected_memop_kinds") != "AtomicRemoteHeadDrain":
            reasons.append("replacement_front_selected_memop_kinds")
        if rows.get("replacement_front_next_producer_slice") != (
            "atomic_remote_head_drain_local_list_mutation_verifier_preconditions"
        ):
            reasons.append("replacement_front_next_producer_slice")
        if "DrainRemoteListToLocalLowering" not in rows.get(
            "replacement_front_deferred_memop_kinds", ""
        ).split(","):
            reasons.append("replacement_front_deferred_memop_kinds")
        if "RemoteOwnerBranchRouting" not in rows.get(
            "replacement_front_deferred_memop_kinds", ""
        ).split(","):
            reasons.append("replacement_front_deferred_memop_kinds")
        if rows.get("atomic_remote_head_drain_exchange_order") != "acquire":
            reasons.append("atomic_remote_head_drain_exchange_order")
        if rows.get("atomic_remote_head_drain_result_kind") != "remote_free_list_token":
            reasons.append("atomic_remote_head_drain_result_kind")
        if rows.get("atomic_remote_head_memory_order_policy") != "acquire_exchange":
            reasons.append("atomic_remote_head_memory_order_policy")
        if rows.get("atomic_remote_head_drain_local_list_head_class") != (
            "owner_local_free_or_free_head"
        ):
            reasons.append("atomic_remote_head_drain_local_list_head_class")
        if rows.get("atomic_remote_head_drain_local_list_publication_order") != (
            "verifier_owned_acquire_then_owner_local"
        ):
            reasons.append("atomic_remote_head_drain_local_list_publication_order")
        for key in ATOMIC_REMOTE_HEAD_DRAIN_LOCAL_LIST_MUTATION_VOCABULARY_EXPECTED_ZERO:
            if int_count(rows, key) != 0:
                reasons.append(key)
        for key in ATOMIC_REMOTE_HEAD_DRAIN_LOCAL_LIST_MUTATION_VOCABULARY_EXPECTED_POSITIVE:
            if int_count(rows, key) <= 0:
                reasons.append(key)
    if atomic_remote_head_drain_local_list_mutation_verifier_profile(rows):
        if rows.get("replacement_front_producer") != "mir_to_llvm_lowering":
            reasons.append("replacement_front_producer")
        if rows.get("replacement_front_selected_memop_family") != "remote_free":
            reasons.append("replacement_front_selected_memop_family")
        if rows.get("replacement_front_selected_memop_kinds") != "DrainRemoteListToLocal":
            reasons.append("replacement_front_selected_memop_kinds")
        if rows.get("replacement_front_next_producer_slice") != (
            "atomic_remote_head_drain_local_list_mutation_lowering_producer_pilot"
        ):
            reasons.append("replacement_front_next_producer_slice")
        if "DrainRemoteListToLocalLowering" not in rows.get(
            "replacement_front_deferred_memop_kinds", ""
        ).split(","):
            reasons.append("replacement_front_deferred_memop_kinds")
        if "RemoteOwnerBranchRouting" not in rows.get(
            "replacement_front_deferred_memop_kinds", ""
        ).split(","):
            reasons.append("replacement_front_deferred_memop_kinds")
        if rows.get("atomic_remote_head_drain_exchange_order") != "acquire":
            reasons.append("atomic_remote_head_drain_exchange_order")
        if rows.get("atomic_remote_head_drain_result_kind") != "remote_free_list_token":
            reasons.append("atomic_remote_head_drain_result_kind")
        if rows.get("atomic_remote_head_memory_order_policy") != "acquire_exchange":
            reasons.append("atomic_remote_head_memory_order_policy")
        if rows.get("atomic_remote_head_drain_local_list_head_class") != (
            "owner_local_free_or_free_head"
        ):
            reasons.append("atomic_remote_head_drain_local_list_head_class")
        if rows.get("atomic_remote_head_drain_local_list_publication_order") != (
            "verifier_owned_acquire_then_owner_local"
        ):
            reasons.append("atomic_remote_head_drain_local_list_publication_order")
        for key in ATOMIC_REMOTE_HEAD_DRAIN_LOCAL_LIST_MUTATION_VERIFIER_EXPECTED_ZERO:
            if int_count(rows, key) != 0:
                reasons.append(key)
        for key in ATOMIC_REMOTE_HEAD_DRAIN_LOCAL_LIST_MUTATION_VERIFIER_EXPECTED_POSITIVE:
            if int_count(rows, key) <= 0:
                reasons.append(key)
    if atomic_remote_head_drain_local_list_mutation_lowering_profile(rows):
        if rows.get("replacement_front_producer") != "mir_to_llvm_lowering":
            reasons.append("replacement_front_producer")
        if rows.get("replacement_front_selected_memop_family") != "remote_free":
            reasons.append("replacement_front_selected_memop_family")
        if rows.get("replacement_front_selected_memop_kinds") != "DrainRemoteListToLocal":
            reasons.append("replacement_front_selected_memop_kinds")
        if rows.get("replacement_front_next_producer_slice") != (
            "remote_owner_branch_routing_preflight"
        ):
            reasons.append("replacement_front_next_producer_slice")
        if "RemoteOwnerBranchRouting" not in rows.get(
            "replacement_front_deferred_memop_kinds", ""
        ).split(","):
            reasons.append("replacement_front_deferred_memop_kinds")
        if rows.get("atomic_remote_head_drain_exchange_order") != "acquire":
            reasons.append("atomic_remote_head_drain_exchange_order")
        if rows.get("atomic_remote_head_drain_result_kind") != "remote_free_list_token":
            reasons.append("atomic_remote_head_drain_result_kind")
        if rows.get("atomic_remote_head_memory_order_policy") != "acquire_exchange":
            reasons.append("atomic_remote_head_memory_order_policy")
        if rows.get("atomic_remote_head_drain_local_list_head_class") != (
            "owner_local_free_or_free_head"
        ):
            reasons.append("atomic_remote_head_drain_local_list_head_class")
        if rows.get("atomic_remote_head_drain_local_list_publication_order") != (
            "verifier_owned_acquire_then_owner_local"
        ):
            reasons.append("atomic_remote_head_drain_local_list_publication_order")
        for key in ATOMIC_REMOTE_HEAD_DRAIN_LOCAL_LIST_MUTATION_LOWERING_EXPECTED_ZERO:
            if int_count(rows, key) != 0:
                reasons.append(key)
        for key in ATOMIC_REMOTE_HEAD_DRAIN_LOCAL_LIST_MUTATION_LOWERING_EXPECTED_POSITIVE:
            if int_count(rows, key) <= 0:
                reasons.append(key)
    if remote_owner_branch_routing_preflight_profile(rows):
        if rows.get("replacement_front_producer") != "mir_to_llvm_lowering":
            reasons.append("replacement_front_producer")
        if rows.get("replacement_front_selected_route") != (
            "remote_owner_branch_routing_preflight"
        ):
            reasons.append("replacement_front_selected_route")
        if rows.get("replacement_front_selected_memop_family") != "remote_free_routing":
            reasons.append("replacement_front_selected_memop_family")
        if rows.get("replacement_front_selected_memop_kinds") != "RemoteOwnerBranchRouting":
            reasons.append("replacement_front_selected_memop_kinds")
        if rows.get("replacement_front_next_producer_slice") != (
            "remote_owner_branch_routing_lowering_preflight"
        ):
            reasons.append("replacement_front_next_producer_slice")
        if "RemoteOwnerBranchRoutingLowering" not in rows.get(
            "replacement_front_deferred_memop_kinds", ""
        ).split(","):
            reasons.append("replacement_front_deferred_memop_kinds")
        if rows.get("atomic_remote_head_drain_exchange_order") != "acquire":
            reasons.append("atomic_remote_head_drain_exchange_order")
        if rows.get("atomic_remote_head_drain_result_kind") != "remote_free_list_token":
            reasons.append("atomic_remote_head_drain_result_kind")
        if rows.get("atomic_remote_head_memory_order_policy") != "acquire_exchange":
            reasons.append("atomic_remote_head_memory_order_policy")
        if rows.get("atomic_remote_head_drain_local_list_head_class") != (
            "owner_local_free_or_free_head"
        ):
            reasons.append("atomic_remote_head_drain_local_list_head_class")
        if rows.get("atomic_remote_head_drain_local_list_publication_order") != (
            "verifier_owned_acquire_then_owner_local"
        ):
            reasons.append("atomic_remote_head_drain_local_list_publication_order")
        for key in REMOTE_OWNER_BRANCH_ROUTING_PREFLIGHT_EXPECTED_ZERO:
            if int_count(rows, key) != 0:
                reasons.append(key)
        for key in REMOTE_OWNER_BRANCH_ROUTING_PREFLIGHT_EXPECTED_POSITIVE:
            if int_count(rows, key) <= 0:
                reasons.append(key)
    if remote_owner_branch_routing_lowering_preflight_profile(rows):
        if rows.get("replacement_front_producer") != "mir_to_llvm_lowering":
            reasons.append("replacement_front_producer")
        if rows.get("replacement_front_selected_route") != (
            "remote_owner_branch_routing_lowering_preflight"
        ):
            reasons.append("replacement_front_selected_route")
        if rows.get("replacement_front_selected_memop_family") != "remote_free_routing":
            reasons.append("replacement_front_selected_memop_family")
        if rows.get("replacement_front_selected_memop_kinds") != "RemoteOwnerBranchRouting":
            reasons.append("replacement_front_selected_memop_kinds")
        if rows.get("replacement_front_next_producer_slice") != (
            "remote_owner_branch_routing_lowering_producer_pilot"
        ):
            reasons.append("replacement_front_next_producer_slice")
        if "RemoteOwnerBranchRoutingLowering" not in rows.get(
            "replacement_front_deferred_memop_kinds", ""
        ).split(","):
            reasons.append("replacement_front_deferred_memop_kinds")
        if rows.get("atomic_remote_head_drain_exchange_order") != "acquire":
            reasons.append("atomic_remote_head_drain_exchange_order")
        if rows.get("atomic_remote_head_drain_result_kind") != "remote_free_list_token":
            reasons.append("atomic_remote_head_drain_result_kind")
        if rows.get("atomic_remote_head_memory_order_policy") != "acquire_exchange":
            reasons.append("atomic_remote_head_memory_order_policy")
        if rows.get("atomic_remote_head_drain_local_list_head_class") != (
            "owner_local_free_or_free_head"
        ):
            reasons.append("atomic_remote_head_drain_local_list_head_class")
        if rows.get("atomic_remote_head_drain_local_list_publication_order") != (
            "verifier_owned_acquire_then_owner_local"
        ):
            reasons.append("atomic_remote_head_drain_local_list_publication_order")
        for key in REMOTE_OWNER_BRANCH_ROUTING_LOWERING_PREFLIGHT_EXPECTED_ZERO:
            if int_count(rows, key) != 0:
                reasons.append(key)
        for key in REMOTE_OWNER_BRANCH_ROUTING_LOWERING_PREFLIGHT_EXPECTED_POSITIVE:
            if int_count(rows, key) <= 0:
                reasons.append(key)
    if remote_owner_branch_routing_lowering_profile(rows):
        if rows.get("replacement_front_producer") != "mir_to_llvm_lowering":
            reasons.append("replacement_front_producer")
        if rows.get("replacement_front_selected_route") != (
            "remote_owner_branch_routing_lowering_producer_pilot"
        ):
            reasons.append("replacement_front_selected_route")
        if rows.get("replacement_front_selected_memop_family") != "remote_free_routing":
            reasons.append("replacement_front_selected_memop_family")
        if rows.get("replacement_front_selected_memop_kinds") != "RemoteOwnerBranchRouting":
            reasons.append("replacement_front_selected_memop_kinds")
        if rows.get("replacement_front_next_producer_slice") != (
            "remote_owner_branch_route_body_preflight"
        ):
            reasons.append("replacement_front_next_producer_slice")
        if "SameRemoteFreeBody" not in rows.get(
            "replacement_front_deferred_memop_kinds", ""
        ).split(","):
            reasons.append("replacement_front_deferred_memop_kinds")
        if "BranchCfgLowering" not in rows.get(
            "replacement_front_deferred_memop_kinds", ""
        ).split(","):
            reasons.append("replacement_front_deferred_memop_kinds")
        if rows.get("atomic_remote_head_drain_exchange_order") != "acquire":
            reasons.append("atomic_remote_head_drain_exchange_order")
        if rows.get("atomic_remote_head_drain_result_kind") != "remote_free_list_token":
            reasons.append("atomic_remote_head_drain_result_kind")
        if rows.get("atomic_remote_head_memory_order_policy") != "acquire_exchange":
            reasons.append("atomic_remote_head_memory_order_policy")
        if rows.get("atomic_remote_head_drain_local_list_head_class") != (
            "owner_local_free_or_free_head"
        ):
            reasons.append("atomic_remote_head_drain_local_list_head_class")
        if rows.get("atomic_remote_head_drain_local_list_publication_order") != (
            "verifier_owned_acquire_then_owner_local"
        ):
            reasons.append("atomic_remote_head_drain_local_list_publication_order")
        for key in REMOTE_OWNER_BRANCH_ROUTING_LOWERING_EXPECTED_ZERO:
            if int_count(rows, key) != 0:
                reasons.append(key)
        for key in REMOTE_OWNER_BRANCH_ROUTING_LOWERING_EXPECTED_POSITIVE:
            if int_count(rows, key) <= 0:
                reasons.append(key)
    if remote_owner_branch_route_body_preflight_profile(rows):
        if rows.get("replacement_front_producer") != "mir_to_llvm_lowering":
            reasons.append("replacement_front_producer")
        if rows.get("replacement_front_selected_route") != (
            "remote_owner_branch_route_body_preflight"
        ):
            reasons.append("replacement_front_selected_route")
        if rows.get("replacement_front_selected_memop_family") != "remote_free_routing":
            reasons.append("replacement_front_selected_memop_family")
        if rows.get("replacement_front_selected_memop_kinds") != "RemoteOwnerBranchRouting":
            reasons.append("replacement_front_selected_memop_kinds")
        if rows.get("replacement_front_next_producer_slice") != "fastmem_branch_cfg_preflight":
            reasons.append("replacement_front_next_producer_slice")
        if "BranchCfgLowering" not in rows.get(
            "replacement_front_deferred_memop_kinds", ""
        ).split(","):
            reasons.append("replacement_front_deferred_memop_kinds")
        if "SameRemoteFreeBody" not in rows.get(
            "replacement_front_deferred_memop_kinds", ""
        ).split(","):
            reasons.append("replacement_front_deferred_memop_kinds")
        for key in REMOTE_OWNER_BRANCH_ROUTE_BODY_PREFLIGHT_EXPECTED_ZERO:
            if int_count(rows, key) != 0:
                reasons.append(key)
        for key in REMOTE_OWNER_BRANCH_ROUTE_BODY_PREFLIGHT_EXPECTED_POSITIVE:
            if int_count(rows, key) <= 0:
                reasons.append(key)
    if fastmem_branch_cfg_preflight_profile(rows):
        if rows.get("replacement_front_producer") != "mir_to_llvm_lowering":
            reasons.append("replacement_front_producer")
        if rows.get("replacement_front_selected_route") != "fastmem_branch_cfg_preflight":
            reasons.append("replacement_front_selected_route")
        if rows.get("replacement_front_selected_memop_family") != "remote_free_routing":
            reasons.append("replacement_front_selected_memop_family")
        if rows.get("replacement_front_selected_memop_kinds") != "RemoteOwnerBranchRouting":
            reasons.append("replacement_front_selected_memop_kinds")
        if rows.get("replacement_front_next_producer_slice") != (
            "fastmem_branch_cfg_lowering_preflight"
        ):
            reasons.append("replacement_front_next_producer_slice")
        if rows.get("fastmem_branch_cfg_source_guard") != "branch_cfg_closed":
            reasons.append("fastmem_branch_cfg_source_guard")
        if "BranchCfgLowering" not in rows.get(
            "replacement_front_deferred_memop_kinds", ""
        ).split(","):
            reasons.append("replacement_front_deferred_memop_kinds")
        if "SameRemoteFreeBody" not in rows.get(
            "replacement_front_deferred_memop_kinds", ""
        ).split(","):
            reasons.append("replacement_front_deferred_memop_kinds")
        for key in FASTMEM_BRANCH_CFG_PREFLIGHT_EXPECTED_ZERO:
            if int_count(rows, key) != 0:
                reasons.append(key)
        for key in FASTMEM_BRANCH_CFG_PREFLIGHT_EXPECTED_POSITIVE:
            if int_count(rows, key) <= 0:
                reasons.append(key)
    if fastmem_branch_cfg_lowering_preflight_profile(rows):
        if rows.get("replacement_front_producer") != "mir_to_llvm_lowering":
            reasons.append("replacement_front_producer")
        if rows.get("replacement_front_selected_route") != (
            "fastmem_branch_cfg_lowering_preflight"
        ):
            reasons.append("replacement_front_selected_route")
        if rows.get("replacement_front_selected_memop_family") != "branch_cfg":
            reasons.append("replacement_front_selected_memop_family")
        if rows.get("replacement_front_selected_memop_kinds") != "FastMemBranchCfg":
            reasons.append("replacement_front_selected_memop_kinds")
        if rows.get("replacement_front_next_producer_slice") != (
            "fastmem_branch_cfg_lowering_producer_pilot"
        ):
            reasons.append("replacement_front_next_producer_slice")
        if rows.get("fastmem_branch_cfg_source_guard") != "branch_cfg_closed":
            reasons.append("fastmem_branch_cfg_source_guard")
        if "BranchCfgLoweringProducer" not in rows.get(
            "replacement_front_deferred_memop_kinds", ""
        ).split(","):
            reasons.append("replacement_front_deferred_memop_kinds")
        if "SameRemoteFreeBody" not in rows.get(
            "replacement_front_deferred_memop_kinds", ""
        ).split(","):
            reasons.append("replacement_front_deferred_memop_kinds")
        for key in FASTMEM_BRANCH_CFG_LOWERING_PREFLIGHT_EXPECTED_ZERO:
            if int_count(rows, key) != 0:
                reasons.append(key)
        for key in FASTMEM_BRANCH_CFG_LOWERING_PREFLIGHT_EXPECTED_POSITIVE:
            if int_count(rows, key) <= 0:
                reasons.append(key)
    if fastmem_branch_cfg_lowering_profile(rows):
        if rows.get("replacement_front_producer") != "mir_to_llvm_lowering":
            reasons.append("replacement_front_producer")
        if rows.get("replacement_front_selected_route") != (
            "fastmem_branch_cfg_lowering_producer_pilot"
        ):
            reasons.append("replacement_front_selected_route")
        if rows.get("replacement_front_selected_memop_family") != "branch_cfg":
            reasons.append("replacement_front_selected_memop_family")
        if rows.get("replacement_front_selected_memop_kinds") != "FastMemBranchCfg":
            reasons.append("replacement_front_selected_memop_kinds")
        if rows.get("replacement_front_next_producer_slice") != (
            "same_remote_free_body_preflight"
        ):
            reasons.append("replacement_front_next_producer_slice")
        if rows.get("fastmem_branch_cfg_source_guard") != "branch_cfg_open":
            reasons.append("fastmem_branch_cfg_source_guard")
        if "SameRemoteFreeBody" not in rows.get(
            "replacement_front_deferred_memop_kinds", ""
        ).split(","):
            reasons.append("replacement_front_deferred_memop_kinds")
        for key in FASTMEM_BRANCH_CFG_LOWERING_PRODUCER_EXPECTED_ZERO:
            if int_count(rows, key) != 0:
                reasons.append(key)
        for key in FASTMEM_BRANCH_CFG_LOWERING_PRODUCER_EXPECTED_POSITIVE:
            if int_count(rows, key) <= 0:
                reasons.append(key)
    return reasons


def render(rows: dict[str, str], reasons: list[str]) -> str:
    status = "OK" if not reasons else "FAILED"
    lines = [
        f"FastMemory check: {status}",
        "",
        "Contract",
        "  output_contract=hako-check-fastmem-check-v0",
        f"  source_contract={rows.get('output_contract', 'unknown')}",
        f"  tool_surface={rows.get('tool_surface', 'unknown')}",
        "",
        "Regions",
        f"  fastmem regions: {rows.get('fastmem_region_count', '0')}",
        f"  fastmem contracts: {rows.get('fastmem_contract_count', '0')}",
        f"  unclassified memops: {rows.get('fastmem_memop_unclassified_count', '0')}",
        f"  unbalanced regions: {rows.get('fastmem_memop_unbalanced_region_count', '0')}",
        "",
        "Boundaries",
        f"  type ABI hot lookup: {rows.get('type_abi_hot_path_lookup_count', '0')}",
        f"  provider hot dispatch: {rows.get('provider_dispatch_hot_path', '0')}",
        f"  fastmem runtime contract lookup: {rows.get('fastmem_contract_runtime_lookup_count', '0')}",
        "",
        "Machine",
        f"  failure_count={len(reasons)}",
    ]
    for idx, reason in enumerate(reasons):
        lines.append(f"  failure_{idx}_reason={reason}")
    lines.append("  summary=ok" if not reasons else "  summary=failed")
    return "\n".join(lines) + "\n"


def emit_kv(rows: dict[str, str], reasons: list[str]) -> str:
    out = [
        "output_contract=hako-check-fastmem-check-v0",
        "input_kind=fastmem_inventory",
        "tool_surface=hako_check_fastmem_check",
        "observation_only=1",
        "rewrite_executed=0",
        "source_rewrite_executed=0",
        "benchmark_run_executed=0",
        "keeper_selection=0",
        f"source_contract={rows.get('output_contract', 'unknown')}",
        f"failure_count={len(reasons)}",
    ]
    for idx, reason in enumerate(reasons):
        out.append(f"failure_{idx}_reason={reason}")
    out.append("summary=ok" if not reasons else "summary=failed")
    return "\n".join(out) + "\n"


def write_output(text: str, out: Path | None) -> None:
    if out is None:
        print(text, end="")
        return
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text(text, encoding="utf-8")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    source = parser.add_mutually_exclusive_group(required=True)
    source.add_argument("--report", type=Path, help="Read a benchmark report via inventory.")
    source.add_argument("--inventory", type=Path, help="Read an existing fastmem inventory kv file.")
    source.add_argument("--ast-json", type=Path, help="Read Rust AST JSON via inventory.")
    source.add_argument("--program-json", type=Path, help="Read Program(JSON v0) via inventory.")
    source.add_argument("--mir-json", type=Path, help="Read MIR JSON via inventory.")
    parser.add_argument("--format", choices=("kv", "text"), default="text")
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    if args.report:
        rows = run_inventory("--report", args.report)
    elif args.ast_json:
        rows = run_inventory("--ast-json", args.ast_json)
    elif args.program_json:
        rows = run_inventory("--program-json", args.program_json)
    elif args.mir_json:
        rows = run_inventory("--mir-json", args.mir_json)
    else:
        rows = read_kv(args.inventory)
    reasons = failure_reasons(rows)
    text = emit_kv(rows, reasons) if args.format == "kv" else render(rows, reasons)
    write_output(text, args.out)
    return 1 if reasons else 0


if __name__ == "__main__":
    raise SystemExit(main())
