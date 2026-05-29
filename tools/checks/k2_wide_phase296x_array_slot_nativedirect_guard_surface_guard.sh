#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-348-ARRAY-SLOT-NATIVEDIRECT-GUARD-SURFACE.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-347-POST-ARRAY-HANDLE-CACHE-OWNER-REFRESH.md"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -Fqx "$expected" "$file"; then
    echo "[row348-array-slot-nativedirect-guard] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_pattern() {
  local file="$1"
  local pattern="$2"
  if ! grep -Fq "$pattern" "$file"; then
    echo "[row348-array-slot-nativedirect-guard] missing pattern in ${file#$ROOT_DIR/}: $pattern" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=array-slot-nativedirect-guard-surface-v0"
require_line "$DOC" "input_contract=array-post-handle-cache-owner-refresh-v0"
require_line "$DOC" "selected_owner=array_slot_nativedirect"
require_line "$DOC" "selected_reason=array_helper_call_boundary_dominates_after_hash_removed"
require_line "$DOC" "public_arraybox_semantics_unchanged=1"
require_line "$DOC" "default_safe_rwlock_path_unchanged=1"
require_line "$DOC" "plugin_arraybox_public_owner=1"
require_line "$DOC" "single_thread_exact_helper_path=fallback_materialization_debug"
require_line "$DOC" "selected_representation=DirectArrayI64BufferV0"
require_line "$DOC" "element_storage=i64"
require_line "$DOC" "mixed_storage_supported=0"
require_line "$DOC" "boxed_storage_supported=0"
require_line "$DOC" "string_storage_supported=0"
require_line "$DOC" "bool_f64_storage_supported=0"
require_line "$DOC" "direct_i64_load_store_selected=1"
require_line "$DOC" "fused_load_store_selected=0"
require_line "$DOC" "method_local_residence_selected=0"
require_line "$DOC" "runtime_helper_internal_fast_lane_repeat=0"
require_line "$DOC" "public_arraybox_storage_change=0"
require_line "$DOC" "hako_source_workaround=0"
require_line "$DOC" "mirbuilder_changes_allowed=0"
require_line "$DOC" "hako_source_changes_allowed=0"
require_line "$DOC" "required_fact_receiver_array_exact=1"
require_line "$DOC" "required_fact_element_storage_i64=1"
require_line "$DOC" "required_fact_index_i64=1"
require_line "$DOC" "required_fact_bounds_policy_known=1"
require_line "$DOC" "required_fact_append_policy_known=1"
require_line "$DOC" "required_fact_materialization_policy_known=1"
require_line "$DOC" "required_positive_net_helper_delta=1"
require_line "$DOC" "unsupported_storage_policy=no_plan"
require_line "$DOC" "oob_policy=preserve_or_no_plan"
require_line "$DOC" "append_at_end_policy=preserve_if_capacity_known_else_no_plan"
require_line "$DOC" "selected_plan_silent_fallback_allowed=0"
require_line "$DOC" "implementation_open=0"
require_line "$DOC" "llvm_lowering_open=0"
require_line "$DOC" "provider_activation=0"
require_line "$DOC" "host_replacement=0"
require_line "$DOC" "replacement_active=0"
require_line "$DOC" "hook_installed=0"
require_line "$DOC" "global_allocator=0"
require_line "$DOC" "winner_claim=0"
require_line "$DOC" "summary=ok"

require_pattern "$DOC" "PublicArrayBox:"
require_pattern "$DOC" "DirectArrayI64BufferV0:"
require_pattern "$DOC" "Planning-time unsupported shapes produce no NativeDirect plan."
require_pattern "$DOC" "selected_plan_silent_fallback_allowed=0"

cat <<REPORT_TEXT
output_contract=array-slot-nativedirect-guard-surface-v0
input_contract=array-post-handle-cache-owner-refresh-v0
workload_id=representative-object-lifecycle-small-block-v0
selected_owner=array_slot_nativedirect
selected_representation=DirectArrayI64BufferV0
implementation_open=0
llvm_lowering_open=0
provider_activation=0
host_replacement=0
hook_installed=0
global_allocator=0
winner_claim=0
summary=ok
REPORT_TEXT
