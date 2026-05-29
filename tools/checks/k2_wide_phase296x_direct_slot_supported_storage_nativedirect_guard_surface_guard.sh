#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-343-DIRECT-SLOT-SUPPORTED-STORAGE-NATIVEDIRECT-GUARD-SURFACE.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-342-POST-DIRECT-SLOT-BOOTSTRAP-OWNER-REFRESH.md"
SRC="$ROOT_DIR/lang/c-abi/shims/hako_llvmc_ffi_same_module_typed_object_emit.inc"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -Fqx "$expected" "$file"; then
    echo "[row343-direct-slot-supported-storage-guard] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_pattern() {
  local file="$1"
  local pattern="$2"
  if ! grep -Fq "$pattern" "$file"; then
    echo "[row343-direct-slot-supported-storage-guard] missing pattern in ${file#$ROOT_DIR/}: $pattern" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=direct-slot-supported-storage-nativedirect-guard-surface-v0"
require_line "$DOC" "input_contract=direct-slot-post-bootstrap-owner-refresh-v0"
require_line "$DOC" "selected_owner=ny_llvmc_boundary_same_module_typed_object_emit"
require_line "$DOC" "selected_owner_file=lang/c-abi/shims/hako_llvmc_ffi_same_module_typed_object_emit.inc"
require_line "$DOC" "selected_backend=direct_slot_exact"
require_line "$DOC" "selection_kind=fact_driven_supported_storage"
require_line "$DOC" "selected_method_only=0"
require_line "$DOC" "by_name_hako_alloc_special_case=0"
require_line "$DOC" "required_receiver_fact=typed_object_binding"
require_line "$DOC" "required_slot_fact=typed_object_plan_runtime_slot"
require_line "$DOC" "required_storage_fact=typed_object_plan_storage"
require_line "$DOC" "supported_storage=i64,u64,usize,handle"
require_line "$DOC" "unsupported_storage_policy=existing_helper_route"
require_line "$DOC" "unsupported_narrow_integer_direct_store=0"
require_line "$DOC" "legacy_field_helper_internal_fast_lane=0"
require_line "$DOC" "runtime_helper_semantics_change=0"
require_line "$DOC" "mirbuilder_changes_allowed=0"
require_line "$DOC" "hako_source_changes_allowed=0"
require_line "$DOC" "direct_slot_payload_formula=object_base_plus_24_plus_slot_times_16_plus_8"
require_line "$DOC" "unsigned_set_nonnegative_trap_required=1"
require_line "$DOC" "exact_status_continue_label_required=1"
require_line "$DOC" "silent_fallback_allowed=0"
require_line "$DOC" "ffi_shim_rebuild_required=1"
require_line "$DOC" "implementation_open=0"
require_line "$DOC" "optimization_open=0"
require_line "$DOC" "provider_activation=0"
require_line "$DOC" "host_replacement=0"
require_line "$DOC" "replacement_active=0"
require_line "$DOC" "hook_installed=0"
require_line "$DOC" "global_allocator=0"
require_line "$DOC" "winner_claim=0"
require_line "$DOC" "summary=ok"

require_pattern "$SRC" "get_typed_object_binding"
require_pattern "$SRC" "typed_object_plan_field_runtime_slot_with_storage"
require_pattern "$SRC" "same_module_function_direct_slot_nativedirect_storage_supported"
require_pattern "$SRC" "typed_object_storage_is_exact_slot_i64"
require_pattern "$SRC" "typed_object_storage_is_exact_slot_u64"
require_pattern "$SRC" "typed_object_storage_is_exact_slot_handle"
require_pattern "$SRC" "SAME_MODULE_DIRECT_SLOT_OBJECT_HEADER_BYTES"
require_pattern "$SRC" "SAME_MODULE_DIRECT_SLOT_CELL_BYTES"
require_pattern "$SRC" "SAME_MODULE_DIRECT_SLOT_CELL_PAYLOAD_OFFSET_BYTES"

cat <<REPORT_TEXT
output_contract=direct-slot-supported-storage-nativedirect-guard-surface-v0
input_contract=direct-slot-post-bootstrap-owner-refresh-v0
workload_id=representative-object-lifecycle-small-block-v0
selection_kind=fact_driven_supported_storage
implementation_open=0
provider_activation=0
host_replacement=0
hook_installed=0
global_allocator=0
winner_claim=0
summary=ok
REPORT_TEXT
