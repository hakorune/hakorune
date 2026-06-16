#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="array-text-loop-session-plan-surface"
CARD="docs/development/current/main/phases/phase-296x/296x-851-MIMALLOC-ARRAY-TEXT-LOOP-SESSION-PLAN-SURFACE-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-850-MIMALLOC-LEAF-ARRAY-STRING-LEN-LOOP-SESSION-DESIGN-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_array_text_loop_session_plan_surface_guard.sh"
PLAN="src/mir/array_text_loop_session_plan.rs"
MIR_MOD="src/mir/mod.rs"

for file in "$CARD" "$PREV_CARD" "$PLAN" "$MIR_MOD"; do
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

for token in \
  "pub struct ArrayTextLoopSessionPlan" \
  "pub enum ArrayTextLoopSessionRejectReason" \
  "backend_session_lowering_allowed" \
  "same_array_handle" \
  "read_only_region" \
  "no_mutation_region" \
  "no_drop_or_publication_boundary" \
  "index_domain_guarded"; do
  grep -F -q "$token" "$PLAN" || {
    echo "[$TAG] missing plan token: $token" >&2
    exit 1
  }
done

grep -F -q "pub mod array_text_loop_session_plan;" "$MIR_MOD" || {
  echo "[$TAG] MIR root facade missing array_text_loop_session_plan" >&2
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
  "output_contract=hako-mimalloc-array-text-loop-session-plan-surface-v0" \
  "source_evidence=296x-850" \
  "row_kind=passive_surface" \
  "target_front=kilo_leaf_array_string_len" \
  "plan_file=src/mir/array_text_loop_session_plan.rs" \
  "plan_type=ArrayTextLoopSessionPlan" \
  "reject_type=ArrayTextLoopSessionRejectReason" \
  "same_array_handle_required=1" \
  "read_only_region_required=1" \
  "no_mutation_region_required=1" \
  "no_drop_or_publication_boundary_required=1" \
  "index_domain_guard_required=1" \
  "metadata_refresh_enabled=0" \
  "mir_json_export_enabled=0" \
  "backend_consumer_enabled=0" \
  "backend_loop_session_lowering_enabled=0" \
  "raw_array_text_session_ffi_enabled=0" \
  "raw_arraybox_pointer_ffi_enabled=0" \
  "product_default_changed=0" \
  "selected_next=MIMALLOC-ARRAY-TEXT-LOOP-SESSION-PLAN-INVENTORY-001" \
  "summary=ok"; do
  require_line_in_file "$CARD" "$expected"
done

for stop_line in \
  "do not add refresh or backend consumer in this row" \
  "do not export ArrayTextLoopSessionPlan to MIR JSON yet" \
  "do not pass raw ArrayTextSession or ArrayBox pointers through FFI" \
  "do not change ArrayBox storage or product runtime defaults" \
  "do not broaden to indexOf/store paths"; do
  grep -F -q "$stop_line" "$CARD" || {
    echo "[$TAG] missing stop line: $stop_line" >&2
    exit 1
  }
done

echo "[$TAG] ok"
