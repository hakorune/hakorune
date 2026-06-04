"""Benchmark/fastpath/perf measurement derivation for coverage reports."""

from __future__ import annotations

from dataclasses import dataclass
from typing import Mapping

from hako_mimalloc_algorithm_coverage_support import int_field, str_field


@dataclass(frozen=True)
class CoverageMeasurementStateInputs:
    benchmark: Mapping[str, object]
    fastpath: Mapping[str, object]
    perf_attribution: Mapping[str, object]
    accumulator: Mapping[str, object]
    benchmark_report_consumed: int
    fastpath_report_consumed: int
    perf_attribution_report_consumed: int
    accumulator_report_consumed: int


@dataclass(frozen=True)
class CoverageMeasurementState:
    benchmark_replacement_subject: str
    page_bins_consumer_enabled: int
    page_bins_route: str
    page_bins_lookup_route: str
    product_bins_consumer_enabled: int
    product_bins_route: str
    product_pages_consumer_enabled: int
    product_pages_route: str
    algorithm_shape: str
    hotcore_consumer_enabled: int
    hotcore_route: str
    hotcore_median_ops_per_sec: str
    hotcore_measurement_reported: int
    fastpath_direct_array_plan_count: int
    fastpath_route_decision_count: int
    fastpath_fast_selected_count: int
    fastpath_slow_selected_count: int
    fastpath_generic_dispatch_count: int
    fastpath_dynamic_route_count: int
    fastpath_boxed_fallback_count: int
    fastpath_clean: int
    perf_delta_plan: int
    perf_delta_ready: int
    perf_delta_blocker: str
    perf_delta_next_bridge: str
    instruction_attribution_available: int
    perf_top_instruction_field_hints: str
    perf_hot_instruction_0_field_hints: str
    perf_hot_instruction_0_context: str
    backend_store_shape_ready: int
    backend_store_shape_selected: str
    backend_store_shape_next_bridge: str
    directarray_owner_instruction_shape_selected: str
    directarray_owner_instruction_shape_next_bridge: str
    inlined_hot_body_selected: str
    inlined_hot_body_next_bridge: str
    inlined_hot_body_split_next_bridge: str


def derive_measurement_state(
    inputs: CoverageMeasurementStateInputs,
) -> CoverageMeasurementState:
    benchmark = inputs.benchmark

    benchmark_subject = "none"
    benchmark_subject_prefix = ""
    for prefix in ("subject_2", "subject_3", "subject_4"):
        subject_id = benchmark.get(f"{prefix}_id") or benchmark.get(f"{prefix}_name")
        if subject_id == "hakorune_replacement_front_ldpreload":
            benchmark_subject = subject_id
            benchmark_subject_prefix = prefix
            break
    if (
        inputs.benchmark_report_consumed
        and benchmark_subject == "none"
        and int_field(benchmark, "replacement_front_page_bins_consumer_enabled", 0)
    ):
        benchmark_subject = "hakorune_replacement_front_ldpreload"

    def subject_key(suffix: str) -> str:
        return (
            f"{benchmark_subject_prefix}_{suffix}" if benchmark_subject_prefix else suffix
        )

    def subject_int(suffix: str, default: int = 0) -> int:
        return int_field(benchmark, subject_key(suffix), default)

    def subject_str(suffix: str, default: str = "none") -> str:
        return str_field(benchmark, subject_key(suffix), default)

    page_bins_consumer_enabled = subject_int("replacement_front_page_bins_consumer_enabled")
    page_bins_route = benchmark.get(
        subject_key("replacement_front_page_bins_route"),
        "not_consumed",
    )
    page_bins_lookup_route = benchmark.get(
        subject_key("replacement_front_page_bins_lookup_route"),
        "not_recorded" if page_bins_consumer_enabled else "not_consumed",
    )
    product_bins_consumer_enabled = subject_int(
        "replacement_front_product_bins_consumer_enabled"
    )
    product_bins_route = benchmark.get(
        subject_key("replacement_front_product_bins_route"),
        "not_consumed",
    )
    product_pages_consumer_enabled = subject_int(
        "replacement_front_product_pages_consumer_enabled"
    )
    product_pages_route = benchmark.get(
        subject_key("replacement_front_product_pages_route"),
        "not_consumed",
    )
    algorithm_shape = benchmark.get(
        subject_key("replacement_front_algorithm_shape"),
        "not_consumed",
    )
    hotcore_consumer_enabled = subject_int("replacement_front_hotcore_consumer_enabled")
    hotcore_route = benchmark.get(
        subject_key("replacement_front_hotcore_route"),
        "not_consumed_by_replacement_front",
    )
    hotcore_median_ops_per_sec = subject_str("throughput_median_ops_per_sec", "0")
    hotcore_measurement_reported = int(
        hotcore_consumer_enabled and hotcore_median_ops_per_sec != "0"
    )
    fastpath_direct_array_plan_count = int_field(
        inputs.fastpath, "direct_array_access_plan_count", 0
    )
    fastpath_route_decision_count = int_field(inputs.fastpath, "route_decision_count", 0)
    fastpath_fast_selected_count = int_field(
        inputs.fastpath, "route_decision_fast_selected_count", 0
    )
    fastpath_slow_selected_count = int_field(
        inputs.fastpath, "route_decision_slow_selected_count", 0
    )
    fastpath_generic_dispatch_count = int_field(
        inputs.fastpath, "generic_method_dispatch_count", 0
    )
    fastpath_dynamic_route_count = int_field(inputs.fastpath, "dynamic_route_count", 0)
    fastpath_boxed_fallback_count = int_field(inputs.fastpath, "boxed_fallback_count", 0)
    fastpath_clean = int_field(inputs.fastpath, "clean", 0)
    perf_delta_plan = int_field(
        inputs.perf_attribution, "page_model_hot_array_perf_delta_measurement_plan_v0", 0
    )
    perf_delta_ready = int_field(
        inputs.perf_attribution, "page_model_hot_array_perf_delta_ready", 0
    )
    perf_delta_blocker = str_field(
        inputs.perf_attribution,
        "page_model_hot_array_perf_delta_blocker",
        "perf_attribution_report_not_consumed"
        if not inputs.perf_attribution_report_consumed
        else "unknown",
    )
    perf_delta_next_bridge = str_field(
        inputs.perf_attribution,
        "page_model_hot_array_perf_delta_next_bridge",
        "run_hako_mimalloc_direct_exact_app_perf_asm"
        if not inputs.perf_attribution_report_consumed
        else "inspect_perf_attribution",
    )
    instruction_attribution_available = int_field(
        inputs.perf_attribution, "instruction_attribution_available", 0
    )
    perf_top_instruction_field_hints = str_field(
        inputs.perf_attribution, "top_instruction_field_hints", "none"
    )
    perf_hot_instruction_0_field_hints = str_field(
        inputs.perf_attribution, "hot_instruction_0_field_hints", "none"
    )
    perf_hot_instruction_0_context = str_field(
        inputs.perf_attribution, "hot_instruction_0_context", "none"
    )
    backend_store_shape_ready = int_field(
        inputs.perf_attribution, "backend_store_shape_ready", 0
    )
    backend_store_shape_selected = str_field(
        inputs.perf_attribution, "backend_store_shape_selected", "none"
    )
    backend_store_shape_next_bridge = str_field(
        inputs.perf_attribution,
        "backend_store_shape_next_bridge",
        "split_symbol_or_classify_backend_store_shape",
    )
    directarray_owner_instruction_shape_selected = str_field(
        inputs.perf_attribution, "directarray_owner_instruction_shape_selected", "none"
    )
    directarray_owner_instruction_shape_next_bridge = str_field(
        inputs.perf_attribution,
        "directarray_owner_instruction_shape_next_bridge",
        "collect_directarray_owner_instruction",
    )
    inlined_hot_body_selected = str_field(
        inputs.perf_attribution, "inlined_hot_body_selected", "none"
    )
    inlined_hot_body_next_bridge = str_field(
        inputs.perf_attribution,
        "inlined_hot_body_next_bridge",
        "rerun_perf_with_wider_context_or_symbol_split",
    )
    inlined_hot_body_split_next_bridge = str_field(
        inputs.perf_attribution,
        "inlined_hot_body_split_next_bridge",
        inlined_hot_body_next_bridge,
    )
    return CoverageMeasurementState(
        benchmark_replacement_subject=benchmark_subject,
        page_bins_consumer_enabled=page_bins_consumer_enabled,
        page_bins_route=page_bins_route,
        page_bins_lookup_route=page_bins_lookup_route,
        product_bins_consumer_enabled=product_bins_consumer_enabled,
        product_bins_route=product_bins_route,
        product_pages_consumer_enabled=product_pages_consumer_enabled,
        product_pages_route=product_pages_route,
        algorithm_shape=algorithm_shape,
        hotcore_consumer_enabled=hotcore_consumer_enabled,
        hotcore_route=hotcore_route,
        hotcore_median_ops_per_sec=hotcore_median_ops_per_sec,
        hotcore_measurement_reported=hotcore_measurement_reported,
        fastpath_direct_array_plan_count=fastpath_direct_array_plan_count,
        fastpath_route_decision_count=fastpath_route_decision_count,
        fastpath_fast_selected_count=fastpath_fast_selected_count,
        fastpath_slow_selected_count=fastpath_slow_selected_count,
        fastpath_generic_dispatch_count=fastpath_generic_dispatch_count,
        fastpath_dynamic_route_count=fastpath_dynamic_route_count,
        fastpath_boxed_fallback_count=fastpath_boxed_fallback_count,
        fastpath_clean=fastpath_clean,
        perf_delta_plan=perf_delta_plan,
        perf_delta_ready=perf_delta_ready,
        perf_delta_blocker=perf_delta_blocker,
        perf_delta_next_bridge=perf_delta_next_bridge,
        instruction_attribution_available=instruction_attribution_available,
        perf_top_instruction_field_hints=perf_top_instruction_field_hints,
        perf_hot_instruction_0_field_hints=perf_hot_instruction_0_field_hints,
        perf_hot_instruction_0_context=perf_hot_instruction_0_context,
        backend_store_shape_ready=backend_store_shape_ready,
        backend_store_shape_selected=backend_store_shape_selected,
        backend_store_shape_next_bridge=backend_store_shape_next_bridge,
        directarray_owner_instruction_shape_selected=directarray_owner_instruction_shape_selected,
        directarray_owner_instruction_shape_next_bridge=directarray_owner_instruction_shape_next_bridge,
        inlined_hot_body_selected=inlined_hot_body_selected,
        inlined_hot_body_next_bridge=inlined_hot_body_next_bridge,
        inlined_hot_body_split_next_bridge=inlined_hot_body_split_next_bridge,
    )
