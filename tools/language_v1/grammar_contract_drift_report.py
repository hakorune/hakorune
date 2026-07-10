#!/usr/bin/env python3
"""Emit deterministic current parser evidence for the Language v1 contract.

This observes two independent implementations. It does not activate either
grammar profile and never treats observed acceptance as language authority.
"""

from __future__ import annotations

import argparse
import json
import os
import pathlib
import subprocess
import tempfile
from typing import Any

from hako_adapter_health import (
    DEFAULT_HAKO_ADAPTER_TIMEOUT_SECONDS,
    run_adapter_json_process,
    run_health_probe,
)
from grammar_contract_corpus import fixtures_by_id
from hako_witness_projection import HakoProjectionError, project_hako_normalized_form
from rust_witness_projection import RustProjectionError, project_rust_normalized_form


ROOT = pathlib.Path(__file__).resolve().parents[2]
HAKO_ADAPTER = ROOT / "tools/language_v1/grammar_contract_hako_adapter.hako"
PROBE_FIXTURE_IDS = (
    "guard_expr_else_canonical",
    "try_statement_canonical_reject",
    "match_canonical",
    "from_super_call_canonical_reject",
)


def reject_tag(stderr: str) -> str:
    for token in stderr.replace("[", " ").replace("]", " ").split():
        if token.startswith("parser/"):
            return token.rstrip(".,:;")
    return "parser/implementation_rejected"


def rust_observation(binary: pathlib.Path, fixture: dict[str, Any]) -> dict[str, Any]:
    with tempfile.TemporaryDirectory(prefix="grammar-contract-rust-") as temp_dir:
        root = pathlib.Path(temp_dir)
        source = root / "fixture.hako"
        ast = root / "ast.json"
        source.write_text(
            fixture.get("parser_inventory_source", "") + fixture["source"],
            encoding="utf-8",
        )
        completed = subprocess.run(
            [
                str(binary),
                "--emit-ast-json",
                str(ast),
                *(
                    ["--grammar-profile", "compat2025"]
                    if fixture["profile"] == "Compat2025"
                    else []
                ),
                str(source),
            ],
            cwd=ROOT,
            text=True,
            capture_output=True,
            check=False,
        )
        ast_payload = json.loads(ast.read_text(encoding="utf-8")) if completed.returncode == 0 else None
    if completed.returncode == 0:
        try:
            normalized_form = project_rust_normalized_form(
                fixture["row_id"], ast_payload
            )
            projection_error = ""
        except RustProjectionError as error:
            return {
                "accepted": False,
                "normalized_form": None,
                "stable_reject_tag": error.stable_reject_tag,
            }
        return {
            "accepted": True,
            "normalized_form": normalized_form,
            "stable_reject_tag": "",
            "projection_error": projection_error,
        }
    return {
        "accepted": False,
        "normalized_form": None,
        "stable_reject_tag": reject_tag(completed.stderr),
    }


def hako_observation(
    binary: pathlib.Path, fixture: dict[str, Any], timeout_seconds: float
) -> dict[str, Any]:
    environment = os.environ | {"HAKO_GRAMMAR_CONTRACT_SOURCE": fixture["source"]}
    environment["HAKO_GRAMMAR_CONTRACT_INVENTORY_JSON"] = fixture.get(
        "parser_inventory_json", "[]"
    )
    profile = {
        "Canonical": "canonical",
        "Compat2025": "compat2025",
    }[fixture["profile"]]
    result = run_adapter_json_process(
        [
            str(binary),
            "--backend",
            "vm",
            str(HAKO_ADAPTER),
            "--",
            "--grammar-profile",
            profile,
        ],
        timeout_seconds=timeout_seconds,
        environment=environment,
    )
    if result.payload is None:
        return {
            "accepted": False,
            "normalized_form": None,
            "stable_reject_tag": result.stable_reject_tag,
        }
    payload = result.payload
    if payload.get("status") == "ok":
        try:
            normalized_form = project_hako_normalized_form(
                fixture["row_id"], payload.get("program")
            )
            projection_error = ""
        except HakoProjectionError as error:
            return {
                "accepted": False,
                "normalized_form": None,
                "stable_reject_tag": error.stable_reject_tag,
            }
        return {
            "accepted": True,
            "normalized_form": normalized_form,
            "stable_reject_tag": "",
            "projection_error": projection_error,
        }
    return {
        "accepted": False,
        "normalized_form": None,
        "stable_reject_tag": payload.get(
            "stable_reject_tag", "parser/hako_adapter_malformed_output"
        ),
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--bin", type=pathlib.Path, default=ROOT / "target/debug/hakorune")
    parser.add_argument(
        "--hako-timeout-sec",
        type=float,
        default=DEFAULT_HAKO_ADAPTER_TIMEOUT_SECONDS,
    )
    parser.add_argument("--output", type=pathlib.Path)
    args = parser.parse_args()
    if not args.bin.is_file():
        raise SystemExit(f"grammar contract report requires binary: {args.bin}")

    fixtures = fixtures_by_id()
    report = []
    for fixture_id in PROBE_FIXTURE_IDS:
        fixture = fixtures[fixture_id]
        report.append(
            {
                "fixture_id": fixture_id,
                "row_id": fixture["row_id"],
                "profile": fixture["profile"],
                "expected": {
                    "accepted": fixture["accepted"],
                    "normalized_form": fixture["normalized_form"],
                    "stable_reject_tag": fixture["stable_reject_tag"],
                },
                "rust": rust_observation(args.bin, fixture),
                "hako": hako_observation(args.bin, fixture, args.hako_timeout_sec),
            }
        )
    adapter_health = run_health_probe(
        binary=args.bin,
        probe_kind="health",
        source="",
        profile="canonical",
        timeout_seconds=args.hako_timeout_sec,
    )
    payload = json.dumps(
        {
            "schema": "language-v1-grammar-drift-v0",
            "hako_adapter_health": adapter_health,
            "probes": report,
        },
        indent=2,
    )
    if args.output:
        args.output.write_text(payload + "\n", encoding="utf-8")
    else:
        print(payload)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
