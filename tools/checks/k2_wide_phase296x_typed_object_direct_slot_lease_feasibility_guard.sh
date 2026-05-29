#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-306-TYPED-OBJECT-DIRECT-SLOT-LEASE-FEASIBILITY.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-305-REPRESENTATION-DIRECT-STORAGE-SUBSTRATE-SSOT.md"
TOOL="$ROOT_DIR/tools/allocator/typed_object_direct_slot_lease_feasibility.py"
TMP_DIR="$(mktemp -d /tmp/hakorune_row306_direct_slot_lease.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

REPORT="$TMP_DIR/report.out"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -Fqx "$expected" "$file"; then
    echo "[row306-direct-slot-lease-feasibility] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=typed-object-direct-slot-lease-feasibility-v0"
require_line "$DOC" "current_store_kind=safe_mutex_or_single_thread_refcell_vec"
require_line "$DOC" "object_storage_container=Vec<TypedSlotObject>"
require_line "$DOC" "field_storage_container=Vec<TypedSlot>"
require_line "$DOC" "object_generation_available=0"
require_line "$DOC" "object_storage_pinned=0"
require_line "$DOC" "field_address_stable=0"
require_line "$DOC" "vec_reallocation_possible=1"
require_line "$DOC" "borrow_lifetime_representable_in_llvm=0"
require_line "$DOC" "direct_slot_lease_feasible_without_storage_change=0"
require_line "$DOC" "raw_runtime_vec_pointer_exposure_allowed=0"
require_line "$DOC" "required_runtime_storage_change=pinned_typed_object_arena"
require_line "$DOC" "selected_next=pinned_typed_object_arena_ssot"
require_line "$DOC" "implementation_open=0"
require_line "$DOC" "optimization_open=0"
require_line "$DOC" "winner_claim=0"
require_line "$DOC" "replacement_active=0"
require_line "$DOC" "hook_installed=0"
require_line "$DOC" "global_allocator=0"
require_line "$DOC" "summary=ok"

"$TOOL" --out "$REPORT" >/tmp/hakorune_row306_direct_slot_lease.log

require_line "$REPORT" "output_contract=typed-object-direct-slot-lease-feasibility-v0"
require_line "$REPORT" "object_storage_pinned=0"
require_line "$REPORT" "field_address_stable=0"
require_line "$REPORT" "vec_reallocation_possible=1"
require_line "$REPORT" "direct_slot_lease_feasible_without_storage_change=0"
require_line "$REPORT" "required_runtime_storage_change=pinned_typed_object_arena"
require_line "$REPORT" "selected_next=pinned_typed_object_arena_ssot"
require_line "$REPORT" "summary=ok"

echo "[row306-direct-slot-lease-feasibility] ok"
