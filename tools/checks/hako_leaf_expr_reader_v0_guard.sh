#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="hako-leaf-expr-reader-v0"
DIR="$ROOT/lang/src/compiler/analysis/bounded_body_snapshot"
READER="$DIR/reader_expr_leaf_v0.hako"
I64="$DIR/canonical_i64_v0.hako"
FIXTURE="$ROOT/tools/checks/fixtures/bounded_body_snapshot_leaf_expr_reader_v0.hako"
SESSION="$ROOT/src/backend/mir_interpreter/strict_json_session.rs"

cd "$ROOT"
cargo build -q --release --features vm-reference --bin hakorune
for mode in unset 1; do
  if [ "$mode" = unset ]; then
    env -u NYASH_STR_CP cargo test -q --features vm-reference --lib \
      backend::mir_interpreter::strict_json_session::tests::hako_leaf_expr_reader_
  else
    NYASH_STR_CP=1 cargo test -q --features vm-reference --lib \
      backend::mir_interpreter::strict_json_session::tests::hako_leaf_expr_reader_
  fi
done
timeout 10s env NYASH_DISABLE_PLUGINS=1 NYASH_MIR_COMPILE_TRACE=1 \
  target/release/hakorune --dump-mir --no-optimize "$FIXTURE" \
  >/dev/null 2>/tmp/hako-leaf-expr-reader-v0.timing.log
grep -q 'stage=build_module' /tmp/hako-leaf-expr-reader-v0.timing.log
grep -q 'stage=semantic_refresh' /tmp/hako-leaf-expr-reader-v0.timing.log

python3 - "$READER" "$I64" "$FIXTURE" "$SESSION" <<'PY'
import sys
from pathlib import Path

reader_path, i64_path, fixture_path, session_path = map(Path, sys.argv[1:])
reader = reader_path.read_text(encoding="utf-8")
i64 = i64_path.read_text(encoding="utf-8")
fixture = fixture_path.read_text(encoding="utf-8")
session = session_path.read_text(encoding="utf-8")
for path in (reader_path, i64_path, fixture_path, session_path):
    if len(path.read_text(encoding="utf-8").splitlines()) > 800:
        raise SystemExit(f"source exceeds 800 lines: {path}")
for needle in (
    "LeafExprObservationV0", 'kind == "Int"', 'kind == "Str"',
    'kind == "Bool"', 'kind == "Null"', 'kind == "Var"',
    "ValidatedTextV0Box.literal", "ValidatedTextV0Box.atom",
    "reader.child_expr_family_pending", "object.forbidden_unknown_field",
):
    if needle not in reader:
        raise SystemExit(f"missing leaf-reader contract: {needle}")
for forbidden in ("MapBox", "indexOf", "substring(", "ValidatedProgramV0BodyView", "MIRBuilder"):
    if forbidden in reader:
        raise SystemExit(f"forbidden leaf-reader dependency: {forbidden}")
for needle in (
    "9223372036854775807", "9223372036854775808",
    "value = value * 10 - decimal", "int.not_canonical_i64",
):
    if needle not in i64:
        raise SystemExit(f"missing canonical-i64 contract: {needle}")
for forbidden in ("toInteger", "MapBox", "indexOf", "ProgramV0BodyView"):
    if forbidden in i64:
        raise SystemExit(f"forbidden canonical-i64 dependency: {forbidden}")
for needle in (
    "hako_leaf_expr_reader_matches_reference_kinds_and_failures",
    "hako_leaf_expr_reader_normalizes_full_canonical_i64_domain",
    "hako_leaf_expr_reader_preserves_decoded_text_and_limits",
):
    if needle not in session:
        raise SystemExit(f"missing executable leaf proof: {needle}")
for needle in ('"猫😸"', '"-9223372036854775808"', "limit.max_atom_bytes", "limit.max_literal_bytes"):
    if needle not in fixture + session:
        raise SystemExit(f"missing leaf fixture boundary: {needle}")
print("accepted_leaf_kinds=5")
print("canonical_i64_owner=hhako")
print("decoded_text_owner=hhako")
print("rust_hako_outcome_parity=green")
print("partial_snapshot_publication=0")
print("summary=ok")
PY

echo "[$TAG] ok"
