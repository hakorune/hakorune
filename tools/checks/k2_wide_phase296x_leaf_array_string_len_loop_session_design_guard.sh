#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="leaf-array-string-len-loop-session-design"
CARD="docs/development/current/main/phases/phase-296x/296x-850-MIMALLOC-LEAF-ARRAY-STRING-LEN-LOOP-SESSION-DESIGN-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-849-MIMALLOC-LEAF-ARRAY-STRING-LEN-ROUTE-SYMBOL-ATTRIBUTION-PROBE-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_leaf_array_string_len_loop_session_design_guard.sh"

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
  "output_contract=hako-mimalloc-leaf-array-string-len-loop-session-design-v0" \
  "source_evidence=296x-849,worker-inventory-2026-06-16" \
  "row_kind=design" \
  "target_front=kilo_leaf_array_string_len" \
  "selected_owner=array_text_loop_session_plan_surface" \
  "selected_owner_confidence=medium" \
  "implementation_allowed=0" \
  "raw_array_text_session_ffi_enabled=0" \
  "raw_arraybox_pointer_ffi_enabled=0" \
  "helper_name_inference_enabled=0" \
  "backend_loop_session_lowering_enabled=0" \
  "mirbuilder_object_management_enabled=0" \
  "product_default_changed=0" \
  "required_plan=ArrayTextLoopSessionPlan" \
  "same_array_handle_required=1" \
  "loop_region_required=1" \
  "read_only_region_required=1" \
  "no_mutation_region_required=1" \
  "no_drop_or_publication_boundary_required=1" \
  "index_domain_guard_required=1" \
  "selected_next=MIMALLOC-ARRAY-TEXT-LOOP-SESSION-PLAN-SURFACE-001" \
  "summary=ok"; do
  require_line_in_file "$CARD" "$expected"
done

for stop_line in \
  "do not pass raw ArrayTextSession or ArrayBox pointers through FFI" \
  "do not implement backend loop-session lowering without ArrayTextLoopSessionPlan" \
  "do not infer from helper aliases" \
  "do not change ArrayBox storage or product runtime defaults" \
  "do not touch MIRBuilder object management" \
  "do not broaden to indexOf/store paths"; do
  grep -F -q "$stop_line" "$CARD" || {
    echo "[$TAG] missing stop line: $stop_line" >&2
    exit 1
  }
done

echo "[$TAG] ok"
