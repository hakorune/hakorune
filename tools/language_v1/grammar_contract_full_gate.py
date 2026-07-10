#!/usr/bin/env python3
"""Run exhaustive Language v1 parser conformance and emit support evidence."""

from __future__ import annotations

import argparse
from collections import defaultdict
import json
import pathlib
from typing import Any

if __package__:
    from .grammar_contract_differential import (
        DEFAULT_CASE_COUNT,
        DEFAULT_MAX_DEPTH,
        DEFAULT_SEED,
        build_report as differential_report,
    )
    from .grammar_contract_corpus import fixtures_by_id
    from .grammar_contract_registry import (
        NormalizationMode,
        all_registry_fixture_ids,
        registry_rows_by_key,
    )
    from .hako_corpus_batch import run_hako_fixture_ids
    from .rust_parser_adapter import observe_rust_fixture
else:
    from grammar_contract_differential import (
        DEFAULT_CASE_COUNT,
        DEFAULT_MAX_DEPTH,
        DEFAULT_SEED,
        build_report as differential_report,
    )
    from grammar_contract_corpus import fixtures_by_id
    from grammar_contract_registry import (
        NormalizationMode,
        all_registry_fixture_ids,
        registry_rows_by_key,
    )
    from hako_corpus_batch import run_hako_fixture_ids
    from rust_parser_adapter import observe_rust_fixture


ROOT = pathlib.Path(__file__).resolve().parents[2]
REPORT_SCHEMA = "language-v1-grammar-full-conformance-v0"


def _matches_expected(fixture: dict[str, Any], observed: dict[str, Any]) -> bool:
    return (
        observed.get("accepted") is fixture["accepted"]
        and observed.get("stable_reject_tag", "") == fixture["stable_reject_tag"]
        and (
            not fixture["accepted"]
            or observed.get("normalized_form") == fixture["normalized_form"]
        )
    )


def rust_report(
    binary: pathlib.Path,
    fixture_ids: list[str],
    fixtures: dict[str, dict[str, Any]],
) -> dict[str, Any]:
    registry = registry_rows_by_key()
    rows = []
    failures = []
    for fixture_id in fixture_ids:
        fixture = fixtures[fixture_id]
        contract = registry[(fixture["row_id"], fixture["profile"])]
        if (
            contract.normalization_mode is NormalizationMode.COMPATIBILITY_TRANSPORT
            and fixture["accepted"]
        ):
            rows.append(
                {
                    "fixture_id": fixture_id,
                    "row_id": fixture["row_id"],
                    "profile": fixture["profile"],
                    "row_status": "migration_transport_owned",
                    "transport_owner": "RustMigrationToolingOnly",
                    "ok": True,
                }
            )
            continue
        observed = observe_rust_fixture(binary, fixture)
        ok = _matches_expected(fixture, observed)
        if not ok:
            failures.append({"fixture_id": fixture_id, "reason": "parser/witness_drift"})
        rows.append(
            {
                "fixture_id": fixture_id,
                "row_id": fixture["row_id"],
                "profile": fixture["profile"],
                "row_status": "observed",
                "expected": {
                    "accepted": fixture["accepted"],
                    "normalized_form": fixture["normalized_form"],
                    "stable_reject_tag": fixture["stable_reject_tag"],
                },
                "observed": observed,
                "ok": ok,
            }
        )
    return {
        "status": "ok" if not failures else "error",
        "fixture_count": len(fixture_ids),
        "rows": rows,
        "failures": failures,
    }


def support_matrix(
    rust_rows: list[dict[str, Any]],
    hako_rows: list[dict[str, Any]],
) -> list[dict[str, Any]]:
    grouped: dict[tuple[str, str, str], list[dict[str, Any]]] = defaultdict(list)
    for parser_name, rows in (
        ("Rust", rust_rows),
        ("Hako", hako_rows),
    ):
        for row in rows:
            grouped[(parser_name, row["row_id"], row["profile"])].append(row)

    matrix = []
    for (parser_name, row_id, profile), rows in sorted(grouped.items()):
        statuses = {row["row_status"] for row in rows}
        if statuses <= {"excluded"}:
            status = "explicitly_excluded"
        elif "migration_transport_owned" in statuses:
            status = "migration_transport_owned"
        elif all(row["ok"] for row in rows):
            status = "supported"
        else:
            status = "drift"
        matrix.append(
            {
                "parser": parser_name,
                "row_id": row_id,
                "profile": profile,
                "fixture_count": len(rows),
                "status": status,
            }
        )
    return matrix


def build_report(
    binary: pathlib.Path,
    *,
    hako_timeout_seconds: float,
    differential_seed: int = DEFAULT_SEED,
    differential_max_depth: int = DEFAULT_MAX_DEPTH,
    differential_case_count: int = DEFAULT_CASE_COUNT,
) -> dict[str, Any]:
    fixture_ids = list(all_registry_fixture_ids())
    fixtures = fixtures_by_id()
    if set(fixture_ids) != set(fixtures):
        return {
            "schema": REPORT_SCHEMA,
            "status": "error",
            "failures": [{"reason": "parser/fixture_index_drift"}],
        }
    rust = rust_report(binary, fixture_ids, fixtures)
    hako = run_hako_fixture_ids(
        binary,
        fixture_ids,
        timeout_seconds=hako_timeout_seconds,
    )
    differential = differential_report(
        binary,
        seed=differential_seed,
        max_depth=differential_max_depth,
        case_count=differential_case_count,
        hako_timeout_seconds=hako_timeout_seconds,
    )
    failures = [
        *({"parser": "Rust", **failure} for failure in rust["failures"]),
        *({"parser": "Hako", **failure} for failure in hako["failures"]),
        *(
            {"parser": "Differential", **failure}
            for failure in differential["failures"]
        ),
    ]
    return {
        "schema": REPORT_SCHEMA,
        "status": "ok" if not failures else "error",
        "fixture_count": len(fixture_ids),
        "rust": rust,
        "hako": hako,
        "differential": differential,
        "support_matrix": support_matrix(rust["rows"], hako["rows"]),
        "failures": failures,
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--bin", type=pathlib.Path, default=ROOT / "target/debug/hakorune")
    parser.add_argument("--hako-timeout-sec", type=float, default=180.0)
    parser.add_argument("--differential-seed", type=int, default=DEFAULT_SEED)
    parser.add_argument("--differential-max-depth", type=int, default=DEFAULT_MAX_DEPTH)
    parser.add_argument("--differential-case-count", type=int, default=DEFAULT_CASE_COUNT)
    parser.add_argument("--output", type=pathlib.Path)
    args = parser.parse_args()
    if not args.bin.is_file():
        parser.error(f"binary is missing: {args.bin}")
    report = build_report(
        args.bin,
        hako_timeout_seconds=args.hako_timeout_sec,
        differential_seed=args.differential_seed,
        differential_max_depth=args.differential_max_depth,
        differential_case_count=args.differential_case_count,
    )
    payload = json.dumps(report, sort_keys=True, separators=(",", ":")) + "\n"
    if args.output:
        args.output.write_text(payload, encoding="utf-8")
    else:
        print(payload, end="")
    return 0 if report["status"] == "ok" else 2


if __name__ == "__main__":
    raise SystemExit(main())
