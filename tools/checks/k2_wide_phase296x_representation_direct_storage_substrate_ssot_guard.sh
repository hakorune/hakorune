#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-305-REPRESENTATION-DIRECT-STORAGE-SUBSTRATE-SSOT.md"
SSOT="$ROOT_DIR/docs/development/current/main/design/representation-direct-storage-substrate-ssot.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-304-TYPED-OBJECT-RESIDENT-SCALAR-FEASIBILITY-CLOSEOUT.md"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -Fqx "$expected" "$file"; then
    echo "[row305-direct-storage-substrate] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_text() {
  local file="$1"
  local expected="$2"
  if ! grep -Fq "$expected" "$file"; then
    echo "[row305-direct-storage-substrate] missing text in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$SSOT" "Status: Active"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=representation-direct-storage-substrate-ssot-v0"
require_line "$DOC" "selected_design_owner=NativeDirectStorageSubstrate"
require_line "$DOC" "selected_reason=resident_scalar_with_helper_load_writeback_has_net_helper_call_delta_zero"
require_line "$DOC" "addressable_slot_defined=1"
require_line "$DOC" "direct_slot_lease_defined=1"
require_line "$DOC" "materialized_local_struct_defined=1"
require_line "$DOC" "value_aggregate_delta_defined=1"
require_line "$DOC" "native_direct_defined=1"
require_line "$DOC" "raw_runtime_vec_pointer_exposure_allowed=0"
require_line "$DOC" "pinned_storage_required_for_direct_slot_lease=1"
require_line "$DOC" "materialization_policy_required=1"
require_line "$DOC" "net_helper_delta_positive_required=1"
require_line "$DOC" "first_feasibility_candidate_0=typed_object_direct_slot_lease"
require_line "$DOC" "first_feasibility_candidate_1=method_local_stack_aggregate"
require_line "$DOC" "first_feasibility_candidate_2=array_slot_native_direct"
require_line "$DOC" "first_feasibility_candidate_3=result_capsule_value_aggregate_region"
require_line "$DOC" "selected_next=typed_object_direct_slot_lease_feasibility"
require_line "$DOC" "implementation_open=0"
require_line "$DOC" "optimization_open=0"
require_line "$DOC" "winner_claim=0"
require_line "$DOC" "replacement_active=0"
require_line "$DOC" "hook_installed=0"
require_line "$DOC" "global_allocator=0"
require_line "$DOC" "summary=ok"

require_text "$SSOT" "ResidentScalarCache"
require_text "$SSOT" "AddressableSlot"
require_text "$SSOT" "DirectSlotLease"
require_text "$SSOT" "MaterializedLocalStruct"
require_text "$SSOT" "ValueAggregateDelta"
require_text "$SSOT" "NativeDirect"
require_text "$SSOT" "raw_runtime_vec_pointer_exposure_allowed=0"
require_text "$SSOT" "object_storage_pinned=1"

echo "[row305-direct-storage-substrate] ok"
