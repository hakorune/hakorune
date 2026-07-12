#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-decoded-utf8-byte-len-v0"
source "$ROOT/tools/checks/lib/guard_common.sh"

LEAF="$ROOT/src/analysis/bounded_body_snapshot_v0/decoded_utf8_byte_len_v0.rs"
VIEW="$ROOT/src/analysis/bounded_body_snapshot_v0/validated_view.rs"
BUDGET="$ROOT/src/analysis/bounded_body_snapshot_v0/budget.rs"
FIXTURES="$ROOT/src/analysis/bounded_body_snapshot_v0/tests/decoded_utf8_byte_len.rs"
WITNESS="$ROOT/src/analysis/bounded_body_snapshot_v0/program_v0_snapshot_witness.rs"

guard_require_command "$TAG" cargo
guard_require_command "$TAG" python3
guard_require_files "$TAG" "$LEAF" "$VIEW" "$BUDGET" "$FIXTURES" "$WITNESS"

for mode in unset 1; do
  if [ "$mode" = unset ]; then
    env -u NYASH_STR_CP cargo test -q --lib \
      analysis::bounded_body_snapshot_v0::tests::decoded_utf8_byte_len
  else
    NYASH_STR_CP=1 cargo test -q --lib \
      analysis::bounded_body_snapshot_v0::tests::decoded_utf8_byte_len
  fi
done

python3 - "$LEAF" "$VIEW" "$BUDGET" "$FIXTURES" "$WITNESS" <<'PY'
import sys
from pathlib import Path

leaf_path, view_path, budget_path, fixtures_path, witness_path = map(Path, sys.argv[1:])
leaf = leaf_path.read_text(encoding="utf-8")
view = view_path.read_text(encoding="utf-8")
budget = budget_path.read_text(encoding="utf-8")
fixtures = fixtures_path.read_text(encoding="utf-8")
witness = witness_path.read_text(encoding="utf-8")

for needle in (
    "pub(crate) struct DecodedUtf8ByteLenV0",
    "pub(crate) fn count(value: &str) -> usize",
    "value.as_bytes().len()",
):
    if needle not in leaf:
        raise SystemExit(f"missing byte-length contract: {needle}")

for consumer in (view, budget):
    if "DecodedUtf8ByteLenV0::count" not in consumer:
        raise SystemExit("normalized text/budget bypasses the byte-length leaf")

for needle in (
    "ValidatedTextV0::from_decoded",
    "pub fn value(self)",
    "pub fn utf8_byte_len(self)",
    "pub fn class(self)",
):
    if needle not in view:
        raise SystemExit(f"missing sealed validated-text contract: {needle}")
for forbidden in ("pub value:", "pub utf8_byte_len:", "pub class:"):
    if forbidden in view:
        raise SystemExit(f"validated-text witness remains caller-injectable: {forbidden}")
if "value.value().to_owned()" not in witness:
    raise SystemExit("snapshot witness bypasses the sealed validated-text accessor")
if "pub fn total_text_bytes(&self)" not in budget:
    raise SystemExit("budget lacks read-only total text observation")

for needle in (
    "猫",
    "😸",
    "e\\u{0301}",
    "\\0",
    "does_not_normalize",
    "constructor_derives_the_only_byte_witness",
    "total_text_bytes",
):
    if needle not in fixtures:
        raise SystemExit(f"missing byte-length fixture: {needle}")

for forbidden in (
    "NYASH_STR_CP",
    "string_codepoint_mode",
    ".length()",
    "value.len()",
    "StringBox",
    "nyash.string.len_h",
    "nyash.any.length_h",
    "nyrt_string_length",
    "strlen",
    "CStr",
):
    if forbidden in leaf:
        raise SystemExit(f"forbidden byte-length dependency: {forbidden}")

for path in (leaf_path, view_path, budget_path, fixtures_path, witness_path):
    if len(path.read_text(encoding="utf-8").splitlines()) > 800:
        raise SystemExit(f"source exceeds 800 lines: {path}")

print("output_contract=DecodedUtf8ByteLenV0")
print("environment_independent=1")
print("utf8_octet_contract=1")
print("normalization=0")
print("embedded_nul=1")
print("sealed_rhako_text_witness=1")
print("public_string_surface=0")
print("existing_length_route_dependency=0")
print("summary=ok")
PY
