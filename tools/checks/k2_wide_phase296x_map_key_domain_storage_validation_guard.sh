#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="map-key-domain-storage-validation"
CARD="docs/development/current/main/phases/phase-296x/296x-879-MIMALLOC-MAP-KEY-DOMAIN-STORAGE-VALIDATION-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-878-MIMALLOC-MAP-KEY-DOMAIN-STORAGE-IMPLEMENTATION-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_map_key_domain_storage_validation_guard.sh"

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
  "output_contract=hako-mimalloc-map-key-domain-storage-validation-v0" \
  "source_evidence=296x-878" \
  "row_kind=validation" \
  "target_front=kilo_leaf_map_get_dynamic_covered_i64" \
  "implementation_guard_passed=1" \
  "cargo_fmt_check_passed=1" \
  "cargo_check_release_bin_hakorune_passed=1" \
  "current_state_pointer_guard_passed=1" \
  "git_diff_check_passed=1" \
  "map_key_domain_unit_tests_passed=1" \
  "mapbox_i64_text_alias_tests_passed=1" \
  "mapbox_public_key_text_test_passed=1" \
  "scalar_load_hi_consumes_map_key_domain=0" \
  "slot_load_hi_consumes_map_key_domain=0" \
  "slot_load_hh_consumes_map_key_domain=0" \
  "kernel_scalar_helper_route_changed=0" \
  "product_default_changed=0" \
  "winner_claim=0" \
  "summary=ok" \
  "selected_next=MIMALLOC-MAP-KEY-DOMAIN-STORAGE-MEASUREMENT-001"; do
  require_line "$expected"
done

grep -F -q "selected_next=MIMALLOC-MAP-KEY-DOMAIN-STORAGE-VALIDATION-001" "$PREV_CARD" || {
  echo "[$TAG] previous row does not hand off to validation" >&2
  exit 1
}

for proof in \
  "bash tools/checks/k2_wide_phase296x_map_key_domain_storage_implementation_guard.sh" \
  "cargo fmt --check" \
  "cargo check --release --bin hakorune" \
  "bash tools/checks/current_state_pointer_guard.sh" \
  "git diff --check" \
  "cargo test --lib test_key_domain -- --nocapture" \
  "cargo test --lib test_keys_public_text_after_key_domain_storage -- --nocapture" \
  "cargo test --lib map_key_domain -- --nocapture"; do
  grep -F -q "$proof" "$CARD" || {
    echo "[$TAG] missing proof command: $proof" >&2
    exit 1
  }
done

echo "[$TAG] ok"
