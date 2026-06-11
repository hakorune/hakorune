#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-654-TYPED-OBJECT-EXACT-SLOT-NATIVEDIRECT-PILOT-SELECTION.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-653-TYPED-OBJECT-EXACT-SLOT-NATIVEDIRECT-GUARD-SURFACE.md"
TMP_DIR="$(mktemp -d /tmp/hakorune_row654_typed_object_nativedirect_pilot_selection.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT
REPORT="$TMP_DIR/report.out"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -Fqx "$expected" "$file"; then
    echo "[row654-typed-object-nativedirect-pilot-selection] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=typed-object-exact-slot-nativedirect-pilot-selection-v0"
require_line "$DOC" "input_contract=typed-object-exact-slot-nativedirect-guard-surface-v0"
require_line "$DOC" "candidate_representation=NativeDirect"
require_line "$DOC" "selected_owner=llvm_field_access_typed_object_exact_slot_nativedirect_pilot_selection"
require_line "$DOC" "selected_owner_file=src/llvm_py/instructions/field_access_helpers_typed.py"
require_line "$DOC" "selected_backend=typed_object_exact_slot_nativedirect"
require_line "$DOC" "selected_route=hako.typed_object.slot_load_i64"
require_line "$DOC" "selected_lowering_form=exact_helper_bridge"
require_line "$DOC" "storage_substrate=PinnedTypedObjectArena"
require_line "$DOC" "fallback_boundary=explicit_materialized_view_handle"
require_line "$DOC" "required_facts=object_storage_pinned_required|field_address_stable_required|object_generation_required|slot_layout_stable_required|handle_generation_validation_required|lease_region_required|lease_barrier_policy_required"
require_line "$DOC" "pilot_open=0"
require_line "$DOC" "implementation_open=0"
require_line "$DOC" "optimization_open=0"
require_line "$DOC" "llvm_lowering_open=0"
require_line "$DOC" "native_direct_open=0"
require_line "$DOC" "direct_load_store_open=0"
require_line "$DOC" "selected_next=typed_object_exact_slot_nativedirect_pilot_implementation"
require_line "$DOC" "winner_claim=0"
require_line "$DOC" "replacement_active=0"
require_line "$DOC" "hook_installed=0"
require_line "$DOC" "global_allocator=0"
require_line "$DOC" "summary=ok"

ROOT_DIR="$ROOT_DIR" python3 - <<'PY' >"$REPORT"
import os
import sys
from pathlib import Path

root = Path(os.environ["ROOT_DIR"])
tools = root / "tools" / "hako_check"
if str(tools) not in sys.path:
    sys.path.insert(0, str(tools))

from typed_object_exact_slot_inventory import (
    typed_object_exact_slot_nativedirect_guard_surface_inventory,
    typed_object_exact_slot_nativedirect_pilot_selection_inventory,
)

guard_surface = typed_object_exact_slot_nativedirect_guard_surface_inventory(
    {
        "candidate_representation": "NativeDirect",
        "selected_route": "hako.typed_object.slot_load_i64",
        "selected_lowering_form": "exact_helper_bridge",
        "storage_substrate": "PinnedTypedObjectArena",
        "fallback_boundary": "explicit_materialized_view_handle",
    }
)
report = typed_object_exact_slot_nativedirect_pilot_selection_inventory(guard_surface)
for key in (
    "output_contract",
    "input_contract",
    "candidate_representation",
    "selected_owner",
    "selected_owner_file",
    "selected_backend",
    "selected_route",
    "selected_lowering_form",
    "storage_substrate",
    "fallback_boundary",
    "required_facts",
    "pilot_open",
    "implementation_open",
    "optimization_open",
    "llvm_lowering_open",
    "native_direct_open",
    "direct_load_store_open",
    "selected_next",
    "winner_claim",
    "replacement_active",
    "hook_installed",
    "global_allocator",
    "summary",
):
    print(f"{key}={report[key]}")
PY

require_line "$REPORT" "output_contract=typed-object-exact-slot-nativedirect-pilot-selection-v0"
require_line "$REPORT" "input_contract=typed-object-exact-slot-nativedirect-guard-surface-v0"
require_line "$REPORT" "candidate_representation=NativeDirect"
require_line "$REPORT" "selected_owner=llvm_field_access_typed_object_exact_slot_nativedirect_pilot_selection"
require_line "$REPORT" "selected_owner_file=src/llvm_py/instructions/field_access_helpers_typed.py"
require_line "$REPORT" "selected_backend=typed_object_exact_slot_nativedirect"
require_line "$REPORT" "selected_route=hako.typed_object.slot_load_i64"
require_line "$REPORT" "selected_lowering_form=exact_helper_bridge"
require_line "$REPORT" "storage_substrate=PinnedTypedObjectArena"
require_line "$REPORT" "fallback_boundary=explicit_materialized_view_handle"
require_line "$REPORT" "required_facts=object_storage_pinned_required|field_address_stable_required|object_generation_required|slot_layout_stable_required|handle_generation_validation_required|lease_region_required|lease_barrier_policy_required"
require_line "$REPORT" "pilot_open=0"
require_line "$REPORT" "implementation_open=0"
require_line "$REPORT" "optimization_open=0"
require_line "$REPORT" "llvm_lowering_open=0"
require_line "$REPORT" "native_direct_open=0"
require_line "$REPORT" "direct_load_store_open=0"
require_line "$REPORT" "selected_next=typed_object_exact_slot_nativedirect_pilot_implementation"
require_line "$REPORT" "winner_claim=0"
require_line "$REPORT" "replacement_active=0"
require_line "$REPORT" "hook_installed=0"
require_line "$REPORT" "global_allocator=0"
require_line "$REPORT" "summary=ok"

cat "$REPORT"
