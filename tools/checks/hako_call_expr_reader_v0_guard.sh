#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="hako-call-expr-reader-v0"
DIR="$ROOT/lang/src/compiler/analysis/bounded_body_snapshot"
READER="$DIR/reader_expr_child_v0.hako"
PUBLISHER="$DIR/flat_publisher_v0.hako"
FIXTURE="$ROOT/tools/checks/fixtures/bounded_body_snapshot_call_expr_reader_v0.hako"
SESSION="$ROOT/src/backend/mir_interpreter/strict_json_session.rs"
TESTS="$ROOT/src/backend/mir_interpreter/strict_json_session/tests/tests_call_expr.rs"

cd "$ROOT"
cargo build -q --release --features vm-reference --bin hakorune
for mode in unset 1; do
  if [ "$mode" = unset ]; then
    env -u NYASH_STR_CP cargo test -q --features vm-reference --lib \
      backend::mir_interpreter::strict_json_session::tests::tests_call_expr
  else
    NYASH_STR_CP=1 cargo test -q --features vm-reference --lib \
      backend::mir_interpreter::strict_json_session::tests::tests_call_expr
  fi
done
TIMING="$(mktemp /tmp/hako-call-expr-reader-v0.XXXXXX.log)"
MIR="$(mktemp /tmp/hako-call-expr-reader-v0.XXXXXX.mir)"
trap 'rm -f "$TIMING" "$MIR"' EXIT
timeout 10s env NYASH_DISABLE_PLUGINS=1 NYASH_MIR_COMPILE_TRACE=1 \
  target/release/hakorune --dump-mir --no-optimize "$FIXTURE" \
  >"$MIR" 2>"$TIMING"
grep -q 'stage=build_module' "$TIMING"
grep -q 'stage=semantic_refresh' "$TIMING"
grep -q 'call_method ChildExprEdgeV0.birth' "$MIR"
grep -q 'call_method BoundedBodySnapshotChildV0.birth' "$MIR"

python3 - "$READER" "$PUBLISHER" "$FIXTURE" "$SESSION" "$TESTS" <<'PY'
import sys
from pathlib import Path

reader_path, publisher_path, fixture_path, session_path, tests_path = map(Path, sys.argv[1:])
reader = reader_path.read_text(encoding="utf-8")
publisher = publisher_path.read_text(encoding="utf-8")
fixture = fixture_path.read_text(encoding="utf-8")
tests = tests_path.read_text(encoding="utf-8")
for path in (reader_path, publisher_path, fixture_path, session_path, tests_path):
    if len(path.read_text(encoding="utf-8").splitlines()) > 800:
        raise SystemExit(f"source exceeds 800 lines: {path}")
for needle in (
    'kind == "Call"', 'kind == "Method"', 'kind == "Field"',
    'ChildExprEdgeV0Box.make("recv"', 'ChildExprEdgeV0Box.make("args"',
    "budget.observe_arguments(args_count)", "ValidatedTextV0Box.atom",
    'path + ".args["', "reader.accepted_expr_unowned",
):
    if needle not in reader:
        raise SystemExit(f"missing call-reader contract: {needle}")
for forbidden in ("MapBox", "indexOf", "substring(", "ValidatedProgramV0BodyView", "MIRBuilder"):
    if forbidden in reader + publisher:
        raise SystemExit(f"forbidden call-reader dependency: {forbidden}")
for needle in (
    "FlatNodeDraftV0", "FlatNodeRecordV0Box", "_reserve_node", "_reconstruct",
    "nodes.push", "draft.seal", 'role == "args"', "target_index()",
):
    if needle not in publisher:
        raise SystemExit(f"missing flat-publication contract: {needle}")
for forbidden in ("nodes.set(",):
    if forbidden in publisher:
        raise SystemExit(f"loop construction bypasses record factory: {forbidden}")
for constructor in ("new BoundedBodySnapshotAtomV0", "new BoundedBodySnapshotChildV0", "new BoundedBodySnapshotNodeV0"):
    if publisher.count(constructor) != 1:
        raise SystemExit(f"snapshot record construction must have one factory entry: {constructor}")
for needle in (
    "hako_call_expr_reader_matches_reference_outcomes",
    "hako_call_expr_reader_flattens_mixed_recursion_in_schema_order",
    "hako_call_expr_reader_enforces_atom_and_argument_limits",
):
    if needle not in tests:
        raise SystemExit(f"missing executable call proof: {needle}")
for needle in ("flat_signature", "tree_signature", "atom_value"):
    if needle not in fixture:
        raise SystemExit(f"missing call fixture surface: {needle}")
print("accepted_call_kinds=3")
print("recursive_entrypoints=1")
print("method_child_order=recv_then_args")
print("flat_preorder_indices=green")
print("argument_limit_127_128_129=green")
print("rust_hako_outcome_parity=green")
print("partial_snapshot_publication=0")
print("summary=ok")
PY

echo "[$TAG] ok"
