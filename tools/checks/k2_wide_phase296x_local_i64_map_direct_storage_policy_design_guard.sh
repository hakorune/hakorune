#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="local-i64-map-direct-storage-policy-design"
CARD="docs/development/current/main/phases/phase-296x/296x-914-LOCAL-I64-MAP-DIRECT-STORAGE-POLICY-DESIGN-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-913-MAP-HASH-OWNER-REFRESH-AFTER-LOCAL-I64-STORAGE-REALIZATION-CLOSEOUT-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_local_i64_map_direct_storage_policy_design_guard.sh"

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
  "output_contract=hako-local-i64-map-direct-storage-policy-design-v0" \
  "source_evidence=296x-913" \
  "row_kind=design" \
  "target_front=kilo_leaf_map_get_dynamic_covered_i64" \
  "selected_policy=exact_aot_unpublished_local_i64_map_direct_storage" \
  "selected_owner=MapStoragePlan" \
  "selected_candidate_shape=known_i64_key_set_and_scalar_get" \
  "selected_first_representation=closed_world_i64_key_value_table" \
  "publication_materialization_required=1" \
  "product_mapbox_storage_changed=0" \
  "product_hasher_swap=0" \
  "sidecar_storage=0" \
  "mirbuilder_map_storage_ownership=0" \
  "new_runtime_helper_enabled=0" \
  "backend_lowering_enabled=0" \
  "winner_claim=0" \
  "first_allowed_slice=passive_direct_storage_plan_descriptor" \
  "next_task=LOCAL-I64-MAP-DIRECT-STORAGE-PLAN-SURFACE-001" \
  "summary=ok"; do
  require_card_line "$expected"
done

for text in \
  "must not mutate product MapBox or swap its hasher" \
  "all observed writes use i64 keys" \
  "all observed hot reads use scalar i64 get" \
  "map is unpublished before those reads" \
  "closed_world_i64_key_value_table" \
  "does not replace product MapBox storage" \
  "materialization into product MapBox semantics"; do
  grep -F -q "$text" "$CARD" || {
    echo "[$TAG] missing design text: $text" >&2
    exit 1
  }
done

grep -F -q "next_task=LOCAL-I64-MAP-DIRECT-STORAGE-POLICY-DESIGN-001" "$PREV_CARD" || {
  echo "[$TAG] previous owner-refresh card does not hand off to direct storage policy design" >&2
  exit 1
}

echo "[$TAG] ok"
