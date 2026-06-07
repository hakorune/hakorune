"""FastMemory MIR-to-LLVM producer report route rows."""

from __future__ import annotations

from dataclasses import dataclass
from typing import Any

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
    page_local_alloc_route_cfg_preflight = (
        profile == "page-local-alloc-route-cfg-preflight"
    )
    page_local_alloc_route_cfg_producer = profile == "page-local-alloc-route-cfg"
    page_local_free_route_cfg_preflight = (
        profile == "page-local-free-route-cfg-preflight"
    )
    page_local_free_route_cfg_producer = profile == "page-local-free-route-cfg"
    page_local_route_body_join_preflight = (
        profile == "page-local-route-body-join-preflight"
    )
    page_local_route_body_join_producer = profile == "page-local-route-body-join"
    terminal_ladder_refresh_preflight = (
        refresh_route == "terminal_ladder_refresh_preflight"
    )
    tls_backing_transfer_preflight_refresh = (
        refresh_route == "tls_backing_transfer_preflight_refresh"
    )
    tls_backing_transfer_producer_refresh = (
        refresh_route == "tls_backing_transfer_producer_refresh"
    )
    page_local_route_body_join_any = (
        page_local_route_body_join_preflight
        or page_local_route_body_join_producer
        or terminal_ladder_refresh_preflight
        or tls_backing_transfer_preflight_refresh
        or tls_backing_transfer_producer_refresh
    )
    page_local_alloc_route_cfg_any = (
        page_local_alloc_route_cfg_preflight
        or page_local_alloc_route_cfg_producer
        or page_local_route_body_join_any
    )
    page_local_free_route_cfg_any = (
        page_local_free_route_cfg_preflight
        or page_local_free_route_cfg_producer
        or page_local_route_body_join_any
    )
    tls_backing_transfer_preflight = profile == "tls-backing-transfer-preflight"
    tls_backing_transfer_producer = profile == "tls-backing-transfer-producer-pilot"
    tls_backing_transfer_any = (
        tls_backing_transfer_preflight
        or tls_backing_transfer_preflight_refresh
        or tls_backing_transfer_producer_refresh
        or tls_backing_transfer_producer
    )
    owner_slot_reuse_preflight = profile == "owner-slot-reuse-preflight"
    owner_slot_reuse_preflight_refresh = (
        refresh_route == "owner_slot_reuse_preflight_refresh"
    )
    owner_slot_reuse_producer_refresh = (
        refresh_route == "owner_slot_reuse_producer_refresh"
    )
    owner_slot_reuse_producer = profile == "owner-slot-reuse-producer-pilot"
    owner_slot_reuse_any = (
        owner_slot_reuse_preflight
        or owner_slot_reuse_preflight_refresh
        or owner_slot_reuse_producer_refresh
        or owner_slot_reuse_producer
    )
    abandoned_reclaim_preflight_refresh = (
        refresh_route == "abandoned_reclaim_preflight_refresh"
    )
    abandoned_reclaim_producer_refresh = (
        refresh_route == "abandoned_reclaim_producer_refresh"
    )
    abandoned_reclaim_preflight = profile == "abandoned-reclaim-preflight"
    abandoned_reclaim_producer = profile == "abandoned-reclaim-producer-pilot"
    abandoned_reclaim_any = (
        abandoned_reclaim_preflight_refresh
        or abandoned_reclaim_producer_refresh
        or abandoned_reclaim_preflight
        or abandoned_reclaim_producer
    )
    product_activation_preflight_refresh = (
        refresh_route == "product_activation_preflight_refresh"
    )
    product_activation_producer_refresh = (
        refresh_route == "product_activation_producer_refresh"
    )
    product_activation_preflight = profile == "product-activation-preflight"
    product_activation_producer = profile == "product-activation-producer-pilot"
    product_activation_any = (
        product_activation_preflight_refresh
        or product_activation_producer_refresh
        or product_activation_preflight
        or product_activation_producer
    )
    hook_install_preflight_refresh = refresh_route == "hook_install_preflight_refresh"
    hook_install_producer_refresh = refresh_route == "hook_install_producer_refresh"
    hook_install_preflight = profile == "hook-install-preflight"
    hook_install_producer = profile == "hook-install-producer-pilot"
    hook_install_any = (
        hook_install_preflight_refresh
        or hook_install_producer_refresh
        or hook_install_preflight
        or hook_install_producer
    )
    global_allocator_claim_preflight_refresh = (
        refresh_route == "global_allocator_claim_preflight_refresh"
    )
    global_allocator_claim_preflight = profile == "global-allocator-claim-preflight"
    global_allocator_claim_producer_refresh = (
        refresh_route == "global_allocator_claim_producer_refresh"
    )
    global_allocator_claim_producer = profile == "global-allocator-claim-producer-pilot"
    global_allocator_claim_any = (
        global_allocator_claim_preflight_refresh
        or global_allocator_claim_producer_refresh
        or global_allocator_claim_preflight
        or global_allocator_claim_producer
    )
    winner_claim_preflight_refresh = refresh_route == "winner_claim_preflight_refresh"
    winner_claim_producer_refresh = refresh_route == "winner_claim_producer_refresh"
    winner_claim_preflight = profile == "winner-claim-preflight"
    winner_claim_producer = profile == "winner-claim-producer-pilot"
    winner_claim_any = (
        winner_claim_preflight_refresh
        or winner_claim_producer_refresh
        or winner_claim_preflight
        or winner_claim_producer
    )
    global_allocator_claim_or_later = global_allocator_claim_any or winner_claim_any
    hook_install_or_later = hook_install_any or global_allocator_claim_or_later
    product_activation_or_later = product_activation_any or hook_install_or_later
    abandoned_reclaim_or_later = abandoned_reclaim_any or product_activation_or_later
    owner_slot_reuse_or_later = owner_slot_reuse_any or abandoned_reclaim_or_later
    tls_backing_transfer_or_later = (
        tls_backing_transfer_any or owner_slot_reuse_or_later
    )
    remote_owner_branch_routing_any = (
        remote_owner_branch_routing_preflight
        or remote_owner_branch_routing_lowering_preflight
        or remote_owner_branch_routing_lowering_producer
        or remote_owner_branch_route_body_preflight
        or fastmem_branch_cfg_preflight
        or fastmem_branch_cfg_lowering_preflight
        or fastmem_branch_cfg_lowering_producer
        or same_remote_free_body_preflight
        or same_remote_free_body_producer
        or page_local_free_route_cfg_any
        or tls_backing_transfer_or_later
    )
    remote_owner_branch_routing_lowering_any = (
        remote_owner_branch_routing_lowering_preflight
        or remote_owner_branch_routing_lowering_producer
        or remote_owner_branch_route_body_preflight
        or fastmem_branch_cfg_preflight
        or fastmem_branch_cfg_lowering_preflight
        or fastmem_branch_cfg_lowering_producer
        or same_remote_free_body_preflight
        or same_remote_free_body_producer
        or page_local_free_route_cfg_any
        or tls_backing_transfer_or_later
    )
    def _build_route_summary() -> dict[str, Any]:
        route_spec = remote_free_route_profile_spec(profile)
        if route_spec is not None:
            remote_free_open = True
            route_family = True
            route_candidate = "none"
            next_slice = route_spec.next_slice
            deferred_remote_kinds = route_spec.deferred_kinds
            selected_remote_kind = route_spec.selected_remote_kind
            selected_route = route_spec.selected_route
        elif profile not in (
            "owner-runtime",
            "local-free",
            "layout_table",
        ):
            route_family = True
            remote_free_open = False
            route_candidate = "none"
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
            elif refresh_spec is not None:
                next_slice = refresh_spec.next_slice
                deferred_remote_kinds = refresh_spec.deferred_kinds
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
                if remote_free_drain_preflight
                or remote_free_drain_exchange_selection
                or remote_free_drain_exchange_producer
                or remote_free_drain_to_local_selection
                or remote_free_drain_to_local_producer
                or remote_free_drain_local_list_mutation_preflight
                or remote_free_drain_local_list_mutation_proof
                or remote_free_drain_local_list_mutation_vocabulary_preflight
                or remote_free_drain_local_list_mutation_verifier_preconditions
                or remote_free_drain_local_list_mutation_lowering_producer
                or remote_owner_branch_routing_any
                else "AtomicRemoteHeadPush"
            )
            if remote_free_drain_local_list_mutation_verifier_preconditions or (
                remote_free_drain_local_list_mutation_lowering_producer
            ):
                selected_remote_kind = "DrainRemoteListToLocal"
            if remote_owner_branch_routing_any:
                selected_remote_kind = "RemoteOwnerBranchRouting"
            if winner_claim_producer:
                selected_route = "winner_claim_producer_pilot"
            elif winner_claim_preflight:
                selected_route = "winner_claim_preflight"
            elif global_allocator_claim_producer:
                selected_route = "global_allocator_claim_producer_pilot"
            elif global_allocator_claim_preflight:
                selected_route = "global_allocator_claim_preflight"
            elif hook_install_producer:
                selected_route = "hook_install_producer_pilot"
            elif hook_install_preflight:
                selected_route = "hook_install_preflight"
            elif product_activation_producer:
                selected_route = "product_activation_producer_pilot"
            elif product_activation_preflight:
                selected_route = "product_activation_preflight"
            elif page_local_route_body_join_producer:
                selected_route = "page_local_route_body_join_producer_pilot"
            elif refresh_spec is not None:
                selected_route = refresh_spec.selected_route
            elif page_local_route_body_join_preflight:
                selected_route = "page_local_route_body_join_preflight"
            elif page_local_alloc_route_cfg_producer:
                selected_route = "page_local_alloc_route_cfg_producer_pilot"
            elif page_local_alloc_route_cfg_preflight:
                selected_route = "page_local_alloc_route_cfg_preflight"
            elif page_local_free_route_cfg_preflight:
                selected_route = "page_local_free_route_cfg_preflight"
            elif abandoned_reclaim_producer:
                selected_route = "abandoned_reclaim_producer_pilot"
            elif abandoned_reclaim_preflight:
                selected_route = "abandoned_reclaim_preflight"
            elif owner_slot_reuse_producer:
                selected_route = "owner_slot_reuse_producer_pilot"
            elif owner_slot_reuse_preflight:
                selected_route = "owner_slot_reuse_preflight"
            elif tls_backing_transfer_producer:
                selected_route = "tls_backing_transfer_producer_pilot"
            elif tls_backing_transfer_preflight:
                selected_route = "tls_backing_transfer_preflight"
            elif page_local_free_route_cfg_producer:
                selected_route = "page_local_free_route_cfg_producer_pilot"
            elif same_remote_free_body_producer:
                selected_route = "same_remote_free_body_producer_pilot"
            elif same_remote_free_body_preflight:
                selected_route = "same_remote_free_body_preflight"
            elif fastmem_branch_cfg_lowering_producer:
                selected_route = "fastmem_branch_cfg_lowering_producer_pilot"
            elif fastmem_branch_cfg_lowering_preflight:
                selected_route = "fastmem_branch_cfg_lowering_preflight"
            elif fastmem_branch_cfg_preflight:
                selected_route = "fastmem_branch_cfg_preflight"
            elif remote_owner_branch_route_body_preflight:
                selected_route = "remote_owner_branch_route_body_preflight"
            elif remote_owner_branch_routing_lowering_producer:
                selected_route = "remote_owner_branch_routing_lowering_producer_pilot"
            elif remote_owner_branch_routing_lowering_preflight:
                selected_route = "remote_owner_branch_routing_lowering_preflight"
            elif remote_owner_branch_routing_preflight:
                selected_route = "remote_owner_branch_routing_preflight"
            else:
                selected_route = "none"
        else:
            route_family = False
            remote_free_open = False
            route_candidate = "none"
            next_slice = "layout_table_producer_pilot"
            deferred_remote_kinds = "CurrentAllocOwnerId,OwnerEq"
            selected_remote_kind = "LayoutTable"
            selected_route = "layout_table_producer_pilot"

        return {
            "route_candidate": route_candidate,
            "remote_free_open": remote_free_open,
            "route_family": route_family,
            "next_slice": next_slice,
            "deferred_remote_kinds": deferred_remote_kinds,
            "selected_remote_kind": selected_remote_kind,
            "selected_route": selected_route,
        }

    route_summary = _build_route_summary()
    route_candidate = route_summary["route_candidate"]
    remote_free_open = route_summary["remote_free_open"]
    route_family = route_summary["route_family"]
    next_slice = route_summary["next_slice"]
    deferred_remote_kinds = route_summary["deferred_remote_kinds"]
    selected_remote_kind = route_summary["selected_remote_kind"]
    selected_route = route_summary["selected_route"]
    route_summary_flags = RouteSummaryFlags(
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
    slice_rows = [
            ("replacement_front_producer_slice_selection_v0", "0"),
            ("replacement_front_selected_route", selected_route),
            ("replacement_front_next_producer_slice", next_slice),
            (
                "replacement_front_selected_memop_family",
                selected_memop_family,
            ),
            (
                "replacement_front_selected_memop_kinds",
                selected_memop_kinds,
            ),
            ("replacement_front_deferred_memop_family", "remote_free_execution"),
            ("replacement_front_deferred_memop_kinds", deferred_remote_kinds),
            ("mir_fmem_008b_layout_table_producer_pilot", "0"),
            ("fastmem_owner_runtime_producer_pilot", "0"),
            ("fastmem_local_free_producer_pilot", "0"),
            (
                "fastmem_atomic_remote_head_cas_preflight",
                str(int_flag(not remote_free_open and not page_local_alloc_route_cfg_any)),
            ),
            (
                "fastmem_atomic_remote_head_cas_producer_pilot",
                str(
                    int_flag(
                        remote_free_open
                        and not remote_free_retry_preflight
                        and not remote_free_drain_preflight
                        and not remote_free_drain_exchange_selection
                        and not remote_free_drain_exchange_producer
                        and not remote_free_drain_to_local_selection
                        and not remote_free_drain_to_local_producer
                        and not remote_free_drain_local_list_mutation_preflight
                        and not remote_free_drain_local_list_mutation_proof
                        and not remote_free_drain_local_list_mutation_vocabulary_preflight
                        and not remote_free_drain_local_list_mutation_verifier_preconditions
                        and not remote_free_drain_local_list_mutation_lowering_producer
                        and not remote_owner_branch_routing_preflight
                        and not remote_owner_branch_routing_lowering_preflight
                        and not remote_owner_branch_routing_lowering_producer
                        and not remote_owner_branch_route_body_preflight
                        and not fastmem_branch_cfg_preflight
                        and not fastmem_branch_cfg_lowering_preflight
                        and not fastmem_branch_cfg_lowering_producer
                        and not same_remote_free_body_preflight
                        and not same_remote_free_body_producer
                        and not page_local_free_route_cfg_any
                        and not tls_backing_transfer_or_later
                    )
                ),
            ),
            (
                "fastmem_atomic_remote_head_retry_preflight",
                str(int_flag(remote_free_retry_preflight)),
            ),
            (
                "fastmem_atomic_remote_head_retry_producer_pilot",
                str(int_flag(remote_free_retry_producer)),
            ),
            (
                "fastmem_atomic_remote_head_drain_preflight",
                str(int_flag(remote_free_drain_preflight)),
            ),
            (
                "fastmem_atomic_remote_head_drain_exchange_selection",
                str(int_flag(remote_free_drain_exchange_selection)),
            ),
            (
                "fastmem_atomic_remote_head_drain_exchange_producer_pilot",
                str(int_flag(remote_free_drain_exchange_producer)),
            ),
            (
                "fastmem_atomic_remote_head_drain_to_local_route_selection",
                str(int_flag(remote_free_drain_to_local_selection)),
            ),
            (
                "fastmem_atomic_remote_head_drain_to_local_route_producer_pilot",
                str(int_flag(remote_free_drain_to_local_producer)),
            ),
            (
                "fastmem_atomic_remote_head_drain_local_list_mutation_preflight",
                str(int_flag(remote_free_drain_local_list_mutation_preflight)),
            ),
            (
                "fastmem_atomic_remote_head_drain_local_list_mutation_proof",
                str(int_flag(remote_free_drain_local_list_mutation_proof)),
            ),
            (
                "fastmem_atomic_remote_head_drain_local_list_mutation_vocabulary_preflight",
                str(int_flag(remote_free_drain_local_list_mutation_vocabulary_preflight)),
            ),
            (
                "fastmem_atomic_remote_head_drain_local_list_mutation_verifier_preconditions",
                str(int_flag(remote_free_drain_local_list_mutation_verifier_preconditions)),
            ),
            (
                "fastmem_atomic_remote_head_drain_local_list_mutation_lowering_producer_pilot",
                str(int_flag(remote_free_drain_local_list_mutation_lowering_producer)),
            ),
            (
                "fastmem_remote_owner_branch_routing_preflight",
                str(int_flag(remote_owner_branch_routing_preflight)),
            ),
            (
                "fastmem_remote_owner_branch_routing_lowering_preflight",
                str(int_flag(remote_owner_branch_routing_lowering_preflight)),
            ),
            (
                "fastmem_remote_owner_branch_routing_lowering_producer_pilot",
                str(int_flag(remote_owner_branch_routing_lowering_producer)),
            ),
            (
                "fastmem_remote_owner_branch_route_body_preflight",
                str(int_flag(remote_owner_branch_route_body_preflight)),
            ),
            (
                "fastmem_branch_cfg_preflight",
                str(int_flag(fastmem_branch_cfg_preflight)),
            ),
            (
                "fastmem_branch_cfg_lowering_preflight",
                str(int_flag(fastmem_branch_cfg_lowering_preflight)),
            ),
            (
                "fastmem_branch_cfg_lowering_producer_pilot",
                str(int_flag(fastmem_branch_cfg_lowering_producer)),
            ),
            (
                "fastmem_same_remote_free_body_preflight",
                str(int_flag(same_remote_free_body_preflight)),
            ),
            (
                "fastmem_same_remote_free_body_producer_pilot",
                str(int_flag(same_remote_free_body_producer)),
            ),
            (
                "fastmem_page_local_free_route_cfg_preflight",
                str(int_flag(page_local_free_route_cfg_preflight)),
            ),
            (
                "fastmem_page_local_alloc_route_cfg_preflight",
                str(int_flag(page_local_alloc_route_cfg_preflight)),
            ),
            (
                "fastmem_page_local_alloc_route_cfg_producer_pilot",
                str(int_flag(page_local_alloc_route_cfg_producer)),
            ),
            (
                "fastmem_page_local_free_route_cfg_producer_pilot",
                str(int_flag(page_local_free_route_cfg_producer)),
            ),
            (
                "fastmem_page_local_route_body_join_preflight",
                str(int_flag(page_local_route_body_join_preflight)),
            ),
            (
                "fastmem_page_local_route_body_join_producer_pilot",
                str(int_flag(page_local_route_body_join_producer)),
            ),
            *refresh_flag_rows,
            (
                "fastmem_tls_backing_transfer_preflight",
                str(int_flag(tls_backing_transfer_preflight)),
            ),
            (
                "fastmem_tls_backing_transfer_producer_pilot",
                str(int_flag(tls_backing_transfer_producer)),
            ),
            (
                "fastmem_allocator_owner_slot_reuse_preflight",
                str(int_flag(owner_slot_reuse_preflight)),
            ),
            (
                "fastmem_allocator_owner_slot_reuse_producer_pilot",
                str(int_flag(owner_slot_reuse_producer)),
            ),
            (
                "fastmem_abandoned_reclaim_preflight",
                str(int_flag(abandoned_reclaim_preflight)),
            ),
            (
                "fastmem_abandoned_reclaim_producer_pilot",
                str(int_flag(abandoned_reclaim_producer)),
            ),
            (
                "fastmem_product_activation_preflight",
                str(int_flag(product_activation_preflight)),
            ),
            (
                "fastmem_product_activation_producer_pilot",
                str(int_flag(product_activation_producer)),
            ),
            (
                "fastmem_hook_install_preflight",
                str(int_flag(hook_install_preflight)),
            ),
            (
                "fastmem_hook_install_producer_pilot",
                str(int_flag(hook_install_producer)),
            ),
            (
                "fastmem_global_allocator_claim_preflight",
                str(int_flag(global_allocator_claim_preflight)),
            ),
            (
                "fastmem_global_allocator_claim_producer_pilot",
                str(int_flag(global_allocator_claim_producer)),
            ),
            (
                "fastmem_winner_claim_preflight",
                str(int_flag(winner_claim_preflight)),
            ),
            (
                "fastmem_winner_claim_producer_pilot",
                str(int_flag(winner_claim_producer)),
            ),
            ("fastmem_owner_runtime_current_owner_source", "closed"),
        ]
    if not route_family and profile == "owner-runtime":
        route_candidate = "none"
        selected_memop_family = "owner_runtime"
        selected_memop_kinds = "CurrentAllocOwnerId,OwnerEq"
        slice_rows = [
            ("replacement_front_producer_slice_selection_v0", "0"),
            ("replacement_front_next_producer_slice", "owner_runtime_producer_pilot"),
            ("replacement_front_selected_memop_family", "owner_runtime"),
            ("replacement_front_selected_memop_kinds", "CurrentAllocOwnerId,OwnerEq"),
            ("replacement_front_deferred_memop_family", "remote_free"),
            ("replacement_front_deferred_memop_kinds", "AtomicRemoteHead"),
            ("mir_fmem_008b_layout_table_producer_pilot", "0"),
            ("fastmem_owner_runtime_producer_pilot", "1"),
            ("fastmem_local_free_producer_pilot", "0"),
            ("fastmem_atomic_remote_head_cas_preflight", "0"),
            ("fastmem_atomic_remote_head_cas_producer_pilot", "0"),
            ("fastmem_atomic_remote_head_retry_preflight", "0"),
            ("fastmem_atomic_remote_head_retry_producer_pilot", "0"),
            ("fastmem_atomic_remote_head_drain_preflight", "0"),
            ("fastmem_atomic_remote_head_drain_exchange_selection", "0"),
            ("fastmem_atomic_remote_head_drain_exchange_producer_pilot", "0"),
            ("fastmem_atomic_remote_head_drain_to_local_route_selection", "0"),
            ("fastmem_atomic_remote_head_drain_to_local_route_producer_pilot", "0"),
            ("fastmem_atomic_remote_head_drain_local_list_mutation_preflight", "0"),
            ("fastmem_atomic_remote_head_drain_local_list_mutation_proof", "0"),
            (
                "fastmem_atomic_remote_head_drain_local_list_mutation_vocabulary_preflight",
                "0",
            ),
            (
                "fastmem_owner_runtime_current_owner_source",
                "llvm_producer_intrinsic",
            ),
        ]
    elif not route_family and profile == "local-free":
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
        selected_memop_family = "local_free"
        selected_memop_kinds = (
            ",".join(selected_local_free_kinds) if selected_local_free_kinds else "none"
        )
        slice_rows = [
            ("replacement_front_producer_slice_selection_v0", "0"),
            ("replacement_front_next_producer_slice", "local_free_producer_pilot"),
            ("replacement_front_selected_memop_family", "local_free"),
            (
                "replacement_front_selected_memop_kinds",
                ",".join(selected_local_free_kinds) if selected_local_free_kinds else "none",
            ),
            ("replacement_front_deferred_memop_family", "remote_free"),
            (
                "replacement_front_deferred_memop_kinds",
                ",".join(deferred_local_free_kinds),
            ),
            ("mir_fmem_008b_layout_table_producer_pilot", "0"),
            ("fastmem_owner_runtime_producer_pilot", "0"),
            ("fastmem_local_free_producer_pilot", "1"),
            ("fastmem_atomic_remote_head_cas_preflight", "0"),
            ("fastmem_atomic_remote_head_cas_producer_pilot", "0"),
            ("fastmem_atomic_remote_head_retry_preflight", "0"),
            ("fastmem_atomic_remote_head_retry_producer_pilot", "0"),
            ("fastmem_atomic_remote_head_drain_preflight", "0"),
            ("fastmem_atomic_remote_head_drain_exchange_selection", "0"),
            ("fastmem_atomic_remote_head_drain_exchange_producer_pilot", "0"),
            ("fastmem_atomic_remote_head_drain_to_local_route_selection", "0"),
            ("fastmem_atomic_remote_head_drain_to_local_route_producer_pilot", "0"),
            ("fastmem_atomic_remote_head_drain_local_list_mutation_preflight", "0"),
            ("fastmem_atomic_remote_head_drain_local_list_mutation_proof", "0"),
            (
                "fastmem_atomic_remote_head_drain_local_list_mutation_vocabulary_preflight",
                "0",
            ),
            ("fastmem_owner_runtime_current_owner_source", "llvm_producer_intrinsic"),
        ]
    elif not route_family:
        route_candidate = "none"
        free_route_candidate = "none"
        selected_memop_family = "layout_table"
        selected_memop_kinds = "TableIndex,FieldLoad,FieldStore"
        slice_rows = [
            ("replacement_front_producer_slice_selection_v0", "1"),
            ("replacement_front_next_producer_slice", "layout_table_producer_pilot"),
            ("replacement_front_selected_memop_family", "layout_table"),
            ("replacement_front_selected_memop_kinds", "TableIndex,FieldLoad,FieldStore"),
            ("replacement_front_deferred_memop_family", "owner_runtime"),
            ("replacement_front_deferred_memop_kinds", "CurrentAllocOwnerId,OwnerEq"),
            ("mir_fmem_008b_layout_table_producer_pilot", "1"),
            ("fastmem_owner_runtime_producer_pilot", "0"),
            ("fastmem_local_free_producer_pilot", "0"),
            ("fastmem_atomic_remote_head_cas_preflight", "0"),
            ("fastmem_atomic_remote_head_cas_producer_pilot", "0"),
            ("fastmem_atomic_remote_head_retry_preflight", "0"),
            ("fastmem_atomic_remote_head_retry_producer_pilot", "0"),
            ("fastmem_atomic_remote_head_drain_preflight", "0"),
            ("fastmem_atomic_remote_head_drain_exchange_selection", "0"),
            ("fastmem_atomic_remote_head_drain_exchange_producer_pilot", "0"),
            ("fastmem_atomic_remote_head_drain_to_local_route_selection", "0"),
            ("fastmem_atomic_remote_head_drain_to_local_route_producer_pilot", "0"),
            ("fastmem_atomic_remote_head_drain_local_list_mutation_preflight", "0"),
            ("fastmem_atomic_remote_head_drain_local_list_mutation_proof", "0"),
            (
                "fastmem_atomic_remote_head_drain_local_list_mutation_vocabulary_preflight",
                "0",
            ),
            ("fastmem_owner_runtime_current_owner_source", "closed"),
        ]


    state.update(locals())
    return state
