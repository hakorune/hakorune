#!/usr/bin/env python3
"""Readiness guard for the finite B1 Global-target disposition matrix.

This guard deliberately checks topology and documentation, not call meaning.
It is the single C0 readiness surface: source paths are derived from the
working tree, while owner/action/terminal/reopen fields remain explicit in the
small manifest.  A passing readiness phase never grants B1 implementation.
"""

from __future__ import annotations

import argparse
from pathlib import Path
import subprocess
import sys
import tomllib
from typing import Any


TAG = "mir-call-global-target-b1-c0-matrix"
SURFACE_TOKEN = "CallTarget::Global"
REQUIRED_REGISTRY_ROWS = {
    "mir-call-global-target-b0-machine-census",
    "mir-call-global-target-b1-static-method-s0",
}
REQUIRED_SURFACE_FIELDS = {
    "id",
    "path",
    "expected_occurrences",
    "scope",
    "owner",
    "action",
    "terminal",
    "successor",
    "reopen",
}
REQUIRED_EVIDENCE_FIELDS = {
    "id",
    "path",
    "anchor",
    "owner",
    "action",
    "terminal",
    "reopen",
}


def fail(message: str) -> None:
    print(f"[{TAG}] ERROR: {message}", file=sys.stderr)
    raise SystemExit(1)


def nonempty(value: Any, label: str) -> str:
    if not isinstance(value, str) or not value.strip():
        fail(f"{label} must be a non-empty string")
    return value


def load_toml(path: Path) -> dict[str, Any]:
    try:
        with path.open("rb") as stream:
            data = tomllib.load(stream)
    except tomllib.TOMLDecodeError as exc:
        fail(f"TOML parse failed: {path}: {exc}")
    if not isinstance(data, dict):
        fail(f"TOML root must be a table: {path}")
    return data


def tracked_rust_paths(root: Path) -> list[Path]:
    result = subprocess.run(
        ["git", "ls-files", "-z", "--", "src", "crates"],
        cwd=root,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=False,
    )
    if result.returncode != 0:
        detail = result.stderr.decode(errors="replace").strip()
        fail(f"git ls-files failed: {detail or result.returncode}")
    paths: list[Path] = []
    for raw in result.stdout.split(b"\0"):
        if not raw:
            continue
        relative = Path(raw.decode())
        if relative.suffix == ".rs":
            paths.append(root / relative)
    return sorted(paths)


def read_surface(path: Path, token: str) -> int:
    try:
        return path.read_text(encoding="utf-8").count(token)
    except UnicodeDecodeError as exc:
        fail(f"source is not UTF-8: {path}: {exc}")


def load_registry_rows(root: Path, path: Path, stack: tuple[Path, ...] = ()) -> list[dict[str, Any]]:
    if path in stack:
        fail("guard registry include cycle: " + " -> ".join(str(item) for item in (*stack, path)))
    data = load_toml(path)
    rows: list[dict[str, Any]] = []
    includes = data.get("includes", [])
    if not isinstance(includes, list) or not all(isinstance(item, str) and item for item in includes):
        fail(f"registry includes must be non-empty strings: {path}")
    for include in includes:
        include_path = root / include
        if not include_path.is_file():
            fail(f"registry include missing: {include}")
        rows.extend(load_registry_rows(root, include_path, (*stack, path)))
    local_rows = data.get("rows", [])
    if not isinstance(local_rows, list):
        fail(f"registry rows must be an array: {path}")
    for index, row in enumerate(local_rows):
        if not isinstance(row, dict):
            fail(f"registry row {index} must be a table: {path}")
        rows.append(row)
    return rows


def check_registry_dependencies(root: Path, required: set[str]) -> None:
    rows = load_registry_rows(root, root / "tools/checks/guard_rows.toml")
    ids: list[str] = []
    for row in rows:
        value = row.get("id")
        if isinstance(value, str):
            ids.append(value)
    for row_id in sorted(required):
        count = ids.count(row_id)
        if count != 1:
            fail(f"required registry dependency {row_id!r} appears {count} times")


def check_card_and_state(root: Path, manifest: dict[str, Any]) -> None:
    card_path = root / nonempty(manifest.get("card_path"), "card_path")
    card = load_toml(card_path)
    c0 = card.get("b1_current_head_c0")
    guard = card.get("b1_c0_guard_i0")
    if not isinstance(c0, dict) or c0.get("status") != "design_stop":
        fail("active C0 card must remain status=design_stop")
    if c0.get("implementation_permission") is not False:
        fail("active C0 implementation_permission must remain false")
    if not isinstance(guard, dict):
        fail("active card is missing [b1_c0_guard_i0]")
    if guard.get("task_id") != manifest.get("task_id"):
        fail("guard row task_id does not match active card")
    if guard.get("guard_phase") != manifest.get("phase"):
        fail("guard phase does not match active card")
    guard_status = guard.get("status")
    if guard_status not in {"fast_guard_only", "landed_readiness"}:
        fail("guard-only child has an unknown status")
    if not isinstance(card.get("observed_commit"), str) or not card["observed_commit"]:
        fail("active card must retain a current observed_commit")

    state = load_toml(root / "docs/development/current/main/CURRENT_STATE.toml")
    task_id = nonempty(manifest.get("task_id"), "task_id")
    work_mode = state.get("work_mode")
    if work_mode == "fast":
        if guard_status != "fast_guard_only":
            fail("fast state requires the guard-only child to be active")
        if guard.get("implementation_permission") is not True:
            fail("active guard-only child must explicitly permit the guard")
        if state.get("current_execution_row") != task_id:
            fail("CURRENT_STATE current_execution_row does not select the guard-only row")
        if state.get("next_execution_card") != task_id:
            fail("CURRENT_STATE next_execution_card does not select the guard-only row")
        if "guard-only" not in nonempty(state.get("current_blocker_token"), "current_blocker_token"):
            fail("CURRENT_STATE blocker must keep the guard-only stop line")
    elif work_mode == "design_stop":
        if guard_status != "landed_readiness":
            fail("design_stop state requires a landed guard-only child")
        if guard.get("implementation_permission") is not False:
            fail("landed guard-only child must close its implementation permission")
        if state.get("current_execution_row") != c0.get("task_id"):
            fail("CURRENT_STATE design stop must return to the C0 row")
        if state.get("next_execution_card") != "none":
            fail("C0 design stop must not retain an execution card")
    else:
        fail("CURRENT_STATE work_mode must be fast or design_stop for this guard")


def check_surface_rows(root: Path, manifest: dict[str, Any]) -> tuple[int, int]:
    token = nonempty(manifest.get("surface_token"), "surface_token")
    if token != SURFACE_TOKEN:
        fail(f"surface_token must remain {SURFACE_TOKEN!r}")

    raw_rows = manifest.get("surface_rows")
    if not isinstance(raw_rows, list) or not raw_rows:
        fail("surface_rows must be a non-empty array")

    listed: dict[str, tuple[Path, int]] = {}
    listed_ids: set[str] = set()
    for index, raw in enumerate(raw_rows, start=1):
        if not isinstance(raw, dict):
            fail(f"surface_rows[{index}] must be a table")
        missing = REQUIRED_SURFACE_FIELDS - set(raw)
        if missing:
            fail(f"surface_rows[{index}] missing fields: {', '.join(sorted(missing))}")
        row_id = nonempty(raw.get("id"), f"surface_rows[{index}].id")
        relative = Path(nonempty(raw.get("path"), f"surface_rows[{index}].path"))
        if relative.is_absolute() or ".." in relative.parts:
            fail(f"surface_rows[{index}] path escapes repository: {relative}")
        if relative.as_posix() in listed:
            fail(f"duplicate surface path: {relative}")
        expected = raw.get("expected_occurrences")
        if not isinstance(expected, int) or expected < 1:
            fail(f"surface_rows[{index}] expected_occurrences must be positive")
        scope = nonempty(raw.get("scope"), f"surface_rows[{index}].scope")
        if scope not in {"compiled", "test-only"}:
            fail(f"surface_rows[{index}] scope is not closed: {scope}")
        for field in ("owner", "action", "terminal", "successor", "reopen"):
            nonempty(raw.get(field), f"surface_rows[{index}].{field}")
        if not row_id:
            fail(f"surface_rows[{index}] id is empty")
        if row_id in listed_ids:
            fail(f"duplicate surface id: {row_id}")
        listed_ids.add(row_id)
        listed[relative.as_posix()] = (relative, expected)

    actual: dict[str, int] = {}
    for path in tracked_rust_paths(root):
        count = read_surface(path, token)
        if count:
            actual[str(path.relative_to(root))] = count

    listed_paths = set(listed)
    actual_paths = set(actual)
    missing = sorted(actual_paths - listed_paths)
    stale = sorted(listed_paths - actual_paths)
    if missing:
        fail("compiled surface paths missing from manifest: " + ", ".join(missing))
    if stale:
        fail("manifest surface paths have no current token: " + ", ".join(stale))
    for relative, (_, expected) in listed.items():
        observed = actual[relative]
        if observed != expected:
            fail(f"{relative} expected {expected} {token} occurrences, observed {observed}")
    return len(actual), sum(actual.values())


def check_evidence_rows(root: Path, manifest: dict[str, Any]) -> int:
    raw_rows = manifest.get("evidence_rows")
    if not isinstance(raw_rows, list) or not raw_rows:
        fail("evidence_rows must be a non-empty array")
    seen: set[str] = set()
    for index, raw in enumerate(raw_rows, start=1):
        if not isinstance(raw, dict):
            fail(f"evidence_rows[{index}] must be a table")
        missing = REQUIRED_EVIDENCE_FIELDS - set(raw)
        if missing:
            fail(f"evidence_rows[{index}] missing fields: {', '.join(sorted(missing))}")
        row_id = nonempty(raw.get("id"), f"evidence_rows[{index}].id")
        if row_id in seen:
            fail(f"duplicate evidence id: {row_id}")
        seen.add(row_id)
        relative = Path(nonempty(raw.get("path"), f"evidence_rows[{index}].path"))
        if relative.is_absolute() or ".." in relative.parts:
            fail(f"evidence_rows[{index}] path escapes repository: {relative}")
        path = root / relative
        if not path.is_file():
            fail(f"evidence path missing: {relative}")
        anchor = nonempty(raw.get("anchor"), f"evidence_rows[{index}].anchor")
        if anchor not in path.read_text(encoding="utf-8"):
            fail(f"evidence anchor missing: {relative}: {anchor}")
        for field in ("owner", "action", "terminal", "reopen"):
            nonempty(raw.get(field), f"evidence_rows[{index}].{field}")
    return len(seen)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("root", nargs="?", default=".")
    parser.add_argument(
        "--manifest",
        default="tools/checks/manifests/mir_call_global_target_b1_c0_matrix.toml",
    )
    args = parser.parse_args()
    root = Path(args.root).resolve()
    manifest_path = root / args.manifest
    if not manifest_path.is_file():
        fail(f"manifest missing: {args.manifest}")
    manifest = load_toml(manifest_path)
    if manifest.get("schema_version") != 0:
        fail("manifest schema_version must be 0")
    if manifest.get("phase") != "c0_readiness":
        fail("manifest phase must remain c0_readiness")
    source_roots = manifest.get("source_roots")
    if source_roots != ["src", "crates"]:
        fail("source_roots must remain [src, crates]")

    check_card_and_state(root, manifest)
    check_registry_dependencies(root, set(manifest.get("required_registry_rows", [])))
    files, occurrences = check_surface_rows(root, manifest)
    evidence = check_evidence_rows(root, manifest)
    print(
        f"[{TAG}] readiness phase: {files} surface files / {occurrences} "
        f"{SURFACE_TOKEN} occurrences / {evidence} additional evidence rows; "
        "C0 remains design-stopped"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
