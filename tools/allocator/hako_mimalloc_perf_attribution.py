#!/usr/bin/env python3
"""Summarize whether a mimalloc perf artifact can select the next owner.

This tool intentionally separates two attribution levels:

* symbol attribution from `perf report`
* instruction attribution from `perf annotate`

The current AOT direct-exact app often collapses hot samples into `ny_main`.
That is useful evidence, but it is not enough to claim a DirectArray or
PageModel-specific perf delta. The report below makes that boundary explicit.
"""

from __future__ import annotations

import argparse
import json
import re
from dataclasses import dataclass
from pathlib import Path

from allocator_field_buckets import (
    bucket_for_field,
    fields_from_hint,
    format_field_buckets,
)


PERCENT_RE = r"(?P<pct>[0-9]+(?:\.[0-9]+)?)%"
PERF_REPORT_RE = re.compile(
    rf"^\s*{PERCENT_RE}\s+\S+\s+\S+\s+\[\.\]\s+(?P<symbol>.+?)\s*$"
)
ANNOTATE_RE = re.compile(
    r"^\s*(?P<pct>[0-9]+(?:\.[0-9]+)?)\s*:\s+"
    r"(?P<addr>[0-9a-fA-F]+):\s+"
    r"(?P<asm>.+?)\s*$"
)
OBJDUMP_RE = re.compile(
    r"^\s*(?P<addr>[0-9a-fA-F]+):\s+"
    r"(?P<bytes>(?:[0-9a-fA-F]{2}\s+)+)\s*"
    r"(?P<asm>.+?)\s*$"
)
MEMORY_OPERAND_RE = re.compile(
    r"(?P<offset>-?(?:0x[0-9a-fA-F]+|\d+))?\((?P<body>[^)]*)\)"
)


@dataclass(frozen=True)
class PerfSymbol:
    percent: float
    symbol: str


@dataclass(frozen=True)
class AnnotatedInstruction:
    percent: float
    address: str
    asm: str

    @property
    def mnemonic(self) -> str:
        return self.asm.strip().split(None, 1)[0] if self.asm.strip() else ""


@dataclass(frozen=True)
class ObjdumpInstruction:
    address: str
    asm: str

    @property
    def mnemonic(self) -> str:
        return self.asm.strip().split(None, 1)[0] if self.asm.strip() else ""


def _read(path: Path | None) -> str:
    if path is None:
        return ""
    return path.read_text(encoding="utf-8", errors="replace")


def parse_perf_symbols(text: str) -> list[PerfSymbol]:
    symbols: list[PerfSymbol] = []
    for line in text.splitlines():
        match = PERF_REPORT_RE.match(line)
        if not match:
            continue
        symbols.append(
            PerfSymbol(
                percent=float(match.group("pct")),
                symbol=match.group("symbol").strip(),
            )
        )
    return symbols


def parse_annotated_instructions(text: str) -> list[AnnotatedInstruction]:
    instructions: list[AnnotatedInstruction] = []
    for line in text.splitlines():
        match = ANNOTATE_RE.match(line)
        if not match:
            continue
        instructions.append(
            AnnotatedInstruction(
                percent=float(match.group("pct")),
                address=match.group("addr").lower(),
                asm=match.group("asm").strip(),
            )
        )
    return instructions


def parse_objdump_instructions(text: str) -> list[ObjdumpInstruction]:
    instructions: list[ObjdumpInstruction] = []
    for line in text.splitlines():
        match = OBJDUMP_RE.match(line)
        if not match:
            continue
        instructions.append(
            ObjdumpInstruction(
                address=match.group("addr").lower(),
                asm=match.group("asm").strip(),
            )
        )
    return instructions


def parse_layout_field_offsets(
    text: str,
    box_name: str,
    *,
    base_offset: int,
    field_stride: int,
) -> dict[int, str]:
    if not text or not box_name:
        return {}
    try:
        payload = json.loads(text)
    except json.JSONDecodeError:
        return {}
    decls = payload.get("user_box_decls") if isinstance(payload, dict) else None
    if not isinstance(decls, list):
        return {}
    for decl in decls:
        if not isinstance(decl, dict):
            continue
        if decl.get("name") != box_name and decl.get("box_name") != box_name:
            continue
        fields = decl.get("field_decls")
        if not isinstance(fields, list):
            return {}
        offsets: dict[int, str] = {}
        for field in fields:
            if not isinstance(field, dict):
                continue
            name = field.get("name")
            index = field.get("field_index")
            if not isinstance(name, str) or not isinstance(index, int):
                continue
            offsets[base_offset + index * field_stride] = name
        return offsets
    return {}


def _parse_int_literal(value: str) -> int | None:
    try:
        return int(value, 0)
    except ValueError:
        return None


def _field_hints_for_asm(asm: str, field_offsets: dict[int, str]) -> str:
    if not field_offsets:
        return "none"
    hints: list[str] = []
    seen: set[int] = set()
    for match in MEMORY_OPERAND_RE.finditer(asm):
        # PageModel field hints only apply to simple object-slot operands such
        # as `0xa0(%rax)`. Scaled operands like `0x20(%rdx,%rsi,8)` are
        # DirectArray element accesses and must not be re-labeled as box fields.
        if "," in match.group("body"):
            continue
        raw = match.group("offset") or "0"
        offset = _parse_int_literal(raw)
        if offset is None or offset in seen:
            continue
        seen.add(offset)
        name = field_offsets.get(offset)
        if name is None:
            continue
        hints.append(f"0x{offset:x}:{name}")
    return ",".join(hints) or "none"


def _field_names_for_asm(asm: str, field_offsets: dict[int, str]) -> list[str]:
    return fields_from_hint(_field_hints_for_asm(asm, field_offsets))


def _append_unique(target: list[str], values: list[str]) -> None:
    for value in values:
        if value not in target:
            target.append(value)


def _context_fields_for_address(
    objdump: list[ObjdumpInstruction],
    address: str,
    radius: int,
    field_offsets: dict[int, str],
) -> list[str]:
    if not objdump or not address:
        return []
    index_by_address = {ins.address: idx for idx, ins in enumerate(objdump)}
    idx = index_by_address.get(address.lower())
    if idx is None:
        return []
    start = max(0, idx - radius)
    end = min(len(objdump), idx + radius + 1)
    fields: list[str] = []
    for ins in objdump[start:end]:
        _append_unique(fields, _field_names_for_asm(ins.asm, field_offsets))
    return fields


def _count_bucket(fields: list[str], bucket: str) -> int:
    return sum(1 for field in fields if bucket_for_field(field) == bucket)


def _count_public_or_proof(fields: list[str]) -> int:
    return sum(
        1
        for field in fields
        if "public_semantics" in bucket_for_field(field)
        or "proof_evidence" in bucket_for_field(field)
    )


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


def _sum_matching(
    instructions: list[AnnotatedInstruction],
    predicate,
) -> float:
    return sum(ins.percent for ins in instructions if predicate(ins))


def _is_branch(ins: AnnotatedInstruction) -> bool:
    mnemonic = ins.mnemonic
    return mnemonic.startswith("j") or mnemonic in {"loop", "ret"}


def _is_call(ins: AnnotatedInstruction) -> bool:
    return ins.mnemonic.startswith("call")


def _is_memory(ins: AnnotatedInstruction) -> bool:
    return "(" in ins.asm or "[" in ins.asm


def _is_store_like(ins: AnnotatedInstruction) -> bool:
    mnemonic = ins.mnemonic
    asm = ins.asm
    if mnemonic.startswith(("cmp", "test")):
        return False
    if mnemonic in {"inc", "incq", "incl", "dec", "decq", "decl"}:
        return _is_memory(ins)
    if "," not in asm:
        return False
    dest = asm.rsplit(",", 1)[-1]
    return "(" in dest or "[" in dest


def _is_arithmetic_or_compare(ins: AnnotatedInstruction) -> bool:
    mnemonic = ins.mnemonic
    return mnemonic.startswith(
        (
            "add",
            "sub",
            "inc",
            "dec",
            "cmp",
            "test",
            "and",
            "or",
            "xor",
            "cmov",
        )
    )


def _instruction_category(ins: AnnotatedInstruction) -> str:
    if _is_call(ins):
        return "call"
    if _is_branch(ins):
        return "branch"
    if _is_store_like(ins):
        return "store_like"
    if _is_memory(ins):
        return "memory"
    if _is_arithmetic_or_compare(ins):
        return "arithmetic_compare"
    return "other"


def _instruction_category_from_asm(asm: str) -> str:
    return _instruction_category(AnnotatedInstruction(0.0, "", asm))


def _sanitize_context(value: str) -> str:
    return value.replace("|", "/").replace("=", "~")


def _context_for_address(
    objdump: list[ObjdumpInstruction],
    address: str,
    radius: int,
    field_offsets: dict[int, str],
) -> tuple[str, str, str]:
    if not objdump or not address:
        return "", "0", "none"
    index_by_address = {ins.address: idx for idx, ins in enumerate(objdump)}
    idx = index_by_address.get(address.lower())
    if idx is None:
        return "", "0", "not_found"
    start = max(0, idx - radius)
    end = min(len(objdump), idx + radius + 1)
    window = objdump[start:end]
    encoded = "|".join(
        f"{ins.address}:{_instruction_category_from_asm(ins.asm)}:"
        f"{_field_hints_for_asm(ins.asm, field_offsets)}:"
        f"{_sanitize_context(ins.asm)}"
        for ins in window
    )
    categories = ",".join(sorted({_instruction_category_from_asm(ins.asm) for ins in window}))
    return encoded, str(len(window)), categories or "none"


def _kv_bool(value: bool) -> str:
    return "1" if value else "0"


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
    backend_store_shape_selected, backend_store_shape_next_bridge = (
        _select_backend_store_shape(
            hot_store_fields,
            hot_context_fields,
            hot_store_dominant_bucket,
        )
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
        lines.extend(
            [
                f"{prefix}_percent={ins.percent:.2f}",
                f"{prefix}_address={ins.address}",
                f"{prefix}_mnemonic={ins.mnemonic}",
                f"{prefix}_category={_instruction_category(ins)}",
                f"{prefix}_field_hints={_field_hints_for_asm(ins.asm, field_offsets)}",
                f"{prefix}_asm={ins.asm}",
                f"{prefix}_context_count={context_count}",
                f"{prefix}_context_categories={context_categories}",
                f"{prefix}_context={context}",
            ]
        )
    lines.append("summary=ok")
    return "\n".join(lines) + "\n"


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--perf-report", type=Path)
    parser.add_argument("--perf-annotate", type=Path)
    parser.add_argument("--objdump", type=Path)
    parser.add_argument("--mir-json", type=Path)
    parser.add_argument("--layout-box", default="")
    parser.add_argument("--layout-base-offset", type=lambda s: int(s, 0), default=0x20)
    parser.add_argument("--layout-field-stride", type=lambda s: int(s, 0), default=0x10)
    parser.add_argument("--symbol", default="ny_main")
    parser.add_argument("--collapse-threshold", type=float, default=90.0)
    parser.add_argument("--hot-limit", type=int, default=8)
    parser.add_argument("--context-radius", type=int, default=3)
    args = parser.parse_args()
    if args.hot_limit < 1:
        parser.error("--hot-limit must be >= 1")
    if args.context_radius < 0:
        parser.error("--context-radius must be >= 0")
    print(emit_report(args), end="")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
