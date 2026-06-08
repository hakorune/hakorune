#!/usr/bin/env python3
"""Check FastMemory capability inventory reports.

This is a verifier adapter over fastmem inventory fields. It fails when a
contract/runtime report contains unclassified MemOps, forbidden operations,
escaping memory values, or Type ABI / Provider ABI hot-path crossings.
"""

from __future__ import annotations

import argparse
import subprocess
import sys
from pathlib import Path
from fastmem_check_config import (
    FAIL_FIELDS,
    FAIL_STRING_FIELDS,
    LAYOUT_TABLE_PRODUCER_EXPECTED_POSITIVE,
    LAYOUT_TABLE_PRODUCER_EXPECTED_ZERO,
    LOCAL_FREE_PRODUCER_EXPECTED_POSITIVE,
    LOCAL_FREE_PRODUCER_EXPECTED_ZERO,
    OWNER_RUNTIME_PRODUCER_EXPECTED_POSITIVE,
    OWNER_RUNTIME_PRODUCER_EXPECTED_ZERO,
    PRODUCER_SLICE_EXPECTED_STRINGS,
    PRODUCER_SLICE_EXPECTED_ZERO,
)
from fastmem_route_profiles import (
    PAGE_LOCAL_ALLOC_ROUTE_CFG_PREFLIGHT_EXPECTED_POSITIVE,
    PAGE_LOCAL_ALLOC_ROUTE_CFG_PREFLIGHT_EXPECTED_ZERO,
    page_local_alloc_route_cfg_preflight_profile,
)
from fastmem_constants import (
    ALLOC_OWNER_ID_KIND_ARENA_OWNER,
    ALLOC_OWNER_ID_KIND_ALLOCATOR_ARENA_OWNER,
    ALLOC_OWNER_ID_REPR_PACKED_U64_SLOT_GENERATION,
    PAGE_OWNER_CHECK_ROUTE,
)
from report_kv import read_kv
from fastmem_check_atomic_rules import check_atomic_rules
from fastmem_check_profile_functions import *
from fastmem_check_route_rules import check_route_rules
from fastmem_check_terminal_rules import check_terminal_rules
from fastmem_check_output import emit_kv, render

ROOT = Path(__file__).resolve().parents[2]
INVENTORY = ROOT / "tools" / "hako_check" / "fastmem_capability_inventory.py"

def failure_reasons(rows: dict[str, str]) -> list[str]:
    reasons: list[str] = []
    input_kind = rows.get("input_kind", "")
    for key in FAIL_FIELDS:
        if int_count(rows, key) > 0:
            reasons.append(key)
    for key, forbidden_values in FAIL_STRING_FIELDS.items():
        if rows.get(key) in forbidden_values:
            reasons.append(key)
    if owner_state_profile(rows):
        if rows.get("alloc_owner_id_kind") != ALLOC_OWNER_ID_KIND_ALLOCATOR_ARENA_OWNER:
            reasons.append("alloc_owner_id_kind")
        if rows.get("worker_id_kind") != ALLOC_OWNER_ID_KIND_ALLOCATOR_ARENA_OWNER:
            reasons.append("worker_id_kind")
        if int_count(rows, "allocator_tls_arena_enabled") <= 0:
            reasons.append("allocator_tls_arena_enabled")
        if int_count(rows, "allocator_tls_arena_init_count") <= 0:
            reasons.append("allocator_tls_arena_init_count")
        if int_count(rows, "page_owner_check_enabled") <= 0:
            reasons.append("page_owner_check_enabled")
        if rows.get("page_owner_check_route") != PAGE_OWNER_CHECK_ROUTE:
            reasons.append("page_owner_check_route")
        if int_count(rows, "page_owner_check_count") <= 0:
            reasons.append("page_owner_check_count")
    if owner_lifecycle_profile(rows):
        if int_count(rows, "allocator_owner_lifecycle_state_machine") != 1:
            reasons.append("allocator_owner_lifecycle_state_machine")
        if int_count(rows, "allocator_owner_generation_enabled") != 1:
            reasons.append("allocator_owner_generation_enabled")
        if rows.get("allocator_owner_id_kind") != ALLOC_OWNER_ID_KIND_ARENA_OWNER:
            reasons.append("allocator_owner_id_kind")
        if rows.get("allocator_owner_id_repr") != ALLOC_OWNER_ID_REPR_PACKED_U64_SLOT_GENERATION:
            reasons.append("allocator_owner_id_repr")
        if int_count(rows, "allocator_owner_slot_bits") != 32:
            reasons.append("allocator_owner_slot_bits")
        if int_count(rows, "allocator_owner_generation_bits") != 32:
            reasons.append("allocator_owner_generation_bits")
        if int_count(rows, "allocator_owner_zero_is_invalid") != 1:
            reasons.append("allocator_owner_zero_is_invalid")
        if (
            int_count(rows, "allocator_abandoned_reclaim_success_count") > 0
            and int_count(rows, "remote_free_drain_supported") <= 0
        ):
            reasons.append("allocator_abandoned_reclaim_success_without_remote_drain")
        if (
            int_count(rows, "allocator_abandoned_reclaim_success_count") > 0
            and int_count(rows, "remote_candidate_unhandled_reclaim_block_count") > 0
        ):
            reasons.append("allocator_abandoned_reclaim_success_with_unhandled_remote")
    if input_kind in {"ast_json", "program_json_v0", "mir_json"}:
        for key in [
            "fastmem_source_dedicated_lowerer_enabled",
            "fastmem_source_dedicated_lowerer_transitional",
            "fastmem_source_dedicated_lowerer_retirement_required",
        ]:
            if int_count(rows, key) != 1:
                reasons.append(key)
        if int_count(rows, "fastmem_region_count") > 0:
            if int_count(rows, "fastmem_field_access_site_count") > 0:
                if int_count(rows, "field_access_required_verified_direct_count") <= 0:
                    reasons.append("field_access_required_verified_direct_count")
                if int_count(rows, "field_access_required_verified_direct_miss_count") != 0:
                    reasons.append("field_access_required_verified_direct_miss_count")
            if int_count(rows, "index_access_required_verified_table_count") <= 0:
                reasons.append("index_access_required_verified_table_count")
            if int_count(rows, "index_access_required_verified_table_miss_count") != 0:
                reasons.append("index_access_required_verified_table_miss_count")
            if int_count(rows, "fastmem_dedicated_branch_lowering_count") != 0:
                reasons.append("fastmem_dedicated_branch_lowering_count")
        branch_route_hits = int_count(rows, "fastmem_branch_condition_required_owner_eq_count") + int_count(
            rows, "fastmem_branch_condition_owner_eq_miss_count"
        )
        if branch_route_hits > 0:
            if int_count(rows, "fastmem_branch_condition_required_owner_eq_count") <= 0:
                reasons.append("fastmem_branch_condition_required_owner_eq_count")
            if int_count(rows, "fastmem_branch_condition_owner_eq_miss_count") != 0:
                reasons.append("fastmem_branch_condition_owner_eq_miss_count")
        numeric_route_hits = int_count(rows, "fastmem_numeric_verified_direct_count") + int_count(
            rows, "fastmem_numeric_required_route_miss_count"
        )
        if numeric_route_hits > 0:
            if int_count(rows, "fastmem_numeric_verified_direct_count") <= 0:
                reasons.append("fastmem_numeric_verified_direct_count")
            if int_count(rows, "fastmem_numeric_required_route_miss_count") != 0:
                reasons.append("fastmem_numeric_required_route_miss_count")
    if atomic_remote_profile(rows):
        if int_count(rows, "atomic_remote_head_plan") <= 0:
            reasons.append("atomic_remote_head_plan")
        if rows.get("atomic_remote_head_route") != "page_remote_head_cas":
            reasons.append("atomic_remote_head_route")
        if rows.get("remote_free_memory_order") not in {"acq_rel", "release_acquire"}:
            reasons.append("remote_free_memory_order")
        if int_count(rows, "remote_owner_free_remote_candidate_count") <= 0:
            reasons.append("remote_owner_free_remote_candidate_count")
        if int_count(rows, "remote_owner_free_remote_push_count") <= 0:
            reasons.append("remote_owner_free_remote_push_count")
        if int_count(rows, "remote_free_push_count") <= 0:
            reasons.append("remote_free_push_count")
        if int_count(rows, "remote_free_drain_count") <= 0:
            reasons.append("remote_free_drain_count")
    if safe_wrapper_profile(rows):
        if rows.get("safe_capability_wrapper_route") != "fastmem_memop_alias":
            reasons.append("safe_capability_wrapper_route")
        if rows.get("safe_capability_wrapper_lowering_route") != "fastmem_memop_alias":
            reasons.append("safe_capability_wrapper_lowering_route")
        if int_count(rows, "safe_capability_wrapper_memop_equivalence") <= 0:
            reasons.append("safe_capability_wrapper_memop_equivalence")
        for key in [
            "address_token_wrapper",
            "page_key_wrapper",
            "page_map_bridge_wrapper",
            "page_meta_handle_wrapper",
            "alloc_owner_id_wrapper",
            "atomic_remote_head_wrapper",
        ]:
            if int_count(rows, key) <= 0:
                reasons.append(key)
    if mimalloc_keeper_profile(rows):
        if int_count(rows, "mimalloc_shape_score") < int_count(rows, "mimalloc_shape_threshold"):
            reasons.append("mimalloc_shape_score")
        if int_count(rows, "mimalloc_safety_score") < int_count(rows, "mimalloc_safety_threshold"):
            reasons.append("mimalloc_safety_score")
        if int_count(rows, "mimalloc_coverage_score") < int_count(
            rows, "mimalloc_coverage_threshold"
        ):
            reasons.append("mimalloc_coverage_score")
        if int_count(rows, "mimalloc_keeper_eligible") <= 0:
            reasons.append("mimalloc_keeper_eligible")
        if rows.get("mimalloc_keeper_block_reason") != expected_mimalloc_keeper_block_reason(rows):
            reasons.append("mimalloc_keeper_block_reason")
    if product_shaped_bridge_profile(rows):
        if int_count(rows, "replacement_front_product_shaped_bridge_non_activating") != 1:
            reasons.append("replacement_front_product_shaped_bridge_non_activating")
        if int_count(rows, "replacement_front_product_shaped_bridge_report_only") != 1:
            reasons.append("replacement_front_product_shaped_bridge_report_only")
        if int_count(rows, "replacement_front_product_shaped_bridge_activation_ready") != 0:
            reasons.append("replacement_front_product_shaped_bridge_activation_ready")
        if int_count(rows, "product_activation_ready") != 0:
            reasons.append("product_activation_ready")
        if int_count(rows, "replacement_front_product_shaped_bridge_requires_activation_row") != 1:
            reasons.append("replacement_front_product_shaped_bridge_requires_activation_row")
        if int_count(rows, "replacement_front_product_shaped_bridge_requires_product_gate_open") != 1:
            reasons.append("replacement_front_product_shaped_bridge_requires_product_gate_open")
        missing = rows.get("replacement_front_product_shaped_bridge_missing", "")
        if "activation_row" not in missing:
            reasons.append("replacement_front_product_shaped_bridge_missing_activation_row")
        if "product_gate_open" not in missing:
            reasons.append("replacement_front_product_shaped_bridge_missing_product_gate_open")
        if int_count(rows, "replacement_front_product_shaped_bridge_evidence_ready") > 0:
            for key in [
                "replacement_front_product_shaped_bridge_shape_ok",
                "replacement_front_product_shaped_bridge_safety_ok",
                "replacement_front_product_shaped_bridge_coverage_ok",
                "replacement_front_product_shaped_bridge_preflight_ok",
                "replacement_front_product_shaped_bridge_no_type_abi_hot_lookup",
                "replacement_front_product_shaped_bridge_no_provider_dispatch",
                "replacement_front_product_shaped_bridge_no_global_lock_hot_path",
                "replacement_front_product_shaped_bridge_no_range_scan_hot_path",
                "replacement_front_product_shaped_bridge_no_host_passthrough",
            ]:
                if int_count(rows, key) != 1:
                    reasons.append(key)
            if rows.get("replacement_front_product_shaped_bridge_source_truth") != (
                "hako_alloc.size_class_box"
            ):
                reasons.append("replacement_front_product_shaped_bridge_source_truth")
            if rows.get("replacement_front_product_shaped_bridge_block_reason") != (
                "activation_row_required"
            ):
                reasons.append("replacement_front_product_shaped_bridge_block_reason")
    if size_class_bridge_profile(rows):
        if int_count(rows, "replacement_front_size_class_bridge_report_only") != 1:
            reasons.append("replacement_front_size_class_bridge_report_only")
        if rows.get("replacement_front_size_class_bridge_source_truth") != (
            "hako_alloc.size_class_box"
        ):
            reasons.append("replacement_front_size_class_bridge_source_truth")
        if rows.get("replacement_front_size_class_bridge_source_file") != (
            "lang/src/hako_alloc/memory/size_class_box.hako"
        ):
            reasons.append("replacement_front_size_class_bridge_source_file")
        if int_count(rows, "replacement_front_size_class_bridge_bound") != 1:
            reasons.append("replacement_front_size_class_bridge_bound")
        if rows.get("replacement_front_size_class_bridge_missing") != "none":
            reasons.append("replacement_front_size_class_bridge_missing")
        for key in [
            "replacement_front_size_class_required_methods_present",
            "replacement_front_size_class_usize_facades_present",
            "replacement_front_size_class_policy_methods_covered",
            "replacement_front_size_class_policy_constants_covered",
            "replacement_front_size_class_policy_huge_sentinel_covered",
            "replacement_front_size_class_policy_mirror_matches_source",
        ]:
            if int_count(rows, key) != 1:
                reasons.append(key)
        for key, expected in [
            ("replacement_front_size_class_word_size", 8),
            ("replacement_front_size_class_max_regular_bin", 72),
            ("replacement_front_size_class_huge_bin", 73),
            ("replacement_front_size_class_huge_sentinel", -1),
        ]:
            if int_count(rows, key) != expected:
                reasons.append(key)
    if page_local_bridge_profile(rows):
        if int_count(rows, "replacement_front_page_local_bridge_report_only") != 1:
            reasons.append("replacement_front_page_local_bridge_report_only")
        if rows.get("replacement_front_page_local_bridge_source_truth") != "hako_alloc.page_box":
            reasons.append("replacement_front_page_local_bridge_source_truth")
        if rows.get("replacement_front_page_local_bridge_source_file") != (
            "lang/src/hako_alloc/memory/page_box.hako"
        ):
            reasons.append("replacement_front_page_local_bridge_source_file")
        if int_count(rows, "replacement_front_page_local_bridge_bound") != 1:
            reasons.append("replacement_front_page_local_bridge_bound")
        if rows.get("replacement_front_page_local_bridge_missing") != "none":
            reasons.append("replacement_front_page_local_bridge_missing")
        for key in [
            "replacement_front_page_local_required_fields_present",
            "replacement_front_page_local_required_methods_present",
            "replacement_front_page_local_directarray_fields_present",
            "replacement_front_page_local_counter_fields_present",
            "replacement_front_page_local_acquire_release_methods_present",
            "replacement_front_page_local_lifecycle_methods_present",
            "replacement_front_page_local_typed_meta_matches_source",
            "replacement_front_page_local_same_owner_route_matches_source",
            "replacement_front_page_local_no_remote_free_claim",
        ]:
            if int_count(rows, key) != 1:
                reasons.append(key)
    if producer_taxonomy_profile(rows):
        producer = rows.get("replacement_front_producer", "unknown")
        if producer not in {
            "python_template_c_bridge",
            "mir_to_c_lowering",
            "mir_to_llvm_lowering",
        }:
            reasons.append("replacement_front_producer")
        if int_count(rows, "replacement_front_python_template_c_semantic_ssot") != 0:
            reasons.append("replacement_front_python_template_c_semantic_ssot")
        if int_count(rows, "replacement_front_mirbuilder_representation_only") != 1:
            reasons.append("replacement_front_mirbuilder_representation_only")
        if int_count(rows, "replacement_front_mirbuilder_route_decision_count") != 0:
            reasons.append("replacement_front_mirbuilder_route_decision_count")
        if producer == "python_template_c_bridge":
            if rows.get("replacement_front_backend_artifact") != "c":
                reasons.append("replacement_front_backend_artifact")
            if int_count(rows, "replacement_front_python_template_c_retirement_required") != 1:
                reasons.append("replacement_front_python_template_c_retirement_required")
            if int_count(rows, "replacement_front_mir_memop_enabled") != 0:
                reasons.append("replacement_front_mir_memop_enabled")
            if int_count(rows, "replacement_front_mir_fastmem_region_enabled") != 0:
                reasons.append("replacement_front_mir_fastmem_region_enabled")
            if rows.get("replacement_front_producer_transition_state") != "current_bridge":
                reasons.append("replacement_front_producer_transition_state")
        elif producer == "mir_to_c_lowering":
            if rows.get("replacement_front_backend_artifact") != "c":
                reasons.append("replacement_front_backend_artifact")
            if rows.get("replacement_front_producer_transition_state") != (
                "transition_backend_artifact"
            ):
                reasons.append("replacement_front_producer_transition_state")
        elif producer == "mir_to_llvm_lowering":
            if rows.get("replacement_front_backend_artifact") not in {
                "llvm_ir",
                "object",
                "exe",
            }:
                reasons.append("replacement_front_backend_artifact")
            if rows.get("replacement_front_producer_transition_state") != "final_primary":
                reasons.append("replacement_front_producer_transition_state")
    if producer_slice_selection_profile(rows):
        if int_count(rows, "replacement_front_producer_taxonomy_v0") != 1:
            reasons.append("replacement_front_producer_taxonomy_v0")
        for key, expected in PRODUCER_SLICE_EXPECTED_STRINGS.items():
            if rows.get(key) != expected:
                reasons.append(key)
        for key in PRODUCER_SLICE_EXPECTED_ZERO:
            if int_count(rows, key) != 0:
                reasons.append(key)
    if layout_table_producer_pilot_profile(rows):
        if not producer_slice_selection_profile(rows):
            reasons.append("replacement_front_producer_slice_selection_v0")
        if rows.get("replacement_front_selected_memop_kinds") != (
            "TableIndex,FieldLoad,FieldStore"
        ):
            reasons.append("replacement_front_selected_memop_kinds")
        if rows.get("replacement_front_deferred_memop_kinds") != "CurrentAllocOwnerId,OwnerEq":
            reasons.append("replacement_front_deferred_memop_kinds")
        for key in LAYOUT_TABLE_PRODUCER_EXPECTED_ZERO:
            if int_count(rows, key) != 0:
                reasons.append(key)
        if complete_layout_table_lowering_candidate(rows):
            for key in LAYOUT_TABLE_PRODUCER_EXPECTED_POSITIVE:
                if int_count(rows, key) <= 0:
                    reasons.append(key)
    if owner_runtime_producer_pilot_profile(rows):
        if rows.get("replacement_front_producer") != "mir_to_llvm_lowering":
            reasons.append("replacement_front_producer")
        if rows.get("fastmem_owner_runtime_current_owner_source") != (
            "llvm_producer_intrinsic"
        ):
            reasons.append("fastmem_owner_runtime_current_owner_source")
        if rows.get("replacement_front_selected_memop_family") != "owner_runtime":
            reasons.append("replacement_front_selected_memop_family")
        if rows.get("replacement_front_selected_memop_kinds") != (
            "CurrentAllocOwnerId,OwnerEq"
        ):
            reasons.append("replacement_front_selected_memop_kinds")
        for key in OWNER_RUNTIME_PRODUCER_EXPECTED_ZERO:
            if int_count(rows, key) != 0:
                reasons.append(key)
        for key in OWNER_RUNTIME_PRODUCER_EXPECTED_POSITIVE:
            if int_count(rows, key) <= 0:
                reasons.append(key)
    if local_free_producer_pilot_profile(rows):
        if rows.get("replacement_front_producer") != "mir_to_llvm_lowering":
            reasons.append("replacement_front_producer")
        if rows.get("replacement_front_selected_memop_family") != "local_free":
            reasons.append("replacement_front_selected_memop_family")
        selected_local_free = rows.get("replacement_front_selected_memop_kinds")
        selected_parts = set(filter(None, (selected_local_free or "").split(",")))
        allowed_local_free = {
            "LocalFreePush",
            "LocalFreePop",
            "FreeHeadPush",
            "FreeHeadPop",
        }
        if (
            not selected_parts
            or selected_local_free == "none"
            or not selected_parts.issubset(allowed_local_free)
        ):
            reasons.append("replacement_front_selected_memop_kinds")
        deferred_local_free = rows.get("replacement_front_deferred_memop_kinds", "")
        if "AtomicRemoteHead" not in deferred_local_free.split(","):
            reasons.append("replacement_front_deferred_memop_kinds")
        for key in LOCAL_FREE_PRODUCER_EXPECTED_ZERO:
            if int_count(rows, key) != 0:
                reasons.append(key)
        for key in LOCAL_FREE_PRODUCER_EXPECTED_POSITIVE:
            if int_count(rows, key) <= 0:
                reasons.append(key)
        if selected_local_free and "LocalFreePush" in selected_local_free:
            for key in (
                "fastmem_local_free_push_plan_count",
                "memop_local_free_push_lowered_count",
                "memop_local_free_push_layout_ref_consumed_count",
                "fastmem_local_free_push_lowering_uses_verified_plan",
            ):
                if int_count(rows, key) <= 0:
                    reasons.append(key)
        if selected_local_free and "LocalFreePop" in selected_local_free:
            for key in (
                "fastmem_local_free_pop_plan_count",
                "memop_local_free_pop_lowered_count",
                "memop_local_free_pop_layout_ref_consumed_count",
                "fastmem_local_free_pop_lowering_uses_verified_plan",
                "fastmem_local_free_pop_lowering_enabled",
            ):
                if int_count(rows, key) <= 0:
                    reasons.append(key)
        if selected_local_free and "FreeHeadPush" in selected_local_free:
            for key in (
                "fastmem_free_head_push_plan_count",
                "memop_free_head_push_lowered_count",
                "memop_free_head_push_layout_ref_consumed_count",
                "fastmem_free_head_push_lowering_uses_verified_plan",
                "fastmem_free_head_push_lowering_enabled",
            ):
                if int_count(rows, key) <= 0:
                    reasons.append(key)
        if selected_local_free and "FreeHeadPop" in selected_local_free:
            for key in (
                "fastmem_free_head_pop_plan_count",
                "memop_free_head_pop_lowered_count",
                "memop_free_head_pop_layout_ref_consumed_count",
                "fastmem_free_head_pop_lowering_uses_verified_plan",
                "fastmem_free_head_pop_lowering_enabled",
            ):
                if int_count(rows, key) <= 0:
                    reasons.append(key)
    reasons.extend(check_atomic_rules(rows))
    reasons.extend(check_route_rules(rows))
    reasons.extend(check_terminal_rules(rows))
    return reasons


def write_output(text: str, out: Path | None) -> None:
    if out is None:
        print(text, end="")
        return
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text(text, encoding="utf-8")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    source = parser.add_mutually_exclusive_group(required=True)
    source.add_argument("--report", type=Path, help="Read a benchmark report via inventory.")
    source.add_argument("--inventory", type=Path, help="Read an existing fastmem inventory kv file.")
    source.add_argument("--ast-json", type=Path, help="Read Rust AST JSON via inventory.")
    source.add_argument("--program-json", type=Path, help="Read Program(JSON v0) via inventory.")
    source.add_argument("--mir-json", type=Path, help="Read MIR JSON via inventory.")
    parser.add_argument("--format", choices=("kv", "text"), default="text")
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    if args.report:
        rows = run_inventory("--report", args.report)
    elif args.ast_json:
        rows = run_inventory("--ast-json", args.ast_json)
    elif args.program_json:
        rows = run_inventory("--program-json", args.program_json)
    elif args.mir_json:
        rows = run_inventory("--mir-json", args.mir_json)
    else:
        rows = read_kv(args.inventory)
    reasons = failure_reasons(rows)
    text = emit_kv(rows, reasons) if args.format == "kv" else render(rows, reasons)
    write_output(text, args.out)
    return 1 if reasons else 0


if __name__ == "__main__":
    raise SystemExit(main())
