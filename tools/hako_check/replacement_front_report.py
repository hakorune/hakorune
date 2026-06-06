#!/usr/bin/env python3
"""Explain replacement-front benchmark reports without changing execution."""

from __future__ import annotations

import argparse
import json
import re
from pathlib import Path
from typing import Any

from report_kv import (
    find_subject,
    first_value,
    float_value,
    int_value,
    prefixed,
    prefixed_float,
    prefixed_int,
    ratio,
    read_kv,
    subject_indices,
)

REPO_ROOT = Path(__file__).resolve().parents[2]
SIZE_CLASS_BOX_SOURCE = Path("lang/src/hako_alloc/memory/size_class_box.hako")
SIZE_CLASS_REQUIRED_METHODS = (
    "word_size",
    "max_regular_bin",
    "huge_bin",
    "normalize_size",
    "bin_size",
    "size_to_bin",
    "size_to_bin_usize",
    "good_size",
    "good_size_usize",
    "bin_size_usize",
    "accepts",
    "accepts_usize",
)
SIZE_CLASS_USIZE_FACADES = (
    "size_to_bin_usize",
    "good_size_usize",
    "bin_size_usize",
    "accepts_usize",
)
PAGE_BOX_SOURCE = Path("lang/src/hako_alloc/memory/page_box.hako")
PAGE_LOCAL_REQUIRED_FIELDS = (
    "free",
    "local_free",
    "block_used",
    "used",
    "free_top",
    "local_free_top",
    "capacity",
    "reserved",
    "block_size",
)
PAGE_LOCAL_DIRECTARRAY_FIELDS = ("free", "local_free", "block_used")
PAGE_LOCAL_COUNTER_FIELDS = (
    "used",
    "free_top",
    "local_free_top",
    "capacity",
    "reserved",
    "block_size",
)
PAGE_LOCAL_REQUIRED_METHODS = (
    "birth",
    "seedFreeBlocks",
    "acquire",
    "acquire_usize",
    "releaseLocal",
    "releaseLocalKnownLive",
    "reactivate",
    "freeCount",
    "localFreeCount",
    "availableBlockCount",
)
PAGE_LOCAL_ACQUIRE_RELEASE_METHODS = (
    "acquire",
    "releaseLocal",
    "releaseLocalKnownLive",
)
PAGE_LOCAL_LIFECYCLE_METHODS = (
    "reactivate",
    "isRetired",
    "isDecommitted",
    "canReuse",
    "reuse",
)
REPLACEMENT_FRONT_PRODUCERS = {
    "python_template_c_bridge",
    "mir_to_c_lowering",
    "mir_to_llvm_lowering",
}
REPLACEMENT_FRONT_BACKEND_ARTIFACTS = {"c", "llvm_ir", "object", "exe"}
REPLACEMENT_FRONT_SOURCE_TRUTHS = {
    "hako_fastmem",
    "hako_alloc.size_class_box",
    "hako_alloc.page_box",
    "unknown",
}
PRODUCER_SLICE_DEFAULTS = {
    "replacement_front_producer_slice_selection_v0": 1,
    "replacement_front_next_producer_slice": "layout_table_producer_pilot",
    "replacement_front_selected_memop_family": "layout_table",
    "replacement_front_selected_memop_kinds": "TableIndex,FieldLoad,FieldStore",
    "replacement_front_deferred_memop_family": "owner_runtime",
    "replacement_front_deferred_memop_kinds": "CurrentAllocOwnerId,OwnerEq",
    "replacement_front_selection_behavior_change": 0,
    "replacement_front_selection_product_activation": 0,
    "replacement_front_selection_bridge_retirement_allowed": 0,
}

def page_lookup_route(rows: dict[str, str], subject_idx: int, report: dict[str, Any]) -> str:
    lookup_route = prefixed(rows, subject_idx, "replacement_front_page_bins_lookup_route")
    page_from_ptr_route = prefixed(rows, subject_idx, "replacement_front_page_from_ptr_route")
    if lookup_route == "range_scan" or report["page_from_ptr_range_scan_count_total"] > 0:
        return "range_scan"
    if lookup_route in {"page_from_ptr_bridge", "indexed_page_table", "page_map_lookup"}:
        return "page_map_bridge"
    if page_from_ptr_route in {"side_table_direct", "page_base_mask", "header_backptr"}:
        return "page_map_bridge"
    if report["page_index_probe_count_total"] > 0 or report["page_from_ptr_count_total"] > 0:
        return "page_index_side_table"
    return "unknown"


def page_map_bridge_kind(rows: dict[str, str], subject_idx: int) -> str:
    page_from_ptr_route = prefixed(rows, subject_idx, "replacement_front_page_from_ptr_route")
    lookup_route = prefixed(rows, subject_idx, "replacement_front_page_bins_lookup_route")
    if page_from_ptr_route == "page_base_mask":
        return "page_base_mask"
    if page_from_ptr_route == "header_backptr":
        return "header_backptr"
    if page_from_ptr_route == "side_table_direct":
        return "flat_side_table"
    if lookup_route in {"indexed_page_table", "page_map_lookup", "page_from_ptr_bridge"}:
        return "flat_side_table"
    return "none"


def normalized_product_bridge_source(source: str) -> str:
    if source in {"hako_alloc.size_class_box", "hako_size_class_box_report_mirror"}:
        return "hako_alloc.size_class_box"
    return "unknown"


def normalized_page_local_bridge_source(source: str) -> str:
    if source in {"hako_alloc.page_box", "hako_page_box_report_mirror"}:
        return "hako_alloc.page_box"
    return "unknown"


def normalized_replacement_front_producer(value: str, front_class: str) -> str:
    if value in REPLACEMENT_FRONT_PRODUCERS:
        return value
    return "unknown"


def normalized_backend_artifact(value: str, producer: str) -> str:
    if value in REPLACEMENT_FRONT_BACKEND_ARTIFACTS:
        return value
    if producer in {"python_template_c_bridge", "mir_to_c_lowering"}:
        return "c"
    if producer == "mir_to_llvm_lowering":
        return "object"
    return "unknown"


def normalized_replacement_front_source_truth(value: str, fallback: str) -> str:
    source = value or fallback
    if source in REPLACEMENT_FRONT_SOURCE_TRUTHS:
        return source
    return "unknown"


def producer_transition_state(producer: str) -> str:
    if producer == "python_template_c_bridge":
        return "current_bridge"
    if producer == "mir_to_c_lowering":
        return "transition_backend_artifact"
    if producer == "mir_to_llvm_lowering":
        return "final_primary"
    return "unknown"


def method_names(source: str) -> set[str]:
    return set(re.findall(r"^\s*([A-Za-z_][A-Za-z0-9_]*)\s*\(", source, re.MULTILINE))


def field_names(source: str) -> set[str]:
    return set(re.findall(r"^\s*([A-Za-z_][A-Za-z0-9_]*)\s*:", source, re.MULTILINE))


def directarray_field_names(source: str) -> set[str]:
    return set(
        re.findall(
            r"^\s*([A-Za-z_][A-Za-z0-9_]*)\s*:\s*DirectArrayI64\b",
            source,
            re.MULTILINE,
        )
    )


def source_int_constant(source: str, method: str) -> int | None:
    pattern = rf"\b{re.escape(method)}\s*\(\)\s*\{{\s*return\s+(-?\d+)\b"
    match = re.search(pattern, source, re.MULTILINE)
    if match is None:
        return None
    return int(match.group(1))


def size_class_box_evidence(mirror_source: str) -> dict[str, Any]:
    source_path = REPO_ROOT / SIZE_CLASS_BOX_SOURCE
    source_text = (
        source_path.read_text(encoding="utf-8", errors="replace")
        if source_path.is_file()
        else ""
    )
    methods = method_names(source_text)
    missing_methods = [
        method for method in SIZE_CLASS_REQUIRED_METHODS if method not in methods
    ]
    word_size = source_int_constant(source_text, "word_size")
    max_regular_bin = source_int_constant(source_text, "max_regular_bin")
    huge_bin = source_int_constant(source_text, "huge_bin")
    huge_sentinel = -1 if re.search(r"\breturn\s+-1\b", source_text) else 0
    methods_present = int(not missing_methods and source_path.is_file())
    constants_covered = int(word_size == 8 and max_regular_bin == 72 and huge_bin == 73)
    huge_sentinel_covered = int(huge_sentinel == -1)
    usize_facades_present = int(all(method in methods for method in SIZE_CLASS_USIZE_FACADES))
    source_truth = normalized_product_bridge_source(mirror_source)
    mirror_matches_source = int(source_truth == "hako_alloc.size_class_box")
    missing_parts = [
        part
        for part, missing in [
            ("source_file", not source_path.is_file()),
            ("methods", not methods_present),
            ("constants", not constants_covered),
            ("huge_sentinel", not huge_sentinel_covered),
            ("usize_facades", not usize_facades_present),
            ("mirror_source", not mirror_matches_source),
        ]
        if missing
    ]
    bridge_enabled = int(bool(mirror_source))
    return {
        "replacement_front_size_class_bridge_v0": bridge_enabled,
        "replacement_front_size_class_bridge_report_only": 1,
        "replacement_front_size_class_bridge_source_truth": source_truth,
        "replacement_front_size_class_bridge_source_file": str(SIZE_CLASS_BOX_SOURCE),
        "replacement_front_size_class_bridge_mirror_source": mirror_source or "unknown",
        "replacement_front_size_class_bridge_bound": int(bridge_enabled and not missing_parts),
        "replacement_front_size_class_bridge_missing": (
            ",".join(missing_parts) if missing_parts else "none"
        ),
        "replacement_front_size_class_required_method_count": len(
            SIZE_CLASS_REQUIRED_METHODS
        ),
        "replacement_front_size_class_required_methods_present": methods_present,
        "replacement_front_size_class_missing_methods": (
            ",".join(missing_methods) if missing_methods else "none"
        ),
        "replacement_front_size_class_word_size": word_size or 0,
        "replacement_front_size_class_max_regular_bin": max_regular_bin or 0,
        "replacement_front_size_class_huge_bin": huge_bin or 0,
        "replacement_front_size_class_huge_sentinel": huge_sentinel,
        "replacement_front_size_class_usize_facades_present": usize_facades_present,
        "replacement_front_size_class_policy_methods_covered": methods_present,
        "replacement_front_size_class_policy_constants_covered": constants_covered,
        "replacement_front_size_class_policy_huge_sentinel_covered": (
            huge_sentinel_covered
        ),
        "replacement_front_size_class_policy_mirror_matches_source": (
            mirror_matches_source
        ),
    }


def page_local_bridge_evidence(
    mirror_source: str,
    report: dict[str, Any],
) -> dict[str, Any]:
    source_path = REPO_ROOT / PAGE_BOX_SOURCE
    source_text = (
        source_path.read_text(encoding="utf-8", errors="replace")
        if source_path.is_file()
        else ""
    )
    fields = field_names(source_text)
    directarray_fields = directarray_field_names(source_text)
    methods = method_names(source_text)
    missing_fields = [
        field for field in PAGE_LOCAL_REQUIRED_FIELDS if field not in fields
    ]
    missing_methods = [
        method for method in PAGE_LOCAL_REQUIRED_METHODS if method not in methods
    ]
    directarray_present = int(
        all(field in directarray_fields for field in PAGE_LOCAL_DIRECTARRAY_FIELDS)
    )
    counter_fields_present = int(
        all(field in fields for field in PAGE_LOCAL_COUNTER_FIELDS)
    )
    acquire_release_present = int(
        all(method in methods for method in PAGE_LOCAL_ACQUIRE_RELEASE_METHODS)
    )
    lifecycle_present = int(
        all(method in methods for method in PAGE_LOCAL_LIFECYCLE_METHODS)
    )
    fields_present = int(not missing_fields and source_path.is_file())
    methods_present = int(not missing_methods and source_path.is_file())
    typed_meta_matches = int(
        report["page_map_bridge_benchmark_front_pilot"]
        and report.get("typed_page_meta_field_block_size", 0) == 1
        and report.get("typed_page_meta_field_free_head", 0) == 1
        and report.get("typed_page_meta_field_local_free_head", 0) == 1
        and report.get("typed_page_meta_field_capacity", 0) == 1
        and report.get("typed_page_meta_field_used", 0) == 1
    )
    same_owner_matches = int(
        report["same_thread_free_local_count_total"] > 0
        and report["same_thread_alloc_local_count_total"] > 0
        and report["global_lock_hot_path_count_total"] == 0
    )
    source_truth = normalized_page_local_bridge_source(mirror_source)
    mirror_matches_source = int(source_truth == "hako_alloc.page_box")
    missing_parts = [
        part
        for part, missing in [
            ("source_file", not source_path.is_file()),
            ("fields", not fields_present),
            ("methods", not methods_present),
            ("directarray_fields", not directarray_present),
            ("counter_fields", not counter_fields_present),
            ("acquire_release", not acquire_release_present),
            ("lifecycle", not lifecycle_present),
            ("typed_meta", not typed_meta_matches),
            ("same_owner_route", not same_owner_matches),
            ("mirror_source", not mirror_matches_source),
        ]
        if missing
    ]
    bridge_enabled = int(bool(mirror_source))
    return {
        "replacement_front_page_local_bridge_v0": bridge_enabled,
        "replacement_front_page_local_bridge_report_only": 1,
        "replacement_front_page_local_bridge_source_truth": source_truth,
        "replacement_front_page_local_bridge_source_file": str(PAGE_BOX_SOURCE),
        "replacement_front_page_local_bridge_mirror_source": mirror_source or "unknown",
        "replacement_front_page_local_bridge_bound": int(bridge_enabled and not missing_parts),
        "replacement_front_page_local_bridge_missing": (
            ",".join(missing_parts) if missing_parts else "none"
        ),
        "replacement_front_page_local_required_field_count": len(
            PAGE_LOCAL_REQUIRED_FIELDS
        ),
        "replacement_front_page_local_required_fields_present": fields_present,
        "replacement_front_page_local_missing_fields": (
            ",".join(missing_fields) if missing_fields else "none"
        ),
        "replacement_front_page_local_required_method_count": len(
            PAGE_LOCAL_REQUIRED_METHODS
        ),
        "replacement_front_page_local_required_methods_present": methods_present,
        "replacement_front_page_local_missing_methods": (
            ",".join(missing_methods) if missing_methods else "none"
        ),
        "replacement_front_page_local_directarray_fields_present": directarray_present,
        "replacement_front_page_local_counter_fields_present": counter_fields_present,
        "replacement_front_page_local_acquire_release_methods_present": acquire_release_present,
        "replacement_front_page_local_lifecycle_methods_present": lifecycle_present,
        "replacement_front_page_local_typed_meta_matches_source": typed_meta_matches,
        "replacement_front_page_local_same_owner_route_matches_source": same_owner_matches,
        "replacement_front_page_local_no_remote_free_claim": 1,
    }


def classify_next_owner(report: dict[str, Any]) -> str:
    if report["global_lock_hot_path_count_total"] > 0:
        return "global_lock_hot_path"
    if report["remote_free_push_count_total"] > 0 or report["remote_free_drain_count_total"] > 0:
        return "remote_free_queue"
    if report["page_from_ptr_range_scan_count_total"] > 0:
        return "range_scan_page_lookup"
    if (
        report["page_from_ptr_count_total"] > 0
        or report["page_index_probe_count_total"] > 0
        or report["owner_thread_id_lookup_count_total"] > 0
    ):
        return "free_path_page_lookup"
    if report["replacement_median_ops_per_sec"] > 0:
        return "perf_asm_owner_refresh"
    return "missing_replacement_front_subject"


def counter_gap_class(replacement_median: float, skip_median: float) -> tuple[str, float]:
    if replacement_median <= 0.0 or skip_median <= 0.0:
        return ("unknown", 0.0)
    gap = ratio(skip_median, replacement_median)
    if gap < 1.05:
        return ("low", gap)
    if gap < 1.15:
        return ("medium", gap)
    return ("high", gap)


def build_report(rows: dict[str, str], skip_rows: dict[str, str] | None) -> dict[str, Any]:
    replacement_idx = find_subject(rows, "replacement_front_c_shim", 2)
    mimalloc_idx = find_subject(rows, "c_mimalloc_ldpreload", 1)

    def front_counter(suffix: str) -> int:
        return int_value(
            rows,
            [
                f"subject_{replacement_idx}_{suffix}_total",
                f"subject_{replacement_idx}_{suffix}",
                f"{suffix}_total",
                suffix,
            ],
            0,
        )

    c_mimalloc_median = prefixed_float(rows, mimalloc_idx, "throughput_median_ops_per_sec")
    replacement_median = prefixed_float(rows, replacement_idx, "throughput_median_ops_per_sec")
    reported_vs_mimalloc = prefixed_float(rows, replacement_idx, "throughput_vs_c_mimalloc")
    throughput_vs_mimalloc = reported_vs_mimalloc or ratio(replacement_median, c_mimalloc_median)

    report: dict[str, Any] = {
        "output_contract": "hako-check-replacement-front-report-v0",
        "input_kind": "benchmark_kv_report",
        "tool_surface": "hako_check_replacement_front_report",
        "observation_only": 1,
        "rewrite_executed": 0,
        "source_rewrite_executed": 0,
        "provider_activation": 0,
        "global_allocator_product_claim": 0,
        "hook_installed": 0,
        "keeper_selection": 0,
        "benchmark_subject_index": replacement_idx,
        "c_mimalloc_subject_index": mimalloc_idx,
        "benchmark_threads": int_value(rows, ["benchmark_threads", "threads"]),
        "benchmark_thread_origin": first_value(rows, ["benchmark_thread_origin"], "c_pthread"),
        "benchmark_front_class": prefixed(rows, replacement_idx, "benchmark_front_class"),
        "hako_hot_path_claim": prefixed_int(rows, replacement_idx, "hako_hot_path_claim"),
        "hako_source_thread_support_claim": int_value(rows, ["hako_source_thread_support_claim"], 0),
        "hako_source_hot_path_claim": 0,
        "mir_builder_hot_path_claim": 0,
        "type_abi_hot_path_lookup_count": int_value(rows, ["type_abi_hot_path_lookup_count"], 0),
        "provider_dispatch_hot_path": int_value(rows, ["provider_dispatch_hot_path"], 0),
        "replacement_front_product_activation_ready": prefixed_int(
            rows, replacement_idx, "replacement_front_product_activation_ready"
        ),
        "replacement_front_is_full_hako_algorithm": int_value(
            rows, ["replacement_front_is_full_hako_algorithm"], 0
        ),
        "c_mimalloc_median_ops_per_sec": c_mimalloc_median,
        "replacement_median_ops_per_sec": replacement_median,
        "throughput_vs_c_mimalloc": throughput_vs_mimalloc,
        "remote_free_push_count_total": front_counter(
            "replacement_front_cross_thread_free_remote_push_count"
        ),
        "remote_free_drain_count_total": front_counter(
            "replacement_front_remote_free_drain_count"
        ),
        "remote_free_cas_retry_count_total": front_counter(
            "replacement_front_remote_free_cas_retry_count"
        ),
        "same_thread_free_local_count_total": front_counter(
            "replacement_front_same_thread_free_local_count"
        ),
        "same_thread_alloc_local_count_total": front_counter(
            "replacement_front_same_thread_alloc_local_count"
        ),
        "page_from_ptr_count_total": front_counter("replacement_front_page_from_ptr_count"),
        "page_from_ptr_range_scan_count_total": front_counter(
            "replacement_front_page_from_ptr_range_scan_count"
        ),
        "page_from_ptr_miss_count_total": front_counter(
            "replacement_front_page_from_ptr_miss_count"
        ),
        "owner_thread_id_lookup_count_total": front_counter(
            "replacement_front_owner_thread_id_lookup_count"
        ),
        "owner_thread_id_same_count_total": front_counter(
            "replacement_front_owner_thread_id_same_count"
        ),
        "owner_thread_id_remote_count_total": front_counter(
            "replacement_front_owner_thread_id_remote_count"
        ),
        "tls_arena_count_total": front_counter("replacement_front_tls_arena_count"),
        "tls_arena_peak_count_total": front_counter("replacement_front_tls_arena_peak_count"),
        "page_index_probe_count_total": front_counter("replacement_front_page_index_probe_count"),
        "global_lock_hot_path_count_total": front_counter(
            "replacement_front_global_lock_hot_path_count"
        ),
        "global_lock_refill_count_total": front_counter(
            "replacement_front_global_lock_refill_count"
        ),
        "host_passthrough_count_total": front_counter("replacement_front_host_passthrough_count"),
        "typed_page_meta_field_block_size": prefixed_int(
            rows, replacement_idx, "typed_page_meta_field_block_size"
        ),
        "typed_page_meta_field_free_head": prefixed_int(
            rows, replacement_idx, "typed_page_meta_field_free_head"
        ),
        "typed_page_meta_field_local_free_head": prefixed_int(
            rows, replacement_idx, "typed_page_meta_field_local_free_head"
        ),
        "typed_page_meta_field_capacity": prefixed_int(
            rows, replacement_idx, "typed_page_meta_field_capacity"
        ),
        "typed_page_meta_field_used": prefixed_int(
            rows, replacement_idx, "typed_page_meta_field_used"
        ),
    }

    generated_c_front = report["benchmark_front_class"] == "replacement_front_c_shim"
    report["measured_hot_path_owner"] = (
        "generated_c_replacement_front" if generated_c_front else "unknown"
    )
    report["api_boundary_gap_suspect"] = (
        0 if generated_c_front and report["hako_hot_path_claim"] == 0 else 1
    )
    report["remote_free_workload"] = int(
        report["remote_free_push_count_total"] > 0 or report["remote_free_drain_count_total"] > 0
    )
    report["same_thread_workload"] = int(
        report["same_thread_free_local_count_total"] > 0 and report["remote_free_workload"] == 0
    )
    report["replacement_front_owner_shadow_counters"] = int(
        report["owner_thread_id_lookup_count_total"] > 0
    )
    report["likely_next_owner"] = classify_next_owner(report)
    report["replacement_front_page_bins_lookup_route"] = prefixed(
        rows, replacement_idx, "replacement_front_page_bins_lookup_route", "unknown"
    )
    report["replacement_front_page_from_ptr_route"] = prefixed(
        rows, replacement_idx, "replacement_front_page_from_ptr_route", "unknown"
    )
    report["free_path_page_lookup_route"] = page_lookup_route(rows, replacement_idx, report)
    report["free_path_page_lookup_range_scan_count"] = report[
        "page_from_ptr_range_scan_count_total"
    ]
    report["page_map_bridge_kind"] = page_map_bridge_kind(rows, replacement_idx)
    report["page_map_bridge_type_abi_hot_lookup_count"] = report[
        "type_abi_hot_path_lookup_count"
    ]
    report["page_map_bridge_provider_abi_hot_dispatch_count"] = report[
        "provider_dispatch_hot_path"
    ]
    report["page_map_bridge_benchmark_front_pilot"] = int(
        report["free_path_page_lookup_route"] == "page_map_bridge"
        and report["free_path_page_lookup_range_scan_count"] == 0
        and report["page_map_bridge_type_abi_hot_lookup_count"] == 0
        and report["page_map_bridge_provider_abi_hot_dispatch_count"] == 0
    )
    product_mirror_source = prefixed(
        rows, replacement_idx, "replacement_front_size_class_policy_source"
    )
    size_class_evidence = size_class_box_evidence(product_mirror_source)
    product_source = size_class_evidence["replacement_front_size_class_bridge_source_truth"]
    producer = normalized_replacement_front_producer(
        prefixed(rows, replacement_idx, "replacement_front_producer"),
        report["benchmark_front_class"],
    )
    backend_artifact = normalized_backend_artifact(
        prefixed(rows, replacement_idx, "replacement_front_backend_artifact"),
        producer,
    )
    report["replacement_front_producer_taxonomy_v0"] = 1
    report["replacement_front_producer"] = producer
    report["replacement_front_backend_artifact"] = backend_artifact
    report["replacement_front_source_truth"] = normalized_replacement_front_source_truth(
        prefixed(rows, replacement_idx, "replacement_front_source_truth"),
        product_source,
    )
    report["replacement_front_python_template_c_semantic_ssot"] = prefixed_int(
        rows,
        replacement_idx,
        "replacement_front_python_template_c_semantic_ssot",
        0,
    )
    report["replacement_front_python_template_c_retirement_required"] = prefixed_int(
        rows,
        replacement_idx,
        "replacement_front_python_template_c_retirement_required",
        int(producer == "python_template_c_bridge"),
    )
    report["replacement_front_mir_memop_enabled"] = prefixed_int(
        rows,
        replacement_idx,
        "replacement_front_mir_memop_enabled",
        int(producer in {"mir_to_c_lowering", "mir_to_llvm_lowering"}),
    )
    report["replacement_front_mir_fastmem_region_enabled"] = prefixed_int(
        rows,
        replacement_idx,
        "replacement_front_mir_fastmem_region_enabled",
        int(producer in {"mir_to_c_lowering", "mir_to_llvm_lowering"}),
    )
    report["replacement_front_mirbuilder_representation_only"] = prefixed_int(
        rows,
        replacement_idx,
        "replacement_front_mirbuilder_representation_only",
        1,
    )
    report["replacement_front_mirbuilder_route_decision_count"] = prefixed_int(
        rows,
        replacement_idx,
        "replacement_front_mirbuilder_route_decision_count",
        0,
    )
    report["replacement_front_producer_transition_state"] = prefixed(
        rows,
        replacement_idx,
        "replacement_front_producer_transition_state",
        producer_transition_state(producer),
    )
    report["replacement_front_producer_slice_selection_v0"] = prefixed_int(
        rows,
        replacement_idx,
        "replacement_front_producer_slice_selection_v0",
        int(PRODUCER_SLICE_DEFAULTS["replacement_front_producer_slice_selection_v0"]),
    )
    for key in [
        "replacement_front_next_producer_slice",
        "replacement_front_selected_memop_family",
        "replacement_front_selected_memop_kinds",
        "replacement_front_deferred_memop_family",
        "replacement_front_deferred_memop_kinds",
    ]:
        report[key] = prefixed(rows, replacement_idx, key, str(PRODUCER_SLICE_DEFAULTS[key]))
    for key in [
        "replacement_front_selection_behavior_change",
        "replacement_front_selection_product_activation",
        "replacement_front_selection_bridge_retirement_allowed",
    ]:
        report[key] = prefixed_int(
            rows,
            replacement_idx,
            key,
            int(PRODUCER_SLICE_DEFAULTS[key]),
        )
    page_local_mirror_source = prefixed(
        rows, replacement_idx, "replacement_front_page_local_state_source"
    )
    product_preflight_report = prefixed_int(
        rows, replacement_idx, "replacement_front_product_preflight_report_v0"
    )
    product_preflight_evidence_ready = prefixed_int(
        rows, replacement_idx, "replacement_front_product_preflight_evidence_ready"
    )
    product_preflight_quality_ok = prefixed_int(
        rows, replacement_idx, "replacement_front_product_preflight_quality_ok"
    )
    product_preflight_provider_ok = prefixed_int(
        rows,
        replacement_idx,
        "replacement_front_product_preflight_provider_dispatch_bypass_ok",
    )
    product_preflight_type_abi_ok = prefixed_int(
        rows,
        replacement_idx,
        "replacement_front_product_preflight_type_abi_hot_lookup_zero_ok",
    )
    product_preflight_cross_thread_ok = prefixed_int(
        rows, replacement_idx, "replacement_front_product_preflight_cross_thread_policy_ok"
    )
    product_preflight_remote_ok = prefixed_int(
        rows,
        replacement_idx,
        "replacement_front_product_preflight_remote_abandoned_counters_ok",
    )
    product_preflight_rollback_ok = prefixed_int(
        rows, replacement_idx, "replacement_front_product_preflight_rollback_optout_ok"
    )
    product_preflight_missing = prefixed(
        rows,
        replacement_idx,
        "replacement_front_product_preflight_missing",
        "product_gate_open,activation_row",
    )
    product_shape_ok = int(
        report["page_map_bridge_benchmark_front_pilot"]
        and report["global_lock_hot_path_count_total"] == 0
        and report["page_from_ptr_range_scan_count_total"] == 0
    )
    product_safety_ok = int(
        report["replacement_front_product_activation_ready"] == 0
        and report["type_abi_hot_path_lookup_count"] == 0
        and report["provider_dispatch_hot_path"] == 0
    )
    product_preflight_ok = int(
        product_preflight_report
        and product_preflight_evidence_ready
        and product_preflight_quality_ok
        and product_preflight_provider_ok
        and product_preflight_type_abi_ok
        and product_preflight_cross_thread_ok
        and product_preflight_remote_ok
        and product_preflight_rollback_ok
    )
    product_no_host_passthrough = int(report["host_passthrough_count_total"] == 0)
    product_coverage_ok = int(
        product_source != "unknown"
        and product_preflight_ok
        and size_class_evidence["replacement_front_size_class_bridge_bound"]
    )
    product_missing_parts = [
        part
        for part, missing in [
            ("source_truth", product_source == "unknown"),
            ("preflight", not product_preflight_ok),
            ("shape", not product_shape_ok),
            ("safety", not product_safety_ok),
            ("host_passthrough_zero", not product_no_host_passthrough),
        ]
        if missing
    ]
    for blocker in product_preflight_missing.split(","):
        if blocker and blocker not in product_missing_parts:
            product_missing_parts.append(blocker)
    report["replacement_front_product_shaped_bridge_v0"] = 1
    report["replacement_front_product_shaped_bridge_non_activating"] = 1
    report["replacement_front_product_shaped_bridge_report_only"] = 1
    report["replacement_front_product_shaped_bridge_route"] = (
        "replacement_front_benchmark_to_product_ldpreload_descriptor"
    )
    report["replacement_front_product_shaped_bridge_source_truth"] = product_source
    report["replacement_front_product_shaped_bridge_shape_ok"] = product_shape_ok
    report["replacement_front_product_shaped_bridge_safety_ok"] = product_safety_ok
    report["replacement_front_product_shaped_bridge_coverage_ok"] = product_coverage_ok
    report["replacement_front_product_shaped_bridge_preflight_ok"] = product_preflight_ok
    report["replacement_front_product_shaped_bridge_no_type_abi_hot_lookup"] = int(
        report["type_abi_hot_path_lookup_count"] == 0
    )
    report["replacement_front_product_shaped_bridge_no_provider_dispatch"] = int(
        report["provider_dispatch_hot_path"] == 0
    )
    report["replacement_front_product_shaped_bridge_no_global_lock_hot_path"] = int(
        report["global_lock_hot_path_count_total"] == 0
    )
    report["replacement_front_product_shaped_bridge_no_range_scan_hot_path"] = int(
        report["page_from_ptr_range_scan_count_total"] == 0
    )
    report["replacement_front_product_shaped_bridge_no_host_passthrough"] = (
        product_no_host_passthrough
    )
    report["replacement_front_product_shaped_bridge_requires_activation_row"] = 1
    report["replacement_front_product_shaped_bridge_requires_product_gate_open"] = 1
    report["replacement_front_product_shaped_bridge_activation_ready"] = 0
    report["replacement_front_product_shaped_bridge_evidence_ready"] = int(
        product_shape_ok and product_safety_ok and product_coverage_ok and product_no_host_passthrough
    )
    report["replacement_front_product_shaped_bridge_missing"] = (
        ",".join(product_missing_parts) if product_missing_parts else "none"
    )
    report["replacement_front_product_shaped_bridge_block_reason"] = (
        "activation_row_required"
        if report["replacement_front_product_shaped_bridge_evidence_ready"]
        else "missing_bridge_evidence"
    )
    report.update(size_class_evidence)
    report.update(page_local_bridge_evidence(page_local_mirror_source, report))

    if skip_rows is not None:
        skip_replacement_idx = find_subject(skip_rows, "replacement_front_c_shim", replacement_idx)
        skip_median = prefixed_float(skip_rows, skip_replacement_idx, "throughput_median_ops_per_sec")
        gap_class, gap_ratio = counter_gap_class(replacement_median, skip_median)
        report["skip_hot_counters_median_ops_per_sec"] = skip_median
        report["skip_hot_counter_gap_ratio"] = gap_ratio
        report["skip_hot_counter_gap_class"] = gap_class
    else:
        report["skip_hot_counters_median_ops_per_sec"] = 0.0
        report["skip_hot_counter_gap_ratio"] = 0.0
        report["skip_hot_counter_gap_class"] = "unknown"

    report["clean"] = int(
        generated_c_front
        and report["hako_hot_path_claim"] == 0
        and report["provider_activation"] == 0
        and report["hook_installed"] == 0
    )
    report["summary"] = "ok" if report["benchmark_front_class"] else "failed"
    return report


def format_value(value: Any) -> str:
    if isinstance(value, float):
        return f"{value:.6f}"
    return str(value)


def emit_kv(report: dict[str, Any]) -> str:
    return "\n".join(f"{key}={format_value(value)}" for key, value in report.items()) + "\n"


def emit_summary(report: dict[str, Any]) -> str:
    lines = [
        f"contract: {report['output_contract']}",
        f"front: {report['benchmark_front_class']} threads={report['benchmark_threads']}",
        (
            "throughput: "
            f"replacement={report['replacement_median_ops_per_sec']:.3f} "
            f"c_mimalloc={report['c_mimalloc_median_ops_per_sec']:.3f} "
            f"ratio={report['throughput_vs_c_mimalloc']:.6f}"
        ),
        (
            "claims: "
            f"hako_hot_path={report['hako_hot_path_claim']} "
            f"mir_builder_hot_path={report['mir_builder_hot_path_claim']} "
            f"provider_activation={report['provider_activation']}"
        ),
        (
            "hot counts: "
            f"page_from_ptr={report['page_from_ptr_count_total']} "
            f"owner_lookup={report['owner_thread_id_lookup_count_total']} "
            f"page_index_probe={report['page_index_probe_count_total']} "
            f"global_hot_lock={report['global_lock_hot_path_count_total']} "
            f"remote_push={report['remote_free_push_count_total']}"
        ),
        (
            "page lookup: "
            f"route={report['free_path_page_lookup_route']} "
            f"bridge={report['page_map_bridge_kind']} "
            f"range_scan={report['free_path_page_lookup_range_scan_count']}"
        ),
        f"next_owner: {report['likely_next_owner']}",
        f"summary: {report['summary']}",
    ]
    if report["skip_hot_counter_gap_class"] != "unknown":
        lines.insert(
            3,
            (
                "skip-counter gap: "
                f"class={report['skip_hot_counter_gap_class']} "
                f"ratio={report['skip_hot_counter_gap_ratio']:.6f}"
            ),
        )
    return "\n".join(lines) + "\n"


def write_output(text: str, out: Path | None) -> None:
    if out is None:
        print(text, end="")
        return
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text(text, encoding="utf-8")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--report", required=True, type=Path)
    parser.add_argument("--baseline-skip-report", type=Path)
    parser.add_argument("--format", choices=("kv", "summary", "json"), default="kv")
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    rows = read_kv(args.report)
    skip_rows = read_kv(args.baseline_skip_report) if args.baseline_skip_report else None
    report = build_report(rows, skip_rows)

    if args.format == "json":
        text = json.dumps(report, indent=2, sort_keys=True) + "\n"
    elif args.format == "summary":
        text = emit_summary(report)
    else:
        text = emit_kv(report)
    write_output(text, args.out)
    return 0 if report["summary"] == "ok" else 1


if __name__ == "__main__":
    raise SystemExit(main())
