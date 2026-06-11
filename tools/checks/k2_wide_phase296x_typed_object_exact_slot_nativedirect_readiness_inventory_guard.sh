#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-652-TYPED-OBJECT-EXACT-SLOT-NATIVEDIRECT-READINESS-INVENTORY.md"
TOOL="$ROOT_DIR/tools/allocator/typed_object_exact_slot_nativedirect_readiness_inventory.py"
TMP_DIR="$(mktemp -d /tmp/hakorune_row652_typed_object_nativedirect_readiness.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT
REPORT="$TMP_DIR/report.out"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -Fqx "$expected" "$file"; then
    echo "[row652-typed-object-nativedirect-readiness] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$DOC" "output_contract=typed-object-exact-slot-nativedirect-readiness-inventory-v0"
require_line "$DOC" "input_contract=typed-object-exact-slot-direct-helper-measurement-v0"
require_line "$DOC" "candidate_representation=NativeDirect"
require_line "$DOC" "storage_substrate=PinnedTypedObjectArena"
require_line "$DOC" "fallback_boundary=explicit_materialized_view_handle"
require_line "$DOC" "typed_object_direct_state_plan_count=9"
require_line "$DOC" "typed_object_direct_state_field_count=79"
require_line "$DOC" "typed_object_direct_state_selected_count=5"
require_line "$DOC" "typed_object_direct_state_selected_field_count=33"
require_line "$DOC" "typed_object_native_direct_candidate_count=5"
require_line "$DOC" "typed_object_native_direct_ready=0"
require_line "$DOC" "typed_object_native_direct_open=0"
require_line "$DOC" "typed_object_direct_load_store_open=0"
require_line "$DOC" "typed_object_native_direct_selected_next=typed_object_exact_slot_nativedirect_guard_surface"
require_line "$DOC" "typed_object_exact_helper_call_count=726"
require_line "$DOC" "typed_object_exact_lowering_forms=exact_helper_bridge"
require_line "$DOC" "typed_object_exact_internal_dispatch_count=0"
require_line "$DOC" "typed_object_exact_silent_fallback_count=0"
require_line "$DOC" "summary=ok"

python3 "$TOOL" --out "$REPORT" >/dev/null

require_line "$REPORT" "output_contract=typed-object-exact-slot-nativedirect-readiness-inventory-v0"
require_line "$REPORT" "input_contract=typed-object-exact-slot-direct-helper-measurement-v0"
require_line "$REPORT" "workload_id=representative-object-lifecycle-small-block-v0"
require_line "$REPORT" "candidate_representation=NativeDirect"
require_line "$REPORT" "storage_substrate=PinnedTypedObjectArena"
require_line "$REPORT" "fallback_boundary=explicit_materialized_view_handle"
require_line "$REPORT" "typed_object_direct_state_plan_count=9"
require_line "$REPORT" "typed_object_direct_state_field_count=79"
require_line "$REPORT" "typed_object_direct_state_selected_count=5"
require_line "$REPORT" "typed_object_direct_state_selected_field_count=33"
require_line "$REPORT" "typed_object_native_direct_candidate_count=5"
require_line "$REPORT" "typed_object_native_direct_ready=0"
require_line "$REPORT" "typed_object_native_direct_open=0"
require_line "$REPORT" "typed_object_direct_load_store_open=0"
require_line "$REPORT" "typed_object_native_direct_selected_next=typed_object_exact_slot_nativedirect_guard_surface"
require_line "$REPORT" "typed_object_exact_helper_call_count=726"
require_line "$REPORT" "typed_object_exact_lowering_forms=exact_helper_bridge"
require_line "$REPORT" "typed_object_exact_internal_dispatch_count=0"
require_line "$REPORT" "typed_object_exact_silent_fallback_count=0"
require_line "$REPORT" "summary=ok"

cat "$REPORT"
