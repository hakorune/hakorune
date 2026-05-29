#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-338-DIRECT-SLOT-NATIVEDIRECT-LOWERING-SELECTED-METHOD-PILOT.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-337-DIRECT-SLOT-USIZE-CELL-STORAGE-COMPATIBILITY-PILOT.md"
SRC="$ROOT_DIR/src/llvm_py/instructions/field_access.py"
TEST="$ROOT_DIR/src/llvm_py/tests/test_typed_user_box_field_access.py"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -Fqx "$expected" "$file"; then
    echo "[row338-direct-slot-nativedirect-lowering-selected-method-pilot] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_pattern() {
  local file="$1"
  local pattern="$2"
  if ! grep -Fq "$pattern" "$file"; then
    echo "[row338-direct-slot-nativedirect-lowering-selected-method-pilot] missing pattern in ${file#$ROOT_DIR/}: $pattern" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=direct-slot-nativedirect-lowering-selected-method-pilot-v0"
require_line "$DOC" "input_contract=direct-slot-usize-cell-storage-compatibility-pilot-v0"
require_line "$DOC" "implemented_owner=llvm_field_access_direct_slot_nativedirect_selected_method_hook"
require_line "$DOC" "selected_method=HakoAllocPageModel.acquire_usize/1"
require_line "$DOC" "selected_backend=direct_slot_exact"
require_line "$DOC" "selected_method_only=1"
require_line "$DOC" "direct_slot_exact_only=1"
require_line "$DOC" "direct_slot_payload_formula=object_base_plus_24_plus_slot_times_16_plus_8"
require_line "$DOC" "implemented_get_lowering=payload_load_i64"
require_line "$DOC" "implemented_set_lowering=payload_store_i64"
require_line "$DOC" "supported_storage=i64,u64,usize,handle"
require_line "$DOC" "unsupported_storage_policy=fail_fast_in_selected_method"
require_line "$DOC" "non_selected_method_policy=existing_helper_path"
require_line "$DOC" "silent_fallback_allowed=0"
require_line "$DOC" "helper_load_writeback_substitution_allowed=0"
require_line "$DOC" "typed_slot_enum_layout_exposure=0"
require_line "$DOC" "selected_method_get_smoke=ok"
require_line "$DOC" "selected_method_set_smoke=ok"
require_line "$DOC" "non_selected_method_helper_smoke=ok"
require_line "$DOC" "summary=ok"

require_pattern "$SRC" "DIRECT_SLOT_NATIVEDIRECT_SELECTED_METHOD = \"HakoAllocPageModel.acquire_usize/1\""
require_pattern "$SRC" "DIRECT_SLOT_OBJECT_HEADER_BYTES = 24"
require_pattern "$SRC" "DIRECT_SLOT_CELL_BYTES = 16"
require_pattern "$SRC" "DIRECT_SLOT_CELL_PAYLOAD_OFFSET_BYTES = 8"
require_pattern "$SRC" "def _direct_slot_payload_ptr"
require_pattern "$SRC" "builder.and_"
require_pattern "$SRC" "builder.inttoptr"
require_pattern "$SRC" "builder.load(ptr, name=\"direct_slot_payload_load\")"
require_pattern "$SRC" "builder.store(value_val, ptr)"
require_pattern "$TEST" "test_direct_slot_nativedirect_selected_method_get_loads_payload"
require_pattern "$TEST" "test_direct_slot_nativedirect_selected_method_set_stores_payload"
require_pattern "$TEST" "test_direct_slot_nativedirect_keeps_non_selected_method_on_helper_path"

python3 "$TEST"

echo "[row338-direct-slot-nativedirect-lowering-selected-method-pilot] ok"
