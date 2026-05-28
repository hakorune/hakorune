#!/usr/bin/env python3
"""Refresh exact-slot callsite attribution with recent non-keeper weighting."""

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


def parse_family_candidate(raw: str) -> tuple[str, int]:
    if "=" not in raw:
        raise argparse.ArgumentTypeError("family candidate must be FAMILY=COUNT")
    family, count = raw.split("=", 1)
    try:
        parsed_count = int(count)
    except ValueError as exc:
        raise argparse.ArgumentTypeError(f"candidate count must be integer: {raw}") from exc
    if parsed_count < 0:
        raise argparse.ArgumentTypeError(f"candidate count must be non-negative: {raw}")
    return family, parsed_count


def parse_perf(path: Path) -> tuple[float, dict[str, float], dict[str, float], list[tuple[float, str, str]]]:
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
        by_family[family_for(symbol)] += pct
        callsites.append((pct, symbol, current_helper))

    return exact_get_set_pct, dict(by_helper), dict(by_family), callsites


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--perf-report", type=Path, required=True)
    parser.add_argument("--out", type=Path, required=True)
    parser.add_argument("--input-contract", default="post-page-queue-rollback-owner-refresh-v0")
    parser.add_argument("--recent-nonkeeper-family", default="page_queue_helpers")
    parser.add_argument("--recent-nonkeeper-row", default="296x-241")
    parser.add_argument(
        "--family-candidate",
        action="append",
        type=parse_family_candidate,
        default=[],
        help="Known static candidate count in FAMILY=COUNT form.",
    )
    args = parser.parse_args()

    exact_get_set_pct, by_helper, by_family, callsites = parse_perf(args.perf_report)
    if exact_get_set_pct <= 0:
        raise SystemExit("no exact-slot get/set helper samples found")
    if not callsites:
        raise SystemExit("no exact-slot caller rows found; capture perf with call graph data")
    if not by_family:
        raise SystemExit("no exact-slot family attribution found")

    candidate_counts = dict(args.family_candidate)
    ranked_families = sorted(by_family.items(), key=lambda item: item[1], reverse=True)
    dominant_family, dominant_family_pct = ranked_families[0]
    top_callsite_pct, top_callsite, top_callsite_helper = max(callsites, key=lambda item: item[0])

    recent_nonkeeper_pct = by_family.get(args.recent_nonkeeper_family, 0.0)
    recent_nonkeeper_count = candidate_counts.get(args.recent_nonkeeper_family, 0)
    recent_hot_per_candidate = (
        recent_nonkeeper_pct / recent_nonkeeper_count if recent_nonkeeper_count > 0 else 0.0
    )
    top_unblocked_family = "none"
    top_unblocked_family_pct = 0.0
    for family, pct in ranked_families:
        if family != args.recent_nonkeeper_family:
            top_unblocked_family = family
            top_unblocked_family_pct = pct
            break

    lines = [
        "output_contract=weighted-exact-slot-callsite-attribution-refresh-v0",
        f"input_contract={args.input_contract}",
        "workload_id=representative-object-lifecycle-small-block-v0",
        "attribution_source=perf_callgraph",
        "callgraph_attribution_available=1",
        f"exact_slot_get_set_pct={exact_get_set_pct:.2f}",
        f"attributed_callsite_count={len(callsites)}",
        f"top_callsite_pct={top_callsite_pct:.2f}",
        f"top_callsite_symbol={top_callsite}",
        f"top_callsite_helper={top_callsite_helper}",
        f"dominant_family={dominant_family}",
        f"dominant_family_pct={dominant_family_pct:.2f}",
        f"recent_nonkeeper_family={args.recent_nonkeeper_family}",
        f"recent_nonkeeper_row={args.recent_nonkeeper_row}",
        f"recent_nonkeeper_family_pct={recent_nonkeeper_pct:.2f}",
        f"recent_nonkeeper_candidate_count={recent_nonkeeper_count}",
        f"recent_nonkeeper_hot_per_candidate_pct={recent_hot_per_candidate:.2f}",
        f"dominant_family_is_recent_nonkeeper={1 if dominant_family == args.recent_nonkeeper_family else 0}",
        "recent_nonkeeper_family_blocked_for_immediate_keeper=1",
        f"top_unblocked_family={top_unblocked_family}",
        f"top_unblocked_family_pct={top_unblocked_family_pct:.2f}",
        "static_candidate_count_only_rejected=1",
        "weighted_hot_candidate_score_required=1",
        "ir_shape_diff_required_before_next_keeper=1",
        "sample_count_3_required_for_keeper_decision=1",
        "selected_boundary=weighted_exact_slot_owner_selection",
        "next_diagnostic=weighted_exact_slot_owner_selection",
        "optimization_open=0",
        "winner_claim=0",
        "replacement_active=0",
        "hook_installed=0",
        "global_allocator=0",
    ]
    for idx, (family, pct) in enumerate(ranked_families):
        known_count = candidate_counts.get(family, 0)
        hot_per_candidate = pct / known_count if known_count > 0 else 0.0
        lines.append(f"family_{idx}_name={family}")
        lines.append(f"family_{idx}_pct={pct:.2f}")
        lines.append(f"family_{idx}_known_candidate_count={known_count}")
        lines.append(f"family_{idx}_hot_per_candidate_pct={hot_per_candidate:.2f}")
        lines.append(f"family_{idx}_recent_nonkeeper={1 if family == args.recent_nonkeeper_family else 0}")
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
