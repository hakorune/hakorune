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


INLINE_OWNER_MOTIFS: dict[str, tuple[str, ...]] = {
    # These are intentionally "like" classifiers. They use PageModel layout
    # hints from surrounding asm context and do not prove the base object.
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


def _context_window(
    objdump: list[ObjdumpInstruction],
    address: str,
    radius: int,
) -> list[ObjdumpInstruction]:
    if not objdump or not address:
        return []
    index_by_address = {ins.address: idx for idx, ins in enumerate(objdump)}
    idx = index_by_address.get(address.lower())
    if idx is None:
        return []
    start = max(0, idx - radius)
    end = min(len(objdump), idx + radius + 1)
    return objdump[start:end]


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
        # The current acquire-like body has `add requested_bytes` followed by
        # a sign/overflow guard (`js`). Sinking that public/proof accumulator
        # past primitive stores needs an explicit overflow policy first.
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


I64_SIGNED_MAX = (1 << 63) - 1


def emit_report(args: argparse.Namespace) -> str:
    from hako_mimalloc_perf_attribution_report import emit_report as _emit_report

    return _emit_report(args)


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
    parser.add_argument("--observed-requested-bytes", type=int)
    args = parser.parse_args()
    if args.hot_limit < 1:
        parser.error("--hot-limit must be >= 1")
    if args.context_radius < 0:
        parser.error("--context-radius must be >= 0")
    print(emit_report(args), end="")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
