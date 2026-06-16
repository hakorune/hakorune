#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="leaf-array-string-len-route-symbol-attribution-probe"
CARD="docs/development/current/main/phases/phase-296x/296x-849-MIMALLOC-LEAF-ARRAY-STRING-LEN-ROUTE-SYMBOL-ATTRIBUTION-PROBE-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-848-MIMALLOC-LEAF-ARRAY-STRING-LEN-NEXT-OWNER-SELECTION-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_leaf_array_string_len_route_symbol_attribution_probe_guard.sh"

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
  "output_contract=hako-mimalloc-leaf-array-string-len-route-symbol-attribution-probe-v0" \
  "source_evidence=296x-848,nm-objdump-2026-06-16,worker-inventory-2026-06-16" \
  "row_kind=attribution_probe" \
  "target_front=kilo_leaf_array_string_len" \
  "ny_main_hot_loop_call_address=0x414930" \
  "nyash_array_string_len_hi_address=0x414930" \
  "hako_array_text_slot_len_address=0x414930" \
  "route_aliases_share_address=1" \
  "route_aliases_share_body=1" \
  "alias_symbol_spelling_is_owner_evidence=0" \
  "selected_owner=array_text_slot_len_loop_local_session_boundary" \
  "selected_owner_confidence=medium" \
  "implementation_allowed=0" \
  "source_hako_changed=0" \
  "mirbuilder_changed=0" \
  "backend_lowering_changed=0" \
  "product_default_changed=0" \
  "helper_name_inference_enabled=0" \
  "selected_next=MIMALLOC-LEAF-ARRAY-STRING-LEN-LOOP-SESSION-DESIGN-001" \
  "summary=ok"; do
  require_line_in_file "$CARD" "$expected"
done

for stop_line in \
  "do not choose owner from alias spelling" \
  "do not patch alias exports" \
  "do not change C shim route selection from this probe" \
  "do not patch ArrayBox storage" \
  "do not reapply ready-path change" \
  "do not broaden to indexOf/store paths"; do
  grep -F -q "$stop_line" "$CARD" || {
    echo "[$TAG] missing stop line: $stop_line" >&2
    exit 1
  }
done

echo "[$TAG] ok"
