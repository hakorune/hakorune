#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="map-hash-owner-refresh-after-local-i64-storage-realization-closeout"
CARD="docs/development/current/main/phases/phase-296x/296x-913-MAP-HASH-OWNER-REFRESH-AFTER-LOCAL-I64-STORAGE-REALIZATION-CLOSEOUT-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-912-LOCAL-I64-MAP-STORAGE-REALIZATION-CLOSEOUT-001.md"
INDEX="docs/tools/check-scripts-index.md"
ALIASES="crates/nyash_kernel/src/plugin/map_aliases.rs"
SLOT_LOAD="crates/nyash_kernel/src/plugin/map_slot_load.rs"
MAP_BOX="src/boxes/map_box.rs"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_map_hash_owner_refresh_after_local_i64_storage_realization_closeout_guard.sh"

for file in "$CARD" "$PREV_CARD" "$INDEX" "$ALIASES" "$SLOT_LOAD" "$MAP_BOX"; do
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
  "output_contract=hako-map-hash-owner-refresh-after-local-i64-storage-realization-closeout-v0" \
  "source_evidence=296x-912" \
  "row_kind=owner_refresh" \
  "target_front=kilo_leaf_map_get_dynamic_covered_i64" \
  "measurement_runner=tools/perf/bench_micro_aot_asm.sh" \
  "measurement_runner_mode=direct" \
  "measurement_runs=2" \
  "ny_main_hot_loop_calls_local_i64_get_hi=1" \
  "ny_main_post_loop_slot_load_hh_call=1" \
  "run1_samples=40" \
  "run1_top_symbol=MapBox::get_scalar_i64_key_domain" \
  "run1_top_symbol_percent=60.10" \
  "run1_hash_one_percent=36.84" \
  "run2_samples=44" \
  "run2_top_symbol=MapBox::get_scalar_i64_key_domain" \
  "run2_top_symbol_percent=67.10" \
  "run2_hash_one_percent=21.89" \
  "selected_owner=product_map_key_domain_hash_lookup_boundary" \
  "selected_owner_confidence=medium" \
  "local_helper_reaches_product_mapbox_storage=1" \
  "codegen_owner_selected=0" \
  "product_hasher_swap_allowed=0" \
  "product_mapbox_storage_change_allowed=0" \
  "sidecar_storage_allowed=0" \
  "mirbuilder_map_storage_ownership=0" \
  "winner_claim=0" \
  "next_task=LOCAL-I64-MAP-DIRECT-STORAGE-POLICY-DESIGN-001" \
  "summary=ok"; do
  require_card_line "$expected"
done

for text in \
  "nyash.map.local_i64_get_hi" \
  "map_scalar_load_i64" \
  "MapBox::get_scalar_i64_key_i64" \
  "MapBox::get_scalar_i64_key_domain" \
  "Do not swap the product hasher"; do
  grep -F -q "$text" "$CARD" || {
    echo "[$TAG] missing evidence text: $text" >&2
    exit 1
  }
done

grep -F -q "nyash.map.local_i64_get_hi" "$ALIASES" || {
  echo "[$TAG] missing local helper alias" >&2
  exit 1
}

grep -F -q "map_scalar_load_i64(handle, key_i64)" "$ALIASES" || {
  echo "[$TAG] local helper no longer delegates to scalar load" >&2
  exit 1
}

grep -F -q "map.get_scalar_i64_key_i64(key_i64).unwrap_or(0)" "$SLOT_LOAD" || {
  echo "[$TAG] scalar load no longer reaches MapBox scalar i64 get" >&2
  exit 1
}

grep -F -q "fn get_scalar_i64_key_domain" "$MAP_BOX" || {
  echo "[$TAG] MapBox key-domain lookup function missing" >&2
  exit 1
}

grep -F -q "next_task=MAP-HASH-OWNER-REFRESH-AFTER-LOCAL-I64-STORAGE-REALIZATION-CLOSEOUT-001" "$PREV_CARD" || {
  echo "[$TAG] previous closeout card does not hand off to owner refresh" >&2
  exit 1
}

echo "[$TAG] ok"
