#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-326-DIRECT-SLOT-OBJECT-BACKEND-CONNECTION-PILOT.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-325-DIRECT-SLOT-OBJECT-BACKEND-CONNECTION-SELECTION.md"
STORE="$ROOT_DIR/crates/nyash_kernel/src/exports/typed_object_store.rs"
ARENA="$ROOT_DIR/crates/nyash_kernel/src/exports/typed_object_pinned_arena.rs"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -Fqx "$expected" "$file"; then
    echo "[row326-direct-slot-object-backend-connection-pilot] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_pattern() {
  local file="$1"
  local pattern="$2"
  if ! grep -Fq "$pattern" "$file"; then
    echo "[row326-direct-slot-object-backend-connection-pilot] missing pattern in ${file#$ROOT_DIR/}: $pattern" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=direct-slot-object-backend-connection-pilot-v0"
require_line "$DOC" "input_contract=direct-slot-object-backend-connection-selection-v0"
require_line "$DOC" "implemented_backend=direct_slot_exact"
require_line "$DOC" "direct_slot_object_allocation_smoke=ok"
require_line "$DOC" "tagged_pointer_handle_smoke=ok"
require_line "$DOC" "generic_helper_route_to_direct_backend=0"
require_line "$DOC" "exact_slot_helper_route_to_direct_backend=0"
require_line "$DOC" "materialization_bridge_implemented=0"
require_line "$DOC" "fallback_bridge_implemented=0"
require_line "$DOC" "existing_helper_abi_unchanged=1"
require_line "$DOC" "llvm_lowering_open=0"
require_line "$DOC" "new_c_abi_helper_symbols=0"
require_line "$DOC" "summary=ok"

require_pattern "$STORE" "DirectSlotExact"
require_pattern "$STORE" "Some(\"direct_slot_exact\")"
require_pattern "$STORE" "DIRECT_SLOT_OBJECTS"
require_pattern "$STORE" "DirectSlotObjectV0Box::from_typed_object"
require_pattern "$STORE" "fn direct_slot_exact_new_object_returns_tagged_pointer_handle"
require_pattern "$ARENA" "pub(crate) fn from_typed_object"

HAKO_TYPED_OBJECT_STORE=direct_slot_exact \
  cargo test -p nyash_kernel direct_slot_exact_new_object_returns_tagged_pointer_handle -- --nocapture

echo "[row326-direct-slot-object-backend-connection-pilot] ok"
