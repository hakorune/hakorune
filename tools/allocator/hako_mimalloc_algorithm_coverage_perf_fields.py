"""Perf attribution report fields for mimalloc algorithm coverage reports."""

from __future__ import annotations

from typing import Mapping

from hako_mimalloc_algorithm_coverage_measurement_state import CoverageMeasurementState
from hako_mimalloc_algorithm_coverage_support import int_field, str_field


def build_perf_report_fields(
    measurement_state: CoverageMeasurementState,
    perf_attribution: Mapping[str, object],
) -> dict[str, object]:
    return {
        "perf_top_symbol": str_field(perf_attribution, "top_symbol", "none"),
        "perf_top_symbol_percent": str_field(perf_attribution, "top_symbol_percent", "0.00"),
        "perf_symbol_collapse_detected": int_field(
            perf_attribution, "symbol_collapse_detected", 0
        ),
        "perf_symbol_attribution_available": int_field(
            perf_attribution, "symbol_attribution_available", 0
        ),
        "perf_instruction_attribution_available": int_field(
            perf_attribution, "instruction_attribution_available", 0
        ),
        "perf_annotate_nonzero_instruction_count": int_field(
            perf_attribution, "annotate_nonzero_instruction_count", 0
        ),
        "perf_top_instruction_percent": str_field(
            perf_attribution, "top_instruction_percent", "0.00"
        ),
        "perf_top_instruction_mnemonic": str_field(
            perf_attribution, "top_instruction_mnemonic", "none"
        ),
        "perf_top_instruction_category": str_field(
            perf_attribution, "top_instruction_category", "none"
        ),
        "perf_top_instruction_field_hints": measurement_state.perf_top_instruction_field_hints,
        "perf_hot_instruction_report_count": int_field(
            perf_attribution, "hot_instruction_report_count", 0
        ),
        "perf_hot_instruction_0_category": str_field(
            perf_attribution, "hot_instruction_0_category", "none"
        ),
        "perf_hot_instruction_0_field_hints": measurement_state.perf_hot_instruction_0_field_hints,
        "perf_hot_instruction_0_asm": str_field(
            perf_attribution, "hot_instruction_0_asm", "none"
        ),
        "perf_hot_instruction_0_context_categories": str_field(
            perf_attribution, "hot_instruction_0_context_categories", "none"
        ),
        "perf_hot_instruction_0_context_count": int_field(
            perf_attribution, "hot_instruction_0_context_count", 0
        ),
        "perf_backend_store_shape_classifier_v0": int_field(
            perf_attribution, "backend_store_shape_classifier_v0", 0
        ),
        "perf_backend_store_shape_ready": measurement_state.backend_store_shape_ready,
        "perf_backend_store_shape_selected": measurement_state.backend_store_shape_selected,
        "perf_backend_store_shape_next_bridge": measurement_state.backend_store_shape_next_bridge,
        "perf_backend_store_shape_hot_store_field_buckets": str_field(
            perf_attribution, "backend_store_shape_hot_store_field_buckets", "none"
        ),
        "perf_backend_store_shape_context_field_buckets": str_field(
            perf_attribution, "backend_store_shape_context_field_buckets", "none"
        ),
        "perf_backend_store_shape_weighted_dominant_bucket": str_field(
            perf_attribution, "backend_store_shape_weighted_dominant_bucket", "none"
        ),
        "perf_backend_store_shape_primitive_hot_state_store_percent": str_field(
            perf_attribution,
            "backend_store_shape_primitive_hot_state_store_percent",
            "0.00",
        ),
        "perf_backend_store_shape_public_or_proof_store_percent": str_field(
            perf_attribution,
            "backend_store_shape_public_or_proof_store_percent",
            "0.00",
        ),
        "perf_backend_store_shape_direct_array_owner_store_percent": str_field(
            perf_attribution,
            "backend_store_shape_direct_array_owner_store_percent",
            "0.00",
        ),
        "perf_backend_store_shape_observer_counter_store_percent": str_field(
            perf_attribution,
            "backend_store_shape_observer_counter_store_percent",
            "0.00",
        ),
        "perf_directarray_owner_instruction_shape_classifier_v0": int_field(
            perf_attribution, "directarray_owner_instruction_shape_classifier_v0", 0
        ),
        "perf_directarray_owner_instruction_shape_selected": measurement_state.directarray_owner_instruction_shape_selected,
        "perf_directarray_owner_instruction_shape_next_bridge": measurement_state.directarray_owner_instruction_shape_next_bridge,
        "perf_inlined_hot_body_classifier_v0": int_field(
            perf_attribution, "inlined_hot_body_classifier_v0", 0
        ),
        "perf_inlined_hot_body_selected": measurement_state.inlined_hot_body_selected,
        "perf_inlined_hot_body_next_bridge": measurement_state.inlined_hot_body_next_bridge,
        "perf_inlined_hot_body_split_ready": int_field(
            perf_attribution, "inlined_hot_body_split_ready", 0
        ),
        "perf_inlined_hot_body_split_blocker": str_field(
            perf_attribution, "inlined_hot_body_split_blocker", "none"
        ),
        "perf_inlined_hot_body_split_next_bridge": measurement_state.inlined_hot_body_split_next_bridge,
        "perf_inlined_hot_body_acquire_fresh_small_percent": str_field(
            perf_attribution,
            "inlined_hot_body_acquire_fresh_small_percent",
            "0.00",
        ),
        "perf_inlined_hot_body_release_local_known_live_percent": str_field(
            perf_attribution,
            "inlined_hot_body_release_local_known_live_percent",
            "0.00",
        ),
        "perf_inlined_hot_body_init_public_store_percent": str_field(
            perf_attribution,
            "inlined_hot_body_init_public_store_percent",
            "0.00",
        ),
        "perf_inlined_hot_body_mixed_percent": str_field(
            perf_attribution,
            "inlined_hot_body_mixed_percent",
            "0.00",
        ),
        "perf_public_proof_accumulator_plan_v0": int_field(
            perf_attribution, "public_proof_accumulator_plan_v0", 0
        ),
        "perf_public_proof_accumulator_fields": str_field(
            perf_attribution, "public_proof_accumulator_fields", "none"
        ),
        "perf_public_proof_accumulator_policy": str_field(
            perf_attribution, "public_proof_accumulator_policy", "none"
        ),
        "perf_public_proof_accumulator_source_reorder_allowed": int_field(
            perf_attribution, "public_proof_accumulator_source_reorder_allowed", 0
        ),
        "perf_public_proof_accumulator_observed_requested_bytes": str_field(
            perf_attribution, "public_proof_accumulator_observed_requested_bytes", "none"
        ),
        "perf_public_proof_accumulator_observed_no_overflow": int_field(
            perf_attribution, "public_proof_accumulator_observed_no_overflow", 0
        ),
        "perf_public_proof_accumulator_observed_i64_margin": str_field(
            perf_attribution, "public_proof_accumulator_observed_i64_margin", "none"
        ),
        "perf_public_proof_accumulator_general_no_overflow_proof": int_field(
            perf_attribution, "public_proof_accumulator_general_no_overflow_proof", 0
        ),
        "perf_public_proof_accumulator_next_bridge": str_field(
            perf_attribution, "public_proof_accumulator_next_bridge", "none"
        ),
    }
