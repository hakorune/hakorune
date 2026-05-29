#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-309-PINNED-TYPED-OBJECT-ARENA-BACKEND-SELECTION.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-308-PINNED-TYPED-OBJECT-ARENA-STORAGE-PILOT.md"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -Fqx "$expected" "$file"; then
    echo "[row309-pinned-typed-object-arena-backend-selection] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=pinned-typed-object-arena-backend-selection-v0"
require_line "$DOC" "input_contract=pinned-typed-object-arena-storage-pilot-v0"
require_line "$DOC" "selected_owner=typed_object_store_backend_selection"
require_line "$DOC" "selected_backend_name=pinned_arena_exact"
require_line "$DOC" "selection_env=HAKO_TYPED_OBJECT_STORE"
require_line "$DOC" "allowed_env_values=safe_mutex|single_thread_exact|pinned_arena_exact"
require_line "$DOC" "default_backend_unchanged=1"
require_line "$DOC" "existing_helper_abi_unchanged=1"
require_line "$DOC" "pinned_arena_backend_default=0"
require_line "$DOC" "allowed_code_owner=crates/nyash_kernel/src/exports/typed_object_store.rs"
require_line "$DOC" "allowed_storage_owner=crates/nyash_kernel/src/exports/typed_object_pinned_arena.rs"
require_line "$DOC" "allowed_export_owner=crates/nyash_kernel/src/exports/typed_object.rs"
require_line "$DOC" "selected_helper_scope=generic_typed_object_helpers_only"
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

echo "[row309-pinned-typed-object-arena-backend-selection] ok"
