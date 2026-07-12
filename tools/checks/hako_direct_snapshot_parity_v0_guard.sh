#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="hako-direct-snapshot-parity-v0"
DIR="$ROOT/lang/src/compiler/analysis/bounded_body_snapshot"
READER="$DIR/reader_v0.hako"
SEALER="$DIR/snapshot_sealer_v0.hako"
PUBLISHER="$DIR/flat_publisher_v0.hako"
FIXTURE="$ROOT/tools/checks/fixtures/bounded_body_snapshot_direct_reader_v0.hako"
SESSION="$ROOT/src/backend/mir_interpreter/strict_json_session.rs"
TESTS="$ROOT/src/backend/mir_interpreter/strict_json_session/tests/tests_snapshot.rs"

cd "$ROOT"
cargo build -q --release --features vm-reference --bin hakorune
for mode in unset 1; do
  if [ "$mode" = unset ]; then
    env -u NYASH_STR_CP cargo test -q --features vm-reference --lib \
      backend::mir_interpreter::strict_json_session::tests::tests_snapshot
  else
    NYASH_STR_CP=1 cargo test -q --features vm-reference --lib \
      backend::mir_interpreter::strict_json_session::tests::tests_snapshot
  fi
done
TIMING="$(mktemp /tmp/hako-direct-snapshot-v0.XXXXXX.log)"
MIR="$(mktemp /tmp/hako-direct-snapshot-v0.XXXXXX.mir)"
trap 'rm -f "$TIMING" "$MIR"' EXIT
timeout 10s env NYASH_DISABLE_PLUGINS=1 NYASH_MIR_COMPILE_TRACE=1 \
  target/release/hakorune --dump-mir --no-optimize "$FIXTURE" \
  >"$MIR" 2>"$TIMING"
grep -q 'stage=build_module' "$TIMING"
grep -q 'stage=semantic_refresh' "$TIMING"
grep -q 'call_method BoundedBodyAnalysisSnapshotV0.birth' "$MIR"
grep -q 'call_method BoundedBodySnapshotNodeV0.birth' "$MIR"

python3 - "$READER" "$SEALER" "$PUBLISHER" "$FIXTURE" "$SESSION" "$TESTS" <<'PY'
import sys
from pathlib import Path

reader_path, sealer_path, publisher_path, fixture_path, session_path, tests_path = map(Path, sys.argv[1:])
reader = reader_path.read_text(encoding="utf-8")
sealer = sealer_path.read_text(encoding="utf-8")
publisher = publisher_path.read_text(encoding="utf-8")
fixture = fixture_path.read_text(encoding="utf-8")
tests = tests_path.read_text(encoding="utf-8")
for path in (reader_path, sealer_path, publisher_path, fixture_path, session_path, tests_path):
    if len(path.read_text(encoding="utf-8").splitlines()) > 800:
        raise SystemExit(f"source exceeds 800 lines: {path}")
for needle in (
    "HakoProgramV0RootReaderV0Box.scan", "HakoProgramV0StmtReaderV0Box.read_body",
    "FlatNodePublisherV0Box.publish_body", "BoundedBodySnapshotSealerV0Box.seal",
    "BoundedBodySnapshotOutcomeV0Box.ready", "BoundedBodySnapshotOutcomeV0Box.internal",
):
    if needle not in reader:
        raise SystemExit(f"missing direct-reader contract: {needle}")
for needle in (
    "snapshot.node_budget_mismatch", "snapshot.child_target_invalid",
    "snapshot.root_shape_invalid", "SnapshotSealRecordV0Box", "max_depth_observed",
):
    if needle not in sealer:
        raise SystemExit(f"missing snapshot-seal contract: {needle}")
for forbidden in ("MapBox", "indexOf", "substring(", "ValidatedProgramV0BodyView", "MIRBuilder", "planner", "route"):
    if forbidden in reader + sealer + publisher:
        raise SystemExit(f"forbidden direct-reader dependency: {forbidden}")
for needle in (
    "hako_direct_snapshot_reader_matches_empty_and_full_rust_snapshots",
    "hako_direct_snapshot_reader_matches_reference_failure_outcomes",
    "hako_direct_snapshot_reader_never_publishes_failure_as_snapshot",
    "hako_direct_snapshot_reader_matches_all_operator_corpus",
):
    if needle not in tests:
        raise SystemExit(f"missing executable direct proof: {needle}")
for needle in ("snapshot_signature", "schema_version", "source_program_version", "max_depth_observed"):
    if needle not in fixture:
        raise SystemExit(f"missing exact snapshot fixture surface: {needle}")
print("direct_program_v0_entry=green")
print("accepted_kind_role_corpus=green")
print("all_18_operators=green")
print("exact_node_atom_edge_path_depth_parity=green")
print("three_way_failure_propagation=green")
print("internal_contract_domain=separate")
print("partial_snapshot_publication=0")
print("planner_connection=0")
print("summary=ok")
PY

echo "[$TAG] ok"
