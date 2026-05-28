#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-207-MIR-ARRAY-SLOT-RESIDENCE-SSOT.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-206-ARRAY-RUNTIME-SINGLE-THREAD-STORE-BACKEND-KEEPER-MEASUREMENT.md"
SSOT="$ROOT_DIR/docs/development/current/main/design/mir-array-slot-residence-ssot.md"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[row207-mir-array-slot-residence] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_text() {
  local file="$1"
  local expected="$2"
  if ! grep -Fq -- "$expected" "$file"; then
    echo "[row207-mir-array-slot-residence] missing text in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Current"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "Decision: provisional"
require_line "$DOC" "mir_array_slot_residence_ssot=accepted"
require_line "$DOC" "runtime_array_backend_floor=measured"
require_line "$DOC" "array_helper_abi_fallback=1"
require_line "$DOC" "transform_open=0"
require_line "$DOC" "positive_net_helper_call_delta_required=1"
require_line "$DOC" "by_name_hako_alloc_special_case=0"
require_line "$DOC" "winner_claim=0"
require_line "$DOC" "replacement_active=0"
require_line "$DOC" "hook_installed=0"
require_line "$DOC" "global_allocator=0"
require_line "$DOC" "summary=ok"

require_line "$SSOT" "mir_array_slot_residence_ssot=accepted"
require_line "$SSOT" "runtime_array_backend_floor=measured"
require_line "$SSOT" "array_helper_abi_fallback=1"
require_line "$SSOT" "transform_open=0"
require_line "$SSOT" "positive_net_helper_call_delta_required=1"
require_line "$SSOT" "by_name_hako_alloc_special_case=0"
require_text "$SSOT" "ArraySlotResidencePlan:"
require_text "$SSOT" "storage_class:"
require_text "$SSOT" "- InlineI64"
require_text "$SSOT" "append_at_end_proven"
require_text "$SSOT" "fallback_helper:"
require_text "$SSOT" "array_runtime_set_idx_i64"
require_text "$SSOT" "array_runtime_get_idx"
require_text "$SSOT" "net_helper_call_delta ="
require_text "$SSOT" "net_helper_call_delta > 0"
require_text "$SSOT" "unknown_call:"
require_text "$SSOT" "boxed_fallback_required:"
require_text "$SSOT" "Do not transform MIR in the SSOT row."

echo "[row207-mir-array-slot-residence] ok"
