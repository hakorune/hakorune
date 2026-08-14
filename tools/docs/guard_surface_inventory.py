#!/usr/bin/env python3
"""Generate a source-backed, non-authoritative guard-surface inventory.

This tool observes the tracked check tree and the existing index/manifests.  It
never runs a check, changes a file, or authorizes retirement.  Unknown rows are
deliberately retained so that a later migration can add evidence without
silently converting a missing owner into a deletion decision.
"""

from __future__ import annotations

import argparse
import json
import re
import subprocess
import sys
import tomllib
from collections import Counter, defaultdict
from pathlib import Path
from typing import Any, Iterable


ROOT = Path(__file__).resolve().parents[2]
INDEX = ROOT / "docs/tools/check-scripts-index.md"
GUARD_MANIFEST = ROOT / "tools/checks/guard_rows.toml"
PROOF_MANIFEST = ROOT / "tools/checks/proof_apps.toml"
SCHEMA = "guard-surface-inventory-v0"
ALLOWED = (
    "stable_public_entry",
    "family_manifest_case",
    "focused_behavior_test",
    "historical_archive",
    "delete_after_equivalent_coverage",
    "unknown_retain",
)


def git_lines(*args: str) -> list[str]:
    return subprocess.check_output(["git", *args], cwd=ROOT, text=True).splitlines()


def git_commit() -> str:
    return subprocess.check_output(
        ["git", "rev-parse", "HEAD"], cwd=ROOT, text=True
    ).strip()


def command_paths(value: Any) -> set[str]:
    """Extract repository paths from a manifest command value."""
    found: set[str] = set()
    if isinstance(value, str):
        for match in re.findall(r"tools/checks/[^\s\"'`|,)]+", value):
            found.add(match.rstrip(".;:"))
    elif isinstance(value, list):
        for item in value:
            found.update(command_paths(item))
    elif isinstance(value, dict):
        for item in value.values():
            found.update(command_paths(item))
    return {path for path in found if "*" not in path}


def load_manifest(path: Path, table_key: str, stack: tuple[Path, ...] = ()) -> list[dict[str, Any]]:
    if path in stack:
        chain = " -> ".join(str(item.relative_to(ROOT)) for item in (*stack, path))
        raise ValueError(f"manifest include cycle: {chain}")
    data = tomllib.loads(path.read_text(encoding="utf-8"))
    rows: list[dict[str, Any]] = []
    for include in data.get("includes", []):
        rows.extend(load_manifest(ROOT / include, table_key, (*stack, path)))
    for row in data.get(table_key, []):
        if isinstance(row, dict):
            rows.append(row)
    return rows


def index_sources() -> tuple[dict[str, list[str]], set[str], set[str], int, int]:
    text = INDEX.read_text(encoding="utf-8")
    owners: dict[str, list[str]] = defaultdict(list)
    table_rows = 0
    for line in text.splitlines():
        if not line.startswith("| `"):
            continue
        table_rows += 1
        match = re.search(r"tools/checks/[^\s`|,)]+", line)
        if match:
            owners[match.group(0).rstrip(".;:")].append("index.stable_table")
    stable_paths = set(owners)
    compat = text.split("<!-- legacy-guard-name-compat-begin", 1)
    compat_text = compat[1].split("legacy-guard-name-compat-end -->", 1)[0] if len(compat) == 2 else ""
    compatibility_paths: set[str] = set()
    compatibility_entries = 0
    for path in (item.strip() for item in compat_text.replace("\n", " ").split(",")):
        if not path.startswith("tools/checks/"):
            continue
        compatibility_entries += 1
        path = path.rstrip(".;:")
        if "*" not in path:
            compatibility_paths.add(path)
            owners[path].append("index.compatibility_block")
    return owners, stable_paths, compatibility_paths, table_rows, compatibility_entries


def manifest_sources(path: Path, table_key: str, label: str) -> dict[str, list[str]]:
    owners: dict[str, list[str]] = defaultdict(list)
    for index, row in enumerate(load_manifest(path, table_key), start=1):
        row_id = row.get("id", f"{table_key}[{index}]")
        for command in command_paths(row):
            owners[command].append(f"{label}:{row_id}")
    return owners


def build_inventory() -> dict[str, Any]:
    tracked = sorted(git_lines("ls-files", "--", "tools/checks"))
    index_owner, stable_paths, compatibility_paths, table_rows, compatibility_entries = index_sources()
    guard_owner = manifest_sources(GUARD_MANIFEST, "rows", "guard_rows")
    proof_owner = manifest_sources(PROOF_MANIFEST, "proof_apps", "proof_apps")
    rows: list[dict[str, Any]] = []
    for path in tracked:
        owners = sorted(set(index_owner.get(path, ())))
        owners.extend(sorted(set(guard_owner.get(path, ()))))
        owners.extend(sorted(set(proof_owner.get(path, ()))))
        compat = "index.compatibility_block" in owners
        stable = "index.stable_table" in owners
        manifest = any(owner.startswith(("guard_rows:", "proof_apps:")) for owner in owners)
        if stable:
            disposition = "stable_public_entry"
            evidence = "human index stable table"
        elif manifest:
            disposition = "family_manifest_case"
            evidence = "existing declarative manifest"
        else:
            disposition = "unknown_retain"
            evidence = "no retirement authority observed"
        rows.append(
            {
                "path": path,
                "owner": owners or ["unclassified.tracked_check"],
                "evidence": evidence,
                "compatibility_name": compat,
                "disposition": disposition,
            }
        )
    counts = Counter(row["disposition"] for row in rows)
    tracked_set = set(tracked)
    manifest_paths = set(guard_owner) | set(proof_owner)
    return {
        "schema": SCHEMA,
        "status": "inventory-only",
        "source_commit": git_commit(),
        "tracked_root": "tools/checks",
        "production_claim": False,
        "behavior_change": False,
        "retirement_authorized": False,
        "source_authority": [
            "git ls-files tools/checks",
            "docs/tools/check-scripts-index.md",
            "tools/checks/guard_rows.toml",
            "tools/checks/proof_apps.toml",
        ],
        "non_authority": [
            "grep-only caller counts",
            "executable mode",
            "historical prose",
            "this generated inventory",
        ],
        "allowed_dispositions": list(ALLOWED),
        "source_counts": {
            "tracked_paths": len(tracked),
            "index_table_rows": table_rows,
            "index_non_check_rows": table_rows - len(stable_paths),
            "index_stable_check_paths": len(stable_paths),
            "index_stable_check_tracked": len(stable_paths & tracked_set),
            "index_stable_check_untracked": len(stable_paths - tracked_set),
            "compatibility_entries": compatibility_entries,
            "compatibility_paths": len(compatibility_paths),
            "compatibility_tracked": len(compatibility_paths & tracked_set),
            "manifest_paths": len(manifest_paths),
            "manifest_tracked": len(manifest_paths & tracked_set),
            "manifest_untracked": len(manifest_paths - tracked_set),
        },
        "counts": dict(sorted(counts.items())),
        "rows": rows,
    }


def validate(payload: dict[str, Any]) -> list[str]:
    rows = payload.get("rows", [])
    paths = [row.get("path") for row in rows if isinstance(row, dict)]
    errors: list[str] = []
    if len(paths) != payload["source_counts"]["tracked_paths"]:
        errors.append("tracked path coverage is incomplete")
    if len(paths) != len(set(paths)):
        errors.append("duplicate inventory path")
    bad = [row.get("disposition") for row in rows if row.get("disposition") not in ALLOWED]
    if bad:
        errors.append(f"unknown disposition(s): {sorted(set(bad))}")
    counts = Counter(row["disposition"] for row in rows)
    if sum(counts.values()) != len(rows):
        errors.append("disposition count does not cover rows")
    return errors


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output", type=Path, help="write JSON instead of stdout")
    parser.add_argument("--check", action="store_true", help="validate coverage and print a compact summary")
    args = parser.parse_args()
    inventory = build_inventory()
    if args.check:
        errors = validate(inventory)
        if errors:
            for error in errors:
                print(f"[guard-surface-inventory] ERROR: {error}", file=sys.stderr)
            return 1
        counts = ", ".join(
            f"{key}={value}" for key, value in sorted(inventory["counts"].items())
        )
        print(
            "[guard-surface-inventory] ok: "
            f"rows={len(inventory['rows'])}, {counts}, "
            f"index_untracked={inventory['source_counts']['index_stable_check_untracked']}, "
            f"manifest_untracked={inventory['source_counts']['manifest_untracked']}"
        )
        return 0
    payload = json.dumps(inventory, ensure_ascii=False, indent=2) + "\n"
    if args.output:
        args.output.write_text(payload, encoding="utf-8")
    else:
        sys.stdout.write(payload)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
