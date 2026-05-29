#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-308-PINNED-TYPED-OBJECT-ARENA-STORAGE-PILOT.md"
SSOT="$ROOT_DIR/docs/development/current/main/design/pinned-typed-object-arena-ssot.md"
SRC="$ROOT_DIR/crates/nyash_kernel/src/exports/typed_object_pinned_arena.rs"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -Fqx "$expected" "$file"; then
    echo "[row308-pinned-typed-object-arena-storage-pilot] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_text() {
  local file="$1"
  local expected="$2"
  if ! grep -Fq "$expected" "$file"; then
    echo "[row308-pinned-typed-object-arena-storage-pilot] missing text in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$SSOT" "Status: Active"
require_line "$DOC" "output_contract=pinned-typed-object-arena-storage-pilot-v0"
require_line "$DOC" "input_contract=pinned-typed-object-arena-ssot-v0"
require_line "$DOC" "selected_owner=typed_object_runtime_storage"
require_line "$DOC" "new_storage_box=typed_object_pinned_arena"
require_line "$DOC" "default_backend_unchanged=1"
require_line "$DOC" "existing_helper_abi_unchanged=1"
require_line "$DOC" "pinned_arena_backend_default=0"
require_line "$DOC" "pinned_object_allocation_smoke=ok"
require_line "$DOC" "generation_validation_smoke=ok"
require_line "$DOC" "slot_stability_smoke=ok"
require_line "$DOC" "direct_lowering_open=0"
require_line "$DOC" "direct_slot_lease_emission_open=0"
require_line "$DOC" "native_direct_open=0"
require_line "$DOC" "optimization_open=0"
require_line "$DOC" "winner_claim=0"
require_line "$DOC" "replacement_active=0"
require_line "$DOC" "hook_installed=0"
require_line "$DOC" "global_allocator=0"
require_line "$DOC" "summary=ok"

require_text "$SRC" "pub(crate) struct PinnedTypedObjectArena {"
require_text "$SRC" "fn pinned_arena_allocates_generation_checked_negative_handles() {"
require_text "$SRC" "fn pinned_arena_keeps_slot_address_stable_across_mutation() {"

cargo test -p nyash_kernel typed_object_pinned_arena --quiet

echo "[row308-pinned-typed-object-arena-storage-pilot] ok"
