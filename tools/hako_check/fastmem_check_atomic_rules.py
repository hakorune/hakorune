#!/usr/bin/env python3
"""FastMemory atomic-remote progression rules."""

from __future__ import annotations

from fastmem_check_profile_functions import *

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

def check_atomic_rules(rows: dict[str, str]) -> list[str]:
    reasons: list[str] = []
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
        if rows.get("atomic_remote_head_drain_exchange_order") != "acquire":
            reasons.append("atomic_remote_head_drain_exchange_order")
        if rows.get("atomic_remote_head_drain_result_kind") != "remote_free_list_token":
            reasons.append("atomic_remote_head_drain_result_kind")
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
    return reasons
