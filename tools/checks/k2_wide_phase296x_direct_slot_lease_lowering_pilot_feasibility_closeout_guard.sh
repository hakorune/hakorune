#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-318-DIRECT-SLOT-LEASE-LOWERING-PILOT-FEASIBILITY-CLOSEOUT.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-317-DIRECT-SLOT-LEASE-LOWERING-GUARD-SURFACE.md"
FIELD_ACCESS="$ROOT_DIR/src/llvm_py/instructions/field_access.py"
STORE="$ROOT_DIR/crates/nyash_kernel/src/exports/typed_object_store.rs"
ARENA="$ROOT_DIR/crates/nyash_kernel/src/exports/typed_object_pinned_arena.rs"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -Fqx "$expected" "$file"; then
    echo "[row318-direct-slot-lease-lowering-pilot-feasibility] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_pattern() {
  local file="$1"
  local pattern="$2"
  if ! grep -Fq "$pattern" "$file"; then
    echo "[row318-direct-slot-lease-lowering-pilot-feasibility] missing pattern in ${file#$ROOT_DIR/}: $pattern" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=direct-slot-lease-lowering-pilot-feasibility-v0"
require_line "$DOC" "input_contract=direct-slot-lease-lowering-guard-surface-v0"
require_line "$DOC" "selected_method=HakoAllocPageModel.acquire_usize/1"
require_line "$DOC" "current_exact_lowering_owner=src/llvm_py/instructions/field_access.py"
require_line "$DOC" "current_exact_lowering_path=exact_slot_helper_call"
require_line "$DOC" "runtime_lease_token_visibility=rust_internal_only"
require_line "$DOC" "runtime_arena_location=thread_local_refcell_pinned_arena"
require_line "$DOC" "addressable_slot_bridge_available=0"
require_line "$DOC" "helper_free_bridge_possible_now=0"
require_line "$DOC" "new_c_abi_helper_symbols_allowed=0"
require_line "$DOC" "raw_runtime_vec_pointer_exposure_allowed=0"
require_line "$DOC" "row317_selected_plan_silent_fallback_allowed=0"
require_line "$DOC" "pilot_codegen_opened=0"
require_line "$DOC" "pilot_implemented=0"
require_line "$DOC" "rejection_reason=missing_helper_free_addressable_slot_bridge"
require_line "$DOC" "selected_next=direct_slot_lease_addressable_slot_bridge_ssot"
require_line "$DOC" "summary=ok"

require_pattern "$FIELD_ACCESS" "nyash.object.exact_slot_get_u64_hii"
require_pattern "$FIELD_ACCESS" "nyash.object.exact_slot_set_u64_hiu"
require_pattern "$STORE" "static PINNED_ARENA_OBJECTS: RefCell<PinnedTypedObjectArena>"
require_pattern "$ARENA" "pub(crate) struct DirectSlotLeaseToken"
require_pattern "$ARENA" "pub(crate) fn lease_slot"

echo "[row318-direct-slot-lease-lowering-pilot-feasibility] ok"
