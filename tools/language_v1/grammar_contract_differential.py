#!/usr/bin/env python3
"""Run bounded grammar-aware Rust/Hako recursive witness composition."""

from __future__ import annotations

import argparse
import json
import pathlib
import random
from typing import Any

if __package__:
    from .grammar_contract_corpus import fixtures_by_id
    from .hako_corpus_batch import run_hako_fixtures
    from .rust_parser_adapter import observe_rust_fixture
else:
    from grammar_contract_corpus import fixtures_by_id
    from hako_corpus_batch import run_hako_fixtures
    from rust_parser_adapter import observe_rust_fixture


ROOT = pathlib.Path(__file__).resolve().parents[2]
REPORT_SCHEMA = "language-v1-grammar-differential-composition-v0"
DEFAULT_SEED = 3478
DEFAULT_MAX_DEPTH = 2
DEFAULT_CASE_COUNT = 12
COMPOSABLE_EXPRESSION_KINDS = frozenset(
    {
        "IntegerLiteral",
        "FloatLiteral",
        "StringLiteral",
        "BoolLiteral",
        "NullLiteral",
        "VoidLiteral",
        "RecordLiteral",
        "RecordWithUpdate",
        "ArrayLiteral",
        "NewBoxExpression",
    }
)


def _accepted_expression_seeds(profile: str) -> list[dict[str, Any]]:
    seeds = []
    for fixture_id, fixture in fixtures_by_id().items():
        normalized_form = fixture["normalized_form"]
        if (
            fixture["profile"] == profile
            and fixture["accepted"] is True
            and normalized_form["kind"] in COMPOSABLE_EXPRESSION_KINDS
            and not fixture.get("parser_inventory_id")
        ):
            seeds.append(
                {
                    "fixture_id": fixture_id,
                    "source": fixture["source"],
                    "normalized_form": normalized_form,
                }
            )
    return sorted(seeds, key=lambda row: row["fixture_id"])


def _compose_expression(
    rng: random.Random,
    seeds: list[dict[str, Any]],
    depth: int,
) -> tuple[str, dict[str, Any], list[str]]:
    if depth == 0:
        seed = rng.choice(seeds)
        return seed["source"], seed["normalized_form"], [seed["fixture_id"]]
    left_source, left_form, left_ids = _compose_expression(rng, seeds, depth - 1)
    right_source, right_form, right_ids = _compose_expression(rng, seeds, depth - 1)
    return (
        f"[{left_source}, {right_source}]",
        {"kind": "ArrayLiteral", "children": [left_form, right_form]},
        [*left_ids, *right_ids],
    )


def generate_cases(
    *,
    seed: int,
    max_depth: int,
    case_count: int,
) -> list[dict[str, Any]]:
    if max_depth < 1 or case_count < 1:
        raise ValueError("differential bounds must be positive")
    rng = random.Random(seed)
    pools = {
        profile: _accepted_expression_seeds(profile)
        for profile in ("Canonical", "Compat2025")
    }
    if any(not pool for pool in pools.values()):
        raise ValueError("accepted expression seed pool is empty")
    cases = []
    for index in range(case_count):
        profile = "Canonical" if index % 2 == 0 else "Compat2025"
        depth = 1 + rng.randrange(max_depth)
        source, normalized_form, seed_fixture_ids = _compose_expression(
            rng, pools[profile], depth
        )
        cases.append(
            {
                "fixture_id": f"differential_{index:03d}",
                "row_id": "array_literal",
                "profile": profile,
                "source": source,
                "accepted": True,
                "normalized_form": normalized_form,
                "stable_reject_tag": "",
                "composition_depth": depth,
                "seed_fixture_ids": seed_fixture_ids,
            }
        )
    return cases


def build_report(
    binary: pathlib.Path,
    *,
    seed: int,
    max_depth: int,
    case_count: int,
    hako_timeout_seconds: float,
) -> dict[str, Any]:
    cases = generate_cases(seed=seed, max_depth=max_depth, case_count=case_count)
    fixture_ids = [case["fixture_id"] for case in cases]
    hako = run_hako_fixtures(
        binary,
        fixture_ids,
        cases,
        timeout_seconds=hako_timeout_seconds,
    )
    hako_by_id = {row["fixture_id"]: row for row in hako["rows"]}
    rows = []
    failures = list(hako["failures"])
    for case in cases:
        rust = observe_rust_fixture(binary, case)
        hako_row = hako_by_id.get(case["fixture_id"], {})
        expected = case["normalized_form"]
        rust_ok = rust.get("accepted") is True and rust.get("normalized_form") == expected
        hako_form = hako_row.get("actual_normalized_form")
        hako_ok = hako_row.get("ok") is True and hako_form == expected
        parity_ok = rust.get("normalized_form") == hako_form
        ok = rust_ok and hako_ok and parity_ok
        if not ok:
            failures.append(
                {
                    "fixture_id": case["fixture_id"],
                    "reason": "parser/differential_witness_drift",
                    "source": case["source"],
                    "seed": seed,
                }
            )
        rows.append(
            {
                "fixture_id": case["fixture_id"],
                "profile": case["profile"],
                "source": case["source"],
                "composition_depth": case["composition_depth"],
                "seed_fixture_ids": case["seed_fixture_ids"],
                "expected_normalized_form": expected,
                "rust_normalized_form": rust.get("normalized_form"),
                "hako_normalized_form": hako_form,
                "ok": ok,
            }
        )
    return {
        "schema": REPORT_SCHEMA,
        "status": "ok" if not failures else "error",
        "seed": seed,
        "max_depth": max_depth,
        "case_count": case_count,
        "hako_adapter_process_count": hako["adapter_process_count"],
        "rows": rows,
        "failures": failures,
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--bin", type=pathlib.Path, default=ROOT / "target/debug/hakorune")
    parser.add_argument("--seed", type=int, default=DEFAULT_SEED)
    parser.add_argument("--max-depth", type=int, default=DEFAULT_MAX_DEPTH)
    parser.add_argument("--case-count", type=int, default=DEFAULT_CASE_COUNT)
    parser.add_argument("--hako-timeout-sec", type=float, default=180.0)
    parser.add_argument("--output", type=pathlib.Path)
    args = parser.parse_args()
    if not args.bin.is_file():
        parser.error(f"binary is missing: {args.bin}")
    report = build_report(
        args.bin,
        seed=args.seed,
        max_depth=args.max_depth,
        case_count=args.case_count,
        hako_timeout_seconds=args.hako_timeout_sec,
    )
    payload = json.dumps(report, sort_keys=True, separators=(",", ":")) + "\n"
    if args.output:
        args.output.write_text(payload, encoding="utf-8")
    else:
        print(payload, end="")
    return 0 if report["status"] == "ok" else 2


if __name__ == "__main__":
    raise SystemExit(main())
