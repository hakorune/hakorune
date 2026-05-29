#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-369-ARRAY-SLOT-NATIVEDIRECT-SELECTED-METHOD-LOWERING-IMPLEMENTATION.md"
STATE="$ROOT_DIR/docs/development/current/main/CURRENT_STATE.toml"
OWNER="$ROOT_DIR/src/llvm_py/instructions/mir_call/collection_method_call.py"
TEST="$ROOT_DIR/src/llvm_py/tests/test_collection_method_call.py"

require_line() {
  local file="$1"
  local needle="$2"
  if ! grep -Fq "$needle" "$file"; then
    echo "[row369-array-slot-nativedirect-implementation] missing line in ${file#$ROOT_DIR/}: $needle" >&2
    exit 1
  fi
}

require_line "$DOC" "output_contract=array-slot-nativedirect-selected-method-lowering-implementation-v0"
require_line "$DOC" "input_contract=array-slot-nativedirect-selected-method-lowering-guard-refresh-v0"
require_line "$DOC" "implemented_owner_file=src/llvm_py/instructions/mir_call/collection_method_call.py"
require_line "$DOC" "receiver_origin_fact=resolver.direct_array_i64_ids"
require_line "$DOC" "direct_array_get_lowering=direct_i64_load_with_oob_zero"
require_line "$DOC" "direct_array_set_lowering=direct_i64_store_with_append_len_update_and_oob_zero"
require_line "$DOC" "legacy_retirement_now=0"
require_line "$DOC" "selected_next=array_slot_nativedirect_selected_method_semantic_smoke"
require_line "$DOC" "summary=ok"

require_line "$OWNER" "DIRECT_ARRAY_NATIVEDIRECT_SELECTED_METHOD = \"HakoAllocPageModel.acquire_usize/1\""
require_line "$OWNER" "DIRECT_ARRAY_DATA_OFFSET_BYTES = 32"
require_line "$OWNER" "resolver.direct_array_i64_ids"
require_line "$OWNER" "def _lower_direct_array_i64_get("
require_line "$OWNER" "def _lower_direct_array_i64_set("
require_line "$OWNER" "direct_array_i64_get_result"
require_line "$OWNER" "direct_array_i64_set_result"

require_line "$TEST" "test_direct_array_selected_method_get_lowers_to_direct_load"
require_line "$TEST" "test_direct_array_selected_method_set_lowers_to_direct_store"
require_line "$TEST" "test_direct_array_non_origin_receiver_keeps_helper_path"

PYTHONPATH="$ROOT_DIR/src/llvm_py:$ROOT_DIR" python3 -m unittest "$TEST"

require_line "$STATE" "latest_card = \"296x-369-ARRAY-SLOT-NATIVEDIRECT-SELECTED-METHOD-LOWERING-IMPLEMENTATION\""
require_line "$STATE" "current_blocker_token = \"ARRAY-SLOT-NATIVEDIRECT-SELECTED-METHOD-SEMANTIC-SMOKE-296X-001\""

echo "[row369-array-slot-nativedirect-implementation] ok"
