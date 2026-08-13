#!/usr/bin/env python3
"""Validate the source-backed G2 default/keep llvmlite census."""

from __future__ import annotations

import json
import subprocess
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
TAG = "llvm-llvmlite-default-independence-guard"
MANIFEST = ROOT / (
    "docs/development/current/main/investigations/"
    "llvmlite-default-independence-census-v0.json"
)
CALLER_MANIFEST = ROOT / (
    "docs/development/current/main/investigations/"
    "llvmlite-shared-smoke-caller-census-v0.json"
)
SCHEMA = "llvmlite-default-independence-census-v0"
CALLER_SCHEMA = "llvmlite-shared-smoke-caller-census-v0"
EXPECTED_IDS = {
    "g2-ci-min-gate-default",
    "g2-ci-min-gate-llvm-phi",
    "g2-ci-portability-default",
    "g2-ci-fast-smoke-compat",
    "g2-smoke-shared-env",
    "g2-smoke-runner-helper",
    "g2-smoke-exe-runner",
    "g2-perf-run-all",
    "g2-perf-microbench",
    "g2-perf-method-call-hot-trace",
    "g2-perf-compare-expr-cse",
    "g2-perf-phase2100-probe",
    "g2-smoke-static-config",
    "g2-smoke-auto-detect",
    "g2-smoke-matrix-config",
    "g2-perf-boundary-fence",
    "g2-build-llvm-tool",
}


def fail(message: str) -> None:
    print(f"[{TAG}] FAIL: {message}", file=sys.stderr)
    raise SystemExit(1)


def read_source(relative: str) -> str:
    path = ROOT / relative
    if not path.is_file():
        fail(f"missing source owner: {relative}")
    return path.read_text(encoding="utf-8")


def rg_paths(pattern: str) -> set[str]:
    result = subprocess.run(
        ["rg", "-l", pattern, "tools/smokes/v2", "--glob", "*.sh"],
        cwd=ROOT,
        check=False,
        capture_output=True,
        text=True,
    )
    if result.returncode not in (0, 1):
        fail(f"caller census rg failed for {pattern!r}: {result.stderr.strip()}")
    return {line for line in result.stdout.splitlines() if line}


def validate_caller_census() -> None:
    if not CALLER_MANIFEST.is_file():
        fail("shared smoke caller census is missing")
    try:
        data = json.loads(CALLER_MANIFEST.read_text(encoding="utf-8"))
    except json.JSONDecodeError as exc:
        fail(f"invalid shared smoke caller census JSON: {exc}")
    if data.get("schema") != CALLER_SCHEMA:
        fail("shared smoke caller census schema drifted")
    if data.get("status") != "g2-caller-census":
        fail("shared smoke caller census status drifted")
    if data.get("production_claim") or data.get("behavior_change") or data.get("g3_deletion"):
        fail("shared smoke caller census may not claim behavior or retirement")
    rows = data.get("rows")
    allowed = set(data.get("allowed_classes", []))
    if not isinstance(rows, list) or not rows or not allowed:
        fail("shared smoke caller census rows/classes are missing")
    ids: set[str] = set()
    paths: dict[str, str] = {}
    for row in rows:
        row_id = row.get("id")
        row_class = row.get("class")
        path = row.get("path")
        if not isinstance(row_id, str) or not row_id or row_id in ids:
            fail("shared smoke caller census ids must be unique")
        if row_class not in allowed:
            fail(f"{row_id}: unknown caller class {row_class!r}")
        if not isinstance(path, str) or not path or path in paths:
            fail(f"{row_id}: caller path is missing or duplicated")
        ids.add(row_id)
        paths[path] = row_class
        text = read_source(path)
        evidence = row.get("evidence")
        if not isinstance(evidence, list) or not evidence:
            fail(f"{row_id}: caller evidence is empty")
        for needle in evidence:
            if not isinstance(needle, str) or not needle or needle not in text:
                fail(f"{row_id}: missing caller evidence {needle!r}")
        if row_class == "explicit_compat":
            if "run_nyash_llvm" not in text or "NYASH_LLVM_USE_HARNESS=1" not in text:
                fail(f"{row_id}: explicit compat caller lacks explicit harness selector")
        elif row_class in {"default_pending", "default_boundary"}:
            if "NYASH_LLVM_USE_HARNESS=1" in text and path not in {
                "tools/smokes/v2/lib/test_runner_llvm_helpers.sh",
                "tools/smokes/v2/lib/result_checker.sh",
            }:
                fail(f"{row_id}: pending caller already has an explicit harness selector")

    observed = rg_paths("run_nyash_llvm") | rg_paths("check_parity")
    if set(paths) != observed:
        fail(f"shared smoke caller universe drifted: {sorted(set(paths) ^ observed)}")
    if paths.get("tools/smokes/v2/lib/test_runner_llvm_helpers.sh") != "helper_owner":
        fail("LLVM helper owner is missing")
    if paths.get("tools/smokes/v2/lib/result_checker.sh") != "helper_owner":
        fail("result checker owner is missing")
    print(
        f"[{TAG}] shared-smoke callers ok (rows={len(paths)}, "
        f"explicit_compat={sum(value == 'explicit_compat' for value in paths.values())}, "
        f"default_pending={sum(value == 'default_pending' for value in paths.values())})"
    )


def validate_shared_default_boundary() -> None:
    required = {
        "tools/smokes/v2/lib/env.sh": 'NYASH_LLVM_USE_HARNESS="${NYASH_LLVM_USE_HARNESS:-0}"',
        "tools/smokes/v2/configs/llvm_static.conf": 'NYASH_LLVM_USE_HARNESS="${NYASH_LLVM_USE_HARNESS:-0}"',
        "tools/smokes/v2/lib/test_runner_llvm_helpers.sh": 'NYASH_LLVM_USE_HARNESS="${NYASH_LLVM_USE_HARNESS:-0}"',
        "tools/smokes/v2/lib/result_checker.sh": 'local llvm_harness="${NYASH_LLVM_USE_HARNESS:-0}"',
    }
    for owner, needle in required.items():
        text = read_source(owner)
        if needle not in text:
            fail(f"shared default boundary missing neutral selector in {owner}")
        if "NYASH_LLVM_USE_HARNESS=1" in text:
            fail(f"shared default boundary still hardcodes harness in {owner}")
    config = read_source("tools/smokes/v2/configs/auto_detect.conf")
    if "config_type=\"llvm_static\"" not in config:
        fail("auto-detect llvm_static route disappeared")


def main() -> int:
    if not MANIFEST.is_file():
        fail("census manifest is missing")
    try:
        data = json.loads(MANIFEST.read_text(encoding="utf-8"))
    except json.JSONDecodeError as exc:
        fail(f"invalid JSON: {exc}")

    if data.get("schema") != SCHEMA:
        fail("manifest schema drifted")
    if data.get("status") != "g2-source-census":
        fail("manifest must remain a G2 source census")
    if data.get("production_claim") or data.get("behavior_change"):
        fail("G2 census may not claim production or behavior change")
    if data.get("g3_deletion"):
        fail("G2 census may not claim G3 deletion")

    rows = data.get("rows")
    if not isinstance(rows, list) or not rows:
        fail("manifest rows are empty")
    ids = [row.get("id") for row in rows]
    if any(not isinstance(item, str) or not item for item in ids):
        fail("row ids must be non-empty strings")
    if len(ids) != len(set(ids)):
        fail("row ids must be unique")
    if set(ids) != EXPECTED_IDS:
        fail(f"row universe drifted: {sorted(set(ids) ^ EXPECTED_IDS)}")

    allowed = set(data.get("allowed_classes", []))
    if not allowed:
        fail("allowed class set is empty")
    counts: dict[str, int] = {}
    pending: list[str] = []
    for row in rows:
        row_id = row["id"]
        row_class = row.get("class")
        if row_class not in allowed:
            fail(f"{row_id}: unknown class {row_class!r}")
        owner = row.get("owner")
        if not isinstance(owner, str) or not owner:
            fail(f"{row_id}: owner is missing")
        text = read_source(owner)
        evidence = row.get("evidence")
        if not isinstance(evidence, list) or not evidence:
            fail(f"{row_id}: source evidence is empty")
        for needle in evidence:
            if not isinstance(needle, str) or not needle or needle not in text:
                fail(f"{row_id}: missing source evidence {needle!r}")

        automatic = row.get("automatic_production")
        python_dependency = row.get("python_dependency")
        if not isinstance(automatic, bool):
            fail(f"{row_id}: automatic_production must be boolean")
        if not isinstance(python_dependency, str) or not python_dependency:
            fail(f"{row_id}: python_dependency is missing")
        if automatic and (row_class != "default_boundary" or python_dependency != "zero"):
            fail(f"{row_id}: automatic roots must be Boundary with Python=zero")
        if row_class == "default_pending":
            pending.append(row_id)
        counts[row_class] = counts.get(row_class, 0) + 1

    # These are the current G2 blockers, not a production-success assertion.
    required_pending = {
        "g2-perf-microbench",
    }
    if not required_pending.issubset(pending):
        fail("known G2 blocker rows disappeared from the pending set")

    bundle = read_source("tools/perf/run_phase21_5_perf_gate_bundle.sh")
    for toggle in (
        "PERF_GATE_METHOD_CALL_HOT_TRACE_CHECK",
        "PERF_GATE_COMPARE_EXPR_CSE_CHECK",
    ):
        if toggle in bundle:
            fail(f"perf bundle still selects oracle toggle by default: {toggle}")

    selectors = {
        "g2-perf-method-call-hot-trace": (
            "tools/smokes/v2/profiles/integration/apps/"
            "phase21_5_perf_method_call_hot_trace_contract_vm.sh",
            'BACKEND="${PERF_METHOD_CALL_HOT_TRACE_BACKEND:-}"',
        ),
        "g2-perf-compare-expr-cse": (
            "tools/smokes/v2/profiles/integration/apps/"
            "phase21_5_perf_compare_expr_cse_contract_vm.sh",
            'BACKEND="${PERF_COMPARE_EXPR_CSE_BACKEND:-}"',
        ),
    }
    for row_id, (owner, empty_default) in selectors.items():
        text = read_source(owner)
        if empty_default not in text:
            fail(f"{row_id}: direct oracle default is not empty")
        if ":-llvmlite" in text:
            fail(f"{row_id}: implicit llvmlite selector remains")
        if "perf_hot_trace_require_llvmlite_backend" not in text:
            fail(f"{row_id}: explicit llvmlite gate is missing")

    validate_caller_census()
    validate_shared_default_boundary()

    print(
        f"[{TAG}] ok (rows={len(rows)}, classes={counts}, "
        f"default_pending={len(pending)}, production_claim=0, g3_deletion=0)"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
