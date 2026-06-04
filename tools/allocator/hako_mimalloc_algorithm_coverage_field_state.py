"""Field-state derivation for mimalloc algorithm coverage reports."""

from __future__ import annotations

from dataclasses import dataclass
from typing import Mapping

from allocator_field_buckets import (
    bucket_for_field,
    fields_from_context,
    fields_from_hint,
    format_field_buckets,
)
from hako_mimalloc_algorithm_coverage_support import int_field, page_model_field_names, str_field


@dataclass(frozen=True)
class CoverageFieldStateInputs:
    page_box: str
    perf_attribution: Mapping[str, object]
    perf_attribution_report_consumed: int
    state: Mapping[str, object]
    state_report_consumed: int


@dataclass(frozen=True)
class CoverageFieldState:
    hot_field_names: list[str]
    hot_field_top: str
    hot_field_top_bucket: str
    hot_field_buckets: str
    hot_field_primitive_hot_state_count: int
    hot_field_public_or_proof_count: int
    hot_field_observer_counter_count: int
    hot_field_plan_ready: int
    hot_field_next_bridge: str
    record_state_static_candidate_fields: str
    record_state_observed_candidate_fields: str
    record_state_rejected_observed_fields: str
    record_state_field_access_ready: int
    record_state_report_ready: int


def derive_field_state(inputs: CoverageFieldStateInputs) -> CoverageFieldState:
    perf_attribution = inputs.perf_attribution
    state = inputs.state

    perf_top_instruction_field_hints = str_field(
        perf_attribution, "top_instruction_field_hints", "none"
    )
    perf_hot_instruction_0_field_hints = str_field(
        perf_attribution, "hot_instruction_0_field_hints", "none"
    )
    perf_hot_instruction_0_context = str_field(
        perf_attribution, "hot_instruction_0_context", "none"
    )

    hot_field_names: list[str] = []
    for field in fields_from_hint(perf_top_instruction_field_hints):
        if field not in hot_field_names:
            hot_field_names.append(field)
    for field in fields_from_hint(perf_hot_instruction_0_field_hints):
        if field not in hot_field_names:
            hot_field_names.append(field)
    for field in fields_from_context(perf_hot_instruction_0_context):
        if field not in hot_field_names:
            hot_field_names.append(field)

    hot_field_bucket_names = [bucket_for_field(field) for field in hot_field_names]
    primitive_hot_state_field_count = sum(
        1 for bucket in hot_field_bucket_names if bucket == "primitive_hot_state"
    )
    public_or_proof_field_count = sum(
        1
        for bucket in hot_field_bucket_names
        if "public_semantics" in bucket or "proof_evidence" in bucket
    )
    observer_counter_field_count = sum(
        1 for bucket in hot_field_bucket_names if bucket == "observer_counter"
    )
    hot_field_top = hot_field_names[0] if hot_field_names else "none"
    hot_field_top_bucket = (
        bucket_for_field(hot_field_top) if hot_field_top != "none" else "none"
    )
    hot_field_plan_ready = int(
        inputs.perf_attribution_report_consumed
        and int_field(perf_attribution, "instruction_attribution_available", 0)
        and primitive_hot_state_field_count > 0
    )
    hot_field_next_bridge = (
        "record_state_residence_plan_report"
        if hot_field_plan_ready
        else (
            "collect_perf_field_hints"
            if inputs.perf_attribution_report_consumed
            else "run_hako_mimalloc_direct_exact_app_perf_asm"
        )
    )

    page_model_fields = page_model_field_names(inputs.page_box)
    record_state_static_candidates = [
        field for field in page_model_fields if bucket_for_field(field) == "primitive_hot_state"
    ]
    record_state_observed_candidates = [
        field for field in hot_field_names if bucket_for_field(field) == "primitive_hot_state"
    ]
    record_state_observed_rejections = [
        field for field in hot_field_names if bucket_for_field(field) != "primitive_hot_state"
    ]
    record_state_field_access_plan_count = int_field(
        state, "record_state_field_access_plan_count", 0
    )
    record_state_field_access_ready = int(
        inputs.state_report_consumed and record_state_field_access_plan_count > 0
    )
    record_state_report_ready = int(
        hot_field_plan_ready and bool(record_state_observed_candidates)
    )

    return CoverageFieldState(
        hot_field_names=hot_field_names,
        hot_field_top=hot_field_top,
        hot_field_top_bucket=hot_field_top_bucket,
        hot_field_buckets=format_field_buckets(hot_field_names),
        hot_field_primitive_hot_state_count=primitive_hot_state_field_count,
        hot_field_public_or_proof_count=public_or_proof_field_count,
        hot_field_observer_counter_count=observer_counter_field_count,
        hot_field_plan_ready=hot_field_plan_ready,
        hot_field_next_bridge=hot_field_next_bridge,
        record_state_static_candidate_fields=",".join(record_state_static_candidates)
        or "none",
        record_state_observed_candidate_fields=",".join(record_state_observed_candidates)
        or "none",
        record_state_rejected_observed_fields=format_field_buckets(
            record_state_observed_rejections
        ),
        record_state_field_access_ready=record_state_field_access_ready,
        record_state_report_ready=record_state_report_ready,
    )
