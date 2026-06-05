#!/usr/bin/env python3
"""Explain replacement-front benchmark reports without changing execution."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any


def read_kv(path: Path) -> dict[str, str]:
    if not path.is_file():
        raise SystemExit(f"missing report file: {path}")
    rows: dict[str, str] = {}
    for raw_line in path.read_text(encoding="utf-8", errors="replace").splitlines():
        line = raw_line.strip()
        if not line or line.startswith("#") or "=" not in line:
            continue
        key, value = line.split("=", 1)
        rows[key.strip()] = value.strip()
    return rows


def first_value(rows: dict[str, str], keys: list[str], default: str = "") -> str:
    for key in keys:
        value = rows.get(key)
        if value is not None and value != "":
            return value
    return default


def int_value(rows: dict[str, str], keys: list[str], default: int = 0) -> int:
    value = first_value(rows, keys)
    if value == "":
        return default
    try:
        return int(float(value))
    except ValueError:
        return default


def float_value(rows: dict[str, str], keys: list[str], default: float = 0.0) -> float:
    value = first_value(rows, keys)
    if value == "":
        return default
    try:
        return float(value)
    except ValueError:
        return default


def subject_indices(rows: dict[str, str]) -> list[int]:
    indices: set[int] = set()
    for key in rows:
        if not key.startswith("subject_"):
            continue
        parts = key.split("_", 2)
        if len(parts) < 3:
            continue
        try:
            indices.add(int(parts[1]))
        except ValueError:
            continue
    return sorted(indices)


def find_subject(rows: dict[str, str], front_class: str, fallback: int) -> int:
    for idx in subject_indices(rows):
        if rows.get(f"subject_{idx}_benchmark_front_class") == front_class:
            return idx
    return fallback


def prefixed(rows: dict[str, str], subject_idx: int, suffix: str, default: str = "") -> str:
    return first_value(rows, [f"subject_{subject_idx}_{suffix}", suffix], default)


def prefixed_int(rows: dict[str, str], subject_idx: int, suffix: str, default: int = 0) -> int:
    return int_value(rows, [f"subject_{subject_idx}_{suffix}", suffix], default)


def prefixed_float(
    rows: dict[str, str],
    subject_idx: int,
    suffix: str,
    default: float = 0.0,
) -> float:
    return float_value(rows, [f"subject_{subject_idx}_{suffix}", suffix], default)


def ratio(numerator: float, denominator: float) -> float:
    if denominator == 0.0:
        return 0.0
    return numerator / denominator


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
        "remote_free_push_count_total": prefixed_int(
            rows, replacement_idx, "replacement_front_cross_thread_free_remote_push_count_total"
        ),
        "remote_free_drain_count_total": prefixed_int(
            rows, replacement_idx, "replacement_front_remote_free_drain_count_total"
        ),
        "remote_free_cas_retry_count_total": prefixed_int(
            rows, replacement_idx, "replacement_front_remote_free_cas_retry_count_total"
        ),
        "same_thread_free_local_count_total": prefixed_int(
            rows, replacement_idx, "replacement_front_same_thread_free_local_count_total"
        ),
        "same_thread_alloc_local_count_total": prefixed_int(
            rows, replacement_idx, "replacement_front_same_thread_alloc_local_count_total"
        ),
        "page_from_ptr_count_total": prefixed_int(
            rows, replacement_idx, "replacement_front_page_from_ptr_count_total"
        ),
        "page_from_ptr_range_scan_count_total": prefixed_int(
            rows, replacement_idx, "replacement_front_page_from_ptr_range_scan_count_total"
        ),
        "page_from_ptr_miss_count_total": prefixed_int(
            rows, replacement_idx, "replacement_front_page_from_ptr_miss_count_total"
        ),
        "owner_thread_id_lookup_count_total": prefixed_int(
            rows, replacement_idx, "replacement_front_owner_thread_id_lookup_count_total"
        ),
        "owner_thread_id_same_count_total": prefixed_int(
            rows, replacement_idx, "replacement_front_owner_thread_id_same_count_total"
        ),
        "owner_thread_id_remote_count_total": prefixed_int(
            rows, replacement_idx, "replacement_front_owner_thread_id_remote_count_total"
        ),
        "tls_arena_count_total": prefixed_int(
            rows, replacement_idx, "replacement_front_tls_arena_count_total"
        ),
        "tls_arena_peak_count_total": prefixed_int(
            rows, replacement_idx, "replacement_front_tls_arena_peak_count_total"
        ),
        "page_index_probe_count_total": prefixed_int(
            rows, replacement_idx, "replacement_front_page_index_probe_count_total"
        ),
        "global_lock_hot_path_count_total": prefixed_int(
            rows, replacement_idx, "replacement_front_global_lock_hot_path_count_total"
        ),
        "global_lock_refill_count_total": prefixed_int(
            rows, replacement_idx, "replacement_front_global_lock_refill_count_total"
        ),
        "host_passthrough_count_total": prefixed_int(
            rows, replacement_idx, "replacement_front_host_passthrough_count_total"
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
    product_source = normalized_product_bridge_source(
        prefixed(rows, replacement_idx, "replacement_front_size_class_policy_source")
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
    product_coverage_ok = int(product_source != "unknown" and product_preflight_ok)
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
