#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-311-PINNED-TYPED-OBJECT-ARENA-NEXT-LEASE-BOUNDARY-SELECTION.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-310-PINNED-TYPED-OBJECT-ARENA-BACKEND-PILOT.md"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -Fqx "$expected" "$file"; then
    echo "[row311-pinned-arena-next-lease-boundary-selection] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=pinned-typed-object-arena-next-lease-boundary-selection-v0"
require_line "$DOC" "input_contract=pinned-typed-object-arena-backend-pilot-v0"
require_line "$DOC" "selected_next=pinned_arena_exact_slot_helper_compatibility"
require_line "$DOC" "selected_reason=direct_slot_lease_needs_existing_exact_slot_helper_fallback_to_work_with_pinned_backend"
require_line "$DOC" "generic_helper_backend_smoke=ok"
require_line "$DOC" "exact_slot_helper_with_pinned_backend_supported=0"
require_line "$DOC" "direct_slot_lease_guard_ready=0"
require_line "$DOC" "selected_owner=typed_object_store_exact_slot_helper_backend"
require_line "$DOC" "allowed_scope=exact_slot_get_set_rmw_record_helpers_only"
require_line "$DOC" "default_backend_unchanged=1"
require_line "$DOC" "existing_helper_abi_unchanged=1"
require_line "$DOC" "pinned_arena_backend_default=0"
require_line "$DOC" "direct_slot_lease_emission_open=0"
require_line "$DOC" "llvm_lowering_open=0"
require_line "$DOC" "native_direct_open=0"
require_line "$DOC" "optimization_open=0"
require_line "$DOC" "winner_claim=0"
require_line "$DOC" "replacement_active=0"
require_line "$DOC" "hook_installed=0"
require_line "$DOC" "global_allocator=0"
require_line "$DOC" "summary=ok"

echo "[row311-pinned-arena-next-lease-boundary-selection] ok"
