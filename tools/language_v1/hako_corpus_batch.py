#!/usr/bin/env python3
"""Run selected shared-corpus rows through one compiled Hako adapter process."""

from __future__ import annotations

import argparse
import json
import os
import pathlib
from typing import Any, Iterable

if __package__:
    from .grammar_contract_corpus import fixtures_by_id
    from .grammar_contract_registry import (
        GrammarRegistryRow,
        HAKO_TRANSPORT_EXCLUSION_TAG,
        RUST_MIGRATION_TRANSPORT_OWNER,
        hako_transport_fixture_ids,
        registry_rows_by_key,
    )
    from .hako_adapter_health import probe_command, run_adapter_json_process
else:
    from grammar_contract_corpus import fixtures_by_id
    from grammar_contract_registry import (
        GrammarRegistryRow,
        HAKO_TRANSPORT_EXCLUSION_TAG,
        RUST_MIGRATION_TRANSPORT_OWNER,
        hako_transport_fixture_ids,
        registry_rows_by_key,
    )
    from hako_adapter_health import probe_command, run_adapter_json_process


ROOT = pathlib.Path(__file__).resolve().parents[2]
BATCH_SCHEMA = "language-v1-hako-raw-evidence-batch-v0"
REPORT_SCHEMA = "language-v1-hako-corpus-batch-report-v0"


def select_hako_semantic_fixtures(
    fixture_ids: list[str],
    fixtures: list[dict[str, Any]],
    *,
    registry: dict[tuple[str, str], GrammarRegistryRow] | None = None,
) -> dict[str, Any]:
    registry = registry_rows_by_key() if registry is None else registry
    included_ids: list[str] = []
    included_fixtures: list[dict[str, Any]] = []
    excluded_rows: list[dict[str, Any]] = []
    failures: list[dict[str, str]] = []

    for fixture_id, fixture in zip(fixture_ids, fixtures, strict=True):
        key = (fixture["row_id"], fixture["profile"])
        row = registry.get(key)
        if row is None:
            reason = (
                "parser/profile_mismatch"
                if any(row_id == fixture["row_id"] for row_id, _ in registry)
                else "parser/registry_row_missing"
            )
            failures.append(
                {"fixture_id": fixture_id, "reason": reason}
            )
            continue
        if fixture_id not in row.fixture_ids:
            failures.append(
                {
                    "fixture_id": fixture_id,
                    "reason": "parser/hako_transport_scope_drift",
                }
            )
            continue
        if row.excluded_from_hako_semantic_conformance:
            excluded_rows.append(
                {
                    "fixture_id": fixture_id,
                    "row_id": row.row_id,
                    "profile": row.profile,
                    "row_status": "excluded",
                    "stable_reject_tag": HAKO_TRANSPORT_EXCLUSION_TAG,
                    "transport_owner": RUST_MIGRATION_TRANSPORT_OWNER,
                    "hako_adapter_invoked": False,
                    "ok": True,
                }
            )
            continue
        included_ids.append(fixture_id)
        included_fixtures.append(fixture)

    return {
        "included_ids": included_ids,
        "included_fixtures": included_fixtures,
        "excluded_rows": excluded_rows,
        "failures": failures,
    }


def batch_environment(
    fixtures: Iterable[dict[str, Any]],
    *,
    base: dict[str, str] | None = None,
) -> dict[str, str]:
    rows = list(fixtures)
    environment = dict(os.environ if base is None else base)
    environment["HAKO_GRAMMAR_CONTRACT_BATCH_COUNT"] = str(len(rows))
    for index, fixture in enumerate(rows):
        prefix = f"HAKO_GRAMMAR_CONTRACT_BATCH_"
        environment[f"{prefix}SOURCE_{index}"] = fixture["source"]
        environment[f"{prefix}PROFILE_{index}"] = fixture["profile"].lower()
        environment[f"{prefix}INVENTORY_JSON_{index}"] = fixture.get(
            "hako_inventory_json", "[]"
        )
    return environment


def compare_batch(
    fixture_ids: list[str],
    fixtures: list[dict[str, Any]],
    payload: dict[str, Any],
    *,
    excluded_rows: list[dict[str, Any]] | None = None,
    total_fixture_count: int | None = None,
) -> dict[str, Any]:
    failures: list[dict[str, str]] = []
    observations = payload.get("observations")
    if payload.get("schema") != BATCH_SCHEMA or not isinstance(observations, list):
        failures.append(
            {
                "fixture_id": "",
                "reason": "parser/hako_adapter_malformed_output",
            }
        )
        observations = []
    if payload.get("raw_program_json_authority") is not False:
        failures.append(
            {
                "fixture_id": "",
                "reason": "parser/hako_raw_json_as_authority_forbidden",
            }
        )
    if len(observations) != len(fixtures):
        failures.append(
            {
                "fixture_id": "",
                "reason": "parser/hako_adapter_batch_count_mismatch",
            }
        )

    rows = []
    observation_by_id: dict[str, dict[str, Any]] = {}
    for fixture_id, fixture, observation in zip(
        fixture_ids, fixtures, observations, strict=False
    ):
        actual_status = observation.get("status") if isinstance(observation, dict) else None
        actual_tag = (
            observation.get("stable_reject_tag", "")
            if isinstance(observation, dict)
            else ""
        )
        expected_status = "ok" if fixture["accepted"] else "error"
        expected_tag = fixture["stable_reject_tag"]
        row_ok = (
            isinstance(observation, dict)
            and observation.get("schema") == "language-v1-hako-raw-evidence-v0"
            and observation.get("deterministic") is True
            and observation.get("raw_program_json_authority") is False
            and actual_status == expected_status
            and actual_tag == expected_tag
        )
        if expected_status == "ok":
            row_ok = row_ok and isinstance(observation.get("program"), dict)
        if not row_ok:
            failures.append(
                {
                    "fixture_id": fixture_id,
                    "reason": "parser/hako_witness_projection_drift",
                }
            )
        if isinstance(observation, dict):
            observation_by_id[fixture_id] = observation
        rows.append(
            {
                "fixture_id": fixture_id,
                "row_id": fixture["row_id"],
                "profile": fixture["profile"],
                "row_status": "observed",
                "expected_status": expected_status,
                "actual_status": actual_status,
                "expected_tag": expected_tag,
                "actual_tag": actual_tag,
                "hako_adapter_invoked": True,
                "ok": row_ok,
            }
        )

    for fixture_id, fixture in zip(fixture_ids, fixtures, strict=False):
        equivalent_id = fixture.get("hako_equivalent_fixture_id")
        if not equivalent_id:
            continue
        left = observation_by_id.get(fixture_id, {}).get("program")
        right = observation_by_id.get(equivalent_id, {}).get("program")
        if not isinstance(left, dict) or left != right:
            failures.append(
                {
                    "fixture_id": fixture_id,
                    "reason": "parser/hako_normalized_program_drift",
                }
            )

    excluded_rows = [] if excluded_rows is None else excluded_rows
    rows.extend(excluded_rows)
    return {
        "schema": REPORT_SCHEMA,
        "status": "ok" if not failures else "error",
        "adapter_process_count": 1,
        "fixture_count": (
            len(fixtures) + len(excluded_rows)
            if total_fixture_count is None
            else total_fixture_count
        ),
        "adapter_fixture_count": len(fixtures),
        "excluded_fixture_count": len(excluded_rows),
        "rows": rows,
        "failures": failures,
    }


def report_without_adapter(
    excluded_rows: list[dict[str, Any]],
    failures: list[dict[str, str]],
    *,
    fixture_count: int,
) -> dict[str, Any]:
    return {
        "schema": REPORT_SCHEMA,
        "status": "ok" if not failures else "error",
        "adapter_process_count": 0,
        "fixture_count": fixture_count,
        "adapter_fixture_count": 0,
        "excluded_fixture_count": len(excluded_rows),
        "rows": excluded_rows,
        "failures": failures,
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--bin", type=pathlib.Path, default=ROOT / "target/debug/hakorune")
    parser.add_argument("--fixture-id", action="append", default=[])
    parser.add_argument("--include-registry-transport-exclusions", action="store_true")
    parser.add_argument("--timeout-sec", type=float, default=180.0)
    args = parser.parse_args()

    fixture_ids = list(args.fixture_id)
    if args.include_registry_transport_exclusions:
        for fixture_id in hako_transport_fixture_ids():
            if fixture_id not in fixture_ids:
                fixture_ids.append(fixture_id)
    corpus = fixtures_by_id()
    try:
        if not fixture_ids:
            raise KeyError("")
        fixtures = [corpus[fixture_id] for fixture_id in fixture_ids]
    except KeyError as error:
        report = {
            "schema": REPORT_SCHEMA,
            "status": "error",
            "adapter_process_count": 0,
            "fixture_count": 0,
            "adapter_fixture_count": 0,
            "excluded_fixture_count": 0,
            "rows": [],
            "failures": [
                {
                    "fixture_id": str(error.args[0]),
                    "reason": "parser/hako_adapter_batch_fixture_unknown",
                }
            ],
        }
        print(json.dumps(report, sort_keys=True, separators=(",", ":")))
        return 2

    selection = select_hako_semantic_fixtures(fixture_ids, fixtures)
    included_ids = selection["included_ids"]
    included_fixtures = selection["included_fixtures"]
    excluded_rows = selection["excluded_rows"]
    selection_failures = selection["failures"]
    if selection_failures or not included_fixtures:
        report = report_without_adapter(
            excluded_rows,
            selection_failures,
            fixture_count=len(fixtures),
        )
        print(json.dumps(report, sort_keys=True, separators=(",", ":")))
        return 0 if report["status"] == "ok" else 2

    command = probe_command(args.bin, "observation", "canonical")
    if command is None:
        raise AssertionError("observation command must exist")
    command.append("--batch")
    result = run_adapter_json_process(
        command,
        timeout_seconds=args.timeout_sec,
        environment=batch_environment(included_fixtures),
    )
    if result.payload is None:
        report = {
            "schema": REPORT_SCHEMA,
            "status": "error",
            "adapter_process_count": 1,
            "fixture_count": len(fixtures),
            "adapter_fixture_count": len(included_fixtures),
            "excluded_fixture_count": len(excluded_rows),
            "rows": excluded_rows,
            "failures": [
                {"fixture_id": "", "reason": result.stable_reject_tag}
            ],
        }
    else:
        report = compare_batch(
            included_ids,
            included_fixtures,
            result.payload,
            excluded_rows=excluded_rows,
            total_fixture_count=len(fixtures),
        )
    print(json.dumps(report, sort_keys=True, separators=(",", ":")))
    return 0 if report["status"] == "ok" else 2


if __name__ == "__main__":
    raise SystemExit(main())
