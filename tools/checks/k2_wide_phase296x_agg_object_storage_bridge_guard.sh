#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-735-AGG-OBJECT-STORAGE-BRIDGE-001.md"
SSOT="docs/development/current/main/design/record-box-two-surface-one-substrate-ssot.md"
OBJECT_SSOT="docs/development/current/main/design/object-storage-plan-boundary-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_agg_object_storage_bridge_guard.sh"

[[ -f "$CARD" ]] || { echo "[agg-object-storage-bridge] missing card: $CARD" >&2; exit 1; }
[[ -f "$SSOT" ]] || { echo "[agg-object-storage-bridge] missing SSOT: $SSOT" >&2; exit 1; }
[[ -f "$OBJECT_SSOT" ]] || { echo "[agg-object-storage-bridge] missing object SSOT: $OBJECT_SSOT" >&2; exit 1; }

grep -q '^Status: Landed$' "$CARD" || { echo "[agg-object-storage-bridge] row735 card must be Landed" >&2; exit 1; }
grep -q "$SELF_SCRIPT" "$INDEX" || { echo "[agg-object-storage-bridge] check index missing guard entry" >&2; exit 1; }

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[agg-object-storage-bridge] missing line in $file: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-aggregate-object-storage-bridge-v0" \
  "aggregate_storage_plan_vocabulary_defined=1" \
  "object_storage_plan_vocabulary_defined=1" \
  "shared_backend_lowering_concepts=1" \
  "source_semantics_merged=0" \
  "record_semantics_used_as_box_proof=0" \
  "mirbuilder_representation_owner=0" \
  "selected_next=RECORD-METHODS-GATE-000" \
  "summary=ok"; do
  require_line_in_file "$CARD" "$expected"
done

grep -F -q "AggregateStoragePlan / ObjectStoragePlan candidates:" "$SSOT" || {
  echo "[agg-object-storage-bridge] missing shared substrate candidates in SSOT" >&2
  exit 1
}
grep -F -q "record-box-two-surface-one-substrate-ssot.md" "$OBJECT_SSOT" || {
  echo "[agg-object-storage-bridge] object SSOT must link record/box SSOT" >&2
  exit 1
}

echo "[agg-object-storage-bridge] ok"
