#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-hako-bounded-body-snapshot-model-v0"
source "$ROOT/tools/checks/lib/guard_common.sh"

DIR="$ROOT/lang/src/compiler/analysis/bounded_body_snapshot"
FIXTURE="$ROOT/tools/checks/fixtures/bounded_body_snapshot_model_v0.hako"
VALIDATED_TEXT="$DIR/validated_text_v0.hako"
CARRIER_FIXTURE="$ROOT/tools/checks/fixtures/validated_text_v0.hako"

guard_require_command "$TAG" cargo
guard_require_command "$TAG" python3
guard_require_files "$TAG" "$DIR/outcome_v0.hako" "$DIR/path_v0.hako" \
  "$DIR/budget_v0.hako" "$DIR/snapshot_model_v0.hako" "$VALIDATED_TEXT" \
  "$FIXTURE" "$CARRIER_FIXTURE"

NYASH_DISABLE_PLUGINS=1 cargo run -q --features vm-reference --bin hakorune -- \
  --backend vm "$FIXTURE" >/tmp/hakorune-bounded-body-snapshot-model-v0.log 2>&1
grep -q 'RC: 0' /tmp/hakorune-bounded-body-snapshot-model-v0.log || {
  tail -n 80 /tmp/hakorune-bounded-body-snapshot-model-v0.log
  guard_fail "$TAG" "Hako snapshot model VM fixture failed"
}

for mode in unset 1; do
  if [ "$mode" = unset ]; then
    env -u NYASH_STR_CP NYASH_DISABLE_PLUGINS=1 cargo run -q --features vm-reference --bin hakorune -- \
      --backend vm "$CARRIER_FIXTURE" >/tmp/hakorune-validated-text-v0-unset.log 2>&1
    log=/tmp/hakorune-validated-text-v0-unset.log
  else
    NYASH_STR_CP=1 NYASH_DISABLE_PLUGINS=1 cargo run -q --features vm-reference --bin hakorune -- \
      --backend vm "$CARRIER_FIXTURE" >/tmp/hakorune-validated-text-v0-cp.log 2>&1
    log=/tmp/hakorune-validated-text-v0-cp.log
  fi
  grep -q 'RC: 0' "$log" || {
    tail -n 80 "$log"
    guard_fail "$TAG" "validated-text fixture failed with NYASH_STR_CP=$mode"
  }
done

python3 - "$DIR" "$VALIDATED_TEXT" "$CARRIER_FIXTURE" "$ROOT/lang/src" "$ROOT/tools/checks/fixtures" <<'PY'
import sys
from pathlib import Path

root, carrier_path, fixture_path, lang_root, fixture_root = map(Path, sys.argv[1:])
sources = list(root.glob("*.hako"))
joined = "\n".join(path.read_text(encoding="utf-8") for path in sources)
canonical_i64_path = root / "canonical_i64_v0.hako"
carrier = carrier_path.read_text(encoding="utf-8")
fixture = fixture_path.read_text(encoding="utf-8")
for needle in ("Ready", "Unsupported", "InvalidInput", "$.body", "max_node_count", "max_total_text_bytes",
               "BoundedBodySnapshotAtomV0", "value_kind", "BoundedBodySnapshotChildV0", "target_index",
               "atom(index)", "child(index)", "node(index)"):
    if needle not in joined:
        raise SystemExit(f"missing model contract: {needle}")
for forbidden in ("indexOf", "MIRBuilder", "planner", "route"):
    if forbidden in joined:
        raise SystemExit(f"forbidden model dependency: {forbidden}")
for path in sources:
    if path == canonical_i64_path:
        continue
    if "substring(" in path.read_text(encoding="utf-8"):
        raise SystemExit(f"substring is confined to canonical decoded-i64 parsing: {path}")
canonical_i64 = canonical_i64_path.read_text(encoding="utf-8")
for needle in ("ProgramV0CanonicalI64V0Box", "_read_decimal", "_digit_value"):
    if needle not in canonical_i64:
        raise SystemExit(f"missing canonical-i64 contract: {needle}")
for forbidden in ("object_key_at", "object_value_at", '== "type"', '== "body"', "PathV0"):
    if forbidden in canonical_i64:
        raise SystemExit(f"canonical-i64 parser escaped scalar ownership: {forbidden}")
for needle in (
    "box ValidatedTextV0",
    "box ValidatedTextBuildV0",
    "DecodedUtf8ByteLenV0Box.count",
    "InternalCarrierContractViolation",
    "internal.carrier.byte_count_mismatch",
    "replay_only",
    "total_text_bytes()",
):
    if needle not in joined:
        raise SystemExit(f"missing validated-text contract: {needle}")
for forbidden in (".length()", ".len()", ".size()", "MapBox", "indexOf", "substring("):
    if forbidden in carrier:
        raise SystemExit(f"forbidden validated-text dependency: {forbidden}")
for search_root in (lang_root, fixture_root):
    for path in search_root.rglob("*.hako"):
        if path in (carrier_path, fixture_path):
            continue
        text = path.read_text(encoding="utf-8")
        if "new ValidatedTextV0" in text:
            raise SystemExit(f"validated-text construction escapes its factory: {path}")
        if "replay_only" in text:
            raise SystemExit(f"direct Hako reader must not consume replay-only carrier input: {path}")
for needle in (
    "猫😸",
    "InternalCarrierContractViolation",
    "internal.carrier.byte_count_mismatch",
    "limit.max_atom_bytes",
    "replay_only",
    "total_text_bytes",
):
    if needle not in fixture:
        raise SystemExit(f"missing validated-text fixture: {needle}")
for path in sources:
    if len(path.read_text(encoding="utf-8").splitlines()) > 800:
        raise SystemExit(f"source exceeds 800 lines: {path}")
print("output_contract=HakoBoundedBodySnapshotModelV0")
print("three_way_outcome=1")
print("structural_path=1")
print("schema_budget=1")
print("immutable_publication_model=1")
print("flat_preorder_table=1")
print("ordered_atom_records=1")
print("ordered_child_edges=1")
print("raw_json_reader=0")
print("sealed_hhako_text_witness=1")
print("replay_count_authority=0")
print("summary=ok")
PY
