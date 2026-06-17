#!/usr/bin/env python3
"""Inventory known direct routes that have not become LocalFastPathFact.

This tool is read-only. It does not infer legality, change route priority, or
emit backend code. Its job is to make the next fastpath owner visible: direct
known-receiver method routes can already exist while no positive
LocalFastPathFact is exported for the same callsite.
"""

from __future__ import annotations

import argparse
import json
from collections import Counter
from pathlib import Path
from typing import Any


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


def function_name(function: dict[str, Any]) -> str:
    return str(function.get("name") or "unknown")


def select_functions(
    data: dict[str, Any],
    method_filter: str | None,
) -> list[dict[str, Any]]:
    rows = functions(data)
    if method_filter is None:
        return rows
    selected = [row for row in rows if function_name(row) == method_filter]
    if not selected:
        raise SystemExit(f"selected method not found: {method_filter}")
    return selected


def metadata(function: dict[str, Any]) -> dict[str, Any]:
    value = function.get("metadata")
    return value if isinstance(value, dict) else {}


def list_meta(function: dict[str, Any], key: str) -> list[dict[str, Any]]:
    value = metadata(function).get(key)
    if not isinstance(value, list):
        return []
    return [row for row in value if isinstance(row, dict)]


def site_key(row: dict[str, Any]) -> tuple[str, str]:
    return (str(row.get("block", "unknown")), str(row.get("instruction_index", "unknown")))


def is_known_receiver_direct_method(row: dict[str, Any]) -> bool:
    return (
        str(row.get("route_kind")) == "user_box.method"
        and str(row.get("emit_kind")) == "direct_function_call"
        and str(row.get("proof")) == "typed_user_box_method_same_module"
        and str(row.get("target_exists")).lower() == "true"
        and str(row.get("target_body_supported")).lower() == "true"
    )


def local_fastpath_fact_sites(function: dict[str, Any]) -> set[tuple[str, str]]:
    sites: set[tuple[str, str]] = set()
    for row in list_meta(function, "local_fastpath_facts"):
        sites.add(site_key(row))
    return sites


def is_user_box_method_thin_entry(row: dict[str, Any]) -> bool:
    return (
        str(row.get("surface")) == "user_box_method"
        and str(row.get("state")) == "candidate"
        and str(row.get("selected_entry")) == "thin_internal_entry"
        and str(row.get("manifest_row")) == "user_box_method.known_receiver"
    )


def thin_entry_method_candidate_sites(function: dict[str, Any]) -> set[tuple[str, str]]:
    return {
        site_key(row)
        for row in list_meta(function, "thin_entry_selections")
        if is_user_box_method_thin_entry(row)
    }


def thin_entry_method_candidate_count(function: dict[str, Any]) -> int:
    return sum(
        1
        for row in list_meta(function, "thin_entry_selections")
        if is_user_box_method_thin_entry(row)
    )


def publication_classifications(function: dict[str, Any]) -> list[dict[str, Any]]:
    return list_meta(function, "user_box_method_publication_classifications")


def summarize_function(function: dict[str, Any]) -> dict[str, Any]:
    routes = [
        row for row in list_meta(function, "user_box_method_routes")
        if is_known_receiver_direct_method(row)
    ]
    fact_sites = local_fastpath_fact_sites(function)
    thin_entry_sites = thin_entry_method_candidate_sites(function)
    missing = [row for row in routes if site_key(row) not in fact_sites]
    thin_entry_covered = [row for row in missing if site_key(row) in thin_entry_sites]
    uncovered = [row for row in missing if site_key(row) not in thin_entry_sites]
    subjects = Counter(
        str(row.get("symbol") or row.get("target_symbol") or "unknown")
        for row in uncovered
    )
    publication_rows = publication_classifications(function)
    publication_states = Counter(str(row.get("publication_state") or "unknown") for row in publication_rows)
    blocker_proofs = Counter(
        str(row.get("proof") or "unknown")
        for row in publication_rows
        if not bool(row.get("fact_allowed"))
    )
    return {
        "function": function_name(function),
        "known_receiver_direct_method_route_count": len(routes),
        "local_fastpath_fact_count": len(fact_sites),
        "known_receiver_direct_method_without_fact_count": len(missing),
        "known_receiver_direct_method_thin_entry_covered_count": len(thin_entry_covered),
        "known_receiver_direct_method_uncovered_count": len(uncovered),
        "thin_entry_method_candidate_count": thin_entry_method_candidate_count(function),
        "user_box_method_publication_classification_count": len(publication_rows),
        "publication_fact_allowed_count": sum(1 for row in publication_rows if bool(row.get("fact_allowed"))),
        "publication_unpublished_count": publication_states.get("unpublished", 0),
        "publication_maybe_published_count": publication_states.get("maybe_published", 0),
        "publication_published_count": publication_states.get("published", 0),
        "top_publication_blocker_proof": blocker_proofs.most_common(1)[0][0] if blocker_proofs else "none",
        "top_publication_blocker_count": blocker_proofs.most_common(1)[0][1] if blocker_proofs else 0,
        "top_missing_subject": subjects.most_common(1)[0][0] if subjects else "none",
        "top_missing_subject_count": subjects.most_common(1)[0][1] if subjects else 0,
    }


def build_report(data: dict[str, Any], method_filter: str | None, front: str) -> dict[str, Any]:
    rows = [summarize_function(function) for function in select_functions(data, method_filter)]
    direct_count = sum(int(row["known_receiver_direct_method_route_count"]) for row in rows)
    fact_count = sum(int(row["local_fastpath_fact_count"]) for row in rows)
    missing_count = sum(int(row["known_receiver_direct_method_without_fact_count"]) for row in rows)
    thin_entry_covered_count = sum(
        int(row["known_receiver_direct_method_thin_entry_covered_count"]) for row in rows
    )
    uncovered_count = sum(int(row["known_receiver_direct_method_uncovered_count"]) for row in rows)
    thin_count = sum(int(row["thin_entry_method_candidate_count"]) for row in rows)
    publication_count = sum(int(row["user_box_method_publication_classification_count"]) for row in rows)
    publication_allowed_count = sum(int(row["publication_fact_allowed_count"]) for row in rows)
    publication_maybe_count = sum(int(row["publication_maybe_published_count"]) for row in rows)
    publication_published_count = sum(int(row["publication_published_count"]) for row in rows)
    top = max(
        rows,
        key=lambda row: int(row["known_receiver_direct_method_uncovered_count"]),
        default=None,
    )
    blocker_proofs = Counter()
    for row in rows:
        proof = str(row["top_publication_blocker_proof"])
        if proof != "none":
            blocker_proofs[proof] += int(row["top_publication_blocker_count"])
    return {
        "output_contract": "hako-fastpath-gap-inventory-v0",
        "front": front,
        "function_filter": method_filter or "@all",
        "function_count": str(len(rows)),
        "known_receiver_direct_method_route_count": str(direct_count),
        "local_fastpath_fact_count": str(fact_count),
        "known_receiver_direct_method_without_fact_count": str(missing_count),
        "known_receiver_direct_method_thin_entry_covered_count": str(thin_entry_covered_count),
        "known_receiver_direct_method_uncovered_count": str(uncovered_count),
        "thin_entry_method_candidate_count": str(thin_count),
        "user_box_method_publication_classification_count": str(publication_count),
        "publication_fact_allowed_count": str(publication_allowed_count),
        "publication_maybe_published_count": str(publication_maybe_count),
        "publication_published_count": str(publication_published_count),
        "top_publication_blocker_proof": blocker_proofs.most_common(1)[0][0] if blocker_proofs else "none",
        "top_publication_blocker_count": str(blocker_proofs.most_common(1)[0][1]) if blocker_proofs else "0",
        "fallback_evidence_fact_enabled": "0",
        "backend_lowering_changed": "0",
        "winner_claim_allowed": "0",
        "top_gap_function": str(top["function"]) if top else "none",
        "top_gap_count": str(top["known_receiver_direct_method_uncovered_count"]) if top else "0",
        "summary": "ok",
        "functions": rows,
    }


def emit_kv(report: dict[str, Any]) -> None:
    for key, value in report.items():
        if key == "functions":
            continue
        print(f"{key}={value}")
    rows = report.get("functions")
    if isinstance(rows, list):
        for idx, row in enumerate(rows):
            if not isinstance(row, dict):
                continue
            for key, value in row.items():
                print(f"function_{idx}_{key}={value}")


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--mir-json", required=True, type=Path)
    parser.add_argument("--method")
    parser.add_argument("--front", default="unknown")
    parser.add_argument("--format", choices=("kv", "json"), default="kv")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    report = build_report(load_json(args.mir_json), args.method, args.front)
    if args.format == "json":
        print(json.dumps(report, indent=2, sort_keys=True))
    else:
        emit_kv(report)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
