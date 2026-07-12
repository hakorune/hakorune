#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="hako-child-expr-reader-v0"
DIR="$ROOT/lang/src/compiler/analysis/bounded_body_snapshot"
READER="$DIR/reader_expr_child_v0.hako"
LEAF="$DIR/reader_expr_leaf_v0.hako"
FIXTURE="$ROOT/tools/checks/fixtures/bounded_body_snapshot_child_expr_reader_v0.hako"
SESSION="$ROOT/src/backend/mir_interpreter/strict_json_session.rs"

cd "$ROOT"
cargo build -q --release --features vm-reference --bin hakorune
for mode in unset 1; do
  if [ "$mode" = unset ]; then
    env -u NYASH_STR_CP cargo test -q --features vm-reference --lib \
      backend::mir_interpreter::strict_json_session::tests::hako_child_expr_reader_
  else
    NYASH_STR_CP=1 cargo test -q --features vm-reference --lib \
      backend::mir_interpreter::strict_json_session::tests::hako_child_expr_reader_
  fi
done
TIMING="$(mktemp /tmp/hako-child-expr-reader-v0.XXXXXX.log)"
trap 'rm -f "$TIMING"' EXIT
timeout 10s env NYASH_DISABLE_PLUGINS=1 NYASH_MIR_COMPILE_TRACE=1 \
  target/release/hakorune --dump-mir --no-optimize "$FIXTURE" \
  >/dev/null 2>"$TIMING"
grep -q 'stage=build_module' "$TIMING"
grep -q 'stage=semantic_refresh' "$TIMING"

python3 - "$READER" "$LEAF" "$FIXTURE" "$SESSION" <<'PY'
import sys
from pathlib import Path

reader_path, leaf_path, fixture_path, session_path = map(Path, sys.argv[1:])
reader = reader_path.read_text(encoding="utf-8")
leaf = leaf_path.read_text(encoding="utf-8")
fixture = fixture_path.read_text(encoding="utf-8")
session = session_path.read_text(encoding="utf-8")
for path in (reader_path, leaf_path, fixture_path, session_path):
    if len(path.read_text(encoding="utf-8").splitlines()) > 800:
        raise SystemExit(f"source exceeds 800 lines: {path}")
for needle in (
    "ChildExprObservationV0", "ChildExprEdgeV0", 'ChildExprEdgeV0("lhs"',
    'ChildExprEdgeV0("rhs"', "budget.observe_node(depth)",
    "operator.invalid_for_kind", "reader.call_expr_family_pending",
    "ValidatedTextV0Box.atom", "HakoProgramV0LeafExprReaderV0Box.read",
):
    if needle not in reader:
        raise SystemExit(f"missing child-reader contract: {needle}")
for forbidden in ("MapBox", "indexOf", "substring(", "ValidatedProgramV0BodyView", "MIRBuilder"):
    if forbidden in reader:
        raise SystemExit(f"forbidden child-reader dependency: {forbidden}")
for needle in ("child_count() { return 0 }", "child(index) { return null }"):
    if needle not in leaf:
        raise SystemExit(f"missing leaf child-interface closure: {needle}")
for needle in (
    "hako_child_expr_reader_matches_reference_outcomes",
    "hako_child_expr_reader_preserves_operator_partition_and_child_order",
    "hako_child_expr_reader_enforces_depth_before_publication",
):
    if needle not in session:
        raise SystemExit(f"missing executable child proof: {needle}")
for needle in ('node.kind() == "Binary"', 'node.kind() == "Compare"',
               'node.kind() == "Logical"', "preorder_signature", "classify_at_limit_depth",
               "classify_at_node_limit"):
    if needle not in fixture:
        raise SystemExit(f"missing child fixture boundary: {needle}")
print("accepted_child_kinds=3")
print("operator_partition=closed")
print("child_order=lhs_then_rhs")
print("recursive_preorder=green")
print("rust_hako_outcome_parity=green")
print("partial_snapshot_publication=0")
print("summary=ok")
PY

echo "[$TAG] ok"
