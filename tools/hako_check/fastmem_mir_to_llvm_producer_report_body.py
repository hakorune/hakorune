"""FastMemory MIR-to-LLVM producer report body."""

from __future__ import annotations

from pathlib import Path
from typing import Any

from fastmem_mir_to_llvm_producer_report_common import (
    branch_cfg_count,
    count_memops,
    count_plans,
    fastmem_access_plans,
    fastmem_free_head_non_empty_facts,
    fastmem_memops,
    fastmem_regions,
    function_has_fastmem_region,
    functions,
    int_flag,
    is_verified,
    load_json,
    metadata_facts,
    page_local_alloc_route_candidate,
    page_local_free_route_candidate,
    run_llvm_builder,
    string_value,
)
from fastmem_mir_to_llvm_producer_report_tail_rows import build_tail_rows
from fastmem_mir_to_llvm_producer_report_route_rows import build_route_state
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
)


def _page_local_route_report_rows(
    *,
    profile: str,
    route_candidate: str,
    free_route_candidate: str,
    page_local_alloc_route_cfg_any: bool,
    page_local_route_body_join_any: bool,
    page_local_alloc_route_cfg_producer: bool,
    page_local_free_route_cfg_producer: bool,
    tls_backing_transfer_or_later: bool,
    free_head_non_empty_facts: list[dict[str, Any]],
) -> list[tuple[str, str]]:
    return [
        (
            "page_local_alloc_route_report_v0",
            str(
                int_flag(
                    profile == "local-free"
                    or page_local_alloc_route_cfg_any
                    or page_local_route_body_join_any
                )
            ),
        ),
        ("page_local_alloc_route_candidate", route_candidate),
        (
            "page_local_alloc_route_candidate_count",
            str(int_flag(route_candidate != "none")),
        ),
        ("page_local_alloc_route_branch_claim", "0"),
        (
            "page_local_alloc_route_cfg_lowering_enabled",
            str(
                int_flag(
                    page_local_alloc_route_cfg_producer
                    or page_local_route_body_join_any
                    or tls_backing_transfer_or_later
                )
            ),
        ),
        ("page_local_alloc_route_verified_plan_source", "fastmem_access_plans"),
        (
            "page_local_free_route_report_v0",
            str(
                int_flag(
                    profile == "local-free" or page_local_route_body_join_any
                )
            ),
        ),
        ("page_local_free_route_candidate", free_route_candidate),
        (
            "page_local_free_route_candidate_count",
            str(int_flag(free_route_candidate != "none")),
        ),
        ("page_local_free_route_branch_claim", "0"),
        (
            "page_local_free_route_cfg_lowering_enabled",
            str(
                int_flag(
                    page_local_free_route_cfg_producer
                    or page_local_route_body_join_any
                    or tls_backing_transfer_or_later
                )
            ),
        ),
        ("page_local_free_route_verified_plan_source", "fastmem_access_plans"),
        (
            "fastmem_free_head_non_empty_source_assume_count",
            str(
                sum(
                    1
                    for fact in free_head_non_empty_facts
                    if string_value(fact.get("proof_kind"))
                    == "source_assume_free_head_non_empty"
                )
            ),
        ),
        (
            "fastmem_free_head_non_empty_derived_from_free_head_push_count",
            str(
                sum(
                    1
                    for fact in free_head_non_empty_facts
                    if string_value(fact.get("proof_kind"))
                    == "derived_from_free_head_push"
                )
            ),
        ),
    ]


def _terminal_ladder_refresh_rows(
    *,
    page_local_route_body_join_open: bool,
    terminal_ladder_refresh_selected_any: bool,
    terminal_ladder_refresh_open_any: bool,
) -> list[tuple[str, str]]:
    return [
        (
            "page_local_route_body_join_open",
            str(int_flag(page_local_route_body_join_open)),
        ),
        (
            "terminal_ladder_refresh_selected",
            str(int_flag(terminal_ladder_refresh_selected_any)),
        ),
        (
            "terminal_ladder_refresh_open",
            str(int_flag(terminal_ladder_refresh_open_any)),
        ),
    ]


def _remote_owner_branch_routing_rows(
    *,
    remote_owner_branch_routing_selected_any: bool,
    remote_owner_branch_routing_open_any: bool,
    remote_owner_branch_routing_lowered_count_value: int,
    remote_owner_branch_routing_preflight_requires_branch_cfg_row_value: int,
    remote_owner_branch_route_body_selected_any: bool,
) -> list[tuple[str, str]]:
    return [
        (
            "remote_owner_branch_routing_selected",
            str(int_flag(remote_owner_branch_routing_selected_any)),
        ),
        (
            "remote_owner_branch_routing_lowering_selected",
            str(int_flag(remote_owner_branch_routing_selected_any)),
        ),
        (
            "remote_owner_branch_routing_open",
            str(int_flag(remote_owner_branch_routing_open_any)),
        ),
        (
            "remote_owner_branch_routing_lowered_count",
            str(remote_owner_branch_routing_lowered_count_value),
        ),
        (
            "remote_owner_branch_routing_preflight_requires_branch_cfg_row",
            str(
                int_flag(
                    remote_owner_branch_routing_preflight_requires_branch_cfg_row_value
                )
            ),
        ),
        (
            "remote_owner_branch_route_body_selected",
            str(int_flag(remote_owner_branch_route_body_selected_any)),
        ),
        ("remote_owner_branch_route_body_open", "0"),
    ]


def _branch_cfg_and_same_remote_rows(
    *,
    fastmem_branch_cfg_selected_any: bool,
    fastmem_branch_cfg_open_any: bool,
    fastmem_branch_cfg_closed_guard_any: bool,
    fastmem_branch_cfg_lowered_count_value: int,
    fastmem_branch_cfg_source_guard_value: str,
    same_remote_free_body_selected_any: bool,
    same_remote_free_body_open_any: bool,
    same_remote_free_body_lowered_count_any: bool,
) -> list[tuple[str, str]]:
    return [
        (
            "fastmem_branch_cfg_selected",
            str(int_flag(fastmem_branch_cfg_selected_any)),
        ),
        (
            "fastmem_branch_cfg_open",
            str(int_flag(fastmem_branch_cfg_open_any)),
        ),
        (
            "fastmem_branch_cfg_closed_guard",
            str(int_flag(fastmem_branch_cfg_closed_guard_any)),
        ),
        (
            "fastmem_branch_cfg_lowered_count",
            str(fastmem_branch_cfg_lowered_count_value),
        ),
        (
            "fastmem_branch_cfg_source_guard",
            fastmem_branch_cfg_source_guard_value,
        ),
        (
            "same_remote_free_body_selected",
            str(int_flag(same_remote_free_body_selected_any)),
        ),
        (
            "same_remote_free_body_open",
            str(int_flag(same_remote_free_body_open_any)),
        ),
        (
            "same_remote_free_body_lowered_count",
            str(int_flag(same_remote_free_body_lowered_count_any)),
        ),
    ]


def _atomic_remote_head_retry_rows(
    *,
    atomic_remote_head_cas_lowering_selected: bool,
    remote_free_open: bool,
    atomic_remote_head_push_plans: list[dict[str, Any]],
    atomic_remote_head_push_lowerable: int,
    atomic_remote_head_remote_owner_required: int,
    atomic_remote_head_remote_owner_missing: int,
    atomic_remote_head_block_next_required: int,
    atomic_remote_head_block_next_missing: int,
    atomic_remote_head_access_resolved: int,
    atomic_remote_head_memory_order_policy: str,
    remote_owner_facts: list[dict[str, Any]],
    remote_owner_source_assume: int,
    remote_free_block_next_source_assume: int,
    atomic_remote_head_retry_attempt_limit: str,
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
    remote_owner_branch_routing_any: bool,
) -> list[tuple[str, str]]:
    return [
        (
            "atomic_remote_head_cas_lowering_selected",
            str(int_flag(atomic_remote_head_cas_lowering_selected)),
        ),
        ("atomic_remote_head_cas_lowering_open", str(int_flag(remote_free_open))),
        ("atomic_remote_head_push_plan_count", str(len(atomic_remote_head_push_plans))),
        ("atomic_remote_head_push_lowerable_count", str(atomic_remote_head_push_lowerable)),
        (
            "atomic_remote_head_remote_owner_required",
            str(atomic_remote_head_remote_owner_required),
        ),
        (
            "atomic_remote_head_remote_owner_missing_count",
            str(atomic_remote_head_remote_owner_missing),
        ),
        (
            "atomic_remote_head_block_next_required",
            str(atomic_remote_head_block_next_required),
        ),
        (
            "atomic_remote_head_block_next_missing_count",
            str(atomic_remote_head_block_next_missing),
        ),
        (
            "atomic_remote_head_access_resolved_count",
            str(atomic_remote_head_access_resolved),
        ),
        ("atomic_remote_head_memory_order_policy", atomic_remote_head_memory_order_policy),
        ("fastmem_remote_owner_fact_count", str(len(remote_owner_facts))),
        (
            "fastmem_remote_owner_source_assume_count",
            str(remote_owner_source_assume),
        ),
        (
            "fastmem_remote_free_block_next_source_assume_count",
            str(remote_free_block_next_source_assume),
        ),
        (
            "atomic_remote_head_retry_policy_selected",
            str(int_flag(remote_free_retry_preflight or remote_free_retry_producer)),
        ),
        (
            "atomic_remote_head_retry_policy_open",
            str(
                int_flag(
                    remote_free_retry_producer
                    or remote_free_drain_preflight
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
                )
            ),
        ),
        (
            "atomic_remote_head_retry_attempt_limit",
            atomic_remote_head_retry_attempt_limit
            if (
                remote_free_retry_preflight
                or remote_free_retry_producer
                or remote_free_drain_preflight
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
            )
            else "0",
        ),
        (
            "atomic_remote_head_retry_lowered_count",
            str(
                atomic_remote_head_push_lowerable
                if (
                    remote_free_retry_producer
                    or remote_free_drain_preflight
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
                )
                else 0
            ),
        ),
    ]


def _atomic_remote_head_rows(state: dict[str, Any]) -> list[tuple[str, str]]:
    remote_free_retry_preflight = state["remote_free_retry_preflight"]
    remote_free_retry_producer = state["remote_free_retry_producer"]
    remote_free_drain_preflight = state["remote_free_drain_preflight"]
    remote_free_drain_exchange_selection = state["remote_free_drain_exchange_selection"]
    remote_free_drain_exchange_producer = state["remote_free_drain_exchange_producer"]
    remote_free_drain_to_local_selection = state["remote_free_drain_to_local_selection"]
    remote_free_drain_to_local_producer = state["remote_free_drain_to_local_producer"]
    remote_free_drain_local_list_mutation_preflight = state["remote_free_drain_local_list_mutation_preflight"]
    remote_free_drain_local_list_mutation_proof = state["remote_free_drain_local_list_mutation_proof"]
    remote_free_drain_local_list_mutation_vocabulary_preflight = state["remote_free_drain_local_list_mutation_vocabulary_preflight"]
    remote_free_drain_local_list_mutation_verifier_preconditions = state["remote_free_drain_local_list_mutation_verifier_preconditions"]
    remote_free_drain_local_list_mutation_lowering_producer = state["remote_free_drain_local_list_mutation_lowering_producer"]
    remote_owner_branch_routing_any = state["remote_owner_branch_routing_any"]
    atomic_remote_head_cas_lowering_selected = state["profile"] in {
        "remote-free-preflight",
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
    }
    remote_free_open = state["remote_free_open"]
    atomic_remote_head_push_plans = state["atomic_remote_head_push_plans"]
    atomic_remote_head_push_lowerable = state["atomic_remote_head_push_lowerable"]
    atomic_remote_head_remote_owner_required = state["atomic_remote_head_remote_owner_required"]
    atomic_remote_head_remote_owner_missing = state["atomic_remote_head_remote_owner_missing"]
    atomic_remote_head_block_next_required = state["atomic_remote_head_block_next_required"]
    atomic_remote_head_block_next_missing = state["atomic_remote_head_block_next_missing"]
    atomic_remote_head_access_resolved = state["atomic_remote_head_access_resolved"]
    atomic_remote_head_memory_order_policy = state["atomic_remote_head_memory_order_policy"]
    remote_owner_facts = state["remote_owner_facts"]
    remote_owner_source_assume = state["remote_owner_source_assume"]
    remote_free_block_next_source_assume = state["remote_free_block_next_source_assume"]
    atomic_remote_head_retry_attempt_limit = state["atomic_remote_head_retry_attempt_limit"]
    atomic_remote_head_drain_plans = state["atomic_remote_head_drain_plans"]
    atomic_remote_head_drain_lowerable = state["atomic_remote_head_drain_lowerable"]
    drain_remote_list_to_local_count = state["drain_remote_list_to_local_count"]
    drain_remote_list_to_local_plans = state["drain_remote_list_to_local_plans"]
    drain_remote_list_to_local_token_provenance_valid = state["drain_remote_list_to_local_token_provenance_valid"]
    drain_remote_list_to_local_page_operand_valid = state["drain_remote_list_to_local_page_operand_valid"]
    drain_remote_list_to_local_head_class_resolved = state["drain_remote_list_to_local_head_class_resolved"]
    atomic_remote_head_drain_lowered_count = state["atomic_remote_head_drain_lowered_count"]
    atomic_remote_head_drain_open = state["atomic_remote_head_drain_open"]
    atomic_remote_head_drain_exchange_selected = state["atomic_remote_head_drain_exchange_selected"]
    atomic_remote_head_drain_to_local_route_selected = state["atomic_remote_head_drain_to_local_route_selected"]
    atomic_remote_head_drain_to_local_route_producer_pilot = state["atomic_remote_head_drain_to_local_route_producer_pilot"]
    atomic_remote_head_drain_to_local_route_open = state["atomic_remote_head_drain_to_local_route_open"]
    atomic_remote_head_drain_local_list_mutation_selected = state["atomic_remote_head_drain_local_list_mutation_selected"]
    atomic_remote_head_drain_local_list_mutation_open = state["atomic_remote_head_drain_local_list_mutation_open"]
    atomic_remote_head_drain_local_list_token_escape_count = state["atomic_remote_head_drain_local_list_token_escape_count"]
    atomic_remote_head_drain_local_list_head_class_resolved = state["atomic_remote_head_drain_local_list_head_class_resolved"]
    atomic_remote_head_drain_local_list_head_class = state["atomic_remote_head_drain_local_list_head_class"]
    atomic_remote_head_drain_local_list_publication_order = state["atomic_remote_head_drain_local_list_publication_order"]
    drain_remote_list_to_local_lowerable = state["drain_remote_list_to_local_lowerable"]
    atomic_remote_head_drain_local_list_mutation_lowerable_count = state["atomic_remote_head_drain_local_list_mutation_lowerable_count"]
    atomic_remote_head_drain_local_list_mutation_lowered_count = state["atomic_remote_head_drain_local_list_mutation_lowered_count"]

    return [
        *_atomic_remote_head_retry_rows(
            atomic_remote_head_cas_lowering_selected=atomic_remote_head_cas_lowering_selected,
            remote_free_open=remote_free_open,
            atomic_remote_head_push_plans=atomic_remote_head_push_plans,
            atomic_remote_head_push_lowerable=atomic_remote_head_push_lowerable,
            atomic_remote_head_remote_owner_required=atomic_remote_head_remote_owner_required,
            atomic_remote_head_remote_owner_missing=atomic_remote_head_remote_owner_missing,
            atomic_remote_head_block_next_required=atomic_remote_head_block_next_required,
            atomic_remote_head_block_next_missing=atomic_remote_head_block_next_missing,
            atomic_remote_head_access_resolved=atomic_remote_head_access_resolved,
            atomic_remote_head_memory_order_policy=atomic_remote_head_memory_order_policy,
            remote_owner_facts=remote_owner_facts,
            remote_owner_source_assume=remote_owner_source_assume,
            remote_free_block_next_source_assume=remote_free_block_next_source_assume,
            atomic_remote_head_retry_attempt_limit=atomic_remote_head_retry_attempt_limit,
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
            remote_owner_branch_routing_any=remote_owner_branch_routing_any,
        ),
        ("atomic_remote_head_drain_selected", str(int_flag(remote_free_drain_preflight or remote_free_drain_exchange_selection or remote_free_drain_exchange_producer or remote_free_drain_to_local_selection or remote_free_drain_to_local_producer or remote_free_drain_local_list_mutation_preflight or remote_free_drain_local_list_mutation_proof or remote_free_drain_local_list_mutation_vocabulary_preflight or remote_free_drain_local_list_mutation_verifier_preconditions or remote_free_drain_local_list_mutation_lowering_producer or remote_owner_branch_routing_any))),
        ("atomic_remote_head_drain_exchange_selected", str(int_flag(remote_free_drain_exchange_selection or remote_free_drain_exchange_producer or remote_free_drain_to_local_selection or remote_free_drain_to_local_producer or remote_free_drain_local_list_mutation_preflight or remote_free_drain_local_list_mutation_proof or remote_free_drain_local_list_mutation_vocabulary_preflight or remote_free_drain_local_list_mutation_verifier_preconditions or remote_free_drain_local_list_mutation_lowering_producer or remote_owner_branch_routing_any))),
        ("atomic_remote_head_drain_exchange_order", "acquire" if (remote_free_drain_exchange_selection or remote_free_drain_exchange_producer or remote_free_drain_to_local_selection or remote_free_drain_to_local_producer or remote_free_drain_local_list_mutation_preflight or remote_free_drain_local_list_mutation_proof or remote_free_drain_local_list_mutation_vocabulary_preflight or remote_free_drain_local_list_mutation_verifier_preconditions or remote_free_drain_local_list_mutation_lowering_producer or remote_owner_branch_routing_any) else "closed"),
        ("atomic_remote_head_drain_result_kind", "remote_free_list_token" if (remote_free_drain_exchange_selection or remote_free_drain_exchange_producer or remote_free_drain_to_local_selection or remote_free_drain_to_local_producer or remote_free_drain_local_list_mutation_preflight or remote_free_drain_local_list_mutation_proof or remote_free_drain_local_list_mutation_vocabulary_preflight or remote_free_drain_local_list_mutation_verifier_preconditions or remote_free_drain_local_list_mutation_lowering_producer or remote_owner_branch_routing_any) else "closed"),
        ("atomic_remote_head_drain_to_local_route_selected", str(int_flag(remote_free_drain_to_local_selection or remote_free_drain_to_local_producer or remote_free_drain_local_list_mutation_preflight or remote_free_drain_local_list_mutation_proof or remote_free_drain_local_list_mutation_vocabulary_preflight or remote_free_drain_local_list_mutation_verifier_preconditions or remote_free_drain_local_list_mutation_lowering_producer or remote_owner_branch_routing_any))),
        ("atomic_remote_head_drain_to_local_route_producer_pilot", str(int_flag(remote_free_drain_to_local_producer))),
        ("atomic_remote_head_drain_to_local_route_open", str(int_flag(remote_free_drain_to_local_producer or remote_free_drain_local_list_mutation_preflight or remote_free_drain_local_list_mutation_proof or remote_free_drain_local_list_mutation_vocabulary_preflight or remote_free_drain_local_list_mutation_verifier_preconditions))),
        ("atomic_remote_head_drain_local_list_mutation_selected", str(int_flag(remote_free_drain_local_list_mutation_preflight or remote_free_drain_local_list_mutation_proof or remote_free_drain_local_list_mutation_vocabulary_preflight or remote_free_drain_local_list_mutation_verifier_preconditions or remote_free_drain_local_list_mutation_lowering_producer or remote_owner_branch_routing_any))),
        ("atomic_remote_head_drain_local_list_mutation_open", str(int_flag(remote_free_drain_local_list_mutation_lowering_producer or remote_owner_branch_routing_any))),
        ("atomic_remote_head_drain_local_list_token_escape_count", str(atomic_remote_head_drain_local_list_token_escape_count)),
        ("atomic_remote_head_drain_local_list_head_class_resolved", str(int_flag(atomic_remote_head_drain_local_list_head_class_resolved))),
        ("atomic_remote_head_drain_local_list_head_class", atomic_remote_head_drain_local_list_head_class),
        ("atomic_remote_head_drain_local_list_publication_order", atomic_remote_head_drain_local_list_publication_order),
        ("atomic_remote_head_drain_open", str(int_flag(atomic_remote_head_drain_open))),
        ("atomic_remote_head_drain_plan_count", str(len(atomic_remote_head_drain_plans))),
        ("atomic_remote_head_drain_lowerable_count", str(atomic_remote_head_drain_lowerable)),
        ("atomic_remote_head_drain_lowered_count", str(atomic_remote_head_drain_lowered_count)),
        ("fastmem_memop_drain_remote_list_to_local_count", str(drain_remote_list_to_local_count)),
        ("drain_remote_list_to_local_plan_count", str(len(drain_remote_list_to_local_plans))),
        ("drain_remote_list_to_local_token_provenance_valid", str(drain_remote_list_to_local_token_provenance_valid)),
        ("drain_remote_list_to_local_page_operand_valid", str(drain_remote_list_to_local_page_operand_valid)),
        ("drain_remote_list_to_local_head_class_resolved", str(drain_remote_list_to_local_head_class_resolved)),
        ("drain_remote_list_to_local_lowerable_count", str(drain_remote_list_to_local_lowerable)),
        ("atomic_remote_head_drain_local_list_mutation_lowerable_count", str(atomic_remote_head_drain_local_list_mutation_lowerable_count)),
        ("atomic_remote_head_drain_local_list_mutation_lowered_count", str(atomic_remote_head_drain_local_list_mutation_lowered_count)),
    ]


def build_report_rows(mir: dict[str, Any], *, object_out: Path, profile: str) -> list[tuple[str, str]]:
    plans = fastmem_access_plans(mir)
    regions = fastmem_regions(mir)
    memops = fastmem_memops(mir)
    free_head_non_empty_facts = fastmem_free_head_non_empty_facts(mir)
    verified_plans = [plan for plan in plans if is_verified(plan)]
    verified_table = [plan for plan in verified_plans if plan.get("kind") == "table_index"]
    verified_field_load = [plan for plan in verified_plans if plan.get("kind") == "field_load"]
    verified_field_store = [plan for plan in verified_plans if plan.get("kind") == "field_store"]
    verified_field = verified_field_load + verified_field_store
    verified_local_free_push = [
        plan
        for plan in verified_plans
        if plan.get("kind") == "local_free_push" and bool(plan.get("lowerable"))
    ]
    verified_local_free_pop = [
        plan
        for plan in verified_plans
        if plan.get("kind") == "local_free_pop" and bool(plan.get("lowerable"))
    ]
    verified_free_head_pop = [
        plan
        for plan in verified_plans
        if plan.get("kind") == "free_head_pop" and bool(plan.get("lowerable"))
    ]
    verified_free_head_push = [
        plan
        for plan in verified_plans
        if plan.get("kind") == "free_head_push" and bool(plan.get("lowerable"))
    ]
    atomic_remote_head_push_plans = [
        plan for plan in plans if plan.get("kind") == "atomic_remote_head_push"
    ]
    atomic_remote_head_drain_plans = [
        plan for plan in plans if plan.get("kind") == "atomic_remote_head_drain"
    ]
    atomic_remote_head_plans = atomic_remote_head_push_plans + atomic_remote_head_drain_plans
    drain_remote_list_to_local_plans = [
        plan for plan in plans if plan.get("kind") == "drain_remote_list_to_local"
    ]
    remote_owner_facts = metadata_facts(mir, "fastmem_remote_owner_facts")
    block_next_facts = metadata_facts(mir, "fastmem_block_next_facts")

    contract_ids = sorted(
        {
            string_value(region.get("contract"))
            for region in regions
            if string_value(region.get("contract"))
        }
    )
    contract_id = contract_ids[0] if contract_ids else "unknown"

    table_missing = sum(1 for plan in verified_table if not plan.get("table_id"))
    field_missing = sum(1 for plan in verified_field if not plan.get("field_id"))
    unchecked = sum(
        1
        for plan in verified_table
        if not bool(plan.get("bounds_proof_valid"))
    )
    incomplete_proof = sum(
        1
        for plan in verified_table
        if not bool(plan.get("table_length_resolved"))
        or not bool(plan.get("stride_resolved"))
        or not bool(plan.get("field_offset_resolved"))
        or not bool(plan.get("element_layout_verified"))
    )
    overflow_missing = sum(
        1
        for plan in verified_table
        if not bool(plan.get("overflow_proof_valid"))
    )
    unknown_alignment = sum(
        1
        for plan in verified_table + verified_field
        if not bool(plan.get("alignment_valid", True))
        or int(plan.get("alignment") or 0) <= 0
    )
    atomic_plain_store = sum(
        1
        for plan in verified_field_store
        if string_value(plan.get("field_class")) == "atomic_remote_head"
    )
    local_free_verified = verified_local_free_push + verified_local_free_pop
    verified_free_head = verified_free_head_push + verified_free_head_pop
    free_head_incomplete = sum(
        1
        for plan in verified_free_head
        if plan.get("free_head_byte_offset") is None
        or plan.get("free_head_field_size") is None
        or plan.get("free_head_field_type") is None
        or plan.get("free_head_alignment") is None
        or plan.get("block_next_byte_offset") is None
        or plan.get("block_next_field_size") is None
        or plan.get("block_next_field_type") is None
        or plan.get("block_next_alignment") is None
    )
    local_free_incomplete = sum(
        1
        for plan in local_free_verified
        if plan.get("local_free_head_byte_offset") is None
        or plan.get("local_free_head_field_size") is None
        or plan.get("local_free_head_field_type") is None
        or plan.get("local_free_head_alignment") is None
        or plan.get("block_next_byte_offset") is None
        or plan.get("block_next_field_size") is None
        or plan.get("block_next_field_type") is None
        or plan.get("block_next_alignment") is None
    )
    current_owner_count = count_memops(memops, "current_alloc_owner_id")
    owner_eq_count = count_memops(memops, "owner_eq")
    atomic_remote_head_push_count = count_memops(memops, "atomic_remote_head_push")
    atomic_remote_head_drain_count = count_memops(memops, "atomic_remote_head_drain")
    drain_remote_list_to_local_count = count_memops(memops, "drain_remote_list_to_local")
    drain_remote_list_to_local_lowerable = sum(
        1 for plan in drain_remote_list_to_local_plans if bool(plan.get("lowerable"))
    )
    drain_remote_list_to_local_token_provenance_valid = sum(
        1
        for plan in drain_remote_list_to_local_plans
        if bool(plan.get("token_provenance_valid"))
    )
    drain_remote_list_to_local_page_operand_valid = sum(
        1
        for plan in drain_remote_list_to_local_plans
        if bool(plan.get("page_operand_valid"))
    )
    drain_remote_list_to_local_head_class_resolved = sum(
        1
        for plan in drain_remote_list_to_local_plans
        if bool(plan.get("head_class_resolved"))
    )
    atomic_remote_head_push_lowerable = sum(
        1 for plan in atomic_remote_head_push_plans if bool(plan.get("lowerable"))
    )
    atomic_remote_head_drain_lowerable = sum(
        1 for plan in atomic_remote_head_drain_plans if bool(plan.get("lowerable"))
    )
    atomic_remote_head_remote_owner_required = int_flag(
        any(bool(plan.get("remote_owner_required")) for plan in atomic_remote_head_push_plans)
    )
    atomic_remote_head_remote_owner_missing = sum(
        1
        for plan in atomic_remote_head_push_plans
        if bool(plan.get("remote_owner_required"))
        and not bool(plan.get("remote_owner_proof_valid"))
    )
    atomic_remote_head_block_next_required = int_flag(
        any(bool(plan.get("block_next_required")) for plan in atomic_remote_head_push_plans)
    )
    atomic_remote_head_block_next_missing = sum(
        1
        for plan in atomic_remote_head_push_plans
        if bool(plan.get("block_next_required"))
        and not bool(plan.get("block_next_proof_valid"))
    )
    atomic_remote_head_access_resolved = sum(
        1
        for plan in atomic_remote_head_plans
        if plan.get("remote_head_byte_offset") not in (None, "")
        and plan.get("remote_head_field_size") not in (None, "")
        and plan.get("remote_head_field_type") not in (None, "")
        and plan.get("remote_head_alignment") not in (None, "")
    )
    atomic_remote_head_memory_order_policy = ",".join(
        sorted(
            {
                string_value(plan.get("memory_order_policy"), "unknown")
                for plan in atomic_remote_head_plans
            }
        )
    )
    if not atomic_remote_head_memory_order_policy:
        atomic_remote_head_memory_order_policy = "none"
    atomic_remote_head_retry_attempt_limits = sorted(
        {
            string_value(plan.get("retry_attempt_limit"), "0")
            for plan in atomic_remote_head_push_plans
            if string_value(plan.get("retry_attempt_limit"), "0") != "0"
        }
    )
    atomic_remote_head_retry_attempt_limit = (
        atomic_remote_head_retry_attempt_limits[0]
        if len(atomic_remote_head_retry_attempt_limits) == 1
        else "0"
    )
    remote_owner_source_assume = sum(
        1
        for fact in remote_owner_facts
        if string_value(fact.get("proof_kind")) == "source_assume_remote_owner"
        and bool(fact.get("same_owner_rejected"))
    )
    remote_free_block_next_source_assume = sum(
        1
        for fact in block_next_facts
        if string_value(fact.get("proof_kind"))
        == "source_assume_remote_free_block_next"
    )
    selected_local_free_kinds = []
    if verified_local_free_push:
        selected_local_free_kinds.append("LocalFreePush")
    if verified_local_free_pop:
        selected_local_free_kinds.append("LocalFreePop")
    if verified_free_head_push:
        selected_local_free_kinds.append("FreeHeadPush")
    if verified_free_head_pop:
        selected_local_free_kinds.append("FreeHeadPop")
    deferred_local_free_kinds = []
    if not verified_local_free_push:
        deferred_local_free_kinds.append("LocalFreePush")
    if not verified_local_free_pop:
        deferred_local_free_kinds.append("LocalFreePop")
    if not verified_free_head_push:
        deferred_local_free_kinds.append("FreeHeadPush")
    if not verified_free_head_pop:
        deferred_local_free_kinds.append("FreeHeadPop")
    deferred_local_free_kinds.append("AtomicRemoteHead")

    route_candidate = "none"
    free_route_candidate = "none"

    route_state = build_route_state(locals())
    globals().update(route_state)
    route_candidate = route_state.get("route_candidate", route_candidate)
    free_route_candidate = route_state.get("free_route_candidate", free_route_candidate)
    remote_owner_branch_routing_selected_any = remote_owner_branch_routing_any
    remote_owner_branch_routing_open_any = (
        remote_owner_branch_routing_lowering_producer
        or remote_owner_branch_route_body_preflight
        or fastmem_branch_cfg_preflight
        or fastmem_branch_cfg_lowering_preflight
        or fastmem_branch_cfg_lowering_producer
        or same_remote_free_body_preflight
        or same_remote_free_body_producer
        or page_local_free_route_cfg_any
        or tls_backing_transfer_or_later
    )
    remote_owner_branch_routing_lowered_count_value = int_flag(
        (
            remote_owner_branch_routing_lowering_producer
            or remote_owner_branch_route_body_preflight
            or fastmem_branch_cfg_preflight
            or fastmem_branch_cfg_lowering_preflight
            or fastmem_branch_cfg_lowering_producer
            or same_remote_free_body_preflight
            or same_remote_free_body_producer
            or page_local_free_route_cfg_any
            or tls_backing_transfer_or_later
        )
        and current_owner_count > 0
        and owner_eq_count > 0
        and drain_remote_list_to_local_lowerable > 0
    )
    remote_owner_branch_routing_preflight_requires_branch_cfg_row_value = int_flag(
        not (
            remote_owner_branch_routing_lowering_producer
            or remote_owner_branch_route_body_preflight
            or fastmem_branch_cfg_preflight
            or fastmem_branch_cfg_lowering_preflight
            or fastmem_branch_cfg_lowering_producer
            or same_remote_free_body_preflight
            or same_remote_free_body_producer
            or page_local_free_route_cfg_any
            or tls_backing_transfer_or_later
        )
    )
    remote_owner_branch_route_body_selected_any = (
        remote_owner_branch_route_body_preflight
        or fastmem_branch_cfg_preflight
        or fastmem_branch_cfg_lowering_preflight
        or fastmem_branch_cfg_lowering_producer
        or same_remote_free_body_preflight
        or same_remote_free_body_producer
        or page_local_free_route_cfg_any
        or tls_backing_transfer_or_later
    )
    fastmem_branch_cfg_selected_any = (
        fastmem_branch_cfg_preflight
        or fastmem_branch_cfg_lowering_preflight
        or fastmem_branch_cfg_lowering_producer
        or same_remote_free_body_preflight
        or same_remote_free_body_producer
        or page_local_alloc_route_cfg_producer
        or page_local_route_body_join_any
        or page_local_free_route_cfg_any
        or tls_backing_transfer_or_later
    )
    fastmem_branch_cfg_open_any = (
        fastmem_branch_cfg_lowering_producer
        or same_remote_free_body_preflight
        or same_remote_free_body_producer
        or page_local_alloc_route_cfg_producer
        or page_local_route_body_join_any
        or page_local_free_route_cfg_any
        or tls_backing_transfer_or_later
    )
    fastmem_branch_cfg_closed_guard_any = (
        fastmem_branch_cfg_preflight
        or fastmem_branch_cfg_lowering_preflight
    ) and not fastmem_branch_cfg_lowering_producer
    fastmem_branch_cfg_lowered_count_value = (
        branch_cfg_count(mir) if fastmem_branch_cfg_open_any else 0
    )
    fastmem_branch_cfg_source_guard_value = (
        "branch_cfg_open" if fastmem_branch_cfg_open_any else "branch_cfg_closed"
    )
    same_remote_free_body_selected_any = (
        same_remote_free_body_preflight
        or same_remote_free_body_producer
        or page_local_free_route_cfg_any
        or tls_backing_transfer_or_later
    )
    same_remote_free_body_open_any = (
        same_remote_free_body_producer
        or page_local_free_route_cfg_any
        or page_local_route_body_join_any
        or tls_backing_transfer_or_later
    )
    same_remote_free_body_lowered_count_any = (
        same_remote_free_body_open_any
        and branch_cfg_count(mir) > 0
        and current_owner_count > 0
        and owner_eq_count > 0
        and drain_remote_list_to_local_lowerable > 0
        and len(verified_field_store) > 0
    )
    terminal_ladder_refresh_selected_any = (
        terminal_ladder_refresh_preflight
        or tls_backing_transfer_preflight_refresh
        or tls_backing_transfer_producer_refresh
        or owner_slot_reuse_preflight_refresh
        or owner_slot_reuse_producer_refresh
        or abandoned_reclaim_preflight_refresh
        or abandoned_reclaim_producer_refresh
        or product_activation_preflight_refresh
        or product_activation_producer_refresh
        or hook_install_preflight_refresh
        or hook_install_producer_refresh
        or global_allocator_claim_preflight_refresh
        or global_allocator_claim_producer_refresh
        or winner_claim_preflight_refresh
        or winner_claim_producer_refresh
    )
    terminal_ladder_refresh_open_any = (
        tls_backing_transfer_preflight_refresh
        or tls_backing_transfer_producer_refresh
        or owner_slot_reuse_preflight_refresh
        or owner_slot_reuse_producer_refresh
        or abandoned_reclaim_preflight_refresh
        or abandoned_reclaim_producer_refresh
        or product_activation_preflight_refresh
        or product_activation_producer_refresh
        or hook_install_preflight_refresh
        or hook_install_producer_refresh
        or global_allocator_claim_preflight_refresh
        or global_allocator_claim_producer_refresh
        or winner_claim_preflight_refresh
        or winner_claim_producer_refresh
    )
    page_local_route_body_join_open_any = (
        page_local_route_body_join_producer
        or terminal_ladder_refresh_selected_any
    )
    atomic_remote_state = {**locals().copy(), **route_state}
    atomic_remote_state.update(
        {
            "atomic_remote_head_cas_lowering_selected": profile
            in {
                "remote-free-preflight",
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
            },
            "atomic_remote_head_drain_lowered_count": atomic_remote_head_drain_lowerable
            if (
                remote_free_drain_exchange_producer
                or remote_free_drain_to_local_selection
                or remote_free_drain_to_local_producer
                or remote_free_drain_local_list_mutation_preflight
                or remote_free_drain_local_list_mutation_proof
                or remote_free_drain_local_list_mutation_vocabulary_preflight
                or remote_free_drain_local_list_mutation_verifier_preconditions
                or remote_free_drain_local_list_mutation_lowering_producer
                or remote_owner_branch_routing_any
            )
            else 0,
            "atomic_remote_head_drain_open": int_flag(
                remote_free_drain_exchange_producer
                or remote_free_drain_to_local_selection
                or remote_free_drain_to_local_producer
                or remote_free_drain_local_list_mutation_preflight
                or remote_free_drain_local_list_mutation_proof
                or remote_free_drain_local_list_mutation_vocabulary_preflight
                or remote_free_drain_local_list_mutation_verifier_preconditions
                or remote_free_drain_local_list_mutation_lowering_producer
                or remote_owner_branch_routing_any
            ),
            "atomic_remote_head_drain_exchange_selected": int_flag(
                remote_free_drain_exchange_selection
                or remote_free_drain_exchange_producer
                or remote_free_drain_to_local_selection
                or remote_free_drain_to_local_producer
                or remote_free_drain_local_list_mutation_preflight
                or remote_free_drain_local_list_mutation_proof
                or remote_free_drain_local_list_mutation_vocabulary_preflight
                or remote_free_drain_local_list_mutation_verifier_preconditions
                or remote_free_drain_local_list_mutation_lowering_producer
                or remote_owner_branch_routing_any
            ),
            "atomic_remote_head_drain_to_local_route_selected": int_flag(
                remote_free_drain_to_local_selection
                or remote_free_drain_to_local_producer
                or remote_free_drain_local_list_mutation_preflight
                or remote_free_drain_local_list_mutation_proof
                or remote_free_drain_local_list_mutation_vocabulary_preflight
                or remote_free_drain_local_list_mutation_verifier_preconditions
                or remote_free_drain_local_list_mutation_lowering_producer
                or remote_owner_branch_routing_any
            ),
            "atomic_remote_head_drain_to_local_route_producer_pilot": int_flag(
                remote_free_drain_to_local_producer
            ),
            "atomic_remote_head_drain_to_local_route_open": int_flag(
                remote_free_drain_to_local_producer
                or remote_free_drain_local_list_mutation_preflight
                or remote_free_drain_local_list_mutation_proof
                or remote_free_drain_local_list_mutation_vocabulary_preflight
                or remote_free_drain_local_list_mutation_verifier_preconditions
            ),
            "atomic_remote_head_drain_local_list_mutation_selected": int_flag(
                remote_free_drain_local_list_mutation_preflight
                or remote_free_drain_local_list_mutation_proof
                or remote_free_drain_local_list_mutation_vocabulary_preflight
                or remote_free_drain_local_list_mutation_verifier_preconditions
                or remote_free_drain_local_list_mutation_lowering_producer
                or remote_owner_branch_routing_any
            ),
            "atomic_remote_head_drain_local_list_mutation_open": int_flag(
                remote_free_drain_local_list_mutation_lowering_producer
                or remote_owner_branch_routing_any
            ),
            "atomic_remote_head_drain_local_list_token_escape_count": "0",
            "atomic_remote_head_drain_local_list_head_class_resolved": int_flag(
                remote_free_drain_local_list_mutation_proof
                or remote_free_drain_local_list_mutation_vocabulary_preflight
                or (
                    remote_free_drain_local_list_mutation_verifier_preconditions
                    and drain_remote_list_to_local_head_class_resolved > 0
                )
                or (
                    remote_free_drain_local_list_mutation_lowering_producer
                    and drain_remote_list_to_local_head_class_resolved > 0
                )
                or (
                    remote_owner_branch_routing_any
                    and drain_remote_list_to_local_head_class_resolved > 0
                )
            ),
            "atomic_remote_head_drain_local_list_head_class": (
                "owner_local_free_or_free_head"
                if remote_free_drain_local_list_mutation_proof
                or remote_free_drain_local_list_mutation_vocabulary_preflight
                or (
                    remote_free_drain_local_list_mutation_verifier_preconditions
                    and drain_remote_list_to_local_head_class_resolved > 0
                )
                or (
                    remote_free_drain_local_list_mutation_lowering_producer
                    and drain_remote_list_to_local_head_class_resolved > 0
                )
                or (
                    remote_owner_branch_routing_any
                    and drain_remote_list_to_local_head_class_resolved > 0
                )
                else "closed"
            ),
            "atomic_remote_head_drain_local_list_publication_order": (
                "verifier_owned_acquire_then_owner_local"
                if remote_free_drain_local_list_mutation_proof
                or remote_free_drain_local_list_mutation_vocabulary_preflight
                or (
                    remote_free_drain_local_list_mutation_verifier_preconditions
                    and drain_remote_list_to_local_head_class_resolved > 0
                )
                or (
                    remote_free_drain_local_list_mutation_lowering_producer
                    and drain_remote_list_to_local_head_class_resolved > 0
                )
                or (
                    remote_owner_branch_routing_any
                    and drain_remote_list_to_local_head_class_resolved > 0
                )
                else "closed"
            ),
            "atomic_remote_head_drain_local_list_mutation_lowerable_count": str(
                drain_remote_list_to_local_lowerable
            ),
            "atomic_remote_head_drain_local_list_mutation_lowered_count": str(
                drain_remote_list_to_local_lowerable
                if remote_free_drain_local_list_mutation_lowering_producer
                or remote_owner_branch_routing_any
                else 0
            ),
        }
    )

    rows: list[tuple[str, str]] = [
        ("output_contract", "hako-check-fastmem-mir-to-llvm-producer-report-v0"),
        ("tool_surface", "fastmem_mir_to_llvm_producer_report"),
        ("input_kind", "mir_json_metadata"),
        ("observation_only", "1"),
        ("behavior_change", "0"),
        ("replacement_front_source_truth", "hako_fastmem"),
        ("replacement_front_producer_taxonomy_v0", "1"),
        ("replacement_front_producer", "mir_to_llvm_lowering"),
        ("replacement_front_backend_artifact", "object"),
        ("replacement_front_producer_transition_state", "final_primary"),
        *slice_rows,
        ("replacement_front_selection_behavior_change", "0"),
        ("replacement_front_selection_product_activation", "0"),
        ("replacement_front_selection_bridge_retirement_allowed", "0"),
        ("replacement_front_python_template_c_semantic_ssot", "0"),
        ("replacement_front_python_template_c_retirement_required", "1"),
        ("replacement_front_mirbuilder_representation_only", "1"),
        ("replacement_front_mirbuilder_route_decision_count", "0"),
        ("replacement_front_mir_memop_enabled", "1"),
        ("replacement_front_mir_fastmem_region_enabled", "1"),
        ("replacement_front_fastmem_enabled", "1"),
        ("replacement_front_is_full_hako_algorithm", "0"),
        ("hako_mimalloc_algorithm_claim", "0"),
        ("fastmem_region_count", str(len(regions))),
        ("fastmem_contract_count", str(len(contract_ids))),
        ("fastmem_contract_id", contract_id),
        ("fastmem_verified_mem_access_plan_count", str(len(verified_plans))),
        ("fastmem_verified_table_access_count", str(len(verified_table))),
        ("fastmem_verified_field_access_count", str(len(verified_field))),
        ("fastmem_table_access_plan_count", str(len(verified_table))),
        ("fastmem_field_load_plan_count", str(len(verified_field_load))),
        ("fastmem_field_store_plan_count", str(len(verified_field_store))),
        ("fastmem_local_free_push_plan_count", str(len(verified_local_free_push))),
        ("fastmem_local_free_pop_plan_count", str(len(verified_local_free_pop))),
        ("fastmem_free_head_push_plan_count", str(len(verified_free_head_push))),
        ("fastmem_free_head_pop_plan_count", str(len(verified_free_head_pop))),
        *_atomic_remote_head_rows(atomic_remote_state),
        *_remote_owner_branch_routing_rows(
            remote_owner_branch_routing_selected_any=remote_owner_branch_routing_selected_any,
            remote_owner_branch_routing_open_any=remote_owner_branch_routing_open_any,
            remote_owner_branch_routing_lowered_count_value=remote_owner_branch_routing_lowered_count_value,
            remote_owner_branch_routing_preflight_requires_branch_cfg_row_value=remote_owner_branch_routing_preflight_requires_branch_cfg_row_value,
            remote_owner_branch_route_body_selected_any=remote_owner_branch_route_body_selected_any,
        ),
        *_branch_cfg_and_same_remote_rows(
            fastmem_branch_cfg_selected_any=fastmem_branch_cfg_selected_any,
            fastmem_branch_cfg_open_any=fastmem_branch_cfg_open_any,
            fastmem_branch_cfg_closed_guard_any=fastmem_branch_cfg_closed_guard_any,
            fastmem_branch_cfg_lowered_count_value=fastmem_branch_cfg_lowered_count_value,
            fastmem_branch_cfg_source_guard_value=fastmem_branch_cfg_source_guard_value,
            same_remote_free_body_selected_any=same_remote_free_body_selected_any,
            same_remote_free_body_open_any=same_remote_free_body_open_any,
            same_remote_free_body_lowered_count_any=same_remote_free_body_lowered_count_any,
        ),
        (
            "page_local_free_route_cfg_selected",
            str(
                int_flag(
                    page_local_free_route_cfg_any
                    or page_local_route_body_join_any
                    or tls_backing_transfer_or_later
                )
            ),
        ),
        (
            "page_local_alloc_route_cfg_selected",
            str(int_flag(page_local_alloc_route_cfg_any)),
        ),
        (
            "page_local_route_body_join_selected",
            str(int_flag(page_local_route_body_join_any)),
        ),
        *_terminal_ladder_refresh_rows(
            page_local_route_body_join_open=page_local_route_body_join_open_any,
            terminal_ladder_refresh_selected_any=terminal_ladder_refresh_selected_any,
            terminal_ladder_refresh_open_any=terminal_ladder_refresh_open_any,
        ),
        (
            "tls_backing_transfer_selected",
            str(int_flag(tls_backing_transfer_or_later)),
        ),
    ]
    rows.extend(
        _page_local_route_report_rows(
            profile=profile,
            route_candidate=route_candidate,
            free_route_candidate=free_route_candidate,
            page_local_alloc_route_cfg_any=page_local_alloc_route_cfg_any,
            page_local_route_body_join_any=page_local_route_body_join_any,
            page_local_alloc_route_cfg_producer=page_local_alloc_route_cfg_producer,
            page_local_free_route_cfg_producer=page_local_free_route_cfg_producer,
            tls_backing_transfer_or_later=tls_backing_transfer_or_later,
            free_head_non_empty_facts=free_head_non_empty_facts,
        )
    )
    state = locals().copy()
    state.update(route_state)
    rows.extend(build_tail_rows(state))
    return rows
