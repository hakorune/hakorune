#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-368-ARRAY-SLOT-NATIVEDIRECT-SELECTED-METHOD-LOWERING-GUARD-REFRESH.md"
STATE="$ROOT_DIR/docs/development/current/main/CURRENT_STATE.toml"
OWNER="$ROOT_DIR/src/llvm_py/instructions/mir_call/collection_method_call.py"

require_line() {
  local file="$1"
  local needle="$2"
  if ! grep -Fq "$needle" "$file"; then
    echo "[row368-array-slot-nativedirect-guard-refresh] missing line in ${file#$ROOT_DIR/}: $needle" >&2
    exit 1
  fi
}

require_line "$DOC" "output_contract=array-slot-nativedirect-selected-method-lowering-guard-refresh-v0"
require_line "$DOC" "input_contract=array-slot-nativedirect-lowering-readiness-refresh-after-constructor-v0"
require_line "$DOC" "selected_owner_file=src/llvm_py/instructions/mir_call/collection_method_call.py"
require_line "$DOC" "selected_backend=direct_array_i64_exact"
require_line "$DOC" "receiver_origin_fact=resolver.direct_array_i64_ids"
require_line "$DOC" "receiver_origin_fact_required=1"
require_line "$DOC" "public_arraybox_handle_as_direct_buffer_allowed=0"
require_line "$DOC" "arraybox_get_selected_method_direct_load_allowed=1"
require_line "$DOC" "arraybox_set_selected_method_direct_store_allowed=1"
require_line "$DOC" "generic_arraybox_rewrite_allowed=0"
require_line "$DOC" "field_address_formula=buffer_base_plus_header_offset_plus_index_times_8"
require_line "$DOC" "planned_net_helper_delta_positive=1"
require_line "$DOC" "implementation_open=0"
require_line "$DOC" "direct_load_store_open=0"
require_line "$DOC" "selected_next=array_slot_nativedirect_selected_method_lowering_implementation"
require_line "$DOC" "Legacy helper/cache retirement remains deferred"
require_line "$DOC" "summary=ok"

require_line "$OWNER" "def _lower_array_collection_method_call("
require_line "$OWNER" "def lower_collection_method_call("
require_line "$OWNER" "select_array_collection_call_spec"

require_line "$STATE" "latest_card = \"296x-368-ARRAY-SLOT-NATIVEDIRECT-SELECTED-METHOD-LOWERING-GUARD-REFRESH\""
require_line "$STATE" "current_blocker_token = \"ARRAY-SLOT-NATIVEDIRECT-SELECTED-METHOD-LOWERING-IMPLEMENTATION-296X-001\""

echo "[row368-array-slot-nativedirect-guard-refresh] ok"
