#!/usr/bin/env python3
"""Inventory exact-seed routes before any retirement decision.

This is an observation tool. It does not delete, demote, or reprioritize exact
seed routes. It answers whether an exact seed has a replacement candidate that
is already reachable in the active MIR front.
"""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from fastpath_reachability_ledger import build_report, load_json, metadata, select_function


def _str(value: Any, default: str = "none") -> str:
    if value is None:
        return default
    return str(value)


def _retire_blocker(
    *,
    exact_seed_present: bool,
    replacement_candidate_exists: bool,
    replacement_reachable: bool,
) -> str:
    if not exact_seed_present:
        return "no_exact_seed"
    if not replacement_candidate_exists:
        return "no_replacement_candidate"
    if not replacement_reachable:
        return "replacement_not_reachable"
    return "deliberate_retire_row_required"


def build_inventory(data: dict[str, Any], function_name: str, front: str) -> dict[str, str]:
    function = select_function(data, function_name)
    meta = metadata(function)
    exact_seed = meta.get("exact_seed_backend_route")
    exact_seed_route = exact_seed if isinstance(exact_seed, dict) else {}
    exact_seed_present = bool(exact_seed_route)

    ledger = build_report(data, function_name, front)
    replacement_candidate_exists = ledger.get("new_consumer_exists") == "1"
    replacement_reachable = ledger.get("new_consumer_reachable") == "1"
    preemption_detected = ledger.get("preemption_detected") == "1"

    return {
        "output_contract": "hako-exact-seed-retire-inventory-v0",
        "route_priority_table_version": _str(ledger.get("route_priority_table_version"), "v0"),
        "front": front,
        "function": function_name,
        "exact_seed_present": "1" if exact_seed_present else "0",
        "exact_seed_tag": _str(exact_seed_route.get("tag")),
        "exact_seed_source_route": _str(exact_seed_route.get("source_route")),
        "exact_seed_proof": _str(exact_seed_route.get("proof")),
        "exact_seed_selected_value": _str(exact_seed_route.get("selected_value")),
        "replacement_family": "string_dead_text_region" if replacement_candidate_exists else "none",
        "replacement_candidate_exists": "1" if replacement_candidate_exists else "0",
        "replacement_reachable": "1" if replacement_reachable else "0",
        "preemption_detected": "1" if preemption_detected else "0",
        "retire_allowed": "0",
        "retire_blocker": _retire_blocker(
            exact_seed_present=exact_seed_present,
            replacement_candidate_exists=replacement_candidate_exists,
            replacement_reachable=replacement_reachable,
        ),
        "drive_by_retire_allowed": "0",
        "backend_lowering_changed": "0",
        "exact_seed_retired": "0",
        "winner_claim_allowed": "0",
        "summary": "ok",
    }


def emit_kv(report: dict[str, str]) -> None:
    for key, value in report.items():
        print(f"{key}={value}")


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--mir-json", required=True, type=Path)
    parser.add_argument("--function", default="main")
    parser.add_argument("--front", default="unknown")
    parser.add_argument("--format", choices=("kv", "json"), default="kv")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    report = build_inventory(load_json(args.mir_json), args.function, args.front)
    if args.format == "json":
        print(json.dumps(report, indent=2, sort_keys=True))
    else:
        emit_kv(report)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
