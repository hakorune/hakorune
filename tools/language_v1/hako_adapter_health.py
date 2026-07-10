#!/usr/bin/env python3
"""Bounded process-health boundary for the independent Hako grammar adapter."""

from __future__ import annotations

import argparse
from dataclasses import dataclass
import hashlib
import json
import os
import pathlib
import subprocess
from typing import Any, Mapping, Sequence

if __package__:
    from .grammar_contract_corpus import inventory_json_by_id
else:
    from grammar_contract_corpus import inventory_json_by_id


ROOT = pathlib.Path(__file__).resolve().parents[2]
HEALTH_PROBE = ROOT / "tools/language_v1/grammar_contract_hako_health_probe.hako"
GRAMMAR_ADAPTER = ROOT / "tools/language_v1/grammar_contract_hako_adapter.hako"
DEFAULT_HAKO_ADAPTER_TIMEOUT_SECONDS = 90.0
HEALTH_SCHEMA = "language-v1-hako-adapter-health-v0"


@dataclass(frozen=True)
class AdapterProcessResult:
    status: str
    stable_reject_tag: str
    raw_program_digest: str = ""

    def normalized(self) -> dict[str, str]:
        return {
            "status": self.status,
            "stable_reject_tag": self.stable_reject_tag,
            "raw_program_digest": self.raw_program_digest,
        }


def _single_json_object(stdout: str) -> AdapterProcessResult:
    lines = [line.strip() for line in stdout.splitlines() if line.strip()]
    if not lines:
        return AdapterProcessResult("error", "parser/hako_adapter_no_output")
    if len(lines) != 1:
        return AdapterProcessResult("error", "parser/hako_adapter_stdout_contaminated")
    try:
        payload = json.loads(lines[0])
    except json.JSONDecodeError:
        return AdapterProcessResult("error", "parser/hako_adapter_malformed_output")
    if not isinstance(payload, dict):
        return AdapterProcessResult("error", "parser/hako_adapter_malformed_output")
    if payload.get("schema") == "language-v1-hako-raw-evidence-v0":
        if payload.get("deterministic") is not True:
            return AdapterProcessResult(
                "error", "parser/hako_adapter_non_deterministic_output"
            )
        if payload.get("raw_program_json_authority") is not False:
            return AdapterProcessResult(
                "error", "parser/hako_raw_json_as_authority_forbidden"
            )
        if payload.get("status") == "error":
            tag = payload.get("stable_reject_tag")
            if not isinstance(tag, str) or not tag.startswith("parser/"):
                return AdapterProcessResult(
                    "error", "parser/hako_adapter_malformed_output"
                )
            return AdapterProcessResult("error", tag)
        if payload.get("status") != "ok" or not isinstance(payload.get("program"), dict):
            return AdapterProcessResult(
                "error", "parser/hako_adapter_malformed_output"
            )
        canonical_program = json.dumps(
            payload["program"], sort_keys=True, separators=(",", ":")
        )
        digest = hashlib.sha256(canonical_program.encode("utf-8")).hexdigest()
        return AdapterProcessResult("ok", "", digest)
    return AdapterProcessResult("ok", "")


def run_adapter_process(
    command: Sequence[str],
    *,
    timeout_seconds: float,
    environment: Mapping[str, str] | None = None,
) -> AdapterProcessResult:
    if timeout_seconds <= 0:
        raise ValueError("timeout_seconds must be positive")
    try:
        completed = subprocess.run(
            list(command),
            cwd=ROOT,
            env=dict(environment) if environment is not None else None,
            text=True,
            capture_output=True,
            timeout=timeout_seconds,
            check=False,
        )
    except subprocess.TimeoutExpired:
        return AdapterProcessResult("error", "parser/hako_adapter_timeout")
    except OSError:
        return AdapterProcessResult("error", "parser/hako_adapter_process_error")
    if completed.returncode != 0:
        return AdapterProcessResult("error", "parser/hako_adapter_process_error")
    return _single_json_object(completed.stdout)


def compare_repeated_results(
    first: AdapterProcessResult, second: AdapterProcessResult
) -> AdapterProcessResult:
    if first.normalized() != second.normalized():
        return AdapterProcessResult(
            "error", "parser/hako_adapter_non_deterministic_output"
        )
    return first


def health_envelope(
    *, probe_kind: str, result: AdapterProcessResult, deterministic: bool
) -> dict[str, Any]:
    envelope = {
        "schema": HEALTH_SCHEMA,
        "adapter_kind": "hako_grammar_contract_adapter",
        "probe_kind": probe_kind,
        "status": result.status,
        "stable_reject_tag": result.stable_reject_tag,
        "bounded": True,
        "deterministic": deterministic,
        "raw_program_json_authority": False,
        "parse_witness_conformance": False,
    }
    if result.raw_program_digest:
        envelope["raw_program_digest"] = result.raw_program_digest
    return envelope


def probe_command(
    binary: pathlib.Path, probe_kind: str, profile: str
) -> list[str] | None:
    entry = {
        "health": HEALTH_PROBE,
        "observation": GRAMMAR_ADAPTER,
    }.get(probe_kind)
    if entry is None:
        return None
    command = [str(binary), "--backend", "vm", str(entry)]
    if probe_kind == "observation":
        command.extend(["--", "--grammar-profile", profile])
    return command


def run_health_probe(
    *,
    binary: pathlib.Path,
    probe_kind: str,
    source: str,
    profile: str = "canonical",
    timeout_seconds: float,
    inventory_json: str = "[]",
    environment: Mapping[str, str] | None = None,
) -> dict[str, Any]:
    command = probe_command(binary, probe_kind, profile)
    if command is None:
        result = AdapterProcessResult("error", "parser/hako_adapter_probe_unknown")
        return health_envelope(probe_kind=probe_kind, result=result, deterministic=True)
    child_environment = dict(os.environ if environment is None else environment)
    if probe_kind == "observation":
        child_environment["HAKO_GRAMMAR_CONTRACT_SOURCE"] = source
        child_environment["HAKO_GRAMMAR_CONTRACT_INVENTORY_JSON"] = inventory_json
    first = run_adapter_process(
        command, timeout_seconds=timeout_seconds, environment=child_environment
    )
    if probe_kind == "observation":
        deterministic = (
            first.stable_reject_tag
            != "parser/hako_adapter_non_deterministic_output"
        )
        return health_envelope(
            probe_kind=probe_kind,
            result=first,
            deterministic=deterministic,
        )
    second = run_adapter_process(
        command, timeout_seconds=timeout_seconds, environment=child_environment
    )
    result = compare_repeated_results(first, second)
    deterministic = result.stable_reject_tag != "parser/hako_adapter_non_deterministic_output"
    return health_envelope(
        probe_kind=probe_kind, result=result, deterministic=deterministic
    )


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--bin", type=pathlib.Path, default=ROOT / "target/debug/hakorune")
    parser.add_argument("--probe", default="health")
    parser.add_argument("--source", default="local x = 1")
    parser.add_argument("--profile", default="canonical")
    parser.add_argument("--inventory-json", default="")
    parser.add_argument("--inventory-id", default="")
    parser.add_argument(
        "--timeout-sec", type=float, default=DEFAULT_HAKO_ADAPTER_TIMEOUT_SECONDS
    )
    args = parser.parse_args()
    try:
        inventory_json = args.inventory_json or inventory_json_by_id(args.inventory_id)
    except KeyError:
        envelope = health_envelope(
            probe_kind=args.probe,
            result=AdapterProcessResult(
                "error", "parser/hako_inventory_context_unknown"
            ),
            deterministic=True,
        )
        print(json.dumps(envelope, sort_keys=True, separators=(",", ":")))
        return 2
    if not args.bin.is_file():
        envelope = health_envelope(
            probe_kind=args.probe,
            result=AdapterProcessResult("error", "parser/hako_adapter_process_error"),
            deterministic=True,
        )
    else:
        envelope = run_health_probe(
            binary=args.bin,
            probe_kind=args.probe,
            source=args.source,
            profile=args.profile,
            timeout_seconds=args.timeout_sec,
            inventory_json=inventory_json,
        )
    print(json.dumps(envelope, sort_keys=True, separators=(",", ":")))
    return 0 if envelope["status"] == "ok" else 2


if __name__ == "__main__":
    raise SystemExit(main())
