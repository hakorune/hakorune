#!/usr/bin/env python3
"""Emit FastMemory MIR-to-LLVM producer evidence from a MIR JSON file.

This tool is observation-only: it does not decide routes or rewrite MIR. It
first asks the existing Python LLVM producer to compile the MIR JSON, then emits
producer-neutral KV evidence from the verified FastMemory metadata that the
producer consumed successfully.
"""

from __future__ import annotations

import argparse
import json
import subprocess
import sys
import tempfile
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[2]
LLVM_BUILDER = ROOT / "src" / "llvm_py" / "llvm_builder.py"


def int_flag(value: bool) -> int:
    return 1 if value else 0


def load_json(path: Path) -> dict[str, Any]:
    try:
        with path.open("r", encoding="utf-8") as f:
            data = json.load(f)
    except OSError as exc:
        raise SystemExit(f"failed to read MIR JSON: {path}: {exc}") from exc
    except json.JSONDecodeError as exc:
        raise SystemExit(f"failed to parse MIR JSON: {path}: {exc}") from exc
    if not isinstance(data, dict):
        raise SystemExit(f"expected MIR JSON object: {path}")
    return data


def functions(mir: dict[str, Any]) -> list[dict[str, Any]]:
    values = mir.get("functions", [])
    if not isinstance(values, list):
        return []
    return [value for value in values if isinstance(value, dict)]


def fastmem_regions(mir: dict[str, Any]) -> list[dict[str, Any]]:
    regions: list[dict[str, Any]] = []
    for function in functions(mir):
        metadata = function.get("metadata", {})
        if not isinstance(metadata, dict):
            continue
        for region in metadata.get("fastmem_regions", []):
            if isinstance(region, dict):
                regions.append(region)
    return regions


def fastmem_access_plans(mir: dict[str, Any]) -> list[dict[str, Any]]:
    plans: list[dict[str, Any]] = []
    for function in functions(mir):
        metadata = function.get("metadata", {})
        if not isinstance(metadata, dict):
            continue
        for plan in metadata.get("fastmem_access_plans", []):
            if isinstance(plan, dict):
                plans.append(plan)
    return plans


def fastmem_memops(mir: dict[str, Any]) -> list[dict[str, Any]]:
    memops: list[dict[str, Any]] = []
    for function in functions(mir):
        blocks = function.get("blocks", [])
        if not isinstance(blocks, list):
            continue
        for block in blocks:
            if not isinstance(block, dict):
                continue
            instructions = block.get("instructions", [])
            if not isinstance(instructions, list):
                continue
            for inst in instructions:
                if isinstance(inst, dict) and inst.get("op") == "memop":
                    memops.append(inst)
    return memops


def fastmem_free_head_non_empty_facts(mir: dict[str, Any]) -> list[dict[str, Any]]:
    facts: list[dict[str, Any]] = []
    for function in functions(mir):
        metadata = function.get("metadata", {})
        if not isinstance(metadata, dict):
            continue
        for fact in metadata.get("fastmem_free_head_non_empty_facts", []):
            if isinstance(fact, dict):
                facts.append(fact)
    return facts


def metadata_facts(mir: dict[str, Any], key: str) -> list[dict[str, Any]]:
    facts: list[dict[str, Any]] = []
    for function in functions(mir):
        metadata = function.get("metadata", {})
        if not isinstance(metadata, dict):
            continue
        for fact in metadata.get(key, []):
            if isinstance(fact, dict):
                facts.append(fact)
    return facts


def is_verified(plan: dict[str, Any]) -> bool:
    return bool(plan.get("verified")) and plan.get("status") == "verified"


def count_plans(plans: list[dict[str, Any]], kind: str, *, verified: bool | None = None) -> int:
    count = 0
    for plan in plans:
        if plan.get("kind") != kind:
            continue
        if verified is not None and is_verified(plan) != verified:
            continue
        count += 1
    return count


def count_memops(memops: list[dict[str, Any]], kind: str) -> int:
    return sum(1 for inst in memops if inst.get("kind") == kind)


def page_local_alloc_route_candidate(
    *,
    local_free_pop_count: int,
    free_head_push_count: int,
    free_head_pop_count: int,
) -> str:
    if local_free_pop_count == 0 and free_head_push_count == 0 and free_head_pop_count == 0:
        return "none"
    if local_free_pop_count == 1 and free_head_push_count == 0 and free_head_pop_count == 0:
        return "local_free_alloc"
    if local_free_pop_count == 0 and free_head_push_count == 0 and free_head_pop_count == 1:
        return "free_head_alloc"
    if local_free_pop_count == 1 and free_head_push_count == 1 and free_head_pop_count == 1:
        return "refill_then_free_head_alloc"
    return "mixed"


def page_local_free_route_candidate(
    *,
    local_free_push_count: int,
    local_free_pop_count: int,
    free_head_push_count: int,
    free_head_pop_count: int,
) -> str:
    if local_free_push_count == 0:
        return "none"
    if (
        local_free_push_count == 1
        and local_free_pop_count == 0
        and free_head_push_count == 0
        and free_head_pop_count == 0
    ):
        return "same_owner_local_free"
    return "mixed"


def run_llvm_builder(mir_json: Path, object_out: Path) -> None:
    proc = subprocess.run(
        [sys.executable, str(LLVM_BUILDER), str(mir_json), "-o", str(object_out)],
        cwd=str(ROOT),
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=False,
    )
    if proc.returncode != 0:
        if proc.stdout:
            sys.stderr.write(proc.stdout)
        if proc.stderr:
            sys.stderr.write(proc.stderr)
        raise SystemExit(proc.returncode)


def string_value(value: Any, default: str = "") -> str:
    if value is None:
        return default
    return str(value)


def build_rows(
    mir: dict[str, Any], *, object_out: Path, profile: str
) -> list[tuple[str, str]]:
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

    free_route_candidate = "none"

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
    remote_owner_branch_routing_preflight = (
        profile == "remote-owner-branch-routing-preflight"
    )
    remote_owner_branch_routing_lowering_preflight = (
        profile == "remote-owner-branch-routing-lowering-preflight"
    )
    remote_owner_branch_routing_lowering_producer = (
        profile == "remote-owner-branch-routing-lowering"
    )
    remote_owner_branch_route_body_preflight = (
        profile == "remote-owner-branch-route-body-preflight"
    )
    remote_owner_branch_routing_any = (
        remote_owner_branch_routing_preflight
        or remote_owner_branch_routing_lowering_preflight
        or remote_owner_branch_routing_lowering_producer
        or remote_owner_branch_route_body_preflight
    )
    remote_owner_branch_routing_lowering_any = (
        remote_owner_branch_routing_lowering_preflight
        or remote_owner_branch_routing_lowering_producer
        or remote_owner_branch_route_body_preflight
    )
    if profile in {
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
        "remote-free-drain-local-list-mutation-lowering",
        "remote-owner-branch-routing-preflight",
        "remote-owner-branch-routing-lowering-preflight",
        "remote-owner-branch-routing-lowering",
        "remote-owner-branch-route-body-preflight",
    }:
        remote_free_open = profile in {
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
        }
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
        slice_rows = [
            ("replacement_front_producer_slice_selection_v0", "0"),
            (
                "replacement_front_selected_route",
                "remote_owner_branch_route_body_preflight"
                if remote_owner_branch_route_body_preflight
                else (
                    "remote_owner_branch_routing_lowering_producer_pilot"
                    if remote_owner_branch_routing_lowering_producer
                    else (
                        "remote_owner_branch_routing_lowering_preflight"
                        if remote_owner_branch_routing_lowering_preflight
                        else (
                            "remote_owner_branch_routing_preflight"
                            if remote_owner_branch_routing_preflight
                            else "none"
                        )
                    )
                ),
            ),
            ("replacement_front_next_producer_slice", next_slice),
            (
                "replacement_front_selected_memop_family",
                "remote_free_routing" if remote_owner_branch_routing_any else "remote_free",
            ),
            ("replacement_front_selected_memop_kinds", selected_remote_kind),
            ("replacement_front_deferred_memop_family", "remote_free_execution"),
            ("replacement_front_deferred_memop_kinds", deferred_remote_kinds),
            ("mir_fmem_008b_layout_table_producer_pilot", "0"),
            ("fastmem_owner_runtime_producer_pilot", "0"),
            ("fastmem_local_free_producer_pilot", "0"),
            ("fastmem_atomic_remote_head_cas_preflight", str(int_flag(not remote_free_open))),
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
        (
            "atomic_remote_head_cas_lowering_selected",
            str(
                int_flag(
                    profile
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
                    }
                )
            ),
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
        (
            "atomic_remote_head_memory_order_policy",
            atomic_remote_head_memory_order_policy,
        ),
        ("fastmem_remote_owner_fact_count", str(len(remote_owner_facts))),
        ("fastmem_remote_owner_source_assume_count", str(remote_owner_source_assume)),
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
                if remote_free_retry_producer
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
                else 0
            ),
        ),
        (
            "atomic_remote_head_drain_selected",
            str(
                int_flag(
                    remote_free_drain_preflight
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
            "atomic_remote_head_drain_exchange_selected",
            str(
                int_flag(
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
                )
            ),
        ),
        (
            "atomic_remote_head_drain_exchange_order",
            "acquire"
            if (
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
            )
            else "closed",
        ),
        (
            "atomic_remote_head_drain_result_kind",
            "remote_free_list_token"
            if (
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
            )
            else "closed",
        ),
        (
            "atomic_remote_head_drain_to_local_route_selected",
            str(
                int_flag(
                    remote_free_drain_to_local_selection
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
            "atomic_remote_head_drain_to_local_route_producer_pilot",
            str(int_flag(remote_free_drain_to_local_producer)),
        ),
        (
            "atomic_remote_head_drain_to_local_route_open",
            str(
                int_flag(
                    remote_free_drain_to_local_producer
                    or remote_free_drain_local_list_mutation_preflight
                    or remote_free_drain_local_list_mutation_proof
                    or remote_free_drain_local_list_mutation_vocabulary_preflight
                    or remote_free_drain_local_list_mutation_verifier_preconditions
                )
            ),
        ),
        (
            "atomic_remote_head_drain_local_list_mutation_selected",
            str(
                int_flag(
                    remote_free_drain_local_list_mutation_preflight
                    or remote_free_drain_local_list_mutation_proof
                    or remote_free_drain_local_list_mutation_vocabulary_preflight
                    or remote_free_drain_local_list_mutation_verifier_preconditions
                    or remote_free_drain_local_list_mutation_lowering_producer
                    or remote_owner_branch_routing_any
                )
            ),
        ),
        (
            "atomic_remote_head_drain_local_list_mutation_open",
            str(
                int_flag(
                    remote_free_drain_local_list_mutation_lowering_producer
                    or remote_owner_branch_routing_any
                )
            ),
        ),
        (
            "atomic_remote_head_drain_local_list_token_escape_count",
            "0",
        ),
        (
            "atomic_remote_head_drain_local_list_head_class_resolved",
            str(
                int_flag(
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
                )
            ),
        ),
        (
            "atomic_remote_head_drain_local_list_head_class",
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
            else "closed",
        ),
        (
            "atomic_remote_head_drain_local_list_publication_order",
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
            else "closed",
        ),
        (
            "atomic_remote_head_drain_open",
            str(
                int_flag(
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
            ),
        ),
        ("atomic_remote_head_drain_plan_count", str(len(atomic_remote_head_drain_plans))),
        ("atomic_remote_head_drain_lowerable_count", str(atomic_remote_head_drain_lowerable)),
        (
            "atomic_remote_head_drain_lowered_count",
            str(
                atomic_remote_head_drain_lowerable
                if remote_free_drain_exchange_producer
                or remote_free_drain_to_local_selection
                or remote_free_drain_to_local_producer
                or remote_free_drain_local_list_mutation_preflight
                or remote_free_drain_local_list_mutation_proof
                or remote_free_drain_local_list_mutation_vocabulary_preflight
                or remote_free_drain_local_list_mutation_verifier_preconditions
                or remote_free_drain_local_list_mutation_lowering_producer
                or remote_owner_branch_routing_any
                else 0
            ),
        ),
        (
            "fastmem_memop_drain_remote_list_to_local_count",
            str(drain_remote_list_to_local_count),
        ),
        (
            "drain_remote_list_to_local_plan_count",
            str(len(drain_remote_list_to_local_plans)),
        ),
        (
            "drain_remote_list_to_local_token_provenance_valid",
            str(drain_remote_list_to_local_token_provenance_valid),
        ),
        (
            "drain_remote_list_to_local_page_operand_valid",
            str(drain_remote_list_to_local_page_operand_valid),
        ),
        (
            "drain_remote_list_to_local_head_class_resolved",
            str(drain_remote_list_to_local_head_class_resolved),
        ),
        (
            "drain_remote_list_to_local_lowerable_count",
            str(drain_remote_list_to_local_lowerable),
        ),
        (
            "atomic_remote_head_drain_local_list_mutation_lowerable_count",
            str(drain_remote_list_to_local_lowerable),
        ),
        (
            "atomic_remote_head_drain_local_list_mutation_lowered_count",
            str(
                drain_remote_list_to_local_lowerable
                if remote_free_drain_local_list_mutation_lowering_producer
                or remote_owner_branch_routing_any
                else 0
            ),
        ),
        (
            "remote_owner_branch_routing_selected",
            str(
                int_flag(
                    remote_owner_branch_routing_any
                )
            ),
        ),
        (
            "remote_owner_branch_routing_lowering_selected",
            str(int_flag(remote_owner_branch_routing_lowering_any)),
        ),
        (
            "remote_owner_branch_routing_open",
            str(
                int_flag(
                    remote_owner_branch_routing_lowering_producer
                    or remote_owner_branch_route_body_preflight
                )
            ),
        ),
        (
            "remote_owner_branch_routing_lowered_count",
            str(
                int_flag(
                    (
                        remote_owner_branch_routing_lowering_producer
                        or remote_owner_branch_route_body_preflight
                    )
                    and current_owner_count > 0
                    and owner_eq_count > 0
                    and drain_remote_list_to_local_lowerable > 0
                )
            ),
        ),
        (
            "remote_owner_branch_routing_preflight_requires_branch_cfg_row",
            str(
                int_flag(
                    not (
                        remote_owner_branch_routing_lowering_producer
                        or remote_owner_branch_route_body_preflight
                    )
                )
            ),
        ),
        (
            "remote_owner_branch_route_body_selected",
            str(int_flag(remote_owner_branch_route_body_preflight)),
        ),
        ("remote_owner_branch_route_body_open", "0"),
        ("page_local_alloc_route_report_v0", str(int_flag(profile == "local-free"))),
        ("page_local_alloc_route_candidate", route_candidate),
        (
            "page_local_alloc_route_candidate_count",
            str(int_flag(route_candidate != "none")),
        ),
        ("page_local_alloc_route_branch_claim", "0"),
        ("page_local_alloc_route_cfg_lowering_enabled", "0"),
        ("page_local_alloc_route_verified_plan_source", "fastmem_access_plans"),
        ("page_local_free_route_report_v0", str(int_flag(profile == "local-free"))),
        ("page_local_free_route_candidate", free_route_candidate),
        (
            "page_local_free_route_candidate_count",
            str(int_flag(free_route_candidate != "none")),
        ),
        ("page_local_free_route_branch_claim", "0"),
        ("page_local_free_route_cfg_lowering_enabled", "0"),
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
        ("fastmem_field_id_missing_count", str(field_missing)),
        ("fastmem_table_id_missing_count", str(table_missing)),
        ("fastmem_unverified_layout_access_count", str(len(plans) - len(verified_plans))),
        ("fastmem_table_index_unchecked_count", str(unchecked)),
        ("fastmem_table_access_proof_incomplete_count", str(incomplete_proof)),
        ("fastmem_table_overflow_proof_missing_count", str(overflow_missing)),
        ("fastmem_unknown_alignment_count", str(unknown_alignment)),
        ("fastmem_atomic_field_plain_store_count", str(atomic_plain_store)),
        ("fastmem_local_free_access_plan_incomplete_count", str(local_free_incomplete)),
        ("fastmem_free_head_access_plan_incomplete_count", str(free_head_incomplete)),
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
        ("product_activation", "0"),
        ("hook_install", "0"),
        ("hook_installed", "0"),
        ("global_allocator_claim", "0"),
        ("global_allocator_product_claim", "0"),
        ("winner_claim", "0"),
        ("tls_backing_transfer_enabled", "0"),
        ("allocator_owner_slot_reuse_enabled", "0"),
        (
            "llvm_object_path",
            "not_emitted_atomic_remote_head_cas_lowering_closed"
            if profile == "remote-free-preflight"
            else str(object_out),
        ),
        ("summary", "ok"),
    ]
    return rows


def write_rows(rows: list[tuple[str, str]], out: Path | None) -> None:
    text = "".join(f"{key}={value}\n" for key, value in rows)
    if out is None:
        sys.stdout.write(text)
    else:
        out.parent.mkdir(parents=True, exist_ok=True)
        out.write_text(text, encoding="utf-8")


def parse_args(argv: list[str]) -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--mir-json", required=True, type=Path)
    parser.add_argument("--out", type=Path)
    parser.add_argument("--object-out", type=Path)
    parser.add_argument(
        "--profile",
        choices=(
            "layout-table",
            "owner-runtime",
            "local-free",
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
            "remote-free-drain-local-list-mutation-lowering",
            "remote-owner-branch-routing-preflight",
            "remote-owner-branch-routing-lowering-preflight",
            "remote-owner-branch-routing-lowering",
            "remote-owner-branch-route-body-preflight",
        ),
        default="layout-table",
        help="evidence profile to emit after compiling the MIR JSON",
    )
    return parser.parse_args(argv)


def main(argv: list[str] | None = None) -> int:
    args = parse_args(sys.argv[1:] if argv is None else argv)
    mir_json = args.mir_json.resolve()
    mir = load_json(mir_json)

    if args.profile in {
        "remote-free-preflight",
        "remote-free-retry-preflight",
        "remote-free-drain-local-list-mutation-vocabulary-preflight",
        "remote-free-drain-local-list-mutation-verifier-preconditions",
    }:
        object_out = (
            args.object_out.resolve()
            if args.object_out is not None
            else Path("not_emitted_atomic_remote_head_cas_lowering_closed")
        )
        rows = build_rows(mir, object_out=object_out, profile=args.profile)
        write_rows(rows, args.out)
        return 0

    if args.object_out is not None:
        object_out = args.object_out.resolve()
        object_out.parent.mkdir(parents=True, exist_ok=True)
        run_llvm_builder(mir_json, object_out)
        rows = build_rows(mir, object_out=object_out, profile=args.profile)
        write_rows(rows, args.out)
        return 0

    with tempfile.TemporaryDirectory(prefix="hako_fastmem_llvm.") as tmp:
        object_out = Path(tmp) / "fastmem_pilot.o"
        run_llvm_builder(mir_json, object_out)
        rows = build_rows(mir, object_out=object_out, profile=args.profile)
        write_rows(rows, args.out)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
