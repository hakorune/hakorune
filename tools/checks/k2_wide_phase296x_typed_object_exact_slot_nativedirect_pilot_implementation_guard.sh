#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-655-TYPED-OBJECT-EXACT-SLOT-NATIVEDIRECT-PILOT-IMPLEMENTATION.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-654-TYPED-OBJECT-EXACT-SLOT-NATIVEDIRECT-PILOT-SELECTION.md"
CODE="$ROOT_DIR/src/llvm_py/instructions/field_access_helpers_typed.py"
TEST="$ROOT_DIR/src/llvm_py/tests/test_typed_user_box_field_access.py"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -Fqx "$expected" "$file"; then
    echo "[row655-typed-object-nativedirect-pilot-implementation] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_contains() {
  local file="$1"
  local expected="$2"
  if ! grep -Fq "$expected" "$file"; then
    echo "[row655-typed-object-nativedirect-pilot-implementation] missing text in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=typed-object-exact-slot-nativedirect-pilot-implementation-v0"
require_line "$DOC" "input_contract=typed-object-exact-slot-nativedirect-pilot-selection-v0"
require_line "$DOC" "selected_owner=llvm_field_access_typed_object_exact_slot_nativedirect_pilot_implementation"
require_line "$DOC" "selected_owner_file=src/llvm_py/instructions/field_access_helpers_typed.py"
require_line "$DOC" "selected_backend=typed_object_exact_slot_nativedirect"
require_line "$DOC" "selected_route=hako.typed_object.slot_load_i64"
require_line "$DOC" "selected_lowering_form=native_direct"
require_line "$DOC" "storage_substrate=PinnedTypedObjectArena"
require_line "$DOC" "fallback_boundary=explicit_materialized_view_handle"
require_line "$DOC" "required_facts=object_storage_pinned_required|field_address_stable_required|object_generation_required|slot_layout_stable_required|handle_generation_validation_required|lease_region_required|lease_barrier_policy_required"
require_line "$DOC" "implementation_open=1"
require_line "$DOC" "pilot_open=0"
require_line "$DOC" "optimization_open=0"
require_line "$DOC" "llvm_lowering_open=1"
require_line "$DOC" "native_direct_open=0"
require_line "$DOC" "direct_load_store_open=1"
require_line "$DOC" "route_decision_native_direct_supported=1"
require_line "$DOC" "helper_bridge_default_unchanged=1"
require_line "$DOC" "helper_bridge_fallback_removed=0"
require_line "$DOC" "winner_claim=0"
require_line "$DOC" "replacement_active=0"
require_line "$DOC" "hook_installed=0"
require_line "$DOC" "global_allocator=0"
require_line "$DOC" "selected_next=typed_object_exact_slot_nativedirect_pilot_route_selection"
require_line "$DOC" "summary=ok"

require_contains "$CODE" "def _selected_typed_object_exact_slot_native_direct_route_decision("
require_contains "$CODE" "def _lower_exact_slot_native_direct_get("
require_contains "$CODE" "def _lower_exact_slot_native_direct_set("
require_contains "$CODE" "native_direct_route = _selected_typed_object_exact_slot_native_direct_route_decision("
require_contains "$CODE" "return _lower_exact_slot_native_direct_get("
require_contains "$CODE" "return _lower_exact_slot_native_direct_set("

require_contains "$TEST" "def test_native_direct_exact_slot_route_decision_loads_payload("
require_contains "$TEST" "def test_native_direct_exact_slot_route_decision_stores_payload("
require_contains "$TEST" "def test_native_direct_exact_slot_route_decision_helper_selects_native_direct("

python3 "$TEST"

cat "$DOC"
