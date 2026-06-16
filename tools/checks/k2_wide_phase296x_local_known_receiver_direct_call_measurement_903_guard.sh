#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="local-known-receiver-direct-call-measurement-903"
CARD="docs/development/current/main/phases/phase-296x/296x-903-LOCAL-KNOWN-RECEIVER-DIRECT-CALL-MEASUREMENT-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-902-LOCAL-FASTPATH-FACT-PRODUCER-SELECTION-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_local_known_receiver_direct_call_measurement_903_guard.sh"

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
  "output_contract=hako-local-known-receiver-direct-call-measurement-v1" \
  "source_evidence=296x-900,296x-901,296x-902" \
  "target_front=kilo_leaf_map_get_dynamic_covered_i64" \
  "target_source=benchmarks/bench_kilo_leaf_map_get_dynamic_covered_i64.hako" \
  "mir_json_local_fastpath_fact_count=1" \
  "mir_json_fact_block=19" \
  "mir_json_fact_instruction_index=5" \
  "mir_json_fact_route_plan=map_repr.generic_hash_runtime" \
  "mir_json_fact_fallback_reason=null" \
  "ny_main_loop_uses_local_fastpath_helper=1" \
  "ny_main_loop_helper=nyash.map.local_i64_get_hi" \
  "ny_main_loop_slot_load_hh_count=0" \
  "post_loop_slot_load_hh_allowed=1" \
  "top_symbol=nyash_rust::boxes::map_box::MapBox::get_scalar_i64_key_domain" \
  "second_symbol=core::hash::BuildHasher::hash_one" \
  "winner_claim=0" \
  "reachability_success=1" \
  "selected_next=MAP-HASH-OWNER-REFRESH-AFTER-LOCAL-FASTPATH-001" \
  "summary=ok"; do
  require_card_line "$expected"
done

for text in \
  "Fact present:" \
  "Fact absent:" \
  "This row is a reachability success, not a performance winner." \
  "no product hasher swap" \
  "no helper-name or benchmark-name inference"; do
  grep -F -q "$text" "$CARD" || {
    echo "[$TAG] missing invariant text: $text" >&2
    exit 1
  }
done

grep -F -q "next_task=LOCAL-KNOWN-RECEIVER-DIRECT-CALL-MEASUREMENT-001" "$PREV_CARD" || {
  echo "[$TAG] previous producer card does not hand off to measurement" >&2
  exit 1
}

echo "[$TAG] ok"
