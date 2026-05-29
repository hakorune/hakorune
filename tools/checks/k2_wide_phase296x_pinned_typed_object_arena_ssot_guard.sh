#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-307-PINNED-TYPED-OBJECT-ARENA-SSOT.md"
SSOT="$ROOT_DIR/docs/development/current/main/design/pinned-typed-object-arena-ssot.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-306-TYPED-OBJECT-DIRECT-SLOT-LEASE-FEASIBILITY.md"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -Fqx "$expected" "$file"; then
    echo "[row307-pinned-typed-object-arena-ssot] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$SSOT" "Status: Active"
require_line "$PREV" "Status: Landed"

require_line "$DOC" "output_contract=pinned-typed-object-arena-ssot-v0"
require_line "$DOC" "input_contract=typed-object-direct-slot-lease-feasibility-v0"
require_line "$DOC" "selected_design_owner=pinned_typed_object_arena"
require_line "$DOC" "selected_reason=current_vec_refcell_store_cannot_support_direct_slot_lease"
require_line "$DOC" "default_backend_unchanged=1"
require_line "$DOC" "existing_helper_abi_unchanged=1"
require_line "$DOC" "pinned_arena_backend_default=0"
require_line "$DOC" "object_storage_pinned_required=1"
require_line "$DOC" "field_address_stable_required=1"
require_line "$DOC" "object_generation_required=1"
require_line "$DOC" "slot_layout_stable_required=1"
require_line "$DOC" "handle_generation_validation_required=1"
require_line "$DOC" "lease_region_required=1"
require_line "$DOC" "lease_barrier_policy_required=1"
require_line "$DOC" "raw_runtime_vec_pointer_exposure_allowed=0"
require_line "$DOC" "silent_fallback_after_lease_selection_allowed=0"
require_line "$DOC" "direct_lowering_before_arena_guard_allowed=0"
require_line "$DOC" "by_name_hako_alloc_special_case_allowed=0"
require_line "$DOC" "first_implementation_boundary=pinned_typed_object_arena_storage_pilot"
require_line "$DOC" "implementation_open=0"
require_line "$DOC" "optimization_open=0"
require_line "$DOC" "winner_claim=0"
require_line "$DOC" "replacement_active=0"
require_line "$DOC" "hook_installed=0"
require_line "$DOC" "global_allocator=0"
require_line "$DOC" "summary=ok"

require_line "$SSOT" "output_contract=pinned-typed-object-arena-ssot-v0"
require_line "$SSOT" "DirectSlotLease requires stable slot access within the lease region:"
require_line "$SSOT" "raw_runtime_vec_pointer_exposure_allowed=0"
require_line "$SSOT" "first_implementation_boundary=pinned_typed_object_arena_storage_pilot"
require_line "$SSOT" "rejected_scope=llvm_lowering_direct_slot_lease_native_direct"

echo "[row307-pinned-typed-object-arena-ssot] ok"
