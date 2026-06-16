#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="map-scalar-load-i64-post-domain-consumer-owner-selection"
CARD="docs/development/current/main/phases/phase-296x/296x-884-MIMALLOC-MAP-SCALAR-LOAD-I64-POST-DOMAIN-CONSUMER-OWNER-SELECTION-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-883-MIMALLOC-MAP-SCALAR-LOAD-I64-KEY-DOMAIN-CONSUMER-MEASUREMENT-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_map_scalar_load_i64_post_domain_consumer_owner_selection_guard.sh"

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

require_line() {
  local expected="$1"
  if ! grep -F -x -q "$expected" "$CARD"; then
    echo "[$TAG] missing card line: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-mimalloc-map-scalar-load-i64-post-domain-consumer-owner-selection-v0" \
  "source_evidence=296x-883" \
  "row_kind=owner_selection" \
  "target_front=kilo_leaf_map_get_dynamic_covered_i64" \
  "fresh_top_owner=MapBox::get_scalar_i64_key_domain" \
  "fresh_secondary_owner=core::hash::BuildHasher::hash_one" \
  "selected_owner=map_key_domain_hash_lookup_policy" \
  "selected_owner_confidence=high" \
  "implementation_allowed=0" \
  "reason=requires_map_storage_policy_design" \
  "hasher_swap_allowed=0" \
  "typed_i64_storage_allowed=0" \
  "sidecar_storage_allowed=0" \
  "public_mapbox_semantics_changed=0" \
  "mirbuilder_changed=0" \
  "route_proof_changed=0" \
  "winner_claim=0" \
  "selected_next=DESIGN-CONSULT-MAP-KEY-DOMAIN-HASH-LOOKUP-POLICY-001" \
  "summary=ok"; do
  require_line "$expected"
done

grep -F -q "selected_next=MIMALLOC-MAP-SCALAR-LOAD-I64-POST-DOMAIN-CONSUMER-OWNER-SELECTION-001" "$PREV_CARD" || {
  echo "[$TAG] previous row does not hand off to owner selection" >&2
  exit 1
}

for text in \
  "Option A:" \
  "Option B:" \
  "Option C:" \
  "do not swap HashMap hasher as a drive-by change" \
  "do not add i64 sidecar storage" \
  "do not introduce typed i64 Map storage without a storage-substrate SSOT" \
  "no benchmark/helper-name hardcode"; do
  grep -F -q "$text" "$CARD" || {
    echo "[$TAG] missing stop/design text: $text" >&2
    exit 1
  }
done

echo "[$TAG] ok"
