#!/usr/bin/env python3
"""Classify the post-bootstrap DirectSlot exact-EXE perf owner."""

from __future__ import annotations

import argparse
import re
from collections import defaultdict
from pathlib import Path


TOP_RE = re.compile(r"^\s*([0-9]+(?:\.[0-9]+)?)%\s+\S+\s+\S+\s+\[\.\]\s+(.+?)\s*$")
CALLER_RE = re.compile(r"^\s+(?:\|--|--)([0-9]+(?:\.[0-9]+)?)%--(.+?)\s*$")


def family_for(symbol: str) -> str:
    if symbol.startswith("HakoAllocObjectLifecycleAllocResult."):
        return "alloc_result_capsule"
    if symbol.startswith("HakoAllocObjectLifecycleReleaseResult."):
        return "release_result_capsule"
    if symbol.startswith("HakoAllocObjectLifecyclePageQueue."):
        return "page_queue_helpers"
    if symbol.startswith("HakoAllocPageModel."):
        return "page_model_hotpath"
    if symbol.startswith("HakoAllocObjectLifecycleFacade."):
        return "object_lifecycle_facade"
    if symbol.startswith("HakoAlloc"):
        return "other_hako_method"
    return "other"


def parse_perf(path: Path):
    field_helper_pct = 0.0
    exact_slot_pct = 0.0
    array_backend_pct = 0.0
    array_hash_pct = 0.0
    hako_method_pct = 0.0
    helper_rows: dict[str, float] = defaultdict(float)
    family_rows: dict[str, float] = defaultdict(float)
    callsites: list[tuple[float, str, str]] = []
    top_rows: list[tuple[float, str]] = []
    current_helper: str | None = None

    for line in path.read_text(encoding="utf-8", errors="replace").splitlines():
        top = TOP_RE.match(line)
        if top:
            pct = float(top.group(1))
            symbol = top.group(2).strip()
            top_rows.append((pct, symbol))
            if "nyash.object.field_" in symbol:
                field_helper_pct += pct
                helper_rows[symbol] += pct
                current_helper = symbol
            else:
                current_helper = None
            if "nyash.object.exact_slot_" in symbol:
                exact_slot_pct += pct
            if "nyash_kernel::plugin::array_slot_backend::" in symbol:
                array_backend_pct += pct
            if (
                "core::hash::BuildHasher::hash_one" in symbol
                or "<core::hash::sip::Hasher" in symbol
            ):
                array_hash_pct += pct
            if symbol.startswith("HakoAlloc"):
                hako_method_pct += pct
            continue

        if current_helper is None:
            continue
        caller = CALLER_RE.match(line)
        if not caller:
            continue
        pct = float(caller.group(1))
        symbol = caller.group(2).strip()
        family = family_for(symbol)
        family_rows[family] += pct
        callsites.append((pct, symbol, current_helper))

    if not top_rows:
        raise SystemExit(f"{path}: no perf top rows found")

    return {
        "field_helper_pct": field_helper_pct,
        "exact_slot_pct": exact_slot_pct,
        "array_backend_pct": array_backend_pct,
        "array_hash_pct": array_hash_pct,
        "hako_method_pct": hako_method_pct,
        "helper_rows": dict(helper_rows),
        "family_rows": dict(family_rows),
        "callsites": callsites,
        "top_rows": top_rows,
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--perf-report", type=Path, required=True)
    parser.add_argument("--out", type=Path, required=True)
    args = parser.parse_args()

    parsed = parse_perf(args.perf_report)
    field_helper_pct = parsed["field_helper_pct"]
    array_total_pct = parsed["array_backend_pct"] + parsed["array_hash_pct"]
    family_rows = parsed["family_rows"]
    helper_rows = parsed["helper_rows"]
    callsites = parsed["callsites"]

    if field_helper_pct > array_total_pct and field_helper_pct > parsed["exact_slot_pct"]:
        selected_boundary = "direct_slot_supported_storage_nativedirect_guard_surface"
        next_diagnostic = "direct_slot_supported_storage_nativedirect_guard_surface"
        selected_reason = "legacy_field_helpers_dominate_after_direct_slot_bootstrap_compatibility"
    else:
        selected_boundary = "owner_refresh_after_direct_slot_bootstrap"
        next_diagnostic = "rerun_perf_owner_refresh"
        selected_reason = "field_helpers_not_dominant"

    ranked_families = sorted(family_rows.items(), key=lambda item: item[1], reverse=True)
    ranked_helpers = sorted(helper_rows.items(), key=lambda item: item[1], reverse=True)

    lines = [
        "output_contract=direct-slot-post-bootstrap-owner-refresh-v0",
        "input_contract=direct-slot-bootstrap-materialization-compatibility-v0",
        "workload_id=representative-object-lifecycle-small-block-v0",
        "attribution_source=perf_callgraph",
        "callgraph_attribution_available=1",
        f"field_helper_pct={field_helper_pct:.2f}",
        f"exact_slot_helper_pct={parsed['exact_slot_pct']:.2f}",
        f"array_slot_backend_pct={parsed['array_backend_pct']:.2f}",
        f"array_hash_pct={parsed['array_hash_pct']:.2f}",
        f"array_total_pct={array_total_pct:.2f}",
        f"hako_method_pct={parsed['hako_method_pct']:.2f}",
        f"selected_boundary={selected_boundary}",
        f"next_diagnostic={next_diagnostic}",
        f"selected_reason={selected_reason}",
        "optimization_open=0",
        "winner_claim=0",
        "replacement_active=0",
        "hook_installed=0",
        "global_allocator=0",
    ]
    for idx, (family, pct) in enumerate(ranked_families):
        lines.append(f"family_{idx}_name={family}")
        lines.append(f"family_{idx}_pct={pct:.2f}")
    for idx, (helper, pct) in enumerate(ranked_helpers):
        lines.append(f"helper_{idx}_symbol={helper}")
        lines.append(f"helper_{idx}_pct={pct:.2f}")
    for idx, (pct, symbol, helper) in enumerate(sorted(callsites, reverse=True)[:10]):
        lines.append(f"callsite_{idx}_pct={pct:.2f}")
        lines.append(f"callsite_{idx}_symbol={symbol}")
        lines.append(f"callsite_{idx}_helper={helper}")
    lines.append("summary=ok")

    args.out.parent.mkdir(parents=True, exist_ok=True)
    args.out.write_text("\n".join(lines) + "\n", encoding="utf-8")
    print("\n".join(lines))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
