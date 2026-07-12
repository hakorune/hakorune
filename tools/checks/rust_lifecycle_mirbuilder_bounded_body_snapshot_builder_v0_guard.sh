#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-bounded-body-snapshot-builder-v0"
source "$ROOT/tools/checks/lib/guard_common.sh"

BUILDER="$ROOT/src/analysis/bounded_body_snapshot_v0/snapshot_builder.rs"
SNAPSHOT="$ROOT/src/analysis/bounded_body_snapshot_v0/snapshot.rs"
TESTS="$ROOT/src/analysis/bounded_body_snapshot_v0/tests.rs"

guard_require_command "$TAG" cargo
guard_require_command "$TAG" python3
guard_require_files "$TAG" "$BUILDER" "$SNAPSHOT" "$TESTS"

cargo test -q --lib analysis::bounded_body_snapshot_v0::tests

python3 - "$BUILDER" "$SNAPSHOT" "$TESTS" <<'PY'
import sys
from pathlib import Path

builder_path, snapshot_path, tests_path = map(Path, sys.argv[1:])
builder = builder_path.read_text(encoding="utf-8")
snapshot = snapshot_path.read_text(encoding="utf-8")
tests = tests_path.read_text(encoding="utf-8")
joined = builder + "\n" + snapshot

for needle in (
    "reserve_node",
    "seal_node",
    "pub fn finish(self)",
    "IncompleteDraft",
    "AlreadySealed",
    "validate_atoms",
    "validate_child_roles",
    "validate_edges_and_paths",
    "validate_preorder_and_depth",
    "target.0 <= parent_index",
    "SnapshotNodeV0::from_verified_parts",
):
    if needle not in joined:
        raise SystemExit(f"missing builder invariant: {needle}")

for needle in (
    "publishes_only_complete_preorder_tables",
    "rejects_incomplete_and_double_sealed_drafts",
    "rejects_atom_and_child_schema_drift",
    "rejects_bad_targets_paths_preorder_and_depth",
):
    if needle not in tests:
        raise SystemExit(f"missing builder negative test: {needle}")

for forbidden in (
    "crate::mir",
    "crate::runner",
    "crate::stage1",
    "MIRBuilder",
    "planner",
    "route",
    "runtime",
):
    if forbidden in joined:
        raise SystemExit(f"forbidden builder dependency: {forbidden}")

for path in (builder_path, snapshot_path, tests_path):
    if len(path.read_text(encoding="utf-8").splitlines()) > 800:
        raise SystemExit(f"source exceeds 800 lines: {path}")

print("output_contract=BoundedBodySnapshotBuilderV0")
print("one_shot_finish=1")
print("all_drafts_sealed=1")
print("canonical_atom_child_schema=1")
print("forward_edges_and_paths=1")
print("flat_preorder_and_depth=1")
print("mutable_storage_shared=0")
print("partial_snapshot_publication=0")
print("summary=ok")
PY
