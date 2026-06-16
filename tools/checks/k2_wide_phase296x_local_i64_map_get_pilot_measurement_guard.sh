#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="local-i64-map-get-pilot-measurement"
CARD="docs/development/current/main/phases/phase-296x/296x-893-LOCAL-I64-MAP-GET-PILOT-MEASUREMENT-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-892-LOCAL-I64-MAP-GET-PILOT-VALIDATION-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_local_i64_map_get_pilot_measurement_guard.sh"

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
  "output_contract=hako-local-i64-map-get-pilot-measurement-v0" \
  "source_evidence=296x-892" \
  "row_kind=measurement" \
  "target_front=kilo_leaf_map_get_dynamic_covered_i64" \
  "ny_main_loop_helper_before=nyash.map.scalar_load_hi" \
  "ny_main_loop_helper_after=nyash.map.local_i64_get_hi" \
  "local_i64_get_helper_reached=1" \
  "scalar_load_hi_loop_call_removed=1" \
  "top_symbol_0=nyash_rust::boxes::map_box::MapBox::get_scalar_i64_key_domain" \
  "top_symbol_0_pct=50.23" \
  "top_symbol_1=core::hash::BuildHasher::hash_one" \
  "top_symbol_1_pct=46.72" \
  "top_symbol_2=nyash_rust::boxes::map_box::MapBox::get_scalar_i64_key_i64" \
  "top_symbol_2_pct=3.03" \
  "helper_delegates_to_existing_scalar_load=1" \
  "remaining_hot_owner=map_hash_lookup_boundary" \
  "winner_claim=0" \
  "next_task=LOCAL-I64-MAP-GET-PILOT-CLOSEOUT-001" \
  "summary=ok"; do
  require_card_line "$expected"
done

grep -F -q "next_task=LOCAL-I64-MAP-GET-PILOT-MEASUREMENT-001" "$PREV_CARD" || {
  echo "[$TAG] validation row does not hand off to measurement" >&2
  exit 1
}

for text in \
  "call   4121a0 <nyash.map.local_i64_get_hi>" \
  "call   411e40 <nyash.map.slot_load_hh>" \
  "This row validates reachability, not a performance win." \
  "remaining hot owner is still \`map_hash_lookup_boundary\`" \
  "no Hako-vs-C winner claim" \
  "no product MapBox storage change" \
  "no product hasher swap" \
  "no sidecar storage" \
  "no MIRBuilder map storage ownership"; do
  grep -F -q "$text" "$CARD" || {
    echo "[$TAG] missing measurement text: $text" >&2
    exit 1
  }
done

echo "[$TAG] ok"
