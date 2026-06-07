#!/usr/bin/env python3
"""FastMemory terminal progression rules."""

from __future__ import annotations

from fastmem_check_profile_functions import *

TLS_BACKING_TRANSFER_PREFLIGHT_EXPECTED_ZERO = (
    "tls_backing_transfer_enabled",
    "allocator_owner_slot_reuse_enabled",
    "type_abi_hot_lookup_count",
    "provider_abi_hot_dispatch_count",
    "product_activation",
    "hook_install",
    "global_allocator_claim",
    "winner_claim",
)
TLS_BACKING_TRANSFER_PREFLIGHT_EXPECTED_POSITIVE = (
    "fastmem_tls_backing_transfer_preflight",
    "tls_backing_transfer_selected",
    "page_local_free_route_cfg_selected",
    "page_local_free_route_cfg_lowering_enabled",
    "same_remote_free_body_selected",
    "same_remote_free_body_open",
    "fastmem_branch_cfg_selected",
    "fastmem_branch_cfg_open",
    "remote_owner_branch_routing_selected",
    "remote_owner_branch_routing_lowering_selected",
    "remote_owner_branch_routing_open",
    "remote_owner_branch_route_body_selected",
)
TLS_BACKING_TRANSFER_PRODUCER_EXPECTED_ZERO = (
    "allocator_owner_slot_reuse_enabled",
    "type_abi_hot_lookup_count",
    "provider_abi_hot_dispatch_count",
    "product_activation",
    "hook_install",
    "global_allocator_claim",
    "winner_claim",
)
TLS_BACKING_TRANSFER_PRODUCER_EXPECTED_POSITIVE = (
    "fastmem_tls_backing_transfer_producer_pilot",
    "tls_backing_transfer_selected",
    "tls_backing_transfer_enabled",
    "page_local_free_route_cfg_selected",
    "page_local_free_route_cfg_lowering_enabled",
    "same_remote_free_body_selected",
    "same_remote_free_body_open",
    "fastmem_branch_cfg_selected",
    "fastmem_branch_cfg_open",
    "remote_owner_branch_routing_selected",
    "remote_owner_branch_routing_lowering_selected",
    "remote_owner_branch_routing_open",
    "remote_owner_branch_route_body_selected",
)
OWNER_SLOT_REUSE_PREFLIGHT_EXPECTED_ZERO = (
    "allocator_owner_slot_reuse_enabled",
    "allocator_owner_reuse_without_generation_bump_count",
    "type_abi_hot_lookup_count",
    "provider_abi_hot_dispatch_count",
    "product_activation",
    "hook_install",
    "global_allocator_claim",
    "winner_claim",
)
OWNER_SLOT_REUSE_PREFLIGHT_EXPECTED_POSITIVE = (
    "fastmem_allocator_owner_slot_reuse_preflight",
    "allocator_owner_slot_reuse_selected",
    "tls_backing_transfer_selected",
    "tls_backing_transfer_enabled",
    "page_local_free_route_cfg_selected",
    "page_local_free_route_cfg_lowering_enabled",
    "same_remote_free_body_selected",
    "same_remote_free_body_open",
    "fastmem_branch_cfg_selected",
    "fastmem_branch_cfg_open",
    "remote_owner_branch_routing_selected",
    "remote_owner_branch_routing_lowering_selected",
    "remote_owner_branch_routing_open",
    "remote_owner_branch_route_body_selected",
)
OWNER_SLOT_REUSE_PRODUCER_EXPECTED_ZERO = (
    "allocator_owner_reuse_without_generation_bump_count",
    "type_abi_hot_lookup_count",
    "provider_abi_hot_dispatch_count",
    "product_activation",
    "hook_install",
    "global_allocator_claim",
    "winner_claim",
)
OWNER_SLOT_REUSE_PRODUCER_EXPECTED_POSITIVE = (
    "fastmem_allocator_owner_slot_reuse_producer_pilot",
    "allocator_owner_slot_reuse_selected",
    "allocator_owner_slot_reuse_enabled",
    "allocator_owner_generation_bump_count",
    "tls_backing_transfer_selected",
    "tls_backing_transfer_enabled",
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
ABANDONED_RECLAIM_PREFLIGHT_EXPECTED_ZERO = (
    "abandoned_reclaim_enabled",
    "page_reclaimed_with_remote_candidates",
    "allocator_owner_reuse_without_generation_bump_count",
    "type_abi_hot_lookup_count",
    "provider_abi_hot_dispatch_count",
    "product_activation",
    "hook_install",
    "global_allocator_claim",
    "winner_claim",
)
ABANDONED_RECLAIM_PREFLIGHT_EXPECTED_POSITIVE = (
    "fastmem_abandoned_reclaim_preflight",
    "abandoned_reclaim_selected",
    "allocator_owner_slot_reuse_selected",
    "allocator_owner_slot_reuse_enabled",
    "allocator_owner_generation_bump_count",
    "tls_backing_transfer_selected",
    "tls_backing_transfer_enabled",
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
ABANDONED_RECLAIM_PRODUCER_EXPECTED_ZERO = (
    "page_reclaimed_with_remote_candidates",
    "allocator_owner_reuse_without_generation_bump_count",
    "type_abi_hot_lookup_count",
    "provider_abi_hot_dispatch_count",
    "product_activation",
    "hook_install",
    "global_allocator_claim",
    "winner_claim",
)
ABANDONED_RECLAIM_PRODUCER_EXPECTED_POSITIVE = (
    "fastmem_abandoned_reclaim_producer_pilot",
    "abandoned_reclaim_selected",
    "abandoned_reclaim_enabled",
    "allocator_owner_slot_reuse_selected",
    "allocator_owner_slot_reuse_enabled",
    "allocator_owner_generation_bump_count",
    "tls_backing_transfer_selected",
    "tls_backing_transfer_enabled",
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
PRODUCT_ACTIVATION_PREFLIGHT_EXPECTED_ZERO = (
    "page_reclaimed_with_remote_candidates",
    "allocator_owner_reuse_without_generation_bump_count",
    "type_abi_hot_lookup_count",
    "provider_abi_hot_dispatch_count",
    "product_activation",
    "hook_install",
    "global_allocator_claim",
    "winner_claim",
)
PRODUCT_ACTIVATION_PREFLIGHT_EXPECTED_POSITIVE = (
    "fastmem_product_activation_preflight",
    "product_activation_selected",
    "abandoned_reclaim_selected",
    "abandoned_reclaim_enabled",
    "allocator_owner_slot_reuse_selected",
    "allocator_owner_slot_reuse_enabled",
    "allocator_owner_generation_bump_count",
    "tls_backing_transfer_selected",
    "tls_backing_transfer_enabled",
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
PRODUCT_ACTIVATION_PRODUCER_EXPECTED_ZERO = (
    "page_reclaimed_with_remote_candidates",
    "allocator_owner_reuse_without_generation_bump_count",
    "type_abi_hot_lookup_count",
    "provider_abi_hot_dispatch_count",
    "hook_install",
    "global_allocator_claim",
    "winner_claim",
)
PRODUCT_ACTIVATION_PRODUCER_EXPECTED_POSITIVE = (
    "fastmem_product_activation_producer_pilot",
    "product_activation_selected",
    "product_activation",
    "abandoned_reclaim_selected",
    "abandoned_reclaim_enabled",
    "allocator_owner_slot_reuse_selected",
    "allocator_owner_slot_reuse_enabled",
    "allocator_owner_generation_bump_count",
    "tls_backing_transfer_selected",
    "tls_backing_transfer_enabled",
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
HOOK_INSTALL_PREFLIGHT_EXPECTED_ZERO = (
    "page_reclaimed_with_remote_candidates",
    "allocator_owner_reuse_without_generation_bump_count",
    "type_abi_hot_lookup_count",
    "provider_abi_hot_dispatch_count",
    "hook_install",
    "global_allocator_claim",
    "winner_claim",
)
HOOK_INSTALL_PREFLIGHT_EXPECTED_POSITIVE = (
    "fastmem_hook_install_preflight",
    "hook_install_selected",
    "product_activation_selected",
    "product_activation",
    "abandoned_reclaim_selected",
    "abandoned_reclaim_enabled",
    "allocator_owner_slot_reuse_selected",
    "allocator_owner_slot_reuse_enabled",
    "allocator_owner_generation_bump_count",
    "tls_backing_transfer_selected",
    "tls_backing_transfer_enabled",
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
HOOK_INSTALL_PRODUCER_EXPECTED_ZERO = (
    "page_reclaimed_with_remote_candidates",
    "allocator_owner_reuse_without_generation_bump_count",
    "type_abi_hot_lookup_count",
    "provider_abi_hot_dispatch_count",
    "global_allocator_claim",
    "winner_claim",
)
HOOK_INSTALL_PRODUCER_EXPECTED_POSITIVE = (
    "fastmem_hook_install_producer_pilot",
    "hook_install_selected",
    "hook_install",
    "product_activation_selected",
    "product_activation",
    "abandoned_reclaim_selected",
    "abandoned_reclaim_enabled",
    "allocator_owner_slot_reuse_selected",
    "allocator_owner_slot_reuse_enabled",
    "allocator_owner_generation_bump_count",
    "tls_backing_transfer_selected",
    "tls_backing_transfer_enabled",
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
GLOBAL_ALLOCATOR_CLAIM_PREFLIGHT_EXPECTED_ZERO = (
    "page_reclaimed_with_remote_candidates",
    "allocator_owner_reuse_without_generation_bump_count",
    "type_abi_hot_lookup_count",
    "provider_abi_hot_dispatch_count",
    "global_allocator_claim",
    "winner_claim",
)
GLOBAL_ALLOCATOR_CLAIM_PREFLIGHT_EXPECTED_POSITIVE = (
    "fastmem_global_allocator_claim_preflight",
    "global_allocator_claim_selected",
    "hook_install_selected",
    "hook_install",
    "product_activation_selected",
    "product_activation",
    "abandoned_reclaim_selected",
    "abandoned_reclaim_enabled",
    "allocator_owner_slot_reuse_selected",
    "allocator_owner_slot_reuse_enabled",
    "allocator_owner_generation_bump_count",
    "tls_backing_transfer_selected",
    "tls_backing_transfer_enabled",
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
GLOBAL_ALLOCATOR_CLAIM_PRODUCER_EXPECTED_ZERO = (
    "page_reclaimed_with_remote_candidates",
    "allocator_owner_reuse_without_generation_bump_count",
    "type_abi_hot_lookup_count",
    "provider_abi_hot_dispatch_count",
    "winner_claim",
)
GLOBAL_ALLOCATOR_CLAIM_PRODUCER_EXPECTED_POSITIVE = (
    "fastmem_global_allocator_claim_producer_pilot",
    "global_allocator_claim_selected",
    "global_allocator_claim",
    "hook_install_selected",
    "hook_install",
    "product_activation_selected",
    "product_activation",
    "abandoned_reclaim_selected",
    "abandoned_reclaim_enabled",
    "allocator_owner_slot_reuse_selected",
    "allocator_owner_slot_reuse_enabled",
    "allocator_owner_generation_bump_count",
    "tls_backing_transfer_selected",
    "tls_backing_transfer_enabled",
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
WINNER_CLAIM_PREFLIGHT_EXPECTED_ZERO = (
    "page_reclaimed_with_remote_candidates",
    "allocator_owner_reuse_without_generation_bump_count",
    "type_abi_hot_lookup_count",
    "provider_abi_hot_dispatch_count",
    "winner_claim",
)
WINNER_CLAIM_PREFLIGHT_EXPECTED_POSITIVE = (
    "fastmem_winner_claim_preflight",
    "winner_claim_selected",
    "global_allocator_claim_selected",
    "global_allocator_claim",
    "hook_install_selected",
    "hook_install",
    "product_activation_selected",
    "product_activation",
    "abandoned_reclaim_selected",
    "abandoned_reclaim_enabled",
    "allocator_owner_slot_reuse_selected",
    "allocator_owner_slot_reuse_enabled",
    "allocator_owner_generation_bump_count",
    "tls_backing_transfer_selected",
    "tls_backing_transfer_enabled",
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
WINNER_CLAIM_PRODUCER_EXPECTED_ZERO = (
    "page_reclaimed_with_remote_candidates",
    "allocator_owner_reuse_without_generation_bump_count",
    "type_abi_hot_lookup_count",
    "provider_abi_hot_dispatch_count",
)
WINNER_CLAIM_PRODUCER_EXPECTED_POSITIVE = (
    "fastmem_winner_claim_producer_pilot",
    "winner_claim_selected",
    "winner_claim",
    "global_allocator_claim_selected",
    "global_allocator_claim",
    "hook_install_selected",
    "hook_install",
    "product_activation_selected",
    "product_activation",
    "abandoned_reclaim_selected",
    "abandoned_reclaim_enabled",
    "allocator_owner_slot_reuse_selected",
    "allocator_owner_slot_reuse_enabled",
    "allocator_owner_generation_bump_count",
    "tls_backing_transfer_selected",
    "tls_backing_transfer_enabled",
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
def check_terminal_rules(rows: dict[str, str]) -> list[str]:
    reasons: list[str] = []
    if tls_backing_transfer_preflight_profile(rows):
        if rows.get("replacement_front_producer") != "mir_to_llvm_lowering":
            reasons.append("replacement_front_producer")
        if rows.get("replacement_front_selected_route") != "tls_backing_transfer_preflight":
            reasons.append("replacement_front_selected_route")
        if rows.get("replacement_front_selected_memop_family") != "page_local_route_cfg":
            reasons.append("replacement_front_selected_memop_family")
        if rows.get("replacement_front_selected_memop_kinds") != "PageLocalFreeRouteCfg":
            reasons.append("replacement_front_selected_memop_kinds")
        if rows.get("replacement_front_next_producer_slice") != (
            "tls_backing_transfer_producer_pilot"
        ):
            reasons.append("replacement_front_next_producer_slice")
        if rows.get("fastmem_branch_cfg_source_guard") != "branch_cfg_open":
            reasons.append("fastmem_branch_cfg_source_guard")
        if "OwnerSlotReuse" not in rows.get(
            "replacement_front_deferred_memop_kinds", ""
        ).split(","):
            reasons.append("replacement_front_deferred_memop_kinds")
        for key in TLS_BACKING_TRANSFER_PREFLIGHT_EXPECTED_ZERO:
            if int_count(rows, key) != 0:
                reasons.append(key)
        for key in TLS_BACKING_TRANSFER_PREFLIGHT_EXPECTED_POSITIVE:
            if key.endswith("_lowered_count"):
                continue
            if int_count(rows, key) <= 0:
                reasons.append(key)
    if tls_backing_transfer_producer_profile(rows):
        if rows.get("replacement_front_producer") != "mir_to_llvm_lowering":
            reasons.append("replacement_front_producer")
        if rows.get("replacement_front_selected_route") != (
            "tls_backing_transfer_producer_pilot"
        ):
            reasons.append("replacement_front_selected_route")
        if rows.get("replacement_front_selected_memop_family") != "tls_backing_transfer":
            reasons.append("replacement_front_selected_memop_family")
        if rows.get("replacement_front_selected_memop_kinds") != "TlsBackingTransfer":
            reasons.append("replacement_front_selected_memop_kinds")
        if rows.get("replacement_front_next_producer_slice") != (
            "owner_slot_reuse_preflight"
        ):
            reasons.append("replacement_front_next_producer_slice")
        if rows.get("fastmem_branch_cfg_source_guard") != "branch_cfg_open":
            reasons.append("fastmem_branch_cfg_source_guard")
        if "OwnerSlotReuse" not in rows.get(
            "replacement_front_deferred_memop_kinds", ""
        ).split(","):
            reasons.append("replacement_front_deferred_memop_kinds")
        for key in TLS_BACKING_TRANSFER_PRODUCER_EXPECTED_ZERO:
            if int_count(rows, key) != 0:
                reasons.append(key)
        for key in TLS_BACKING_TRANSFER_PRODUCER_EXPECTED_POSITIVE:
            if key.endswith("_lowered_count"):
                continue
            if int_count(rows, key) <= 0:
                reasons.append(key)
    if owner_slot_reuse_preflight_profile(rows):
        if rows.get("replacement_front_producer") != "mir_to_llvm_lowering":
            reasons.append("replacement_front_producer")
        if rows.get("replacement_front_selected_route") != "owner_slot_reuse_preflight":
            reasons.append("replacement_front_selected_route")
        if rows.get("replacement_front_selected_memop_family") != "owner_slot_reuse":
            reasons.append("replacement_front_selected_memop_family")
        if rows.get("replacement_front_selected_memop_kinds") != "OwnerSlotReuse":
            reasons.append("replacement_front_selected_memop_kinds")
        if rows.get("replacement_front_next_producer_slice") != (
            "owner_slot_reuse_producer_pilot"
        ):
            reasons.append("replacement_front_next_producer_slice")
        if rows.get("fastmem_branch_cfg_source_guard") != "branch_cfg_open":
            reasons.append("fastmem_branch_cfg_source_guard")
        deferred = rows.get("replacement_front_deferred_memop_kinds", "").split(",")
        if "OwnerSlotReuseProducer" not in deferred:
            reasons.append("replacement_front_deferred_memop_kinds")
        if "AbandonedReclaim" not in deferred:
            reasons.append("replacement_front_deferred_memop_kinds")
        for key in OWNER_SLOT_REUSE_PREFLIGHT_EXPECTED_ZERO:
            if int_count(rows, key) != 0:
                reasons.append(key)
        for key in OWNER_SLOT_REUSE_PREFLIGHT_EXPECTED_POSITIVE:
            if key.endswith("_lowered_count"):
                continue
            if int_count(rows, key) <= 0:
                reasons.append(key)
    if owner_slot_reuse_producer_profile(rows):
        if rows.get("replacement_front_producer") != "mir_to_llvm_lowering":
            reasons.append("replacement_front_producer")
        if rows.get("replacement_front_selected_route") != (
            "owner_slot_reuse_producer_pilot"
        ):
            reasons.append("replacement_front_selected_route")
        if rows.get("replacement_front_selected_memop_family") != "owner_slot_reuse":
            reasons.append("replacement_front_selected_memop_family")
        if rows.get("replacement_front_selected_memop_kinds") != "OwnerSlotReuse":
            reasons.append("replacement_front_selected_memop_kinds")
        if rows.get("replacement_front_next_producer_slice") != (
            "abandoned_reclaim_preflight"
        ):
            reasons.append("replacement_front_next_producer_slice")
        if rows.get("fastmem_branch_cfg_source_guard") != "branch_cfg_open":
            reasons.append("fastmem_branch_cfg_source_guard")
        if "AbandonedReclaim" not in rows.get(
            "replacement_front_deferred_memop_kinds", ""
        ).split(","):
            reasons.append("replacement_front_deferred_memop_kinds")
        for key in OWNER_SLOT_REUSE_PRODUCER_EXPECTED_ZERO:
            if int_count(rows, key) != 0:
                reasons.append(key)
        for key in OWNER_SLOT_REUSE_PRODUCER_EXPECTED_POSITIVE:
            if key.endswith("_lowered_count"):
                continue
            if int_count(rows, key) <= 0:
                reasons.append(key)
    if abandoned_reclaim_preflight_profile(rows):
        if rows.get("replacement_front_producer") != "mir_to_llvm_lowering":
            reasons.append("replacement_front_producer")
        if rows.get("replacement_front_selected_route") != "abandoned_reclaim_preflight":
            reasons.append("replacement_front_selected_route")
        if rows.get("replacement_front_selected_memop_family") != "abandoned_reclaim":
            reasons.append("replacement_front_selected_memop_family")
        if rows.get("replacement_front_selected_memop_kinds") != "AbandonedReclaim":
            reasons.append("replacement_front_selected_memop_kinds")
        if rows.get("replacement_front_next_producer_slice") != (
            "abandoned_reclaim_producer_pilot"
        ):
            reasons.append("replacement_front_next_producer_slice")
        if rows.get("fastmem_branch_cfg_source_guard") != "branch_cfg_open":
            reasons.append("fastmem_branch_cfg_source_guard")
        deferred = rows.get("replacement_front_deferred_memop_kinds", "").split(",")
        if "AbandonedReclaimProducer" not in deferred:
            reasons.append("replacement_front_deferred_memop_kinds")
        if "ProductActivation" not in deferred:
            reasons.append("replacement_front_deferred_memop_kinds")
        for key in ABANDONED_RECLAIM_PREFLIGHT_EXPECTED_ZERO:
            if int_count(rows, key) != 0:
                reasons.append(key)
        for key in ABANDONED_RECLAIM_PREFLIGHT_EXPECTED_POSITIVE:
            if key.endswith("_lowered_count"):
                continue
            if int_count(rows, key) <= 0:
                reasons.append(key)
    if abandoned_reclaim_producer_profile(rows):
        if rows.get("replacement_front_producer") != "mir_to_llvm_lowering":
            reasons.append("replacement_front_producer")
        if rows.get("replacement_front_selected_route") != (
            "abandoned_reclaim_producer_pilot"
        ):
            reasons.append("replacement_front_selected_route")
        if rows.get("replacement_front_selected_memop_family") != "abandoned_reclaim":
            reasons.append("replacement_front_selected_memop_family")
        if rows.get("replacement_front_selected_memop_kinds") != "AbandonedReclaim":
            reasons.append("replacement_front_selected_memop_kinds")
        if rows.get("replacement_front_next_producer_slice") != (
            "product_activation_preflight"
        ):
            reasons.append("replacement_front_next_producer_slice")
        if rows.get("fastmem_branch_cfg_source_guard") != "branch_cfg_open":
            reasons.append("fastmem_branch_cfg_source_guard")
        if "ProductActivation" not in rows.get(
            "replacement_front_deferred_memop_kinds", ""
        ).split(","):
            reasons.append("replacement_front_deferred_memop_kinds")
        for key in ABANDONED_RECLAIM_PRODUCER_EXPECTED_ZERO:
            if int_count(rows, key) != 0:
                reasons.append(key)
        for key in ABANDONED_RECLAIM_PRODUCER_EXPECTED_POSITIVE:
            if key.endswith("_lowered_count"):
                continue
            if int_count(rows, key) <= 0:
                reasons.append(key)
    if product_activation_preflight_profile(rows):
        if rows.get("replacement_front_producer") != "mir_to_llvm_lowering":
            reasons.append("replacement_front_producer")
        if rows.get("replacement_front_selected_route") != "product_activation_preflight":
            reasons.append("replacement_front_selected_route")
        if rows.get("replacement_front_selected_memop_family") != "product_activation":
            reasons.append("replacement_front_selected_memop_family")
        if rows.get("replacement_front_selected_memop_kinds") != "ProductActivation":
            reasons.append("replacement_front_selected_memop_kinds")
        if rows.get("replacement_front_next_producer_slice") != (
            "product_activation_producer_pilot"
        ):
            reasons.append("replacement_front_next_producer_slice")
        if rows.get("fastmem_branch_cfg_source_guard") != "branch_cfg_open":
            reasons.append("fastmem_branch_cfg_source_guard")
        deferred = rows.get("replacement_front_deferred_memop_kinds", "").split(",")
        if "ProductActivationProducer" not in deferred:
            reasons.append("replacement_front_deferred_memop_kinds")
        if "HookInstall" not in deferred:
            reasons.append("replacement_front_deferred_memop_kinds")
        for key in PRODUCT_ACTIVATION_PREFLIGHT_EXPECTED_ZERO:
            if int_count(rows, key) != 0:
                reasons.append(key)
        for key in PRODUCT_ACTIVATION_PREFLIGHT_EXPECTED_POSITIVE:
            if key.endswith("_lowered_count"):
                continue
            if int_count(rows, key) <= 0:
                reasons.append(key)
    if product_activation_producer_profile(rows):
        if rows.get("replacement_front_producer") != "mir_to_llvm_lowering":
            reasons.append("replacement_front_producer")
        if rows.get("replacement_front_selected_route") != (
            "product_activation_producer_pilot"
        ):
            reasons.append("replacement_front_selected_route")
        if rows.get("replacement_front_selected_memop_family") != "product_activation":
            reasons.append("replacement_front_selected_memop_family")
        if rows.get("replacement_front_selected_memop_kinds") != "ProductActivation":
            reasons.append("replacement_front_selected_memop_kinds")
        if rows.get("replacement_front_next_producer_slice") != "hook_install_preflight":
            reasons.append("replacement_front_next_producer_slice")
        if rows.get("fastmem_branch_cfg_source_guard") != "branch_cfg_open":
            reasons.append("fastmem_branch_cfg_source_guard")
        if "HookInstall" not in rows.get(
            "replacement_front_deferred_memop_kinds", ""
        ).split(","):
            reasons.append("replacement_front_deferred_memop_kinds")
        for key in PRODUCT_ACTIVATION_PRODUCER_EXPECTED_ZERO:
            if int_count(rows, key) != 0:
                reasons.append(key)
        for key in PRODUCT_ACTIVATION_PRODUCER_EXPECTED_POSITIVE:
            if key.endswith("_lowered_count"):
                continue
            if int_count(rows, key) <= 0:
                reasons.append(key)
    if hook_install_preflight_profile(rows):
        if rows.get("replacement_front_producer") != "mir_to_llvm_lowering":
            reasons.append("replacement_front_producer")
        if rows.get("replacement_front_selected_route") != "hook_install_preflight":
            reasons.append("replacement_front_selected_route")
        if rows.get("replacement_front_selected_memop_family") != "hook_install":
            reasons.append("replacement_front_selected_memop_family")
        if rows.get("replacement_front_selected_memop_kinds") != "HookInstall":
            reasons.append("replacement_front_selected_memop_kinds")
        if rows.get("replacement_front_next_producer_slice") != "hook_install_producer_pilot":
            reasons.append("replacement_front_next_producer_slice")
        if rows.get("fastmem_branch_cfg_source_guard") != "branch_cfg_open":
            reasons.append("fastmem_branch_cfg_source_guard")
        deferred = rows.get("replacement_front_deferred_memop_kinds", "").split(",")
        if "HookInstallProducer" not in deferred:
            reasons.append("replacement_front_deferred_memop_kinds")
        if "GlobalAllocatorClaim" not in deferred:
            reasons.append("replacement_front_deferred_memop_kinds")
        for key in HOOK_INSTALL_PREFLIGHT_EXPECTED_ZERO:
            if int_count(rows, key) != 0:
                reasons.append(key)
        for key in HOOK_INSTALL_PREFLIGHT_EXPECTED_POSITIVE:
            if key.endswith("_lowered_count"):
                continue
            if int_count(rows, key) <= 0:
                reasons.append(key)
    if hook_install_producer_profile(rows):
        if rows.get("replacement_front_producer") != "mir_to_llvm_lowering":
            reasons.append("replacement_front_producer")
        if rows.get("replacement_front_selected_route") != "hook_install_producer_pilot":
            reasons.append("replacement_front_selected_route")
        if rows.get("replacement_front_selected_memop_family") != "hook_install":
            reasons.append("replacement_front_selected_memop_family")
        if rows.get("replacement_front_selected_memop_kinds") != "HookInstall":
            reasons.append("replacement_front_selected_memop_kinds")
        if rows.get("replacement_front_next_producer_slice") != (
            "global_allocator_claim_preflight"
        ):
            reasons.append("replacement_front_next_producer_slice")
        if rows.get("fastmem_branch_cfg_source_guard") != "branch_cfg_open":
            reasons.append("fastmem_branch_cfg_source_guard")
        if "GlobalAllocatorClaim" not in rows.get(
            "replacement_front_deferred_memop_kinds", ""
        ).split(","):
            reasons.append("replacement_front_deferred_memop_kinds")
        for key in HOOK_INSTALL_PRODUCER_EXPECTED_ZERO:
            if int_count(rows, key) != 0:
                reasons.append(key)
        for key in HOOK_INSTALL_PRODUCER_EXPECTED_POSITIVE:
            if key.endswith("_lowered_count"):
                continue
            if int_count(rows, key) <= 0:
                reasons.append(key)
    if global_allocator_claim_preflight_profile(rows):
        if rows.get("replacement_front_producer") != "mir_to_llvm_lowering":
            reasons.append("replacement_front_producer")
        if rows.get("replacement_front_selected_route") != (
            "global_allocator_claim_preflight"
        ):
            reasons.append("replacement_front_selected_route")
        if rows.get("replacement_front_selected_memop_family") != (
            "global_allocator_claim"
        ):
            reasons.append("replacement_front_selected_memop_family")
        if rows.get("replacement_front_selected_memop_kinds") != (
            "GlobalAllocatorClaim"
        ):
            reasons.append("replacement_front_selected_memop_kinds")
        if rows.get("replacement_front_next_producer_slice") != (
            "global_allocator_claim_producer_pilot"
        ):
            reasons.append("replacement_front_next_producer_slice")
        if rows.get("fastmem_branch_cfg_source_guard") != "branch_cfg_open":
            reasons.append("fastmem_branch_cfg_source_guard")
        deferred = rows.get("replacement_front_deferred_memop_kinds", "").split(",")
        if "GlobalAllocatorClaimProducer" not in deferred:
            reasons.append("replacement_front_deferred_memop_kinds")
        if "WinnerClaim" not in deferred:
            reasons.append("replacement_front_deferred_memop_kinds")
        for key in GLOBAL_ALLOCATOR_CLAIM_PREFLIGHT_EXPECTED_ZERO:
            if int_count(rows, key) != 0:
                reasons.append(key)
        for key in GLOBAL_ALLOCATOR_CLAIM_PREFLIGHT_EXPECTED_POSITIVE:
            if key.endswith("_lowered_count"):
                continue
            if int_count(rows, key) <= 0:
                reasons.append(key)
    if global_allocator_claim_producer_profile(rows):
        if rows.get("replacement_front_producer") != "mir_to_llvm_lowering":
            reasons.append("replacement_front_producer")
        if rows.get("replacement_front_selected_route") != (
            "global_allocator_claim_producer_pilot"
        ):
            reasons.append("replacement_front_selected_route")
        if rows.get("replacement_front_selected_memop_family") != (
            "global_allocator_claim"
        ):
            reasons.append("replacement_front_selected_memop_family")
        if rows.get("replacement_front_selected_memop_kinds") != (
            "GlobalAllocatorClaim"
        ):
            reasons.append("replacement_front_selected_memop_kinds")
        if rows.get("replacement_front_next_producer_slice") != "winner_claim_preflight":
            reasons.append("replacement_front_next_producer_slice")
        if rows.get("fastmem_branch_cfg_source_guard") != "branch_cfg_open":
            reasons.append("fastmem_branch_cfg_source_guard")
        if "WinnerClaim" not in rows.get(
            "replacement_front_deferred_memop_kinds", ""
        ).split(","):
            reasons.append("replacement_front_deferred_memop_kinds")
        for key in GLOBAL_ALLOCATOR_CLAIM_PRODUCER_EXPECTED_ZERO:
            if int_count(rows, key) != 0:
                reasons.append(key)
        for key in GLOBAL_ALLOCATOR_CLAIM_PRODUCER_EXPECTED_POSITIVE:
            if key.endswith("_lowered_count"):
                continue
            if int_count(rows, key) <= 0:
                reasons.append(key)
    if winner_claim_preflight_profile(rows):
        if rows.get("replacement_front_producer") != "mir_to_llvm_lowering":
            reasons.append("replacement_front_producer")
        if rows.get("replacement_front_selected_route") != "winner_claim_preflight":
            reasons.append("replacement_front_selected_route")
        if rows.get("replacement_front_selected_memop_family") != "winner_claim":
            reasons.append("replacement_front_selected_memop_family")
        if rows.get("replacement_front_selected_memop_kinds") != "WinnerClaim":
            reasons.append("replacement_front_selected_memop_kinds")
        if rows.get("replacement_front_next_producer_slice") != (
            "winner_claim_producer_pilot"
        ):
            reasons.append("replacement_front_next_producer_slice")
        if rows.get("fastmem_branch_cfg_source_guard") != "branch_cfg_open":
            reasons.append("fastmem_branch_cfg_source_guard")
        if "WinnerClaimProducer" not in rows.get(
            "replacement_front_deferred_memop_kinds", ""
        ).split(","):
            reasons.append("replacement_front_deferred_memop_kinds")
        for key in WINNER_CLAIM_PREFLIGHT_EXPECTED_ZERO:
            if int_count(rows, key) != 0:
                reasons.append(key)
        for key in WINNER_CLAIM_PREFLIGHT_EXPECTED_POSITIVE:
            if key.endswith("_lowered_count"):
                continue
            if int_count(rows, key) <= 0:
                reasons.append(key)
    if winner_claim_producer_profile(rows):
        if rows.get("replacement_front_producer") != "mir_to_llvm_lowering":
            reasons.append("replacement_front_producer")
        if rows.get("replacement_front_selected_route") != "winner_claim_producer_pilot":
            reasons.append("replacement_front_selected_route")
        if rows.get("replacement_front_selected_memop_family") != "winner_claim":
            reasons.append("replacement_front_selected_memop_family")
        if rows.get("replacement_front_selected_memop_kinds") != "WinnerClaim":
            reasons.append("replacement_front_selected_memop_kinds")
        if rows.get("replacement_front_next_producer_slice") != "complete":
            reasons.append("replacement_front_next_producer_slice")
        if rows.get("fastmem_branch_cfg_source_guard") != "branch_cfg_open":
            reasons.append("fastmem_branch_cfg_source_guard")
        if rows.get("replacement_front_deferred_memop_kinds") != "none":
            reasons.append("replacement_front_deferred_memop_kinds")
        for key in WINNER_CLAIM_PRODUCER_EXPECTED_ZERO:
            if int_count(rows, key) != 0:
                reasons.append(key)
        for key in WINNER_CLAIM_PRODUCER_EXPECTED_POSITIVE:
            if key.endswith("_lowered_count"):
                continue
            if int_count(rows, key) <= 0:
                reasons.append(key)
    return reasons
