#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-bounded-body-snapshot-rust-witness-v0"
source "$ROOT/tools/checks/lib/guard_common.sh"

WITNESS="$ROOT/src/analysis/bounded_body_snapshot_v0/program_v0_snapshot_witness.rs"
FIXTURES="$ROOT/src/analysis/bounded_body_snapshot_v0/tests/witness.rs"

guard_require_command "$TAG" cargo
guard_require_command "$TAG" python3
guard_require_files "$TAG" "$WITNESS" "$FIXTURES"

cargo test -q --lib analysis::bounded_body_snapshot_v0::tests

python3 - "$WITNESS" "$FIXTURES" <<'PY'
import sys
from pathlib import Path

witness_path, fixtures_path = map(Path, sys.argv[1:])
witness = witness_path.read_text(encoding="utf-8")
fixtures = fixtures_path.read_text(encoding="utf-8")

for needle in (
    "pub(crate) fn build_snapshot_from_validated_view_v0",
    "SnapshotBuilderV0::new",
    "visit_node(",
    "node.children()",
    ".atoms()",
    "builder.reserve_node",
    "builder.seal_node",
    "builder.finish()",
):
    if needle not in witness:
        raise SystemExit(f"missing witness contract: {needle}")

for needle in (
    "witness_accepts_empty_body_without_nodes",
    "witness_makes_integer_wire_equivalence_exact",
    "witness_covers_every_accepted_kind_role_and_operator",
    "every_schema_limit_is_inclusive_at_minus_one_limit_and_plus_one",
    "BinaryOperatorV0::ALL",
    "CompareOperatorV0::ALL",
    "LogicalOperatorV0::ALL",
    "猫😸",
):
    if needle not in fixtures:
        raise SystemExit(f"missing witness fixture: {needle}")

for forbidden in (
    "serde_json",
    "crate::ast",
    "crate::mir",
    "crate::runner",
    "crate::stage1",
    "serializer",
    "MIRBuilder",
    "planner",
    "route",
    "runtime",
):
    if forbidden in witness:
        raise SystemExit(f"forbidden witness dependency: {forbidden}")

for path in (witness_path, fixtures_path):
    if len(path.read_text(encoding="utf-8").splitlines()) > 800:
        raise SystemExit(f"source exceeds 800 lines: {path}")

print("output_contract=BoundedBodySnapshotRustWitnessV0")
print("verification_only_entry=1")
print("validated_view_input=1")
print("flat_preorder_builder_only=1")
print("all_kinds_roles_operators=1")
print("integer_wire_equivalence=1")
print("multibyte_text=1")
print("all_limit_boundaries=1")
print("producer_serializer_dependency=0")
print("summary=ok")
PY
