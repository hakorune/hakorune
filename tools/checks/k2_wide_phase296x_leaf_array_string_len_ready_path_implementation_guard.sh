#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="leaf-array-string-len-ready-path-implementation"
CARD="docs/development/current/main/phases/phase-296x/296x-847-MIMALLOC-LEAF-ARRAY-STRING-LEN-READY-PATH-IMPLEMENTATION-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-846-MIMALLOC-LEAF-ARRAY-STRING-LEN-ATTRIBUTION-REPEAT-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_leaf_array_string_len_ready_path_implementation_guard.sh"
ARRAY_CACHE="crates/nyash_kernel/src/plugin/array_handle_cache.rs"
ARRAY_STRING="crates/nyash_kernel/src/plugin/array_string_slot_indexof.rs"

for file in "$CARD" "$PREV_CARD" "$ARRAY_CACHE" "$ARRAY_STRING"; do
  [[ -f "$file" ]] || { echo "[$TAG] missing file: $file" >&2; exit 1; }
done

for file in "$CARD" "$PREV_CARD"; do
  grep -q '^Status: Landed$' "$file" || {
    echo "[$TAG] card must be Landed: $file" >&2
    exit 1
  }
done

grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[$TAG] check index missing guard entry" >&2
  exit 1
}

if grep -q "with_array_text_session_ready" "$ARRAY_CACHE" "$ARRAY_STRING"; then
  echo "[$TAG] rejected ready-path helper still present after rollback" >&2
  exit 1
fi

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[$TAG] missing line in $file: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-mimalloc-leaf-array-string-len-ready-path-implementation-v0" \
  "source_evidence=296x-846,ready-path-attempt-2026-06-16" \
  "row_kind=implementation_nonkeeper" \
  "target_front=kilo_leaf_array_string_len" \
  "implementation_attempted=1" \
  "implementation_kept=0" \
  "implementation_reverted=1" \
  "keeper_claim=0" \
  "nonkeeper_reason=cycles_and_wall_time_regressed" \
  "before_ny_aot_instr=92925832" \
  "before_ny_aot_cycles=32183346" \
  "before_ny_aot_ms=10" \
  "after_ny_aot_instr=89325806" \
  "after_ny_aot_cycles=54242794" \
  "after_ny_aot_ms=13" \
  "arraybox_storage_changed=0" \
  "backend_route_changed=0" \
  "mirbuilder_changed=0" \
  "product_default_changed=0" \
  "helper_name_inference_enabled=0" \
  "selected_next=MIMALLOC-LEAF-ARRAY-STRING-LEN-NEXT-OWNER-SELECTION-001" \
  "summary=ok"; do
  require_line_in_file "$CARD" "$expected"
done

for stop_line in \
  "do not reapply with_array_text_session_ready without a new owner row" \
  "do not keep an instruction-only win when cycles and wall time regress" \
  "do not broaden this into indexOf or store paths" \
  "do not patch MIRBuilder or backend route selection from this nonkeeper" \
  "do not claim ready-path keeper for array string len"; do
  grep -F -q "$stop_line" "$CARD" || {
    echo "[$TAG] missing stop line: $stop_line" >&2
    exit 1
  }
done

echo "[$TAG] ok"
