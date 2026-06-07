#!/usr/bin/env python3
"""FastMemory remote-route progression rules."""

from __future__ import annotations

from fastmem_route_profiles import (
    PAGE_LOCAL_ALLOC_ROUTE_CFG_PRODUCER_EXPECTED_POSITIVE,
    PAGE_LOCAL_ALLOC_ROUTE_CFG_PRODUCER_EXPECTED_ZERO,
    PAGE_LOCAL_ALLOC_ROUTE_CFG_PREFLIGHT_EXPECTED_POSITIVE,
    PAGE_LOCAL_ALLOC_ROUTE_CFG_PREFLIGHT_EXPECTED_ZERO,
    page_local_alloc_route_cfg_producer_profile,
    page_local_alloc_route_cfg_preflight_profile,
)
from fastmem_check_profile_functions import *

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
SAME_REMOTE_FREE_BODY_PREFLIGHT_EXPECTED_ZERO = (
    "same_remote_free_body_open",
    "same_remote_free_body_lowered_count",
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
SAME_REMOTE_FREE_BODY_PREFLIGHT_EXPECTED_POSITIVE = (
    "fastmem_same_remote_free_body_preflight",
    "same_remote_free_body_selected",
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
SAME_REMOTE_FREE_BODY_PRODUCER_EXPECTED_ZERO = (
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
SAME_REMOTE_FREE_BODY_PRODUCER_EXPECTED_POSITIVE = (
    "fastmem_same_remote_free_body_producer_pilot",
    "same_remote_free_body_selected",
    "same_remote_free_body_open",
    "same_remote_free_body_lowered_count",
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
PAGE_LOCAL_FREE_ROUTE_CFG_PREFLIGHT_EXPECTED_ZERO = (
    "page_local_free_route_cfg_lowering_enabled",
    "page_local_alloc_route_branch_claim",
    "page_local_alloc_route_cfg_lowering_enabled",
    "page_local_free_route_branch_claim",
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
PAGE_LOCAL_FREE_ROUTE_CFG_PREFLIGHT_EXPECTED_POSITIVE = (
    "fastmem_page_local_free_route_cfg_preflight",
    "page_local_free_route_cfg_selected",
    "same_remote_free_body_selected",
    "same_remote_free_body_open",
    "same_remote_free_body_lowered_count",
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
PAGE_LOCAL_FREE_ROUTE_CFG_PRODUCER_EXPECTED_ZERO = (
    "page_local_alloc_route_branch_claim",
    "page_local_alloc_route_cfg_lowering_enabled",
    "page_local_free_route_branch_claim",
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
PAGE_LOCAL_FREE_ROUTE_CFG_PRODUCER_EXPECTED_POSITIVE = (
    "fastmem_page_local_free_route_cfg_producer_pilot",
    "page_local_free_route_cfg_selected",
    "page_local_free_route_cfg_lowering_enabled",
    "same_remote_free_body_selected",
    "same_remote_free_body_open",
    "same_remote_free_body_lowered_count",
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

def check_route_rules(rows: dict[str, str]) -> list[str]:
    reasons: list[str] = []
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
    if same_remote_free_body_preflight_profile(rows):
        if rows.get("replacement_front_producer") != "mir_to_llvm_lowering":
            reasons.append("replacement_front_producer")
        if rows.get("replacement_front_selected_route") != (
            "same_remote_free_body_preflight"
        ):
            reasons.append("replacement_front_selected_route")
        if rows.get("replacement_front_selected_memop_family") != "same_remote_free_body":
            reasons.append("replacement_front_selected_memop_family")
        if rows.get("replacement_front_selected_memop_kinds") != "SameRemoteFreeBody":
            reasons.append("replacement_front_selected_memop_kinds")
        if rows.get("replacement_front_next_producer_slice") != (
            "same_remote_free_body_producer_pilot"
        ):
            reasons.append("replacement_front_next_producer_slice")
        if rows.get("fastmem_branch_cfg_source_guard") != "branch_cfg_open":
            reasons.append("fastmem_branch_cfg_source_guard")
        if "SameRemoteFreeBodyProducer" not in rows.get(
            "replacement_front_deferred_memop_kinds", ""
        ).split(","):
            reasons.append("replacement_front_deferred_memop_kinds")
        for key in SAME_REMOTE_FREE_BODY_PREFLIGHT_EXPECTED_ZERO:
            if int_count(rows, key) != 0:
                reasons.append(key)
        for key in SAME_REMOTE_FREE_BODY_PREFLIGHT_EXPECTED_POSITIVE:
            if int_count(rows, key) <= 0:
                reasons.append(key)
    if same_remote_free_body_producer_profile(rows):
        if rows.get("replacement_front_producer") != "mir_to_llvm_lowering":
            reasons.append("replacement_front_producer")
        if rows.get("replacement_front_selected_route") != (
            "same_remote_free_body_producer_pilot"
        ):
            reasons.append("replacement_front_selected_route")
        if rows.get("replacement_front_selected_memop_family") != "same_remote_free_body":
            reasons.append("replacement_front_selected_memop_family")
        if rows.get("replacement_front_selected_memop_kinds") != "SameRemoteFreeBody":
            reasons.append("replacement_front_selected_memop_kinds")
        if rows.get("replacement_front_next_producer_slice") != (
            "page_local_free_route_cfg_preflight"
        ):
            reasons.append("replacement_front_next_producer_slice")
        if rows.get("fastmem_branch_cfg_source_guard") != "branch_cfg_open":
            reasons.append("fastmem_branch_cfg_source_guard")
        if "PageLocalFreeRouteCfg" not in rows.get(
            "replacement_front_deferred_memop_kinds", ""
        ).split(","):
            reasons.append("replacement_front_deferred_memop_kinds")
        for key in SAME_REMOTE_FREE_BODY_PRODUCER_EXPECTED_ZERO:
            if int_count(rows, key) != 0:
                reasons.append(key)
        for key in SAME_REMOTE_FREE_BODY_PRODUCER_EXPECTED_POSITIVE:
            if int_count(rows, key) <= 0:
                reasons.append(key)
    if page_local_alloc_route_cfg_preflight_profile(rows):
        if rows.get("replacement_front_producer") != "mir_to_llvm_lowering":
            reasons.append("replacement_front_producer")
        if rows.get("replacement_front_selected_route") != (
            "page_local_alloc_route_cfg_preflight"
        ):
            reasons.append("replacement_front_selected_route")
        if rows.get("replacement_front_selected_memop_family") != (
            "page_local_alloc_route_cfg"
        ):
            reasons.append("replacement_front_selected_memop_family")
        if rows.get("replacement_front_selected_memop_kinds") != (
            "PageLocalAllocRouteCfg"
        ):
            reasons.append("replacement_front_selected_memop_kinds")
        if rows.get("replacement_front_next_producer_slice") != (
            "page_local_alloc_route_cfg_producer_pilot"
        ):
            reasons.append("replacement_front_next_producer_slice")
        if rows.get("fastmem_branch_cfg_source_guard") != "branch_cfg_open":
            reasons.append("fastmem_branch_cfg_source_guard")
        if "PageLocalAllocRouteCfgProducer" not in rows.get(
            "replacement_front_deferred_memop_kinds", ""
        ).split(","):
            reasons.append("replacement_front_deferred_memop_kinds")
        for key in PAGE_LOCAL_ALLOC_ROUTE_CFG_PREFLIGHT_EXPECTED_ZERO:
            if int_count(rows, key) != 0:
                reasons.append(key)
        for key in PAGE_LOCAL_ALLOC_ROUTE_CFG_PREFLIGHT_EXPECTED_POSITIVE:
            if int_count(rows, key) <= 0:
                reasons.append(key)
        return reasons
    if page_local_alloc_route_cfg_producer_profile(rows):
        if rows.get("replacement_front_producer") != "mir_to_llvm_lowering":
            reasons.append("replacement_front_producer")
        if rows.get("replacement_front_selected_route") != (
            "page_local_alloc_route_cfg_producer_pilot"
        ):
            reasons.append("replacement_front_selected_route")
        if rows.get("replacement_front_selected_memop_family") != (
            "page_local_alloc_route_cfg"
        ):
            reasons.append("replacement_front_selected_memop_family")
        if rows.get("replacement_front_selected_memop_kinds") != (
            "PageLocalAllocRouteCfgProducer"
        ):
            reasons.append("replacement_front_selected_memop_kinds")
        if rows.get("replacement_front_next_producer_slice") != (
            "page_local_free_route_cfg_preflight"
        ):
            reasons.append("replacement_front_next_producer_slice")
        if rows.get("fastmem_branch_cfg_source_guard") != "branch_cfg_open":
            reasons.append("fastmem_branch_cfg_source_guard")
        if "PageLocalFreeRouteCfg" not in rows.get(
            "replacement_front_deferred_memop_kinds", ""
        ).split(","):
            reasons.append("replacement_front_deferred_memop_kinds")
        for key in PAGE_LOCAL_ALLOC_ROUTE_CFG_PRODUCER_EXPECTED_ZERO:
            if int_count(rows, key) != 0:
                reasons.append(key)
        for key in PAGE_LOCAL_ALLOC_ROUTE_CFG_PRODUCER_EXPECTED_POSITIVE:
            if int_count(rows, key) <= 0:
                reasons.append(key)
        return reasons
    if page_local_free_route_cfg_preflight_profile(rows):
        if rows.get("replacement_front_producer") != "mir_to_llvm_lowering":
            reasons.append("replacement_front_producer")
        if rows.get("replacement_front_selected_route") != (
            "page_local_free_route_cfg_preflight"
        ):
            reasons.append("replacement_front_selected_route")
        if rows.get("replacement_front_selected_memop_family") != "page_local_route_cfg":
            reasons.append("replacement_front_selected_memop_family")
        if rows.get("replacement_front_selected_memop_kinds") != "PageLocalFreeRouteCfg":
            reasons.append("replacement_front_selected_memop_kinds")
        if rows.get("replacement_front_next_producer_slice") != (
            "page_local_free_route_cfg_producer_pilot"
        ):
            reasons.append("replacement_front_next_producer_slice")
        if rows.get("fastmem_branch_cfg_source_guard") != "branch_cfg_open":
            reasons.append("fastmem_branch_cfg_source_guard")
        if "PageLocalFreeRouteCfgProducer" not in rows.get(
            "replacement_front_deferred_memop_kinds", ""
        ).split(","):
            reasons.append("replacement_front_deferred_memop_kinds")
        for key in PAGE_LOCAL_FREE_ROUTE_CFG_PREFLIGHT_EXPECTED_ZERO:
            if int_count(rows, key) != 0:
                reasons.append(key)
        for key in PAGE_LOCAL_FREE_ROUTE_CFG_PREFLIGHT_EXPECTED_POSITIVE:
            if int_count(rows, key) <= 0:
                reasons.append(key)
    if page_local_free_route_cfg_producer_profile(rows):
        if rows.get("replacement_front_producer") != "mir_to_llvm_lowering":
            reasons.append("replacement_front_producer")
        if rows.get("replacement_front_selected_route") != (
            "page_local_free_route_cfg_producer_pilot"
        ):
            reasons.append("replacement_front_selected_route")
        if rows.get("replacement_front_selected_memop_family") != "page_local_route_cfg":
            reasons.append("replacement_front_selected_memop_family")
        if rows.get("replacement_front_selected_memop_kinds") != "PageLocalFreeRouteCfg":
            reasons.append("replacement_front_selected_memop_kinds")
        if rows.get("replacement_front_next_producer_slice") != (
            "tls_backing_transfer_preflight"
        ):
            reasons.append("replacement_front_next_producer_slice")
        if rows.get("fastmem_branch_cfg_source_guard") != "branch_cfg_open":
            reasons.append("fastmem_branch_cfg_source_guard")
        if "TlsBackingTransfer" not in rows.get(
            "replacement_front_deferred_memop_kinds", ""
        ).split(","):
            reasons.append("replacement_front_deferred_memop_kinds")
        for key in PAGE_LOCAL_FREE_ROUTE_CFG_PRODUCER_EXPECTED_ZERO:
            if int_count(rows, key) != 0:
                reasons.append(key)
        for key in PAGE_LOCAL_FREE_ROUTE_CFG_PRODUCER_EXPECTED_POSITIVE:
            if int_count(rows, key) <= 0:
                reasons.append(key)
    return reasons
