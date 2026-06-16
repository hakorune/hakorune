#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="map-hash-owner-refresh-after-local-fastpath"
CARD="docs/development/current/main/phases/phase-296x/296x-904-MAP-HASH-OWNER-REFRESH-AFTER-LOCAL-FASTPATH-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-903-LOCAL-KNOWN-RECEIVER-DIRECT-CALL-MEASUREMENT-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_map_hash_owner_refresh_after_local_fastpath_guard.sh"

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
  "output_contract=hako-map-hash-owner-refresh-after-local-fastpath-v0" \
  "source_evidence=296x-903" \
  "row_kind=owner_refresh" \
  "target_front=kilo_leaf_map_get_dynamic_covered_i64" \
  "local_fastpath_fact_reached=1" \
  "ny_main_loop_helper=nyash.map.local_i64_get_hi" \
  "slot_load_hh_loop_boundary_removed=1" \
  "post_loop_slot_load_hh_allowed=1" \
  "remaining_hot_owner=map_key_domain_hash_lookup_boundary" \
  "mapbox_get_scalar_i64_key_domain_pct=70.20" \
  "build_hasher_hash_one_pct=22.41" \
  "canonical_i64_hot_lookup_visible=1" \
  "text_key_hot_lookup_visible=0" \
  "mixed_key_hot_lookup_visible=0" \
  "product_hasher_policy=std_default" \
  "product_hasher_swap_allowed=0" \
  "product_mapbox_i64_only_allowed=0" \
  "sidecar_storage_allowed=0" \
  "mirbuilder_map_storage_ownership=0" \
  "implementation_allowed=0" \
  "selected_next=LOCAL-I64-MAP-STORAGE-REALIZATION-DESIGN-001" \
  "summary=ok"; do
  require_card_line "$expected"
done

for text in \
  "The current evidence is not a reason to mutate product \`MapBox\` hasher policy." \
  "before publication:" \
  "at publication:" \
  "after publication:" \
  "do not swap the product \`HashMap\` hasher from this evidence" \
  "do not move map storage decisions into MIRBuilder"; do
  grep -F -q "$text" "$CARD" || {
    echo "[$TAG] missing invariant text: $text" >&2
    exit 1
  }
done

grep -F -q "selected_next=MAP-HASH-OWNER-REFRESH-AFTER-LOCAL-FASTPATH-001" "$PREV_CARD" || {
  echo "[$TAG] previous measurement card does not hand off to owner refresh" >&2
  exit 1
}

echo "[$TAG] ok"
