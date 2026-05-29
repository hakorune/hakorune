#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-316-DIRECT-SLOT-LEASE-SELECTED-METHOD-INVENTORY.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-315-DIRECT-SLOT-LEASE-COMPILER-PLAN-INVENTORY-SELECTION.md"
TOOL="$ROOT_DIR/tools/allocator/direct_slot_lease_selected_method_inventory.py"
PLAN="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-302-TYPED-OBJECT-RESIDENT-SCALAR-SELECTED-METHOD-PLAN.md"
CLOSEOUT="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-304-TYPED-OBJECT-RESIDENT-SCALAR-FEASIBILITY-CLOSEOUT.md"
TMP_DIR="$(mktemp -d /tmp/hakorune_row316_direct_slot_lease_inventory.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT
REPORT="$TMP_DIR/report.out"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -Fqx "$expected" "$file"; then
    echo "[row316-direct-slot-lease-selected-method-inventory] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=direct-slot-lease-selected-method-inventory-v0"
require_line "$DOC" "input_contract=direct-slot-lease-compiler-plan-inventory-selection-v0"
require_line "$DOC" "selected_method=HakoAllocPageModel.acquire_usize/1"
require_line "$DOC" "candidate_exact_slot_get_count=13"
require_line "$DOC" "candidate_exact_slot_set_count=8"
require_line "$DOC" "candidate_exact_slot_helper_count=21"
require_line "$DOC" "resident_field_key_count=11"
require_line "$DOC" "lease_acquire_count=11"
require_line "$DOC" "lease_acquire_c_abi_helper_count=0"
require_line "$DOC" "materialization_helper_count=0"
require_line "$DOC" "planned_erased_helper_ops=21"
require_line "$DOC" "planned_added_helper_ops=0"
require_line "$DOC" "planned_net_helper_delta=21"
require_line "$DOC" "planned_net_helper_delta_positive=1"
require_line "$DOC" "prior_resident_scalar_net_helper_call_delta=0"
require_line "$DOC" "selected_next=direct_slot_lease_lowering_guard_surface"
require_line "$DOC" "implementation_open=0"
require_line "$DOC" "summary=ok"

"$TOOL" --resident-plan "$PLAN" --feasibility-closeout "$CLOSEOUT" --out "$REPORT"
require_line "$REPORT" "output_contract=direct-slot-lease-selected-method-inventory-v0"
require_line "$REPORT" "candidate_exact_slot_helper_count=21"
require_line "$REPORT" "planned_net_helper_delta=21"
require_line "$REPORT" "prior_resident_scalar_net_helper_call_delta=0"
require_line "$REPORT" "summary=ok"

echo "[row316-direct-slot-lease-selected-method-inventory] ok"
