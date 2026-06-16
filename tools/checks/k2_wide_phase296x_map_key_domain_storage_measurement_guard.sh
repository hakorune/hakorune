#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="map-key-domain-storage-measurement"
CARD="docs/development/current/main/phases/phase-296x/296x-880-MIMALLOC-MAP-KEY-DOMAIN-STORAGE-MEASUREMENT-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-879-MIMALLOC-MAP-KEY-DOMAIN-STORAGE-VALIDATION-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_map_key_domain_storage_measurement_guard.sh"
KERNEL_DOMAIN="crates/nyash_kernel/src/plugin/map_key_domain.rs"

for file in "$CARD" "$PREV_CARD" "$INDEX" "$KERNEL_DOMAIN"; do
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
  "output_contract=hako-mimalloc-map-key-domain-storage-measurement-v0" \
  "source_evidence=296x-879" \
  "row_kind=measurement" \
  "target_front=kilo_leaf_map_get_dynamic_covered_i64" \
  "measurement_command=bash tools/perf/build_perf_release.sh && bash tools/perf/bench_micro_aot_asm.sh kilo_leaf_map_get_dynamic_covered_i64 ny_main 3" \
  "top_symbol_1=core::hash::BuildHasher::hash_one" \
  "top_symbol_1_percent=39.73" \
  "top_symbol_2=nyash.map.scalar_load_hi" \
  "top_symbol_2_percent=31.14" \
  "top_symbol_3=<i64 as alloc::string::SpecToString>::spec_to_string" \
  "top_symbol_3_percent=23.81" \
  "mapbox_get_scalar_i64_key_str_percent=0.88" \
  "storage_key_domain_reached=1" \
  "scalar_helper_still_stringifies_i64_key=1" \
  "selected_owner=scalar_helper_key_domain_consumer" \
  "selected_owner_confidence=high" \
  "product_default_changed=0" \
  "winner_claim=0" \
  "summary=ok" \
  "selected_next=MIMALLOC-MAP-SCALAR-LOAD-I64-KEY-DOMAIN-CONSUMER-DESIGN-001"; do
  require_line "$expected"
done

grep -F -q "selected_next=MIMALLOC-MAP-KEY-DOMAIN-STORAGE-MEASUREMENT-001" "$PREV_CARD" || {
  echo "[$TAG] previous row does not hand off to measurement" >&2
  exit 1
}

grep -F -q "#![allow(dead_code)]" "$KERNEL_DOMAIN" || {
  echo "[$TAG] kernel prototype MapKeyDomain should be warning-clean while unused" >&2
  exit 1
}

for text in \
  "i64 -> decimal text -> MapKeyDomain::from_text -> HashMap lookup" \
  "i64 -> MapKeyDomain::from_i64 -> HashMap lookup" \
  "do not add sidecar storage" \
  "do not change the hasher before direct key-domain consumption is tested"; do
  grep -F -q "$text" "$CARD" || {
    echo "[$TAG] missing interpretation text: $text" >&2
    exit 1
  }
done

echo "[$TAG] ok"
