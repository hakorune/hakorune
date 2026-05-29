#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-351-DIRECT-ARRAY-I64-MATERIALIZATION-SYNC-SSOT.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-350-DIRECT-ARRAY-I64-BUFFER-V0-STORAGE-PILOT.md"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -Fqx "$expected" "$file"; then
    echo "[row351-direct-array-i64-sync-ssot] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_pattern() {
  local file="$1"
  local pattern="$2"
  if ! grep -Fq "$pattern" "$file"; then
    echo "[row351-direct-array-i64-sync-ssot] missing pattern in ${file#$ROOT_DIR/}: $pattern" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=direct-array-i64-materialization-sync-ssot-v0"
require_line "$DOC" "input_contract=direct-array-i64-buffer-v0-storage-pilot-v0"
require_line "$DOC" "selected_owner=direct_array_i64_materialization_sync_policy"
require_line "$DOC" "direct_array_primary_storage_policy=selected_native_direct_region"
require_line "$DOC" "public_arraybox_primary_storage_policy=default_runtime_path"
require_line "$DOC" "dual_truth_allowed=0"
require_line "$DOC" "materialized_view_kind=public_arraybox_snapshot"
require_line "$DOC" "materialization_direction=direct_array_to_public_arraybox"
require_line "$DOC" "bootstrap_direction=public_arraybox_to_direct_array_deferred"
require_line "$DOC" "helper_fallback_direction=public_arraybox_only_until_backend_connection"
require_line "$DOC" "direct_array_helper_route_open=0"
require_line "$DOC" "backend_connection_open=0"
require_line "$DOC" "llvm_lowering_open=0"
require_line "$DOC" "native_direct_open=0"
require_line "$DOC" "direct_load_store_open=0"
require_line "$DOC" "public_arraybox_semantics_unchanged=1"
require_line "$DOC" "default_safe_rwlock_path_unchanged=1"
require_line "$DOC" "existing_array_helper_abi_unchanged=1"
require_line "$DOC" "arraybox_items_rwlock_exposure=0"
require_line "$DOC" "array_slot_cache_vec_exposure=0"
require_line "$DOC" "selected_plan_silent_fallback_allowed=0"
require_line "$DOC" "unsupported_storage_policy=no_plan"
require_line "$DOC" "append_grow_policy=no_plan_until_capacity_or_grow_policy"
require_line "$DOC" "oob_policy=preserve_or_no_plan"
require_line "$DOC" "materialization_boundary_public_observer=1"
require_line "$DOC" "materialization_boundary_unknown_escape=1"
require_line "$DOC" "materialization_boundary_generic_array_method=1"
require_line "$DOC" "materialization_boundary_storage_kind_change=1"
require_line "$DOC" "materialization_boundary_capacity_growth_required=1"
require_line "$DOC" "materialization_boundary_debug_or_proof_observer=1"
require_line "$DOC" "generation_validation_required_before_handle_route=1"
require_line "$DOC" "backend_selection_required_before_helper_route=1"
require_line "$DOC" "positive_net_helper_delta_required_before_lowering=1"
require_line "$DOC" "selected_next=direct_array_i64_backend_connection_selection"
require_line "$DOC" "summary=ok"

require_pattern "$DOC" 'The direct buffer is not a cache beside `ArrayBox`.'
require_pattern "$DOC" "DirectArrayI64BufferV0 is primary storage."
require_pattern "$DOC" "Public ArrayBox remains primary storage."
require_pattern "$DOC" "This prevents a split-brain state"

cat <<REPORT_TEXT
output_contract=direct-array-i64-materialization-sync-ssot-v0
input_contract=direct-array-i64-buffer-v0-storage-pilot-v0
direct_array_primary_storage_policy=selected_native_direct_region
dual_truth_allowed=0
backend_connection_open=0
llvm_lowering_open=0
native_direct_open=0
direct_load_store_open=0
summary=ok
REPORT_TEXT
