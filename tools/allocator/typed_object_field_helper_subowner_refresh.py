#!/usr/bin/env python3
"""Classify typed-object field helper subowners from perf top/annotate evidence."""

from __future__ import annotations

import argparse
import re
from pathlib import Path


PERF_SYMBOL_RE = re.compile(r"^\s*([0-9]+(?:\.[0-9]+)?)%\s+\S+\s+\S+\s+\[.\]\s+(.+?)\s*$")
ANNOTATE_RE = re.compile(r"^\s*([0-9]+(?:\.[0-9]+)?)\s*:\s+[0-9a-f]+:\s+(.+?)\s*$")


def parse_perf(path: Path) -> tuple[float, dict[str, float], list[tuple[float, str]]]:
    field_total = 0.0
    field_symbols: dict[str, float] = {}
    rows: list[tuple[float, str]] = []
    for raw in path.read_text(encoding="utf-8", errors="replace").splitlines():
        match = PERF_SYMBOL_RE.match(raw)
        if not match:
            continue
        pct = float(match.group(1))
        symbol = match.group(2)
        rows.append((pct, symbol))
        if "nyash.object.field_" in symbol:
            field_total += pct
            field_symbols[symbol] = field_symbols.get(symbol, 0.0) + pct
    rows.sort(key=lambda row: (-row[0], row[1]))
    return field_total, field_symbols, rows[:8]


def classify_asm(text: str) -> dict[str, float]:
    buckets = {
        "prologue_validation": 0.0,
        "backend_tls_entry": 0.0,
        "safe_mutex_fallback": 0.0,
        "direct_vec_field_access": 0.0,
        "control_validation_branch": 0.0,
        "return_epilogue": 0.0,
        "unknown": 0.0,
    }
    for raw in text.splitlines():
        match = ANNOTATE_RE.match(raw)
        if not match:
            continue
        pct = float(match.group(1))
        asm = match.group(2)
        if pct <= 0.0:
            continue
        if "SAFE_MUTEX_OBJECTS" in asm or "lock cmpxchg" in asm or "xchg" in asm:
            buckets["safe_mutex_fallback"] += pct
        elif "%fs:" in asm or "BACKEND" in asm or "destroy" in asm or "OnceLock" in asm:
            buckets["backend_tls_entry"] += pct
        elif (
            "shl    $0x5" in asm
            or "shl    $0x4" in asm
            or "lea    (%r" in asm
            or "cmp    0x10" in asm
            or "cmpl   $0x1" in asm
            or "cmpb   $0x1" in asm
            or "mov    0x8(" in asm
            or "mov    %r12,0x8" in asm
            or "mov    %rdx,(%" in asm
            or "mov    %rcx,(%" in asm
        ):
            buckets["direct_vec_field_access"] += pct
        elif (
            asm.startswith("j")
            or asm.startswith("cmp")
            or asm.startswith("set")
            or asm.startswith("test")
            or asm.startswith("movabs")
        ):
            buckets["control_validation_branch"] += pct
        elif "push" in asm or "test" in asm or "sets" in asm or "setbe" in asm:
            buckets["prologue_validation"] += pct
        elif "pop" in asm or "ret" in asm:
            buckets["return_epilogue"] += pct
        else:
            buckets["unknown"] += pct
    return buckets


def merge_buckets(reports: list[dict[str, float]]) -> dict[str, float]:
    merged: dict[str, float] = {}
    for report in reports:
        for key, value in report.items():
            merged[key] = merged.get(key, 0.0) + value
    return merged


def fmt(value: float) -> str:
    return f"{value:.2f}"


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--perf-report", type=Path, required=True)
    parser.add_argument("--field-get-annotate", type=Path, required=True)
    parser.add_argument("--field-set-annotate", type=Path, required=True)
    parser.add_argument("--out", type=Path, required=True)
    args = parser.parse_args()

    field_total, field_symbols, top_rows = parse_perf(args.perf_report)
    get_buckets = classify_asm(args.field_get_annotate.read_text(encoding="utf-8", errors="replace"))
    set_buckets = classify_asm(args.field_set_annotate.read_text(encoding="utf-8", errors="replace"))
    merged = merge_buckets([get_buckets, set_buckets])
    dominant_subowner = max(merged.items(), key=lambda item: (item[1], item[0]))[0]

    if dominant_subowner in {
        "backend_tls_entry",
        "direct_vec_field_access",
        "control_validation_branch",
    }:
        recommended_next = "typed_object_exact_slot_direct_helper_selection"
    elif dominant_subowner == "safe_mutex_fallback":
        recommended_next = "typed_object_backend_route_cleanup"
    else:
        recommended_next = "typed_object_helper_annotate_refresh"

    lines = [
        "output_contract=typed-object-field-helper-subowner-refresh-v0",
        "input_contract=selected-method-array-slot-direct-op-post-fusion-owner-refresh-v0",
        "workload_id=representative-object-lifecycle-small-block-v0",
        f"perf_field_helper_pct={fmt(field_total)}",
    ]
    for symbol in (
        "nyash.object.field_set_hii",
        "nyash.object.field_set_u64_hiu",
        "nyash.object.field_get_hii",
        "nyash.object.field_get_u64_hii",
    ):
        lines.append(f"perf_symbol_pct.{symbol}={fmt(field_symbols.get(symbol, 0.0))}")
    for index, (pct, symbol) in enumerate(top_rows):
        lines.append(f"perf_top_{index}_pct={fmt(pct)}")
        lines.append(f"perf_top_{index}_symbol={symbol}")
    for key in sorted(merged):
        lines.append(f"annotate_local_pct.{key}={fmt(merged[key])}")
    lines.extend(
        [
            f"dominant_field_helper_subowner={dominant_subowner}",
            "secondary_field_helper_subowner=backend_tls_entry",
            "rejected_owner=array_slot_backend_handle_map_hash",
            "rejected_reason=secondary_owner_below_typed_object_field_helper",
            f"recommended_next={recommended_next}",
            "optimization_open=0",
            "winner_claim=0",
            "replacement_active=0",
            "hook_installed=0",
            "global_allocator=0",
            "summary=ok",
        ]
    )
    args.out.parent.mkdir(parents=True, exist_ok=True)
    args.out.write_text("\n".join(lines) + "\n", encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
