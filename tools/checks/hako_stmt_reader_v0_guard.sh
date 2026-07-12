#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="hako-stmt-reader-v0"
DIR="$ROOT/lang/src/compiler/analysis/bounded_body_snapshot"
READER="$DIR/reader_stmt_v0.hako"
PUBLISHER="$DIR/flat_publisher_v0.hako"
FIXTURE="$ROOT/tools/checks/fixtures/bounded_body_snapshot_stmt_reader_v0.hako"
SESSION="$ROOT/src/backend/mir_interpreter/strict_json_session.rs"
TESTS="$ROOT/src/backend/mir_interpreter/strict_json_session/tests/tests_stmt.rs"

cd "$ROOT"
cargo build -q --release --features vm-reference --bin hakorune
for mode in unset 1; do
  if [ "$mode" = unset ]; then
    env -u NYASH_STR_CP cargo test -q --features vm-reference --lib \
      backend::mir_interpreter::strict_json_session::tests::tests_stmt
  else
    NYASH_STR_CP=1 cargo test -q --features vm-reference --lib \
      backend::mir_interpreter::strict_json_session::tests::tests_stmt
  fi
done
TIMING="$(mktemp /tmp/hako-stmt-reader-v0.XXXXXX.log)"
MIR="$(mktemp /tmp/hako-stmt-reader-v0.XXXXXX.mir)"
trap 'rm -f "$TIMING" "$MIR"' EXIT
timeout 10s env NYASH_DISABLE_PLUGINS=1 NYASH_MIR_COMPILE_TRACE=1 \
  target/release/hakorune --dump-mir --no-optimize "$FIXTURE" \
  >"$MIR" 2>"$TIMING"
grep -q 'stage=build_module' "$TIMING"
grep -q 'stage=semantic_refresh' "$TIMING"
grep -q 'call_method StmtObservationEdgeV0.birth' "$MIR"
grep -q 'call_method BoundedBodySnapshotNodeV0.birth' "$MIR"

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
for kind in ("Local", "Expr", "If", "Loop", "LoopRange", "Return", "Break", "Continue"):
    if f'kind == "{kind}"' not in reader:
        raise SystemExit(f"missing accepted statement kind: {kind}")
for needle in (
    "read_body", "budget.observe_body_children(count)", "budget.observe_node(depth)",
    "StmtObservationEdgeV0Box.make", "ValidatedTextV0Box.atom",
    'declared_kind != "String" && declared_kind != "Null"',
    'StrictJsonTreeV0Box.kind(session, else_node) != "Null"',
):
    if needle not in reader:
        raise SystemExit(f"missing statement-reader contract: {needle}")
for forbidden in ("MapBox", "indexOf", "substring(", "ValidatedProgramV0BodyView", "MIRBuilder"):
    if forbidden in reader + publisher:
        raise SystemExit(f"forbidden statement-reader dependency: {forbidden}")
for needle in (
    "publish_body", "root_count", 'role == "then"', 'role == "else"',
    'role == "body"', "observation.domain()", "draft.seal", "_reconstruct",
):
    if needle not in publisher:
        raise SystemExit(f"missing mixed flat-publication contract: {needle}")
for needle in (
    "hako_stmt_reader_covers_all_accepted_kinds_and_reference_outcomes",
    "hako_stmt_reader_flattens_roots_and_nested_body_roles_in_preorder",
    "hako_stmt_reader_enforces_body_limits_before_child_traversal",
):
    if needle not in tests:
        raise SystemExit(f"missing executable statement proof: {needle}")
for needle in ("flat_signature", "first_atom", "outcome"):
    if needle not in fixture:
        raise SystemExit(f"missing statement fixture surface: {needle}")
print("accepted_statement_kinds=8")
print("top_and_nested_body_preflight=green")
print("optional_else_absence=canonical")
print("mixed_stmt_expr_preorder=green")
print("body_limit_2047_2048_2049=green")
print("rust_hako_outcome_parity=green")
print("partial_snapshot_publication=0")
print("summary=ok")
PY

echo "[$TAG] ok"
