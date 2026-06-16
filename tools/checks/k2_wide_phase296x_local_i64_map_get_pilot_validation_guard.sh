#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="local-i64-map-get-pilot-validation"
CARD="docs/development/current/main/phases/phase-296x/296x-892-LOCAL-I64-MAP-GET-PILOT-VALIDATION-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-891-LOCAL-I64-MAP-GET-PILOT-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_local_i64_map_get_pilot_validation_guard.sh"

for file in "$CARD" "$PREV_CARD" "$INDEX"; do
  [[ -f "$file" ]] || { echo "[$TAG] missing file: $file" >&2; exit 1; }
done

grep -q '^Status: Landed$' "$CARD" || {
  echo "[$TAG] card must be Landed" >&2
  exit 1
}

grep -F -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[$TAG] check index missing guard entry" >&2
  exit 1
}

require_card_line() {
  local expected="$1"
  if ! grep -F -x -q "$expected" "$CARD"; then
    echo "[$TAG] missing card line: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-local-i64-map-get-pilot-validation-v0" \
  "source_evidence=296x-891" \
  "row_kind=validation" \
  "target_front=kilo_leaf_map_get_dynamic_covered_i64" \
  "local_i64_map_shadow_get_consumer_enabled=1" \
  "python_metadata_consumer_test=pass" \
  "map_repr_plan_unit_tests=pass" \
  "release_cargo_check=pass" \
  "current_state_pointer_guard=pass" \
  "diff_check=pass" \
  "product_mapbox_storage_changed=0" \
  "product_hasher_swap=0" \
  "sidecar_storage=0" \
  "mirbuilder_map_storage_ownership=0" \
  "winner_claim=0" \
  "next_task=LOCAL-I64-MAP-GET-PILOT-MEASUREMENT-001" \
  "summary=ok"; do
  require_card_line "$expected"
done

grep -F -q "next_task=LOCAL-I64-MAP-GET-PILOT-VALIDATION-001" "$PREV_CARD" || {
  echo "[$TAG] implementation row does not hand off to validation" >&2
  exit 1
}

for text in \
  "This is not a performance claim." \
  "helper still delegates to the existing" \
  "next row must measure" \
  "no Hako-vs-C winner claim" \
  "no product MapBox storage change" \
  "no product hasher swap" \
  "no sidecar storage" \
  "no MIRBuilder map storage ownership" \
  "no helper-name or benchmark-name inference"; do
  grep -F -q "$text" "$CARD" || {
    echo "[$TAG] missing validation text: $text" >&2
    exit 1
  }
done

echo "[$TAG] ok"
