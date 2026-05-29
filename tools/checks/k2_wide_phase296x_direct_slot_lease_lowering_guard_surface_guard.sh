#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-317-DIRECT-SLOT-LEASE-LOWERING-GUARD-SURFACE.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-316-DIRECT-SLOT-LEASE-SELECTED-METHOD-INVENTORY.md"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -Fqx "$expected" "$file"; then
    echo "[row317-direct-slot-lease-lowering-guard-surface] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=direct-slot-lease-lowering-guard-surface-v0"
require_line "$DOC" "input_contract=direct-slot-lease-selected-method-inventory-v0"
require_line "$DOC" "selected_owner=compiler_direct_slot_lease_lowering_guard"
require_line "$DOC" "selected_method=HakoAllocPageModel.acquire_usize/1"
require_line "$DOC" "candidate_representation=NativeDirectViaDirectSlotLease"
require_line "$DOC" "planned_erased_helper_ops=21"
require_line "$DOC" "planned_added_helper_ops=0"
require_line "$DOC" "planned_net_helper_delta=21"
require_line "$DOC" "planned_net_helper_delta_positive=1"
require_line "$DOC" "lease_acquire_c_abi_helper_count_required=0"
require_line "$DOC" "materialization_helper_count_required=0"
require_line "$DOC" "selected_method_only=1"
require_line "$DOC" "receiver_exact_plan_required=1"
require_line "$DOC" "slot_constant_required=1"
require_line "$DOC" "storage_class_exact_required=1"
require_line "$DOC" "pinned_arena_exact_backend_required=1"
require_line "$DOC" "addressable_slot_bridge_required=1"
require_line "$DOC" "unknown_call_barrier_policy=no_plan"
require_line "$DOC" "selected_plan_silent_fallback_allowed=0"
require_line "$DOC" "default_backend_exact_lease_emission=0"
require_line "$DOC" "new_c_abi_helper_symbols=0"
require_line "$DOC" "raw_runtime_vec_pointer_exposure=0"
require_line "$DOC" "by_name_hako_alloc_special_case=0"
require_line "$DOC" "llvm_lowering_open=0"
require_line "$DOC" "native_direct_open=0"
require_line "$DOC" "direct_load_store_open=0"
require_line "$DOC" "implementation_open=0"
require_line "$DOC" "selected_next=direct_slot_lease_lowering_pilot"
require_line "$DOC" "summary=ok"

echo "[row317-direct-slot-lease-lowering-guard-surface] ok"
