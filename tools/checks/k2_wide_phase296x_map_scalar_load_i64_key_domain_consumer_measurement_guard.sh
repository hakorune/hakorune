#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="map-scalar-load-i64-key-domain-consumer-measurement"
CARD="docs/development/current/main/phases/phase-296x/296x-883-MIMALLOC-MAP-SCALAR-LOAD-I64-KEY-DOMAIN-CONSUMER-MEASUREMENT-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-882-MIMALLOC-MAP-SCALAR-LOAD-I64-KEY-DOMAIN-CONSUMER-IMPLEMENTATION-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_map_scalar_load_i64_key_domain_consumer_measurement_guard.sh"

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
  "output_contract=hako-mimalloc-map-scalar-load-i64-key-domain-consumer-measurement-v0" \
  "source_evidence=296x-882" \
  "row_kind=measurement" \
  "target_front=kilo_leaf_map_get_dynamic_covered_i64" \
  "measurement_command=bash tools/perf/build_perf_release.sh && bash tools/perf/bench_micro_aot_asm.sh kilo_leaf_map_get_dynamic_covered_i64 ny_main 3" \
  "cycles_before=1180142598" \
  "cycles_after=484694805" \
  "cycles_reduction_pct=58.93" \
  "spec_to_string_top_before_percent=23.81" \
  "spec_to_string_top_after_percent=0" \
  "scalar_load_hi_top_before_percent=31.14" \
  "scalar_load_hi_top_after_percent=3.30" \
  "top_symbol_1=nyash_rust::boxes::map_box::MapBox::get_scalar_i64_key_domain" \
  "top_symbol_1_percent=64.18" \
  "top_symbol_2=core::hash::BuildHasher::hash_one" \
  "top_symbol_2_percent=31.43" \
  "top_symbol_3=nyash.map.scalar_load_hi" \
  "top_symbol_3_percent=3.30" \
  "target_counter_shrinks=1" \
  "selected_keeper=scalar_helper_key_domain_consumer" \
  "winner_claim=1" \
  "selected_next=MIMALLOC-MAP-SCALAR-LOAD-I64-POST-DOMAIN-CONSUMER-OWNER-SELECTION-001" \
  "summary=ok"; do
  require_line "$expected"
done

grep -F -q "selected_next=MIMALLOC-MAP-SCALAR-LOAD-I64-KEY-DOMAIN-CONSUMER-MEASUREMENT-001" "$PREV_CARD" || {
  echo "[$TAG] previous row does not hand off to measurement" >&2
  exit 1
}

for text in \
  "do not jump directly to a hasher swap" \
  "do not add sidecar storage from this measurement alone" \
  "MapBox::get_scalar_i64_key_domain" \
  "core::hash::BuildHasher::hash_one"; do
  grep -F -q "$text" "$CARD" || {
    echo "[$TAG] missing measurement text: $text" >&2
    exit 1
  }
done

echo "[$TAG] ok"
