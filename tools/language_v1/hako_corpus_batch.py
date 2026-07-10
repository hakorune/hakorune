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
        all_registry_fixture_ids,
        fixture_ids_for_row,
        hako_transport_fixture_ids,
        registry_rows_by_key,
    )
    from .hako_adapter_health import probe_command, run_adapter_json_process
    from .hako_witness_projection import (
        HakoProjectionError,
        project_hako_normalized_form,
    )
else:
    from grammar_contract_corpus import fixtures_by_id
    from grammar_contract_registry import (
        GrammarRegistryRow,
        HAKO_TRANSPORT_EXCLUSION_TAG,
        RUST_MIGRATION_TRANSPORT_OWNER,
        all_registry_fixture_ids,
        fixture_ids_for_row,
        hako_transport_fixture_ids,
        registry_rows_by_key,
    )
    from hako_adapter_health import probe_command, run_adapter_json_process
    from hako_witness_projection import (
        HakoProjectionError,
        project_hako_normalized_form,
    )


ROOT = pathlib.Path(__file__).resolve().parents[2]
BATCH_SCHEMA = "language-v1-hako-raw-evidence-batch-v0"
REPORT_SCHEMA = "language-v1-hako-corpus-batch-report-v0"


def _match_scrutinee_kind(observation: dict[str, Any]) -> str:
    program = observation.get("program")
    if not isinstance(program, dict):
        return ""
    body = program.get("body")
    if not isinstance(body, list) or not body or not isinstance(body[0], dict):
        return ""
    expression = body[0].get("expr")
    if not isinstance(expression, dict) or expression.get("type") != "EnumMatch":
        return ""
    scrutinee = expression.get("scrutinee")
    if not isinstance(scrutinee, dict):
        return ""
    kind = scrutinee.get("type")
    return kind if isinstance(kind, str) else ""


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
            "parser_inventory_json", "[]"
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
        expected_scrutinee_kind = fixture.get("hako_expected_scrutinee_kind", "")
        actual_scrutinee_kind = (
            _match_scrutinee_kind(observation)
            if isinstance(observation, dict) and expected_scrutinee_kind
            else ""
        )
        actual_normalized_form = None
        projection_error = ""
        if actual_status == "ok" and isinstance(observation, dict):
            try:
                actual_normalized_form = project_hako_normalized_form(
                    fixture["row_id"], observation.get("program")
                )
            except HakoProjectionError as error:
                projection_error = error.stable_reject_tag
                actual_status = "error"
                actual_tag = error.stable_reject_tag
        row_ok = (
            isinstance(observation, dict)
            and observation.get("schema") == "language-v1-hako-raw-evidence-v0"
            and observation.get("deterministic") is True
            and observation.get("raw_program_json_authority") is False
            and actual_status == expected_status
            and actual_tag == expected_tag
        )
        if expected_status == "ok":
            row_ok = (
                row_ok
                and isinstance(observation.get("program"), dict)
                and actual_normalized_form == fixture["normalized_form"]
            )
        if expected_scrutinee_kind:
            row_ok = row_ok and actual_scrutinee_kind == expected_scrutinee_kind
        if not row_ok:
            failures.append(
                {
                    "fixture_id": fixture_id,
                    "reason": "parser/hako_witness_projection_drift",
                }
            )
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
                "expected_scrutinee_kind": expected_scrutinee_kind,
                "actual_scrutinee_kind": actual_scrutinee_kind,
                "expected_normalized_form": fixture["normalized_form"],
                "actual_normalized_form": actual_normalized_form,
                "projection_error": projection_error,
                "hako_adapter_invoked": True,
                "ok": row_ok,
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


def run_hako_fixture_ids(
    binary: pathlib.Path,
    fixture_ids: list[str],
    *,
    timeout_seconds: float,
) -> dict[str, Any]:
    corpus = fixtures_by_id()
    try:
        if not fixture_ids:
            raise KeyError("")
        fixtures = [corpus[fixture_id] for fixture_id in fixture_ids]
    except KeyError as error:
        return {
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

    selection = select_hako_semantic_fixtures(fixture_ids, fixtures)
    included_ids = selection["included_ids"]
    included_fixtures = selection["included_fixtures"]
    excluded_rows = selection["excluded_rows"]
    selection_failures = selection["failures"]
    if selection_failures or not included_fixtures:
        return report_without_adapter(
            excluded_rows,
            selection_failures,
            fixture_count=len(fixtures),
        )

    command = probe_command(binary, "observation", "canonical")
    if command is None:
        raise AssertionError("observation command must exist")
    command.append("--batch")
    result = run_adapter_json_process(
        command,
        timeout_seconds=timeout_seconds,
        environment=batch_environment(included_fixtures),
    )
    if result.payload is None:
        return {
            "schema": REPORT_SCHEMA,
            "status": "error",
            "adapter_process_count": 1,
            "fixture_count": len(fixtures),
            "adapter_fixture_count": len(included_fixtures),
            "excluded_fixture_count": len(excluded_rows),
            "rows": excluded_rows,
            "failures": [{"fixture_id": "", "reason": result.stable_reject_tag}],
        }
    return compare_batch(
        included_ids,
        included_fixtures,
        result.payload,
        excluded_rows=excluded_rows,
        total_fixture_count=len(fixtures),
    )


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--bin", type=pathlib.Path, default=ROOT / "target/debug/hakorune")
    parser.add_argument("--fixture-id", action="append", default=[])
    parser.add_argument(
        "--include-registry-row-fixtures",
        action="append",
        nargs=2,
        metavar=("ROW_ID", "PROFILE"),
        default=[],
    )
    parser.add_argument("--include-registry-transport-exclusions", action="store_true")
    parser.add_argument("--include-all-registry-fixtures", action="store_true")
    parser.add_argument("--timeout-sec", type=float, default=180.0)
    args = parser.parse_args()

    fixture_ids = list(args.fixture_id)
    if args.include_all_registry_fixtures:
        for fixture_id in all_registry_fixture_ids():
            if fixture_id not in fixture_ids:
                fixture_ids.append(fixture_id)
    try:
        for row_id, profile in args.include_registry_row_fixtures:
            for fixture_id in fixture_ids_for_row(row_id, profile):
                if fixture_id not in fixture_ids:
                    fixture_ids.append(fixture_id)
    except KeyError as error:
        parser.error(f"unknown grammar registry row: {error.args[0]}")
    if args.include_registry_transport_exclusions:
        for fixture_id in hako_transport_fixture_ids():
            if fixture_id not in fixture_ids:
                fixture_ids.append(fixture_id)
    report = run_hako_fixture_ids(
        args.bin,
        fixture_ids,
        timeout_seconds=args.timeout_sec,
    )
    print(json.dumps(report, sort_keys=True, separators=(",", ":")))
    return 0 if report["status"] == "ok" else 2


if __name__ == "__main__":
    raise SystemExit(main())
