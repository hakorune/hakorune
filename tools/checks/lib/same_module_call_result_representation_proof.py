#!/usr/bin/env python3
"""Observe forward same-module call-result representation timing for S0."""

from __future__ import annotations

import argparse
import json
import os
from pathlib import Path
import re
import subprocess
import sys
from typing import Any


APP_DIR = Path("apps/same-module-call-result-representation-proof")
ARTIFACT_DIR = Path("target/checks/same-module-call-result-representation-proof")
MISSING = "GenericLoop carrier representation failed: MissingTransientType { init:"
CASES = {
    "forward_direct": {"expected_rc": 1, "outcome": "missing"},
    "forward_copy": {"expected_rc": 1, "outcome": "missing"},
    "reverse_direct": {"expected_rc": 6, "outcome": "exact"},
    "typed_forward": {"expected_rc": 1, "outcome": "missing"},
    "valid_numeric_control": {"expected_rc": 6, "outcome": "exact"},
}


class ProofFailure(RuntimeError):
    pass


def run(
    argv: list[str], root: Path, env: dict[str, str] | None = None
) -> subprocess.CompletedProcess[str]:
    merged = os.environ.copy()
    if env:
        merged.update(env)
    return subprocess.run(
        argv, cwd=root, env=merged, text=True, capture_output=True, check=False
    )


def require_success(completed: subprocess.CompletedProcess[str], label: str) -> None:
    if completed.returncode != 0:
        raise ProofFailure(
            f"{label} failed rc={completed.returncode}\n"
            f"stdout:\n{completed.stdout}\nstderr:\n{completed.stderr}"
        )


def build_bins(root: Path) -> dict[str, Path]:
    common = ["cargo", "build", "-q", "--features", "vm-reference", "--bin", "hakorune"]
    require_success(run(common, root), "debug build")
    require_success(run(common[:2] + ["--release"] + common[2:], root), "release build")
    return {
        "debug": root / "target/debug/hakorune",
        "release": root / "target/release/hakorune",
    }


def verify_source(root: Path) -> dict[str, Any]:
    sources = {
        name: (root / APP_DIR / f"{name}.hako").read_text(encoding="utf-8")
        for name in CASES
    }
    combined = "\n".join(sources.values())
    if re.search(r"\bHMI\b|\bHmi", combined):
        raise ProofFailure("S0 fixture must remain HMI-independent")

    forward = sources["forward_direct"]
    if forward.index("static box OrderConsumerV1") > forward.index(
        "static box OrderProviderV1"
    ):
        raise ProofFailure("forward provider must remain after its consumer")
    reverse = sources["reverse_direct"]
    if reverse.index("static box OrderProviderV1") > reverse.index(
        "static box OrderConsumerV1"
    ):
        raise ProofFailure("reverse provider must remain before its consumer")
    for name in ("forward_direct", "forward_copy", "reverse_direct"):
        if re.search(r"\bseed\(\)\s*:\s*", sources[name]):
            raise ProofFailure(f"{name} provider must remain return-untyped")
    if len(re.findall(r"\bseed\(\)\s*:\s*i64", sources["typed_forward"])) != 1:
        raise ProofFailure("typed forward control must contain one exact i64 return annotation")
    if "local pos = initial" not in sources["forward_copy"]:
        raise ProofFailure("forward copy case must retain one explicit local carrier copy")
    if "ProviderV1" in sources["valid_numeric_control"]:
        raise ProofFailure("numeric control must not depend on a same-module provider")

    for path in (root / APP_DIR).glob("*.hako"):
        lines = len(path.read_text(encoding="utf-8").splitlines())
        if lines >= 800:
            raise ProofFailure(f"source must stay below 800 lines: {path} has {lines}")
    return {
        "forward_provider_after_consumer": True,
        "reverse_provider_before_consumer": True,
        "untyped_provider_cases": 3,
        "typed_forward_controls": 1,
        "hmi_source_mentions": 0,
    }


def run_case(root: Path, binary: Path, mode: str, name: str) -> dict[str, Any]:
    fixture = root / APP_DIR / f"{name}.hako"
    completed = run(
        [str(binary), "--backend", "mir", str(fixture)],
        root,
        {
            "HAKO_EMIT_EXE_CACHE": "0",
            "HAKO_JOINIR_DEBUG": "1",
            "NYASH_DEBUG_ANNOTATION": "1",
        },
    )
    log = completed.stdout + completed.stderr
    (root / ARTIFACT_DIR / f"{mode}.{name}.log").write_text(log, encoding="utf-8")

    expected = CASES[name]
    if completed.returncode != expected["expected_rc"]:
        raise ProofFailure(
            f"{mode} {name} rc drift: expected {expected['expected_rc']}, "
            f"got {completed.returncode}\n{log}"
        )
    missing = MISSING in log
    if expected["outcome"] == "missing" and not missing:
        raise ProofFailure(f"{mode} {name} must expose MissingTransientType")
    if expected["outcome"] == "exact":
        if missing or "route=generic_loop_v1" not in log:
            raise ProofFailure(f"{mode} {name} must execute the exact GenericLoop route")
    return {
        "returncode": completed.returncode,
        "transient_state": "Missing" if missing else "Exact",
        "generic_loop_route": "generic_loop_v1" in log,
    }


def instructions(function: dict[str, Any]):
    for block in function.get("blocks", []):
        yield from block.get("instructions", [])


def final_mir_diagnostic(root: Path, binary: Path, mode: str) -> dict[str, Any]:
    output = root / ARTIFACT_DIR / f"{mode}.reverse_direct.mir.json"
    fixture = root / APP_DIR / "reverse_direct.hako"
    completed = run([str(binary), "--emit-mir-json", str(output), str(fixture)], root)
    require_success(completed, f"{mode} reverse MIR emission")
    document = json.loads(output.read_text(encoding="utf-8"))
    rows = [
        function
        for function in document.get("functions", [])
        if function.get("name") == "OrderConsumerV1.run/1"
    ]
    if len(rows) != 1:
        raise ProofFailure(f"{mode} reverse consumer function cardinality drift")
    function = rows[0]
    insts = list(instructions(function))
    calls = [
        inst
        for inst in insts
        if inst.get("op") == "mir_call"
        and inst.get("mir_call", {}).get("callee", {}).get("name")
        == "OrderProviderV1.seed/0"
    ]
    if len(calls) != 1 or not isinstance(calls[0].get("dst"), int):
        raise ProofFailure(f"{mode} reverse call row drift")
    call_dst = calls[0]["dst"]
    copies = [
        inst
        for inst in insts
        if inst.get("op") == "copy" and inst.get("src") == call_dst
    ]
    if len(copies) != 1 or not isinstance(copies[0].get("dst"), int):
        raise ProofFailure(f"{mode} reverse call-result copy drift")
    copy_dst = copies[0]["dst"]
    phis = [
        inst
        for inst in insts
        if inst.get("op") == "phi"
        and any(row[0] == copy_dst for row in inst.get("incoming", []))
    ]
    if len(phis) != 1 or phis[0].get("dst_type") != "i64":
        raise ProofFailure(f"{mode} reverse carrier PHI representation drift")
    types = function.get("metadata", {}).get("value_types", {})
    for value in (call_dst, copy_dst, phis[0].get("dst")):
        if types.get(str(value)) != "i64":
            raise ProofFailure(f"{mode} final diagnostic value %{value} is not i64")
    return {
        "authority": "diagnostic-only-final-metadata",
        "call_result_type": "i64",
        "copy_result_type": "i64",
        "phi_result_type": "i64",
    }


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("root", nargs="?", default=".")
    args = parser.parse_args()
    root = Path(args.root).resolve()
    (root / ARTIFACT_DIR).mkdir(parents=True, exist_ok=True)

    source = verify_source(root)
    bins = build_bins(root)
    modes: dict[str, Any] = {}
    for mode, binary in bins.items():
        cases = {
            name: run_case(root, binary, mode, name)
            for name in CASES
        }
        modes[mode] = {
            "cases": cases,
            "final_mir_diagnostic": final_mir_diagnostic(root, binary, mode),
        }
    if modes["debug"] != modes["release"]:
        raise ProofFailure("debug/release normalized observation drift")

    observation = {
        "schema_version": 1,
        "row": "R0-SAME-MODULE-CALL-RESULT-REP0-S0",
        "selection": "FORWARD_UNTYPED_RESULT_MISSING_BEFORE_CONSUMER",
        "production_behavior_delta": 0,
        "production_type_publishers": 0,
        "source": source,
        "normalized": modes["debug"],
    }
    (root / ARTIFACT_DIR / "s0_observation.json").write_text(
        json.dumps(observation, indent=2, sort_keys=True) + "\n", encoding="utf-8"
    )
    print("selection=FORWARD_UNTYPED_RESULT_MISSING_BEFORE_CONSUMER")
    print("forward_direct_transient=Missing")
    print("forward_copy_transient=Missing")
    print("typed_forward_transient=Missing")
    print("reverse_direct_transient=Exact")
    print("valid_numeric_control=Exact")
    print("final_metadata_authority=diagnostic-only")
    print("production_behavior_delta=0")
    print("production_type_publishers=0")
    print("debug_release_parity=green")
    print("summary=observed")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except ProofFailure as exc:
        print(f"[same-module-call-result-representation-proof] ERROR: {exc}", file=sys.stderr)
        raise SystemExit(1)
