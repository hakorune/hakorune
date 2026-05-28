#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-266-PAGE-MODEL-RELEASE-KNOWN-LIVE-SINGLE-USE-RMW-GUARD-SURFACE.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-265-PAGE-MODEL-RELEASE-KNOWN-LIVE-OWNER-SELECTION.md"
PAGE="$ROOT_DIR/lang/src/hako_alloc/memory/page_box.hako"
LOWER="$ROOT_DIR/lang/c-abi/shims/hako_llvmc_ffi_same_module_body_emit.inc"
EXPORT="$ROOT_DIR/crates/nyash_kernel/src/exports/typed_object.rs"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[row266-release-known-live-rmw-surface] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_contains() {
  local file="$1"
  local expected="$2"
  if ! grep -qF "$expected" "$file"; then
    echo "[row266-release-known-live-rmw-surface] missing text in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=page-model-release-known-live-single-use-rmw-guard-surface-v0"
require_line "$DOC" "selected_owner=page_model_release_known_live_single_use_rmw_guard_surface"
require_line "$DOC" "implementation_owner=c_abi_same_module_typed_field_rmw_fusion"
require_line "$DOC" "existing_helper_symbol=nyash.object.exact_slot_rmw_add_u64_hiii"
require_line "$DOC" "new_runtime_helper_required=0"
require_line "$DOC" "candidate_count=2"
require_line "$DOC" "candidate_0_field=local_free_count"
require_line "$DOC" "candidate_0_slot=11"
require_line "$DOC" "candidate_0_storage=usize_u64"
require_line "$DOC" "candidate_1_field=retire_count"
require_line "$DOC" "candidate_1_slot=17"
require_line "$DOC" "candidate_1_storage=usize_u64"
require_line "$DOC" "planned_net_helper_call_delta=2"
require_line "$DOC" "multi_use_rmw_rejected=1"
require_line "$DOC" "array_bridge_rejected=1"
require_line "$DOC" "source_rewrite=0"
require_line "$DOC" "implementation_open=0"
require_line "$DOC" "optimization_open=0"
require_line "$DOC" "winner_claim=0"
require_line "$DOC" "replacement_active=0"
require_line "$DOC" "hook_installed=0"
require_line "$DOC" "global_allocator=0"
require_line "$DOC" "summary=ok"

require_contains "$PAGE" "local_free_count: usize = 0"
require_contains "$PAGE" "retire_count: usize = 0"
require_contains "$LOWER" "nyash.object.exact_slot_rmw_add_u64_hiii"
require_contains "$EXPORT" "nyash.object.exact_slot_rmw_add_u64_hiii"

echo "[row266-release-known-live-rmw-surface] ok"
