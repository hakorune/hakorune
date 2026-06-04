"""Top-level summary fields for mimalloc algorithm coverage reports."""

from __future__ import annotations

from pathlib import Path
from typing import Mapping, cast

from hako_mimalloc_algorithm_coverage_support import (
    HAKO_ALLOC,
    ROOT,
    REPLACEMENT_FRONT,
    CoverageRow,
    int_field,
    str_field,
)


def build_summary_report_fields(
    *,
    context: Mapping[str, object],
) -> dict[str, object]:
    rows = cast(list[CoverageRow], context["rows"])
    benchmark_report = cast(Path | None, context["benchmark_report"])
    fastpath_report = cast(Path | None, context["fastpath_report"])
    state_report = cast(Path | None, context["state_report"])
    perf_attribution_report = cast(Path | None, context["perf_attribution_report"])
    accumulator_report = cast(Path | None, context["accumulator_report"])
    benchmark_report_consumed = cast(int, context["benchmark_report_consumed"])
    fastpath_report_consumed = cast(int, context["fastpath_report_consumed"])
    state_report_consumed = cast(int, context["state_report_consumed"])
    perf_attribution_report_consumed = cast(
        int, context["perf_attribution_report_consumed"]
    )
    accumulator_report_consumed = cast(int, context["accumulator_report_consumed"])
    benchmark_subject = cast(str, context["benchmark_subject"])
    accumulator_contract_ready = cast(int, context["accumulator_contract_ready"])
    accumulator = cast(dict[str, str], context["accumulator"])
    size_class_single_bridge_supported = cast(
        int, context["size_class_single_bridge_supported"]
    )
    page_bins_bridge_supported = cast(int, context["page_bins_bridge_supported"])
    page_bins_consumer_enabled = cast(int, context["page_bins_consumer_enabled"])
    page_bins_route = cast(str, context["page_bins_route"])
    page_bins_lookup_route = cast(str, context["page_bins_lookup_route"])
    product_bins_consumer_enabled = cast(int, context["product_bins_consumer_enabled"])
    product_bins_route = cast(str, context["product_bins_route"])
    product_pages_consumer_enabled = cast(int, context["product_pages_consumer_enabled"])
    product_pages_route = cast(str, context["product_pages_route"])
    product_pages_source_ready = cast(int, context["product_pages_source_ready"])
    product_pages_full_source_ready = cast(
        int, context["product_pages_full_source_ready"]
    )
    product_pages_bridge_blocker = cast(str, context["product_pages_bridge_blocker"])
    product_pages_next_bridge = cast(str, context["product_pages_next_bridge"])
    product_pages_non_linear_lookup_plan = cast(
        str, context["product_pages_non_linear_lookup_plan"]
    )
    product_pages_linear_probe_closed = cast(
        int, context["product_pages_linear_probe_closed"]
    )
    product_pages_non_linear_lookup_probe_closed = cast(
        int, context["product_pages_non_linear_lookup_probe_closed"]
    )
    product_pages_non_linear_lookup_decision = cast(
        str, context["product_pages_non_linear_lookup_decision"]
    )
    product_pages_non_linear_lookup_strategy = cast(
        str, context["product_pages_non_linear_lookup_strategy"]
    )
    product_pages_non_linear_next_bridge = cast(
        str, context["product_pages_non_linear_next_bridge"]
    )
    page_map_source_ready = cast(int, context["page_map_source_ready"])
    page_map_release_source_ready = cast(int, context["page_map_release_source_ready"])
    realloc_same_class_source_ready = cast(int, context["realloc_same_class_source_ready"])
    realloc_grow_copy_release_source_ready = cast(
        int, context["realloc_grow_copy_release_source_ready"]
    )
    huge_page_source_ready = cast(int, context["huge_page_source_ready"])
    osvm_page_source_pilot_ready = cast(int, context["osvm_page_source_pilot_ready"])
    locked_front = cast(int, context["locked_front"])
    tls_front = cast(int, context["tls_front"])
    replacement_full_hako = cast(int, context["replacement_full_hako"])

    return {
        "output_contract": "hako-mimalloc-algorithm-coverage-v0",
        "hako_alloc_root": str(HAKO_ALLOC.relative_to(ROOT)),
        "replacement_front": str(REPLACEMENT_FRONT.relative_to(ROOT)),
        "replacement_front_is_full_hako_algorithm": replacement_full_hako,
        "provider_activation": 0,
        "production_replacement_active": 0,
        "winner_claim": 0,
        "benchmark_report": str(benchmark_report) if benchmark_report is not None else "none",
        "benchmark_report_consumed": benchmark_report_consumed,
        "benchmark_replacement_subject": benchmark_subject,
        "fastpath_report": str(fastpath_report) if fastpath_report is not None else "none",
        "fastpath_report_consumed": fastpath_report_consumed,
        "state_report": str(state_report) if state_report is not None else "none",
        "state_report_consumed": state_report_consumed,
        "perf_attribution_report": str(perf_attribution_report)
        if perf_attribution_report is not None
        else "none",
        "perf_attribution_report_consumed": perf_attribution_report_consumed,
        "accumulator_report": str(accumulator_report)
        if accumulator_report is not None
        else "none",
        "accumulator_report_consumed": accumulator_report_consumed,
        "requested_bytes_accumulator_contract_v0": accumulator_contract_ready,
        "requested_bytes_accumulator_expected_no_overflow": int_field(
            accumulator, "expected_no_overflow", 0
        ),
        "requested_bytes_accumulator_observed_no_overflow": int_field(
            accumulator, "observed_no_overflow", 0
        ),
        "requested_bytes_accumulator_general_no_overflow_proof": int_field(
            accumulator, "general_no_overflow_proof", 0
        ),
        "requested_bytes_accumulator_source_reorder_allowed": int_field(
            accumulator, "source_reorder_allowed", 0
        ),
        "requested_bytes_accumulator_next_bridge": str_field(
            accumulator,
            "next_bridge",
            "add_public_proof_accumulator_overflow_policy_before_source_reorder",
        ),
        "area_count": len(rows),
        "hako_model_area_count": sum(row.hako_model for row in rows),
        "replacement_front_area_count": sum(row.replacement_front for row in rows),
        "model_only_area_count": sum(1 for row in rows if row.status == "model_only"),
        "split_model_and_fixed_front_area_count": sum(
            1 for row in rows if row.status == "split_model_and_fixed_front"
        ),
        "open_area_count": sum(1 for row in rows if row.status == "open"),
        "size_class_policy_bridge_plan_v0": 1,
        "size_class_policy_product_bins_connected": product_bins_consumer_enabled,
        "size_class_policy_single_class_benchmark_bridge_supported": int(
            size_class_single_bridge_supported
        ),
        "size_class_policy_single_class_bridge_mode": "hako_good_size_request_ceiling"
        if size_class_single_bridge_supported
        else "none",
        "size_class_policy_next_bridge": "product_replacement_bins_pages",
        "replacement_front_page_bins_plan_v0": 1,
        "replacement_front_page_bins_supported": int(page_bins_bridge_supported),
        "replacement_front_page_bins_consumer_enabled": page_bins_consumer_enabled,
        "replacement_front_page_bins_route": page_bins_route,
        "replacement_front_page_bins_lookup_route": page_bins_lookup_route,
        "replacement_front_page_bins_owner": "benchmark_only",
        "replacement_front_page_bins_product_claim": 0,
        "replacement_front_benchmark_algorithm_shape": cast(
            str, context["algorithm_shape"]
        ),
        "replacement_front_product_bins_consumer_enabled": product_bins_consumer_enabled,
        "replacement_front_product_bins_route": product_bins_route,
        "replacement_front_product_pages_bridge_plan_v0": 1,
        "replacement_front_product_pages_bridge_report_only": 1,
        "replacement_front_product_pages_consumer_enabled": product_pages_consumer_enabled,
        "replacement_front_product_pages_route": product_pages_route,
        "replacement_front_product_pages_source_ready": product_pages_source_ready,
        "replacement_front_product_pages_full_source_ready": product_pages_full_source_ready,
        "replacement_front_product_pages_bridge_blocker": product_pages_bridge_blocker,
        "replacement_front_product_pages_next_bridge": product_pages_next_bridge,
        "replacement_front_product_pages_non_linear_lookup_plan_v0": product_pages_non_linear_lookup_plan,
        "replacement_front_product_pages_linear_probe_closed": product_pages_linear_probe_closed,
        "replacement_front_product_pages_non_linear_lookup_probe_closed": product_pages_non_linear_lookup_probe_closed,
        "replacement_front_product_pages_non_linear_lookup_decision": product_pages_non_linear_lookup_decision,
        "replacement_front_product_pages_non_linear_lookup_strategy": product_pages_non_linear_lookup_strategy,
        "replacement_front_product_pages_non_linear_next_bridge": product_pages_non_linear_next_bridge,
        "page_map_source_ready": page_map_source_ready,
        "page_map_release_source_ready": page_map_release_source_ready,
        "realloc_same_class_source_ready": realloc_same_class_source_ready,
        "realloc_grow_copy_release_source_ready": realloc_grow_copy_release_source_ready,
        "huge_page_source_ready": huge_page_source_ready,
        "osvm_page_source_pilot_ready": osvm_page_source_pilot_ready,
        "replacement_front_locked_global_multithread_supported": locked_front,
        "replacement_front_thread_local_multithread_supported": tls_front,
        "replacement_front_multithread_claim": 0,
    }
