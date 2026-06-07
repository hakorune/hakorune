"""FastMemory MIR-to-LLVM producer report route rows."""

from __future__ import annotations

from typing import Any

from fastmem_mir_to_llvm_producer_report_common import (
    int_flag,
    page_local_alloc_route_candidate,
    page_local_free_route_candidate,
)
from fastmem_route_profiles import (
    abandoned_reclaim_preflight_profile,
    abandoned_reclaim_producer_profile,
    FASTMEM_REMOTE_FREE_ROUTE_PROFILE_NAMES,
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
    profile_in,
    REFRESH_PROFILE_SPECS,
    refresh_profile_spec,
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
    if profile_in(profile, FASTMEM_REMOTE_FREE_ROUTE_PROFILE_NAMES):
        remote_free_open = True
        route_candidate = "none"
        if profile == "remote-free-preflight":
            next_slice = "atomic_remote_head_cas_lowering_preflight"
            deferred_remote_kinds = (
                "AtomicRemoteHeadCasLowering,AtomicRemoteHeadDrain,RemoteOwnerBranchRouting"
            )
        elif remote_free_retry_preflight:
            next_slice = "atomic_remote_head_retry_policy_preflight"
            deferred_remote_kinds = (
                "AtomicRemoteHeadRetryLowering,AtomicRemoteHeadDrain,RemoteOwnerBranchRouting"
            )
        elif remote_free_retry_producer:
            next_slice = "atomic_remote_head_retry_lowering_producer_pilot"
            deferred_remote_kinds = "AtomicRemoteHeadDrain,RemoteOwnerBranchRouting"
        elif remote_free_drain_preflight:
            next_slice = "atomic_remote_head_drain_preflight"
            deferred_remote_kinds = (
                "AtomicRemoteHeadDrainLowering,RemoteOwnerBranchRouting"
            )
        elif remote_free_drain_exchange_selection:
            next_slice = "atomic_remote_head_drain_exchange_lowering_producer_pilot"
            deferred_remote_kinds = (
                "AtomicRemoteHeadDrainLowering,DrainToLocalRoute,RemoteOwnerBranchRouting"
            )
        elif remote_free_drain_exchange_producer:
            next_slice = "atomic_remote_head_drain_to_local_route_selection"
            deferred_remote_kinds = "DrainToLocalRoute,RemoteOwnerBranchRouting"
        elif remote_free_drain_to_local_selection:
            next_slice = "atomic_remote_head_drain_to_local_route_producer_pilot"
            deferred_remote_kinds = "DrainToLocalRouteLowering,RemoteOwnerBranchRouting"
        elif remote_free_drain_to_local_producer:
            next_slice = "atomic_remote_head_drain_local_list_mutation_preflight"
            deferred_remote_kinds = "DrainToLocalMutation,RemoteOwnerBranchRouting"
        elif remote_free_drain_local_list_mutation_preflight:
            next_slice = "atomic_remote_head_drain_local_list_mutation_proof"
            deferred_remote_kinds = "DrainLocalListMutation,RemoteOwnerBranchRouting"
        elif remote_free_drain_local_list_mutation_proof:
            next_slice = "atomic_remote_head_drain_local_list_mutation_vocabulary_preflight"
            deferred_remote_kinds = (
                "DrainLocalListMutationVocabulary,RemoteOwnerBranchRouting"
            )
        elif remote_free_drain_local_list_mutation_vocabulary_preflight:
            next_slice = "atomic_remote_head_drain_local_list_mutation_verifier_preconditions"
            deferred_remote_kinds = (
                "DrainRemoteListToLocalLowering,RemoteOwnerBranchRouting"
            )
        elif remote_free_drain_local_list_mutation_verifier_preconditions:
            next_slice = "atomic_remote_head_drain_local_list_mutation_lowering_producer_pilot"
            deferred_remote_kinds = (
                "DrainRemoteListToLocalLowering,RemoteOwnerBranchRouting"
            )
        elif remote_free_drain_local_list_mutation_lowering_producer:
            next_slice = "remote_owner_branch_routing_preflight"
            deferred_remote_kinds = "RemoteOwnerBranchRouting"
        elif remote_owner_branch_routing_preflight:
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
        slice_rows = [
            ("replacement_front_producer_slice_selection_v0", "0"),
            ("replacement_front_selected_route", selected_route),
            ("replacement_front_next_producer_slice", next_slice),
            (
                "replacement_front_selected_memop_family",
                "winner_claim"
                if winner_claim_any
                else "global_allocator_claim"
                if global_allocator_claim_any
                else "hook_install"
                if hook_install_any
                else "product_activation"
                if product_activation_any
                else "abandoned_reclaim"
                if abandoned_reclaim_any
                else "owner_slot_reuse"
                if owner_slot_reuse_any
                else "tls_backing_transfer"
                if tls_backing_transfer_preflight_refresh
                or tls_backing_transfer_producer_refresh
                or tls_backing_transfer_producer
                else "terminal_ladder_refresh"
                if terminal_ladder_refresh_preflight
                else "page_local_route_body_join"
                if page_local_route_body_join_any
                else "page_local_route_cfg"
                if page_local_free_route_cfg_any or tls_backing_transfer_preflight
                else "page_local_alloc_route_cfg"
                if page_local_alloc_route_cfg_any
                else (
                    "same_remote_free_body"
                    if same_remote_free_body_preflight or same_remote_free_body_producer
                    else (
                        "branch_cfg"
                        if fastmem_branch_cfg_lowering_preflight
                        or fastmem_branch_cfg_lowering_producer
                        else (
                            "remote_free_routing"
                            if remote_owner_branch_routing_any
                            else "remote_free"
                        )
                    )
                ),
            ),
            (
                "replacement_front_selected_memop_kinds",
                "WinnerClaim"
                if winner_claim_any
                else "GlobalAllocatorClaim"
                if global_allocator_claim_any
                else "HookInstall"
                if hook_install_any
                else "ProductActivation"
                if product_activation_any
                else "AbandonedReclaim"
                if abandoned_reclaim_any
                else "OwnerSlotReuse"
                if owner_slot_reuse_any
                else "TlsBackingTransfer"
                if tls_backing_transfer_preflight_refresh
                or tls_backing_transfer_producer_refresh
                or tls_backing_transfer_producer
                else "TerminalLadderRefresh"
                if terminal_ladder_refresh_preflight
                else "PageLocalRouteBodyJoin"
                if page_local_route_body_join_preflight
                else "PageLocalRouteBodyJoinProducer"
                if page_local_route_body_join_producer
                else "PageLocalFreeRouteCfg"
                if page_local_free_route_cfg_any or tls_backing_transfer_preflight
                else "PageLocalAllocRouteCfgProducer"
                if page_local_alloc_route_cfg_producer
                else "PageLocalAllocRouteCfg"
                if page_local_alloc_route_cfg_preflight
                else (
                    "SameRemoteFreeBody"
                    if same_remote_free_body_preflight or same_remote_free_body_producer
                    else (
                        "FastMemBranchCfg"
                        if fastmem_branch_cfg_lowering_preflight
                        or fastmem_branch_cfg_lowering_producer
                        else selected_remote_kind
                    )
                ),
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
    elif profile == "owner-runtime":
        route_candidate = "none"
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
    elif profile == "local-free":
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
    else:
        route_candidate = "none"
        free_route_candidate = "none"
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
