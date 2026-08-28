#!/usr/bin/env python3
"""Readiness/contract-close guard for the finite B1 Global-target matrix.

This guard deliberately checks topology and documentation, not call meaning.
It is the single C0 readiness surface: source paths are derived from the
working tree, while owner/action/terminal/reopen fields remain explicit in the
small manifest.  A passing readiness phase never grants B1 implementation.
"""

from __future__ import annotations

import argparse
import hashlib
from pathlib import Path
import subprocess
import sys
import tomllib
from typing import Any


TAG = "mir-call-global-target-b1-c0-matrix"
DUAL_SURFACE_TOKENS = ("Callee::Global", "CallTarget::Global")
ALLOWED_PHASES = {"c0_dual_readiness", "c0_dual_closeout"}
ALLOWED_DISPOSITION_ACTIONS = {"adapt", "isolate", "retire", "retain"}
ALLOWED_DISPOSITION_STATES = {
    "CutoverBlockerOpen",
    "SuccessorContractSealed",
    "CutoverBlockerClosed",
    "ParkedSealed",
}
REQUIRED_REGISTRY_ROWS = {
    "mir-call-global-target-b0-machine-census",
    "mir-call-global-target-b1-static-method-s0",
}
REQUIRED_EVIDENCE_FIELDS = {
    "id",
    "path",
    "anchor",
    "disposition_ref",
    "owner",
    "action",
    "terminal",
    "reopen",
}
REQUIRED_DUAL_AGGREGATE_FIELDS = {
    "id",
    "token",
    "expected_files",
    "expected_occurrences",
    "site_digest",
}
REQUIRED_BUCKET_FIELDS = {
    "id",
    "token_families",
    "scope",
    "role",
    "selector",
    "disposition_ref",
    "expected_files",
    "expected_occurrences",
}
REQUIRED_DISPOSITION_FIELDS = {
    "id",
    "owner",
    "action",
    "state",
    "reopen",
    "non_authority",
}

BUCKET_SELECTORS = {
    "test-fixture",
    "compiled-builder-calls",
    "compiled-builder-adjacent",
    "compiled-mir-core",
    "compiled-backend",
    "compiled-runner",
    "compiled-crates",
    "compiled-other",
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


def is_test_path(relative: Path) -> bool:
    value = relative.as_posix()
    name = relative.name
    return (
        "/tests/" in value
        or name == "tests.rs"
        or name.endswith("_tests.rs")
        or "/test_" in value
    )


def bucket_selector(relative: Path) -> str:
    value = relative.as_posix()
    if is_test_path(relative):
        return "test-fixture"
    if value.startswith("src/mir/builder/calls/"):
        return "compiled-builder-calls"
    if value.startswith("src/mir/builder/"):
        return "compiled-builder-adjacent"
    if value.startswith("src/mir/"):
        return "compiled-mir-core"
    if value.startswith("src/backend/"):
        return "compiled-backend"
    if value.startswith("src/runner/"):
        return "compiled-runner"
    if value.startswith("crates/"):
        return "compiled-crates"
    return "compiled-other"


def site_digest(rows: list[tuple[str, int]]) -> str:
    payload = "".join(f"{path}\0{count}\n" for path, count in rows)
    return hashlib.sha256(payload.encode("utf-8")).hexdigest()


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
    if not isinstance(c0, dict) or c0.get("status") not in {
        "design_stop",
        "contract_sealed",
    }:
        fail("active C0 card must be design_stop or contract_sealed")
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
        if c0.get("status") != "contract_sealed":
            fail("fast state requires the C0 contract to be sealed")
        if guard_status != "landed_readiness":
            fail("fast B1 state requires the landed C0 contract guard")
        if state.get("current_execution_row") == "MIR-CALL-GLOBAL-TARGET-B1-CUTOVER":
            b1 = card.get("b1_cutover")
            if not isinstance(b1, dict):
                fail("fast B1 state requires the active [b1_cutover] section")
            if b1.get("task_id") != "MIR-CALL-GLOBAL-TARGET-B1-CUTOVER":
                fail("active B1 cutover task id drifted")
            if b1.get("status") != "fast_open":
                fail("active B1 cutover must be status=fast_open")
            if b1.get("implementation_permission") is not True:
                fail("active B1 cutover must explicitly permit implementation")
            if state.get("next_execution_card") != b1["task_id"]:
                fail("CURRENT_STATE next_execution_card does not select B1 cutover")
        else:
            scoped_path = nonempty(
                manifest.get("scoped_fast_card_path"), "scoped_fast_card_path"
            )
            scoped_key = nonempty(
                manifest.get("scoped_fast_row_key"), "scoped_fast_row_key"
            )
            scoped_task_id = nonempty(
                manifest.get("scoped_fast_task_id"), "scoped_fast_task_id"
            )
            scoped_card = load_toml(root / scoped_path)
            scoped_row = scoped_card.get(scoped_key)
            if not isinstance(scoped_row, dict):
                fail(f"scoped fast card is missing [{scoped_key}]")
            if scoped_row.get("task_id") != scoped_task_id:
                fail("scoped fast task id drifted")
            if scoped_row.get("status") != "fast_open":
                fail("scoped fast row must be status=fast_open")
            if scoped_row.get("implementation_permission") is not True:
                fail("scoped fast row must explicitly permit implementation")
            if state.get("current_execution_row") != scoped_task_id:
                fail("CURRENT_STATE current_execution_row does not select scoped fast row")
            if state.get("next_execution_card") != scoped_task_id:
                fail("CURRENT_STATE next_execution_card does not select scoped fast row")
    elif work_mode == "design_stop":
        if c0.get("status") not in {"design_stop", "contract_sealed"}:
            fail("design_stop state requires an open or sealed C0 contract")
        if guard_status != "landed_readiness":
            fail("design_stop state requires a landed guard-only child")
        if guard.get("implementation_permission") is not False:
            fail("landed guard-only child must close its implementation permission")
        if c0.get("status") == "design_stop" and state.get("current_execution_row") != c0.get("task_id"):
            fail("CURRENT_STATE design stop must return to the C0 row")
        if state.get("next_execution_card") != "none":
            fail("design stop must not retain an execution card")
    else:
        fail("CURRENT_STATE work_mode must be fast or design_stop for this guard")


def check_dual_surface(
    root: Path, manifest: dict[str, Any]
) -> tuple[int, int, int, dict[str, dict[str, Any]]]:
    raw_aggregates = manifest.get("token_aggregates")
    if not isinstance(raw_aggregates, list) or not raw_aggregates:
        fail("token_aggregates must be a non-empty array")
    aggregates: dict[str, tuple[int, int, str]] = {}
    if {row.get("token") for row in raw_aggregates if isinstance(row, dict)} != set(
        DUAL_SURFACE_TOKENS
    ):
        fail("token_aggregates must cover exactly Callee::Global and CallTarget::Global")

    paths = tracked_rust_paths(root)
    sites_by_token: dict[str, list[tuple[str, int]]] = {}
    for index, raw in enumerate(raw_aggregates, start=1):
        if not isinstance(raw, dict):
            fail(f"token_aggregates[{index}] must be a table")
        missing = REQUIRED_DUAL_AGGREGATE_FIELDS - set(raw)
        if missing:
            fail(f"token_aggregates[{index}] missing fields: {', '.join(sorted(missing))}")
        token = nonempty(raw.get("token"), f"token_aggregates[{index}].token")
        if token not in DUAL_SURFACE_TOKENS:
            fail(f"token_aggregates[{index}] has unknown token: {token}")
        expected_files = raw.get("expected_files")
        expected_occurrences = raw.get("expected_occurrences")
        if not isinstance(expected_files, int) or expected_files < 1:
            fail(f"token_aggregates[{index}].expected_files must be positive")
        if not isinstance(expected_occurrences, int) or expected_occurrences < 1:
            fail(f"token_aggregates[{index}].expected_occurrences must be positive")
        digest = nonempty(raw.get("site_digest"), f"token_aggregates[{index}].site_digest")
        if token in aggregates:
            fail(f"duplicate token aggregate: {token}")
        rows: list[tuple[str, int]] = []
        for path in paths:
            count = read_surface(path, token)
            if count:
                rows.append((str(path.relative_to(root)), count))
        rows.sort()
        actual_files = len(rows)
        actual_occurrences = sum(count for _, count in rows)
        if actual_files != expected_files:
            fail(
                f"{token} expected {expected_files} files, observed {actual_files}"
            )
        if actual_occurrences != expected_occurrences:
            fail(
                f"{token} expected {expected_occurrences} occurrences, observed {actual_occurrences}"
            )
        actual_digest = site_digest(rows)
        if actual_digest != digest:
            fail(
                f"{token} site digest drift: expected {digest}, observed {actual_digest}"
            )
        aggregates[token] = (actual_files, actual_occurrences, actual_digest)
        sites_by_token[token] = rows

    raw_buckets = manifest.get("coverage_buckets")
    if not isinstance(raw_buckets, list) or not raw_buckets:
        fail("coverage_buckets must be a non-empty array")
    buckets: dict[str, dict[str, Any]] = {}
    for index, raw in enumerate(raw_buckets, start=1):
        if not isinstance(raw, dict):
            fail(f"coverage_buckets[{index}] must be a table")
        missing = REQUIRED_BUCKET_FIELDS - set(raw)
        if missing:
            fail(f"coverage_buckets[{index}] missing fields: {', '.join(sorted(missing))}")
        row_id = nonempty(raw.get("id"), f"coverage_buckets[{index}].id")
        selector = nonempty(raw.get("selector"), f"coverage_buckets[{index}].selector")
        if row_id in buckets:
            fail(f"duplicate coverage bucket: {row_id}")
        if selector not in BUCKET_SELECTORS:
            fail(f"unknown coverage selector: {selector}")
        families = raw.get("token_families")
        if not isinstance(families, list) or set(families) != set(DUAL_SURFACE_TOKENS):
            fail(f"coverage_buckets[{index}].token_families must cover both Global tokens")
        scope = nonempty(raw.get("scope"), f"coverage_buckets[{index}].scope")
        if scope not in {"compiled", "test-only"}:
            fail(f"coverage_buckets[{index}].scope is not closed: {scope}")
        nonempty(raw.get("role"), f"coverage_buckets[{index}].role")
        disposition = nonempty(
            raw.get("disposition_ref"), f"coverage_buckets[{index}].disposition_ref"
        )
        expected_files = raw.get("expected_files")
        expected_occurrences = raw.get("expected_occurrences")
        if not isinstance(expected_files, int) or expected_files < 0:
            fail(f"coverage_buckets[{index}].expected_files must be non-negative")
        if not isinstance(expected_occurrences, int) or expected_occurrences < 0:
            fail(f"coverage_buckets[{index}].expected_occurrences must be non-negative")
        buckets[row_id] = raw

    raw_dispositions = manifest.get("dispositions")
    if not isinstance(raw_dispositions, list) or not raw_dispositions:
        fail("dispositions must be a non-empty array")
    dispositions: dict[str, dict[str, Any]] = {}
    for index, raw in enumerate(raw_dispositions, start=1):
        if not isinstance(raw, dict):
            fail(f"dispositions[{index}] must be a table")
        missing = REQUIRED_DISPOSITION_FIELDS - set(raw)
        if missing:
            fail(f"dispositions[{index}] missing fields: {', '.join(sorted(missing))}")
        row_id = nonempty(raw.get("id"), f"dispositions[{index}].id")
        if row_id in dispositions:
            fail(f"duplicate disposition: {row_id}")
        action = nonempty(raw.get("action"), f"dispositions[{index}].action")
        if action not in ALLOWED_DISPOSITION_ACTIONS:
            fail(f"dispositions[{index}].action is not closed: {action}")
        state = nonempty(raw.get("state"), f"dispositions[{index}].state")
        if state not in ALLOWED_DISPOSITION_STATES:
            fail(f"dispositions[{index}].state is not closed: {state}")
        for field in ("owner", "reopen", "non_authority"):
            nonempty(raw.get(field), f"dispositions[{index}].{field}")
        terminal_kind = raw.get("terminal_kind")
        successor_ref = raw.get("successor_ref")
        terminal_present = isinstance(terminal_kind, str) and bool(terminal_kind.strip())
        successor_present = isinstance(successor_ref, str) and bool(successor_ref.strip())
        if terminal_present == successor_present:
            fail(
                f"dispositions[{index}] must provide exactly one non-empty "
                "terminal_kind or successor_ref"
            )
        if state == "ParkedSealed" and not terminal_present:
            fail(f"dispositions[{index}] ParkedSealed requires terminal_kind")
        if state == "SuccessorContractSealed" and not successor_present:
            fail(
                f"dispositions[{index}] SuccessorContractSealed requires successor_ref"
            )
        if state == "SuccessorContractSealed":
            nonempty(
                raw.get("source_relation_ref"),
                f"dispositions[{index}].source_relation_ref",
            )
        if state == "CutoverBlockerClosed" and not terminal_present:
            fail(f"dispositions[{index}] closed blocker requires terminal_kind")
        dispositions[row_id] = raw
    for bucket_id, raw in buckets.items():
        ref = raw["disposition_ref"]
        if ref not in dispositions:
            fail(f"coverage bucket {bucket_id} references unknown disposition: {ref}")

    bucket_counts: dict[str, tuple[set[str], int]] = {
        bucket_id: (set(), 0) for bucket_id in buckets
    }
    total_sites = 0
    for token in DUAL_SURFACE_TOKENS:
        for relative, count in sites_by_token[token]:
            selector = bucket_selector(Path(relative))
            matches = [
                bucket_id
                for bucket_id, raw in buckets.items()
                if raw["selector"] == selector and token in raw["token_families"]
            ]
            if len(matches) != 1:
                fail(
                    f"{token} occurrence path {relative} maps to {len(matches)} coverage buckets"
                )
            bucket_id = matches[0]
            files, occurrences = bucket_counts[bucket_id]
            files.add(relative)
            bucket_counts[bucket_id] = (files, occurrences + count)
            total_sites += count

    for bucket_id, raw in buckets.items():
        files, occurrences = bucket_counts[bucket_id]
        expected_files = raw["expected_files"]
        expected_occurrences = raw["expected_occurrences"]
        if len(files) != expected_files:
            fail(
                f"coverage bucket {bucket_id} expected {expected_files} files, observed {len(files)}"
            )
        if occurrences != expected_occurrences:
            fail(
                f"coverage bucket {bucket_id} expected {expected_occurrences} occurrences, observed {occurrences}"
            )

    return len(aggregates), total_sites, len(buckets), dispositions


def check_evidence_rows(
    root: Path, manifest: dict[str, Any], dispositions: dict[str, dict[str, Any]]
) -> int:
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
        disposition_ref = nonempty(
            raw.get("disposition_ref"), f"evidence_rows[{index}].disposition_ref"
        )
        if disposition_ref not in dispositions:
            fail(
                f"evidence_rows[{index}] references unknown disposition: "
                f"{disposition_ref}"
            )
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


def check_contract_close_consistency(
    manifest: dict[str, Any], dispositions: dict[str, dict[str, Any]]
) -> None:
    """Validate evidence/disposition agreement only at the contract-close phase.

    Readiness intentionally permits open rows and descriptive evidence while the
    finite owner map is being assembled.  Contract close is stricter: the C0
    state is a sealed successor contract, not proof that the successor code has
    already landed.  Executable proof belongs to the B1 implementation guard.
    """

    raw_rows = manifest.get("evidence_rows")
    if not isinstance(raw_rows, list):
        fail("evidence_rows must be an array for contract-close consistency")
    evidence_by_disposition: dict[str, int] = {
        disposition_id: 0 for disposition_id in dispositions
    }
    for index, raw in enumerate(raw_rows, start=1):
        if not isinstance(raw, dict):
            fail(f"evidence_rows[{index}] must be a table")
        disposition_ref = nonempty(
            raw.get("disposition_ref"), f"evidence_rows[{index}].disposition_ref"
        )
        disposition = dispositions.get(disposition_ref)
        if disposition is None:
            fail(
                f"evidence_rows[{index}] references unknown disposition: "
                f"{disposition_ref}"
            )
        evidence_by_disposition[disposition_ref] += 1
        evidence_action = nonempty(
            raw.get("action"), f"evidence_rows[{index}].action"
        )
        disposition_action = nonempty(
            disposition.get("action"),
            f"disposition {disposition_ref}.action",
        )
        if evidence_action != disposition_action:
            fail(
                f"evidence_rows[{index}] action {evidence_action!r} disagrees with "
                f"disposition {disposition_ref} action {disposition_action!r}"
            )
        disposition_state = nonempty(
            disposition.get("state"), f"disposition {disposition_ref}.state"
        )
        disposition_successor = disposition.get("successor_ref")
        if disposition_state == "SuccessorContractSealed":
            if not isinstance(disposition_successor, str) or not disposition_successor.strip():
                fail(
                    f"disposition {disposition_ref} successor contract lacks successor_ref"
                )
            evidence_successor = nonempty(
                raw.get("successor_ref"),
                f"evidence_rows[{index}].successor_ref",
            )
            if evidence_successor != disposition_successor:
                fail(
                    f"evidence_rows[{index}] successor_ref does not match "
                    f"disposition {disposition_ref}"
                )
        elif disposition_state == "ParkedSealed":
            nonempty(raw.get("terminal"), f"evidence_rows[{index}].terminal")
        else:
            fail(
                f"contract-close disposition {disposition_ref} has non-sealed state: "
                f"{disposition_state}"
            )
    missing = sorted(
        disposition_id
        for disposition_id, count in evidence_by_disposition.items()
        if count == 0
    )
    if missing:
        fail(
            "contract-close dispositions without evidence: "
            + ", ".join(missing)
        )


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
    schema_version = manifest.get("schema_version")
    if schema_version != 1:
        fail("manifest schema_version must be 1")
    phase = manifest.get("phase")
    if phase not in ALLOWED_PHASES:
        fail(f"manifest phase is not closed: {phase}")
    source_roots = manifest.get("source_roots")
    if source_roots != ["src", "crates"]:
        fail("source_roots must remain [src, crates]")

    check_card_and_state(root, manifest)
    check_registry_dependencies(root, set(manifest.get("required_registry_rows", [])))
    aggregates, occurrences, buckets, dispositions = check_dual_surface(root, manifest)
    surface_summary = (
        f"{aggregates} dual token aggregates / {occurrences} Global occurrences / "
        f"{buckets} coverage buckets"
    )
    evidence = check_evidence_rows(root, manifest, dispositions)
    if phase == "c0_dual_closeout":
        check_contract_close_consistency(manifest, dispositions)
        open_rows = [
            row_id
            for row_id, row in dispositions.items()
            if row.get("state") == "CutoverBlockerOpen"
        ]
        if open_rows:
            fail(
                "c0_dual_closeout has open dispositions: "
                + ", ".join(sorted(open_rows))
            )
        non_contract_rows = [
            row_id
            for row_id, row in dispositions.items()
            if row.get("state") not in {"SuccessorContractSealed", "ParkedSealed"}
        ]
        if non_contract_rows:
            fail(
                "c0_dual_closeout requires sealed successor/parked states, found: "
                + ", ".join(sorted(non_contract_rows))
            )
        print(
            f"[{TAG}] contract-close phase: {surface_summary} / {evidence} "
            "evidence rows; C0 contract evidence remains valid, executable B1 proof is checked by the B1 cutover guard"
        )
    else:
        print(
            f"[{TAG}] readiness phase: {surface_summary} / {evidence} "
            "additional evidence rows; C0 remains design-stopped"
        )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
