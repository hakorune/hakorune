#!/usr/bin/env python3
"""Shared FastMemory verifier profile helpers."""

from __future__ import annotations

from pathlib import Path
import subprocess
import sys
from typing import Any

ROOT = Path(__file__).resolve().parents[2]
INVENTORY = ROOT / "tools" / "hako_check" / "fastmem_capability_inventory.py"

def int_count(rows: dict[str, Any], key: str) -> int:
    value = rows.get(key, "0")
    try:
        return int(float(str(value)))
    except (TypeError, ValueError):
        return 0


def owner_state_profile(rows: dict[str, str]) -> bool:
    return (
        int_count(rows, "alloc_owner_id_capability") > 0
        or int_count(rows, "worker_id_capability") > 0
        or int_count(rows, "page_owner_check_enabled") > 0
    )


def atomic_remote_profile(rows: dict[str, str]) -> bool:
    return (
        int_count(rows, "atomic_remote_head_pilot_enabled") > 0
        or int_count(rows, "atomic_remote_head_enabled") > 0
    )


def owner_lifecycle_profile(rows: dict[str, str]) -> bool:
    return (
        int_count(rows, "allocator_owner_lifecycle_state_machine") > 0
        or int_count(rows, "allocator_owner_exiting_flush_count") > 0
        or int_count(rows, "allocator_owner_abandoned_count") > 0
        or int_count(rows, "allocator_owner_reclaimed_count") > 0
        or int_count(rows, "allocator_thread_exit_observed_count") > 0
        or int_count(rows, "allocator_abandoned_reclaim_attempt_count") > 0
    )


def safe_wrapper_profile(rows: dict[str, str]) -> bool:
    return int_count(rows, "safe_capability_wrapper_plan") > 0


def mimalloc_keeper_profile(rows: dict[str, str]) -> bool:
    return int_count(rows, "mimalloc_keeper_candidate") > 0


def product_shaped_bridge_profile(rows: dict[str, str]) -> bool:
    return int_count(rows, "replacement_front_product_shaped_bridge_v0") > 0


def size_class_bridge_profile(rows: dict[str, str]) -> bool:
    return int_count(rows, "replacement_front_size_class_bridge_v0") > 0


def page_local_bridge_profile(rows: dict[str, str]) -> bool:
    return int_count(rows, "replacement_front_page_local_bridge_v0") > 0


def producer_taxonomy_profile(rows: dict[str, str]) -> bool:
    return int_count(rows, "replacement_front_producer_taxonomy_v0") > 0


def producer_slice_selection_profile(rows: dict[str, str]) -> bool:
    return int_count(rows, "replacement_front_producer_slice_selection_v0") > 0


def layout_table_producer_pilot_profile(rows: dict[str, str]) -> bool:
    return int_count(rows, "mir_fmem_008b_layout_table_producer_pilot") > 0


def owner_runtime_producer_pilot_profile(rows: dict[str, str]) -> bool:
    return int_count(rows, "fastmem_owner_runtime_producer_pilot") > 0


def local_free_producer_pilot_profile(rows: dict[str, str]) -> bool:
    return int_count(rows, "fastmem_local_free_producer_pilot") > 0


def atomic_remote_head_cas_preflight_profile(rows: dict[str, str]) -> bool:
    return int_count(rows, "fastmem_atomic_remote_head_cas_preflight") > 0


def atomic_remote_head_cas_producer_profile(rows: dict[str, str]) -> bool:
    return int_count(rows, "fastmem_atomic_remote_head_cas_producer_pilot") > 0


def atomic_remote_head_retry_preflight_profile(rows: dict[str, str]) -> bool:
    return int_count(rows, "fastmem_atomic_remote_head_retry_preflight") > 0


def atomic_remote_head_retry_producer_profile(rows: dict[str, str]) -> bool:
    return int_count(rows, "fastmem_atomic_remote_head_retry_producer_pilot") > 0


def atomic_remote_head_drain_preflight_profile(rows: dict[str, str]) -> bool:
    return int_count(rows, "fastmem_atomic_remote_head_drain_preflight") > 0


def atomic_remote_head_drain_exchange_selection_profile(rows: dict[str, str]) -> bool:
    return int_count(rows, "fastmem_atomic_remote_head_drain_exchange_selection") > 0


def atomic_remote_head_drain_exchange_producer_profile(rows: dict[str, str]) -> bool:
    return int_count(rows, "fastmem_atomic_remote_head_drain_exchange_producer_pilot") > 0


def atomic_remote_head_drain_to_local_selection_profile(rows: dict[str, str]) -> bool:
    return int_count(rows, "fastmem_atomic_remote_head_drain_to_local_route_selection") > 0


def atomic_remote_head_drain_to_local_producer_profile(rows: dict[str, str]) -> bool:
    return int_count(rows, "fastmem_atomic_remote_head_drain_to_local_route_producer_pilot") > 0


def atomic_remote_head_drain_local_list_mutation_preflight_profile(
    rows: dict[str, str],
) -> bool:
    return (
        int_count(
            rows,
            "fastmem_atomic_remote_head_drain_local_list_mutation_preflight",
        )
        > 0
    )


def atomic_remote_head_drain_local_list_mutation_proof_profile(
    rows: dict[str, str],
) -> bool:
    return (
        int_count(
            rows,
            "fastmem_atomic_remote_head_drain_local_list_mutation_proof",
        )
        > 0
    )


def atomic_remote_head_drain_local_list_mutation_vocabulary_profile(
    rows: dict[str, str],
) -> bool:
    return (
        int_count(
            rows,
            "fastmem_atomic_remote_head_drain_local_list_mutation_vocabulary_preflight",
        )
        > 0
    )


def atomic_remote_head_drain_local_list_mutation_verifier_profile(
    rows: dict[str, str],
) -> bool:
    return (
        int_count(
            rows,
            "fastmem_atomic_remote_head_drain_local_list_mutation_verifier_preconditions",
        )
        > 0
    )


def atomic_remote_head_drain_local_list_mutation_lowering_profile(
    rows: dict[str, str],
) -> bool:
    return (
        int_count(
            rows,
            "fastmem_atomic_remote_head_drain_local_list_mutation_lowering_producer_pilot",
        )
        > 0
    )


def remote_owner_branch_routing_preflight_profile(rows: dict[str, str]) -> bool:
    return int_count(rows, "fastmem_remote_owner_branch_routing_preflight") > 0


def remote_owner_branch_routing_lowering_preflight_profile(rows: dict[str, str]) -> bool:
    return int_count(rows, "fastmem_remote_owner_branch_routing_lowering_preflight") > 0


def remote_owner_branch_routing_lowering_profile(rows: dict[str, str]) -> bool:
    return int_count(rows, "fastmem_remote_owner_branch_routing_lowering_producer_pilot") > 0


def remote_owner_branch_route_body_preflight_profile(rows: dict[str, str]) -> bool:
    return int_count(rows, "fastmem_remote_owner_branch_route_body_preflight") > 0


def fastmem_branch_cfg_preflight_profile(rows: dict[str, str]) -> bool:
    return int_count(rows, "fastmem_branch_cfg_preflight") > 0


def fastmem_branch_cfg_lowering_preflight_profile(rows: dict[str, str]) -> bool:
    return int_count(rows, "fastmem_branch_cfg_lowering_preflight") > 0


def fastmem_branch_cfg_lowering_profile(rows: dict[str, str]) -> bool:
    return int_count(rows, "fastmem_branch_cfg_lowering_producer_pilot") > 0


def same_remote_free_body_preflight_profile(rows: dict[str, str]) -> bool:
    return int_count(rows, "fastmem_same_remote_free_body_preflight") > 0


def same_remote_free_body_producer_profile(rows: dict[str, str]) -> bool:
    return int_count(rows, "fastmem_same_remote_free_body_producer_pilot") > 0


def page_local_free_route_cfg_preflight_profile(rows: dict[str, str]) -> bool:
    return int_count(rows, "fastmem_page_local_free_route_cfg_preflight") > 0


def page_local_free_route_cfg_producer_profile(rows: dict[str, str]) -> bool:
    return int_count(rows, "fastmem_page_local_free_route_cfg_producer_pilot") > 0


def terminal_ladder_refresh_preflight_profile(rows: dict[str, str]) -> bool:
    return int_count(rows, "fastmem_terminal_ladder_refresh_preflight") > 0


def tls_backing_transfer_preflight_refresh_profile(rows: dict[str, str]) -> bool:
    return int_count(rows, "fastmem_tls_backing_transfer_preflight_refresh") > 0


def tls_backing_transfer_producer_refresh_profile(rows: dict[str, str]) -> bool:
    return int_count(rows, "fastmem_tls_backing_transfer_producer_refresh") > 0


def tls_backing_transfer_preflight_profile(rows: dict[str, str]) -> bool:
    return int_count(rows, "fastmem_tls_backing_transfer_preflight") > 0


def tls_backing_transfer_producer_profile(rows: dict[str, str]) -> bool:
    return int_count(rows, "fastmem_tls_backing_transfer_producer_pilot") > 0


def owner_slot_reuse_preflight_refresh_profile(rows: dict[str, str]) -> bool:
    return int_count(rows, "fastmem_allocator_owner_slot_reuse_preflight_refresh") > 0


def owner_slot_reuse_producer_refresh_profile(rows: dict[str, str]) -> bool:
    return int_count(rows, "fastmem_allocator_owner_slot_reuse_producer_refresh") > 0


def owner_slot_reuse_preflight_profile(rows: dict[str, str]) -> bool:
    return int_count(rows, "fastmem_allocator_owner_slot_reuse_preflight") > 0


def owner_slot_reuse_producer_profile(rows: dict[str, str]) -> bool:
    return int_count(rows, "fastmem_allocator_owner_slot_reuse_producer_pilot") > 0


def abandoned_reclaim_preflight_profile(rows: dict[str, str]) -> bool:
    return int_count(rows, "fastmem_abandoned_reclaim_preflight") > 0


def abandoned_reclaim_preflight_refresh_profile(rows: dict[str, str]) -> bool:
    return int_count(rows, "fastmem_abandoned_reclaim_preflight_refresh") > 0


def abandoned_reclaim_producer_refresh_profile(rows: dict[str, str]) -> bool:
    return int_count(rows, "fastmem_abandoned_reclaim_producer_refresh") > 0


def abandoned_reclaim_producer_profile(rows: dict[str, str]) -> bool:
    return int_count(rows, "fastmem_abandoned_reclaim_producer_pilot") > 0


def product_activation_preflight_profile(rows: dict[str, str]) -> bool:
    return int_count(rows, "fastmem_product_activation_preflight") > 0


def product_activation_preflight_refresh_profile(rows: dict[str, str]) -> bool:
    return int_count(rows, "fastmem_product_activation_preflight_refresh") > 0


def product_activation_producer_profile(rows: dict[str, str]) -> bool:
    return int_count(rows, "fastmem_product_activation_producer_pilot") > 0


def product_activation_producer_refresh_profile(rows: dict[str, str]) -> bool:
    return int_count(rows, "fastmem_product_activation_producer_refresh") > 0


def hook_install_preflight_profile(rows: dict[str, str]) -> bool:
    return int_count(rows, "fastmem_hook_install_preflight") > 0


def hook_install_preflight_refresh_profile(rows: dict[str, str]) -> bool:
    return int_count(rows, "fastmem_hook_install_preflight_refresh") > 0


def hook_install_producer_refresh_profile(rows: dict[str, str]) -> bool:
    return int_count(rows, "fastmem_hook_install_producer_refresh") > 0


def hook_install_producer_profile(rows: dict[str, str]) -> bool:
    return int_count(rows, "fastmem_hook_install_producer_pilot") > 0


def global_allocator_claim_preflight_profile(rows: dict[str, str]) -> bool:
    return int_count(rows, "fastmem_global_allocator_claim_preflight") > 0


def global_allocator_claim_producer_profile(rows: dict[str, str]) -> bool:
    return int_count(rows, "fastmem_global_allocator_claim_producer_pilot") > 0


def winner_claim_preflight_profile(rows: dict[str, str]) -> bool:
    return int_count(rows, "fastmem_winner_claim_preflight") > 0


def winner_claim_producer_profile(rows: dict[str, str]) -> bool:
    return int_count(rows, "fastmem_winner_claim_producer_pilot") > 0


LAYOUT_TABLE_PRODUCER_EXPECTED_ZERO = (
    "memop_current_alloc_owner_id_lowered_count",
    "memop_owner_eq_lowered_count",
    "memop_atomic_remote_head_lowered_count",
    "fastmem_field_id_missing_count",
    "fastmem_table_id_missing_count",
    "fastmem_unverified_layout_access_count",
    "fastmem_table_index_unchecked_count",
    "fastmem_table_access_proof_incomplete_count",
    "fastmem_table_overflow_proof_missing_count",
    "fastmem_unknown_alignment_count",
    "fastmem_atomic_field_plain_store_count",
    "fastmem_layout_ref_escape_count",
    "fastmem_lowering_recomputed_layout_offset_count",
)


def complete_layout_table_lowering_candidate(rows: dict[str, str]) -> bool:
    if not layout_table_producer_pilot_profile(rows):
        return False
    if rows.get("replacement_front_producer") != "mir_to_llvm_lowering":
        return False
    if int_count(rows, "fastmem_verified_mem_access_plan_count") <= 0:
        return False
    return all(int_count(rows, key) == 0 for key in LAYOUT_TABLE_PRODUCER_EXPECTED_ZERO)


def expected_mimalloc_keeper_block_reason(rows: dict[str, str]) -> str:
    if int_count(rows, "mimalloc_shape_score") < int_count(rows, "mimalloc_shape_threshold"):
        return "shape_below_threshold"
    if int_count(rows, "mimalloc_safety_score") < int_count(rows, "mimalloc_safety_threshold"):
        return "safety_below_threshold"
    if int_count(rows, "mimalloc_coverage_score") < int_count(rows, "mimalloc_coverage_threshold"):
        return "coverage_below_threshold"
    return "eligible"


def run_inventory(source_flag: str, source_path: Path) -> dict[str, str]:
    cmd = [sys.executable, str(INVENTORY), source_flag, str(source_path)]
    proc = subprocess.run(cmd, check=False, text=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    if proc.returncode != 0:
        if proc.stderr:
            sys.stderr.write(proc.stderr)
        if proc.stdout:
            sys.stderr.write(proc.stdout)
        raise SystemExit(proc.returncode)
    rows: dict[str, str] = {}
    for raw_line in proc.stdout.splitlines():
        line = raw_line.strip()
        if not line or line.startswith("#") or "=" not in line:
            continue
        key, value = line.split("=", 1)
        rows[key.strip()] = value.strip()
    return rows
