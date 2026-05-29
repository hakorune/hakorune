#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-314-DIRECT-SLOT-LEASE-RUNTIME-TOKEN-PILOT.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-313-DIRECT-SLOT-LEASE-GUARD-SURFACE.md"
SRC="$ROOT_DIR/crates/nyash_kernel/src/exports/typed_object_pinned_arena.rs"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -Fqx "$expected" "$file"; then
    echo "[row314-direct-slot-lease-runtime-token-pilot] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_text() {
  local file="$1"
  local expected="$2"
  if ! grep -Fq "$expected" "$file"; then
    echo "[row314-direct-slot-lease-runtime-token-pilot] missing text in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=direct-slot-lease-runtime-token-pilot-v0"
require_line "$DOC" "input_contract=direct-slot-lease-guard-surface-v0"
require_line "$DOC" "selected_owner=typed_object_direct_slot_lease_runtime_token"
require_line "$DOC" "selected_storage_backend=pinned_arena_exact"
require_line "$DOC" "selected_storage_classes=i64|u64|handle"
require_line "$DOC" "lease_token_struct=1"
require_line "$DOC" "lease_validate_i64_u64_handle=1"
require_line "$DOC" "lease_read_write_smoke=ok"
require_line "$DOC" "wrong_storage_reject_smoke=ok"
require_line "$DOC" "existing_helper_abi_unchanged=1"
require_line "$DOC" "default_backend_unchanged=1"
require_line "$DOC" "new_c_abi_helper_symbols=0"
require_line "$DOC" "llvm_lowering_open=0"
require_line "$DOC" "native_direct_open=0"
require_line "$DOC" "direct_load_store_open=0"
require_line "$DOC" "optimization_open=0"
require_line "$DOC" "winner_claim=0"
require_line "$DOC" "replacement_active=0"
require_line "$DOC" "hook_installed=0"
require_line "$DOC" "global_allocator=0"
require_line "$DOC" "summary=ok"

require_text "$SRC" "pub(crate) struct DirectSlotLeaseToken {"
require_text "$SRC" "fn direct_slot_lease_token_reads_and_writes_supported_storage() {"
require_text "$SRC" "fn direct_slot_lease_rejects_wrong_storage_class() {"

cargo test -p nyash_kernel direct_slot_lease --quiet

echo "[row314-direct-slot-lease-runtime-token-pilot] ok"
