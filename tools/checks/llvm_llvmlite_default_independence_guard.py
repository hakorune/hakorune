#!/usr/bin/env python3
"""Validate the source-backed G2 default/keep llvmlite census."""

from __future__ import annotations

import json
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
TAG = "llvm-llvmlite-default-independence-guard"
MANIFEST = ROOT / (
    "docs/development/current/main/investigations/"
    "llvmlite-default-independence-census-v0.json"
)
SCHEMA = "llvmlite-default-independence-census-v0"
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
    if not {"g2-smoke-shared-env", "g2-smoke-runner-helper", "g2-perf-method-call-hot-trace"}.issubset(pending):
        fail("known G2 blocker rows disappeared from the pending set")

    print(
        f"[{TAG}] ok (rows={len(rows)}, classes={counts}, "
        f"default_pending={len(pending)}, production_claim=0, g3_deletion=0)"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
