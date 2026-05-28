#!/usr/bin/env python3
"""Attribute exact-slot typed-object helper cost to perf callsites."""

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


def is_exact_get_set(symbol: str) -> bool:
    return "nyash.object.exact_slot_" in symbol and "nyash.object.exact_slot_rmw_" not in symbol


def parse(path: Path) -> tuple[float, dict[str, float], dict[str, float], list[tuple[float, str, str]]]:
    exact_get_set_pct = 0.0
    by_helper: dict[str, float] = defaultdict(float)
    by_family: dict[str, float] = defaultdict(float)
    callsites: list[tuple[float, str, str]] = []
    current_helper: str | None = None

    for line in path.read_text(encoding="utf-8", errors="replace").splitlines():
        top = TOP_RE.match(line)
        if top:
            pct = float(top.group(1))
            symbol = top.group(2).strip()
            if is_exact_get_set(symbol):
                current_helper = symbol
                exact_get_set_pct += pct
                by_helper[symbol] += pct
            else:
                current_helper = None
            continue

        if current_helper is None:
            continue
        caller = CALLER_RE.match(line)
        if not caller:
            continue
        pct = float(caller.group(1))
        symbol = caller.group(2).strip()
        family = family_for(symbol)
        by_family[family] += pct
        callsites.append((pct, symbol, current_helper))

    return exact_get_set_pct, dict(by_helper), dict(by_family), callsites


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--perf-report", type=Path, required=True)
    parser.add_argument("--out", type=Path, required=True)
    args = parser.parse_args()

    exact_get_set_pct, by_helper, by_family, callsites = parse(args.perf_report)
    if exact_get_set_pct <= 0:
        raise SystemExit("no exact-slot get/set helper samples found")

    top_family, top_family_pct = max(by_family.items(), key=lambda item: item[1])
    top_callsite_pct, top_callsite, top_callsite_helper = max(callsites, key=lambda item: item[0])

    if top_family in {"alloc_result_capsule", "release_result_capsule"}:
        selected_boundary = "result_capsule_field_batching_or_shape_refresh"
    elif top_family == "page_model_hotpath":
        selected_boundary = "page_model_exact_slot_helper_fusion"
    elif top_family == "page_queue_helpers":
        selected_boundary = "page_queue_exact_slot_helper_fusion"
    else:
        selected_boundary = "exact_slot_callsite_owner_selection"

    lines = [
        "output_contract=typed-object-exact-slot-callsite-attribution-v0",
        "input_contract=typed-object-post-rmw-fusion-owner-refresh-v0",
        "workload_id=representative-object-lifecycle-small-block-v0",
        f"exact_slot_get_set_pct={exact_get_set_pct:.2f}",
        f"attributed_callsite_count={len(callsites)}",
        f"top_callsite_pct={top_callsite_pct:.2f}",
        f"top_callsite_symbol={top_callsite}",
        f"top_callsite_helper={top_callsite_helper}",
        f"dominant_family={top_family}",
        f"dominant_family_pct={top_family_pct:.2f}",
        f"selected_boundary={selected_boundary}",
        "optimization_open=0",
        "winner_claim=0",
        "replacement_active=0",
        "hook_installed=0",
        "global_allocator=0",
    ]
    for idx, (family, pct) in enumerate(sorted(by_family.items(), key=lambda item: item[1], reverse=True)):
        lines.append(f"family_{idx}_name={family}")
        lines.append(f"family_{idx}_pct={pct:.2f}")
    for idx, (helper, pct) in enumerate(sorted(by_helper.items(), key=lambda item: item[1], reverse=True)):
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
