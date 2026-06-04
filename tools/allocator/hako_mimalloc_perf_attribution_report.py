"""Render perf attribution reports for mimalloc owner selection."""

from __future__ import annotations

import argparse
from pathlib import Path

from hako_mimalloc_perf_attribution_support import (
    I64_SIGNED_MAX,
    INLINE_OWNER_NEXT_BRIDGE,
    PerfSymbol,
    _append_unique,
    _context_fields_for_address,
    _context_for_address,
    _count_bucket,
    _count_public_or_proof,
    _dominant_inline_owner,
    _dominant_store_bucket,
    _field_hints_for_asm,
    _field_names_for_asm,
    _has_checked_public_accumulator_barrier,
    _instruction_category,
    _is_arithmetic_or_compare,
    _is_branch,
    _is_call,
    _is_memory,
    _is_store_like,
    _kv_bool,
    _read,
    _select_backend_store_shape,
    _select_directarray_owner_instruction_shape,
    _select_inline_owner_for_fields,
    _store_bucket_weights,
    _inline_owner_weights,
    _public_proof_accumulator_fields,
    _sum_matching,
    parse_annotated_instructions,
    parse_layout_field_offsets,
    parse_objdump_instructions,
    parse_perf_symbols,
)
from allocator_field_buckets import format_field_buckets


def emit_report(args: argparse.Namespace) -> str:
    perf_symbols = parse_perf_symbols(_read(args.perf_report))
    annotated = parse_annotated_instructions(_read(args.perf_annotate))
    objdump = parse_objdump_instructions(_read(args.objdump))
    field_offsets = parse_layout_field_offsets(
        _read(args.mir_json),
        args.layout_box,
        base_offset=args.layout_base_offset,
        field_stride=args.layout_field_stride,
    )

    top_symbol = perf_symbols[0] if perf_symbols else PerfSymbol(0.0, "")
    top_target_is_symbol = bool(args.symbol and top_symbol.symbol == args.symbol)
    symbol_collapse = top_target_is_symbol and top_symbol.percent >= args.collapse_threshold

    direct_array_symbol_pct = sum(
        symbol.percent
        for symbol in perf_symbols
        if "DirectArray" in symbol.symbol or "direct_array" in symbol.symbol
    )
    page_model_symbol_pct = sum(
        symbol.percent
        for symbol in perf_symbols
        if "HakoAllocPageModel" in symbol.symbol or "PageModel" in symbol.symbol
    )

    nonzero = [ins for ins in annotated if ins.percent > 0.0]
    top_instruction = max(nonzero, key=lambda ins: ins.percent, default=None)
    total_local = sum(ins.percent for ins in nonzero)
    hot_instructions = sorted(nonzero, key=lambda ins: ins.percent, reverse=True)[
        : args.hot_limit
    ]
    hot_store_fields: list[str] = []
    hot_context_fields: list[str] = []
    for ins in hot_instructions:
        if _is_store_like(ins):
            _append_unique(
                hot_store_fields,
                _field_names_for_asm(ins.asm, field_offsets),
            )
        _append_unique(
            hot_context_fields,
            _context_fields_for_address(
                objdump,
                ins.address,
                args.context_radius,
                field_offsets,
            ),
        )
    hot_store_bucket_weights = _store_bucket_weights(hot_instructions, field_offsets)
    hot_store_dominant_bucket = _dominant_store_bucket(hot_store_bucket_weights)
    hot_inline_owner_weights = _inline_owner_weights(
        hot_instructions,
        objdump,
        field_offsets,
        args.context_radius,
    )
    hot_inline_owner = _dominant_inline_owner(hot_inline_owner_weights)
    hot_inline_owner_next_bridge = INLINE_OWNER_NEXT_BRIDGE.get(
        hot_inline_owner,
        "rerun_perf_with_wider_context_or_symbol_split",
    )
    checked_public_accumulator_barrier = _has_checked_public_accumulator_barrier(
        hot_instructions,
        objdump,
        field_offsets,
        args.context_radius,
    )
    public_proof_accumulator_fields = _public_proof_accumulator_fields(
        hot_instructions,
        objdump,
        field_offsets,
        args.context_radius,
    )
    public_proof_accumulator_policy = (
        "checked_add_sign_guard"
        if checked_public_accumulator_barrier
        else "none"
        if not public_proof_accumulator_fields
        else "unclassified"
    )
    observed_no_overflow = bool(
        args.observed_requested_bytes is not None
        and 0 <= args.observed_requested_bytes <= I64_SIGNED_MAX
    )
    observed_margin = (
        I64_SIGNED_MAX - args.observed_requested_bytes
        if args.observed_requested_bytes is not None
        else None
    )
    split_ready = bool(
        hot_inline_owner != "none" and not checked_public_accumulator_barrier
    )
    split_blocker = (
        "checked_public_proof_accumulator_requires_overflow_policy"
        if checked_public_accumulator_barrier
        else "none"
        if split_ready
        else "missing_inlined_hot_body_candidate"
    )
    split_next_bridge = (
        "add_public_proof_accumulator_overflow_policy_before_source_reorder"
        if checked_public_accumulator_barrier
        else hot_inline_owner_next_bridge
    )
    backend_store_shape_selected, backend_store_shape_next_bridge = (
        _select_backend_store_shape(
            hot_store_fields,
            hot_context_fields,
            hot_store_dominant_bucket,
        )
    )
    directarray_owner_instruction_shape, directarray_owner_next_bridge = (
        _select_directarray_owner_instruction_shape(top_instruction, field_offsets)
    )

    symbol_attribution_available = (
        direct_array_symbol_pct > 0.0 or page_model_symbol_pct > 0.0
    )
    instruction_attribution_available = bool(nonzero)
    perf_delta_ready = bool(symbol_attribution_available)
    blocker = "none"
    next_bridge = "owner_delta_measurement"
    if not perf_delta_ready:
        if symbol_collapse and instruction_attribution_available:
            blocker = "ny_main_symbol_collapse"
            next_bridge = "asm_instruction_classifier_or_in_process_perf_mode"
        elif not instruction_attribution_available:
            blocker = "missing_perf_annotate_samples"
            next_bridge = "rerun_perf_with_higher_repeat_or_symbol"
        else:
            blocker = "missing_directarray_or_pagemodel_symbol_attribution"
            next_bridge = "asm_instruction_classifier_or_symbol_split"

    lines = [
        "output_contract=hako-mimalloc-perf-attribution-v0",
        f"perf_report={args.perf_report or ''}",
        f"perf_annotate={args.perf_annotate or ''}",
        f"target_symbol={args.symbol}",
        f"top_symbol={top_symbol.symbol}",
        f"top_symbol_percent={top_symbol.percent:.2f}",
        f"top_symbol_is_target={_kv_bool(top_target_is_symbol)}",
        f"symbol_collapse_detected={_kv_bool(symbol_collapse)}",
        f"symbol_attribution_available={_kv_bool(symbol_attribution_available)}",
        f"direct_array_symbol_percent={direct_array_symbol_pct:.2f}",
        f"page_model_symbol_percent={page_model_symbol_pct:.2f}",
        f"instruction_attribution_available={_kv_bool(instruction_attribution_available)}",
        f"annotate_nonzero_instruction_count={len(nonzero)}",
        f"annotate_total_local_percent={total_local:.2f}",
        f"annotate_branch_percent={_sum_matching(nonzero, _is_branch):.2f}",
        f"annotate_call_percent={_sum_matching(nonzero, _is_call):.2f}",
        f"annotate_memory_percent={_sum_matching(nonzero, _is_memory):.2f}",
        f"annotate_store_like_percent={_sum_matching(nonzero, _is_store_like):.2f}",
        f"annotate_arithmetic_compare_percent={_sum_matching(nonzero, _is_arithmetic_or_compare):.2f}",
        f"layout_hint_box={args.layout_box or 'none'}",
        f"layout_hint_field_count={len(field_offsets)}",
        f"layout_hint_base_offset=0x{args.layout_base_offset:x}",
        f"layout_hint_field_stride=0x{args.layout_field_stride:x}",
        f"page_model_hot_array_perf_delta_measurement_plan_v0=1",
        f"page_model_hot_array_perf_delta_ready={_kv_bool(perf_delta_ready)}",
        f"page_model_hot_array_perf_delta_blocker={blocker}",
        f"page_model_hot_array_perf_delta_next_bridge={next_bridge}",
        "backend_store_shape_classifier_v0=1",
        f"backend_store_shape_ready={_kv_bool(bool(hot_store_fields or hot_context_fields))}",
        f"backend_store_shape_selected={backend_store_shape_selected}",
        f"backend_store_shape_next_bridge={backend_store_shape_next_bridge}",
        f"backend_store_shape_hot_store_fields={','.join(hot_store_fields) or 'none'}",
        f"backend_store_shape_hot_store_field_buckets={format_field_buckets(hot_store_fields)}",
        f"backend_store_shape_context_fields={','.join(hot_context_fields) or 'none'}",
        f"backend_store_shape_context_field_buckets={format_field_buckets(hot_context_fields)}",
        f"backend_store_shape_primitive_hot_state_field_count={_count_bucket(hot_store_fields, 'primitive_hot_state')}",
        f"backend_store_shape_public_or_proof_field_count={_count_public_or_proof(hot_store_fields)}",
        f"backend_store_shape_direct_array_owner_field_count={_count_bucket(hot_store_fields, 'direct_array_owner')}",
        f"backend_store_shape_weighted_dominant_bucket={hot_store_dominant_bucket}",
        f"backend_store_shape_primitive_hot_state_store_percent={hot_store_bucket_weights.get('primitive_hot_state', 0.0):.2f}",
        f"backend_store_shape_public_or_proof_store_percent={hot_store_bucket_weights.get('public_or_proof', 0.0):.2f}",
        f"backend_store_shape_direct_array_owner_store_percent={hot_store_bucket_weights.get('direct_array_owner', 0.0):.2f}",
        f"backend_store_shape_observer_counter_store_percent={hot_store_bucket_weights.get('observer_counter', 0.0):.2f}",
        f"backend_store_shape_unknown_store_percent={hot_store_bucket_weights.get('unknown', 0.0):.2f}",
        "directarray_owner_instruction_shape_classifier_v0=1",
        f"directarray_owner_instruction_shape_selected={directarray_owner_instruction_shape}",
        f"directarray_owner_instruction_shape_next_bridge={directarray_owner_next_bridge}",
        "inlined_hot_body_classifier_v0=1",
        f"inlined_hot_body_selected={hot_inline_owner}",
        f"inlined_hot_body_next_bridge={hot_inline_owner_next_bridge}",
        f"inlined_hot_body_acquire_fresh_small_percent={hot_inline_owner_weights.get('acquire_fresh_small_like', 0.0):.2f}",
        f"inlined_hot_body_release_local_known_live_percent={hot_inline_owner_weights.get('release_local_known_live_like', 0.0):.2f}",
        f"inlined_hot_body_init_public_store_percent={hot_inline_owner_weights.get('init_public_store_like', 0.0):.2f}",
        f"inlined_hot_body_direct_array_owner_init_percent={hot_inline_owner_weights.get('direct_array_owner_init_like', 0.0):.2f}",
        f"inlined_hot_body_mixed_percent={hot_inline_owner_weights.get('mixed_hot_body_like', 0.0):.2f}",
        f"inlined_hot_body_unknown_percent={hot_inline_owner_weights.get('none', 0.0):.2f}",
        f"inlined_hot_body_split_ready={_kv_bool(split_ready)}",
        f"inlined_hot_body_split_blocker={split_blocker}",
        f"inlined_hot_body_split_next_bridge={split_next_bridge}",
        "public_proof_accumulator_plan_v0=1",
        f"public_proof_accumulator_fields={','.join(public_proof_accumulator_fields) or 'none'}",
        f"public_proof_accumulator_policy={public_proof_accumulator_policy}",
        f"public_proof_accumulator_source_reorder_allowed={_kv_bool(not checked_public_accumulator_barrier)}",
        f"public_proof_accumulator_observed_requested_bytes={args.observed_requested_bytes if args.observed_requested_bytes is not None else 'none'}",
        f"public_proof_accumulator_observed_no_overflow={_kv_bool(observed_no_overflow)}",
        f"public_proof_accumulator_observed_i64_margin={observed_margin if observed_margin is not None else 'none'}",
        "public_proof_accumulator_general_no_overflow_proof=0",
        f"public_proof_accumulator_next_bridge={split_next_bridge}",
    ]
    if top_instruction is not None:
        lines.extend(
            [
                f"top_instruction_percent={top_instruction.percent:.2f}",
                f"top_instruction_address={top_instruction.address}",
                f"top_instruction_mnemonic={top_instruction.mnemonic}",
                f"top_instruction_category={_instruction_category(top_instruction)}",
                f"top_instruction_field_hints={_field_hints_for_asm(top_instruction.asm, field_offsets)}",
                f"top_instruction_asm={top_instruction.asm}",
            ]
        )
    else:
        lines.extend(
            [
                "top_instruction_percent=0.00",
                "top_instruction_address=",
                "top_instruction_mnemonic=",
                "top_instruction_category=none",
                "top_instruction_field_hints=none",
                "top_instruction_asm=",
            ]
        )
    lines.append(f"hot_instruction_report_limit={args.hot_limit}")
    lines.append(f"hot_instruction_report_count={len(hot_instructions)}")
    lines.append(f"hot_instruction_context_radius={args.context_radius}")
    for idx, ins in enumerate(hot_instructions):
        prefix = f"hot_instruction_{idx}"
        context, context_count, context_categories = _context_for_address(
            objdump, ins.address, args.context_radius, field_offsets
        )
        context_fields = _context_fields_for_address(
            objdump, ins.address, args.context_radius, field_offsets
        )
        inline_owner = _select_inline_owner_for_fields(context_fields)
        lines.extend(
            [
                f"{prefix}_percent={ins.percent:.2f}",
                f"{prefix}_address={ins.address}",
                f"{prefix}_mnemonic={ins.mnemonic}",
                f"{prefix}_category={_instruction_category(ins)}",
                f"{prefix}_field_hints={_field_hints_for_asm(ins.asm, field_offsets)}",
                f"{prefix}_inlined_owner_candidate={inline_owner}",
                f"{prefix}_asm={ins.asm}",
                f"{prefix}_context_count={context_count}",
                f"{prefix}_context_categories={context_categories}",
                f"{prefix}_context={context}",
            ]
        )
    lines.append("summary=ok")
    return "\n".join(lines) + "\n"
