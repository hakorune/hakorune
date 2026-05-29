#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-313-DIRECT-SLOT-LEASE-GUARD-SURFACE.md"
SSOT="$ROOT_DIR/docs/development/current/main/design/direct-slot-lease-guard-surface-ssot.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-312-PINNED-TYPED-OBJECT-ARENA-EXACT-SLOT-HELPER-PILOT.md"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -Fqx "$expected" "$file"; then
    echo "[row313-direct-slot-lease-guard-surface] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$SSOT" "Status: Active"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=direct-slot-lease-guard-surface-v0"
require_line "$DOC" "input_contract=pinned-typed-object-arena-exact-slot-helper-pilot-v0"
require_line "$DOC" "selected_owner=typed_object_direct_slot_lease_guard"
require_line "$DOC" "selected_storage_backend=pinned_arena_exact"
require_line "$DOC" "selected_storage_classes=i64|u64|handle"
require_line "$DOC" "hako_alloc_policy_state_owner=unchanged"
require_line "$DOC" "raw_memory_owner=capability_substrate_or_native_metal"
require_line "$DOC" "representation_owner=compiler_direct_lowering"
require_line "$DOC" "helper_path=fallback_materialization_debug"
require_line "$DOC" "lease_token_runtime_smoke_open=1"
require_line "$DOC" "helper_fallback_required=1"
require_line "$DOC" "materialization_policy_required=1"
require_line "$DOC" "barrier_policy_required=1"
require_line "$DOC" "llvm_lowering_open=0"
require_line "$DOC" "native_direct_open=0"
require_line "$DOC" "direct_load_store_open=0"
require_line "$DOC" "new_c_abi_helper_symbols=0"
require_line "$DOC" "raw_runtime_vec_pointer_exposure=0"
require_line "$DOC" "by_name_hako_alloc_special_case=0"
require_line "$DOC" "silent_fallback_after_lease_selection=0"
require_line "$DOC" "winner_claim=0"
require_line "$DOC" "replacement_active=0"
require_line "$DOC" "hook_installed=0"
require_line "$DOC" "global_allocator=0"
require_line "$DOC" "summary=ok"

require_line "$SSOT" "output_contract=direct-slot-lease-guard-surface-v0"
require_line "$SSOT" "lease selected and backend_is_pinned_arena_exact != 1"
require_line "$SSOT" "DIRECT-SLOT-LEASE-RUNTIME-TOKEN-PILOT-296X-001"

echo "[row313-direct-slot-lease-guard-surface] ok"
