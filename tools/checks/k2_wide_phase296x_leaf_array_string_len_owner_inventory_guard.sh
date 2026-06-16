#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="leaf-array-string-len-owner-inventory"
CARD="docs/development/current/main/phases/phase-296x/296x-845-MIMALLOC-LEAF-ARRAY-STRING-LEN-OWNER-INVENTORY-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-844-MIMALLOC-FRESH-FRONT-SELECTION-AFTER-MAP-MISSING-EMPTY-CLOSEOUT-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_leaf_array_string_len_owner_inventory_guard.sh"

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
  "output_contract=hako-mimalloc-leaf-array-string-len-owner-inventory-v0" \
  "source_evidence=296x-844,microasm-2026-06-16" \
  "row_kind=owner_inventory" \
  "target_front=kilo_leaf_array_string_len" \
  "hako_slower_front=1" \
  "ratio_instr=0.16" \
  "ratio_cycles=0.10" \
  "ratio_ms=0.40" \
  "aot_status=ok" \
  "asm_top_symbol_0=std::thread::local::LocalKey<T>::with" \
  "asm_top_symbol_0_percent=97.52" \
  "asm_sample_count=25" \
  "selected_owner=runtime_tls_boundary_low_confidence" \
  "selected_owner_confidence=low" \
  "array_string_helper_owner_selected=0" \
  "string_length_body_owner_selected=0" \
  "compiler_lowering_owner_selected=0" \
  "implementation_allowed=0" \
  "runtime_tls_boundary_visible=1" \
  "measurement_boundary_confidence=low" \
  "repeat_attribution_required=1" \
  "source_hako_changed=0" \
  "mirbuilder_changed=0" \
  "arraybox_changed=0" \
  "stringbox_changed=0" \
  "runtime_helper_changed=0" \
  "product_default_changed=0" \
  "helper_name_inference_enabled=0" \
  "selected_next=MIMALLOC-LEAF-ARRAY-STRING-LEN-ATTRIBUTION-REPEAT-001" \
  "summary=ok"; do
  require_line_in_file "$CARD" "$expected"
done

for stop_line in \
  "do not patch ArrayBox or StringBox from this low-confidence owner" \
  "do not optimize std::thread::local::LocalKey<T>::with without a runtime owner row" \
  "do not infer string length ownership from benchmark name" \
  "do not touch MIRBuilder" \
  "do not change product runtime defaults" \
  "do not claim a keeper from a 25-sample top report"; do
  grep -F -q "$stop_line" "$CARD" || {
    echo "[$TAG] missing stop line: $stop_line" >&2
    exit 1
  }
done

echo "[$TAG] ok"
