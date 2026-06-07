"""Shared FastMemory route/profile helpers.

This module keeps the route vocabulary used by the FastMemory report/check
adapters in one place so the large scripts do not duplicate the same profile
switches and expected-field sets.
"""

from __future__ import annotations

from dataclasses import dataclass
from typing import Mapping


@dataclass(frozen=True)
class RefreshProfileSpec:
    profile: str
    report_flag: str
    selected_route: str
    family: str
    memop_kinds: str
    next_slice: str
    deferred_kinds: str
    expected_zero: tuple[str, ...]
    expected_positive: tuple[str, ...]


@dataclass(frozen=True)
class RemoteFreeRouteProfileSpec:
    profile: str
    selected_route: str
    next_slice: str
    deferred_kinds: str
    selected_remote_kind: str
    selected_memop_family: str
    selected_memop_kinds: str


FASTMEM_REMOTE_FREE_ROUTE_PROFILE_NAMES: tuple[str, ...] = (
    "remote-free",
    "remote-free-retry-preflight",
    "remote-free-retry",
    "remote-free-drain-preflight",
    "remote-free-drain-exchange-selection",
    "remote-free-drain-exchange",
    "remote-free-drain-to-local-selection",
    "remote-free-drain-to-local",
    "remote-free-drain-local-list-mutation-preflight",
    "remote-free-drain-local-list-mutation-proof",
    "remote-free-drain-local-list-mutation-vocabulary-preflight",
    "remote-free-drain-local-list-mutation-verifier-preconditions",
    "remote-free-drain-local-list-mutation-lowering",
    "remote-owner-branch-routing-preflight",
    "remote-owner-branch-routing-lowering-preflight",
    "remote-owner-branch-routing-lowering",
    "remote-owner-branch-route-body-preflight",
    "fastmem-branch-cfg-preflight",
    "fastmem-branch-cfg-lowering-preflight",
    "fastmem-branch-cfg-lowering",
    "same-remote-free-body-preflight",
    "same-remote-free-body",
    "page-local-alloc-route-cfg-preflight",
    "page-local-alloc-route-cfg",
    "page-local-free-route-cfg-preflight",
    "page-local-free-route-cfg",
    "page-local-route-body-join-preflight",
    "page-local-route-body-join",
    "terminal-ladder-refresh-preflight",
    "tls-backing-transfer-preflight-refresh",
    "tls-backing-transfer-producer-refresh",
    "owner-slot-reuse-preflight-refresh",
    "owner-slot-reuse-producer-refresh",
    "abandoned-reclaim-preflight-refresh",
    "abandoned-reclaim-producer-refresh",
    "product-activation-preflight-refresh",
    "product-activation-producer-refresh",
    "hook-install-preflight-refresh",
    "hook-install-producer-refresh",
    "global-allocator-claim-preflight-refresh",
    "global-allocator-claim-producer-refresh",
    "winner-claim-preflight-refresh",
    "winner-claim-producer-refresh",
    "tls-backing-transfer-preflight",
    "tls-backing-transfer-producer-pilot",
    "owner-slot-reuse-preflight",
    "owner-slot-reuse-producer-pilot",
    "abandoned-reclaim-preflight",
    "abandoned-reclaim-producer-pilot",
    "product-activation-preflight",
    "product-activation-producer-pilot",
    "hook-install-preflight",
    "hook-install-producer-pilot",
    "global-allocator-claim-preflight",
    "global-allocator-claim-producer-pilot",
    "winner-claim-preflight",
    "winner-claim-producer-pilot",
)


REMOTE_FREE_ROUTE_PROFILE_SPECS: tuple[RemoteFreeRouteProfileSpec, ...] = (
    RemoteFreeRouteProfileSpec(
        profile="remote-free-preflight",
        selected_route="none",
        next_slice="atomic_remote_head_cas_lowering_preflight",
        deferred_kinds="AtomicRemoteHeadCasLowering,AtomicRemoteHeadDrain,RemoteOwnerBranchRouting",
        selected_remote_kind="AtomicRemoteHeadPush",
        selected_memop_family="remote_free",
        selected_memop_kinds="AtomicRemoteHeadPush",
    ),
    RemoteFreeRouteProfileSpec(
        profile="remote-free",
        selected_route="none",
        next_slice="atomic_remote_head_cas_lowering_producer_pilot",
        deferred_kinds="AtomicRemoteHeadDrain,RemoteOwnerBranchRouting",
        selected_remote_kind="AtomicRemoteHeadPush",
        selected_memop_family="remote_free",
        selected_memop_kinds="AtomicRemoteHeadPush",
    ),
    RemoteFreeRouteProfileSpec(
        profile="remote-free-retry-preflight",
        selected_route="none",
        next_slice="atomic_remote_head_retry_policy_preflight",
        deferred_kinds="AtomicRemoteHeadRetryLowering,AtomicRemoteHeadDrain,RemoteOwnerBranchRouting",
        selected_remote_kind="AtomicRemoteHeadPush",
        selected_memop_family="remote_free",
        selected_memop_kinds="AtomicRemoteHeadPush",
    ),
    RemoteFreeRouteProfileSpec(
        profile="remote-free-retry",
        selected_route="none",
        next_slice="atomic_remote_head_retry_lowering_producer_pilot",
        deferred_kinds="AtomicRemoteHeadDrain,RemoteOwnerBranchRouting",
        selected_remote_kind="AtomicRemoteHeadPush",
        selected_memop_family="remote_free",
        selected_memop_kinds="AtomicRemoteHeadPush",
    ),
    RemoteFreeRouteProfileSpec(
        profile="remote-free-drain-preflight",
        selected_route="none",
        next_slice="atomic_remote_head_drain_preflight",
        deferred_kinds="AtomicRemoteHeadDrainLowering,RemoteOwnerBranchRouting",
        selected_remote_kind="AtomicRemoteHeadDrain",
        selected_memop_family="remote_free",
        selected_memop_kinds="AtomicRemoteHeadDrain",
    ),
    RemoteFreeRouteProfileSpec(
        profile="remote-free-drain-exchange-selection",
        selected_route="none",
        next_slice="atomic_remote_head_drain_exchange_lowering_producer_pilot",
        deferred_kinds="AtomicRemoteHeadDrainLowering,DrainToLocalRoute,RemoteOwnerBranchRouting",
        selected_remote_kind="AtomicRemoteHeadDrain",
        selected_memop_family="remote_free",
        selected_memop_kinds="AtomicRemoteHeadDrain",
    ),
    RemoteFreeRouteProfileSpec(
        profile="remote-free-drain-exchange",
        selected_route="none",
        next_slice="atomic_remote_head_drain_to_local_route_selection",
        deferred_kinds="DrainToLocalRoute,RemoteOwnerBranchRouting",
        selected_remote_kind="AtomicRemoteHeadDrain",
        selected_memop_family="remote_free",
        selected_memop_kinds="AtomicRemoteHeadDrain",
    ),
    RemoteFreeRouteProfileSpec(
        profile="remote-free-drain-to-local-selection",
        selected_route="none",
        next_slice="atomic_remote_head_drain_to_local_route_producer_pilot",
        deferred_kinds="DrainToLocalRouteLowering,RemoteOwnerBranchRouting",
        selected_remote_kind="AtomicRemoteHeadDrain",
        selected_memop_family="remote_free",
        selected_memop_kinds="AtomicRemoteHeadDrain",
    ),
    RemoteFreeRouteProfileSpec(
        profile="remote-free-drain-to-local",
        selected_route="none",
        next_slice="atomic_remote_head_drain_local_list_mutation_preflight",
        deferred_kinds="DrainLocalListMutation,RemoteOwnerBranchRouting",
        selected_remote_kind="AtomicRemoteHeadDrain",
        selected_memop_family="remote_free",
        selected_memop_kinds="AtomicRemoteHeadDrain",
    ),
    RemoteFreeRouteProfileSpec(
        profile="remote-free-drain-local-list-mutation-preflight",
        selected_route="none",
        next_slice="atomic_remote_head_drain_local_list_mutation_proof",
        deferred_kinds="DrainLocalListMutation,RemoteOwnerBranchRouting",
        selected_remote_kind="AtomicRemoteHeadDrain",
        selected_memop_family="remote_free",
        selected_memop_kinds="AtomicRemoteHeadDrain",
    ),
    RemoteFreeRouteProfileSpec(
        profile="remote-free-drain-local-list-mutation-proof",
        selected_route="none",
        next_slice="atomic_remote_head_drain_local_list_mutation_vocabulary_preflight",
        deferred_kinds="DrainRemoteListToLocalLowering,RemoteOwnerBranchRouting",
        selected_remote_kind="AtomicRemoteHeadDrain",
        selected_memop_family="remote_free",
        selected_memop_kinds="AtomicRemoteHeadDrain",
    ),
    RemoteFreeRouteProfileSpec(
        profile="remote-free-drain-local-list-mutation-vocabulary-preflight",
        selected_route="none",
        next_slice="atomic_remote_head_drain_local_list_mutation_verifier_preconditions",
        deferred_kinds="DrainRemoteListToLocalLowering,RemoteOwnerBranchRouting",
        selected_remote_kind="AtomicRemoteHeadDrain",
        selected_memop_family="remote_free",
        selected_memop_kinds="AtomicRemoteHeadDrain",
    ),
    RemoteFreeRouteProfileSpec(
        profile="remote-free-drain-local-list-mutation-verifier-preconditions",
        selected_route="none",
        next_slice="atomic_remote_head_drain_local_list_mutation_lowering_producer_pilot",
        deferred_kinds="DrainRemoteListToLocalLowering,RemoteOwnerBranchRouting",
        selected_remote_kind="DrainRemoteListToLocal",
        selected_memop_family="remote_free_routing",
        selected_memop_kinds="DrainRemoteListToLocal",
    ),
    RemoteFreeRouteProfileSpec(
        profile="remote-free-drain-local-list-mutation-lowering",
        selected_route="none",
        next_slice="remote_owner_branch_routing_preflight",
        deferred_kinds="RemoteOwnerBranchRouting",
        selected_remote_kind="DrainRemoteListToLocal",
        selected_memop_family="remote_free_routing",
        selected_memop_kinds="DrainRemoteListToLocal",
    ),
)


REMOTE_FREE_ROUTE_PROFILE_BY_NAME = {
    spec.profile: spec for spec in REMOTE_FREE_ROUTE_PROFILE_SPECS
}


def remote_free_route_profile_spec(profile: str) -> RemoteFreeRouteProfileSpec | None:
    return REMOTE_FREE_ROUTE_PROFILE_BY_NAME.get(profile)


def profile_in(profile: str, expected: tuple[str, ...]) -> bool:
    return profile in expected


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


def abandoned_reclaim_preflight_refresh_profile(rows: Mapping[str, str]) -> bool:
    return _profile_flag(rows, "fastmem_abandoned_reclaim_preflight_refresh")


def abandoned_reclaim_producer_refresh_profile(rows: Mapping[str, str]) -> bool:
    return _profile_flag(rows, "fastmem_abandoned_reclaim_producer_refresh")


def abandoned_reclaim_producer_profile(rows: Mapping[str, str]) -> bool:
    return _profile_flag(rows, "fastmem_abandoned_reclaim_producer_pilot")


def product_activation_preflight_profile(rows: Mapping[str, str]) -> bool:
    return _profile_flag(rows, "fastmem_product_activation_preflight")


def product_activation_preflight_refresh_profile(rows: Mapping[str, str]) -> bool:
    return _profile_flag(rows, "fastmem_product_activation_preflight_refresh")


def product_activation_producer_profile(rows: Mapping[str, str]) -> bool:
    return _profile_flag(rows, "fastmem_product_activation_producer_pilot")


def product_activation_producer_refresh_profile(rows: Mapping[str, str]) -> bool:
    return _profile_flag(rows, "fastmem_product_activation_producer_refresh")


def hook_install_preflight_profile(rows: Mapping[str, str]) -> bool:
    return _profile_flag(rows, "fastmem_hook_install_preflight")


def hook_install_preflight_refresh_profile(rows: Mapping[str, str]) -> bool:
    return _profile_flag(rows, "fastmem_hook_install_preflight_refresh")


def hook_install_producer_refresh_profile(rows: Mapping[str, str]) -> bool:
    return _profile_flag(rows, "fastmem_hook_install_producer_refresh")


def hook_install_producer_profile(rows: Mapping[str, str]) -> bool:
    return _profile_flag(rows, "fastmem_hook_install_producer_pilot")


def global_allocator_claim_preflight_profile(rows: Mapping[str, str]) -> bool:
    return _profile_flag(rows, "fastmem_global_allocator_claim_preflight")


def global_allocator_claim_preflight_refresh_profile(rows: Mapping[str, str]) -> bool:
    return _profile_flag(rows, "fastmem_global_allocator_claim_preflight_refresh")


def global_allocator_claim_producer_refresh_profile(rows: Mapping[str, str]) -> bool:
    return _profile_flag(rows, "fastmem_global_allocator_claim_producer_refresh")


def global_allocator_claim_producer_profile(rows: Mapping[str, str]) -> bool:
    return _profile_flag(rows, "fastmem_global_allocator_claim_producer_pilot")


def winner_claim_preflight_profile(rows: Mapping[str, str]) -> bool:
    return _profile_flag(rows, "fastmem_winner_claim_preflight")


def winner_claim_preflight_refresh_profile(rows: Mapping[str, str]) -> bool:
    return _profile_flag(rows, "fastmem_winner_claim_preflight_refresh")


def winner_claim_producer_refresh_profile(rows: Mapping[str, str]) -> bool:
    return _profile_flag(rows, "fastmem_winner_claim_producer_refresh")


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

ABANDONED_RECLAIM_PREFLIGHT_REFRESH_EXPECTED_ZERO = (
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

ABANDONED_RECLAIM_PREFLIGHT_REFRESH_EXPECTED_POSITIVE = (
    "fastmem_abandoned_reclaim_preflight_refresh",
    "terminal_ladder_refresh_open",
    "page_local_route_body_join_open",
    "tls_backing_transfer_enabled",
    "allocator_owner_slot_reuse_selected",
    "allocator_owner_slot_reuse_enabled",
    "allocator_owner_generation_bump_count",
    "abandoned_reclaim_selected",
    "page_local_alloc_route_cfg_lowering_enabled",
    "page_local_free_route_cfg_lowering_enabled",
)

ABANDONED_RECLAIM_PRODUCER_REFRESH_EXPECTED_ZERO = (
    "page_reclaimed_with_remote_candidates",
    "allocator_owner_reuse_without_generation_bump_count",
    "type_abi_hot_lookup_count",
    "provider_abi_hot_dispatch_count",
    "product_activation",
    "hook_install",
    "global_allocator_claim",
    "winner_claim",
)

ABANDONED_RECLAIM_PRODUCER_REFRESH_EXPECTED_POSITIVE = (
    "fastmem_abandoned_reclaim_producer_refresh",
    "terminal_ladder_refresh_open",
    "page_local_route_body_join_open",
    "tls_backing_transfer_enabled",
    "allocator_owner_slot_reuse_selected",
    "allocator_owner_slot_reuse_enabled",
    "allocator_owner_generation_bump_count",
    "abandoned_reclaim_selected",
    "abandoned_reclaim_enabled",
    "page_local_alloc_route_cfg_lowering_enabled",
    "page_local_free_route_cfg_lowering_enabled",
)

PRODUCT_ACTIVATION_PREFLIGHT_REFRESH_EXPECTED_ZERO = (
    "page_reclaimed_with_remote_candidates",
    "allocator_owner_reuse_without_generation_bump_count",
    "type_abi_hot_lookup_count",
    "provider_abi_hot_dispatch_count",
    "product_activation",
    "hook_install",
    "global_allocator_claim",
    "winner_claim",
)

PRODUCT_ACTIVATION_PREFLIGHT_REFRESH_EXPECTED_POSITIVE = (
    "fastmem_product_activation_preflight_refresh",
    "terminal_ladder_refresh_open",
    "page_local_route_body_join_open",
    "tls_backing_transfer_enabled",
    "allocator_owner_slot_reuse_enabled",
    "abandoned_reclaim_selected",
    "abandoned_reclaim_enabled",
    "product_activation_selected",
    "page_local_alloc_route_cfg_lowering_enabled",
    "page_local_free_route_cfg_lowering_enabled",
)

PRODUCT_ACTIVATION_PRODUCER_REFRESH_EXPECTED_ZERO = (
    "page_reclaimed_with_remote_candidates",
    "allocator_owner_reuse_without_generation_bump_count",
    "type_abi_hot_lookup_count",
    "provider_abi_hot_dispatch_count",
    "hook_install",
    "global_allocator_claim",
    "winner_claim",
)

PRODUCT_ACTIVATION_PRODUCER_REFRESH_EXPECTED_POSITIVE = (
    "fastmem_product_activation_producer_refresh",
    "terminal_ladder_refresh_open",
    "page_local_route_body_join_open",
    "tls_backing_transfer_enabled",
    "allocator_owner_slot_reuse_enabled",
    "abandoned_reclaim_selected",
    "abandoned_reclaim_enabled",
    "product_activation_selected",
    "product_activation",
    "page_local_alloc_route_cfg_lowering_enabled",
    "page_local_free_route_cfg_lowering_enabled",
)

HOOK_INSTALL_PREFLIGHT_REFRESH_EXPECTED_ZERO = (
    "page_reclaimed_with_remote_candidates",
    "allocator_owner_reuse_without_generation_bump_count",
    "type_abi_hot_lookup_count",
    "provider_abi_hot_dispatch_count",
    "hook_install",
    "global_allocator_claim",
    "winner_claim",
)

HOOK_INSTALL_PREFLIGHT_REFRESH_EXPECTED_POSITIVE = (
    "fastmem_hook_install_preflight_refresh",
    "terminal_ladder_refresh_open",
    "page_local_route_body_join_open",
    "product_activation_selected",
    "product_activation",
    "hook_install_selected",
    "page_local_alloc_route_cfg_lowering_enabled",
    "page_local_free_route_cfg_lowering_enabled",
)

HOOK_INSTALL_PRODUCER_REFRESH_EXPECTED_ZERO = (
    "page_reclaimed_with_remote_candidates",
    "allocator_owner_reuse_without_generation_bump_count",
    "type_abi_hot_lookup_count",
    "provider_abi_hot_dispatch_count",
    "global_allocator_claim",
    "winner_claim",
)

HOOK_INSTALL_PRODUCER_REFRESH_EXPECTED_POSITIVE = (
    "fastmem_hook_install_producer_refresh",
    "terminal_ladder_refresh_open",
    "page_local_route_body_join_open",
    "product_activation_selected",
    "product_activation",
    "hook_install_selected",
    "hook_install",
    "page_local_alloc_route_cfg_lowering_enabled",
    "page_local_free_route_cfg_lowering_enabled",
)

GLOBAL_ALLOCATOR_CLAIM_PREFLIGHT_REFRESH_EXPECTED_ZERO = (
    "page_reclaimed_with_remote_candidates",
    "allocator_owner_reuse_without_generation_bump_count",
    "type_abi_hot_lookup_count",
    "provider_abi_hot_dispatch_count",
    "global_allocator_claim",
    "winner_claim",
)

GLOBAL_ALLOCATOR_CLAIM_PREFLIGHT_REFRESH_EXPECTED_POSITIVE = (
    "fastmem_global_allocator_claim_preflight_refresh",
    "terminal_ladder_refresh_open",
    "page_local_route_body_join_open",
    "product_activation_selected",
    "product_activation",
    "hook_install_selected",
    "hook_install",
    "global_allocator_claim_selected",
    "page_local_alloc_route_cfg_lowering_enabled",
    "page_local_free_route_cfg_lowering_enabled",
)

GLOBAL_ALLOCATOR_CLAIM_PRODUCER_REFRESH_EXPECTED_ZERO = (
    "page_reclaimed_with_remote_candidates",
    "allocator_owner_reuse_without_generation_bump_count",
    "type_abi_hot_lookup_count",
    "provider_abi_hot_dispatch_count",
    "winner_claim",
)

GLOBAL_ALLOCATOR_CLAIM_PRODUCER_REFRESH_EXPECTED_POSITIVE = (
    "fastmem_global_allocator_claim_producer_refresh",
    "terminal_ladder_refresh_open",
    "page_local_route_body_join_open",
    "product_activation_selected",
    "product_activation",
    "hook_install_selected",
    "hook_install",
    "global_allocator_claim_selected",
    "global_allocator_claim",
    "page_local_alloc_route_cfg_lowering_enabled",
    "page_local_free_route_cfg_lowering_enabled",
)

WINNER_CLAIM_PREFLIGHT_REFRESH_EXPECTED_ZERO = (
    "page_reclaimed_with_remote_candidates",
    "allocator_owner_reuse_without_generation_bump_count",
    "type_abi_hot_lookup_count",
    "provider_abi_hot_dispatch_count",
    "winner_claim",
)

WINNER_CLAIM_PREFLIGHT_REFRESH_EXPECTED_POSITIVE = (
    "fastmem_winner_claim_preflight_refresh",
    "terminal_ladder_refresh_open",
    "page_local_route_body_join_open",
    "product_activation_selected",
    "product_activation",
    "hook_install_selected",
    "hook_install",
    "global_allocator_claim_selected",
    "global_allocator_claim",
    "winner_claim_selected",
    "page_local_alloc_route_cfg_lowering_enabled",
    "page_local_free_route_cfg_lowering_enabled",
)

WINNER_CLAIM_PRODUCER_REFRESH_EXPECTED_ZERO = (
    "page_reclaimed_with_remote_candidates",
    "allocator_owner_reuse_without_generation_bump_count",
    "type_abi_hot_lookup_count",
    "provider_abi_hot_dispatch_count",
)

WINNER_CLAIM_PRODUCER_REFRESH_EXPECTED_POSITIVE = (
    "fastmem_winner_claim_producer_refresh",
    "terminal_ladder_refresh_open",
    "page_local_route_body_join_open",
    "product_activation_selected",
    "product_activation",
    "hook_install_selected",
    "hook_install",
    "global_allocator_claim_selected",
    "global_allocator_claim",
    "winner_claim_selected",
    "winner_claim",
    "page_local_alloc_route_cfg_lowering_enabled",
    "page_local_free_route_cfg_lowering_enabled",
)


REFRESH_PROFILE_SPECS: tuple[RefreshProfileSpec, ...] = (
    RefreshProfileSpec(
        profile="terminal-ladder-refresh-preflight",
        report_flag="fastmem_terminal_ladder_refresh_preflight",
        selected_route="terminal_ladder_refresh_preflight",
        family="terminal_ladder_refresh",
        memop_kinds="TerminalLadderRefresh",
        next_slice="tls_backing_transfer_preflight_refresh",
        deferred_kinds="TlsBackingTransfer",
        expected_zero=TERMINAL_LADDER_REFRESH_PREFLIGHT_EXPECTED_ZERO,
        expected_positive=TERMINAL_LADDER_REFRESH_PREFLIGHT_EXPECTED_POSITIVE,
    ),
    RefreshProfileSpec(
        profile="tls-backing-transfer-preflight-refresh",
        report_flag="fastmem_tls_backing_transfer_preflight_refresh",
        selected_route="tls_backing_transfer_preflight_refresh",
        family="tls_backing_transfer",
        memop_kinds="TlsBackingTransfer",
        next_slice="tls_backing_transfer_producer_refresh",
        deferred_kinds="TlsBackingTransferProducer,OwnerSlotReuse",
        expected_zero=TLS_BACKING_TRANSFER_PREFLIGHT_REFRESH_EXPECTED_ZERO,
        expected_positive=TLS_BACKING_TRANSFER_PREFLIGHT_REFRESH_EXPECTED_POSITIVE,
    ),
    RefreshProfileSpec(
        profile="tls-backing-transfer-producer-refresh",
        report_flag="fastmem_tls_backing_transfer_producer_refresh",
        selected_route="tls_backing_transfer_producer_refresh",
        family="tls_backing_transfer",
        memop_kinds="TlsBackingTransfer",
        next_slice="owner_slot_reuse_preflight_refresh",
        deferred_kinds="OwnerSlotReuse",
        expected_zero=TLS_BACKING_TRANSFER_PRODUCER_REFRESH_EXPECTED_ZERO,
        expected_positive=TLS_BACKING_TRANSFER_PRODUCER_REFRESH_EXPECTED_POSITIVE,
    ),
    RefreshProfileSpec(
        profile="owner-slot-reuse-preflight-refresh",
        report_flag="fastmem_allocator_owner_slot_reuse_preflight_refresh",
        selected_route="owner_slot_reuse_preflight_refresh",
        family="owner_slot_reuse",
        memop_kinds="OwnerSlotReuse",
        next_slice="owner_slot_reuse_producer_refresh",
        deferred_kinds="OwnerSlotReuseProducer,AbandonedReclaim",
        expected_zero=OWNER_SLOT_REUSE_PREFLIGHT_REFRESH_EXPECTED_ZERO,
        expected_positive=OWNER_SLOT_REUSE_PREFLIGHT_REFRESH_EXPECTED_POSITIVE,
    ),
    RefreshProfileSpec(
        profile="owner-slot-reuse-producer-refresh",
        report_flag="fastmem_allocator_owner_slot_reuse_producer_refresh",
        selected_route="owner_slot_reuse_producer_refresh",
        family="owner_slot_reuse",
        memop_kinds="OwnerSlotReuse",
        next_slice="abandoned_reclaim_preflight_refresh",
        deferred_kinds="AbandonedReclaim",
        expected_zero=OWNER_SLOT_REUSE_PRODUCER_REFRESH_EXPECTED_ZERO,
        expected_positive=OWNER_SLOT_REUSE_PRODUCER_REFRESH_EXPECTED_POSITIVE,
    ),
    RefreshProfileSpec(
        profile="abandoned-reclaim-preflight-refresh",
        report_flag="fastmem_abandoned_reclaim_preflight_refresh",
        selected_route="abandoned_reclaim_preflight_refresh",
        family="abandoned_reclaim",
        memop_kinds="AbandonedReclaim",
        next_slice="abandoned_reclaim_producer_refresh",
        deferred_kinds="AbandonedReclaimProducer,ProductActivation",
        expected_zero=ABANDONED_RECLAIM_PREFLIGHT_REFRESH_EXPECTED_ZERO,
        expected_positive=ABANDONED_RECLAIM_PREFLIGHT_REFRESH_EXPECTED_POSITIVE,
    ),
    RefreshProfileSpec(
        profile="abandoned-reclaim-producer-refresh",
        report_flag="fastmem_abandoned_reclaim_producer_refresh",
        selected_route="abandoned_reclaim_producer_refresh",
        family="abandoned_reclaim",
        memop_kinds="AbandonedReclaim",
        next_slice="product_activation_preflight_refresh",
        deferred_kinds="ProductActivation",
        expected_zero=ABANDONED_RECLAIM_PRODUCER_REFRESH_EXPECTED_ZERO,
        expected_positive=ABANDONED_RECLAIM_PRODUCER_REFRESH_EXPECTED_POSITIVE,
    ),
    RefreshProfileSpec(
        profile="product-activation-preflight-refresh",
        report_flag="fastmem_product_activation_preflight_refresh",
        selected_route="product_activation_preflight_refresh",
        family="product_activation",
        memop_kinds="ProductActivation",
        next_slice="product_activation_producer_refresh",
        deferred_kinds="ProductActivationProducer,HookInstall",
        expected_zero=PRODUCT_ACTIVATION_PREFLIGHT_REFRESH_EXPECTED_ZERO,
        expected_positive=PRODUCT_ACTIVATION_PREFLIGHT_REFRESH_EXPECTED_POSITIVE,
    ),
    RefreshProfileSpec(
        profile="product-activation-producer-refresh",
        report_flag="fastmem_product_activation_producer_refresh",
        selected_route="product_activation_producer_refresh",
        family="product_activation",
        memop_kinds="ProductActivation",
        next_slice="hook_install_preflight_refresh",
        deferred_kinds="HookInstall",
        expected_zero=PRODUCT_ACTIVATION_PRODUCER_REFRESH_EXPECTED_ZERO,
        expected_positive=PRODUCT_ACTIVATION_PRODUCER_REFRESH_EXPECTED_POSITIVE,
    ),
    RefreshProfileSpec(
        profile="hook-install-preflight-refresh",
        report_flag="fastmem_hook_install_preflight_refresh",
        selected_route="hook_install_preflight_refresh",
        family="hook_install",
        memop_kinds="HookInstall",
        next_slice="hook_install_producer_refresh",
        deferred_kinds="HookInstallProducer,GlobalAllocatorClaim",
        expected_zero=HOOK_INSTALL_PREFLIGHT_REFRESH_EXPECTED_ZERO,
        expected_positive=HOOK_INSTALL_PREFLIGHT_REFRESH_EXPECTED_POSITIVE,
    ),
    RefreshProfileSpec(
        profile="hook-install-producer-refresh",
        report_flag="fastmem_hook_install_producer_refresh",
        selected_route="hook_install_producer_refresh",
        family="hook_install",
        memop_kinds="HookInstall",
        next_slice="global_allocator_claim_preflight_refresh",
        deferred_kinds="GlobalAllocatorClaim",
        expected_zero=HOOK_INSTALL_PRODUCER_REFRESH_EXPECTED_ZERO,
        expected_positive=HOOK_INSTALL_PRODUCER_REFRESH_EXPECTED_POSITIVE,
    ),
    RefreshProfileSpec(
        profile="global-allocator-claim-preflight-refresh",
        report_flag="fastmem_global_allocator_claim_preflight_refresh",
        selected_route="global_allocator_claim_preflight_refresh",
        family="global_allocator_claim",
        memop_kinds="GlobalAllocatorClaim",
        next_slice="global_allocator_claim_producer_refresh",
        deferred_kinds="GlobalAllocatorClaimProducer,WinnerClaim",
        expected_zero=GLOBAL_ALLOCATOR_CLAIM_PREFLIGHT_REFRESH_EXPECTED_ZERO,
        expected_positive=GLOBAL_ALLOCATOR_CLAIM_PREFLIGHT_REFRESH_EXPECTED_POSITIVE,
    ),
    RefreshProfileSpec(
        profile="global-allocator-claim-producer-refresh",
        report_flag="fastmem_global_allocator_claim_producer_refresh",
        selected_route="global_allocator_claim_producer_refresh",
        family="global_allocator_claim",
        memop_kinds="GlobalAllocatorClaim",
        next_slice="winner_claim_preflight_refresh",
        deferred_kinds="WinnerClaim",
        expected_zero=GLOBAL_ALLOCATOR_CLAIM_PRODUCER_REFRESH_EXPECTED_ZERO,
        expected_positive=GLOBAL_ALLOCATOR_CLAIM_PRODUCER_REFRESH_EXPECTED_POSITIVE,
    ),
    RefreshProfileSpec(
        profile="winner-claim-preflight-refresh",
        report_flag="fastmem_winner_claim_preflight_refresh",
        selected_route="winner_claim_preflight_refresh",
        family="winner_claim",
        memop_kinds="WinnerClaim",
        next_slice="winner_claim_producer_refresh",
        deferred_kinds="WinnerClaimProducer",
        expected_zero=WINNER_CLAIM_PREFLIGHT_REFRESH_EXPECTED_ZERO,
        expected_positive=WINNER_CLAIM_PREFLIGHT_REFRESH_EXPECTED_POSITIVE,
    ),
    RefreshProfileSpec(
        profile="winner-claim-producer-refresh",
        report_flag="fastmem_winner_claim_producer_refresh",
        selected_route="winner_claim_producer_refresh",
        family="winner_claim",
        memop_kinds="WinnerClaim",
        next_slice="complete",
        deferred_kinds="none",
        expected_zero=WINNER_CLAIM_PRODUCER_REFRESH_EXPECTED_ZERO,
        expected_positive=WINNER_CLAIM_PRODUCER_REFRESH_EXPECTED_POSITIVE,
    ),
)

REFRESH_PROFILE_BY_NAME = {spec.profile: spec for spec in REFRESH_PROFILE_SPECS}
REFRESH_PROFILE_BY_FLAG = {spec.report_flag: spec for spec in REFRESH_PROFILE_SPECS}
REFRESH_PROFILE_NAMES = tuple(spec.profile for spec in REFRESH_PROFILE_SPECS)


def refresh_profile_spec(profile: str) -> RefreshProfileSpec | None:
    return REFRESH_PROFILE_BY_NAME.get(profile)


def refresh_profile_spec_for_rows(
    rows: Mapping[str, str],
) -> RefreshProfileSpec | None:
    for spec in REFRESH_PROFILE_SPECS:
        if _profile_flag(rows, spec.report_flag):
            return spec
    return None
