#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="leaf-array-string-len-attribution-repeat"
CARD="docs/development/current/main/phases/phase-296x/296x-846-MIMALLOC-LEAF-ARRAY-STRING-LEN-ATTRIBUTION-REPEAT-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-845-MIMALLOC-LEAF-ARRAY-STRING-LEN-OWNER-INVENTORY-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_leaf_array_string_len_attribution_repeat_guard.sh"

for file in "$CARD" "$PREV_CARD"; do
  [[ -f "$file" ]] || { echo "[$TAG] missing file: $file" >&2; exit 1; }
  grep -q '^Status: Landed$' "$file" || {
    echo "[$TAG] card must be Landed: $file" >&2
    exit 1
  }
done

grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[$TAG] check index missing guard entry" >&2
  exit 1
}

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[$TAG] missing line in $file: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-mimalloc-leaf-array-string-len-attribution-repeat-v0" \
  "source_evidence=296x-845,microasm-repeat-2026-06-16" \
  "row_kind=owner_selection" \
  "target_front=kilo_leaf_array_string_len" \
  "ny_main_calls_hako_array_text_slot_len=1" \
  "hako_array_text_slot_len_calls_localkey=1" \
  "runtime_tls_boundary_visible=1" \
  "selected_owner=array_text_slot_len_handle_cache_tls_boundary" \
  "selected_owner_confidence=medium" \
  "implementation_allowed=1" \
  "implementation_scope=array_string_len_readonly_ready_path" \
  "existing_ready_seam=with_array_box_ready" \
  "new_backend_route_enabled=0" \
  "mirbuilder_changed=0" \
  "backend_lowering_changed=0" \
  "product_default_changed=0" \
  "helper_name_inference_enabled=0" \
  "selected_next=MIMALLOC-LEAF-ARRAY-STRING-LEN-READY-PATH-IMPLEMENTATION-001" \
  "summary=ok"; do
  require_line_in_file "$CARD" "$expected"
done

for stop_line in \
  "do not change indexOf/session cache policy" \
  "do not change store/write paths" \
  "do not change ArrayBox storage" \
  "do not change backend route selection" \
  "do not touch MIRBuilder" \
  "do not infer from benchmark or helper names"; do
  grep -F -q "$stop_line" "$CARD" || {
    echo "[$TAG] missing stop line: $stop_line" >&2
    exit 1
  }
done

echo "[$TAG] ok"
