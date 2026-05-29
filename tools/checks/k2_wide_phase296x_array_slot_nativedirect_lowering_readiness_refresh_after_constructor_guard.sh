#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-367-ARRAY-SLOT-NATIVEDIRECT-LOWERING-READINESS-REFRESH-AFTER-CONSTRUCTOR.md"
STATE="$ROOT_DIR/docs/development/current/main/CURRENT_STATE.toml"
NEWBOX="$ROOT_DIR/src/llvm_py/instructions/newbox.py"
CONSTRUCTOR="$ROOT_DIR/src/llvm_py/instructions/mir_call/constructor_call.py"

require_line() {
  local file="$1"
  local needle="$2"
  if ! grep -Fq "$needle" "$file"; then
    echo "[row367-array-slot-nativedirect-readiness-refresh] missing line in ${file#$ROOT_DIR/}: $needle" >&2
    exit 1
  fi
}

require_line "$DOC" "output_contract=array-slot-nativedirect-lowering-readiness-refresh-after-constructor-v0"
require_line "$DOC" "input_contract=direct-array-i64-constructor-lowering-pilot-v0"
require_line "$DOC" "selected_owner_file=src/llvm_py/instructions/mir_call/collection_method_call.py"
require_line "$DOC" "selected_direct_birth_symbol=nyash.array.direct_i64.birth_h"
require_line "$DOC" "default_public_birth_symbol=nyash.array.birth_h"
require_line "$DOC" "direct_array_birth_handle_producer_available=1"
require_line "$DOC" "constructor_exact_lane_origin_fact_available=1"
require_line "$DOC" "direct_array_origin_fact=resolver.direct_array_i64_ids"
require_line "$DOC" "public_arraybox_handle_as_direct_buffer_allowed=0"
require_line "$DOC" "legacy_retirement_policy=defer_until_direct_array_semantic_smoke_and_perf_owner_refresh"
require_line "$DOC" "legacy_retirement_now=0"
require_line "$DOC" "selected_method_lowering_unblocked=1"
require_line "$DOC" "llvm_lowering_open=0"
require_line "$DOC" "direct_load_store_open=0"
require_line "$DOC" "summary=ok"

require_line "$NEWBOX" "DIRECT_ARRAY_I64_BIRTH_SYMBOL = \"nyash.array.direct_i64.birth_h\""
require_line "$NEWBOX" "PUBLIC_ARRAY_BIRTH_SYMBOL = \"nyash.array.birth_h\""
require_line "$NEWBOX" "resolver.direct_array_i64_ids"
require_line "$CONSTRUCTOR" "DIRECT_ARRAY_I64_BIRTH_SYMBOL = \"nyash.array.direct_i64.birth_h\""
require_line "$CONSTRUCTOR" "PUBLIC_ARRAY_BIRTH_SYMBOL = \"nyash.array.birth_h\""
require_line "$CONSTRUCTOR" "resolver.direct_array_i64_ids"

require_line "$STATE" "latest_card = \"296x-367-ARRAY-SLOT-NATIVEDIRECT-LOWERING-READINESS-REFRESH-AFTER-CONSTRUCTOR\""
require_line "$STATE" "current_blocker_token = \"ARRAY-SLOT-NATIVEDIRECT-SELECTED-METHOD-LOWERING-GUARD-REFRESH-296X-001\""

echo "[row367-array-slot-nativedirect-readiness-refresh] ok"
