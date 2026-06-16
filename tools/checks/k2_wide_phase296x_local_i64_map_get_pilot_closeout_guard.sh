#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="local-i64-map-get-pilot-closeout"
CARD="docs/development/current/main/phases/phase-296x/296x-894-LOCAL-I64-MAP-GET-PILOT-CLOSEOUT-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-893-LOCAL-I64-MAP-GET-PILOT-MEASUREMENT-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_local_i64_map_get_pilot_closeout_guard.sh"

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
  "output_contract=hako-local-i64-map-get-pilot-closeout-v0" \
  "source_evidence=296x-893" \
  "row_kind=closeout" \
  "target_front=kilo_leaf_map_get_dynamic_covered_i64" \
  "helper_reachability_keeper=1" \
  "local_i64_get_helper_reached=1" \
  "performance_winner_claim=0" \
  "remaining_hot_owner=map_hash_lookup_boundary" \
  "pilot_result=metadata_consumer_reachability_only" \
  "close_pilot_helper_extension=1" \
  "next_design_owner=local_fastpath_eligibility" \
  "next_task=LOCAL-FASTPATH-ELIGIBILITY-SSOT-001" \
  "product_mapbox_storage_changed=0" \
  "product_hasher_swap=0" \
  "sidecar_storage=0" \
  "mirbuilder_map_storage_ownership=0" \
  "benchmark_name_branch=0" \
  "helper_name_inference=0" \
  "summary=ok"; do
  require_card_line "$expected"
done

grep -F -q "next_task=LOCAL-I64-MAP-GET-PILOT-CLOSEOUT-001" "$PREV_CARD" || {
  echo "[$TAG] measurement row does not hand off to closeout" >&2
  exit 1
}

for text in \
  "reachability keeper, not as a" \
  "performance keeper" \
  "Do not extend this family by adding more helper aliases." \
  "Observation" \
  "Eligibility Decision" \
  "LocalFastPathFact" \
  "fallback evidence:" \
  "backend-consumable=0" \
  "LocalFastPathFact:" \
  "no helper alias extension as optimization"; do
  grep -F -q "$text" "$CARD" || {
    echo "[$TAG] missing closeout text: $text" >&2
    exit 1
  }
done

echo "[$TAG] ok"
