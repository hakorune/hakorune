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

import tomllib

from hako_adapter_health import (
    DEFAULT_HAKO_ADAPTER_TIMEOUT_SECONDS,
    run_adapter_process,
    run_health_probe,
)


ROOT = pathlib.Path(__file__).resolve().parents[2]
CORPUS = ROOT / "grammar/language-v1-grammar-contract-corpus.toml"
HAKO_ADAPTER = ROOT / "tools/language_v1/grammar_contract_hako_adapter.hako"
PROBE_FIXTURE_IDS = (
    "guard_expr_else_canonical",
    "try_statement_canonical_reject",
    "match_canonical",
    "from_super_call_canonical_reject",
)


def fixtures_by_id() -> dict[str, dict[str, Any]]:
    with CORPUS.open("rb") as handle:
        fixtures = tomllib.load(handle)["fixtures"]
    return {fixture["fixture_id"]: fixture for fixture in fixtures}


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
        source.write_text(fixture["source"], encoding="utf-8")
        completed = subprocess.run(
            [str(binary), "--emit-ast-json", str(ast), str(source)],
            cwd=ROOT,
            text=True,
            capture_output=True,
            check=False,
        )
    if completed.returncode == 0:
        return {"accepted": True, "normalized_kind": "ImplementationAccepted", "stable_reject_tag": ""}
    return {
        "accepted": False,
        "normalized_kind": "",
        "stable_reject_tag": reject_tag(completed.stderr),
    }


def hako_observation(
    binary: pathlib.Path, fixture: dict[str, Any], timeout_seconds: float
) -> dict[str, Any]:
    environment = os.environ | {"HAKO_GRAMMAR_CONTRACT_SOURCE": fixture["source"]}
    result = run_adapter_process(
        [str(binary), "--backend", "vm", str(HAKO_ADAPTER)],
        timeout_seconds=timeout_seconds,
        environment=environment,
    )
    if result.status == "ok":
        return {"accepted": True, "normalized_kind": "ImplementationAccepted", "stable_reject_tag": ""}
    return {
        "accepted": False,
        "normalized_kind": "",
        "stable_reject_tag": result.stable_reject_tag,
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
                    "normalized_kind": fixture["normalized_kind"],
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
