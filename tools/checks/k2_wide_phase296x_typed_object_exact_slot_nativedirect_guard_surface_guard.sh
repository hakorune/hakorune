#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-653-TYPED-OBJECT-EXACT-SLOT-NATIVEDIRECT-GUARD-SURFACE.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-652-TYPED-OBJECT-EXACT-SLOT-NATIVEDIRECT-READINESS-INVENTORY.md"
TOOL="$ROOT_DIR/tools/allocator/typed_object_exact_slot_nativedirect_guard_surface.py"
TMP_DIR="$(mktemp -d /tmp/hakorune_row653_typed_object_nativedirect_guard_surface.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT
REPORT="$TMP_DIR/report.out"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -Fqx "$expected" "$file"; then
    echo "[row653-typed-object-nativedirect-guard-surface] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=typed-object-exact-slot-nativedirect-guard-surface-v0"
require_line "$DOC" "input_contract=typed-object-exact-slot-nativedirect-readiness-inventory-v0"
require_line "$DOC" "candidate_representation=NativeDirect"
require_line "$DOC" "selected_route=hako.typed_object.slot_load_i64"
require_line "$DOC" "selected_lowering_form=exact_helper_bridge"
require_line "$DOC" "storage_substrate=PinnedTypedObjectArena"
require_line "$DOC" "fallback_boundary=explicit_materialized_view_handle"
require_line "$DOC" "object_storage_pinned_required=1"
require_line "$DOC" "field_address_stable_required=1"
require_line "$DOC" "object_generation_required=1"
require_line "$DOC" "slot_layout_stable_required=1"
require_line "$DOC" "handle_generation_validation_required=1"
require_line "$DOC" "lease_region_required=1"
require_line "$DOC" "lease_barrier_policy_required=1"
require_line "$DOC" "silent_fallback_allowed=0"
require_line "$DOC" "helper_load_writeback_substitution_allowed=0"
require_line "$DOC" "raw_runtime_vec_pointer_exposure_allowed=0"
require_line "$DOC" "by_name_hako_alloc_special_case_allowed=0"
require_line "$DOC" "selected_next=typed_object_exact_slot_nativedirect_pilot_selection"
require_line "$DOC" "summary=ok"

python3 "$TOOL" --out "$REPORT" >/dev/null

require_line "$REPORT" "output_contract=typed-object-exact-slot-nativedirect-guard-surface-v0"
require_line "$REPORT" "input_contract=typed-object-exact-slot-nativedirect-readiness-inventory-v0"
require_line "$REPORT" "candidate_representation=NativeDirect"
require_line "$REPORT" "selected_route=hako.typed_object.slot_load_i64"
require_line "$REPORT" "selected_lowering_form=exact_helper_bridge"
require_line "$REPORT" "storage_substrate=PinnedTypedObjectArena"
require_line "$REPORT" "fallback_boundary=explicit_materialized_view_handle"
require_line "$REPORT" "object_storage_pinned_required=1"
require_line "$REPORT" "field_address_stable_required=1"
require_line "$REPORT" "object_generation_required=1"
require_line "$REPORT" "slot_layout_stable_required=1"
require_line "$REPORT" "handle_generation_validation_required=1"
require_line "$REPORT" "lease_region_required=1"
require_line "$REPORT" "lease_barrier_policy_required=1"
require_line "$REPORT" "silent_fallback_allowed=0"
require_line "$REPORT" "helper_load_writeback_substitution_allowed=0"
require_line "$REPORT" "raw_runtime_vec_pointer_exposure_allowed=0"
require_line "$REPORT" "by_name_hako_alloc_special_case_allowed=0"
require_line "$REPORT" "selected_next=typed_object_exact_slot_nativedirect_pilot_selection"
require_line "$REPORT" "summary=ok"

cat "$REPORT"
