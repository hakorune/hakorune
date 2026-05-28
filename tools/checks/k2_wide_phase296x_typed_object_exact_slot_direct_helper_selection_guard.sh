#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-214-TYPED-OBJECT-EXACT-SLOT-DIRECT-HELPER-SELECTION.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-213-TYPED-OBJECT-FIELD-HELPER-SUBOWNER-REFRESH.md"
TOOL="$ROOT_DIR/tools/allocator/typed_object_exact_slot_direct_helper_selection.py"
TMP_DIR="$(mktemp -d /tmp/hakorune_row214_exact_slot_selection.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT
REPORT="$TMP_DIR/report.out"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[row214-exact-slot-selection] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=typed-object-exact-slot-direct-helper-selection-v0"
require_line "$DOC" "selected_owner_family=typed_object_exact_slot_direct_helper"
require_line "$DOC" "default_helper_abi=unchanged"
require_line "$DOC" "new_helper_symbols=separate"
require_line "$DOC" "default_exact_helper_emission=0"
require_line "$DOC" "selected_symbol_count=6"
require_line "$DOC" "selected_symbol_0=nyash.object.exact_slot_get_i64_hii"
require_line "$DOC" "selected_symbol_1=nyash.object.exact_slot_set_i64_hii"
require_line "$DOC" "selected_symbol_2=nyash.object.exact_slot_get_u64_hii"
require_line "$DOC" "selected_symbol_3=nyash.object.exact_slot_set_u64_hiu"
require_line "$DOC" "selected_symbol_4=nyash.object.exact_slot_get_handle_hii"
require_line "$DOC" "selected_symbol_5=nyash.object.exact_slot_set_handle_hii"
require_line "$DOC" "direct_storage_allowed_0=i64"
require_line "$DOC" "direct_storage_allowed_1=u64"
require_line "$DOC" "direct_storage_allowed_2=usize_if_target_pointer_width_64"
require_line "$DOC" "direct_storage_allowed_3=handle"
require_line "$DOC" "lowering_gate_0=HAKO_TYPED_OBJECT_STORE_single_thread_exact"
require_line "$DOC" "lowering_gate_1=HAKO_TYPED_OBJECT_EXACT_SLOT_HELPER_1"
require_line "$DOC" "runtime_helper_env_check=0"
require_line "$DOC" "runtime_helper_safe_mutex_fallback=0"
require_line "$DOC" "runtime_helper_memory_safety_bounds=preserved"
require_line "$DOC" "rejected_owner_0=existing_helper_mutation"
require_line "$DOC" "rejected_owner_1=generic_typed_field_residence_retry"
require_line "$DOC" "rejected_owner_2=hako_alloc_by_name_special_case"
require_line "$DOC" "optimization_open=0"
require_line "$DOC" "winner_claim=0"
require_line "$DOC" "replacement_active=0"
require_line "$DOC" "hook_installed=0"
require_line "$DOC" "global_allocator=0"
require_line "$DOC" "summary=ok"

"$TOOL" --out "$REPORT"

for expected in \
  "output_contract=typed-object-exact-slot-direct-helper-selection-v0" \
  "input_contract=typed-object-field-helper-subowner-refresh-v0" \
  "selected_owner_family=typed_object_exact_slot_direct_helper" \
  "default_helper_abi=unchanged" \
  "new_helper_symbols=separate" \
  "default_exact_helper_emission=0" \
  "selected_symbol_count=6" \
  "selected_symbol_0=nyash.object.exact_slot_get_i64_hii" \
  "selected_symbol_1=nyash.object.exact_slot_set_i64_hii" \
  "selected_symbol_2=nyash.object.exact_slot_get_u64_hii" \
  "selected_symbol_3=nyash.object.exact_slot_set_u64_hiu" \
  "selected_symbol_4=nyash.object.exact_slot_get_handle_hii" \
  "selected_symbol_5=nyash.object.exact_slot_set_handle_hii" \
  "lowering_gate_0=HAKO_TYPED_OBJECT_STORE_single_thread_exact" \
  "lowering_gate_1=HAKO_TYPED_OBJECT_EXACT_SLOT_HELPER_1" \
  "runtime_helper_env_check=0" \
  "runtime_helper_safe_mutex_fallback=0" \
  "runtime_helper_memory_safety_bounds=preserved" \
  "rejected_owner_0=existing_helper_mutation" \
  "rejected_owner_1=generic_typed_field_residence_retry" \
  "rejected_owner_2=hako_alloc_by_name_special_case" \
  "optimization_open=0" \
  "winner_claim=0" \
  "replacement_active=0" \
  "hook_installed=0" \
  "global_allocator=0" \
  "summary=ok"; do
  require_line "$REPORT" "$expected"
done

cat "$REPORT"
