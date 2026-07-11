#!/usr/bin/env python3
"""Build and validate the S3 control-flow evidence inventory.

This manifest is evidence-only.  It does not select a Unit/absence/Err/Fault
meaning and does not alter any control-flow implementation.
"""

from __future__ import annotations

import argparse
import json
import re
from pathlib import Path
from typing import Any, Iterable


ROOT = Path(__file__).resolve().parents[2]
OUTPUT = ROOT / "tools/checks/manifests/failure_outcome_control_flow_inventory_v0.json"

FAMILIES = (
    "local_default",
    "return_unit",
    "fault_and_throw",
    "cleanup_precedence",
    "catchability",
    "top_level_normalization",
)
PENDING_REASONS = frozenset({"ControlFlowOwnerPending", "PrecedenceContractPending"})


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    mode = parser.add_mutually_exclusive_group(required=True)
    mode.add_argument("--write", action="store_true")
    mode.add_argument("--check", action="store_true")
    parser.add_argument("--output", type=Path, default=OUTPUT)
    return parser.parse_args()


def relative(path: Path) -> str:
    return path.relative_to(ROOT).as_posix()


def source_files(prefixes: tuple[str, ...]) -> Iterable[Path]:
    for prefix in prefixes:
        root = ROOT / prefix
        if root.is_file():
            yield root
        elif root.is_dir():
            yield from sorted(path for path in root.rglob("*") if path.is_file())


def scan(
    family: str,
    prefixes: tuple[str, ...],
    patterns: tuple[tuple[str, str], ...],
    path_filter: re.Pattern[str],
) -> list[dict[str, Any]]:
    rows: list[dict[str, Any]] = []
    for path in source_files(prefixes):
        path_text = relative(path)
        if not path_filter.search(path_text):
            continue
        lines = path.read_text(encoding="utf-8", errors="ignore").splitlines()
        for line_number, line in enumerate(lines, start=1):
            for token, pattern in patterns:
                if not re.search(pattern, line, flags=re.IGNORECASE):
                    continue
                rows.append(
                    {
                        "control_flow_evidence_id": f"control.{family}.{path_text}:{line_number}:{token}",
                        "family": family,
                        "source_path": path_text,
                        "line": line_number,
                        "token": token,
                        "evidence_kind": token,
                        "evidence": line.strip(),
                        "site_ref": None,
                        "resolution": "Pending",
                        "pending_reason": pending_reason(family),
                    }
                )
    return rows


def pending_reason(family: str) -> str:
    return "PrecedenceContractPending" if family == "cleanup_precedence" else "ControlFlowOwnerPending"


def build_manifest() -> dict[str, Any]:
    rows: list[dict[str, Any]] = []
    rows += scan(
        "local_default",
        ("src/mir", "docs/reference"),
        (("uninitialized_local", r"uninitialized"), ("local_default", r"local.*default|default.*local")),
        re.compile(r"(?:variable_stmt|pass1_discovery|local_slot|constraints|invariants|runtime)"),
    )
    rows += scan(
        "return_unit",
        ("src/mir", "src/backend", "docs/reference"),
        (("mir_return", r"Return \{"), ("void_carrier", r"VMValue::Void"), ("void_return_shape", r"return_shape.*void")),
        re.compile(r"(?:builder|return|interpreter|vm_types|substrate|mir-to-vm)"),
    )
    rows += scan(
        "fault_and_throw",
        ("src/mir", "src/backend", "docs/reference"),
        (("throw_instruction", r"\bThrow\b"), ("fault_surface", r"\bFault\b"), ("throw_reserved", r"throw_reserved")),
        re.compile(r"(?:throw|exception|fault|control_flow|instruction_set|environment|parser_mvp)"),
    )
    rows += scan(
        "cleanup_precedence",
        ("src", "docs/reference"),
        (("cleanup", r"\bcleanup\b"), ("finally", r"\bfinally\b"), ("defer", r"\bdefer\b"), ("fini", r"\bfini\b")),
        re.compile(r"(?:cleanup|finally|defer|fini|try|macro/ast_json|parser_mvp|constraints|invariants)"),
    )
    rows += scan(
        "catchability",
        ("src", "docs/reference"),
        (("postfix_catch", r"postfix_catch"), ("catch_block", r"\bCatch\b"), ("catchable", r"catchable")),
        re.compile(r"(?:catch|try|exception|constraints|invariants|parser_mvp|environment)"),
    )
    rows += scan(
        "top_level_normalization",
        ("src", "docs/reference"),
        (("outcome_normalize", r"normalize.*(?:outcome|return)|(?:outcome|return).*normalize"), ("top_level", r"top[-_ ]level"), ("entry_outcome", r"entry.*outcome")),
        re.compile(r"(?:runner|runtime|return|outcome|invariants|architecture)"),
    )
    rows.sort(key=lambda row: row["control_flow_evidence_id"])
    return {
        "schema_version": 0,
        "status": "control_flow_evidence_inventory",
        "semantic_activation": 0,
        "families": list(FAMILIES),
        "pending_reasons": sorted(PENDING_REASONS),
        "control_flow_evidence": rows,
    }


def validate(manifest: dict[str, Any]) -> list[str]:
    errors: list[str] = []
    if manifest.get("schema_version") != 0:
        errors.append("schema_version must be 0")
    if manifest.get("semantic_activation") != 0:
        errors.append("semantic activation must remain 0")
    if tuple(manifest.get("families", ())) != FAMILIES:
        errors.append("control-flow family vocabulary drift")
    rows = manifest.get("control_flow_evidence", [])
    ids = [row.get("control_flow_evidence_id") for row in rows]
    if len(ids) != len(set(ids)):
        errors.append("duplicate control-flow evidence id")
    seen = set()
    for row in rows:
        evidence_id = row.get("control_flow_evidence_id")
        family = row.get("family")
        seen.add(family)
        if family not in FAMILIES:
            errors.append(f"unknown control-flow family: {evidence_id}")
        if not row.get("source_path") or not row.get("line") or not row.get("token"):
            errors.append(f"control-flow location incomplete: {evidence_id}")
        if not row.get("evidence"):
            errors.append(f"control-flow evidence text missing: {evidence_id}")
        if row.get("resolution") != "Pending":
            errors.append(f"control-flow row must remain pending: {evidence_id}")
        if row.get("site_ref") is not None:
            errors.append(f"pending control-flow row has site reference: {evidence_id}")
        if row.get("pending_reason") not in PENDING_REASONS:
            errors.append(f"unknown control-flow pending reason: {evidence_id}")
    missing = set(FAMILIES) - seen
    if missing:
        errors.append(f"control-flow families missing: {sorted(missing)}")
    if not rows:
        errors.append("control-flow evidence inventory is empty")
    return errors


def main() -> int:
    args = parse_args()
    expected = json.dumps(build_manifest(), ensure_ascii=True, indent=2, sort_keys=True) + "\n"
    if args.write:
        args.output.write_text(expected, encoding="utf-8")
        print(f"[failure-outcome-control-flow] wrote {args.output}")
        return 0
    actual = args.output.read_text(encoding="utf-8") if args.output.is_file() else ""
    if actual != expected:
        print("[failure-outcome-control-flow] drift detected")
        return 1
    errors = validate(json.loads(actual))
    if errors:
        for error in errors:
            print(f"[failure-outcome-control-flow] {error}")
        return 1
    rows = json.loads(actual)["control_flow_evidence"]
    counts = {family: sum(row["family"] == family for row in rows) for family in FAMILIES}
    print(f"[failure-outcome-control-flow] rows={len(rows)} counts={counts}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
