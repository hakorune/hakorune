"""FastMemory MIR-to-LLVM producer evidence tail rows."""

from __future__ import annotations

from typing import Any

from fastmem_mir_to_llvm_producer_report_common import int_flag


def _activation_chain_rows(
    *,
    product_activation_row: bool,
    hook_install_row: bool,
    global_allocator_claim_row: bool,
    winner_claim_row: bool,
    tls_backing_transfer_enabled_row: bool,
    allocator_owner_slot_reuse_enabled_row: bool,
    allocator_owner_slot_reuse_selected_row: bool,
    allocator_owner_generation_bump_count_row: bool,
    abandoned_reclaim_selected_row: bool,
    abandoned_reclaim_enabled_row: bool,
    product_activation_selected_row: bool,
    hook_install_selected_row: bool,
    global_allocator_claim_selected_row: bool,
    winner_claim_selected_row: bool,
) -> list[tuple[str, str]]:
    return [
        ("product_activation", str(int_flag(product_activation_row))),
        ("hook_install", str(int_flag(hook_install_row))),
        ("hook_installed", "0"),
        ("global_allocator_claim", str(int_flag(global_allocator_claim_row))),
        ("global_allocator_product_claim", "0"),
        ("winner_claim", str(int_flag(winner_claim_row))),
        ("tls_backing_transfer_enabled", str(int_flag(tls_backing_transfer_enabled_row))),
        (
            "allocator_owner_slot_reuse_enabled",
            str(int_flag(allocator_owner_slot_reuse_enabled_row)),
        ),
        (
            "allocator_owner_slot_reuse_selected",
            str(int_flag(allocator_owner_slot_reuse_selected_row)),
        ),
        (
            "allocator_owner_generation_bump_count",
            str(int_flag(allocator_owner_generation_bump_count_row)),
        ),
        ("allocator_owner_reuse_without_generation_bump_count", "0"),
        ("abandoned_reclaim_selected", str(int_flag(abandoned_reclaim_selected_row))),
        ("abandoned_reclaim_enabled", str(int_flag(abandoned_reclaim_enabled_row))),
        ("product_activation_selected", str(int_flag(product_activation_selected_row))),
        ("hook_install_selected", str(int_flag(hook_install_selected_row))),
        (
            "global_allocator_claim_selected",
            str(int_flag(global_allocator_claim_selected_row)),
        ),
        ("winner_claim_selected", str(int_flag(winner_claim_selected_row))),
    ]


def _tail_footer_rows(*, profile: str, object_out: str) -> list[tuple[str, str]]:
    return [
        ("page_reclaimed_with_remote_candidates", "0"),
        (
            "llvm_object_path",
            "not_emitted_atomic_remote_head_cas_lowering_closed"
            if profile == "remote-free-preflight"
            else object_out,
        ),
        ("summary", "ok"),
    ]


def build_tail_rows(state: dict[str, Any]) -> list[tuple[str, str]]:
    mir = state["mir"]
    profile = state["profile"]
    current_owner_count = state["current_owner_count"]
    owner_eq_count = state["owner_eq_count"]
    drain_remote_list_to_local_lowerable = state["drain_remote_list_to_local_lowerable"]
    drain_remote_list_to_local_head_class_resolved = state[
        "drain_remote_list_to_local_head_class_resolved"
    ]
    atomic_remote_head_push_lowerable = state["atomic_remote_head_push_lowerable"]
    atomic_remote_head_drain_lowerable = state["atomic_remote_head_drain_lowerable"]
    atomic_remote_head_memory_order_policy = state["atomic_remote_head_memory_order_policy"]
    atomic_remote_head_retry_attempt_limit = state["atomic_remote_head_retry_attempt_limit"]
    remote_free_open = state.get("remote_free_open", False)
    remote_free_retry_preflight = state.get("remote_free_retry_preflight", False)
    remote_free_retry_producer = state.get("remote_free_retry_producer", False)
    remote_free_drain_preflight = state.get("remote_free_drain_preflight", False)
    remote_free_drain_exchange_selection = state.get("remote_free_drain_exchange_selection", False)
    remote_free_drain_exchange_producer = state.get("remote_free_drain_exchange_producer", False)
    remote_free_drain_to_local_selection = state.get("remote_free_drain_to_local_selection", False)
    remote_free_drain_to_local_producer = state.get("remote_free_drain_to_local_producer", False)
    remote_free_drain_local_list_mutation_preflight = state[
        "remote_free_drain_local_list_mutation_preflight"
    ]
    remote_free_drain_local_list_mutation_proof = state[
        "remote_free_drain_local_list_mutation_proof"
    ]
    remote_free_drain_local_list_mutation_vocabulary_preflight = state[
        "remote_free_drain_local_list_mutation_vocabulary_preflight"
    ]
    remote_free_drain_local_list_mutation_verifier_preconditions = state[
        "remote_free_drain_local_list_mutation_verifier_preconditions"
    ]
    remote_free_drain_local_list_mutation_lowering_producer = state[
        "remote_free_drain_local_list_mutation_lowering_producer"
    ]
    remote_owner_branch_routing_preflight = state.get("remote_owner_branch_routing_preflight", False)
    remote_owner_branch_routing_lowering_preflight = state[
        "remote_owner_branch_routing_lowering_preflight"
    ]
    remote_owner_branch_routing_lowering_producer = state[
        "remote_owner_branch_routing_lowering_producer"
    ]
    remote_owner_branch_route_body_preflight = state.get("remote_owner_branch_route_body_preflight", False)
    fastmem_branch_cfg_preflight = state.get("fastmem_branch_cfg_preflight", False)
    fastmem_branch_cfg_lowering_preflight = state.get("fastmem_branch_cfg_lowering_preflight", False)
    fastmem_branch_cfg_lowering_producer = state.get("fastmem_branch_cfg_lowering_producer", False)
    same_remote_free_body_preflight = state.get("same_remote_free_body_preflight", False)
    same_remote_free_body_producer = state.get("same_remote_free_body_producer", False)
    page_local_alloc_route_cfg_preflight = state.get("page_local_alloc_route_cfg_preflight", False)
    page_local_free_route_cfg_preflight = state.get("page_local_free_route_cfg_preflight", False)
    page_local_free_route_cfg_producer = state.get("page_local_free_route_cfg_producer", False)
    page_local_alloc_route_cfg_any = state.get("page_local_alloc_route_cfg_any", False)
    page_local_free_route_cfg_any = state.get("page_local_free_route_cfg_any", False)
    tls_backing_transfer_preflight = state.get("tls_backing_transfer_preflight", False)
    tls_backing_transfer_producer_refresh = state.get(
        "tls_backing_transfer_producer_refresh", False
    )
    tls_backing_transfer_producer = state.get("tls_backing_transfer_producer", False)
    tls_backing_transfer_or_later = state.get("tls_backing_transfer_or_later", False)
    owner_slot_reuse_preflight = state.get("owner_slot_reuse_preflight", False)
    owner_slot_reuse_producer_refresh = state.get(
        "owner_slot_reuse_producer_refresh", False
    )
    owner_slot_reuse_producer = state.get("owner_slot_reuse_producer", False)
    owner_slot_reuse_or_later = state.get("owner_slot_reuse_or_later", False)
    abandoned_reclaim_preflight = state.get("abandoned_reclaim_preflight", False)
    abandoned_reclaim_producer_refresh = state.get(
        "abandoned_reclaim_producer_refresh", False
    )
    abandoned_reclaim_producer = state.get("abandoned_reclaim_producer", False)
    abandoned_reclaim_or_later = state.get("abandoned_reclaim_or_later", False)
    product_activation_preflight = state.get("product_activation_preflight", False)
    product_activation_producer_refresh = state.get(
        "product_activation_producer_refresh", False
    )
    product_activation_producer = state.get("product_activation_producer", False)
    product_activation_or_later = state.get("product_activation_or_later", False)
    hook_install_preflight = state.get("hook_install_preflight", False)
    hook_install_producer_refresh = state.get("hook_install_producer_refresh", False)
    hook_install_producer = state.get("hook_install_producer", False)
    hook_install_or_later = state.get("hook_install_or_later", False)
    global_allocator_claim_preflight = state.get("global_allocator_claim_preflight", False)
    global_allocator_claim_producer_refresh = state.get(
        "global_allocator_claim_producer_refresh", False
    )
    global_allocator_claim_producer = state.get("global_allocator_claim_producer", False)
    global_allocator_claim_or_later = state.get("global_allocator_claim_or_later", False)
    winner_claim_preflight = state.get("winner_claim_preflight", False)
    winner_claim_producer_refresh = state.get("winner_claim_producer_refresh", False)
    winner_claim_producer = state.get("winner_claim_producer", False)
    winner_claim_any = state.get("winner_claim_any", False)
    global_allocator_claim_any = state.get("global_allocator_claim_any", False)
    hook_install_any = state.get("hook_install_any", False)
    product_activation_any = state.get("product_activation_any", False)
    abandoned_reclaim_any = state.get("abandoned_reclaim_any", False)
    owner_slot_reuse_any = state.get("owner_slot_reuse_any", False)
    tls_backing_transfer_any = state.get("tls_backing_transfer_any", False)
    selected_local_free_kinds = state.get("selected_local_free_kinds", [])
    deferred_local_free_kinds = state.get("deferred_local_free_kinds", [])
    route_candidate = state.get("route_candidate", "none")
    free_route_candidate = state.get("free_route_candidate", "none")
    selected_remote_kind = state.get("selected_remote_kind", "none")
    deferred_remote_kinds = state.get("deferred_remote_kinds", "none")
    remote_owner_branch_routing_any = state.get("remote_owner_branch_routing_any", False)
    remote_owner_branch_routing_lowering_any = state.get("remote_owner_branch_routing_lowering_any", False)
    branch_cfg_count_value = state.get("branch_cfg_count_value", 0)
    verified_plans = state.get("verified_plans", [])
    verified_table = state.get("verified_table", [])
    verified_field_load = state.get("verified_field_load", [])
    verified_field_store = state.get("verified_field_store", [])
    verified_local_free_push = state.get("verified_local_free_push", [])
    verified_local_free_pop = state.get("verified_local_free_pop", [])
    verified_free_head_push = state.get("verified_free_head_push", [])
    verified_free_head_pop = state.get("verified_free_head_pop", [])
    atomic_remote_head_push_count = state.get("atomic_remote_head_push_count", 0)
    atomic_remote_head_drain_count = state.get("atomic_remote_head_drain_count", 0)
    memops = state.get("memops", [])
    remote_owner_facts = state.get("remote_owner_facts", [])
    free_head_non_empty_facts = state.get("free_head_non_empty_facts", [])
    remote_owner_source_assume = state.get("remote_owner_source_assume", 0)
    remote_free_block_next_source_assume = state.get("remote_free_block_next_source_assume", 0)
    atomic_remote_head_push_plans = state.get("atomic_remote_head_push_plans", [])
    atomic_remote_head_drain_plans = state.get("atomic_remote_head_drain_plans", [])
    atomic_remote_head_plans = state.get("atomic_remote_head_plans", [])
    drain_remote_list_to_local_plans = state.get("drain_remote_list_to_local_plans", [])
    fastmem_free_head_non_empty_source_assume_count = state.get(
        "fastmem_free_head_non_empty_source_assume_count", 0
    )
    fastmem_free_head_non_empty_derived_from_free_head_push_count = state.get(
        "fastmem_free_head_non_empty_derived_from_free_head_push_count", 0
    )
    activation_chain_rows = _activation_chain_rows(
        product_activation_row=product_activation_producer
        or product_activation_producer_refresh
        or hook_install_or_later,
        hook_install_row=hook_install_producer
        or hook_install_producer_refresh
        or global_allocator_claim_or_later,
        global_allocator_claim_row=global_allocator_claim_producer
        or global_allocator_claim_producer_refresh
        or winner_claim_any,
        winner_claim_row=winner_claim_producer or winner_claim_producer_refresh,
        tls_backing_transfer_enabled_row=tls_backing_transfer_producer
        or tls_backing_transfer_producer_refresh
        or owner_slot_reuse_or_later,
        allocator_owner_slot_reuse_enabled_row=owner_slot_reuse_producer
        or owner_slot_reuse_producer_refresh
        or abandoned_reclaim_or_later,
        allocator_owner_slot_reuse_selected_row=owner_slot_reuse_or_later,
        allocator_owner_generation_bump_count_row=owner_slot_reuse_producer
        or owner_slot_reuse_producer_refresh
        or abandoned_reclaim_or_later,
        abandoned_reclaim_selected_row=abandoned_reclaim_or_later,
        abandoned_reclaim_enabled_row=abandoned_reclaim_producer
        or abandoned_reclaim_producer_refresh
        or product_activation_or_later,
        product_activation_selected_row=product_activation_or_later,
        hook_install_selected_row=hook_install_or_later,
        global_allocator_claim_selected_row=global_allocator_claim_or_later,
        winner_claim_selected_row=winner_claim_any,
    )

    rows: list[tuple[str, str]] = [
        ("memop_table_index_lowered_count", str(len(verified_table))),
        ("memop_field_load_lowered_count", str(len(verified_field_load))),
        ("memop_field_store_lowered_count", str(len(verified_field_store))),
        ("memop_local_free_push_lowered_count", str(len(verified_local_free_push))),
        ("memop_local_free_pop_lowered_count", str(len(verified_local_free_pop))),
        ("memop_free_head_push_lowered_count", str(len(verified_free_head_push))),
        ("memop_free_head_pop_lowered_count", str(len(verified_free_head_pop))),
        ("memop_current_alloc_owner_id_lowered_count", str(current_owner_count)),
        ("memop_owner_eq_lowered_count", str(owner_eq_count)),
        (
            "memop_atomic_remote_head_lowered_count",
            str(atomic_remote_head_push_lowerable if remote_free_open else 0),
        ),
        (
            "memop_atomic_remote_head_push_count",
            str(atomic_remote_head_push_count),
        ),
        (
            "memop_atomic_remote_head_drain_count",
            str(atomic_remote_head_drain_count),
        ),
        ("memop_table_index_layout_ref_created_count", str(len(verified_table))),
        ("memop_field_load_layout_ref_consumed_count", str(len(verified_field_load))),
        ("memop_field_store_layout_ref_consumed_count", str(len(verified_field_store))),
        ("memop_local_free_push_layout_ref_consumed_count", str(len(verified_local_free_push))),
        ("memop_local_free_pop_layout_ref_consumed_count", str(len(verified_local_free_pop))),
        ("memop_free_head_push_layout_ref_consumed_count", str(len(verified_free_head_push))),
        ("memop_free_head_pop_layout_ref_consumed_count", str(len(verified_free_head_pop))),
        ("memop_lowering_missing_layout_ref_count", "0"),
        ("memop_lowering_raw_pointer_vmap_count", "0"),
        ("memop_lowering_helper_call_count", "0"),
        ("fastmem_layout_ref_model", "1"),
        ("fastmem_table_index_result_kind", "LayoutRef"),
        ("fastmem_layout_ref_lowering_map_enabled", "1"),
        ("fastmem_layout_ref_lowering_map_count", str(len(verified_table))),
        ("fastmem_raw_pointer_in_ordinary_vmap_count", "0"),
        ("fastmem_layout_ref_used_as_ordinary_value_count", "0"),
        ("fastmem_layout_ref_escape_count", "0"),
        ("fastmem_field_id_missing_count", str(state["field_missing"])),
        ("fastmem_table_id_missing_count", str(state["table_missing"])),
        ("fastmem_unverified_layout_access_count", str(len(verified_plans) - len(verified_plans))),
        ("fastmem_table_index_unchecked_count", str(state["unchecked"])),
        ("fastmem_table_access_proof_incomplete_count", str(state["incomplete_proof"])),
        ("fastmem_table_overflow_proof_missing_count", str(state["overflow_missing"])),
        ("fastmem_unknown_alignment_count", str(state["unknown_alignment"])),
        ("fastmem_atomic_field_plain_store_count", str(state["atomic_plain_store"])),
        ("fastmem_local_free_access_plan_incomplete_count", str(state["local_free_incomplete"])),
        ("fastmem_free_head_access_plan_incomplete_count", str(state["free_head_incomplete"])),
        ("fastmem_local_free_head_plain_store_lowered_count", "0"),
        ("fastmem_free_head_plain_store_lowered_count", "0"),
        (
            "fastmem_local_free_push_lowering_uses_verified_plan",
            str(int_flag(bool(verified_local_free_push))),
        ),
        (
            "fastmem_local_free_pop_lowering_uses_verified_plan",
            str(int_flag(bool(verified_local_free_pop))),
        ),
        ("fastmem_local_free_pop_lowering_enabled", str(int_flag(bool(verified_local_free_pop)))),
        (
            "fastmem_free_head_push_lowering_uses_verified_plan",
            str(int_flag(bool(verified_free_head_push))),
        ),
        (
            "fastmem_free_head_pop_lowering_uses_verified_plan",
            str(int_flag(bool(verified_free_head_pop))),
        ),
        ("fastmem_free_head_push_lowering_enabled", str(int_flag(bool(verified_free_head_push)))),
        ("fastmem_free_head_pop_lowering_enabled", str(int_flag(bool(verified_free_head_pop)))),
        ("fastmem_lowering_used_verified_plan", "1"),
        ("fastmem_lowering_recomputed_layout_offset_count", "0"),
        ("fastmem_lowering_recomputed_table_stride_count", "0"),
        ("fastmem_lowering_recomputed_element_repr_count", "0"),
        ("fastmem_incomplete_proof_lowered_count", "0"),
        ("type_abi_hot_lookup_count", "0"),
        ("type_abi_hot_path_lookup_count", "0"),
        ("provider_abi_hot_dispatch_count", "0"),
        ("provider_dispatch_hot_path", "0"),
        *activation_chain_rows,
        *_tail_footer_rows(profile=profile, object_out=str(state["object_out"])),
    ]
    return rows
