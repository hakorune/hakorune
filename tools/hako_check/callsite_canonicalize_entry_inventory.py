#!/usr/bin/env python3
"""Inventory production callsite_canonicalize entry points.

This tool is read-only. It does not move, delete, or reorder canonicalization.
It makes the current multi-entry shape explicit so a later design row can
decide whether to centralize scheduling without guessing from grep output.
"""

from __future__ import annotations

import argparse
import json
import re
from pathlib import Path


ENTRY_RE = re.compile(r"canonicalize_for_site\s*\(")
TRANSFORM_RE = re.compile(r"canonicalize_callsites\s*\(")
DEFAULT_INCLUDE_ROOTS = ("src",)
SKIP_PARTS = {
    ".git",
    "target",
    "__pycache__",
}


def iter_source_files(root: Path) -> list[Path]:
    files: list[Path] = []
    for rel in DEFAULT_INCLUDE_ROOTS:
        base = root / rel
        if not base.exists():
            continue
        for path in base.rglob("*"):
            if not path.is_file():
                continue
            if path.suffix != ".rs":
                continue
            if any(part in SKIP_PARTS for part in path.parts):
                continue
            files.append(path)
    return sorted(files)


def is_test_path(path: Path) -> bool:
    parts = set(path.parts)
    return "tests" in parts or path.name.startswith("test_")


def classify_entry(path: Path) -> str:
    text = str(path).replace("\\", "/")
    if text.endswith("src/mir/compiler/mod.rs"):
        return "mir_compiler_post_rc"
    if text.endswith("src/mir/optimizer/core.rs"):
        return "mir_optimizer_late_call_and_inline"
    if text.endswith("src/runner/json_v0_bridge/core.rs"):
        return "program_json_v0_bridge"
    if text.endswith("src/runner/mir_json_v0.rs"):
        return "mir_json_v0_loader"
    if text.endswith("src/mir/passes/callsite_canonicalize/pass.rs"):
        return "pass_owner_definition"
    if text.endswith("src/mir/passes/callsite_canonicalize/schedule.rs"):
        return "schedule_owner_definition"
    return "unknown"


def find_entries(root: Path) -> list[dict[str, str]]:
    rows: list[dict[str, str]] = []
    for path in iter_source_files(root):
        if is_test_path(path):
            continue
        rel = path.relative_to(root)
        for lineno, line in enumerate(path.read_text(encoding="utf-8").splitlines(), start=1):
            is_facade = bool(ENTRY_RE.search(line))
            is_transform = bool(TRANSFORM_RE.search(line))
            if not is_facade and not is_transform:
                continue
            kind = classify_entry(rel)
            if kind in {"pass_owner_definition", "schedule_owner_definition"}:
                continue
            rows.append(
                {
                    "path": str(rel),
                    "line": str(lineno),
                    "entry_kind": kind,
                    "call_kind": "schedule_facade" if is_facade else "direct_transform",
                }
            )
    return rows


def build_report(root: Path) -> dict[str, object]:
    entries = find_entries(root)
    known = [row for row in entries if row["entry_kind"] != "unknown"]
    unknown = [row for row in entries if row["entry_kind"] == "unknown"]
    kinds = {row["entry_kind"] for row in known}
    centralized = (
        len(entries) == 4
        and not unknown
        and all(row.get("call_kind") == "schedule_facade" for row in known)
    )
    return {
        "output_contract": "hako-callsite-canonicalize-entry-inventory-v0",
        "production_entry_count": str(len(entries)),
        "known_entry_count": str(len(known)),
        "unknown_entry_count": str(len(unknown)),
        "mir_compiler_entry": "1" if "mir_compiler_post_rc" in kinds else "0",
        "mir_optimizer_entry": "1" if "mir_optimizer_late_call_and_inline" in kinds else "0",
        "program_json_v0_bridge_entry": "1" if "program_json_v0_bridge" in kinds else "0",
        "mir_json_v0_loader_entry": "1" if "mir_json_v0_loader" in kinds else "0",
        "transform_owner": "src/mir/passes/callsite_canonicalize",
        "single_transform_owner": "1",
        "centralized_schedule_owner": "1" if centralized else "0",
        "behavior_changed": "0",
        "canonicalize_entry_refactor_allowed": "0",
        "entry_removal_enabled": "0",
        "schedule_reorder_enabled": "0",
        "next_task": "CALLSITE-CANONICALIZE-SCHEDULE-FACADE-001" if not centralized else "CALLSITE-CANONICALIZE-SCHEDULE-FACADE-CLOSEOUT-001",
        "summary": "ok",
        "entries": entries,
    }


def emit_kv(report: dict[str, object]) -> None:
    for key, value in report.items():
        if key == "entries":
            continue
        print(f"{key}={value}")
    entries = report.get("entries")
    if isinstance(entries, list):
        for idx, row in enumerate(entries):
            if not isinstance(row, dict):
                continue
            for key, value in row.items():
                print(f"entry_{idx}_{key}={value}")


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", type=Path, default=Path.cwd())
    parser.add_argument("--format", choices=("kv", "json"), default="kv")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    report = build_report(args.repo_root.resolve())
    if args.format == "json":
        print(json.dumps(report, indent=2, sort_keys=True))
    else:
        emit_kv(report)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
