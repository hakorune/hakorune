#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-656-TYPED-OBJECT-EXACT-SLOT-NATIVEDIRECT-PILOT-ROUTE-SELECTION.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-655-TYPED-OBJECT-EXACT-SLOT-NATIVEDIRECT-PILOT-IMPLEMENTATION.md"
CODE="$ROOT_DIR/src/mir/route_decision.rs"
TEST="$ROOT_DIR/tools/hako_check/tests/test_fastmem_report_key_consistency.py"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -Fqx "$expected" "$file"; then
    echo "[row656-typed-object-nativedirect-pilot-route-selection] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_contains() {
  local file="$1"
  local expected="$2"
  if ! grep -Fq "$expected" "$file"; then
    echo "[row656-typed-object-nativedirect-pilot-route-selection] missing text in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=typed-object-exact-slot-nativedirect-pilot-route-selection-v0"
require_line "$DOC" "input_contract=typed-object-exact-slot-nativedirect-pilot-implementation-v0"
require_line "$DOC" "candidate_representation=NativeDirect"
require_line "$DOC" "selected_owner=mir_route_decision_typed_object_exact_slot_nativedirect_pilot_route_selection"
require_line "$DOC" "selected_owner_file=src/mir/route_decision.rs"
require_line "$DOC" "selected_backend=typed_object_exact_slot_nativedirect"
require_line "$DOC" "selected_route=hako.typed_object.slot_load_i64"
require_line "$DOC" "selected_lowering_form=native_direct"
require_line "$DOC" "selected_bridge_symbol=none"
require_line "$DOC" "storage_substrate=PinnedTypedObjectArena"
require_line "$DOC" "fallback_boundary=explicit_materialized_view_handle"
require_line "$DOC" "required_facts=object_storage_pinned_required|field_address_stable_required|object_generation_required|slot_layout_stable_required|handle_generation_validation_required|lease_region_required|lease_barrier_policy_required"
require_line "$DOC" "native_direct_ready=1"
require_line "$DOC" "pilot_open=1"
require_line "$DOC" "implementation_open=1"
require_line "$DOC" "optimization_open=1"
require_line "$DOC" "llvm_lowering_open=1"
require_line "$DOC" "native_direct_open=1"
require_line "$DOC" "direct_load_store_open=1"
require_line "$DOC" "route_decision_native_direct_supported=1"
require_line "$DOC" "helper_bridge_default_unchanged=1"
require_line "$DOC" "helper_bridge_fallback_removed=0"
require_line "$DOC" "winner_claim=0"
require_line "$DOC" "replacement_active=0"
require_line "$DOC" "hook_installed=0"
require_line "$DOC" "global_allocator=0"
require_line "$DOC" "selected_next=typed_object_exact_slot_nativedirect_native_direct_smoke"
require_line "$DOC" "summary=ok"

require_contains "$CODE" "typed_object_exact_slot_native_direct_ready_for_field("
require_contains "$CODE" "selected_lowering_form: Some(if native_direct_ready {"
require_contains "$CODE" "selected_bridge_symbol: if native_direct_ready {"
require_contains "$CODE" "&module.metadata.direct_state_plans,"

require_contains "$CODE" "route_decision_reports_typed_object_exact_slot_native_direct_when_direct_state_is_ready"
require_contains "$CODE" "selected_lowering_form, Some(\"native_direct\")"
require_contains "$CODE" "selected_bridge_symbol, None"

cargo test -p nyash-rust route_decision_reports_typed_object_exact_slot_native_direct_when_direct_state_is_ready --lib

python3 "$TEST"

cat "$DOC"
