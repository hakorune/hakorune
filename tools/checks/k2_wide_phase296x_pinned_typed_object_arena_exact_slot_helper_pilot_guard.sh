#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-312-PINNED-TYPED-OBJECT-ARENA-EXACT-SLOT-HELPER-PILOT.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-311-PINNED-TYPED-OBJECT-ARENA-NEXT-LEASE-BOUNDARY-SELECTION.md"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -Fqx "$expected" "$file"; then
    echo "[row312-pinned-arena-exact-slot-helper-pilot] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=pinned-typed-object-arena-exact-slot-helper-pilot-v0"
require_line "$DOC" "input_contract=pinned-typed-object-arena-next-lease-boundary-selection-v0"
require_line "$DOC" "selected_owner=typed_object_store_exact_slot_helper_backend"
require_line "$DOC" "selected_backend_name=pinned_arena_exact"
require_line "$DOC" "exact_slot_helper_with_pinned_backend_supported=1"
require_line "$DOC" "generic_helper_backend_smoke=ok"
require_line "$DOC" "exact_slot_helper_smoke=ok"
require_line "$DOC" "default_backend_unchanged=1"
require_line "$DOC" "existing_helper_abi_unchanged=1"
require_line "$DOC" "new_helper_symbol_count=0"
require_line "$DOC" "direct_slot_lease_emission_open=0"
require_line "$DOC" "llvm_lowering_open=0"
require_line "$DOC" "native_direct_open=0"
require_line "$DOC" "optimization_open=0"
require_line "$DOC" "winner_claim=0"
require_line "$DOC" "replacement_active=0"
require_line "$DOC" "hook_installed=0"
require_line "$DOC" "global_allocator=0"
require_line "$DOC" "summary=ok"

HAKO_TYPED_OBJECT_STORE=pinned_arena_exact \
  cargo test -p nyash_kernel pinned_arena_exact_slot_helpers_roundtrip_when_selected --quiet

echo "[row312-pinned-arena-exact-slot-helper-pilot] ok"
