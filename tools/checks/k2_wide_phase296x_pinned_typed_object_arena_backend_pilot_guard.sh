#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-310-PINNED-TYPED-OBJECT-ARENA-BACKEND-PILOT.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-309-PINNED-TYPED-OBJECT-ARENA-BACKEND-SELECTION.md"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -Fqx "$expected" "$file"; then
    echo "[row310-pinned-typed-object-arena-backend-pilot] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=pinned-typed-object-arena-backend-pilot-v0"
require_line "$DOC" "input_contract=pinned-typed-object-arena-backend-selection-v0"
require_line "$DOC" "selected_owner=typed_object_store_backend_selection"
require_line "$DOC" "selected_backend_name=pinned_arena_exact"
require_line "$DOC" "selection_env=HAKO_TYPED_OBJECT_STORE"
require_line "$DOC" "allowed_env_values=safe_mutex|single_thread_exact|pinned_arena_exact"
require_line "$DOC" "default_backend_unchanged=1"
require_line "$DOC" "existing_helper_abi_unchanged=1"
require_line "$DOC" "pinned_arena_backend_default=0"
require_line "$DOC" "pinned_arena_generic_helper_smoke=ok"
require_line "$DOC" "default_backend_smoke=ok"
require_line "$DOC" "invalid_backend_fail_fast=1"
require_line "$DOC" "exact_slot_helper_rewrite_open=0"
require_line "$DOC" "direct_slot_lease_emission_open=0"
require_line "$DOC" "llvm_lowering_open=0"
require_line "$DOC" "native_direct_open=0"
require_line "$DOC" "optimization_open=0"
require_line "$DOC" "winner_claim=0"
require_line "$DOC" "replacement_active=0"
require_line "$DOC" "hook_installed=0"
require_line "$DOC" "global_allocator=0"
require_line "$DOC" "summary=ok"

cargo test -p nyash_kernel typed_object_helpers_store_and_load_i64_slots --quiet
HAKO_TYPED_OBJECT_STORE=pinned_arena_exact \
  cargo test -p nyash_kernel typed_object_helpers_store_and_load_i64_slots --quiet
if HAKO_TYPED_OBJECT_STORE=bad_backend \
  cargo test -p nyash_kernel typed_object_helpers_store_and_load_i64_slots --quiet >/tmp/hakorune_row310_bad_backend.log 2>&1; then
  echo "[row310-pinned-typed-object-arena-backend-pilot] bad backend unexpectedly succeeded" >&2
  exit 1
fi
grep -Fq "[freeze:contract][typed-object-store/backend]" /tmp/hakorune_row310_bad_backend.log

echo "[row310-pinned-typed-object-arena-backend-pilot] ok"
