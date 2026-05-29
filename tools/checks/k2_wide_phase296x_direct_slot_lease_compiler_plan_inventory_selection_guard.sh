#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-315-DIRECT-SLOT-LEASE-COMPILER-PLAN-INVENTORY-SELECTION.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-314-DIRECT-SLOT-LEASE-RUNTIME-TOKEN-PILOT.md"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -Fqx "$expected" "$file"; then
    echo "[row315-direct-slot-lease-compiler-plan-inventory-selection] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=direct-slot-lease-compiler-plan-inventory-selection-v0"
require_line "$DOC" "input_contract=direct-slot-lease-runtime-token-pilot-v0"
require_line "$DOC" "selected_owner=compiler_direct_slot_lease_plan_inventory"
require_line "$DOC" "selected_method=HakoAllocPageModel.acquire_usize/1"
require_line "$DOC" "selected_reason=prior_resident_scalar_plan_had_21_candidate_ops_but_helper_load_writeback_zero_net"
require_line "$DOC" "selected_storage_backend=pinned_arena_exact"
require_line "$DOC" "selected_storage_classes=i64|u64|handle"
require_line "$DOC" "inventory_only=1"
require_line "$DOC" "llvm_lowering_open=0"
require_line "$DOC" "native_direct_open=0"
require_line "$DOC" "direct_load_store_open=0"
require_line "$DOC" "runtime_token_available=1"
require_line "$DOC" "helper_fallback_available=1"
require_line "$DOC" "materialization_policy_required=1"
require_line "$DOC" "positive_net_helper_delta_required=1"
require_line "$DOC" "unknown_barrier_policy_fail_fast=1"
require_line "$DOC" "default_backend_unchanged=1"
require_line "$DOC" "existing_helper_abi_unchanged=1"
require_line "$DOC" "optimization_open=0"
require_line "$DOC" "winner_claim=0"
require_line "$DOC" "replacement_active=0"
require_line "$DOC" "hook_installed=0"
require_line "$DOC" "global_allocator=0"
require_line "$DOC" "summary=ok"

echo "[row315-direct-slot-lease-compiler-plan-inventory-selection] ok"
