#!/usr/bin/env python3
"""Report which fast path candidate is reachable for a MIR JSON function.

This is an observation tool. It does not decide legality and does not rewrite
routes. The first v0 surface makes old exact-seed preemption visible so a new
metadata consumer cannot be mistaken for a reachable executable path.
"""

from __future__ import annotations

import argparse
import json
from dataclasses import dataclass, replace
from pathlib import Path
from typing import Any

from fastpath_route_priority import priority_value_for_family


@dataclass(frozen=True)
class Candidate:
    family: str
    producer: str
    backend_consumer: str
    expected_route: str
    priority: int
    selected: bool = False
    reachable: bool = False
    preempted_by: str = "none"
    preempted_reason: str = "none"


def load_json(path: Path) -> dict[str, Any]:
    with path.open("r", encoding="utf-8") as fh:
        data = json.load(fh)
    if not isinstance(data, dict):
        raise SystemExit("MIR JSON root must be an object")
    return data


def functions(data: dict[str, Any]) -> list[dict[str, Any]]:
    rows = data.get("functions")
    if not isinstance(rows, list):
        raise SystemExit("MIR JSON missing functions[]")
    return [row for row in rows if isinstance(row, dict)]


def select_function(data: dict[str, Any], name: str) -> dict[str, Any]:
    for function in functions(data):
        if function.get("name") == name:
            return function
    raise SystemExit(f"function not found: {name}")


def metadata(function: dict[str, Any]) -> dict[str, Any]:
    value = function.get("metadata")
    return value if isinstance(value, dict) else {}


def list_metadata(meta: dict[str, Any], key: str) -> list[dict[str, Any]]:
    value = meta.get(key)
    if not isinstance(value, list):
        return []
    return [row for row in value if isinstance(row, dict)]


def exact_seed_candidate(meta: dict[str, Any]) -> Candidate | None:
    route = meta.get("exact_seed_backend_route")
    if not isinstance(route, dict):
        return None
    tag = str(route.get("tag") or "unknown_exact_seed")
    return Candidate(
        family="exact_seed",
        producer="function_level_exact_seed",
        backend_consumer=tag,
        expected_route=tag,
        priority=priority_value_for_family("exact_seed"),
        selected=True,
        reachable=True,
    )


def string_dead_text_candidates(meta: dict[str, Any]) -> list[Candidate]:
    plans = list_metadata(meta, "string_dead_text_region_plans")
    return [
        Candidate(
            family="string_dead_text_region",
            producer="StringDeadTextRegionPlan",
            backend_consumer="cabi_string_dead_text_region_consumer",
            expected_route="generic_metadata_path",
            priority=priority_value_for_family("string_dead_text_region"),
        )
        for _ in plans
    ]


def collect_candidates(meta: dict[str, Any]) -> list[Candidate]:
    candidates: list[Candidate] = []
    exact = exact_seed_candidate(meta)
    if exact is not None:
        candidates.append(exact)
    candidates.extend(string_dead_text_candidates(meta))
    return candidates


def resolve_reachability(candidates: list[Candidate]) -> list[Candidate]:
    selected = next((candidate for candidate in candidates if candidate.selected), None)
    # v0 only treats an explicitly selected route as reachable. Candidate
    # existence alone must not become a winner claim.
    if selected is None:
        return candidates
    out: list[Candidate] = []
    for candidate in candidates:
        if candidate == selected:
            out.append(
                replace(
                    candidate,
                    selected=True,
                    reachable=True,
                    preempted_by="none",
                    preempted_reason="none",
                )
            )
        else:
            out.append(
                replace(
                    candidate,
                    selected=False,
                    reachable=False,
                    preempted_by=selected.expected_route,
                    preempted_reason="lower_priority_selected_route",
                )
            )
    return out


def candidate_rows(candidates: list[Candidate]) -> list[dict[str, str]]:
    return [
        {
            "family": candidate.family,
            "producer": candidate.producer,
            "backend_consumer": candidate.backend_consumer,
            "expected_route": candidate.expected_route,
            "priority": str(candidate.priority),
            "selected": "1" if candidate.selected else "0",
            "reachable": "1" if candidate.reachable else "0",
            "preempted_by": candidate.preempted_by,
            "preempted_reason": candidate.preempted_reason,
        }
        for candidate in candidates
    ]


def build_report(data: dict[str, Any], function_name: str, front: str) -> dict[str, Any]:
    function = select_function(data, function_name)
    meta = metadata(function)
    candidates = resolve_reachability(collect_candidates(meta))
    selected = next((candidate for candidate in candidates if candidate.selected), None)
    string_dead_text = [
        candidate for candidate in candidates if candidate.family == "string_dead_text_region"
    ]
    old_exact_seed_selected = bool(
        selected and selected.producer == "function_level_exact_seed"
    )
    new_consumer_exists = bool(string_dead_text)
    new_consumer_reachable = any(candidate.reachable for candidate in string_dead_text)
    preemption_detected = any(
        candidate.preempted_by != "none" for candidate in candidates
    )
    winner_claim_allowed = bool(selected and selected.reachable and not preemption_detected)
    return {
        "output_contract": "hako-fastpath-reachability-ledger-v1",
        "route_priority_table_version": "v0",
        "front": front,
        "function": function_name,
        "candidate_count": str(len(candidates)),
        "selected_route": selected.expected_route if selected else "none",
        "selected_route_owner": selected.producer if selected else "none",
        "selected_backend_consumer": selected.backend_consumer if selected else "none",
        "selected_route_priority": str(selected.priority) if selected else "0",
        "selected_route_priority_source": "route_priority_table_v0" if selected else "none",
        "new_consumer_exists": "1" if new_consumer_exists else "0",
        "new_consumer_reachable": "1" if new_consumer_reachable else "0",
        "old_exact_seed_selected": "1" if old_exact_seed_selected else "0",
        "preemption_detected": "1" if preemption_detected else "0",
        "forced_reachability_allowed": "0",
        "winner_claim_allowed": "1" if winner_claim_allowed else "0",
        "summary": "ok",
        "candidates": candidate_rows(candidates),
    }


def emit_kv(report: dict[str, Any]) -> None:
    for key, value in report.items():
        if key == "candidates":
            continue
        print(f"{key}={value}")
    candidates = report.get("candidates")
    if isinstance(candidates, list):
        for idx, candidate in enumerate(candidates):
            if not isinstance(candidate, dict):
                continue
            for key, value in candidate.items():
                print(f"candidate_{idx}_{key}={value}")


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--mir-json", required=True, type=Path)
    parser.add_argument("--function", default="main")
    parser.add_argument("--front", default="unknown")
    parser.add_argument("--format", choices=("kv", "json"), default="kv")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    report = build_report(load_json(args.mir_json), args.function, args.front)
    if args.format == "json":
        print(json.dumps(report, indent=2, sort_keys=True))
    else:
        emit_kv(report)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
