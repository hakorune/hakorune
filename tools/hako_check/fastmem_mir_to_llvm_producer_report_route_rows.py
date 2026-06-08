"""FastMemory MIR-to-LLVM producer report route rows."""

from __future__ import annotations

from dataclasses import dataclass
from typing import Any, Mapping

from fastmem_mir_to_llvm_producer_report_common import (
    int_flag,
    page_local_alloc_route_candidate,
    page_local_free_route_candidate,
)
from fastmem_route_profiles import (
    abandoned_reclaim_preflight_profile,
    abandoned_reclaim_producer_profile,
    fastmem_branch_cfg_lowering_preflight_profile,
    fastmem_branch_cfg_lowering_profile,
    fastmem_branch_cfg_preflight_profile,
    global_allocator_claim_preflight_profile,
    global_allocator_claim_producer_profile,
    hook_install_preflight_profile,
    hook_install_producer_profile,
    owner_slot_reuse_preflight_profile,
    owner_slot_reuse_producer_profile,
    page_local_alloc_route_cfg_preflight_profile,
    page_local_free_route_cfg_preflight_profile,
    page_local_free_route_cfg_producer_profile,
    product_activation_preflight_profile,
    product_activation_producer_profile,
    remote_owner_branch_route_body_preflight_profile,
    remote_owner_branch_routing_lowering_preflight_profile,
    remote_owner_branch_routing_lowering_profile,
    remote_owner_branch_routing_preflight_profile,
    same_remote_free_body_preflight_profile,
    same_remote_free_body_producer_profile,
    tls_backing_transfer_preflight_profile,
    tls_backing_transfer_producer_profile,
    winner_claim_preflight_profile,
    winner_claim_producer_profile,
    REFRESH_PROFILE_SPECS,
    refresh_profile_spec,
    remote_free_route_profile_spec,
)


@dataclass(frozen=True)
class RouteSummaryFlags:
    winner_claim_any: bool
    global_allocator_claim_any: bool
    hook_install_any: bool
    product_activation_any: bool
    abandoned_reclaim_any: bool
    owner_slot_reuse_any: bool
    tls_backing_transfer_preflight_refresh: bool
    tls_backing_transfer_producer_refresh: bool
    tls_backing_transfer_preflight: bool
    tls_backing_transfer_producer: bool
    terminal_ladder_refresh_preflight: bool
    page_local_route_body_join_any: bool
    page_local_route_body_join_preflight: bool
    page_local_route_body_join_producer: bool
    page_local_free_route_cfg_any: bool
    page_local_free_route_cfg_preflight: bool
    page_local_alloc_route_cfg_any: bool
    page_local_alloc_route_cfg_preflight: bool
    page_local_alloc_route_cfg_producer: bool
    same_remote_free_body_preflight: bool
    same_remote_free_body_producer: bool
    fastmem_branch_cfg_lowering_preflight: bool
    fastmem_branch_cfg_lowering_producer: bool
    remote_owner_branch_routing_any: bool


@dataclass(frozen=True)
class ActivationProgressionFlags:
    tls_backing_transfer_preflight_refresh: bool
    tls_backing_transfer_producer_refresh: bool
    tls_backing_transfer_preflight: bool
    tls_backing_transfer_producer: bool
    tls_backing_transfer_any: bool
    tls_backing_transfer_or_later: bool
    owner_slot_reuse_preflight: bool
    owner_slot_reuse_preflight_refresh: bool
    owner_slot_reuse_producer_refresh: bool
    owner_slot_reuse_producer: bool
    owner_slot_reuse_any: bool
    owner_slot_reuse_or_later: bool
    abandoned_reclaim_preflight_refresh: bool
    abandoned_reclaim_producer_refresh: bool
    abandoned_reclaim_preflight: bool
    abandoned_reclaim_producer: bool
    abandoned_reclaim_any: bool
    abandoned_reclaim_or_later: bool
    product_activation_preflight_refresh: bool
    product_activation_producer_refresh: bool
    product_activation_preflight: bool
    product_activation_producer: bool
    product_activation_any: bool
    product_activation_or_later: bool
    hook_install_preflight_refresh: bool
    hook_install_producer_refresh: bool
    hook_install_preflight: bool
    hook_install_producer: bool
    hook_install_any: bool
    hook_install_or_later: bool
    global_allocator_claim_preflight_refresh: bool
    global_allocator_claim_producer_refresh: bool
    global_allocator_claim_preflight: bool
    global_allocator_claim_producer: bool
    global_allocator_claim_any: bool
    global_allocator_claim_or_later: bool
    winner_claim_preflight_refresh: bool
    winner_claim_producer_refresh: bool
    winner_claim_preflight: bool
    winner_claim_producer: bool
    winner_claim_any: bool
    page_local_route_body_join_preflight: bool
    page_local_route_body_join_producer: bool
    terminal_ladder_refresh_preflight: bool
    page_local_route_body_join_any: bool
    page_local_alloc_route_cfg_preflight: bool
    page_local_alloc_route_cfg_producer: bool
    page_local_alloc_route_cfg_any: bool
    page_local_free_route_cfg_preflight: bool
    page_local_free_route_cfg_producer: bool
    page_local_free_route_cfg_any: bool


@dataclass(frozen=True)
class RemoteFreeSliceContext:
    selected_route: str
    next_slice: str
    selected_memop_family: str
    selected_memop_kinds: str
    deferred_remote_kinds: str
    remote_free_open: bool
    refresh_flag_rows: list[tuple[str, str]]
    flag_scope: dict[str, bool]


def _activation_progression_flags(profile: str, refresh_route: str) -> ActivationProgressionFlags:
    tls_backing_transfer_preflight_refresh = (
        refresh_route == "tls_backing_transfer_preflight_refresh"
    )
    tls_backing_transfer_producer_refresh = (
        refresh_route == "tls_backing_transfer_producer_refresh"
    )
    page_local_route_body_join_preflight = (
        profile == "page-local-route-body-join-preflight"
    )
    page_local_route_body_join_producer = profile == "page-local-route-body-join"
    terminal_ladder_refresh_preflight = (
        refresh_route == "terminal_ladder_refresh_preflight"
    )
    page_local_route_body_join_any = _any_true(
        page_local_route_body_join_preflight,
        page_local_route_body_join_producer,
        terminal_ladder_refresh_preflight,
        tls_backing_transfer_preflight_refresh,
        tls_backing_transfer_producer_refresh,
    )
    page_local_alloc_route_cfg_preflight = (
        profile == "page-local-alloc-route-cfg-preflight"
    )
    page_local_alloc_route_cfg_producer = profile == "page-local-alloc-route-cfg"
    page_local_alloc_route_cfg_any = _any_true(
        page_local_alloc_route_cfg_preflight,
        page_local_alloc_route_cfg_producer,
        page_local_route_body_join_any,
    )
    page_local_free_route_cfg_preflight = (
        profile == "page-local-free-route-cfg-preflight"
    )
    page_local_free_route_cfg_producer = profile == "page-local-free-route-cfg"
    page_local_free_route_cfg_any = _any_true(
        page_local_free_route_cfg_preflight,
        page_local_free_route_cfg_producer,
        page_local_route_body_join_any,
    )
    tls_backing_transfer_preflight = profile == "tls-backing-transfer-preflight"
    tls_backing_transfer_producer = profile == "tls-backing-transfer-producer-pilot"
    tls_backing_transfer_any = _any_true(
        tls_backing_transfer_preflight,
        tls_backing_transfer_preflight_refresh,
        tls_backing_transfer_producer_refresh,
        tls_backing_transfer_producer,
    )
    owner_slot_reuse_preflight = profile == "owner-slot-reuse-preflight"
    owner_slot_reuse_preflight_refresh = (
        refresh_route == "owner_slot_reuse_preflight_refresh"
    )
    owner_slot_reuse_producer_refresh = (
        refresh_route == "owner_slot_reuse_producer_refresh"
    )
    owner_slot_reuse_producer = profile == "owner-slot-reuse-producer-pilot"
    owner_slot_reuse_any = _any_true(
        owner_slot_reuse_preflight,
        owner_slot_reuse_preflight_refresh,
        owner_slot_reuse_producer_refresh,
        owner_slot_reuse_producer,
    )
    abandoned_reclaim_preflight_refresh = (
        refresh_route == "abandoned_reclaim_preflight_refresh"
    )
    abandoned_reclaim_producer_refresh = (
        refresh_route == "abandoned_reclaim_producer_refresh"
    )
    abandoned_reclaim_preflight = profile == "abandoned-reclaim-preflight"
    abandoned_reclaim_producer = profile == "abandoned-reclaim-producer-pilot"
    abandoned_reclaim_any = _any_true(
        abandoned_reclaim_preflight_refresh,
        abandoned_reclaim_producer_refresh,
        abandoned_reclaim_preflight,
        abandoned_reclaim_producer,
    )
    product_activation_preflight_refresh = (
        refresh_route == "product_activation_preflight_refresh"
    )
    product_activation_producer_refresh = (
        refresh_route == "product_activation_producer_refresh"
    )
    product_activation_preflight = profile == "product-activation-preflight"
    product_activation_producer = profile == "product-activation-producer-pilot"
    product_activation_any = _any_true(
        product_activation_preflight_refresh,
        product_activation_producer_refresh,
        product_activation_preflight,
        product_activation_producer,
    )
    hook_install_preflight_refresh = refresh_route == "hook_install_preflight_refresh"
    hook_install_producer_refresh = refresh_route == "hook_install_producer_refresh"
    hook_install_preflight = profile == "hook-install-preflight"
    hook_install_producer = profile == "hook-install-producer-pilot"
    hook_install_any = _any_true(
        hook_install_preflight_refresh,
        hook_install_producer_refresh,
        hook_install_preflight,
        hook_install_producer,
    )
    global_allocator_claim_preflight_refresh = (
        refresh_route == "global_allocator_claim_preflight_refresh"
    )
    global_allocator_claim_producer_refresh = (
        refresh_route == "global_allocator_claim_producer_refresh"
    )
    global_allocator_claim_preflight = profile == "global-allocator-claim-preflight"
    global_allocator_claim_producer = profile == "global-allocator-claim-producer-pilot"
    global_allocator_claim_any = _any_true(
        global_allocator_claim_preflight_refresh,
        global_allocator_claim_producer_refresh,
        global_allocator_claim_preflight,
        global_allocator_claim_producer,
    )
    winner_claim_preflight_refresh = refresh_route == "winner_claim_preflight_refresh"
    winner_claim_producer_refresh = refresh_route == "winner_claim_producer_refresh"
    winner_claim_preflight = profile == "winner-claim-preflight"
    winner_claim_producer = profile == "winner-claim-producer-pilot"
    winner_claim_any = _any_true(
        winner_claim_preflight_refresh,
        winner_claim_producer_refresh,
        winner_claim_preflight,
        winner_claim_producer,
    )
    global_allocator_claim_or_later = global_allocator_claim_any or winner_claim_any
    hook_install_or_later = hook_install_any or global_allocator_claim_or_later
    product_activation_or_later = product_activation_any or hook_install_or_later
    abandoned_reclaim_or_later = abandoned_reclaim_any or product_activation_or_later
    owner_slot_reuse_or_later = owner_slot_reuse_any or abandoned_reclaim_or_later
    tls_backing_transfer_or_later = (
        tls_backing_transfer_any or owner_slot_reuse_or_later
    )
    return ActivationProgressionFlags(
        tls_backing_transfer_preflight_refresh=tls_backing_transfer_preflight_refresh,
        tls_backing_transfer_producer_refresh=tls_backing_transfer_producer_refresh,
        tls_backing_transfer_preflight=tls_backing_transfer_preflight,
        tls_backing_transfer_producer=tls_backing_transfer_producer,
        tls_backing_transfer_any=tls_backing_transfer_any,
        tls_backing_transfer_or_later=tls_backing_transfer_or_later,
        owner_slot_reuse_preflight=owner_slot_reuse_preflight,
        owner_slot_reuse_preflight_refresh=owner_slot_reuse_preflight_refresh,
        owner_slot_reuse_producer_refresh=owner_slot_reuse_producer_refresh,
        owner_slot_reuse_producer=owner_slot_reuse_producer,
        owner_slot_reuse_any=owner_slot_reuse_any,
        owner_slot_reuse_or_later=owner_slot_reuse_or_later,
        abandoned_reclaim_preflight_refresh=abandoned_reclaim_preflight_refresh,
        abandoned_reclaim_producer_refresh=abandoned_reclaim_producer_refresh,
        abandoned_reclaim_preflight=abandoned_reclaim_preflight,
        abandoned_reclaim_producer=abandoned_reclaim_producer,
        abandoned_reclaim_any=abandoned_reclaim_any,
        abandoned_reclaim_or_later=abandoned_reclaim_or_later,
        product_activation_preflight_refresh=product_activation_preflight_refresh,
        product_activation_producer_refresh=product_activation_producer_refresh,
        product_activation_preflight=product_activation_preflight,
        product_activation_producer=product_activation_producer,
        product_activation_any=product_activation_any,
        product_activation_or_later=product_activation_or_later,
        hook_install_preflight_refresh=hook_install_preflight_refresh,
        hook_install_producer_refresh=hook_install_producer_refresh,
        hook_install_preflight=hook_install_preflight,
        hook_install_producer=hook_install_producer,
        hook_install_any=hook_install_any,
        hook_install_or_later=hook_install_or_later,
        global_allocator_claim_preflight_refresh=global_allocator_claim_preflight_refresh,
        global_allocator_claim_producer_refresh=global_allocator_claim_producer_refresh,
        global_allocator_claim_preflight=global_allocator_claim_preflight,
        global_allocator_claim_producer=global_allocator_claim_producer,
        global_allocator_claim_any=global_allocator_claim_any,
        global_allocator_claim_or_later=global_allocator_claim_or_later,
        winner_claim_preflight_refresh=winner_claim_preflight_refresh,
        winner_claim_producer_refresh=winner_claim_producer_refresh,
        winner_claim_preflight=winner_claim_preflight,
        winner_claim_producer=winner_claim_producer,
        winner_claim_any=winner_claim_any,
        page_local_route_body_join_preflight=page_local_route_body_join_preflight,
        page_local_route_body_join_producer=page_local_route_body_join_producer,
        terminal_ladder_refresh_preflight=terminal_ladder_refresh_preflight,
        page_local_route_body_join_any=page_local_route_body_join_any,
        page_local_alloc_route_cfg_preflight=page_local_alloc_route_cfg_preflight,
        page_local_alloc_route_cfg_producer=page_local_alloc_route_cfg_producer,
        page_local_alloc_route_cfg_any=page_local_alloc_route_cfg_any,
        page_local_free_route_cfg_preflight=page_local_free_route_cfg_preflight,
        page_local_free_route_cfg_producer=page_local_free_route_cfg_producer,
        page_local_free_route_cfg_any=page_local_free_route_cfg_any,
    )


REMOTE_FREE_ATOMIC_FLAG_NAMES: tuple[str, ...] = (
    "fastmem_atomic_remote_head_cas_preflight",
    "fastmem_atomic_remote_head_cas_producer_pilot",
    "fastmem_atomic_remote_head_retry_preflight",
    "fastmem_atomic_remote_head_retry_producer_pilot",
    "fastmem_atomic_remote_head_drain_preflight",
    "fastmem_atomic_remote_head_drain_exchange_selection",
    "fastmem_atomic_remote_head_drain_exchange_producer_pilot",
    "fastmem_atomic_remote_head_drain_to_local_route_selection",
    "fastmem_atomic_remote_head_drain_to_local_route_producer_pilot",
    "fastmem_atomic_remote_head_drain_local_list_mutation_preflight",
    "fastmem_atomic_remote_head_drain_local_list_mutation_proof",
    "fastmem_atomic_remote_head_drain_local_list_mutation_vocabulary_preflight",
    "fastmem_atomic_remote_head_drain_local_list_mutation_verifier_preconditions",
    "fastmem_atomic_remote_head_drain_local_list_mutation_lowering_producer_pilot",
)

REMOTE_FREE_ROUTE_FAMILY_FLAG_NAMES: tuple[str, ...] = (
    "fastmem_remote_owner_branch_routing_preflight",
    "fastmem_remote_owner_branch_routing_lowering_preflight",
    "fastmem_remote_owner_branch_routing_lowering_producer_pilot",
    "fastmem_remote_owner_branch_route_body_preflight",
    "fastmem_branch_cfg_preflight",
    "fastmem_branch_cfg_lowering_preflight",
    "fastmem_branch_cfg_lowering_producer_pilot",
    "fastmem_same_remote_free_body_preflight",
    "fastmem_same_remote_free_body_producer_pilot",
    "fastmem_page_local_free_route_cfg_preflight",
    "fastmem_page_local_alloc_route_cfg_preflight",
    "fastmem_page_local_alloc_route_cfg_producer_pilot",
    "fastmem_page_local_free_route_cfg_producer_pilot",
    "fastmem_page_local_route_body_join_preflight",
    "fastmem_page_local_route_body_join_producer_pilot",
)

REMOTE_FREE_REFRESH_FLAG_NAMES: tuple[str, ...] = (
    "fastmem_tls_backing_transfer_preflight",
    "fastmem_tls_backing_transfer_producer_pilot",
    "fastmem_allocator_owner_slot_reuse_preflight",
    "fastmem_allocator_owner_slot_reuse_producer_pilot",
    "fastmem_abandoned_reclaim_preflight",
    "fastmem_abandoned_reclaim_producer_pilot",
    "fastmem_product_activation_preflight",
    "fastmem_product_activation_producer_pilot",
    "fastmem_hook_install_preflight",
    "fastmem_hook_install_producer_pilot",
    "fastmem_global_allocator_claim_preflight",
    "fastmem_global_allocator_claim_producer_pilot",
    "fastmem_winner_claim_preflight",
    "fastmem_winner_claim_producer_pilot",
)

REMOTE_FREE_CAS_BLOCKER_NAMES: tuple[str, ...] = (
    "fastmem_atomic_remote_head_retry_preflight",
    "fastmem_atomic_remote_head_drain_preflight",
    "fastmem_atomic_remote_head_drain_exchange_selection",
    "fastmem_atomic_remote_head_drain_exchange_producer_pilot",
    "fastmem_atomic_remote_head_drain_to_local_route_selection",
    "fastmem_atomic_remote_head_drain_to_local_route_producer_pilot",
    "fastmem_atomic_remote_head_drain_local_list_mutation_preflight",
    "fastmem_atomic_remote_head_drain_local_list_mutation_proof",
    "fastmem_atomic_remote_head_drain_local_list_mutation_vocabulary_preflight",
    "fastmem_atomic_remote_head_drain_local_list_mutation_verifier_preconditions",
    "fastmem_atomic_remote_head_drain_local_list_mutation_lowering_producer_pilot",
    "fastmem_remote_owner_branch_routing_preflight",
    "fastmem_remote_owner_branch_routing_lowering_preflight",
    "fastmem_remote_owner_branch_routing_lowering_producer_pilot",
    "fastmem_remote_owner_branch_route_body_preflight",
    "fastmem_branch_cfg_preflight",
    "fastmem_branch_cfg_lowering_preflight",
    "fastmem_branch_cfg_lowering_producer_pilot",
    "fastmem_same_remote_free_body_preflight",
    "fastmem_same_remote_free_body_producer_pilot",
    "fastmem_page_local_free_route_cfg_preflight",
    "page_local_free_route_cfg_any",
    "fastmem_tls_backing_transfer_preflight",
    "fastmem_tls_backing_transfer_producer_pilot",
    "tls_backing_transfer_or_later",
)


def _flag_rows(*pairs: tuple[str, bool]) -> list[tuple[str, str]]:
    return [(name, str(int_flag(flag))) for name, flag in pairs]


def _any_true(*flags: bool) -> bool:
    return any(flags)


def _slice_prefix_rows(
    *,
    selection_v0: str,
    next_slice: str,
    selected_memop_family: str,
    selected_memop_kinds: str,
    deferred_memop_family: str,
    deferred_memop_kinds: str,
    owner_runtime_pilot: bool,
    local_free_pilot: bool,
    layout_table_pilot: bool,
    selected_route: str | None = None,
) -> list[tuple[str, str]]:
    rows: list[tuple[str, str]] = [
        ("replacement_front_producer_slice_selection_v0", selection_v0),
    ]
    if selected_route is not None:
        rows.append(("replacement_front_selected_route", selected_route))
    rows.extend(
        [
            ("replacement_front_next_producer_slice", next_slice),
            ("replacement_front_selected_memop_family", selected_memop_family),
            ("replacement_front_selected_memop_kinds", selected_memop_kinds),
            ("replacement_front_deferred_memop_family", deferred_memop_family),
            ("replacement_front_deferred_memop_kinds", deferred_memop_kinds),
            ("mir_fmem_008b_layout_table_producer_pilot", str(int_flag(layout_table_pilot))),
            ("fastmem_owner_runtime_producer_pilot", str(int_flag(owner_runtime_pilot))),
            ("fastmem_local_free_producer_pilot", str(int_flag(local_free_pilot))),
        ]
    )
    return rows


def _remote_free_flag_scope(
    *,
    remote_free_open: bool,
    page_local_alloc_route_cfg_any: bool,
    remote_free_retry_preflight: bool,
    remote_free_retry_producer: bool,
    remote_free_drain_preflight: bool,
    remote_free_drain_exchange_selection: bool,
    remote_free_drain_exchange_producer: bool,
    remote_free_drain_to_local_selection: bool,
    remote_free_drain_to_local_producer: bool,
    remote_free_drain_local_list_mutation_preflight: bool,
    remote_free_drain_local_list_mutation_proof: bool,
    remote_free_drain_local_list_mutation_vocabulary_preflight: bool,
    remote_free_drain_local_list_mutation_verifier_preconditions: bool,
    remote_free_drain_local_list_mutation_lowering_producer: bool,
    remote_owner_branch_routing_preflight: bool,
    remote_owner_branch_routing_lowering_preflight: bool,
    remote_owner_branch_routing_lowering_producer: bool,
    remote_owner_branch_route_body_preflight: bool,
    fastmem_branch_cfg_preflight: bool,
    fastmem_branch_cfg_lowering_preflight: bool,
    fastmem_branch_cfg_lowering_producer: bool,
    same_remote_free_body_preflight: bool,
    same_remote_free_body_producer: bool,
    page_local_free_route_cfg_preflight: bool,
    page_local_free_route_cfg_any: bool,
    page_local_alloc_route_cfg_preflight: bool,
    page_local_alloc_route_cfg_producer: bool,
    page_local_free_route_cfg_producer: bool,
    page_local_route_body_join_preflight: bool,
    page_local_route_body_join_producer: bool,
    tls_backing_transfer_preflight: bool,
    tls_backing_transfer_producer: bool,
    tls_backing_transfer_or_later: bool,
    owner_slot_reuse_preflight: bool,
    owner_slot_reuse_producer: bool,
    abandoned_reclaim_preflight: bool,
    abandoned_reclaim_producer: bool,
    product_activation_preflight: bool,
    product_activation_producer: bool,
    hook_install_preflight: bool,
    hook_install_producer: bool,
    global_allocator_claim_preflight: bool,
    global_allocator_claim_producer: bool,
    winner_claim_preflight: bool,
    winner_claim_producer: bool,
) -> dict[str, bool]:
    return {
        "fastmem_atomic_remote_head_cas_preflight": not remote_free_open
        and not page_local_alloc_route_cfg_any,
        "fastmem_atomic_remote_head_cas_producer_pilot": False,
        "fastmem_atomic_remote_head_retry_preflight": remote_free_retry_preflight,
        "fastmem_atomic_remote_head_retry_producer_pilot": remote_free_retry_producer,
        "fastmem_atomic_remote_head_drain_preflight": remote_free_drain_preflight,
        "fastmem_atomic_remote_head_drain_exchange_selection": remote_free_drain_exchange_selection,
        "fastmem_atomic_remote_head_drain_exchange_producer_pilot": remote_free_drain_exchange_producer,
        "fastmem_atomic_remote_head_drain_to_local_route_selection": remote_free_drain_to_local_selection,
        "fastmem_atomic_remote_head_drain_to_local_route_producer_pilot": remote_free_drain_to_local_producer,
        "fastmem_atomic_remote_head_drain_local_list_mutation_preflight": remote_free_drain_local_list_mutation_preflight,
        "fastmem_atomic_remote_head_drain_local_list_mutation_proof": remote_free_drain_local_list_mutation_proof,
        "fastmem_atomic_remote_head_drain_local_list_mutation_vocabulary_preflight": remote_free_drain_local_list_mutation_vocabulary_preflight,
        "fastmem_atomic_remote_head_drain_local_list_mutation_verifier_preconditions": remote_free_drain_local_list_mutation_verifier_preconditions,
        "fastmem_atomic_remote_head_drain_local_list_mutation_lowering_producer_pilot": remote_free_drain_local_list_mutation_lowering_producer,
        "fastmem_remote_owner_branch_routing_preflight": remote_owner_branch_routing_preflight,
        "fastmem_remote_owner_branch_routing_lowering_preflight": remote_owner_branch_routing_lowering_preflight,
        "fastmem_remote_owner_branch_routing_lowering_producer_pilot": remote_owner_branch_routing_lowering_producer,
        "fastmem_remote_owner_branch_route_body_preflight": remote_owner_branch_route_body_preflight,
        "fastmem_branch_cfg_preflight": fastmem_branch_cfg_preflight,
        "fastmem_branch_cfg_lowering_preflight": fastmem_branch_cfg_lowering_preflight,
        "fastmem_branch_cfg_lowering_producer_pilot": fastmem_branch_cfg_lowering_producer,
        "fastmem_same_remote_free_body_preflight": same_remote_free_body_preflight,
        "fastmem_same_remote_free_body_producer_pilot": same_remote_free_body_producer,
        "fastmem_page_local_free_route_cfg_preflight": page_local_free_route_cfg_preflight,
        "fastmem_page_local_alloc_route_cfg_preflight": page_local_alloc_route_cfg_preflight,
        "fastmem_page_local_alloc_route_cfg_producer_pilot": page_local_alloc_route_cfg_producer,
        "fastmem_page_local_free_route_cfg_producer_pilot": page_local_free_route_cfg_producer,
        "fastmem_page_local_route_body_join_preflight": page_local_route_body_join_preflight,
        "fastmem_page_local_route_body_join_producer_pilot": page_local_route_body_join_producer,
        "fastmem_tls_backing_transfer_preflight": tls_backing_transfer_preflight,
        "fastmem_tls_backing_transfer_producer_pilot": tls_backing_transfer_producer,
        "page_local_free_route_cfg_any": page_local_free_route_cfg_any,
        "tls_backing_transfer_or_later": tls_backing_transfer_or_later,
        "fastmem_allocator_owner_slot_reuse_preflight": owner_slot_reuse_preflight,
        "fastmem_allocator_owner_slot_reuse_producer_pilot": owner_slot_reuse_producer,
        "fastmem_abandoned_reclaim_preflight": abandoned_reclaim_preflight,
        "fastmem_abandoned_reclaim_producer_pilot": abandoned_reclaim_producer,
        "fastmem_product_activation_preflight": product_activation_preflight,
        "fastmem_product_activation_producer_pilot": product_activation_producer,
        "fastmem_hook_install_preflight": hook_install_preflight,
        "fastmem_hook_install_producer_pilot": hook_install_producer,
        "fastmem_global_allocator_claim_preflight": global_allocator_claim_preflight,
        "fastmem_global_allocator_claim_producer_pilot": global_allocator_claim_producer,
        "fastmem_winner_claim_preflight": winner_claim_preflight,
        "fastmem_winner_claim_producer_pilot": winner_claim_producer,
    }


def _flag_rows_from_scope(
    flag_scope: Mapping[str, bool],
    *names: str,
) -> list[tuple[str, str]]:
    return _flag_rows(*((name, flag_scope[name]) for name in names))


def _remote_free_cas_producer_pilot_open(
    remote_free_open: bool,
    flag_scope: Mapping[str, bool],
) -> bool:
    return remote_free_open and not any(
        flag_scope[name] for name in REMOTE_FREE_CAS_BLOCKER_NAMES
    )


def _remote_free_atomic_rows(
    remote_free_open: bool,
    flag_scope: Mapping[str, bool],
) -> list[tuple[str, str]]:
    return [
        *_flag_rows(
            (
                "fastmem_atomic_remote_head_cas_preflight",
                flag_scope["fastmem_atomic_remote_head_cas_preflight"],
            ),
            (
                "fastmem_atomic_remote_head_cas_producer_pilot",
                _remote_free_cas_producer_pilot_open(remote_free_open, flag_scope),
            ),
        ),
        *_flag_rows_from_scope(flag_scope, *REMOTE_FREE_ATOMIC_FLAG_NAMES[2:]),
    ]


def _remote_free_route_family_rows(
    flag_scope: Mapping[str, bool],
) -> list[tuple[str, str]]:
    return [
        *_flag_rows_from_scope(flag_scope, *REMOTE_FREE_ROUTE_FAMILY_FLAG_NAMES),
    ]


def _remote_free_refresh_rows(
    flag_scope: Mapping[str, bool],
    refresh_flag_rows: list[tuple[str, str]],
) -> list[tuple[str, str]]:
    return [
        *refresh_flag_rows,
        *_flag_rows_from_scope(flag_scope, *REMOTE_FREE_REFRESH_FLAG_NAMES),
    ]


def _first_true_route(
    choices: tuple[tuple[bool, str], ...],
    default: str,
) -> str:
    for condition, value in choices:
        if condition:
            return value
    return default


def _inactive_atomic_remote_rows(
    current_owner_source: str,
) -> list[tuple[str, str]]:
    return [
        *_flag_rows(
            ("fastmem_atomic_remote_head_cas_preflight", False),
            ("fastmem_atomic_remote_head_cas_producer_pilot", False),
            ("fastmem_atomic_remote_head_retry_preflight", False),
            ("fastmem_atomic_remote_head_retry_producer_pilot", False),
            ("fastmem_atomic_remote_head_drain_preflight", False),
            ("fastmem_atomic_remote_head_drain_exchange_selection", False),
            ("fastmem_atomic_remote_head_drain_exchange_producer_pilot", False),
            ("fastmem_atomic_remote_head_drain_to_local_route_selection", False),
            ("fastmem_atomic_remote_head_drain_to_local_route_producer_pilot", False),
            ("fastmem_atomic_remote_head_drain_local_list_mutation_preflight", False),
            ("fastmem_atomic_remote_head_drain_local_list_mutation_proof", False),
            (
                "fastmem_atomic_remote_head_drain_local_list_mutation_vocabulary_preflight",
                False,
            ),
        ),
        ("fastmem_owner_runtime_current_owner_source", current_owner_source),
    ]


def _selected_memop_family(flags: RouteSummaryFlags) -> str:
    if flags.winner_claim_any:
        return "winner_claim"
    if flags.global_allocator_claim_any:
        return "global_allocator_claim"
    if flags.hook_install_any:
        return "hook_install"
    if flags.product_activation_any:
        return "product_activation"
    if flags.abandoned_reclaim_any:
        return "abandoned_reclaim"
    if flags.owner_slot_reuse_any:
        return "owner_slot_reuse"
    if (
        flags.tls_backing_transfer_preflight_refresh
        or flags.tls_backing_transfer_producer_refresh
        or flags.tls_backing_transfer_producer
    ):
        return "tls_backing_transfer"
    if flags.terminal_ladder_refresh_preflight:
        return "terminal_ladder_refresh"
    if flags.page_local_route_body_join_any:
        return "page_local_route_body_join"
    if flags.page_local_free_route_cfg_any or flags.tls_backing_transfer_preflight:
        return "page_local_route_cfg"
    if flags.page_local_alloc_route_cfg_any:
        return "page_local_alloc_route_cfg"
    if (
        flags.same_remote_free_body_preflight
        or flags.same_remote_free_body_producer
    ):
        return "same_remote_free_body"
    if (
        flags.fastmem_branch_cfg_lowering_preflight
        or flags.fastmem_branch_cfg_lowering_producer
    ):
        return "branch_cfg"
    if flags.remote_owner_branch_routing_any:
        return "remote_free_routing"
    return "remote_free"


def _selected_memop_kinds(flags: RouteSummaryFlags, selected_remote_kind: str) -> str:
    if flags.winner_claim_any:
        return "WinnerClaim"
    if flags.global_allocator_claim_any:
        return "GlobalAllocatorClaim"
    if flags.hook_install_any:
        return "HookInstall"
    if flags.product_activation_any:
        return "ProductActivation"
    if flags.abandoned_reclaim_any:
        return "AbandonedReclaim"
    if flags.owner_slot_reuse_any:
        return "OwnerSlotReuse"
    if (
        flags.tls_backing_transfer_preflight_refresh
        or flags.tls_backing_transfer_producer_refresh
        or flags.tls_backing_transfer_producer
    ):
        return "TlsBackingTransfer"
    if flags.terminal_ladder_refresh_preflight:
        return "TerminalLadderRefresh"
    if flags.page_local_route_body_join_preflight:
        return "PageLocalRouteBodyJoin"
    if flags.page_local_route_body_join_producer:
        return "PageLocalRouteBodyJoinProducer"
    if flags.page_local_free_route_cfg_any or flags.tls_backing_transfer_preflight:
        return "PageLocalFreeRouteCfg"
    if flags.page_local_alloc_route_cfg_producer:
        return "PageLocalAllocRouteCfgProducer"
    if flags.page_local_alloc_route_cfg_preflight:
        return "PageLocalAllocRouteCfg"
    if (
        flags.same_remote_free_body_preflight
        or flags.same_remote_free_body_producer
    ):
        return "SameRemoteFreeBody"
    if (
        flags.fastmem_branch_cfg_lowering_preflight
        or flags.fastmem_branch_cfg_lowering_producer
    ):
        return "FastMemBranchCfg"
    return selected_remote_kind


def _selected_route_from_state(
    state: Mapping[str, Any],
    refresh_spec: Any | None,
) -> str:
    return _first_true_route(
        (
            (state.get("winner_claim_producer", False), "winner_claim_producer_pilot"),
            (state.get("winner_claim_preflight", False), "winner_claim_preflight"),
            (
                state.get("global_allocator_claim_producer", False),
                "global_allocator_claim_producer_pilot",
            ),
            (
                state.get("global_allocator_claim_preflight", False),
                "global_allocator_claim_preflight",
            ),
            (state.get("hook_install_producer", False), "hook_install_producer_pilot"),
            (state.get("hook_install_preflight", False), "hook_install_preflight"),
            (
                state.get("product_activation_producer", False),
                "product_activation_producer_pilot",
            ),
            (state.get("product_activation_preflight", False), "product_activation_preflight"),
            (
                state.get("page_local_route_body_join_producer", False),
                "page_local_route_body_join_producer_pilot",
            ),
            (
                refresh_spec is not None,
                refresh_spec.selected_route if refresh_spec is not None else "none",
            ),
            (
                state.get("page_local_route_body_join_preflight", False),
                "page_local_route_body_join_preflight",
            ),
            (
                state.get("page_local_alloc_route_cfg_producer", False),
                "page_local_alloc_route_cfg_producer_pilot",
            ),
            (
                state.get("page_local_alloc_route_cfg_preflight", False),
                "page_local_alloc_route_cfg_preflight",
            ),
            (
                state.get("page_local_free_route_cfg_preflight", False),
                "page_local_free_route_cfg_preflight",
            ),
            (
                state.get("abandoned_reclaim_producer", False),
                "abandoned_reclaim_producer_pilot",
            ),
            (state.get("abandoned_reclaim_preflight", False), "abandoned_reclaim_preflight"),
            (state.get("owner_slot_reuse_producer", False), "owner_slot_reuse_producer_pilot"),
            (state.get("owner_slot_reuse_preflight", False), "owner_slot_reuse_preflight"),
            (
                state.get("tls_backing_transfer_producer", False),
                "tls_backing_transfer_producer_pilot",
            ),
            (state.get("tls_backing_transfer_preflight", False), "tls_backing_transfer_preflight"),
            (
                state.get("page_local_free_route_cfg_producer", False),
                "page_local_free_route_cfg_producer_pilot",
            ),
            (
                state.get("same_remote_free_body_producer", False),
                "same_remote_free_body_producer_pilot",
            ),
            (
                state.get("same_remote_free_body_preflight", False),
                "same_remote_free_body_preflight",
            ),
            (
                state.get("fastmem_branch_cfg_lowering_producer", False),
                "fastmem_branch_cfg_lowering_producer_pilot",
            ),
            (
                state.get("fastmem_branch_cfg_lowering_preflight", False),
                "fastmem_branch_cfg_lowering_preflight",
            ),
            (state.get("fastmem_branch_cfg_preflight", False), "fastmem_branch_cfg_preflight"),
            (
                state.get("remote_owner_branch_route_body_preflight", False),
                "remote_owner_branch_route_body_preflight",
            ),
            (
                state.get("remote_owner_branch_routing_lowering_producer", False),
                "remote_owner_branch_routing_lowering_producer_pilot",
            ),
            (
                state.get("remote_owner_branch_routing_lowering_preflight", False),
                "remote_owner_branch_routing_lowering_preflight",
            ),
            (
                state.get("remote_owner_branch_routing_preflight", False),
                "remote_owner_branch_routing_preflight",
            ),
        ),
        default="none",
    )


def _route_family_progression_from_flags(
    *,
    remote_owner_branch_routing_preflight: bool,
    remote_owner_branch_routing_lowering_preflight: bool,
    remote_owner_branch_routing_lowering_producer: bool,
    remote_owner_branch_route_body_preflight: bool,
    fastmem_branch_cfg_preflight: bool,
    fastmem_branch_cfg_lowering_preflight: bool,
    fastmem_branch_cfg_lowering_producer: bool,
    same_remote_free_body_preflight: bool,
    same_remote_free_body_producer: bool,
    page_local_alloc_route_cfg_preflight: bool,
    page_local_alloc_route_cfg_producer: bool,
    page_local_free_route_cfg_preflight: bool,
    page_local_free_route_cfg_producer: bool,
    page_local_route_body_join_preflight: bool,
    page_local_route_body_join_producer: bool,
    tls_backing_transfer_preflight: bool,
    tls_backing_transfer_producer: bool,
    owner_slot_reuse_preflight: bool,
    owner_slot_reuse_producer: bool,
    abandoned_reclaim_preflight: bool,
    abandoned_reclaim_producer: bool,
    product_activation_preflight: bool,
    product_activation_producer: bool,
    hook_install_preflight: bool,
    hook_install_producer: bool,
    global_allocator_claim_preflight: bool,
    global_allocator_claim_producer: bool,
    winner_claim_preflight: bool,
    winner_claim_producer: bool,
    remote_free_drain_local_list_mutation_verifier_preconditions: bool,
    remote_free_drain_local_list_mutation_lowering_producer: bool,
    remote_free_drain_any: bool,
    remote_owner_branch_routing_any: bool,
) -> tuple[str, str, str]:
    if remote_owner_branch_routing_preflight:
        next_slice = "remote_owner_branch_routing_lowering_preflight"
        deferred_remote_kinds = "RemoteOwnerBranchRoutingLowering"
    elif remote_owner_branch_routing_lowering_preflight:
        next_slice = "remote_owner_branch_routing_lowering_producer_pilot"
        deferred_remote_kinds = "RemoteOwnerBranchRoutingLowering"
    elif remote_owner_branch_routing_lowering_producer:
        next_slice = "remote_owner_branch_route_body_preflight"
        deferred_remote_kinds = "SameRemoteFreeBody,BranchCfgLowering"
    elif remote_owner_branch_route_body_preflight:
        next_slice = "fastmem_branch_cfg_preflight"
        deferred_remote_kinds = "BranchCfgLowering,SameRemoteFreeBody"
    elif fastmem_branch_cfg_preflight:
        next_slice = "fastmem_branch_cfg_lowering_preflight"
        deferred_remote_kinds = "BranchCfgLowering,SameRemoteFreeBody"
    elif fastmem_branch_cfg_lowering_preflight:
        next_slice = "fastmem_branch_cfg_lowering_producer_pilot"
        deferred_remote_kinds = "BranchCfgLoweringProducer,SameRemoteFreeBody"
    elif fastmem_branch_cfg_lowering_producer:
        next_slice = "same_remote_free_body_preflight"
        deferred_remote_kinds = "SameRemoteFreeBody"
    elif same_remote_free_body_preflight:
        next_slice = "same_remote_free_body_producer_pilot"
        deferred_remote_kinds = "SameRemoteFreeBodyProducer"
    elif same_remote_free_body_producer:
        next_slice = "page_local_free_route_cfg_preflight"
        deferred_remote_kinds = "PageLocalFreeRouteCfg,TlsBackingTransfer"
    elif page_local_alloc_route_cfg_preflight:
        next_slice = "page_local_alloc_route_cfg_producer_pilot"
        deferred_remote_kinds = "PageLocalAllocRouteCfgProducer,PageLocalFreeRouteCfg"
    elif page_local_alloc_route_cfg_producer:
        next_slice = "page_local_free_route_cfg_preflight"
        deferred_remote_kinds = "PageLocalFreeRouteCfg"
    elif page_local_free_route_cfg_preflight:
        next_slice = "page_local_free_route_cfg_producer_pilot"
        deferred_remote_kinds = "PageLocalFreeRouteCfgProducer,TlsBackingTransfer"
    elif page_local_free_route_cfg_producer:
        next_slice = "tls_backing_transfer_preflight"
        deferred_remote_kinds = "TlsBackingTransfer"
    elif page_local_route_body_join_preflight:
        next_slice = "page_local_route_body_join_producer_pilot"
        deferred_remote_kinds = "PageLocalRouteBodyJoinProducer,TlsBackingTransfer"
    elif page_local_route_body_join_producer:
        next_slice = "terminal_ladder_refresh_preflight"
        deferred_remote_kinds = "TerminalLadderRefresh,TlsBackingTransfer"
    elif tls_backing_transfer_preflight:
        next_slice = "tls_backing_transfer_producer_pilot"
        deferred_remote_kinds = "TlsBackingTransferProducer,OwnerSlotReuse"
    elif tls_backing_transfer_producer:
        next_slice = "owner_slot_reuse_preflight"
        deferred_remote_kinds = "OwnerSlotReuse"
    elif owner_slot_reuse_preflight:
        next_slice = "owner_slot_reuse_producer_pilot"
        deferred_remote_kinds = "OwnerSlotReuseProducer,AbandonedReclaim"
    elif owner_slot_reuse_producer:
        next_slice = "abandoned_reclaim_preflight"
        deferred_remote_kinds = "AbandonedReclaim"
    elif abandoned_reclaim_preflight:
        next_slice = "abandoned_reclaim_producer_pilot"
        deferred_remote_kinds = "AbandonedReclaimProducer,ProductActivation"
    elif abandoned_reclaim_producer:
        next_slice = "product_activation_preflight"
        deferred_remote_kinds = "ProductActivation"
    elif product_activation_preflight:
        next_slice = "product_activation_producer_pilot"
        deferred_remote_kinds = "ProductActivationProducer,HookInstall"
    elif product_activation_producer:
        next_slice = "hook_install_preflight"
        deferred_remote_kinds = "HookInstall"
    elif hook_install_preflight:
        next_slice = "hook_install_producer_pilot"
        deferred_remote_kinds = "HookInstallProducer,GlobalAllocatorClaim"
    elif hook_install_producer:
        next_slice = "global_allocator_claim_preflight"
        deferred_remote_kinds = "GlobalAllocatorClaim"
    elif global_allocator_claim_preflight:
        next_slice = "global_allocator_claim_producer_pilot"
        deferred_remote_kinds = "GlobalAllocatorClaimProducer,WinnerClaim"
    elif global_allocator_claim_producer:
        next_slice = "winner_claim_preflight"
        deferred_remote_kinds = "WinnerClaim"
    elif winner_claim_preflight:
        next_slice = "winner_claim_producer_pilot"
        deferred_remote_kinds = "WinnerClaimProducer"
    elif winner_claim_producer:
        next_slice = "complete"
        deferred_remote_kinds = "none"
    else:
        next_slice = "atomic_remote_head_cas_lowering_producer_pilot"
        deferred_remote_kinds = "AtomicRemoteHeadDrain,RemoteOwnerBranchRouting"

    selected_remote_kind = (
        "AtomicRemoteHeadDrain"
        if remote_free_drain_any or remote_owner_branch_routing_any
        else "AtomicRemoteHeadPush"
    )
    if remote_free_drain_local_list_mutation_verifier_preconditions or (
        remote_free_drain_local_list_mutation_lowering_producer
    ):
        selected_remote_kind = "DrainRemoteListToLocal"
    if remote_owner_branch_routing_any:
        selected_remote_kind = "RemoteOwnerBranchRouting"
    return next_slice, deferred_remote_kinds, selected_remote_kind


def _build_route_summary(
    *,
    profile: str,
    state: Mapping[str, Any],
    refresh_spec: Any | None,
    route_family_flags: Mapping[str, bool],
) -> dict[str, Any]:
    route_spec = remote_free_route_profile_spec(profile)
    if route_spec is not None:
        return {
            "route_candidate": "none",
            "remote_free_open": True,
            "route_family": True,
            "next_slice": route_spec.next_slice,
            "deferred_remote_kinds": route_spec.deferred_kinds,
            "selected_remote_kind": route_spec.selected_remote_kind,
            "selected_route": route_spec.selected_route,
        }

    if profile not in (
        "owner-runtime",
        "local-free",
        "layout_table",
    ):
        if refresh_spec is not None:
            next_slice = refresh_spec.next_slice
            deferred_remote_kinds = refresh_spec.deferred_kinds
            _, _, selected_remote_kind = _route_family_progression_from_flags(
                **route_family_flags
            )
        else:
            next_slice, deferred_remote_kinds, selected_remote_kind = (
                _route_family_progression_from_flags(**route_family_flags)
            )
        return {
            "route_candidate": "none",
            "remote_free_open": False,
            "route_family": True,
            "next_slice": next_slice,
            "deferred_remote_kinds": deferred_remote_kinds,
            "selected_remote_kind": selected_remote_kind,
            "selected_route": _selected_route_from_state(state, refresh_spec),
        }

    return {
        "route_candidate": "none",
        "remote_free_open": False,
        "route_family": False,
        "next_slice": "layout_table_producer_pilot",
        "deferred_remote_kinds": "CurrentAllocOwnerId,OwnerEq",
        "selected_remote_kind": "LayoutTable",
        "selected_route": _selected_route_from_state(state, refresh_spec),
    }


def _non_route_family_state(
    *,
    profile: str,
    verified_local_free_pop: list[dict[str, Any]],
    verified_free_head_push: list[dict[str, Any]],
    verified_free_head_pop: list[dict[str, Any]],
    verified_local_free_push: list[dict[str, Any]],
    selected_local_free_kinds: list[str],
    deferred_local_free_kinds: list[str],
) -> tuple[
    str,
    str,
    str,
    str,
    list[tuple[str, str]],
]:
    if profile == "owner-runtime":
        return (
            "none",
            "none",
            "owner_runtime",
            "CurrentAllocOwnerId,OwnerEq",
            _owner_runtime_slice_rows(),
        )
    if profile == "local-free":
        route_candidate = page_local_alloc_route_candidate(
            local_free_pop_count=len(verified_local_free_pop),
            free_head_push_count=len(verified_free_head_push),
            free_head_pop_count=len(verified_free_head_pop),
        )
        free_route_candidate = page_local_free_route_candidate(
            local_free_push_count=len(verified_local_free_push),
            local_free_pop_count=len(verified_local_free_pop),
            free_head_push_count=len(verified_free_head_push),
            free_head_pop_count=len(verified_free_head_pop),
        )
        selected_local_free_kinds_str = (
            ",".join(selected_local_free_kinds)
            if selected_local_free_kinds
            else "none"
        )
        return (
            route_candidate,
            free_route_candidate,
            "local_free",
            selected_local_free_kinds_str,
            _local_free_slice_rows(
                selected_local_free_kinds_str,
                ",".join(deferred_local_free_kinds),
            ),
        )
    return (
        "none",
        "none",
        "layout_table",
        "TableIndex,FieldLoad,FieldStore",
        _layout_table_slice_rows(),
    )


def _owner_runtime_slice_rows() -> list[tuple[str, str]]:
    return [
        *_slice_prefix_rows(
            selection_v0="0",
            next_slice="owner_runtime_producer_pilot",
            selected_memop_family="owner_runtime",
            selected_memop_kinds="CurrentAllocOwnerId,OwnerEq",
            deferred_memop_family="remote_free",
            deferred_memop_kinds="AtomicRemoteHead",
            owner_runtime_pilot=True,
            local_free_pilot=False,
            layout_table_pilot=False,
        ),
        *_inactive_atomic_remote_rows("llvm_producer_intrinsic"),
    ]


def _local_free_slice_rows(
    selected_local_free_kinds: str,
    deferred_local_free_kinds: str,
) -> list[tuple[str, str]]:
    return [
        *_slice_prefix_rows(
            selection_v0="0",
            next_slice="local_free_producer_pilot",
            selected_memop_family="local_free",
            selected_memop_kinds=selected_local_free_kinds,
            deferred_memop_family="remote_free",
            deferred_memop_kinds=deferred_local_free_kinds,
            owner_runtime_pilot=False,
            local_free_pilot=True,
            layout_table_pilot=False,
        ),
        *_inactive_atomic_remote_rows("llvm_producer_intrinsic"),
    ]


def _layout_table_slice_rows() -> list[tuple[str, str]]:
    return [
        *_slice_prefix_rows(
            selection_v0="1",
            next_slice="layout_table_producer_pilot",
            selected_memop_family="layout_table",
            selected_memop_kinds="TableIndex,FieldLoad,FieldStore",
            deferred_memop_family="owner_runtime",
            deferred_memop_kinds="CurrentAllocOwnerId,OwnerEq",
            owner_runtime_pilot=False,
            local_free_pilot=False,
            layout_table_pilot=True,
        ),
        *_inactive_atomic_remote_rows("closed"),
    ]


def _remote_free_slice_rows(ctx: RemoteFreeSliceContext) -> list[tuple[str, str]]:
    return [
        *_slice_prefix_rows(
            selection_v0="0",
            selected_route=ctx.selected_route,
            next_slice=ctx.next_slice,
            selected_memop_family=ctx.selected_memop_family,
            selected_memop_kinds=ctx.selected_memop_kinds,
            deferred_memop_family="remote_free_execution",
            deferred_memop_kinds=ctx.deferred_remote_kinds,
            owner_runtime_pilot=False,
            local_free_pilot=False,
            layout_table_pilot=False,
        ),
        *_remote_free_atomic_rows(ctx.remote_free_open, ctx.flag_scope),
        *_remote_free_route_family_rows(ctx.flag_scope),
        *_remote_free_refresh_rows(ctx.flag_scope, ctx.refresh_flag_rows),
        ("fastmem_owner_runtime_current_owner_source", "closed"),
    ]




def _route_family_flags(state: dict[str, Any]) -> dict[str, bool]:
    return {
        "remote_owner_branch_routing_preflight": state["remote_owner_branch_routing_preflight"],
        "remote_owner_branch_routing_lowering_preflight": state["remote_owner_branch_routing_lowering_preflight"],
        "remote_owner_branch_routing_lowering_producer": state["remote_owner_branch_routing_lowering_producer"],
        "remote_owner_branch_route_body_preflight": state["remote_owner_branch_route_body_preflight"],
        "fastmem_branch_cfg_preflight": state["fastmem_branch_cfg_preflight"],
        "fastmem_branch_cfg_lowering_preflight": state["fastmem_branch_cfg_lowering_preflight"],
        "fastmem_branch_cfg_lowering_producer": state["fastmem_branch_cfg_lowering_producer"],
        "same_remote_free_body_preflight": state["same_remote_free_body_preflight"],
        "same_remote_free_body_producer": state["same_remote_free_body_producer"],
        "page_local_alloc_route_cfg_preflight": state["page_local_alloc_route_cfg_preflight"],
        "page_local_alloc_route_cfg_producer": state["page_local_alloc_route_cfg_producer"],
        "page_local_free_route_cfg_preflight": state["page_local_free_route_cfg_preflight"],
        "page_local_free_route_cfg_producer": state["page_local_free_route_cfg_producer"],
        "page_local_route_body_join_preflight": state["page_local_route_body_join_preflight"],
        "page_local_route_body_join_producer": state["page_local_route_body_join_producer"],
        "tls_backing_transfer_preflight": state["tls_backing_transfer_preflight"],
        "tls_backing_transfer_producer": state["tls_backing_transfer_producer"],
        "owner_slot_reuse_preflight": state["owner_slot_reuse_preflight"],
        "owner_slot_reuse_producer": state["owner_slot_reuse_producer"],
        "abandoned_reclaim_preflight": state["abandoned_reclaim_preflight"],
        "abandoned_reclaim_producer": state["abandoned_reclaim_producer"],
        "product_activation_preflight": state["product_activation_preflight"],
        "product_activation_producer": state["product_activation_producer"],
        "hook_install_preflight": state["hook_install_preflight"],
        "hook_install_producer": state["hook_install_producer"],
        "global_allocator_claim_preflight": state["global_allocator_claim_preflight"],
        "global_allocator_claim_producer": state["global_allocator_claim_producer"],
        "winner_claim_preflight": state["winner_claim_preflight"],
        "winner_claim_producer": state["winner_claim_producer"],
        "remote_free_drain_local_list_mutation_verifier_preconditions": state["remote_free_drain_local_list_mutation_verifier_preconditions"],
        "remote_free_drain_local_list_mutation_lowering_producer": state["remote_free_drain_local_list_mutation_lowering_producer"],
        "remote_free_drain_any": state["remote_free_drain_any"],
        "remote_owner_branch_routing_any": state["remote_owner_branch_routing_any"],
    }


def _route_summary_flags(
    *,
    winner_claim_any: bool,
    global_allocator_claim_any: bool,
    hook_install_any: bool,
    product_activation_any: bool,
    abandoned_reclaim_any: bool,
    owner_slot_reuse_any: bool,
    tls_backing_transfer_preflight_refresh: bool,
    tls_backing_transfer_producer_refresh: bool,
    tls_backing_transfer_preflight: bool,
    tls_backing_transfer_producer: bool,
    terminal_ladder_refresh_preflight: bool,
    page_local_route_body_join_any: bool,
    page_local_route_body_join_preflight: bool,
    page_local_route_body_join_producer: bool,
    page_local_free_route_cfg_any: bool,
    page_local_free_route_cfg_preflight: bool,
    page_local_alloc_route_cfg_any: bool,
    page_local_alloc_route_cfg_preflight: bool,
    page_local_alloc_route_cfg_producer: bool,
    same_remote_free_body_preflight: bool,
    same_remote_free_body_producer: bool,
    fastmem_branch_cfg_lowering_preflight: bool,
    fastmem_branch_cfg_lowering_producer: bool,
    remote_owner_branch_routing_any: bool,
) -> RouteSummaryFlags:
    return RouteSummaryFlags(
        winner_claim_any=winner_claim_any,
        global_allocator_claim_any=global_allocator_claim_any,
        hook_install_any=hook_install_any,
        product_activation_any=product_activation_any,
        abandoned_reclaim_any=abandoned_reclaim_any,
        owner_slot_reuse_any=owner_slot_reuse_any,
        tls_backing_transfer_preflight_refresh=tls_backing_transfer_preflight_refresh,
        tls_backing_transfer_producer_refresh=tls_backing_transfer_producer_refresh,
        tls_backing_transfer_preflight=tls_backing_transfer_preflight,
        tls_backing_transfer_producer=tls_backing_transfer_producer,
        terminal_ladder_refresh_preflight=terminal_ladder_refresh_preflight,
        page_local_route_body_join_any=page_local_route_body_join_any,
        page_local_route_body_join_preflight=page_local_route_body_join_preflight,
        page_local_route_body_join_producer=page_local_route_body_join_producer,
        page_local_free_route_cfg_any=page_local_free_route_cfg_any,
        page_local_free_route_cfg_preflight=page_local_free_route_cfg_preflight,
        page_local_alloc_route_cfg_any=page_local_alloc_route_cfg_any,
        page_local_alloc_route_cfg_preflight=page_local_alloc_route_cfg_preflight,
        page_local_alloc_route_cfg_producer=page_local_alloc_route_cfg_producer,
        same_remote_free_body_preflight=same_remote_free_body_preflight,
        same_remote_free_body_producer=same_remote_free_body_producer,
        fastmem_branch_cfg_lowering_preflight=fastmem_branch_cfg_lowering_preflight,
        fastmem_branch_cfg_lowering_producer=fastmem_branch_cfg_lowering_producer,
        remote_owner_branch_routing_any=remote_owner_branch_routing_any,
    )

def build_route_state(state: dict[str, Any]) -> dict[str, Any]:
    state = dict(state)
    deferred_local_free_kinds = state["deferred_local_free_kinds"]
    profile = state["profile"]
    refresh_spec = refresh_profile_spec(profile)
    refresh_route = refresh_spec.selected_route if refresh_spec is not None else ""
    refresh_flag_rows = [
        (spec.report_flag, str(int_flag(profile == spec.profile)))
        for spec in REFRESH_PROFILE_SPECS
    ]
    selected_local_free_kinds = state["selected_local_free_kinds"]
    verified_free_head_pop = state["verified_free_head_pop"]
    verified_free_head_push = state["verified_free_head_push"]
    verified_local_free_pop = state["verified_local_free_pop"]
    verified_local_free_push = state["verified_local_free_push"]
    remote_free_open = False
    remote_free_retry_preflight = profile == "remote-free-retry-preflight"
    remote_free_retry_producer = profile == "remote-free-retry"
    remote_free_drain_preflight = profile == "remote-free-drain-preflight"
    remote_free_drain_exchange_selection = (
        profile == "remote-free-drain-exchange-selection"
    )
    remote_free_drain_exchange_producer = profile == "remote-free-drain-exchange"
    remote_free_drain_to_local_selection = (
        profile == "remote-free-drain-to-local-selection"
    )
    remote_free_drain_to_local_producer = profile == "remote-free-drain-to-local"
    remote_free_drain_local_list_mutation_preflight = (
        profile == "remote-free-drain-local-list-mutation-preflight"
    )
    remote_free_drain_local_list_mutation_proof = (
        profile == "remote-free-drain-local-list-mutation-proof"
    )
    remote_free_drain_local_list_mutation_vocabulary_preflight = (
        profile == "remote-free-drain-local-list-mutation-vocabulary-preflight"
    )
    remote_free_drain_local_list_mutation_verifier_preconditions = (
        profile == "remote-free-drain-local-list-mutation-verifier-preconditions"
    )
    remote_free_drain_local_list_mutation_lowering_producer = (
        profile == "remote-free-drain-local-list-mutation-lowering"
    )
    remote_owner_branch_routing_preflight = profile == "remote-owner-branch-routing-preflight"
    remote_owner_branch_routing_lowering_preflight = (
        profile == "remote-owner-branch-routing-lowering-preflight"
    )
    remote_owner_branch_routing_lowering_producer = (
        profile == "remote-owner-branch-routing-lowering"
    )
    remote_owner_branch_route_body_preflight = (
        profile == "remote-owner-branch-route-body-preflight"
    )
    fastmem_branch_cfg_preflight = profile == "fastmem-branch-cfg-preflight"
    fastmem_branch_cfg_lowering_preflight = (
        profile == "fastmem-branch-cfg-lowering-preflight"
    )
    fastmem_branch_cfg_lowering_producer = profile == "fastmem-branch-cfg-lowering"
    same_remote_free_body_preflight = profile == "same-remote-free-body-preflight"
    same_remote_free_body_producer = profile == "same-remote-free-body"
    progression = _activation_progression_flags(profile, refresh_route)
    page_local_route_body_join_preflight = progression.page_local_route_body_join_preflight
    page_local_route_body_join_producer = progression.page_local_route_body_join_producer
    terminal_ladder_refresh_preflight = progression.terminal_ladder_refresh_preflight
    page_local_route_body_join_any = progression.page_local_route_body_join_any
    page_local_alloc_route_cfg_preflight = progression.page_local_alloc_route_cfg_preflight
    page_local_alloc_route_cfg_producer = progression.page_local_alloc_route_cfg_producer
    page_local_alloc_route_cfg_any = progression.page_local_alloc_route_cfg_any
    page_local_free_route_cfg_preflight = progression.page_local_free_route_cfg_preflight
    page_local_free_route_cfg_producer = progression.page_local_free_route_cfg_producer
    page_local_free_route_cfg_any = progression.page_local_free_route_cfg_any
    tls_backing_transfer_preflight_refresh = progression.tls_backing_transfer_preflight_refresh
    tls_backing_transfer_producer_refresh = progression.tls_backing_transfer_producer_refresh
    tls_backing_transfer_preflight = progression.tls_backing_transfer_preflight
    tls_backing_transfer_producer = progression.tls_backing_transfer_producer
    tls_backing_transfer_any = progression.tls_backing_transfer_any
    owner_slot_reuse_preflight = progression.owner_slot_reuse_preflight
    owner_slot_reuse_preflight_refresh = progression.owner_slot_reuse_preflight_refresh
    owner_slot_reuse_producer_refresh = progression.owner_slot_reuse_producer_refresh
    owner_slot_reuse_producer = progression.owner_slot_reuse_producer
    owner_slot_reuse_any = progression.owner_slot_reuse_any
    owner_slot_reuse_or_later = progression.owner_slot_reuse_or_later
    abandoned_reclaim_preflight_refresh = progression.abandoned_reclaim_preflight_refresh
    abandoned_reclaim_producer_refresh = progression.abandoned_reclaim_producer_refresh
    abandoned_reclaim_preflight = progression.abandoned_reclaim_preflight
    abandoned_reclaim_producer = progression.abandoned_reclaim_producer
    abandoned_reclaim_any = progression.abandoned_reclaim_any
    abandoned_reclaim_or_later = progression.abandoned_reclaim_or_later
    product_activation_preflight_refresh = progression.product_activation_preflight_refresh
    product_activation_producer_refresh = progression.product_activation_producer_refresh
    product_activation_preflight = progression.product_activation_preflight
    product_activation_producer = progression.product_activation_producer
    product_activation_any = progression.product_activation_any
    product_activation_or_later = progression.product_activation_or_later
    hook_install_preflight_refresh = progression.hook_install_preflight_refresh
    hook_install_producer_refresh = progression.hook_install_producer_refresh
    hook_install_preflight = progression.hook_install_preflight
    hook_install_producer = progression.hook_install_producer
    hook_install_any = progression.hook_install_any
    hook_install_or_later = progression.hook_install_or_later
    global_allocator_claim_preflight_refresh = progression.global_allocator_claim_preflight_refresh
    global_allocator_claim_producer_refresh = progression.global_allocator_claim_producer_refresh
    global_allocator_claim_preflight = progression.global_allocator_claim_preflight
    global_allocator_claim_producer = progression.global_allocator_claim_producer
    global_allocator_claim_any = progression.global_allocator_claim_any
    global_allocator_claim_or_later = progression.global_allocator_claim_or_later
    winner_claim_preflight_refresh = progression.winner_claim_preflight_refresh
    winner_claim_producer_refresh = progression.winner_claim_producer_refresh
    winner_claim_preflight = progression.winner_claim_preflight
    winner_claim_producer = progression.winner_claim_producer
    winner_claim_any = progression.winner_claim_any
    tls_backing_transfer_or_later = progression.tls_backing_transfer_or_later
    remote_owner_branch_routing_any = _any_true(
        remote_owner_branch_routing_preflight,
        remote_owner_branch_routing_lowering_preflight,
        remote_owner_branch_routing_lowering_producer,
        remote_owner_branch_route_body_preflight,
        fastmem_branch_cfg_preflight,
        fastmem_branch_cfg_lowering_preflight,
        fastmem_branch_cfg_lowering_producer,
        same_remote_free_body_preflight,
        same_remote_free_body_producer,
        page_local_free_route_cfg_any,
        tls_backing_transfer_or_later,
    )
    remote_free_drain_local_list_mutation_any = _any_true(
        remote_free_drain_local_list_mutation_preflight,
        remote_free_drain_local_list_mutation_proof,
        remote_free_drain_local_list_mutation_vocabulary_preflight,
        remote_free_drain_local_list_mutation_verifier_preconditions,
        remote_free_drain_local_list_mutation_lowering_producer,
    )
    remote_free_drain_to_local_any = _any_true(
        remote_free_drain_to_local_selection,
        remote_free_drain_to_local_producer,
        remote_free_drain_local_list_mutation_any,
    )
    remote_free_drain_exchange_any = _any_true(
        remote_free_drain_exchange_selection,
        remote_free_drain_exchange_producer,
        remote_free_drain_to_local_any,
    )
    remote_free_drain_any = _any_true(
        remote_free_drain_preflight,
        remote_free_drain_exchange_any,
        remote_free_drain_to_local_any,
        remote_free_drain_local_list_mutation_any,
    )
    route_family_flags = _route_family_flags(locals())
    route_summary = _build_route_summary(
        profile=profile,
        state=state,
        refresh_spec=refresh_spec,
        route_family_flags=route_family_flags,
    )
    route_candidate = route_summary["route_candidate"]
    remote_free_open = route_summary["remote_free_open"]
    route_family = route_summary["route_family"]
    next_slice = route_summary["next_slice"]
    deferred_remote_kinds = route_summary["deferred_remote_kinds"]
    selected_remote_kind = route_summary["selected_remote_kind"]
    selected_route = route_summary["selected_route"]
    free_route_candidate = "none"
    route_summary_flags = _route_summary_flags(
        winner_claim_any=winner_claim_any,
        global_allocator_claim_any=global_allocator_claim_any,
        hook_install_any=hook_install_any,
        product_activation_any=product_activation_any,
        abandoned_reclaim_any=abandoned_reclaim_any,
        owner_slot_reuse_any=owner_slot_reuse_any,
        tls_backing_transfer_preflight_refresh=tls_backing_transfer_preflight_refresh,
        tls_backing_transfer_producer_refresh=tls_backing_transfer_producer_refresh,
        tls_backing_transfer_preflight=tls_backing_transfer_preflight,
        tls_backing_transfer_producer=tls_backing_transfer_producer,
        terminal_ladder_refresh_preflight=terminal_ladder_refresh_preflight,
        page_local_route_body_join_any=page_local_route_body_join_any,
        page_local_route_body_join_preflight=page_local_route_body_join_preflight,
        page_local_route_body_join_producer=page_local_route_body_join_producer,
        page_local_free_route_cfg_any=page_local_free_route_cfg_any,
        page_local_free_route_cfg_preflight=page_local_free_route_cfg_preflight,
        page_local_alloc_route_cfg_any=page_local_alloc_route_cfg_any,
        page_local_alloc_route_cfg_preflight=page_local_alloc_route_cfg_preflight,
        page_local_alloc_route_cfg_producer=page_local_alloc_route_cfg_producer,
        same_remote_free_body_preflight=same_remote_free_body_preflight,
        same_remote_free_body_producer=same_remote_free_body_producer,
        fastmem_branch_cfg_lowering_preflight=fastmem_branch_cfg_lowering_preflight,
        fastmem_branch_cfg_lowering_producer=fastmem_branch_cfg_lowering_producer,
        remote_owner_branch_routing_any=remote_owner_branch_routing_any,
    )
    selected_memop_family = _selected_memop_family(route_summary_flags)
    selected_memop_kinds = _selected_memop_kinds(
        route_summary_flags,
        selected_remote_kind,
    )
    flag_scope = _remote_free_flag_scope(
        remote_free_open=remote_free_open,
        page_local_alloc_route_cfg_any=page_local_alloc_route_cfg_any,
        remote_free_retry_preflight=remote_free_retry_preflight,
        remote_free_retry_producer=remote_free_retry_producer,
        remote_free_drain_preflight=remote_free_drain_preflight,
        remote_free_drain_exchange_selection=remote_free_drain_exchange_selection,
        remote_free_drain_exchange_producer=remote_free_drain_exchange_producer,
        remote_free_drain_to_local_selection=remote_free_drain_to_local_selection,
        remote_free_drain_to_local_producer=remote_free_drain_to_local_producer,
        remote_free_drain_local_list_mutation_preflight=remote_free_drain_local_list_mutation_preflight,
        remote_free_drain_local_list_mutation_proof=remote_free_drain_local_list_mutation_proof,
        remote_free_drain_local_list_mutation_vocabulary_preflight=remote_free_drain_local_list_mutation_vocabulary_preflight,
        remote_free_drain_local_list_mutation_verifier_preconditions=remote_free_drain_local_list_mutation_verifier_preconditions,
        remote_free_drain_local_list_mutation_lowering_producer=remote_free_drain_local_list_mutation_lowering_producer,
        remote_owner_branch_routing_preflight=remote_owner_branch_routing_preflight,
        remote_owner_branch_routing_lowering_preflight=remote_owner_branch_routing_lowering_preflight,
        remote_owner_branch_routing_lowering_producer=remote_owner_branch_routing_lowering_producer,
        remote_owner_branch_route_body_preflight=remote_owner_branch_route_body_preflight,
        fastmem_branch_cfg_preflight=fastmem_branch_cfg_preflight,
        fastmem_branch_cfg_lowering_preflight=fastmem_branch_cfg_lowering_preflight,
        fastmem_branch_cfg_lowering_producer=fastmem_branch_cfg_lowering_producer,
        same_remote_free_body_preflight=same_remote_free_body_preflight,
        same_remote_free_body_producer=same_remote_free_body_producer,
        page_local_free_route_cfg_preflight=page_local_free_route_cfg_preflight,
        page_local_alloc_route_cfg_preflight=page_local_alloc_route_cfg_preflight,
        page_local_alloc_route_cfg_producer=page_local_alloc_route_cfg_producer,
        page_local_free_route_cfg_producer=page_local_free_route_cfg_producer,
        page_local_route_body_join_preflight=page_local_route_body_join_preflight,
        page_local_route_body_join_producer=page_local_route_body_join_producer,
        tls_backing_transfer_preflight=tls_backing_transfer_preflight,
        tls_backing_transfer_producer=tls_backing_transfer_producer,
        page_local_free_route_cfg_any=page_local_free_route_cfg_any,
        tls_backing_transfer_or_later=tls_backing_transfer_or_later,
        owner_slot_reuse_preflight=owner_slot_reuse_preflight,
        owner_slot_reuse_producer=owner_slot_reuse_producer,
        abandoned_reclaim_preflight=abandoned_reclaim_preflight,
        abandoned_reclaim_producer=abandoned_reclaim_producer,
        product_activation_preflight=product_activation_preflight,
        product_activation_producer=product_activation_producer,
        hook_install_preflight=hook_install_preflight,
        hook_install_producer=hook_install_producer,
        global_allocator_claim_preflight=global_allocator_claim_preflight,
        global_allocator_claim_producer=global_allocator_claim_producer,
        winner_claim_preflight=winner_claim_preflight,
        winner_claim_producer=winner_claim_producer,
    )
    slice_rows = _remote_free_slice_rows(
        RemoteFreeSliceContext(
            selected_route=selected_route,
            next_slice=next_slice,
            selected_memop_family=selected_memop_family,
            selected_memop_kinds=selected_memop_kinds,
            deferred_remote_kinds=deferred_remote_kinds,
            remote_free_open=remote_free_open,
            refresh_flag_rows=refresh_flag_rows,
            flag_scope=flag_scope,
        )
    )
    if not route_family:
        (
            route_candidate,
            free_route_candidate,
            selected_memop_family,
            selected_memop_kinds,
            slice_rows,
        ) = _non_route_family_state(
            profile=profile,
            verified_local_free_pop=verified_local_free_pop,
            verified_free_head_push=verified_free_head_push,
            verified_free_head_pop=verified_free_head_pop,
            verified_local_free_push=verified_local_free_push,
            selected_local_free_kinds=selected_local_free_kinds,
            deferred_local_free_kinds=deferred_local_free_kinds,
        )

    state.update(locals())
    return state
