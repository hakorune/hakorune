#!/usr/bin/env python3
"""Verify the behavior-neutral SSA-RC-RET-P0 ReleaseStrong inventory."""

from __future__ import annotations

from collections import Counter
import json
from pathlib import Path
import subprocess
import sys


SCHEMA = "LegacyReleaseStrongInventoryV1"
TOKENS = ("ReleaseStrong", "release_strong")
SURFACE_KINDS = {
    "producer",
    "consumer",
    "opcode_surface",
    "pass",
    "fixture",
    "document",
    "guard",
}
DISPOSITIONS = {
    "canonical_caller_zero_delete",
    "legacy_builder_isolate",
    "optional_rc_insertion_isolate",
    "optimizer_cfg_rewrite_isolate",
    "backend_json_compatibility_isolate",
    "dead_after_repository_caller_zero",
}
EXPECTED_EXCLUSIONS = {
    "docs/development/current/main/CURRENT_STATE.toml",
    "docs/development/current/main/investigations/"
    "mirbuilder-dprime-binding-ssa-final-form-task-2026-07-14.md",
    "docs/development/current/main/investigations/"
    "mirbuilder-ssa-rc-ret-p0-legacy-release-inventory-2026-07-14.md",
    "tools/checks/fixtures/legacy_release_strong_inventory_v1.json",
    "tools/checks/lib/resolved_ownership_legacy_release_contract.sh",
    "tools/checks/lib/resolved_ownership_legacy_release_inventory.py",
}
EXPECTED_SURFACE_COUNTS = {
    "producer": 5,
    "consumer": 24,
    "opcode_surface": 7,
    "pass": 14,
    "fixture": 23,
    "document": 48,
    "guard": 4,
}
EXPECTED_DISPOSITION_COUNTS = {
    "canonical_caller_zero_delete": 1,
    "legacy_builder_isolate": 4,
    "optional_rc_insertion_isolate": 3,
    "optimizer_cfg_rewrite_isolate": 11,
    "backend_json_compatibility_isolate": 31,
    "dead_after_repository_caller_zero": 75,
}
EXPECTED_CANONICAL = {"src/mir/builder/resolved_lowering/lowerer.rs"}
EXPECTED_LEGACY_BUILDERS = {
    "src/mir/array_getset_micro_seed_plan.rs",
    "src/mir/array_rmw_add1_leaf_seed_plan.rs",
    "src/mir/array_string_store_micro_seed_plan.rs",
    "src/mir/builder/builder_build.rs",
}
EXPECTED_OPTIONAL_RC = {
    "src/mir/passes/rc_insertion_helpers.rs",
    "src/mir/passes/rc_insertion_helpers/apply.rs",
    "src/mir/passes/rc_insertion_helpers/util.rs",
}
EXPECTED_OPTIMIZER = {
    "src/mir/builder/joinir_id_remapper.rs",
    "src/mir/builder/ssa/analysis.rs",
    "src/mir/builder/ssa/local/finalize.rs",
    "src/mir/global_call_route_plan/box_type_inspector_describe_body.rs",
    "src/mir/global_call_route_plan/generic_i64_body/refine.rs",
    "src/mir/global_call_route_plan/generic_string_body_analysis.rs",
    "src/mir/global_call_route_plan/mir_schema_map_constructor_body.rs",
    "src/mir/global_call_route_plan/program_json_emit_body.rs",
    "src/mir/passes/simplify_cfg/flow.rs",
    "src/mir/same_module_body_shape.rs",
    "src/mir/value_consumer.rs",
}


def fail(message: str) -> None:
    raise SystemExit(f"SSA-RC-RET-P0 legacy ReleaseStrong inventory: {message}")


def tracked_paths(root: Path) -> list[str]:
    result = subprocess.run(
        ["git", "ls-files", "-z"],
        cwd=root,
        check=True,
        capture_output=True,
    )
    return [raw.decode() for raw in result.stdout.split(b"\0") if raw]


def token_count(path: Path) -> int:
    text = path.read_text(errors="ignore")
    return sum(text.count(token) for token in TOKENS)


def disposition_paths(rows: list[dict], disposition: str) -> set[str]:
    return {row["path"] for row in rows if row["disposition"] == disposition}


def require_count(text: str, literal: str, expected: int, label: str) -> None:
    actual = text.count(literal)
    if actual != expected:
        fail(f"{label} drifted: expected={expected} actual={actual}")


def main() -> None:
    if len(sys.argv) != 3:
        fail("usage: resolved_ownership_legacy_release_inventory.py ROOT INVENTORY")

    root = Path(sys.argv[1]).resolve()
    inventory_path = Path(sys.argv[2]).resolve()
    data = json.loads(inventory_path.read_text())
    if set(data) != {
        "schema",
        "decision",
        "tokens",
        "surface_kinds",
        "dispositions",
        "scan_exclusions",
        "rows",
    }:
        fail("top-level schema drifted")
    if data["schema"] != SCHEMA:
        fail("schema name drifted")
    if data["decision"] != "inventory_and_isolate_without_semantic_change":
        fail("decision drifted")
    if tuple(data["tokens"]) != TOKENS:
        fail("scan token vocabulary drifted")
    if set(data["surface_kinds"]) != SURFACE_KINDS:
        fail("surface-kind vocabulary drifted")
    if set(data["dispositions"]) != DISPOSITIONS:
        fail("retirement disposition vocabulary drifted")
    exclusions = set(data["scan_exclusions"])
    if exclusions != EXPECTED_EXCLUSIONS:
        fail("inventory-control exclusions drifted")

    rows = data["rows"]
    paths = [row.get("path") for row in rows]
    if paths != sorted(paths) or len(paths) != len(set(paths)):
        fail("rows must have unique deterministic paths")
    tracked = set(tracked_paths(root))
    for row in rows:
        if set(row) != {"path", "occurrences", "surface_kind", "disposition"}:
            fail(f"row schema drifted: {row!r}")
        path = row["path"]
        if path not in tracked:
            fail(f"untracked or missing surface: {path}")
        if path in exclusions:
            fail(f"inventory-control path appeared as a legacy surface: {path}")
        if row["surface_kind"] not in SURFACE_KINDS:
            fail(f"unknown surface kind: {path}")
        if row["disposition"] not in DISPOSITIONS:
            fail(f"unknown disposition: {path}")
        actual = token_count(root / path)
        if actual != row["occurrences"] or actual <= 0:
            fail(
                f"occurrence drift: path={path} "
                f"expected={row['occurrences']} actual={actual}"
            )

    actual_paths = {
        path
        for path in tracked
        if path not in exclusions and token_count(root / path) > 0
    }
    expected_paths = set(paths)
    if actual_paths != expected_paths:
        fail(
            f"surface set drifted: missing={sorted(actual_paths - expected_paths)} "
            f"stale={sorted(expected_paths - actual_paths)}"
        )

    surface_counts = Counter(row["surface_kind"] for row in rows)
    disposition_counts = Counter(row["disposition"] for row in rows)
    if dict(surface_counts) != EXPECTED_SURFACE_COUNTS:
        fail(f"surface counts drifted: {dict(surface_counts)}")
    if dict(disposition_counts) != EXPECTED_DISPOSITION_COUNTS:
        fail(f"disposition counts drifted: {dict(disposition_counts)}")
    if disposition_paths(rows, "canonical_caller_zero_delete") != EXPECTED_CANONICAL:
        fail("canonical ReleaseStrong owner drifted")
    if disposition_paths(rows, "legacy_builder_isolate") != EXPECTED_LEGACY_BUILDERS:
        fail("legacy builder set drifted")
    if disposition_paths(rows, "optional_rc_insertion_isolate") != EXPECTED_OPTIONAL_RC:
        fail("optional RC insertion set drifted")
    if disposition_paths(rows, "optimizer_cfg_rewrite_isolate") != EXPECTED_OPTIMIZER:
        fail("optimizer/CFG preservation set drifted")

    lowerer = (root / next(iter(EXPECTED_CANONICAL))).read_text()
    builder = (root / "src/mir/builder/builder_build.rs").read_text()
    rc_apply = (root / "src/mir/passes/rc_insertion_helpers/apply.rs").read_text()
    instruction = (root / "src/mir/instruction.rs").read_text()
    parser = (root / "src/runner/mir_json_v0/lifecycle.rs").read_text()
    require_count(lowerer, "MirInstruction::ReleaseStrong", 1, "canonical caller")
    require_count(builder, "MirInstruction::ReleaseStrong", 1, "legacy builder caller")
    require_count(rc_apply, "MirInstruction::ReleaseStrong", 2, "optional RC producers")
    require_count(instruction, "ReleaseStrong { values: Vec<ValueId> }", 1, "opcode variant")
    require_count(parser, '"release_strong" =>', 1, "JSON parser arm")

    print(
        "SSA-RC-RET-P0 legacy ReleaseStrong inventory: "
        f"rows={len(rows)} occurrences={sum(row['occurrences'] for row in rows)} "
        "canonical=1 legacy-builders=4 optional-rc=3 optimizer-cfg=11 "
        "backend-json=31 retirement-only=75 semantic-delta=0"
    )


if __name__ == "__main__":
    main()
