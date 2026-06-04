"""Owner/shape selection helpers for mimalloc perf attribution reports."""

from __future__ import annotations

from hako_mimalloc_perf_attribution_support import (
    AnnotatedInstruction,
    ObjdumpInstruction,
    _context_fields_for_address,
    _context_window,
    _count_bucket,
    _count_public_or_proof,
    _field_hints_for_asm,
    _field_names_for_asm,
    _is_store_like,
)
from allocator_field_buckets import bucket_for_field, fields_from_hint


INLINE_OWNER_MOTIFS: dict[str, tuple[str, ...]] = {
    "acquire_fresh_small_like": (
        "requested_bytes",
        "free_top",
        "peak_used",
        "used",
    ),
    "release_local_known_live_like": (
        "local_free_top",
        "local_free_count",
        "used",
        "retired",
        "retire_count",
    ),
    "init_public_store_like": (
        "page_id",
        "block_size",
        "capacity",
        "reserved",
    ),
    "direct_array_owner_init_like": (
        "free",
        "local_free",
        "block_used",
    ),
}

INLINE_OWNER_NEXT_BRIDGE: dict[str, str] = {
    "acquire_fresh_small_like": "split_public_proof_stores_from_acquire_fresh_small_like_body",
    "release_local_known_live_like": "split_observer_or_retire_stores_from_release_like_body",
    "init_public_store_like": "sink_or_outline_public_init_store_cluster",
    "direct_array_owner_init_like": "classify_directarray_owner_init_store_cluster",
    "mixed_hot_body_like": "split_or_sink_public_init_stores_around_primitive_hot_state_body",
    "none": "rerun_perf_with_wider_context_or_symbol_split",
}


def _store_bucket_for_fields(fields: list[str]) -> str:
    if not fields:
        return "unknown"
    buckets = [bucket_for_field(field) for field in fields]
    if any(bucket == "primitive_hot_state" for bucket in buckets):
        return "primitive_hot_state"
    if any("public_semantics" in bucket or "proof_evidence" in bucket for bucket in buckets):
        return "public_or_proof"
    if any(bucket == "direct_array_owner" for bucket in buckets):
        return "direct_array_owner"
    if any(bucket == "observer_counter" for bucket in buckets):
        return "observer_counter"
    return "unknown"


def _store_bucket_weights(
    instructions: list[AnnotatedInstruction],
    field_offsets: dict[int, str],
) -> dict[str, float]:
    weights = {
        "primitive_hot_state": 0.0,
        "public_or_proof": 0.0,
        "direct_array_owner": 0.0,
        "observer_counter": 0.0,
        "unknown": 0.0,
    }
    for ins in instructions:
        if not _is_store_like(ins):
            continue
        fields = _field_names_for_asm(ins.asm, field_offsets)
        bucket = _store_bucket_for_fields(fields)
        weights[bucket] = weights.get(bucket, 0.0) + ins.percent
    return weights


def _dominant_store_bucket(weights: dict[str, float]) -> str:
    if not weights:
        return "none"
    bucket, value = max(weights.items(), key=lambda item: item[1])
    return bucket if value > 0.0 else "none"


def _owner_scores_for_fields(fields: list[str]) -> dict[str, int]:
    present = set(fields)
    return {
        owner: sum(1 for field in motif_fields if field in present)
        for owner, motif_fields in INLINE_OWNER_MOTIFS.items()
    }


def _select_inline_owner_for_fields(fields: list[str]) -> str:
    scores = _owner_scores_for_fields(fields)
    if not scores:
        return "none"
    best_score = max(scores.values())
    if best_score <= 0:
        return "none"
    winners = [owner for owner, score in scores.items() if score == best_score]
    if len(winners) > 1:
        return "mixed_hot_body_like"
    return winners[0]


def _inline_owner_weights(
    instructions: list[AnnotatedInstruction],
    objdump: list[ObjdumpInstruction],
    field_offsets: dict[int, str],
    context_radius: int,
) -> dict[str, float]:
    weights = {owner: 0.0 for owner in INLINE_OWNER_MOTIFS}
    weights["mixed_hot_body_like"] = 0.0
    weights["none"] = 0.0
    for ins in instructions:
        fields = _context_fields_for_address(
            objdump,
            ins.address,
            context_radius,
            field_offsets,
        )
        owner = _select_inline_owner_for_fields(fields)
        weights[owner] = weights.get(owner, 0.0) + ins.percent
    return weights


def _dominant_inline_owner(weights: dict[str, float]) -> str:
    if not weights:
        return "none"
    owner, value = max(weights.items(), key=lambda item: item[1])
    return owner if value > 0.0 else "none"


def _has_checked_public_accumulator_barrier(
    instructions: list[AnnotatedInstruction],
    objdump: list[ObjdumpInstruction],
    field_offsets: dict[int, str],
    context_radius: int,
) -> bool:
    for ins in instructions:
        fields = _context_fields_for_address(
            objdump,
            ins.address,
            context_radius,
            field_offsets,
        )
        if "requested_bytes" not in fields:
            continue
        if _select_inline_owner_for_fields(fields) != "acquire_fresh_small_like":
            continue
        window = _context_window(objdump, ins.address, context_radius)
        has_requested_bytes = any(
            "requested_bytes" in _field_names_for_asm(row.asm, field_offsets)
            for row in window
        )
        has_overflow_branch = any(row.mnemonic == "js" for row in window)
        if has_requested_bytes and has_overflow_branch:
            return True
    return False


def _public_proof_accumulator_fields(
    instructions: list[AnnotatedInstruction],
    objdump: list[ObjdumpInstruction],
    field_offsets: dict[int, str],
    context_radius: int,
) -> list[str]:
    fields: list[str] = []
    for ins in instructions:
        context_fields = _context_fields_for_address(
            objdump,
            ins.address,
            context_radius,
            field_offsets,
        )
        if _select_inline_owner_for_fields(context_fields) != "acquire_fresh_small_like":
            continue
        for field in context_fields:
            if "public_semantics" not in bucket_for_field(field):
                continue
            if "proof_evidence" not in bucket_for_field(field):
                continue
            if field not in fields:
                fields.append(field)
    return fields


def _select_backend_store_shape(
    store_fields: list[str],
    context_fields: list[str],
    weighted_dominant_bucket: str,
) -> tuple[str, str]:
    fields = store_fields or context_fields
    if not fields:
        return "none", "rerun_perf_with_context_or_symbol_split"
    primitive = _count_bucket(fields, "primitive_hot_state")
    public_or_proof = _count_public_or_proof(fields)
    direct_array = _count_bucket(fields, "direct_array_owner")
    if direct_array > 0 and primitive > 0:
        if weighted_dominant_bucket == "direct_array_owner":
            return (
                "direct_array_dominant_mixed_store_shape",
                "classify_directarray_owner_instruction_shape",
            )
        if weighted_dominant_bucket == "primitive_hot_state":
            return (
                "primitive_dominant_directarray_mixed_store_shape",
                "classify_backend_store_shape_for_state_write_elision",
            )
        return (
            "mixed_primitive_and_directarray_store_shape",
            "split_directarray_owner_stores_from_primitive_hot_state_stores",
        )
    if primitive > 0 and public_or_proof > 0:
        if weighted_dominant_bucket == "primitive_hot_state":
            return (
                "primitive_dominant_mixed_store_shape",
                "split_or_sink_public_init_stores_around_primitive_hot_state_body",
            )
        return (
            "mixed_primitive_and_public_store_shape",
            "split_init_public_stores_from_primitive_hot_state_stores",
        )
    if primitive > 0:
        return (
            "primitive_hot_state_store_shape",
            "classify_backend_store_shape_for_state_write_elision",
        )
    if public_or_proof > 0:
        return (
            "public_or_proof_store_shape",
            "separate_init_or_proof_stores_from_hot_lifecycle_body",
        )
    if direct_array > 0:
        return (
            "direct_array_owner_store_shape",
            "classify_directarray_owner_instruction_shape",
        )
    return "unknown_store_shape", "rerun_perf_with_wider_context"


def _select_directarray_owner_instruction_shape(
    instruction: AnnotatedInstruction | None,
    field_offsets: dict[int, str],
) -> tuple[str, str]:
    if instruction is None:
        return "none", "collect_directarray_owner_instruction"
    fields = fields_from_hint(_field_hints_for_asm(instruction.asm, field_offsets))
    if not fields or not any(
        bucket_for_field(field) == "direct_array_owner" for field in fields
    ):
        return "none", "collect_directarray_owner_instruction"
    if instruction.mnemonic in {"incq", "decq"}:
        return (
            "directarray_owner_handle_field_refcount_like",
            "classify_handle_field_materialization_or_owner_handle_loads",
        )
    if _is_store_like(instruction):
        return (
            "directarray_owner_handle_field_store_like",
            "classify_directarray_owner_handle_store_site",
        )
    return (
        "directarray_owner_field_access_like",
        "classify_directarray_owner_field_access_site",
    )
