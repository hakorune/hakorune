"""Shared FastMemory route/profile helpers.

This module keeps the route vocabulary used by the FastMemory report/check
adapters in one place so the large scripts do not duplicate the same profile
switches and expected-field sets.
"""

from __future__ import annotations

from typing import Callable, Mapping


def _profile_flag(rows: Mapping[str, str], key: str) -> bool:
    return int(rows.get(key, "0") or 0) > 0


def remote_owner_branch_routing_preflight_profile(rows: Mapping[str, str]) -> bool:
    return _profile_flag(rows, "fastmem_remote_owner_branch_routing_preflight")


def remote_owner_branch_routing_lowering_preflight_profile(
    rows: Mapping[str, str],
) -> bool:
    return _profile_flag(rows, "fastmem_remote_owner_branch_routing_lowering_preflight")


def remote_owner_branch_routing_lowering_profile(rows: Mapping[str, str]) -> bool:
    return _profile_flag(rows, "fastmem_remote_owner_branch_routing_lowering_producer_pilot")


def remote_owner_branch_route_body_preflight_profile(rows: Mapping[str, str]) -> bool:
    return _profile_flag(rows, "fastmem_remote_owner_branch_route_body_preflight")


def fastmem_branch_cfg_preflight_profile(rows: Mapping[str, str]) -> bool:
    return _profile_flag(rows, "fastmem_branch_cfg_preflight")


def fastmem_branch_cfg_lowering_preflight_profile(rows: Mapping[str, str]) -> bool:
    return _profile_flag(rows, "fastmem_branch_cfg_lowering_preflight")


def fastmem_branch_cfg_lowering_profile(rows: Mapping[str, str]) -> bool:
    return _profile_flag(rows, "fastmem_branch_cfg_lowering_producer_pilot")


def same_remote_free_body_preflight_profile(rows: Mapping[str, str]) -> bool:
    return _profile_flag(rows, "fastmem_same_remote_free_body_preflight")


def same_remote_free_body_producer_profile(rows: Mapping[str, str]) -> bool:
    return _profile_flag(rows, "fastmem_same_remote_free_body_producer_pilot")


def page_local_alloc_route_cfg_preflight_profile(rows: Mapping[str, str]) -> bool:
    return _profile_flag(rows, "fastmem_page_local_alloc_route_cfg_preflight")


def page_local_alloc_route_cfg_producer_profile(rows: Mapping[str, str]) -> bool:
    return _profile_flag(rows, "fastmem_page_local_alloc_route_cfg_producer_pilot")


def page_local_free_route_cfg_preflight_profile(rows: Mapping[str, str]) -> bool:
    return _profile_flag(rows, "fastmem_page_local_free_route_cfg_preflight")


def page_local_free_route_cfg_producer_profile(rows: Mapping[str, str]) -> bool:
    return _profile_flag(rows, "fastmem_page_local_free_route_cfg_producer_pilot")


def page_local_route_body_join_preflight_profile(rows: Mapping[str, str]) -> bool:
    return _profile_flag(rows, "fastmem_page_local_route_body_join_preflight")


def page_local_route_body_join_producer_profile(rows: Mapping[str, str]) -> bool:
    return _profile_flag(rows, "fastmem_page_local_route_body_join_producer_pilot")


def terminal_ladder_refresh_preflight_profile(rows: Mapping[str, str]) -> bool:
    return _profile_flag(rows, "fastmem_terminal_ladder_refresh_preflight")


def tls_backing_transfer_preflight_refresh_profile(rows: Mapping[str, str]) -> bool:
    return _profile_flag(rows, "fastmem_tls_backing_transfer_preflight_refresh")


def tls_backing_transfer_producer_refresh_profile(rows: Mapping[str, str]) -> bool:
    return _profile_flag(rows, "fastmem_tls_backing_transfer_producer_refresh")


def tls_backing_transfer_preflight_profile(rows: Mapping[str, str]) -> bool:
    return _profile_flag(rows, "fastmem_tls_backing_transfer_preflight")


def tls_backing_transfer_producer_profile(rows: Mapping[str, str]) -> bool:
    return _profile_flag(rows, "fastmem_tls_backing_transfer_producer_pilot")


def owner_slot_reuse_preflight_refresh_profile(rows: Mapping[str, str]) -> bool:
    return _profile_flag(rows, "fastmem_allocator_owner_slot_reuse_preflight_refresh")


def owner_slot_reuse_producer_refresh_profile(rows: Mapping[str, str]) -> bool:
    return _profile_flag(rows, "fastmem_allocator_owner_slot_reuse_producer_refresh")


def owner_slot_reuse_preflight_profile(rows: Mapping[str, str]) -> bool:
    return _profile_flag(rows, "fastmem_allocator_owner_slot_reuse_preflight")


def owner_slot_reuse_producer_profile(rows: Mapping[str, str]) -> bool:
    return _profile_flag(rows, "fastmem_allocator_owner_slot_reuse_producer_pilot")


def abandoned_reclaim_preflight_profile(rows: Mapping[str, str]) -> bool:
    return _profile_flag(rows, "fastmem_abandoned_reclaim_preflight")


def abandoned_reclaim_producer_profile(rows: Mapping[str, str]) -> bool:
    return _profile_flag(rows, "fastmem_abandoned_reclaim_producer_pilot")


def product_activation_preflight_profile(rows: Mapping[str, str]) -> bool:
    return _profile_flag(rows, "fastmem_product_activation_preflight")


def product_activation_producer_profile(rows: Mapping[str, str]) -> bool:
    return _profile_flag(rows, "fastmem_product_activation_producer_pilot")


def hook_install_preflight_profile(rows: Mapping[str, str]) -> bool:
    return _profile_flag(rows, "fastmem_hook_install_preflight")


def hook_install_producer_profile(rows: Mapping[str, str]) -> bool:
    return _profile_flag(rows, "fastmem_hook_install_producer_pilot")


def global_allocator_claim_preflight_profile(rows: Mapping[str, str]) -> bool:
    return _profile_flag(rows, "fastmem_global_allocator_claim_preflight")


def global_allocator_claim_producer_profile(rows: Mapping[str, str]) -> bool:
    return _profile_flag(rows, "fastmem_global_allocator_claim_producer_pilot")


def winner_claim_preflight_profile(rows: Mapping[str, str]) -> bool:
    return _profile_flag(rows, "fastmem_winner_claim_preflight")


def winner_claim_producer_profile(rows: Mapping[str, str]) -> bool:
    return _profile_flag(rows, "fastmem_winner_claim_producer_pilot")


PAGE_LOCAL_ALLOC_ROUTE_CFG_PREFLIGHT_EXPECTED_ZERO = (
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

PAGE_LOCAL_ALLOC_ROUTE_CFG_PREFLIGHT_EXPECTED_POSITIVE = (
    "fastmem_page_local_alloc_route_cfg_preflight",
    "page_local_alloc_route_cfg_selected",
    "page_local_alloc_route_report_v0",
)

PAGE_LOCAL_ALLOC_ROUTE_CFG_PRODUCER_EXPECTED_ZERO = (
    "page_local_alloc_route_branch_claim",
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

PAGE_LOCAL_ALLOC_ROUTE_CFG_PRODUCER_EXPECTED_POSITIVE = (
    "fastmem_page_local_alloc_route_cfg_producer_pilot",
    "page_local_alloc_route_cfg_selected",
    "page_local_alloc_route_cfg_lowering_enabled",
    "page_local_alloc_route_report_v0",
    "fastmem_branch_cfg_selected",
    "fastmem_branch_cfg_open",
    "fastmem_branch_cfg_lowered_count",
    "memop_current_alloc_owner_id_lowered_count",
    "memop_owner_eq_lowered_count",
    "memop_local_free_pop_lowered_count",
    "memop_free_head_pop_lowered_count",
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

PAGE_LOCAL_ROUTE_BODY_JOIN_PREFLIGHT_EXPECTED_ZERO = (
    "page_local_route_body_join_open",
    "page_local_alloc_route_branch_claim",
    "page_local_free_route_branch_claim",
    "tls_backing_transfer_enabled",
    "allocator_owner_slot_reuse_enabled",
    "type_abi_hot_lookup_count",
    "provider_abi_hot_dispatch_count",
    "product_activation",
    "hook_install",
    "global_allocator_claim",
    "winner_claim",
)

PAGE_LOCAL_ROUTE_BODY_JOIN_PREFLIGHT_EXPECTED_POSITIVE = (
    "fastmem_page_local_route_body_join_preflight",
    "page_local_route_body_join_selected",
    "page_local_alloc_route_cfg_selected",
    "page_local_alloc_route_cfg_lowering_enabled",
    "page_local_alloc_route_report_v0",
    "page_local_free_route_cfg_selected",
    "page_local_free_route_cfg_lowering_enabled",
    "page_local_free_route_report_v0",
    "fastmem_branch_cfg_selected",
    "fastmem_branch_cfg_open",
    "fastmem_branch_cfg_lowered_count",
    "memop_current_alloc_owner_id_lowered_count",
    "memop_owner_eq_lowered_count",
)

PAGE_LOCAL_ROUTE_BODY_JOIN_PRODUCER_EXPECTED_ZERO = (
    "page_local_alloc_route_branch_claim",
    "page_local_free_route_branch_claim",
    "tls_backing_transfer_enabled",
    "allocator_owner_slot_reuse_enabled",
    "type_abi_hot_lookup_count",
    "provider_abi_hot_dispatch_count",
    "product_activation",
    "hook_install",
    "global_allocator_claim",
    "winner_claim",
)

PAGE_LOCAL_ROUTE_BODY_JOIN_PRODUCER_EXPECTED_POSITIVE = (
    "fastmem_page_local_route_body_join_producer_pilot",
    "page_local_route_body_join_selected",
    "page_local_route_body_join_open",
    "page_local_alloc_route_cfg_selected",
    "page_local_alloc_route_cfg_lowering_enabled",
    "page_local_alloc_route_report_v0",
    "page_local_free_route_cfg_selected",
    "page_local_free_route_cfg_lowering_enabled",
    "page_local_free_route_report_v0",
    "fastmem_branch_cfg_selected",
    "fastmem_branch_cfg_open",
    "fastmem_branch_cfg_lowered_count",
    "memop_current_alloc_owner_id_lowered_count",
    "memop_owner_eq_lowered_count",
)

TERMINAL_LADDER_REFRESH_PREFLIGHT_EXPECTED_ZERO = (
    "terminal_ladder_refresh_open",
    "tls_backing_transfer_enabled",
    "allocator_owner_slot_reuse_enabled",
    "type_abi_hot_lookup_count",
    "provider_abi_hot_dispatch_count",
    "product_activation",
    "hook_install",
    "global_allocator_claim",
    "winner_claim",
)

TERMINAL_LADDER_REFRESH_PREFLIGHT_EXPECTED_POSITIVE = (
    "fastmem_terminal_ladder_refresh_preflight",
    "terminal_ladder_refresh_selected",
    "page_local_route_body_join_selected",
    "page_local_route_body_join_open",
    "page_local_alloc_route_cfg_selected",
    "page_local_alloc_route_cfg_lowering_enabled",
    "page_local_alloc_route_report_v0",
    "page_local_free_route_cfg_selected",
    "page_local_free_route_cfg_lowering_enabled",
    "page_local_free_route_report_v0",
)

TLS_BACKING_TRANSFER_PREFLIGHT_REFRESH_EXPECTED_ZERO = (
    "tls_backing_transfer_enabled",
    "allocator_owner_slot_reuse_enabled",
    "type_abi_hot_lookup_count",
    "provider_abi_hot_dispatch_count",
    "product_activation",
    "hook_install",
    "global_allocator_claim",
    "winner_claim",
)

TLS_BACKING_TRANSFER_PREFLIGHT_REFRESH_EXPECTED_POSITIVE = (
    "fastmem_tls_backing_transfer_preflight_refresh",
    "terminal_ladder_refresh_selected",
    "terminal_ladder_refresh_open",
    "tls_backing_transfer_selected",
    "page_local_route_body_join_selected",
    "page_local_route_body_join_open",
    "page_local_alloc_route_cfg_selected",
    "page_local_alloc_route_cfg_lowering_enabled",
    "page_local_free_route_cfg_selected",
    "page_local_free_route_cfg_lowering_enabled",
)

TLS_BACKING_TRANSFER_PRODUCER_REFRESH_EXPECTED_ZERO = (
    "allocator_owner_slot_reuse_enabled",
    "type_abi_hot_lookup_count",
    "provider_abi_hot_dispatch_count",
    "product_activation",
    "hook_install",
    "global_allocator_claim",
    "winner_claim",
)

TLS_BACKING_TRANSFER_PRODUCER_REFRESH_EXPECTED_POSITIVE = (
    "fastmem_tls_backing_transfer_producer_refresh",
    "terminal_ladder_refresh_selected",
    "terminal_ladder_refresh_open",
    "tls_backing_transfer_selected",
    "tls_backing_transfer_enabled",
    "page_local_route_body_join_selected",
    "page_local_route_body_join_open",
    "page_local_alloc_route_cfg_selected",
    "page_local_alloc_route_cfg_lowering_enabled",
    "page_local_free_route_cfg_selected",
    "page_local_free_route_cfg_lowering_enabled",
)

OWNER_SLOT_REUSE_PREFLIGHT_REFRESH_EXPECTED_ZERO = (
    "allocator_owner_slot_reuse_enabled",
    "allocator_owner_reuse_without_generation_bump_count",
    "type_abi_hot_lookup_count",
    "provider_abi_hot_dispatch_count",
    "product_activation",
    "hook_install",
    "global_allocator_claim",
    "winner_claim",
)

OWNER_SLOT_REUSE_PREFLIGHT_REFRESH_EXPECTED_POSITIVE = (
    "fastmem_allocator_owner_slot_reuse_preflight_refresh",
    "terminal_ladder_refresh_open",
    "page_local_route_body_join_open",
    "tls_backing_transfer_selected",
    "tls_backing_transfer_enabled",
    "allocator_owner_slot_reuse_selected",
    "page_local_alloc_route_cfg_lowering_enabled",
    "page_local_free_route_cfg_lowering_enabled",
)

OWNER_SLOT_REUSE_PRODUCER_REFRESH_EXPECTED_ZERO = (
    "allocator_owner_reuse_without_generation_bump_count",
    "abandoned_reclaim_enabled",
    "type_abi_hot_lookup_count",
    "provider_abi_hot_dispatch_count",
    "product_activation",
    "hook_install",
    "global_allocator_claim",
    "winner_claim",
)

OWNER_SLOT_REUSE_PRODUCER_REFRESH_EXPECTED_POSITIVE = (
    "fastmem_allocator_owner_slot_reuse_producer_refresh",
    "terminal_ladder_refresh_open",
    "page_local_route_body_join_open",
    "tls_backing_transfer_selected",
    "tls_backing_transfer_enabled",
    "allocator_owner_slot_reuse_selected",
    "allocator_owner_slot_reuse_enabled",
    "allocator_owner_generation_bump_count",
    "page_local_alloc_route_cfg_lowering_enabled",
    "page_local_free_route_cfg_lowering_enabled",
)
