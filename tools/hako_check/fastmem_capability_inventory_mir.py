#!/usr/bin/env python3
"""Inventory FastMemory capability coverage from benchmark reports.

This adapter is observation-only. It reads existing replacement-front report
key/value files and reports whether fastmem/capability surfaces are present.
It does not run benchmarks, rewrite source, choose keepers, or activate
allocator replacement.
"""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from fastmem_capability_inventory_common import (
    MIMALLOC_COVERAGE_DEFAULT_THRESHOLD,
    MIMALLOC_SAFETY_DEFAULT_THRESHOLD,
    MIMALLOC_SHAPE_COMPONENT_POINTS,
    MIMALLOC_SHAPE_DEFAULT_THRESHOLD,
    add_count,
    analyze_expr,
    analyze_stmt,
    base_inventory,
    build_source_inventory,
    call_name,
    child_expr,
    classify_remote_memory_order,
    contract_family,
    emit_summary,
    first_subject_value,
    int_route_flag,
    int_subject_value,
    iter_fastmem_regions,
    iter_nodes,
    is_mem_method_call,
    is_node,
    route_value,
    speed_score_from_ratio,
    typed_page_meta_fields,
    write_output,
)
from report_kv import first_value, int_value, prefixed, read_kv
from replacement_front_report import (
    build_report as build_replacement_report,
    emit_kv,
    format_value,
    page_lookup_route,
    page_map_bridge_kind,
)


def build_mir_metadata_inventory(root: dict[str, Any]) -> dict[str, Any]:
    report = base_inventory("mir_json_metadata")
    regions: list[dict[str, Any]] = []
    plans: list[dict[str, Any]] = []
    memops: list[dict[str, Any]] = []
    same_owner_facts: list[dict[str, Any]] = []
    remote_owner_facts: list[dict[str, Any]] = []
    block_next_facts: list[dict[str, Any]] = []
    local_free_non_empty_facts: list[dict[str, Any]] = []
    free_head_non_empty_facts: list[dict[str, Any]] = []

    for function in root.get("functions", []):
        metadata = function.get("metadata", {})
        regions.extend(
            region
            for region in metadata.get("fastmem_regions", [])
            if isinstance(region, dict)
        )
        plans.extend(
            plan
            for plan in metadata.get("fastmem_access_plans", [])
            if isinstance(plan, dict)
        )
        same_owner_facts.extend(
            fact
            for fact in metadata.get("fastmem_same_owner_facts", [])
            if isinstance(fact, dict)
        )
        remote_owner_facts.extend(
            fact
            for fact in metadata.get("fastmem_remote_owner_facts", [])
            if isinstance(fact, dict)
        )
        block_next_facts.extend(
            fact
            for fact in metadata.get("fastmem_block_next_facts", [])
            if isinstance(fact, dict)
        )
        local_free_non_empty_facts.extend(
            fact
            for fact in metadata.get("fastmem_local_free_non_empty_facts", [])
            if isinstance(fact, dict)
        )
        free_head_non_empty_facts.extend(
            fact
            for fact in metadata.get("fastmem_free_head_non_empty_facts", [])
            if isinstance(fact, dict)
        )
        for block in function.get("blocks", []):
            for inst in block.get("instructions", []):
                if isinstance(inst, dict) and inst.get("op") == "memop":
                    memops.append(inst)

    contracts = sorted(
        {
            str(region.get("contract"))
            for region in regions
            if region.get("contract") not in (None, "")
        }
    )
    verified_plans = [plan for plan in plans if bool(plan.get("verified"))]
    verified_field_plans = [
        plan
        for plan in verified_plans
        if str(plan.get("kind")) in {"field_load", "field_store"}
    ]
    verified_table_plans = [
        plan for plan in verified_plans if str(plan.get("kind")) == "table_index"
    ]
    local_free_push_plans = [
        plan for plan in plans if str(plan.get("kind")) == "local_free_push"
    ]
    local_free_pop_plans = [
        plan for plan in plans if str(plan.get("kind")) == "local_free_pop"
    ]
    local_free_plans = local_free_push_plans + local_free_pop_plans
    free_head_push_plans = [
        plan for plan in plans if str(plan.get("kind")) == "free_head_push"
    ]
    free_head_pop_plans = [
        plan for plan in plans if str(plan.get("kind")) == "free_head_pop"
    ]
    free_head_plans = free_head_push_plans + free_head_pop_plans
    atomic_remote_head_push_plans = [
        plan for plan in plans if str(plan.get("kind")) == "atomic_remote_head_push"
    ]
    atomic_remote_head_drain_plans = [
        plan for plan in plans if str(plan.get("kind")) == "atomic_remote_head_drain"
    ]
    atomic_remote_head_plans = atomic_remote_head_push_plans + atomic_remote_head_drain_plans
    drain_remote_list_to_local_plans = [
        plan for plan in plans if str(plan.get("kind")) == "drain_remote_list_to_local"
    ]
    rejected_table_plans = [
        plan
        for plan in plans
        if str(plan.get("kind")) == "table_index" and not bool(plan.get("verified"))
    ]

    def count_memop(kind: str) -> int:
        return sum(1 for inst in memops if inst.get("kind") == kind)

    missing_field_id = sum(
        1
        for plan in plans
        if str(plan.get("kind")) in {"field_load", "field_store"}
        and not plan.get("field_id")
    )
    missing_table_id = sum(
        1
        for plan in plans
        if str(plan.get("kind")) == "table_index" and not plan.get("table_id")
    )
    table_unchecked = sum(
        1
        for plan in plans
        if str(plan.get("kind")) == "table_index"
        and bool(plan.get("table_length_resolved"))
        and not bool(plan.get("bounds_proof_valid"))
    )
    table_incomplete = sum(
        1
        for plan in rejected_table_plans
        if str(plan.get("failure_reason") or "").endswith("proof-incomplete")
        or not bool(plan.get("table_length_resolved"))
        or not bool(plan.get("overflow_proof_valid"))
    )
    table_overflow_missing = sum(
        1
        for plan in plans
        if str(plan.get("kind")) == "table_index"
        and bool(plan.get("table_length_resolved"))
        and bool(plan.get("bounds_proof_valid"))
        and not bool(plan.get("overflow_proof_valid"))
    )
    unknown_alignment = sum(
        1
        for plan in plans
        if str(plan.get("kind")) in {"table_index", "field_load", "field_store"}
        and plan.get("alignment") in (None, "", 0)
        or (
            str(plan.get("kind")) == "table_index"
            and not bool(plan.get("alignment_valid"))
        )
    )
    atomic_plain_store = sum(
        1
        for plan in plans
        if str(plan.get("kind")) == "field_store"
        and str(plan.get("field_class")) == "atomic_remote_head"
    )
    local_free_nonlowerable = sum(
        1 for plan in local_free_plans if not bool(plan.get("lowerable"))
    )
    local_free_push_lowerable = sum(
        1 for plan in local_free_push_plans if bool(plan.get("lowerable"))
    )
    local_free_pop_lowerable = sum(
        1 for plan in local_free_pop_plans if bool(plan.get("lowerable"))
    )
    local_free_same_owner_missing = sum(
        1
        for plan in local_free_plans
        if not bool(plan.get("same_owner_proof_valid"))
    )
    local_free_remote_owner_rejected = sum(
        1
        for plan in local_free_plans
        if bool(plan.get("remote_owner_rejected"))
    )
    local_free_block_next_missing = sum(
        1
        for plan in local_free_push_plans
        if not bool(plan.get("block_next_proof_valid"))
    )
    local_free_non_empty_missing = sum(
        1
        for plan in local_free_pop_plans
        if not bool(plan.get("non_empty_proof_valid"))
    )
    local_free_head_access_resolved = sum(
        1
        for plan in local_free_plans
        if plan.get("local_free_head_byte_offset") not in (None, "")
        and plan.get("local_free_head_field_size") not in (None, "")
        and plan.get("local_free_head_field_type") not in (None, "")
        and plan.get("local_free_head_alignment") not in (None, "")
    )
    local_free_block_next_access_resolved = sum(
        1
        for plan in local_free_plans
        if plan.get("block_next_byte_offset") not in (None, "")
        and plan.get("block_next_field_size") not in (None, "")
        and plan.get("block_next_field_type") not in (None, "")
        and plan.get("block_next_alignment") not in (None, "")
    )
    local_free_access_plan_incomplete = sum(
        1
        for plan in local_free_plans
        if bool(plan.get("lowerable"))
        and (
            plan.get("local_free_head_byte_offset") in (None, "")
            or plan.get("local_free_head_field_size") in (None, "")
            or plan.get("local_free_head_field_type") in (None, "")
            or plan.get("local_free_head_alignment") in (None, "")
            or plan.get("block_next_byte_offset") in (None, "")
            or plan.get("block_next_field_size") in (None, "")
            or plan.get("block_next_field_type") in (None, "")
            or plan.get("block_next_alignment") in (None, "")
        )
    )
    free_head_nonlowerable = sum(
        1 for plan in free_head_plans if not bool(plan.get("lowerable"))
    )
    free_head_push_lowerable = sum(
        1 for plan in free_head_push_plans if bool(plan.get("lowerable"))
    )
    free_head_pop_lowerable = sum(
        1 for plan in free_head_pop_plans if bool(plan.get("lowerable"))
    )
    free_head_same_owner_missing = sum(
        1
        for plan in free_head_plans
        if not bool(plan.get("same_owner_proof_valid"))
    )
    free_head_remote_owner_rejected = sum(
        1
        for plan in free_head_plans
        if bool(plan.get("remote_owner_rejected"))
    )
    free_head_block_next_missing = sum(
        1
        for plan in free_head_push_plans
        if not bool(plan.get("block_next_proof_valid"))
    )
    free_head_non_empty_missing = sum(
        1
        for plan in free_head_pop_plans
        if not bool(plan.get("non_empty_proof_valid"))
    )
    free_head_non_empty_source_assume = sum(
        1
        for fact in free_head_non_empty_facts
        if str(fact.get("proof_kind")) == "source_assume_free_head_non_empty"
    )
    free_head_non_empty_derived_from_push = sum(
        1
        for fact in free_head_non_empty_facts
        if str(fact.get("proof_kind")) == "derived_from_free_head_push"
    )
    free_head_access_resolved = sum(
        1
        for plan in free_head_plans
        if plan.get("free_head_byte_offset") not in (None, "")
        and plan.get("free_head_field_size") not in (None, "")
        and plan.get("free_head_field_type") not in (None, "")
        and plan.get("free_head_alignment") not in (None, "")
    )
    free_head_block_next_access_resolved = sum(
        1
        for plan in free_head_plans
        if plan.get("block_next_byte_offset") not in (None, "")
        and plan.get("block_next_field_size") not in (None, "")
        and plan.get("block_next_field_type") not in (None, "")
        and plan.get("block_next_alignment") not in (None, "")
    )
    free_head_access_plan_incomplete = sum(
        1
        for plan in free_head_plans
        if bool(plan.get("lowerable"))
        and (
            plan.get("free_head_byte_offset") in (None, "")
            or plan.get("free_head_field_size") in (None, "")
            or plan.get("free_head_field_type") in (None, "")
            or plan.get("free_head_alignment") in (None, "")
            or plan.get("block_next_byte_offset") in (None, "")
            or plan.get("block_next_field_size") in (None, "")
            or plan.get("block_next_field_type") in (None, "")
            or plan.get("block_next_alignment") in (None, "")
        )
    )
    remote_owner_source_assume = sum(
        1
        for fact in remote_owner_facts
        if str(fact.get("proof_kind")) == "source_assume_remote_owner"
        and bool(fact.get("same_owner_rejected"))
    )
    remote_free_block_next_source_assume = sum(
        1
        for fact in block_next_facts
        if str(fact.get("proof_kind")) == "source_assume_remote_free_block_next"
    )
    atomic_remote_head_push_lowerable = sum(
        1 for plan in atomic_remote_head_push_plans if bool(plan.get("lowerable"))
    )
    atomic_remote_head_drain_lowerable = sum(
        1 for plan in atomic_remote_head_drain_plans if bool(plan.get("lowerable"))
    )
    atomic_remote_head_remote_owner_required = int(
        any(bool(plan.get("remote_owner_required")) for plan in atomic_remote_head_push_plans)
    )
    atomic_remote_head_remote_owner_missing = sum(
        1
        for plan in atomic_remote_head_push_plans
        if bool(plan.get("remote_owner_required"))
        and not bool(plan.get("remote_owner_proof_valid"))
    )
    atomic_remote_head_block_next_required = int(
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
                str(plan.get("memory_order_policy") or "unknown")
                for plan in atomic_remote_head_plans
            }
        )
    )
    if not atomic_remote_head_memory_order_policy:
        atomic_remote_head_memory_order_policy = "none"
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

    report.update(
        {
            "input_kind": "mir_json_metadata",
            "measured_hot_path_owner": "hako_source",
            "replacement_front_producer": "mir_json_metadata",
            "replacement_front_source_truth": "hako_fastmem",
            "replacement_front_mir_memop_enabled": int(bool(memops)),
            "replacement_front_mir_fastmem_region_enabled": int(bool(regions)),
            "fastmem_region_count": len(regions),
            "fastmem_contract_count": len(contracts),
            "fastmem_contract_id": ",".join(contracts) if contracts else "unknown",
            "fastmem_contract_family": (
                contract_family(contracts[0]) if len(contracts) == 1 else "mixed"
            )
            if contracts
            else "unknown",
            "fastmem_memop_region_begin_count": len(regions),
            "fastmem_memop_region_end_count": len(regions),
            "fastmem_memop_unbalanced_region_count": 0,
            "fastmem_memop_addr_of_count": count_memop("addr_of"),
            "fastmem_memop_add_count": count_memop("add"),
            "fastmem_memop_sub_count": count_memop("sub"),
            "fastmem_memop_logical_shr_count": count_memop("logical_shr"),
            "fastmem_memop_and_count": count_memop("bit_and"),
            "fastmem_memop_table_index_count": count_memop("table_index"),
            "fastmem_memop_field_load_count": count_memop("field_load"),
            "fastmem_memop_field_store_count": count_memop("field_store"),
            "fastmem_memop_current_alloc_owner_id_count": count_memop(
                "current_alloc_owner_id"
            ),
            "fastmem_memop_owner_eq_count": count_memop("owner_eq"),
            "fastmem_memop_local_free_push_count": count_memop("local_free_push"),
            "fastmem_memop_local_free_pop_count": count_memop("local_free_pop"),
            "fastmem_memop_free_head_push_count": count_memop("free_head_push"),
            "fastmem_memop_free_head_pop_count": count_memop("free_head_pop"),
            "fastmem_memop_atomic_remote_head_push_count": count_memop(
                "atomic_remote_head_push"
            ),
            "fastmem_memop_atomic_remote_head_drain_count": count_memop(
                "atomic_remote_head_drain"
            ),
            "fastmem_memop_drain_remote_list_to_local_count": count_memop(
                "drain_remote_list_to_local"
            ),
            "fastmem_local_free_list_plan": int(bool(local_free_plans)),
            "fastmem_local_free_push_plan_count": len(local_free_push_plans),
            "fastmem_local_free_pop_plan_count": len(local_free_pop_plans),
            "fastmem_local_free_nonlowerable_count": local_free_nonlowerable,
            "fastmem_local_free_push_lowerable_count": local_free_push_lowerable,
            "fastmem_local_free_pop_lowerable_count": local_free_pop_lowerable,
            "fastmem_local_free_head_access_resolved_count": local_free_head_access_resolved,
            "fastmem_local_free_block_next_access_resolved_count": local_free_block_next_access_resolved,
            "fastmem_local_free_access_plan_incomplete_count": local_free_access_plan_incomplete,
            "fastmem_same_owner_fact_count": len(same_owner_facts),
            "fastmem_remote_owner_fact_count": len(remote_owner_facts),
            "fastmem_remote_owner_source_assume_count": remote_owner_source_assume,
            "fastmem_block_next_fact_count": len(block_next_facts),
            "fastmem_remote_free_block_next_source_assume_count": (
                remote_free_block_next_source_assume
            ),
            "fastmem_local_free_non_empty_fact_count": len(local_free_non_empty_facts),
            "fastmem_local_free_same_owner_required": int(bool(local_free_plans)),
            "fastmem_local_free_same_owner_missing_count": local_free_same_owner_missing,
            "fastmem_local_free_non_empty_required": int(bool(local_free_pop_plans)),
            "fastmem_local_free_non_empty_missing_count": local_free_non_empty_missing,
            "fastmem_local_free_remote_owner_rejected_count": local_free_remote_owner_rejected,
            "fastmem_local_free_block_next_proof_missing_count": local_free_block_next_missing,
            "fastmem_free_head_list_plan": int(bool(free_head_plans)),
            "fastmem_free_head_push_plan_count": len(free_head_push_plans),
            "fastmem_free_head_pop_plan_count": len(free_head_pop_plans),
            "fastmem_free_head_nonlowerable_count": free_head_nonlowerable,
            "fastmem_free_head_push_lowerable_count": free_head_push_lowerable,
            "fastmem_free_head_pop_lowerable_count": free_head_pop_lowerable,
            "fastmem_free_head_access_resolved_count": free_head_access_resolved,
            "fastmem_free_head_block_next_access_resolved_count": free_head_block_next_access_resolved,
            "fastmem_free_head_access_plan_incomplete_count": free_head_access_plan_incomplete,
            "fastmem_free_head_non_empty_fact_count": len(free_head_non_empty_facts),
            "fastmem_free_head_non_empty_source_assume_count": free_head_non_empty_source_assume,
            "fastmem_free_head_non_empty_derived_from_free_head_push_count": (
                free_head_non_empty_derived_from_push
            ),
            "fastmem_free_head_same_owner_required": int(bool(free_head_plans)),
            "fastmem_free_head_same_owner_missing_count": free_head_same_owner_missing,
            "fastmem_free_head_non_empty_required": int(bool(free_head_pop_plans)),
            "fastmem_free_head_non_empty_missing_count": free_head_non_empty_missing,
            "fastmem_free_head_remote_owner_rejected_count": free_head_remote_owner_rejected,
            "fastmem_free_head_block_next_proof_missing_count": free_head_block_next_missing,
            "atomic_remote_head_push_plan_count": len(atomic_remote_head_push_plans),
            "atomic_remote_head_push_lowerable_count": atomic_remote_head_push_lowerable,
            "atomic_remote_head_drain_plan_count": len(atomic_remote_head_drain_plans),
            "atomic_remote_head_drain_lowerable_count": atomic_remote_head_drain_lowerable,
            "drain_remote_list_to_local_plan_count": len(
                drain_remote_list_to_local_plans
            ),
            "drain_remote_list_to_local_token_provenance_valid": (
                drain_remote_list_to_local_token_provenance_valid
            ),
            "drain_remote_list_to_local_page_operand_valid": (
                drain_remote_list_to_local_page_operand_valid
            ),
            "drain_remote_list_to_local_head_class_resolved": (
                drain_remote_list_to_local_head_class_resolved
            ),
            "drain_remote_list_to_local_lowerable_count": (
                drain_remote_list_to_local_lowerable
            ),
            "atomic_remote_head_drain_local_list_mutation_lowerable_count": (
                drain_remote_list_to_local_lowerable
            ),
            "atomic_remote_head_remote_owner_required": (
                atomic_remote_head_remote_owner_required
            ),
            "atomic_remote_head_remote_owner_missing_count": (
                atomic_remote_head_remote_owner_missing
            ),
            "atomic_remote_head_block_next_required": atomic_remote_head_block_next_required,
            "atomic_remote_head_block_next_missing_count": atomic_remote_head_block_next_missing,
            "atomic_remote_head_access_resolved_count": atomic_remote_head_access_resolved,
            "atomic_remote_head_memory_order_policy": atomic_remote_head_memory_order_policy,
            "fastmem_verified_mem_access_plan_count": len(verified_plans),
            "fastmem_verified_field_access_count": len(verified_field_plans),
            "fastmem_verified_table_access_count": len(verified_table_plans),
            "fastmem_field_id_missing_count": missing_field_id,
            "fastmem_table_id_missing_count": missing_table_id,
            "fastmem_table_index_unchecked_count": table_unchecked,
            "fastmem_table_access_proof_incomplete_count": table_incomplete,
            "fastmem_table_overflow_proof_missing_count": table_overflow_missing,
            "fastmem_unknown_alignment_count": unknown_alignment,
            "fastmem_atomic_field_plain_store_count": atomic_plain_store,
            "summary": "ok" if regions or memops or plans else "failed",
        }
    )
    return report

